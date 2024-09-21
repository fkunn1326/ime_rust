@REM reg add HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Themes\Personalize /v AppsUseLightTheme /t REG_DWORD /d 00000000 /f
@REM reg add HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Themes\Personalize /v SystemUsesLightTheme /t REG_DWORD /d 00000000 /f
@REM taskkill /im explorer.exe /f
@REM start explorer.exe
C:\Users\WDAGUtilityAccount\Desktop\wsb\VC_redist.x64.exe /install /passive /norestart
C:\Users\WDAGUtilityAccount\Desktop\wsb\MicrosoftEdgeWebView2RuntimeInstallerX64.exe
regsvr32.exe C:\Users\WDAGUtilityAccount\Desktop\IME\ime.dll /s
start C:\Users\WDAGUtilityAccount\Desktop\Service\azookey-service.exe
notepad.exe