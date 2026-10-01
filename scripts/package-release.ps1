[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [ValidatePattern('^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?$')]
    [string] $Version,

    [Parameter(Mandatory = $true)]
    [string] $Binary,

    [Parameter(Mandatory = $true)]
    [string] $OutputDirectory,

    [string] $InnoCompiler = '',

    [ValidatePattern('^https://github\.com/[^/]+/[^/]+/?$')]
    [string] $RepositoryUrl = ''
)

$ErrorActionPreference = 'Stop'
$repositoryRoot = Split-Path -Parent $PSScriptRoot
$sourceBinary = (Resolve-Path -LiteralPath $Binary).Path
$output = [System.IO.Path]::GetFullPath($OutputDirectory)
$staging = Join-Path $output 'portable'

if ([string]::IsNullOrWhiteSpace($InnoCompiler)) {
    $InnoCompiler = @(
        (Join-Path ${env:ProgramFiles(x86)} 'Inno Setup 6\ISCC.exe'),
        (Join-Path $env:ProgramFiles 'Inno Setup 6\ISCC.exe'),
        (Join-Path $env:LOCALAPPDATA 'Programs\Inno Setup 6\ISCC.exe')
    ) | Where-Object { Test-Path -LiteralPath $_ } | Select-Object -First 1
}

if ($output.TrimEnd('\') -eq [System.IO.Path]::GetPathRoot($output).TrimEnd('\') -or
    $output.TrimEnd('\') -eq $repositoryRoot.TrimEnd('\')) {
    throw "Refusing to use a filesystem or repository root as the packaging output: $output"
}

if (Test-Path -LiteralPath $output) {
    Remove-Item -Recurse -Force -LiteralPath $output
}
New-Item -ItemType Directory -Path $staging -Force | Out-Null

$portableName = "FastMarkdownViewer-$Version-windows-x86_64.exe"
$portableExe = Join-Path $output $portableName
Copy-Item -LiteralPath $sourceBinary -Destination $portableExe
Copy-Item -LiteralPath $sourceBinary -Destination (Join-Path $staging 'FastMarkdownViewer.exe')
foreach ($name in @('README.md', 'LICENSE-MIT', 'LICENSE-APACHE', 'THIRD_PARTY_NOTICES.md')) {
    Copy-Item -LiteralPath (Join-Path $repositoryRoot $name) -Destination $staging
}
Copy-Item -LiteralPath (Join-Path $repositoryRoot 'assets\fonts\OFL-NotoEmoji.txt') -Destination (Join-Path $staging 'LICENSE-NOTO-EMOJI.txt')

$zip = Join-Path $output "FastMarkdownViewer-$Version-windows-x86_64.zip"
Compress-Archive -Path (Join-Path $staging '*') -DestinationPath $zip -CompressionLevel Optimal

$icon = Join-Path $output 'FastMarkdownViewer.ico'
Copy-Item -LiteralPath (Join-Path $repositoryRoot 'assets\icons\windows\app.ico') -Destination $icon
if (-not (Test-Path -LiteralPath $InnoCompiler)) {
    throw "Inno Setup compiler not found: $InnoCompiler"
}
$innoArguments = @(
    "/DAppVersion=$Version",
    "/DVersionInfoVersion=$(($Version -split '-', 2)[0]).0",
    "/DBuildRoot=$(Split-Path -Parent $sourceBinary)",
    "/DOutputDir=$output",
    "/DIconFile=$icon"
)
if (-not [string]::IsNullOrWhiteSpace($RepositoryUrl)) {
    $innoArguments += "/DRepositoryUrl=$($RepositoryUrl.TrimEnd('/'))"
}
$innoArguments += Join-Path $repositoryRoot 'packaging\windows\FastMarkdownViewer.iss'
& $InnoCompiler @innoArguments
if ($LASTEXITCODE -ne 0) { throw "Inno Setup failed with exit code $LASTEXITCODE" }

Remove-Item -Recurse -Force -LiteralPath $staging
Remove-Item -Force -LiteralPath $icon

$artifacts = @(
    $portableExe,
    $zip,
    (Join-Path $output "FastMarkdownViewer-Setup-$Version.exe")
)
$checksumLines = foreach ($artifact in $artifacts) {
    $hash = (Get-FileHash -Algorithm SHA256 -LiteralPath $artifact).Hash.ToLowerInvariant()
    "$hash *$(Split-Path -Leaf $artifact)"
}
[System.IO.File]::WriteAllLines((Join-Path $output 'SHA256SUMS.txt'), $checksumLines, [System.Text.UTF8Encoding]::new($false))

$sourceHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $sourceBinary).Hash
$portableHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $portableExe).Hash
if ($sourceHash -ne $portableHash) { throw 'Direct portable executable differs from the release build.' }

Get-ChildItem -LiteralPath $output | Select-Object Name, Length
