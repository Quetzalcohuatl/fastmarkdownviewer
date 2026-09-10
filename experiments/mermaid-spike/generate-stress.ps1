$ErrorActionPreference = 'Stop'
$corpus = Join-Path $PSScriptRoot 'corpus'
$chain = @('flowchart TD') + (0..199 | ForEach-Object { "    N$_ --> N$($_ + 1)" })
$chain | Set-Content (Join-Path $corpus 'chain-200.mmd')
$large = @('flowchart TD') + (0..1999 | ForEach-Object { "    N$_ --> N$($_ + 1)" })
$large | Set-Content (Join-Path $corpus 'chain-2000.mmd')
("flowchart LR`n%% " + ('x' * 66000)) | Set-Content (Join-Path $corpus 'oversize.mmd')
