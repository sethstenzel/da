#Requires -Version 5.1
Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$origin = Get-Location

try {
    Write-Host "Building release binary..."
    Set-Location "$PSScriptRoot\da"
    cargo build --release

    Write-Host "Building installer..."
    Set-Location "$PSScriptRoot\installer"
    & "C:\Program Files (x86)\NSIS\makensis.exe" da.nsi

    Write-Host "Done. Installer: $PSScriptRoot\installer\$(Get-ChildItem '*.exe' | Sort-Object LastWriteTime | Select-Object -Last 1 -ExpandProperty Name)"
} finally {
    Set-Location $origin
}
