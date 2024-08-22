[Setup]
; Nome del setup
AppName=Emergency Backup
AppVersion=1.0
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
Source: "main_app.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "initialize_app.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "assets\*"; DestDir: "{app}\assets"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
; Crea i collegamenti nel menu Start e sul desktop
Name: "{group}\Main"; Filename: "{app}\main_app.exe"
Name: "{group}\Initializer"; Filename: "{app}\initialize_app.exe"
Name: "{userdesktop}\Emergency Backup Main"; Filename: "{app}\main_app.exe"
Name: "{userdesktop}\Emergency Backup Initializer"; Filename: "{app}\initialize_app.exe"

[Registry]
; Aggiunge la voce di avvio automatico per main_app.exe
Root: HKCU; Subkey: "Software\Microsoft\Windows\CurrentVersion\Run"; ValueType: string; ValueData: """{app}\main_app.exe"""; Flags: uninsdeletevalue

[Run]
; Avvia `initialize_app.exe` dopo l'installazione
Filename: "{app}\initialize_app.exe"; Description: "Start Emergency Backup Initializer"; Flags: nowait postinstall

[UninstallDelete]
; Rimuove i file e le directory durante la disinstallazione
Type: filesandordirs; Name: "{app}\assets\*"
Type: filesandordirs; Name: "{app}"

[Icons]
; Rimuove i collegamenti durante la disinstallazione
Name: "{group}\Main"; Filename: "{app}\main_app.exe"; 
Name: "{group}\Initializer"; Filename: "{app}\initialize_app.exe"; 
Name: "{userdesktop}\Emergency Backup Main"; Filename: "{app}\main_app.exe"; 
Name: "{userdesktop}\Emergency Backup Initializer"; Filename: "{app}\initialize_app.exe";

[Registry]
; Rimuove la voce di avvio automatico e la voce di disinstallazione
Root: HKCU; Subkey: "Software\Microsoft\Windows\CurrentVersion\Run"; Flags: uninsdeletevalue
Root: HKLM; Subkey: "Software\Microsoft\Windows\CurrentVersion\Uninstall\Emergency Backup"; Flags: uninsdeletekey
