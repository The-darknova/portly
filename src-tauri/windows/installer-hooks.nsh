; NSIS installer hooks to register and clean up Portly's `bin` directory in User PATH.
; Uses PowerShell via nsExec so arbitrary-length PATH values are handled without
; NSIS's 1024-byte ReadRegStr truncation, while preserving REG_EXPAND_SZ and
; broadcasting WM_SETTINGCHANGE.
; Note: Literal `$` for PowerShell variables is escaped as `$$` in NSIS strings.

!macro NSIS_HOOK_POSTINSTALL
  System::Call 'Kernel32::SetEnvironmentVariable(t "PORTLY_BIN_DIR", t "$INSTDIR\bin")'
  nsExec::ExecToLog `"$SYSDIR\WindowsPowerShell\v1.0\powershell.exe" -NoProfile -NonInteractive -ExecutionPolicy Bypass -Command "$$target = $$env:PORTLY_BIN_DIR; $$cur = [Environment]::GetEnvironmentVariable('Path', 'User'); $$parts = @(); if ($$cur) { $$parts = $$cur -split ';' | Where-Object { $$_ -and ($$_.TrimEnd('\/') -ne $$target.TrimEnd('\/')) } }; $$parts += $$target; [Environment]::SetEnvironmentVariable('Path', ($$parts -join ';'), 'User')"`
  Pop $0
!macroend

!macro NSIS_HOOK_POSTUNINSTALL
  System::Call 'Kernel32::SetEnvironmentVariable(t "PORTLY_BIN_DIR", t "$INSTDIR\bin")'
  nsExec::ExecToLog `"$SYSDIR\WindowsPowerShell\v1.0\powershell.exe" -NoProfile -NonInteractive -ExecutionPolicy Bypass -Command "$$target = $$env:PORTLY_BIN_DIR; $$cur = [Environment]::GetEnvironmentVariable('Path', 'User'); if ($$cur) { $$parts = $$cur -split ';' | Where-Object { $$_ -and ($$_.TrimEnd('\/') -ne $$target.TrimEnd('\/')) } }; [Environment]::SetEnvironmentVariable('Path', ($$parts -join ';'), 'User')"`
  Pop $0
!macroend
