; da — Directory Alias Manager
; NSIS installer script
; Build: makensis da.nsi  (run from the installer\ directory)
; Requires: NSIS 3.x  —  winget install NSIS.NSIS

Unicode True

!include "MUI2.nsh"

; ---- metadata ---------------------------------------------------------------

!define APPNAME   "da"
!define PUBLISHER "Seth Stenzel"
!define VERSION   "0.3.0"
!define BINARY    "..\da\target\release\da.exe"
!define UNINSTKEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}"

Name    "${APPNAME}"
OutFile "${APPNAME}-${VERSION}-installer.exe"

InstallDir       "$LOCALAPPDATA\Programs\${APPNAME}"
InstallDirRegKey HKCU "Software\${APPNAME}" "InstallDir"
RequestExecutionLevel user

; ---- pages ------------------------------------------------------------------

!define MUI_ABORTWARNING
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

!insertmacro MUI_LANGUAGE "English"

; ---- install ----------------------------------------------------------------

Section "da" SEC_MAIN
  SetOutPath "$INSTDIR"
  File "${BINARY}"
  File "path_add.ps1"
  File "path_remove.ps1"

  ; Write registry entries first so path_add.ps1 can read InstallDir
  WriteRegStr   HKCU "Software\${APPNAME}"  "InstallDir"     "$INSTDIR"
  WriteRegStr   HKCU "${UNINSTKEY}" "DisplayName"     "${APPNAME} - Directory Alias Manager"
  WriteRegStr   HKCU "${UNINSTKEY}" "DisplayVersion"  "${VERSION}"
  WriteRegStr   HKCU "${UNINSTKEY}" "Publisher"       "${PUBLISHER}"
  WriteRegStr   HKCU "${UNINSTKEY}" "InstallLocation" "$INSTDIR"
  WriteRegStr   HKCU "${UNINSTKEY}" "UninstallString" '"$INSTDIR\uninstall.exe"'
  WriteRegDWORD HKCU "${UNINSTKEY}" "NoModify"        1
  WriteRegDWORD HKCU "${UNINSTKEY}" "NoRepair"        1

  ; Append INSTDIR to user PATH via PowerShell (no 1024-char registry limit).
  ; path_add.ps1 reads InstallDir from HKCU\Software\da written above.
  nsExec::Exec '"powershell" -NonInteractive -NoProfile -ExecutionPolicy Bypass -File "$INSTDIR\path_add.ps1"'

  WriteUninstaller "$INSTDIR\uninstall.exe"
SectionEnd

; ---- uninstall --------------------------------------------------------------

Section "Uninstall"
  ; Remove INSTDIR from user PATH before deleting registry keys.
  ; path_remove.ps1 reads InstallDir from HKCU\Software\da.
  nsExec::Exec '"powershell" -NonInteractive -NoProfile -ExecutionPolicy Bypass -File "$INSTDIR\path_remove.ps1"'

  Delete "$INSTDIR\da.exe"
  Delete "$INSTDIR\path_add.ps1"
  Delete "$INSTDIR\path_remove.ps1"
  Delete "$INSTDIR\uninstall.exe"
  RMDir  "$INSTDIR"

  DeleteRegKey HKCU "Software\${APPNAME}"
  DeleteRegKey HKCU "${UNINSTKEY}"
SectionEnd
