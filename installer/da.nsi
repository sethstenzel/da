; da — Directory Alias Manager
; NSIS installer script
; Build: makensis da.nsi  (run from the installer\ directory)
; Requires: NSIS 3.x  —  winget install NSIS.NSIS

Unicode True

!include "MUI2.nsh"
!include "WinMessages.nsh"
!include "LogicLib.nsh"

; ---- metadata ---------------------------------------------------------------

!define APPNAME   "da"
!define PUBLISHER "Seth Stenzel"
!define VERSION   "0.1.0"
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

; ---- string replace helper (uninstall only, for PATH cleanup) ---------------
; Usage: Push string / Push find / Push replace / Call un.ReplaceInString / Pop result
; Clobbers $R0-$R4. Saves/restores $0.

Function un.ReplaceInString
  Pop  $R2          ; replace
  Pop  $R1          ; find
  Pop  $R0          ; input string
  Push $0           ; save $0 (used as scratch below)
  StrLen $R3 $R1    ; length of find string
  StrCpy $R4 $R0    ; $R4 = remaining text
  StrCpy $R0 ""     ; $R0 = accumulated result

  ${If} $R3 == 0
    Pop  $0
    Push $R4
    Return
  ${EndIf}

  ris_loop:
    StrLen $0 $R4
    ${If} $0 == 0
      Goto ris_done
    ${EndIf}
    StrCpy $0 $R4 $R3         ; grab len(find) chars from front of remaining
    StrCmp $0 $R1 ris_match   ; if they match, replace
    StrCpy $0 $R4 1           ; otherwise take one char
    StrCpy $R0 "$R0$0"        ; append it to result
    StrCpy $R4 $R4 "" 1       ; advance remaining by 1
    Goto ris_loop

  ris_match:
    StrCpy $R0 "$R0$R2"       ; append replacement (empty = delete)
    StrCpy $R4 $R4 "" $R3     ; advance remaining past matched text
    Goto ris_loop

  ris_done:
  Pop  $0                     ; restore $0
  Push $R0                    ; return result
FunctionEnd

; ---- install ----------------------------------------------------------------

Section "da" SEC_MAIN
  SetOutPath "$INSTDIR"
  File "${BINARY}"

  ; Append install dir to user PATH (no duplicate check — uninstall cleans all occurrences)
  ReadRegStr $0 HKCU "Environment" "PATH"
  ${If} $0 == ""
    WriteRegExpandStr HKCU "Environment" "PATH" "$INSTDIR"
  ${Else}
    WriteRegExpandStr HKCU "Environment" "PATH" "$0;$INSTDIR"
  ${EndIf}
  SendMessage ${HWND_BROADCAST} ${WM_WININICHANGE} 0 "STR:Environment" /TIMEOUT=5000

  WriteUninstaller "$INSTDIR\uninstall.exe"

  WriteRegStr   HKCU "${UNINSTKEY}" "DisplayName"     "${APPNAME} - Directory Alias Manager"
  WriteRegStr   HKCU "${UNINSTKEY}" "DisplayVersion"  "${VERSION}"
  WriteRegStr   HKCU "${UNINSTKEY}" "Publisher"       "${PUBLISHER}"
  WriteRegStr   HKCU "${UNINSTKEY}" "InstallLocation" "$INSTDIR"
  WriteRegStr   HKCU "${UNINSTKEY}" "UninstallString" '"$INSTDIR\uninstall.exe"'
  WriteRegDWORD HKCU "${UNINSTKEY}" "NoModify"        1
  WriteRegDWORD HKCU "${UNINSTKEY}" "NoRepair"        1
  WriteRegStr   HKCU "Software\${APPNAME}" "InstallDir" "$INSTDIR"
SectionEnd

; ---- uninstall --------------------------------------------------------------

Section "Uninstall"
  ; Strip all occurrences of INSTDIR from user PATH
  ; Three passes handle: mid-path (;dir), end-of-path (;dir), start-of-path (dir;), and sole entry (dir)
  ReadRegStr $0 HKCU "Environment" "PATH"

  Push $0
  Push ";$INSTDIR"
  Push ""
  Call un.ReplaceInString
  Pop $0

  Push $0
  Push "$INSTDIR;"
  Push ""
  Call un.ReplaceInString
  Pop $0

  Push $0
  Push "$INSTDIR"
  Push ""
  Call un.ReplaceInString
  Pop $0

  WriteRegExpandStr HKCU "Environment" "PATH" "$0"
  SendMessage ${HWND_BROADCAST} ${WM_WININICHANGE} 0 "STR:Environment" /TIMEOUT=5000

  Delete "$INSTDIR\da.exe"
  Delete "$INSTDIR\uninstall.exe"
  RMDir  "$INSTDIR"

  DeleteRegKey HKCU "Software\${APPNAME}"
  DeleteRegKey HKCU "${UNINSTKEY}"
SectionEnd
