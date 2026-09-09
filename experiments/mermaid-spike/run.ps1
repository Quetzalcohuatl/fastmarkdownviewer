param([string]$BinaryDirectory = '.\target\mermaid-spike', [string]$OutputDirectory = '.\target\mermaid-results')
$ErrorActionPreference = 'Stop'
New-Item -ItemType Directory -Force $OutputDirectory | Out-Null
$rows = foreach ($variant in @('mmdr', 'merman')) {
    $binary = (Resolve-Path (Join-Path $BinaryDirectory "$variant.exe")).Path
    foreach ($fixture in (Get-ChildItem (Join-Path $PSScriptRoot 'corpus') -Filter '*.mmd')) {
        $svg = [IO.Path]::GetFullPath((Join-Path $OutputDirectory "$variant-$($fixture.BaseName).svg"))
        $stdout = "$svg.stdout.txt"
        $stderr = "$svg.stderr.txt"
        $process = Start-Process $binary -ArgumentList @(('"' + $fixture.FullName + '"'), ('"' + $svg + '"')) -WindowStyle Hidden -PassThru -RedirectStandardOutput $stdout -RedirectStandardError $stderr
        $finished = $process.WaitForExit(10000)
        if (!$finished) { Stop-Process -Id $process.Id; $process.WaitForExit() }
        $process.Refresh()
        [pscustomobject]@{renderer=$variant;fixture=$fixture.Name;status=if(!$finished){'timeout'}elseif($process.ExitCode -eq 0){'ok'}elseif($process.ExitCode -eq 2){'error'}else{'aborted'};exit_code=$process.ExitCode;output=([IO.File]::ReadAllText($stdout)).Trim();error=([IO.File]::ReadAllText($stderr)).Trim()}
        $process.Dispose()
    }
}
$rows | Export-Csv -NoTypeInformation (Join-Path $OutputDirectory 'results.csv')
$rows | Format-Table renderer,fixture,status,output
