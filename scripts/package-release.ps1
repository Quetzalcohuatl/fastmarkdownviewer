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

$zip = Join-Path $output "FastMarkdownViewer-$Version-windows-x86_64.zip"
Compress-Archive -Path (Join-Path $staging '*') -DestinationPath $zip -CompressionLevel Optimal

function New-ApplicationIcon([string] $Path) {
    $size = 32
    $xorSize = $size * $size * 4
    $andSize = 4 * $size
    $imageSize = 40 + $xorSize + $andSize
    $stream = [System.IO.MemoryStream]::new()
    $writer = [System.IO.BinaryWriter]::new($stream)
    $writer.Write([uint16] 0); $writer.Write([uint16] 1); $writer.Write([uint16] 1)
    $writer.Write([byte] $size); $writer.Write([byte] $size); $writer.Write([byte] 0); $writer.Write([byte] 0)
    $writer.Write([uint16] 1); $writer.Write([uint16] 32); $writer.Write([uint32] $imageSize); $writer.Write([uint32] 22)
    $writer.Write([uint32] 40); $writer.Write([int32] $size); $writer.Write([int32] ($size * 2))
    $writer.Write([uint16] 1); $writer.Write([uint16] 32); $writer.Write([uint32] 0); $writer.Write([uint32] $xorSize)
    $writer.Write([int32] 0); $writer.Write([int32] 0); $writer.Write([uint32] 0); $writer.Write([uint32] 0)
    for ($y = $size - 1; $y -ge 0; $y--) {
        for ($x = 0; $x -lt $size; $x++) {
            $page = $x -ge 5 -and $x -lt 27 -and $y -ge 3 -and $y -lt 29
            $fold = $x -ge 21 -and $y -lt 9 -and ($x - 21) -ge (8 - $y)
            $line = $x -ge 9 -and $x -lt 24 -and $y -in @(12, 17, 22) -and (-not $fold -or $y -ge 12)
            if ($line) { $rgba = @(31, 41, 55, 255) }
            elseif ($page -and -not $fold) { $rgba = @(243, 244, 246, 255) }
            elseif ($page) { $rgba = @(145, 164, 188, 255) }
            else { $rgba = @(30, 111, 214, 255) }
            $writer.Write([byte] $rgba[2]); $writer.Write([byte] $rgba[1]); $writer.Write([byte] $rgba[0]); $writer.Write([byte] $rgba[3])
        }
    }
    for ($index = 0; $index -lt $andSize; $index++) { $writer.Write([byte] 0) }
    $writer.Flush()
    [System.IO.File]::WriteAllBytes($Path, $stream.ToArray())
    $writer.Dispose(); $stream.Dispose()
}

$icon = Join-Path $output 'FastMarkdownViewer.ico'
New-ApplicationIcon $icon
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
