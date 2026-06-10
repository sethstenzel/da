#Requires -Version 5.1
$installDir = (Get-ItemProperty -Path "HKCU:\Software\da" -Name "InstallDir" -ErrorAction SilentlyContinue).InstallDir
if (-not $installDir) { exit 1 }

$p = [Environment]::GetEnvironmentVariable("PATH", "User")
if (($p -split ";") -notcontains $installDir) {
    $np = if ($p) { "$p;$installDir" } else { $installDir }
    [Environment]::SetEnvironmentVariable("PATH", $np, "User")
}
