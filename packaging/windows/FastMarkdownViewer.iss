#ifndef AppVersion
  #error AppVersion must be provided with /DAppVersion=x.y.z
#endif
#ifndef BuildRoot
  #error BuildRoot must be provided with /DBuildRoot=path
#endif
#ifndef OutputDir
  #error OutputDir must be provided with /DOutputDir=path
#endif
#ifndef IconFile
  #error IconFile must be provided with /DIconFile=path
#endif
#ifndef RepositoryUrl
  #define RepositoryUrl ""
#endif

[Setup]
AppId={{7E293586-4C95-4672-9342-671EC95E57A5}
AppName=FastMarkdownViewer
AppVersion={#AppVersion}
AppPublisher=FastMarkdownViewer contributors
#if RepositoryUrl != ""
AppPublisherURL={#RepositoryUrl}
AppSupportURL={#RepositoryUrl}/issues
AppUpdatesURL={#RepositoryUrl}/releases
#endif
DefaultDirName={localappdata}\Programs\FastMarkdownViewer
DefaultGroupName=FastMarkdownViewer
PrivilegesRequired=lowest
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
DisableProgramGroupPage=yes
DisableReadyMemo=yes
OutputDir={#OutputDir}
OutputBaseFilename=FastMarkdownViewer-Setup-{#AppVersion}
SetupIconFile={#IconFile}
UninstallDisplayIcon={app}\FastMarkdownViewer.exe
Compression=lzma2/max
SolidCompression=yes
WizardStyle=modern
ChangesAssociations=yes
CloseApplications=no
RestartApplications=no
VersionInfoVersion={#AppVersion}
VersionInfoDescription=FastMarkdownViewer per-user installer
LicenseFile={#SourcePath}\..\..\LICENSE-MIT

[Files]
Source: "{#BuildRoot}\FastMarkdownViewer.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#SourcePath}\..\..\README.md"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#SourcePath}\..\..\LICENSE-MIT"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#SourcePath}\..\..\LICENSE-APACHE"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#SourcePath}\..\..\THIRD_PARTY_NOTICES.md"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\FastMarkdownViewer"; Filename: "{app}\FastMarkdownViewer.exe"; WorkingDir: "{userdocs}"
Name: "{group}\Uninstall FastMarkdownViewer"; Filename: "{uninstallexe}"

[Registry]
Root: HKCU; Subkey: "Software\Classes\FastMarkdownViewer.Document"; ValueType: string; ValueName: ""; ValueData: "Markdown document"; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Classes\FastMarkdownViewer.Document\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\FastMarkdownViewer.exe,0"
Root: HKCU; Subkey: "Software\Classes\FastMarkdownViewer.Document\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\FastMarkdownViewer.exe"" ""%1"""
Root: HKCU; Subkey: "Software\Classes\.md\OpenWithProgids"; ValueType: string; ValueName: "FastMarkdownViewer.Document"; ValueData: ""; Flags: uninsdeletevalue
Root: HKCU; Subkey: "Software\Classes\.markdown\OpenWithProgids"; ValueType: string; ValueName: "FastMarkdownViewer.Document"; ValueData: ""; Flags: uninsdeletevalue

[Run]
Filename: "{app}\FastMarkdownViewer.exe"; Description: "Launch FastMarkdownViewer"; Flags: nowait postinstall skipifsilent
