param([string]$TargetRoot = (Join-Path $PSScriptRoot '../target'))

$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$target = $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($TargetRoot)
& (Join-Path $PSScriptRoot 'build-host.ps1') -TargetRoot $target

$hostExe = Join-Path $target 'release/PanopticonHub.exe'
& (Join-Path $root 'installer/build-setup.ps1') -HostExe $hostExe -TargetRoot $target

$dist = Join-Path $root 'dist'
$null = New-Item -ItemType Directory -Path $dist -Force
$setup = Join-Path $dist 'PanopticonHubSetup.exe'
Copy-Item -LiteralPath (Join-Path $target 'release/PanopticonHubSetup.exe') -Destination $setup -Force
Get-FileHash -LiteralPath $setup -Algorithm SHA256
