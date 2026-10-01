[CmdletBinding()]
param(
    [Parameter(Mandatory)] [string] $Before,
    [Parameter(Mandatory)] [string] $After,
    [Parameter(Mandatory)] [string] $OutputDirectory,
    [int] $Runs = 9
)
$ErrorActionPreference = 'Stop'
New-Item -ItemType Directory -Force -Path $OutputDirectory | Out-Null
$outputPath = (Resolve-Path -LiteralPath $OutputDirectory).Path
$beforePath = (Resolve-Path -LiteralPath $Before).Path
$afterPath = (Resolve-Path -LiteralPath $After).Path
$light = @{ background=@(248,248,248); surface=@(248,248,248); text=@(60,60,60); muted=@(100,100,100); accent=@(0,90,170); selection=@(180,210,240); font=$null }
$dark = @{ background=@(39,40,34); surface=@(52,53,47); text=@(248,248,242); muted=@(176,176,155); accent=@(166,226,46); selection=@(73,72,62); font=$null }
$font = $dark.Clone()
$font.font = if ($IsWindows) { 'georgia.ttf' } elseif ($IsMacOS) { 'Helvetica.ttc' } else { 'DejaVuSans.ttf' }
$variants = @(
    @{ name='before'; exe=$beforePath; appearance=$null },
    @{ name='after-light'; exe=$afterPath; appearance=$light },
    @{ name='after-monokai'; exe=$afterPath; appearance=$dark },
    @{ name='after-monokai-font'; exe=$afterPath; appearance=$font }
)
$fixtures = @{
    flowchart = "flowchart LR`n A[Open document] --> B{Theme selected?}`n B -->|Yes| C[Match colors and font]`n B -->|System| D[Follow system theme]`n C --> E[Read comfortably]`n D --> E"
    sequence = "sequenceDiagram`n participant A as Reader`n participant B as Renderer`n A->>B: Open document`n Note over A,B: Match the reader theme`n B-->>A: Display diagram"
}
foreach ($name in $fixtures.Keys) {
    [IO.File]::WriteAllText((Join-Path $outputPath "$name.mmd"), $fixtures[$name])
}
$rows = [Collections.Generic.List[object]]::new()
# Round zero warms filesystem caches and is deliberately excluded.
for ($run = 0; $run -le $Runs; $run++) {
    foreach ($name in $fixtures.Keys) {
        foreach ($variant in ($variants | Sort-Object { Get-Random })) {
            $info = [Diagnostics.ProcessStartInfo]::new($variant.exe)
            $info.UseShellExecute = $false
            $info.CreateNoWindow = $true
            $info.ArgumentList.Add('--internal-mermaid')
            $info.ArgumentList.Add((Join-Path $outputPath "$name.mmd"))
            $info.ArgumentList.Add((Join-Path $outputPath "$name-$($variant.name).png"))
            if ($null -ne $variant.appearance) {
                $info.ArgumentList.Add(($variant.appearance | ConvertTo-Json -Compress))
            }
            $watch = [Diagnostics.Stopwatch]::StartNew()
            $process = [Diagnostics.Process]::Start($info)
            try {
                if (!$process.WaitForExit(15000)) {
                    $process.Kill()
                    throw 'Mermaid helper timed out'
                }
                $watch.Stop()
                if ($process.ExitCode -ne 0) { throw "Mermaid helper failed: $($variant.name)" }
                if ($run -gt 0) {
                    $rows.Add([pscustomobject]@{run=$run; fixture=$name; variant=$variant.name; helper_ms=$watch.Elapsed.TotalMilliseconds})
                }
            } finally { $process.Dispose() }
        }
    }
}
$rows | Export-Csv -NoTypeInformation -LiteralPath (Join-Path $outputPath 'mermaid.csv')
$rows | Group-Object fixture,variant | ForEach-Object {
    $values = @($_.Group.helper_ms | Sort-Object)
    [pscustomobject]@{case=$_.Name; median_ms=$values[[int][Math]::Floor($values.Count/2)]}
} | Format-Table
