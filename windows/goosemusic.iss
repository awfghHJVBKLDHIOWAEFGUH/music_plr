#ifndef MyAppVersion
  #define MyAppVersion "1.5.1"
#endif

[Setup]
AppId={{DDA3B2E4-8B1A-4B4B-9D2D-7A1C9A5E2F10}
AppName=Goosemusic
AppVersion={#MyAppVersion}
AppPublisher=GooseMusic TinySkia Fork
DefaultDirName={localappdata}\Programs\Goosemusic
DefaultGroupName=Goosemusic
DisableProgramGroupPage=yes
PrivilegesRequired=lowest
ArchitecturesAllowed=x64
ArchitecturesInstallIn64BitMode=x64
OutputDir=..\dist
OutputBaseFilename=goosemusic-setup-x86_64-pc-windows-msvc
SetupIconFile=app.ico
UninstallDisplayIcon={app}\goosemusic.exe
LicenseFile=..\LICENSE
WizardStyle=modern
Compression=lzma2
SolidCompression=yes

[Tasks]
Name: "desktopicon"; Description: "Create a desktop shortcut"; GroupDescription: "Additional icons:"; Flags: unchecked

[Files]
Source: "..\target\x86_64-pc-windows-msvc\release\goosemusic.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{autoprograms}\Goosemusic"; Filename: "{app}\goosemusic.exe"
Name: "{autodesktop}\Goosemusic"; Filename: "{app}\goosemusic.exe"; Tasks: desktopicon

[Run]
Filename: "{app}\goosemusic.exe"; Description: "Launch Goosemusic"; Flags: nowait postinstall skipifsilent
