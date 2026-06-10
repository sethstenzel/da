#Requires -Version 5.1
$fn = 'function dacd { if (-not $args[0]) { Write-Host "Usage: dacd <alias>"; return }; $path = da $args[0]; if ($LASTEXITCODE -eq 0) { Set-Location $path } }'

$profiles = @(
    "$HOME\Documents\WindowsPowerShell\Microsoft.PowerShell_profile.ps1",
    "$HOME\Documents\PowerShell\Microsoft.PowerShell_profile.ps1"
)

foreach ($p in $profiles) {
    $dir = Split-Path $p
    if (-not (Test-Path $dir)) { New-Item -ItemType Directory -Path $dir -Force | Out-Null }
    if (-not (Test-Path $p))   { New-Item -ItemType File -Path $p -Force | Out-Null }

    $content = Get-Content $p -Raw -ErrorAction SilentlyContinue
    if ($content -match 'function dacd') {
        $updated = ($content -split "`n" | Where-Object { $_ -notmatch 'function dacd' }) -join "`n"
        Set-Content $p -Value $updated.TrimEnd() -Encoding UTF8
    }

    Add-Content -Path $p -Value "`n$fn"
    Write-Host "Installed 'dacd' in $p"
}
