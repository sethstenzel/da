#Requires -Version 5.1
$installDir = (Get-ItemProperty -Path "HKCU:\Software\da" -Name "InstallDir" -ErrorAction SilentlyContinue).InstallDir
if (-not $installDir) { exit 0 }

$p = [Environment]::GetEnvironmentVariable("PATH", "User")
$np = ($p -split ";" | Where-Object { $_ -ne $installDir }) -join ";"
[Environment]::SetEnvironmentVariable("PATH", $np, "User")
