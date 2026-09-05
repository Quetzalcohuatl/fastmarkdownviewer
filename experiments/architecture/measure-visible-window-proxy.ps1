[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)] [hashtable] $Variants,
    [Parameter(Mandatory = $true)] [string[]] $Fixtures,
    [int] $Runs = 30,
    [string] $OutputCsv = (Join-Path $PSScriptRoot 'raw-visible-window-proxy.csv')
)

$ErrorActionPreference = 'Stop'
$rows = [System.Collections.Generic.List[object]]::new()
$cases = foreach ($variant in $Variants.GetEnumerator()) {
    foreach ($fixture in $Fixtures) {
        [pscustomobject]@{ Variant = $variant.Key; Executable = (Resolve-Path -LiteralPath $variant.Value).Path; Fixture = (Resolve-Path -LiteralPath $fixture).Path }
    }
}

for ($run = 1; $run -le $Runs; $run++) {
    foreach ($case in ($cases | Sort-Object { Get-Random })) {
        $watch = [System.Diagnostics.Stopwatch]::StartNew()
        $process = Start-Process -FilePath $case.Executable -ArgumentList @("`"$($case.Fixture)`"") -PassThru
        try {
            [void] $process.WaitForInputIdle(10000)
            $deadline = [DateTime]::UtcNow.AddSeconds(10)
            do {
                $process.Refresh()
                if ($process.MainWindowHandle -ne [IntPtr]::Zero) { break }
                Start-Sleep -Milliseconds 5
            } while ([DateTime]::UtcNow -lt $deadline -and -not $process.HasExited)
            $watch.Stop()
            Start-Sleep -Milliseconds 250
            $process.Refresh()
            $rows.Add([pscustomobject]@{
                run = $run
                variant = $case.Variant
                fixture = [System.IO.Path]::GetFileName($case.Fixture)
                visible_window_proxy_ms = [math]::Round($watch.Elapsed.TotalMilliseconds, 3)
                first_idle_working_set_bytes = $process.WorkingSet64
                peak_working_set_bytes = $process.PeakWorkingSet64
                executable_bytes = (Get-Item -LiteralPath $case.Executable).Length
            })
        }
        finally {
            if (-not $process.HasExited) {
                [void] $process.CloseMainWindow()
                if (-not $process.WaitForExit(2000)) { Stop-Process -Id $process.Id -Force }
            }
            $process.Dispose()
        }
    }
}

$rows | Export-Csv -NoTypeInformation -Encoding utf8 -LiteralPath $OutputCsv
Write-Warning 'visible_window_proxy_ms is not first-content present timing. Use ETW for published startup claims.'
