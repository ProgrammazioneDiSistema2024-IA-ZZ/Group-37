[Setup]
; Nome del setup
AppName=Emergency Backup
AppVersion=1.0
;DefaultDirName={pf}\EmergencyBackup
DefaultDirName={userdesktop}\EmergencyBackup
DefaultGroupName=Emergency Backup
AllowNoIcons=yes
OutputDir=.
OutputBaseFilename=BackupAppInstaller
Compression=lzma
SolidCompression=yes
PrivilegesRequired=admin

[Files]
; Specifica i file da installare
Source: "EmergencyBackup.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "initialize_app.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "popup.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "assets\*"; DestDir: "{app}\assets"; Flags: ignoreversion recursesubdirs createallsubdirs
Source: "assets\icona.ico"; DestDir: "{app}\assets"; Flags: ignoreversion

[Icons]
; Crea solo il collegamento per initialize_app.exe con l'icona personalizzata
Name: "{group}\Initializer"; Filename: "{app}\initialize_app.exe"; IconFilename: "{app}\assets\icona.ico"
Name: "{userdesktop}\Emergency Backup Initializer"; Filename: "{app}\initialize_app.exe"; IconFilename: "{app}\assets\icona.ico"

[Registry]
; Aggiunge la voce di avvio automatico per EmergencyBackup.exe
Root: HKCU; Subkey: "Software\Microsoft\Windows\CurrentVersion\Run"; ValueType: string; ValueData: """{app}\EmergencyBackup.exe"""; Flags: uninsdeletevalue

[Run]
; Avvia `initialize_app.exe` dopo l'installazione
Filename: "{app}\initialize_app.exe"; Description: "Start Emergency Backup Initializer"; Flags: runascurrentuser nowait postinstall

[UninstallDelete]
; Rimuove i file e le directory durante la disinstallazione
Type: filesandordirs; Name: "{app}\assets\*"
Type: filesandordirs; Name: "{app}"

[Icons]
; Rimuove solo il collegamento per initialize_app.exe durante la disinstallazione
Name: "{group}\Initializer"; Filename: "{app}\initialize_app.exe"; IconFilename: "{app}\assets\icona.ico"
Name: "{userdesktop}\Emergency Backup Initializer"; Filename: "{app}\initialize_app.exe"; IconFilename: "{app}\assets\icona.ico"

[Registry]
; Rimuove la voce di avvio automatico e la voce di disinstallazione
Root: HKCU; Subkey: "Software\Microsoft\Windows\CurrentVersion\Run"; Flags: uninsdeletevalue
Root: HKLM; Subkey: "Software\Microsoft\Windows\CurrentVersion\Uninstall\Emergency Backup"; Flags: uninsdeletekey
