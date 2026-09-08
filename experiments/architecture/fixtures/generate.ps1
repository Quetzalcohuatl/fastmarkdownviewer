[CmdletBinding()]
param([string] $OutputDirectory = (Join-Path $PSScriptRoot 'generated'))

$ErrorActionPreference = 'Stop'
$output = [System.IO.Path]::GetFullPath($OutputDirectory)
New-Item -ItemType Directory -Path $output -Force | Out-Null

$seed = @'
# Architecture fixture

This paragraph contains **strong**, *emphasis*, `inline code`, Unicode Ελληνικά 日本語 😀, and a [relative link](next.md).

| Item | Value |
|:--|--:|
| latency | measured |
| renderer | candidate |

- [x] task
- [ ] another task

Inline math $x^2+y_1$.

$$\frac{-b \pm \sqrt{b^2-4ac}}{2a}$$

```rust
fn plain_code() -> &'static str { "never highlighted synchronously" }
```

> [!NOTE]
> Resource work stays behind the visible viewport.

'@

function Write-SizedFixture([string] $Name, [int] $TargetBytes) {
    $builder = [System.Text.StringBuilder]::new($TargetBytes + 512)
    $seedBytes = [System.Text.Encoding]::UTF8.GetByteCount($seed)
    $currentBytes = 0
    while ($currentBytes -lt $TargetBytes) {
        [void] $builder.Append($seed)
        $currentBytes += $seedBytes
    }
    $text = $builder.ToString()
    [System.IO.File]::WriteAllText((Join-Path $output $Name), $text, [System.Text.UTF8Encoding]::new($false))
}

Write-SizedFixture 'ordinary-5k.md' (5 * 1024)
Write-SizedFixture 'gfm-math-images-100k.md' (100 * 1024)
Write-SizedFixture 'stress-2m.md' (2 * 1024 * 1024)
$images = [System.Text.StringBuilder]::new("# Image cache workload`n`n")
for ($index = 0; $index -lt 24; $index++) {
    $imagePath = Join-Path $output "image-$index.svg"
    [System.IO.File]::WriteAllText($imagePath, '<svg xmlns="http://www.w3.org/2000/svg" width="512" height="512"><rect width="512" height="512" fill="#5099bb"/></svg>', [System.Text.UTF8Encoding]::new($false))
    $uri = [System.Uri]::new($imagePath).AbsoluteUri
    [void] $images.AppendLine("![Image $index]($uri)`n`n")
}
[System.IO.File]::WriteAllText((Join-Path $output 'images.md'), $images.ToString(), [System.Text.UTF8Encoding]::new($false))
Get-ChildItem -LiteralPath $output | Select-Object Name, Length
