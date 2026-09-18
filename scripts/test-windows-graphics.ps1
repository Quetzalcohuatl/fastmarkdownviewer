[CmdletBinding()]
param(
    [string]$Binary = '.\target\debug\examples\visual_check.exe',
    [string]$OutputDirectory = '.\target\windows-graphics'
)
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$binaryPath = (Resolve-Path -LiteralPath $Binary).Path
[void](New-Item -ItemType Directory -Path $OutputDirectory -Force)
$outputPath = (Resolve-Path -LiteralPath $OutputDirectory).Path
$fixture = (Resolve-Path -LiteralPath '.\tests\fixtures\feature-matrix.md').Path
$originalRenderer = $env:FMV_GRAPHICS
$originalSoftwareGl = $env:FMV_VISUAL_SOFTWARE_OPENGL
try {
    foreach ($mode in @('fallback', 'software')) {
        $env:FMV_GRAPHICS = if ($mode -eq 'software') { 'software' } else { 'auto' }
        # Request the system's unaccelerated OpenGL driver to reproduce WinGet's failure.
        $env:FMV_VISUAL_SOFTWARE_OPENGL = '1'
        $capture = Join-Path $outputPath "$mode.png"
        $diagnostics = Join-Path $outputPath "$mode.log"
        if (Test-Path -LiteralPath $capture) { Remove-Item -LiteralPath $capture }
        $process = Start-Process -FilePath $binaryPath -WindowStyle Hidden -PassThru `
            -ArgumentList @('"' + $fixture + '"', '"' + $capture + '"') -RedirectStandardError $diagnostics
        if (-not $process.WaitForExit(30000)) {
            Stop-Process -Id $process.Id
            throw "$mode capture timed out."
        }
        $log = Get-Content -LiteralPath $diagnostics -Raw
        Write-Output $log
        if ($process.ExitCode -ne 0 -or -not (Test-Path -LiteralPath $capture)) {
            throw "$mode capture failed (exit $($process.ExitCode)): $log"
        }
        if ((Get-Item -LiteralPath $capture).Length -lt 1000) { throw "$mode capture is empty." }
        if ($mode -eq 'fallback' -and $log -notmatch 'Graphics startup \(Primary\) failed:') {
            throw 'The OpenGL failure was not reproduced; the automatic retry was not exercised.'
        }
        if ($log -notmatch 'FMV_RENDERER=Dx12') { throw 'Direct3D did not render the document.' }
        if ($mode -eq 'software' -and $log -notmatch 'device=Cpu') { throw 'WARP was not selected.' }
        Write-Output "WINDOWS_GRAPHICS_CAPTURE=passed mode=$mode path=$capture"
    }
}
finally {
    $env:FMV_GRAPHICS = $originalRenderer
    $env:FMV_VISUAL_SOFTWARE_OPENGL = $originalSoftwareGl
}
