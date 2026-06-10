; da — Directory Alias Manager
; NSIS installer script
; Build: makensis da.nsi  (run from the installer\ directory)
; Requires: NSIS 3.x  —  winget install NSIS.NSIS

Unicode True

!include "MUI2.nsh"
!include "Sections.nsh"

; ---- metadata ---------------------------------------------------------------

!define APPNAME   "da"
!define PUBLISHER "Seth Stenzel"
!define VERSION   "0.4.3"
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
!insertmacro MUI_PAGE_COMPONENTS
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

!insertmacro MUI_LANGUAGE "English"

; ---- section descriptions ---------------------------------------------------

LangString DESC_SEC_MAIN  ${LANG_ENGLISH} "Install da.exe and add its directory to your user PATH."
LangString DESC_SEC_SHELL ${LANG_ENGLISH} "Add a 'dacd <alias>' function to your PowerShell profile so you can change directories directly."

; ---- install ----------------------------------------------------------------

Section "da (required)" SEC_MAIN
  SectionIn RO
  SetOutPath "$INSTDIR"
  File "${BINARY}"
  File "path_add.ps1"
  File "path_remove.ps1"
  File "setup_shell.ps1"
  File "remove_shell.ps1"

  ; Write registry entries first so path_add.ps1 can read InstallDir
  WriteRegStr   HKCU "Software\${APPNAME}"  "InstallDir"     "$INSTDIR"
  WriteRegStr   HKCU "${UNINSTKEY}" "DisplayName"     "${APPNAME} - Directory Alias Manager"
  WriteRegStr   HKCU "${UNINSTKEY}" "DisplayVersion"  "${VERSION}"
  WriteRegStr   HKCU "${UNINSTKEY}" "Publisher"       "${PUBLISHER}"
  WriteRegStr   HKCU "${UNINSTKEY}" "InstallLocation" "$INSTDIR"
  WriteRegStr   HKCU "${UNINSTKEY}" "UninstallString" '"$INSTDIR\uninstall.exe"'
  WriteRegDWORD HKCU "${UNINSTKEY}" "NoModify"        1
  WriteRegDWORD HKCU "${UNINSTKEY}" "NoRepair"        1

  ; Append INSTDIR to user PATH via PowerShell (no 1024-char registry limit)
  nsExec::Exec '"powershell" -NonInteractive -NoProfile -ExecutionPolicy Bypass -File "$INSTDIR\path_add.ps1"'

  WriteUninstaller "$INSTDIR\uninstall.exe"
SectionEnd

Section "Add 'dacd' function to PowerShell profile" SEC_SHELL
  nsExec::Exec '"powershell" -NonInteractive -NoProfile -ExecutionPolicy Bypass -File "$INSTDIR\setup_shell.ps1"'
  WriteRegDWORD HKCU "Software\${APPNAME}" "ShellIntegration" 1
SectionEnd

!insertmacro MUI_FUNCTION_DESCRIPTION_BEGIN
  !insertmacro MUI_DESCRIPTION_TEXT ${SEC_MAIN}  $(DESC_SEC_MAIN)
  !insertmacro MUI_DESCRIPTION_TEXT ${SEC_SHELL} $(DESC_SEC_SHELL)
!insertmacro MUI_FUNCTION_DESCRIPTION_END

; ---- uninstall --------------------------------------------------------------

Section "Uninstall"
  ; Remove INSTDIR from user PATH before deleting registry keys
  nsExec::Exec '"powershell" -NonInteractive -NoProfile -ExecutionPolicy Bypass -File "$INSTDIR\path_remove.ps1"'

  ; Remove dacd from PowerShell profile if shell integration was installed
  ReadRegDWORD $0 HKCU "Software\${APPNAME}" "ShellIntegration"
  ${If} $0 == 1
    nsExec::Exec '"powershell" -NonInteractive -NoProfile -ExecutionPolicy Bypass -File "$INSTDIR\remove_shell.ps1"'
  ${EndIf}

  Delete "$INSTDIR\da.exe"
  Delete "$INSTDIR\path_add.ps1"
  Delete "$INSTDIR\path_remove.ps1"
  Delete "$INSTDIR\setup_shell.ps1"
  Delete "$INSTDIR\remove_shell.ps1"
  Delete "$INSTDIR\uninstall.exe"
  RMDir  "$INSTDIR"

  DeleteRegKey HKCU "Software\${APPNAME}"
  DeleteRegKey HKCU "${UNINSTKEY}"
SectionEnd
