; Focus Launcher installer hooks (NSIS, Tauri). Keep this file ASCII.

!macro NSIS_HOOK_PREINSTALL
  ; Update: the guardian service keeps its executable open, stop it first.
  nsExec::Exec 'sc.exe stop FocusGuardian'
  Sleep 1500
!macroend

!macro NSIS_HOOK_POSTINSTALL
  ; If it was installed, the service starts again with the new version.
  nsExec::Exec 'sc.exe start FocusGuardian'
!macroend

!macro NSIS_HOOK_PREUNINSTALL
  ; Nothing stays behind: no logon task, no service.
  nsExec::Exec 'schtasks.exe /Delete /F /TN "Focus Launcher"'
  nsExec::Exec '"$INSTDIR\guardian\focus-guardian.exe" uninstall'
!macroend
