param(
    [string]$Setup = (Join-Path $PSScriptRoot '../dist/PanopticonHubSetup.exe'),
    [Parameter(Mandatory)][string]$NotesFile
)

$ErrorActionPreference = 'Stop'
$repo = 'werton-core/PanopticonHub'
$assetName = 'PanopticonHubSetup.exe'
$installer = Get-Item -LiteralPath $Setup
$notes = Get-Item -LiteralPath $NotesFile
$version = $installer.VersionInfo.ProductVersion
$package = Get-Content -LiteralPath (Join-Path $PSScriptRoot '../installer/Cargo.toml') -Raw
$packageVersion = [regex]::Match($package, '(?m)^version\s*=\s*"([^"]+)"').Groups[1].Value
if ($installer.Name -cne $assetName -or $installer.VersionInfo.OriginalFilename -cne $assetName -or
    $version -cnotmatch '^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$' -or $version -cne $packageVersion) {
    throw 'Имя и версия установщика должны совпадать с installer/Cargo.toml.'
}
$tag = "v$version"
$hash = (Get-FileHash -LiteralPath $installer.FullName -Algorithm SHA256).Hash.ToLowerInvariant()
$payload = & $installer.FullName --check-payload | Out-String
if ($LASTEXITCODE -ne 0 -or $payload -notmatch 'fixture=false') {
    throw 'Установщик не содержит продуктивный пакет.'
}

function Read-Release {
    $query = '.[] | select(.tag_name == "{0}")' -f $tag
    $releaseJson = & gh api "repos/$repo/releases?per_page=100" --paginate --jq $query
    if ($LASTEXITCODE -ne 0) { throw "Не удалось прочитать релизы $repo." }
    if ($releaseJson) { $releaseJson | ConvertFrom-Json }
}

$release = Read-Release
if (!$release) {
    $notice = Join-Path $installer.DirectoryName 'ThirdPartyNotices.zip'
    $files = (Join-Path $PSScriptRoot '../THIRD_PARTY.md'), (Join-Path $PSScriptRoot '../licenses')
    Compress-Archive -LiteralPath $files -DestinationPath $notice -Force
    & gh release create $tag $installer.FullName $notice --repo $repo --title "Panopticon Hub $version" --notes-file $notes.FullName --verify-tag --draft
    if ($LASTEXITCODE -ne 0) { throw "Не удалось создать черновик $tag." }
    $release = Read-Release
    if (!$release) { throw "Не удалось прочитать черновик $tag." }
}
$assets = @($release.assets | Where-Object { $_.name -ceq $assetName -and $_.state -ceq 'uploaded' })
$notices = @($release.assets | Where-Object { $_.name -ceq 'ThirdPartyNotices.zip' -and $_.state -ceq 'uploaded' })
if ($release.prerelease -or $assets.Count -ne 1 -or $notices.Count -ne 1 -or $assets[0].size -ne $installer.Length -or
    $assets[0].digest -cne "sha256:$hash") {
    throw "Файл $assetName в $tag не прошёл проверку размера и SHA-256."
}
if ($release.draft) {
    & gh release edit $tag --repo $repo --draft=false --latest
    if ($LASTEXITCODE -ne 0) { throw "Не удалось опубликовать $tag." }
} else {
    Write-Output $release.html_url
}

