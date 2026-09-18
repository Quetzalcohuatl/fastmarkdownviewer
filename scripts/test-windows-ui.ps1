[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string]$Binary,

    [Parameter(Mandatory = $true)]
    [string]$Fixture,

    [ValidateRange(2, 60)]
    [int]$TimeoutSeconds = 15,

    [ValidateSet('auto', 'software')]
    [string]$Renderer = 'auto',

    [switch]$Headless
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$binaryPath = (Resolve-Path -LiteralPath $Binary).Path
$fixturePath = (Resolve-Path -LiteralPath $Fixture).Path
if ([System.IO.Path]::GetExtension($binaryPath) -ne '.exe') {
    throw "UI smoke target must be an .exe: $binaryPath"
}

if ($Headless) {
    $versionOutput = & $binaryPath --version | Out-String
    if ($LASTEXITCODE -ne 0) {
        throw "FastMarkdownViewer --version failed with exit code $LASTEXITCODE."
    }
    if ($versionOutput.Trim() -notmatch '^FastMarkdownViewer \d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?$') {
        throw "Unexpected --version output: $($versionOutput.Trim())"
    }
    Write-Output "WINDOWS_HEADLESS_SMOKE=passed version=$($versionOutput.Trim())"
    return
}

Add-Type -TypeDefinition @'
using System;
using System.Runtime.InteropServices;

public static class FastMarkdownViewerNativeTest
{
    [DllImport("user32.dll")]
    [return: MarshalAs(UnmanagedType.Bool)]
    private static extern bool PostMessage(IntPtr window, uint message, UIntPtr word, IntPtr data);

    public static bool SendKey(IntPtr window, uint virtualKey)
    {
        const uint KeyDown = 0x0100;
        const uint KeyUp = 0x0101;
        return PostMessage(window, KeyDown, (UIntPtr)virtualKey, IntPtr.Zero)
            && PostMessage(window, KeyUp, (UIntPtr)virtualKey, new IntPtr(unchecked((int)0xC0000001)));
    }
}
'@

function Wait-ForViewerWindow {
    param(
        [Parameter(Mandatory = $true)]
        [System.Diagnostics.Process]$Process
    )

    $deadline = [DateTime]::UtcNow.AddSeconds($TimeoutSeconds)
    while ([DateTime]::UtcNow -lt $deadline) {
        $Process.Refresh()
        if ($Process.HasExited) {
            throw "FastMarkdownViewer exited before showing a window (exit code $($Process.ExitCode))."
        }
        if (
            $Process.MainWindowHandle -ne [IntPtr]::Zero -and
            -not [string]::IsNullOrWhiteSpace($Process.MainWindowTitle)
        ) {
            return
        }
        Start-Sleep -Milliseconds 100
    }
    throw "FastMarkdownViewer did not show a window within $TimeoutSeconds seconds."
}

function Close-TestProcess {
    param([System.Diagnostics.Process]$Process)

    if ($null -eq $Process) {
        return
    }
    $Process.Refresh()
    if ($Process.HasExited) {
        return
    }
    [void]$Process.CloseMainWindow()
    if (-not $Process.WaitForExit(3000)) {
        Stop-Process -Id $Process.Id
    }
}

$primary = $null
$secondary = $null
$bare = $null
$originalAppData = $env:APPDATA
$originalRenderer = $env:FMV_GRAPHICS
$testDirectory = Join-Path ([System.IO.Path]::GetTempPath()) ('fmv-ui-' + [Guid]::NewGuid())
[void](New-Item -ItemType Directory -Path $testDirectory)
$env:APPDATA = $testDirectory
$env:FMV_GRAPHICS = $Renderer
try {
    # WinGet launches with no document and requires survival beyond ten seconds.
    $diagnostics = Join-Path $testDirectory 'startup.log'
    $bare = Start-Process -FilePath $binaryPath -WindowStyle Hidden -RedirectStandardError $diagnostics -PassThru
    Wait-ForViewerWindow -Process $bare
    Start-Sleep -Seconds 11
    $bare.Refresh()
    if ($bare.HasExited) {
        throw "Bare startup failed (exit code $($bare.ExitCode)): $(Get-Content $diagnostics -Raw)"
    }
    $startupLog = Get-Content $diagnostics -Raw
    if ($Renderer -eq 'software' -and $startupLog -notmatch 'FMV_RENDERER=Dx12 adapter=.+ device=Cpu') {
        throw "Software test did not select the Direct3D CPU adapter: $startupLog"
    }
    Write-Output $startupLog
    Close-TestProcess -Process $bare

    $quotedFixture = '"' + $fixturePath + '"'
    $primary = Start-Process -FilePath $binaryPath -ArgumentList $quotedFixture -WindowStyle Hidden -PassThru
    Wait-ForViewerWindow -Process $primary
    $primary.Refresh()
    $expectedTitle = [System.IO.Path]::GetFileName($fixturePath) + ' — FastMarkdownViewer'
    if ($primary.MainWindowTitle -ne $expectedTitle) {
        throw "Unexpected primary window title: $($primary.MainWindowTitle)"
    }

    foreach ($virtualKey in @(0x22, 0x28, 0x23, 0x24)) {
        if (-not [FastMarkdownViewerNativeTest]::SendKey($primary.MainWindowHandle, $virtualKey)) {
            throw "Could not deliver native scroll key 0x$($virtualKey.ToString('X'))."
        }
    }
    Start-Sleep -Milliseconds 500
    $primary.Refresh()
    if ($primary.HasExited) {
        throw "FastMarkdownViewer crashed while processing scroll keys (exit code $($primary.ExitCode))."
    }

    $secondary = Start-Process -FilePath $binaryPath -ArgumentList $quotedFixture -WindowStyle Hidden -PassThru
    Wait-ForViewerWindow -Process $secondary
    if ($primary.Id -eq $secondary.Id) {
        throw 'Separate launches unexpectedly reused one process.'
    }
    $primary.Refresh()
    $secondary.Refresh()
    if ($primary.HasExited -or $secondary.HasExited) {
        throw 'One of the independent viewer windows exited unexpectedly.'
    }

    Write-Output "WINDOWS_UI_SMOKE=passed renderer=$Renderer bare-startup=passed primary=$($primary.Id) secondary=$($secondary.Id)"
}
finally {
    Close-TestProcess -Process $secondary
    Close-TestProcess -Process $primary
    Close-TestProcess -Process $bare
    $env:APPDATA = $originalAppData
    $env:FMV_GRAPHICS = $originalRenderer
}
