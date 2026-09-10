$ErrorActionPreference = 'Stop'
$corpus = Join-Path $PSScriptRoot 'focused-corpus'
New-Item -ItemType Directory -Force $corpus | Out-Null
foreach ($count in @(25, 50, 100, 250, 500)) {
    (@('flowchart TD') + (0..($count - 1) | ForEach-Object { "N$_ --> N$($_ + 1)" })) | Set-Content (Join-Path $corpus "chain-$count.mmd")
    (@('flowchart LR') + (1..$count | ForEach-Object { "Root --> N$_" })) | Set-Content (Join-Path $corpus "fan-$count.mmd")
}
