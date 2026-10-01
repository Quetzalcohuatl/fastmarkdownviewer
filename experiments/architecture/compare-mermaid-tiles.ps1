[CmdletBinding()]
param(
    [Parameter(Mandatory)] [string] $Before,
    [Parameter(Mandatory)] [string] $After,
    [Parameter(Mandatory)] [string] $OutputDirectory,
    [int] $Runs = 5
)
$ErrorActionPreference = 'Stop'
New-Item -ItemType Directory -Force -Path $OutputDirectory | Out-Null
$outputPath = (Resolve-Path -LiteralPath $OutputDirectory).Path
$beforePath = (Resolve-Path -LiteralPath $Before).Path
$afterPath = (Resolve-Path -LiteralPath $After).Path
$appearance = @{ background=@(39,40,34); surface=@(52,53,47); text=@(248,248,242); muted=@(176,176,155); accent=@(166,226,46); selection=@(73,72,62); font=$null } | ConvertTo-Json -Compress
$variants = @(
    @{ name='before'; exe=$beforePath; request=$null },
    @{ name='tiles-display'; exe=$afterPath; request=@{max_width=800;pixel_scale=1} },
    @{ name='tiles-full'; exe=$afterPath; request=@{max_width=50000;pixel_scale=1} }
)
$fixtures = @{
    small = "flowchart LR`n A[Open document] --> B[Read text] --> C[Load diagram]"
    large = "sequenceDiagram`n participant A as Reader`n participant B as Renderer`n" + ((1..70 | ForEach-Object { ' A->>B: ' + ('wide label ' * 22) }) -join "`n")
}
foreach ($name in $fixtures.Keys) {
    [IO.File]::WriteAllText((Join-Path $outputPath "$name.mmd"), $fixtures[$name])
    [IO.File]::WriteAllText((Join-Path $outputPath "$name.md"), "# Progressive Mermaid`n`nThis text is available while the diagram renders.`n`n``````mermaid`n" + $fixtures[$name] + "`n```````n")
}
$rows = [Collections.Generic.List[object]]::new()
# Fresh helper processes, with one excluded warmup round. Polling adds ~5 ms granularity.
for ($run = 0; $run -le $Runs; $run++) {
    foreach ($name in $fixtures.Keys) {
        foreach ($variant in ($variants | Sort-Object { Get-Random })) {
            $directory = Join-Path $outputPath "$run-$name-$($variant.name)"
            New-Item -ItemType Directory -Path $directory | Out-Null
            $tiled = $null -ne $variant.request
            $result = Join-Path $directory $(if ($tiled) { 'diagram.json' } else { 'diagram.png' })
            $firstTile = Join-Path $directory 'tile-0-0.png'
            $info = [Diagnostics.ProcessStartInfo]::new($variant.exe)
            $info.UseShellExecute = $false
            $info.CreateNoWindow = $true
            $info.ArgumentList.Add('--internal-mermaid')
            $info.ArgumentList.Add((Join-Path $outputPath "$name.mmd"))
            $info.ArgumentList.Add($result)
            $info.ArgumentList.Add($appearance)
            if ($tiled) { $info.ArgumentList.Add(($variant.request | ConvertTo-Json -Compress)) }
            $firstMs = $null
            $geometryMs = $null
            $peak = 0L
            $watch = [Diagnostics.Stopwatch]::StartNew()
            $process = [Diagnostics.Process]::Start($info)
            try {
                do {
                    if ($null -eq $geometryMs -and (Test-Path -LiteralPath $result)) { $geometryMs = $watch.Elapsed.TotalMilliseconds }
                    if ($tiled -and $null -eq $firstMs -and (Test-Path -LiteralPath $firstTile)) { $firstMs = $watch.Elapsed.TotalMilliseconds }
                    $process.Refresh()
                    if (!$process.HasExited) { $peak = [Math]::Max($peak, $process.PeakWorkingSet64) }
                    if ($watch.Elapsed.TotalSeconds -gt 60) { $process.Kill(); throw 'Benchmark deadline exceeded' }
                } while (!$process.WaitForExit(5))
                $watch.Stop()
                if ($process.ExitCode -eq 0) {
                    if ($null -eq $firstMs) { $firstMs = $watch.Elapsed.TotalMilliseconds }
                    if ($null -eq $geometryMs) { $geometryMs = $watch.Elapsed.TotalMilliseconds }
                }
                $width = $null
                $height = $null
                if ($tiled -and (Test-Path -LiteralPath $result)) {
                    $geometry = Get-Content -Raw -LiteralPath $result | ConvertFrom-Json
                    $width = $geometry.width
                    $height = $geometry.height
                }
                if ($run -gt 0) {
                    $rows.Add([pscustomobject]@{run=$run;fixture=$name;variant=$variant.name;exit_code=$process.ExitCode;geometry_ms=$geometryMs;first_tile_ms=$firstMs;total_ms=$watch.Elapsed.TotalMilliseconds;sampled_peak_working_set=$peak;width=$width;height=$height})
                }
            } finally { $process.Dispose() }
        }
    }
}
$rows | Export-Csv -NoTypeInformation -LiteralPath (Join-Path $outputPath 'mermaid-tiles.csv')
$rows | Group-Object fixture,variant | ForEach-Object {
    $times = @($_.Group.total_ms | Sort-Object)
    $first = @($_.Group.first_tile_ms | Where-Object { $null -ne $_ } | Sort-Object)
    [pscustomobject]@{case=$_.Name;median_total_ms=$times[[int][Math]::Floor($times.Count/2)];median_first_tile_ms=$(if($first.Count){$first[[int][Math]::Floor($first.Count/2)]});exit_code=$_.Group[0].exit_code}
} | Format-Table
