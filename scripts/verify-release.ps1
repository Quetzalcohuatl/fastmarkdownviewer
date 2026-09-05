[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [ValidatePattern('^\d+\.\d+\.\d+$')]
    [string] $Version,

    [Parameter(Mandatory = $true)]
    [string] $Directory
)

$ErrorActionPreference = 'Stop'
$directoryPath = (Resolve-Path -LiteralPath $Directory).Path
$expected = @(
    "FastMarkdownViewer-$Version-windows-x86_64.exe",
    "FastMarkdownViewer-$Version-windows-x86_64.zip",
    "FastMarkdownViewer-Setup-$Version.exe",
    'SHA256SUMS.txt'
)
$actual = Get-ChildItem -LiteralPath $directoryPath -File | Select-Object -ExpandProperty Name | Sort-Object
if (Compare-Object ($expected | Sort-Object) $actual) {
    throw "Release directory does not contain exactly the expected four assets: $($actual -join ', ')"
}

foreach ($line in [System.IO.File]::ReadAllLines((Join-Path $directoryPath 'SHA256SUMS.txt'))) {
    if ($line -notmatch '^([0-9a-f]{64}) \*(.+)$') { throw "Malformed checksum line: $line" }
    $actualHash = (Get-FileHash -Algorithm SHA256 -LiteralPath (Join-Path $directoryPath $Matches[2])).Hash.ToLowerInvariant()
    if ($actualHash -ne $Matches[1]) { throw "Checksum mismatch: $($Matches[2])" }
}

$extract = Join-Path ([System.IO.Path]::GetTempPath()) "FastMarkdownViewer-verify-$([guid]::NewGuid())"
New-Item -ItemType Directory -Path $extract | Out-Null
try {
    Expand-Archive -LiteralPath (Join-Path $directoryPath "FastMarkdownViewer-$Version-windows-x86_64.zip") -DestinationPath $extract
    $zipNames = Get-ChildItem -LiteralPath $extract -File | Select-Object -ExpandProperty Name | Sort-Object
    $expectedZip = @('FastMarkdownViewer.exe', 'README.md', 'LICENSE-MIT', 'LICENSE-APACHE', 'THIRD_PARTY_NOTICES.md') | Sort-Object
    if (Compare-Object $expectedZip $zipNames) { throw "Unexpected ZIP contents: $($zipNames -join ', ')" }
    $directHash = (Get-FileHash -Algorithm SHA256 -LiteralPath (Join-Path $directoryPath "FastMarkdownViewer-$Version-windows-x86_64.exe")).Hash
    $zipExeHash = (Get-FileHash -Algorithm SHA256 -LiteralPath (Join-Path $extract 'FastMarkdownViewer.exe')).Hash
    if ($directHash -ne $zipExeHash) { throw 'The direct EXE and ZIP portable EXE are not byte-identical.' }
}
finally {
    Remove-Item -Recurse -Force -LiteralPath $extract
}

Write-Host 'Release assets and checksums verified.'
