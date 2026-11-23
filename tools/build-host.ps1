param([string]$TargetRoot = (Join-Path $PSScriptRoot '..\target'))
$ErrorActionPreference = 'Stop'
$workspace = Join-Path $PSScriptRoot '..\Cargo.toml'
$cargo = Get-Command cargo.exe -ErrorAction SilentlyContinue
$cargoExe = if ($cargo) { $cargo.Source } else { Join-Path $env:USERPROFILE '.cargo\bin\cargo.exe' }
$target = $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($TargetRoot)
$previousModule = $env:PANOPTICON_GAME_MODULE
$previousTarget = $env:CARGO_TARGET_DIR
try {
    $env:CARGO_TARGET_DIR = $target
    & $cargoExe build --locked --manifest-path $workspace --release -p panopticon-native
    if ($LASTEXITCODE -ne 0) { throw 'Сборка игрового модуля завершилась ошибкой.' }
    $env:PANOPTICON_GAME_MODULE = Join-Path $target 'release\panopticon_native.dll'
    & $cargoExe build --locked --manifest-path $workspace --release -p panopticon-hub
    if ($LASTEXITCODE -ne 0) { throw 'Сборка Panopticon Hub завершилась ошибкой.' }
    Get-FileHash -LiteralPath $env:PANOPTICON_GAME_MODULE, (Join-Path $target 'release\PanopticonHub.exe') -Algorithm SHA256
} finally {
    $env:PANOPTICON_GAME_MODULE = $previousModule
    $env:CARGO_TARGET_DIR = $previousTarget
}

