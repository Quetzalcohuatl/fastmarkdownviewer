[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string]$Installer,

    [Parameter(Mandatory = $true)]
    [string]$Fixture
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$installerPath = (Resolve-Path -LiteralPath $Installer).Path
$fixturePath = (Resolve-Path -LiteralPath $Fixture).Path
$installDirectory = Join-Path $env:LOCALAPPDATA 'Programs\FastMarkdownViewer'
$installedBinary = Join-Path $installDirectory 'FastMarkdownViewer.exe'
$uninstaller = Join-Path $installDirectory 'unins000.exe'
$progIdKey = 'HKCU:\Software\Classes\FastMarkdownViewer.Document'
$markdownKey = 'HKCU:\Software\Classes\.md'
$longMarkdownKey = 'HKCU:\Software\Classes\.markdown'

function Get-DefaultAssociation([string]$Extension) {
    $base = [Microsoft.Win32.Registry]::CurrentUser.OpenSubKey("Software\Classes\$Extension")
    if ($null -eq $base) {
        return $null
    }
    try {
        return $base.GetValue('', $null, [Microsoft.Win32.RegistryValueOptions]::DoNotExpandEnvironmentNames)
    }
    finally {
        $base.Dispose()
    }
}

function Has-OpenWithValue([string]$KeyPath) {
    $key = Get-Item -LiteralPath "$KeyPath\OpenWithProgids" -ErrorAction SilentlyContinue
    if ($null -eq $key) {
        return $false
    }
    return $key.GetValueNames() -contains 'FastMarkdownViewer.Document'
}

function Invoke-Uninstaller {
    if (-not (Test-Path -LiteralPath $uninstaller -PathType Leaf)) {
        return
    }
    $process = Start-Process -FilePath $uninstaller -ArgumentList @(
        '/VERYSILENT',
        '/SUPPRESSMSGBOXES',
        '/NORESTART'
    ) -PassThru -Wait
    if ($process.ExitCode -ne 0) {
        throw "Uninstaller failed with exit code $($process.ExitCode)."
    }

    $deadline = [DateTime]::UtcNow.AddSeconds(15)
    while ((Test-Path -LiteralPath $installedBinary) -and [DateTime]::UtcNow -lt $deadline) {
        Start-Sleep -Milliseconds 100
    }
}

if (Test-Path -LiteralPath $installDirectory) {
    throw "Refusing to replace a pre-existing installation: $installDirectory"
}
if ((Test-Path -LiteralPath $progIdKey) -or
    (Has-OpenWithValue -KeyPath $markdownKey) -or
    (Has-OpenWithValue -KeyPath $longMarkdownKey)) {
    throw 'Refusing to replace pre-existing FastMarkdownViewer registry entries.'
}

$mdDefaultBefore = Get-DefaultAssociation '.md'
$markdownDefaultBefore = Get-DefaultAssociation '.markdown'
$installed = $false
try {
    $install = Start-Process -FilePath $installerPath -ArgumentList @(
        '/VERYSILENT',
        '/SUPPRESSMSGBOXES',
        '/NORESTART',
        '/SP-'
    ) -PassThru -Wait
    if ($install.ExitCode -ne 0) {
        throw "Installer failed with exit code $($install.ExitCode)."
    }
    $installed = $true

    if (-not (Test-Path -LiteralPath $installedBinary -PathType Leaf)) {
        throw "Installer did not create $installedBinary"
    }
    if (-not (Test-Path -LiteralPath $uninstaller -PathType Leaf)) {
        throw 'Installer did not create its uninstaller.'
    }
    if (-not (Test-Path -LiteralPath $progIdKey)) {
        throw 'Installer did not register the FastMarkdownViewer ProgID.'
    }
    if (-not (Has-OpenWithValue -KeyPath $markdownKey)) {
        throw 'Installer did not add the .md Open with entry.'
    }
    if (-not (Has-OpenWithValue -KeyPath $longMarkdownKey)) {
        throw 'Installer did not add the .markdown Open with entry.'
    }
    if ((Get-DefaultAssociation '.md') -ne $mdDefaultBefore -or
        (Get-DefaultAssociation '.markdown') -ne $markdownDefaultBefore) {
        throw 'Installer changed a default file association.'
    }

    & (Join-Path $PSScriptRoot 'test-windows-ui.ps1') `
        -Binary $installedBinary `
        -Fixture $fixturePath
    if ($LASTEXITCODE -ne 0) {
        throw "Installed application smoke test failed with exit code $LASTEXITCODE."
    }

    Invoke-Uninstaller
    $installed = $false

    if (Test-Path -LiteralPath $installedBinary) {
        throw 'Uninstaller left the application binary behind.'
    }
    if (Test-Path -LiteralPath $progIdKey) {
        throw 'Uninstaller left the application ProgID behind.'
    }
    if ((Has-OpenWithValue -KeyPath $markdownKey) -or
        (Has-OpenWithValue -KeyPath $longMarkdownKey)) {
        throw 'Uninstaller left an Open with entry behind.'
    }
    if ((Get-DefaultAssociation '.md') -ne $mdDefaultBefore -or
        (Get-DefaultAssociation '.markdown') -ne $markdownDefaultBefore) {
        throw 'Install/uninstall changed a default file association.'
    }

    Write-Output 'WINDOWS_INSTALLER_SMOKE=passed'
}
finally {
    if ($installed) {
        Invoke-Uninstaller
    }
}
