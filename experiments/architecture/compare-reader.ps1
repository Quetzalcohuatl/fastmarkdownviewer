[CmdletBinding()]
param(
    [Parameter(Mandatory)] [hashtable] $Variants,
    [Parameter(Mandatory)] [string[]] $Fixtures,
    [int] $Runs = 5,
    [int] $LayoutRuns = 3,
    [Parameter(Mandatory)] [string] $OutputDirectory
)
$ErrorActionPreference='Stop'
New-Item -ItemType Directory -Force -Path $OutputDirectory | Out-Null
$native=[Collections.Generic.List[object]]::new()
$layout=[Collections.Generic.List[object]]::new()
$cases=foreach($variant in $Variants.GetEnumerator()) { foreach($fixture in $Fixtures) { [pscustomobject]@{Name=$variant.Key;Exe=(Resolve-Path -LiteralPath $variant.Value.Exe).Path;Layout=(Resolve-Path -LiteralPath $variant.Value.Layout).Path;Fixture=(Resolve-Path -LiteralPath $fixture).Path} } }
for($run=1;$run -le $Runs;$run++) {
    foreach($case in ($cases | Sort-Object {Get-Random})) {
        $watch=[Diagnostics.Stopwatch]::StartNew()
        # Visible windows are required to measure the actual native rendering workload.
        $process=Start-Process -FilePath $case.Exe -ArgumentList ('"'+$case.Fixture+'"') -PassThru
        try {
            [void]$process.WaitForInputIdle(15000)
            $deadline=[DateTime]::UtcNow.AddSeconds(15)
            do { $process.Refresh(); if($process.HasExited){throw 'Viewer exited during startup'}; if($process.MainWindowHandle -ne [IntPtr]::Zero){break}; Start-Sleep -Milliseconds 10 } while([DateTime]::UtcNow -lt $deadline)
            if($process.MainWindowHandle -eq [IntPtr]::Zero){throw 'No native window within deadline'}
            $watch.Stop()
            Start-Sleep -Milliseconds 1500
            $process.Refresh(); $cpuBefore=$process.TotalProcessorTime.TotalMilliseconds
            $idleWatch=[Diagnostics.Stopwatch]::StartNew(); Start-Sleep -Milliseconds 1000
            $process.Refresh(); $idleWatch.Stop()
            $native.Add([pscustomobject]@{run=$run;variant=$case.Name;fixture=[IO.Path]::GetFileName($case.Fixture);visible_window_proxy_ms=$watch.Elapsed.TotalMilliseconds;idle_sample_ms=$idleWatch.Elapsed.TotalMilliseconds;idle_cpu_ms=$process.TotalProcessorTime.TotalMilliseconds-$cpuBefore;working_set_bytes=$process.WorkingSet64;private_bytes=$process.PrivateMemorySize64;peak_working_set_bytes=$process.PeakWorkingSet64;executable_bytes=(Get-Item -LiteralPath $case.Exe).Length})
        } finally { if(!$process.HasExited){[void]$process.CloseMainWindow(); if(!$process.WaitForExit(5000)){Stop-Process -Id $process.Id}}; $process.Dispose() }
    }
    Write-Output "Native sample round $run/$Runs complete"
}
$native | Export-Csv -NoTypeInformation -LiteralPath (Join-Path $OutputDirectory 'native.csv')
for($run=1;$run -le $LayoutRuns;$run++) {
    foreach($case in ($cases | Sort-Object {Get-Random})) {
        $info=[Diagnostics.ProcessStartInfo]::new($case.Layout)
        $info.ArgumentList.Add($case.Fixture); $info.UseShellExecute=$false; $info.CreateNoWindow=$true
        $info.RedirectStandardOutput=$true; $info.RedirectStandardInput=$true; $info.RedirectStandardError=$true
        $process=[Diagnostics.Process]::Start($info)
        $scrollMedian=$null; $scrollP95=$null
        try {
            while($null -ne ($line=$process.StandardOutput.ReadLine())) {
                if($line.StartsWith('SCROLL ')) { $values=$line.Substring(7).Split(','); $scrollMedian=[double]::Parse($values[0],[Globalization.CultureInfo]::InvariantCulture); $scrollP95=[double]::Parse($values[1],[Globalization.CultureInfo]::InvariantCulture); if([double]::Parse($values[2],[Globalization.CultureInfo]::InvariantCulture) -le 0){throw 'Scroll workload did not scroll'} }
                if($line.StartsWith('PHASE ')) {
                    $process.Refresh()
                    $layout.Add([pscustomobject]@{run=$run;variant=$case.Name;fixture=[IO.Path]::GetFileName($case.Fixture);phase=$line.Substring(6);working_set_bytes=$process.WorkingSet64;private_bytes=$process.PrivateMemorySize64;scroll_layout_median_ms=$scrollMedian;scroll_layout_p95_ms=$scrollP95})
                    $process.StandardInput.WriteLine(); $process.StandardInput.Flush()
                }
            }
            $process.WaitForExit(); if($process.ExitCode -ne 0){throw $process.StandardError.ReadToEnd()}
        } finally {if(!$process.HasExited){Stop-Process -Id $process.Id}; $process.Dispose()}
    }
    Write-Output "Layout/lifecycle sample round $run/$LayoutRuns complete"
}
$layout | Export-Csv -NoTypeInformation -LiteralPath (Join-Path $OutputDirectory 'layout-lifecycle.csv')
Write-Output 'Exploratory comparison complete: window timing is a proxy; layout timing excludes GPU/presentation.'
