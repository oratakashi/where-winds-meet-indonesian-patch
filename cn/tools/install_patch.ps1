<#
install_patch.ps1 - pasang / kembalikan / cek file terjemahan WWM China di client CN.

    powershell -ExecutionPolicy Bypass -File cn\tools\install_patch.ps1 install
    powershell -ExecutionPolicy Bypass -File cn\tools\install_patch.ps1 status
    powershell -ExecutionPolicy Bypass -File cn\tools\install_patch.ps1 restore

install  backup file asli game (sekali, ke cn\work\backup-original\) lalu salin file patch
         (default cn\work\test-install\, hasil `tools/patch_all.py --profile cn`)
status   per file: apakah yang terpasang di game = PATCH, = ASLI, atau lain. Jalankan
         setelah membuka launcher/game untuk melihat apakah patcher mengembalikan file.
restore  kembalikan file asli dari backup.

Game & launcher harus ditutup untuk install/restore. Client CN berjalan sebagai admin;
kalau penyalinan ditolak, jalankan dari PowerShell Administrator.
#>
param(
    [Parameter(Mandatory = $true)][ValidateSet('install', 'status', 'restore')][string]$Action,
    [string]$Source = '',
    [string]$GameDir = 'D:\yysls\yysls_fast\LocalData\Patch\HD\v2\locale'
)
$ErrorActionPreference = 'Stop'
# PowerShell 5.1: $PSScriptRoot is still empty while parameter defaults are evaluated
if (-not $Source) { $Source = Join-Path $PSScriptRoot '..\work\test-install' }
$Files = 'translate_words_map_zh_cn', 'translate_words_map_zh_cn__small'
$Backup = Join-Path $PSScriptRoot '..\work\backup-original'

function Hash($p) { if (Test-Path $p) { (Get-FileHash $p -Algorithm SHA256).Hash } }

function Assert-Closed {
    $running = Get-Process -Name yysls, launcher -ErrorAction SilentlyContinue
    if ($running) {
        throw "Tutup dulu game/launcher (masih jalan: $($running.Name -join ', '))."
    }
}

if (-not (Test-Path $GameDir)) { throw "Folder game tidak ditemukan: $GameDir" }

switch ($Action) {
    'install' {
        Assert-Closed
        New-Item -ItemType Directory -Force $Backup | Out-Null
        foreach ($f in $Files) {
            $src = Join-Path $Source $f
            if (-not (Test-Path $src)) { throw "File patch tidak ada: $src (jalankan tools/patch_all.py --profile cn)" }
            $bak = Join-Path $Backup $f
            if (-not (Test-Path $bak)) {
                Copy-Item (Join-Path $GameDir $f) $bak
                Write-Host "backup  $f -> $bak"
            } elseif ((Hash (Join-Path $GameDir $f)) -ne (Hash $bak) -and (Hash (Join-Path $GameDir $f)) -ne (Hash $src)) {
                Write-Host "info    $f di game bukan file asli (patch lama terpasang, atau update game?) - backup asli dipertahankan."
            }
            Copy-Item $src (Join-Path $GameDir $f) -Force
            Write-Host "pasang  $f"
        }
        Write-Host "Selesai. Jalankan game (pastikan bahasa game = Chinese (Simplified)), lalu cek: install_patch.ps1 status"
    }
    'restore' {
        Assert-Closed
        foreach ($f in $Files) {
            $bak = Join-Path $Backup $f
            if (-not (Test-Path $bak)) { throw "Backup tidak ada: $bak" }
            Copy-Item $bak (Join-Path $GameDir $f) -Force
            Write-Host "kembali $f"
        }
    }
    'status' {
        foreach ($f in $Files) {
            $g = Join-Path $GameDir $f
            $h = Hash $g
            # original reference: the backup, or before the first install the copy in cn\
            $orig = Join-Path $Backup $f
            if (-not (Test-Path $orig)) { $orig = Join-Path $PSScriptRoot "..\$f" }
            $state = if ($h -eq (Hash (Join-Path $Source $f))) { 'PATCH (terjemahan terpasang)' }
                     elseif ($h -eq (Hash $orig)) { 'ASLI (belum dipasang / dikembalikan patcher)' }
                     else { 'LAIN (versi game baru?)' }
            $item = Get-Item $g
            "{0,-34} {1}  [{2:N0} byte, diubah {3}]" -f $f, $state, $item.Length, $item.LastWriteTime
        }
    }
}
