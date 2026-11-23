param(
    [Parameter(Mandatory)][string]$HostExe,
    [string]$TargetRoot = (Join-Path $PSScriptRoot '..\target\installer')
)
$ErrorActionPreference = 'Stop'
$hostItem = Get-Item -LiteralPath $HostExe
if ($hostItem.PSIsContainer -or $hostItem.Name -ine 'PanopticonHub.exe') {
    throw 'HostExe должен указывать на готовый PanopticonHub.exe.'
}
$package = Get-Content -LiteralPath (Join-Path $PSScriptRoot 'Cargo.toml') -Raw
$version = [regex]::Match($package, '(?m)^version\s*=\s*"([^"]+)"').Groups[1].Value
$hostPackage = Get-Content -LiteralPath (Join-Path $PSScriptRoot '../host/Cargo.toml') -Raw
$hostVersion = [regex]::Match($hostPackage, '(?m)^version\s*=\s*"([^"]+)"').Groups[1].Value
if ($version -cnotmatch '^\d+\.\d+\.\d+$' -or $hostVersion -cnotmatch '^\d+\.\d+\.\d+$' -or
    [version]$hostVersion -gt [version]$version -or $hostItem.VersionInfo.ProductVersion -cne $hostVersion -or
    $hostItem.VersionInfo.ProductName -cne 'Panopticon Hub') {
    throw "Ожидается PanopticonHub.exe версии $hostVersion из host/Cargo.toml."
}
$target = $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($TargetRoot)
$cargo = Get-Command cargo.exe -ErrorAction SilentlyContinue
$cargoExe = if ($cargo) { $cargo.Source } else { Join-Path $env:USERPROFILE '.cargo\bin\cargo.exe' }
$originalHost = $env:PANOPTICON_HOST_EXE
$originalFixture = $env:PANOPTICON_SETUP_TEST_PAYLOAD
$originalUninstallMode = $env:PANOPTICON_SETUP_UNINSTALL_ONLY
$originalUninstaller = $env:PANOPTICON_SETUP_UNINSTALL_EXE
$originalTarget = $env:CARGO_TARGET_DIR
$manifest = Join-Path $PSScriptRoot '..\Cargo.toml'
$lightTarget = Join-Path $target 'uninstaller'
$uninstallerName = 'Uninstall Panopticon Hub.exe'
$uninstaller = Join-Path $lightTarget "release\$uninstallerName"
try {
    $env:PANOPTICON_HOST_EXE = $null
    $env:PANOPTICON_SETUP_TEST_PAYLOAD = $null
    $env:PANOPTICON_SETUP_UNINSTALL_EXE = $null
    $env:PANOPTICON_SETUP_UNINSTALL_ONLY = '1'
    $env:CARGO_TARGET_DIR = $lightTarget
    & $cargoExe build --locked --manifest-path $manifest --release -p panopticon-setup
    if ($LASTEXITCODE -ne 0) { throw 'Сборка деинсталлятора завершилась ошибкой.' }
    Copy-Item -LiteralPath (Join-Path $lightTarget 'release\PanopticonHubSetup.exe') -Destination $uninstaller
    $uninstallVersion = (Get-Item -LiteralPath $uninstaller).VersionInfo
    if ($uninstallVersion.ProductVersion -cne $version -or
        $uninstallVersion.OriginalFilename -cne $uninstallerName) {
        throw 'Метаданные собранного деинсталлятора не совпадают с версией и именем пакета.'
    }
    $env:PANOPTICON_SETUP_UNINSTALL_ONLY = $null
    $env:PANOPTICON_SETUP_UNINSTALL_EXE = $uninstaller
    $env:PANOPTICON_HOST_EXE = $hostItem.FullName
    $env:CARGO_TARGET_DIR = $target
    & $cargoExe build --locked --manifest-path $manifest --release -p panopticon-setup
    if ($LASTEXITCODE -ne 0) { throw 'Сборка установщика завершилась ошибкой.' }
} finally {
    $env:PANOPTICON_HOST_EXE = $originalHost
    $env:PANOPTICON_SETUP_TEST_PAYLOAD = $originalFixture
    $env:PANOPTICON_SETUP_UNINSTALL_ONLY = $originalUninstallMode
    $env:PANOPTICON_SETUP_UNINSTALL_EXE = $originalUninstaller
    $env:CARGO_TARGET_DIR = $originalTarget
}
$setup = Join-Path $target 'release\PanopticonHubSetup.exe'
$setupVersion = (Get-Item -LiteralPath $setup).VersionInfo
if ($setupVersion.ProductVersion -cne $version -or $setupVersion.OriginalFilename -cne 'PanopticonHubSetup.exe') {
    throw 'Метаданные собранного установщика не совпадают с версией и именем пакета.'
}
$hash = (Get-FileHash -LiteralPath $hostItem.FullName -Algorithm SHA256).Hash
$payload = & $setup --check-payload | Out-String
if ($LASTEXITCODE -ne 0 -or $payload -notmatch "payload: $($hostItem.Length) bytes, SHA256 $hash, fixture=false") {
    throw "Встроенный файл не совпадает с PanopticonHub.exe: $payload"
}
$payload.Trim()
Get-FileHash -LiteralPath $hostItem.FullName, $uninstaller, $setup -Algorithm SHA256

