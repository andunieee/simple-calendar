; Built by the release workflow: iscc /DVersion=0.2.0 /DBundle=<dir with simple-calendar.exe> /DOutput=<dir> simple-calendar.iss
[Setup]
AppId={{3F6A1D52-8B7E-4C0A-9E21-5C1A7D0E0002}
AppName=Simple Calendar
AppVersion={#Version}
AppPublisher=andunie
DefaultDirName={autopf}\Simple Calendar
DefaultGroupName=Simple Calendar
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
OutputDir={#Output}
OutputBaseFilename=simple-calendar-windows-x64-setup-v{#Version}
UninstallDisplayIcon={app}\simple-calendar.exe
Compression=lzma2
SolidCompression=yes
WizardStyle=modern

[Files]
Source: "{#Bundle}\*"; DestDir: "{app}"; Flags: recursesubdirs createallsubdirs ignoreversion

[Icons]
Name: "{group}\Simple Calendar"; Filename: "{app}\simple-calendar.exe"
Name: "{autodesktop}\Simple Calendar"; Filename: "{app}\simple-calendar.exe"; Tasks: desktopicon

[Tasks]
Name: "desktopicon"; Description: "Create a desktop shortcut"; Flags: unchecked

[Run]
Filename: "{app}\simple-calendar.exe"; Description: "Launch Simple Calendar"; Flags: nowait postinstall skipifsilent
