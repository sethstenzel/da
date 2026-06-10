#Requires -Version 5.1
$profiles = @(
    "$HOME\Documents\WindowsPowerShell\Microsoft.PowerShell_profile.ps1",
    "$HOME\Documents\PowerShell\Microsoft.PowerShell_profile.ps1"
)

foreach ($p in $profiles) {
    if (-not (Test-Path $p)) { continue }
    $lines = Get-Content $p | Where-Object { $_ -notmatch 'function dacd' }
    $lines | Set-Content $p -Encoding UTF8
    Write-Host "Removed 'dacd' from $p"
}
