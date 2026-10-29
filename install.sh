#!/usr/bin/env bash
#
# KozMO — Eternal Dominion kurulum betiği
#
# Oyunu derler, sistem yoluna kurar ve KDE/Plasma masaüstüne entegre eder:
#   • <önek>/bin/kozmo                      → başlatıcı (launcher)
#   • <önek>/lib/kozmo/dominion-game        → oyun ikilisi
#   • <önek>/share/kozmo/assets             → oyun varlıkları
#   • .desktop kaydı, hicolor ikonları, AppStream metainfo, KDE önbellek tazeleme
#
# Kullanım:
#   ./install.sh                     # /usr/local'e kur (gerekirse sudo ister)
#   ./install.sh --user              # ~/.local'e kur (sudo yok)
#   ./install.sh --prefix /opt/kozmo # özel dizine kur
#   ./install.sh --desktop-shortcut  # ayrıca masaüstü kısayolu oluştur
#   ./install.sh --no-kde            # KDE/masaüstü entegrasyonunu atla
#   ./install.sh --no-build          # derlemeden, mevcut ikiliyi kur
#   ./install.sh --uninstall         # kaldır (--purge ile kayıtlar da silinir)
#
set -Eeuo pipefail

APP_NAME="KozMO"
APP_ID="kozmo"
BIN_NAME="dominion-game"
METAINFO_ID="io.github.Padrosum.KozMO"
REPO_URL="https://github.com/Padrosum/KozMO"

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
STAGE_DIR=""
LOG_PREFIX="kozmo"

PREFIX=""
USER_MODE=0
DO_BUILD=1
DO_KDE=1
DO_DESKTOP_SHORTCUT=0
DO_UNINSTALL=0
DO_PURGE=0
ASSUME_YES=0
JOBS=""
NEED_ROOT=0

# ---------------------------------------------------------------- yardımcılar

log()  { printf '\033[1;36m[%s]\033[0m %s\n' "$LOG_PREFIX" "$*"; }
ok()   { printf '\033[1;32m[%s]\033[0m %s\n' "$LOG_PREFIX" "$*"; }
warn() { printf '\033[1;33m[%s] uyarı:\033[0m %s\n' "$LOG_PREFIX" "$*" >&2; }
die()  { printf '\033[1;31m[%s] hata:\033[0m %s\n' "$LOG_PREFIX" "$*" >&2; exit 1; }

cleanup() {
    if [ -n "$STAGE_DIR" ] && [ -d "$STAGE_DIR" ]; then
        rm -rf -- "$STAGE_DIR"
    fi
}
trap cleanup EXIT

on_error() {
    local code=$?
    printf '\033[1;31m[%s] hata:\033[0m satır %s: beklenmeyen hata (çıkış kodu %s)\n' \
        "$LOG_PREFIX" "${BASH_LINENO[0]:-?}" "$code" >&2
    exit "$code"
}
trap on_error ERR

usage() {
    cat <<'EOF'
KozMO — Eternal Dominion kurulum betiği

Kullanım:
  ./install.sh [seçenekler]

Seçenekler:
  --user                 Kullanıcı dizinine kur (~/.local), sudo gerekmez
  --prefix <dizin>       Kurulum öneki (varsayılan: /usr/local)
  --no-build             Derlemeyi atla, mevcut target/release ikilisini kullan
  --no-kde               .desktop, ikon ve AppStream dosyalarını kurma
  --desktop-shortcut     Masaüstüne KozMO kısayolu bırak
  --uninstall            Kurulu dosyaları kaldır
  --purge                --uninstall ile birlikte kayıtları ve ayarları da sil
  --yes, -y              Onay sorularını otomatik yanıtla
  --jobs <N>             Derlemede kullanılacak paralel iş sayısı
  --help, -h             Bu yardımı göster
EOF
}

# ---------------------------------------------------------------- argümanlar

while [ $# -gt 0 ]; do
    case "$1" in
        --user)              USER_MODE=1 ;;
        --prefix)            [ $# -ge 2 ] || die "--prefix bir dizin gerektirir"; PREFIX="$2"; shift ;;
        --prefix=*)          PREFIX="${1#*=}" ;;
        --no-build)          DO_BUILD=0 ;;
        --no-kde)            DO_KDE=0 ;;
        --desktop-shortcut)  DO_DESKTOP_SHORTCUT=1 ;;
        --uninstall)         DO_UNINSTALL=1 ;;
        --purge)             DO_PURGE=1; DO_UNINSTALL=1 ;;
        --yes|-y)            ASSUME_YES=1 ;;
        --jobs)              [ $# -ge 2 ] || die "--jobs bir sayı gerektirir"; JOBS="$2"; shift ;;
        --jobs=*)            JOBS="${1#*=}" ;;
        --help|-h)           usage; exit 0 ;;
        *)                   die "bilinmeyen seçenek: $1 (--help ile bakın)" ;;
    esac
    shift
done

[ -n "$PREFIX" ] || { if [ "$USER_MODE" = 1 ]; then PREFIX="$HOME/.local"; else PREFIX="/usr/local"; fi; }
if [ "$USER_MODE" = 1 ]; then
    PREFIX="$HOME/.local"
fi

# Yol normalleştirme: sondaki eğik çizgileri temizle
while [ "${PREFIX%/}" != "$PREFIX" ]; do PREFIX="${PREFIX%/}"; done
[ -n "$PREFIX" ] || die "kurulum öneki boş olamaz"

if [ "$(id -u)" = 0 ] && [ "$DO_UNINSTALL" = 0 ]; then
    die "bu betiği doğrudan root/sudo ile çalıştırmayın. Normal kullanıcı olarak çalıştırın; sudo yalnızca gereken adımlarda istenir."
fi

can_write() {
    local target="$1" parent
    if [ -d "$target" ]; then
        [ -w "$target" ]
        return
    fi
    parent="$(dirname -- "$target")"
    while [ ! -d "$parent" ]; do
        parent="$(dirname -- "$parent")"
        [ "$parent" = "/" ] && break
    done
    [ -w "$parent" ]
}

if ! can_write "$PREFIX"; then
    NEED_ROOT=1
    command -v sudo >/dev/null 2>&1 || \
        die "'$PREFIX' yazılabilir değil ve sudo bulunamadı. '--user' veya '--prefix <dizin>' kullanın."
fi

run_root() {
    if [ "$NEED_ROOT" = 1 ]; then
        sudo "$@"
    else
        "$@"
    fi
}

# ---------------------------------------------------------------- temel bilgiler

detect_version() {
    if [ -f "$SCRIPT_DIR/Cargo.toml" ]; then
        sed -n 's/^version[[:space:]]*=[[:space:]]*"\(.*\)"/\1/p' "$SCRIPT_DIR/Cargo.toml" | head -n 1
    fi
}

APP_VERSION="$(detect_version)"; APP_VERSION="${APP_VERSION:-0.2.0}"

build_hint() {
    cat >&2 <<EOF

Olası nedenler ve çözümler:
  • Rust araç zinciri eksik        → https://rustup.rs
  • C bağlayıcı yok                → Ubuntu: sudo apt install build-essential
  • Bevy sistem kütüphaneleri yok:
      Ubuntu/Debian : sudo apt install pkg-config libx11-dev libxi-dev libxtst-dev \\
                        libxkbcommon-dev libwayland-dev libasound2-dev libudev-dev
      Fedora        : sudo dnf install pkgconf-pkg-config libX11-devel libXi-devel \\
                        libxkbcommon-devel wayland-devel alsa-lib-devel systemd-devel
      Arch          : sudo pacman -S --needed pkgconf libx11 libxi libxkbcommon \\
                        wayland alsa-lib systemd-libs
EOF
}

desktop_dir() {
    local dir=""
    if command -v xdg-user-dir >/dev/null 2>&1; then
        dir="$(xdg-user-dir DESKTOP 2>/dev/null || true)"
    fi
    [ -n "$dir" ] || dir="$HOME/Desktop"
    printf '%s' "$dir"
}

refresh_caches() {
    [ "$DO_KDE" = 1 ] || return 0
    log "masaüstü önbellekleri yenileniyor"

    if command -v update-desktop-database >/dev/null 2>&1; then
        update-desktop-database -q "$PREFIX/share/applications" 2>/dev/null || true
    fi
    if command -v gtk-update-icon-cache >/dev/null 2>&1 && [ -d "$PREFIX/share/icons/hicolor" ]; then
        gtk-update-icon-cache -f -t "$PREFIX/share/icons/hicolor" >/dev/null 2>&1 || true
    fi
    if [ -n "${XDG_CURRENT_DESKTOP:-}" ] && printf '%s' "$XDG_CURRENT_DESKTOP" | grep -qi kde; then
        (
            export XDG_DATA_DIRS="$PREFIX/share:${XDG_DATA_DIRS:-/usr/local/share:/usr/share}"
            if command -v kbuildsycoca6 >/dev/null 2>&1; then kbuildsycoca6 >/dev/null 2>&1 || true; fi
            if command -v kbuildsycoca5 >/dev/null 2>&1; then kbuildsycoca5 >/dev/null 2>&1 || true; fi
        )
    fi
    return 0
}

# ---------------------------------------------------------------- kaldırma

uninstall() {
    if [ "$ASSUME_YES" != 1 ] && [ -t 0 ]; then
        printf '%s, "%s" önekinden kaldırılacak. Devam edilsin mi? [e/H] ' "$APP_NAME" "$PREFIX"
        read -r answer
        case "$answer" in
            e|E|y|Y|evet|yes) ;;
            *) die "kaldırma iptal edildi." ;;
        esac
    fi

    log "$APP_NAME kaldırılıyor (önek: $PREFIX)"

    local data_dir="${XDG_DATA_HOME:-$HOME/.local/share}/$APP_ID"

    local f size
    for f in \
        "$PREFIX/bin/$APP_ID" \
        "$PREFIX/share/applications/$APP_ID.desktop" \
        "$PREFIX/share/metainfo/$METAINFO_ID.metainfo.xml" \
        "$PREFIX/share/pixmaps/$APP_ID.png" \
        "$PREFIX/share/icons/hicolor/scalable/apps/$APP_ID.svg"
    do
        if [ -e "$f" ] || [ -L "$f" ]; then
            run_root rm -f -- "$f"
            log "silindi: $f"
        fi
    done

    for size in 16 24 32 48 64 128 256 512; do
        f="$PREFIX/share/icons/hicolor/${size}x${size}/apps/$APP_ID.png"
        if [ -e "$f" ]; then
            run_root rm -f -- "$f"
        fi
    done

    local d
    for d in "$PREFIX/lib/$APP_ID" "$PREFIX/share/$APP_ID"; do
        [ -d "$d" ] || continue
        if [ "$d" = "$data_dir" ] && [ "$DO_PURGE" != 1 ]; then
            # --user kurulumunda veri dizini kurulum ağacıyla aynıdır: kayıtları koru.
            run_root find "$d" -mindepth 1 -maxdepth 1 ! -name saves -exec rm -rf -- {} + 2>/dev/null || true
            log "kaldırıldı, kayıtlar korundu: $d"
        else
            run_root rm -rf -- "$d"
            log "silindi: $d"
        fi
    done

    local shortcut
    shortcut="$(desktop_dir)/$APP_NAME.desktop"
    if [ -e "$shortcut" ]; then
        rm -f -- "$shortcut"
        log "silindi: $shortcut"
    fi

    refresh_caches

    if [ -L "$data_dir/assets" ]; then
        local link_target
        link_target="$(readlink -- "$data_dir/assets" || true)"
        case "$link_target" in
            "$PREFIX"/*) rm -f -- "$data_dir/assets"; log "silindi: $data_dir/assets (başlatıcı bağı)" ;;
        esac
    fi
    if [ "$DO_PURGE" = 1 ]; then
        if [ -d "$data_dir" ]; then
            rm -rf -- "$data_dir"
            log "kayıtlar ve kullanıcı verisi silindi: $data_dir"
        fi
    elif [ -d "$data_dir/saves" ]; then
        warn "kayıtlar korundu: $data_dir/saves (silmek için --purge)"
    fi

    ok "$APP_NAME kaldırıldı."
    exit 0
}

if [ "$DO_UNINSTALL" = 1 ]; then
    uninstall
fi

# ---------------------------------------------------------------- derleme

[ -f "$SCRIPT_DIR/Cargo.toml" ] || die "Cargo.toml bulunamadı; betiği depo kökünden çalıştırın."
[ -d "$SCRIPT_DIR/assets" ] || die "assets/ dizini bulunamadı; betiği depo kökünden çalıştırın."

if [ "$DO_BUILD" = 1 ]; then
    command -v cargo >/dev/null 2>&1 || die "cargo bulunamadı. Rust kurun: https://rustup.rs"
    log "derleniyor: cargo build --release -p dominion-game (ilk derleme birkaç dakika sürebilir)"
    if [ -n "$JOBS" ]; then
        build_ok=0
        ( cd "$SCRIPT_DIR" && cargo build --release -p dominion-game -j "$JOBS" ) || build_ok=$?
    else
        build_ok=0
        ( cd "$SCRIPT_DIR" && cargo build --release -p dominion-game ) || build_ok=$?
    fi
    if [ "$build_ok" -ne 0 ]; then
        build_hint
        die "derleme başarısız oldu."
    fi
fi

BUILT_BIN="$SCRIPT_DIR/target/release/$BIN_NAME"
[ -x "$BUILT_BIN" ] || die "oyun ikilisi bulunamadı: $BUILT_BIN (--no-build kullanmayın veya önce 'cargo build --release -p dominion-game' çalıştırın)"
ok "ikili hazır: $BUILT_BIN ($(du -h "$BUILT_BIN" | cut -f1))"

# ---------------------------------------------------------------- staging

STAGE_DIR="$(mktemp -d "${TMPDIR:-/tmp}/kozmo-install.XXXXXX")"
mkdir -p "$STAGE_DIR/bin" \
         "$STAGE_DIR/lib/$APP_ID" \
         "$STAGE_DIR/share/$APP_ID" \
         "$STAGE_DIR/share/applications" \
         "$STAGE_DIR/share/metainfo" \
         "$STAGE_DIR/share/icons/hicolor/scalable/apps" \
         "$STAGE_DIR/share/pixmaps"

install -m 755 "$BUILT_BIN" "$STAGE_DIR/lib/$APP_ID/$BIN_NAME"
cp -a "$SCRIPT_DIR/assets" "$STAGE_DIR/share/$APP_ID/assets"

# --- başlatıcı: varlık yollarını ve yazılabilir kayıt dizinini garanti eder ---
cat > "$STAGE_DIR/bin/$APP_ID" <<LAUNCHER
#!/bin/sh
# KozMO başlatıcı — kurulum betiği tarafından üretildi.
set -eu

PREFIX="$PREFIX"
BIN="\$PREFIX/lib/$APP_ID/$BIN_NAME"
ASSETS="\$PREFIX/share/$APP_ID/assets"
DATA_DIR="\${XDG_DATA_HOME:-\$HOME/.local/share}/$APP_ID"
SAVE_DIR="\$DATA_DIR/saves"

[ -x "\$BIN" ] || {
    echo "KozMO ikilisi bulunamadı: \$BIN" >&2
    echo "Yeniden kurmak için depodaki install.sh betiğini çalıştırın." >&2
    exit 1
}

mkdir -p "\$SAVE_DIR"

# Oyun 'assets' klasörünü çalışma dizinine göre arar; kalıcı bağ kullanıyoruz.
if [ "\$ASSETS" = "\$DATA_DIR/assets" ]; then
    : # --user kurulumu: varlıklar doğrudan veri dizininde
elif [ -L "\$DATA_DIR/assets" ]; then
    ln -sfn "\$ASSETS" "\$DATA_DIR/assets"
elif [ ! -e "\$DATA_DIR/assets" ]; then
    ln -s "\$ASSETS" "\$DATA_DIR/assets"
else
    echo "uyarı: \$DATA_DIR/assets zaten var ve bağ değil; dokunulmadı." >&2
fi

if [ "\${1:-}" = "--open-saves" ]; then
    exec xdg-open "\$SAVE_DIR"
fi
if [ "\${1:-}" = "--save-dir" ]; then
    printf '%s\n' "\$SAVE_DIR"
    exit 0
fi

# Kayıtlar kullanıcı dizinine yazılsın diye çalışma dizinini değiştiriyoruz.
cd "\$DATA_DIR"
exec "\$BIN" "\$@"
LAUNCHER
chmod 755 "$STAGE_DIR/bin/$APP_ID"

# --- ikon: kare uygulama ikonu varsa o, yoksa geniş logo ---
ICON_SRC="$SCRIPT_DIR/assets/art/logos/${APP_ID}_icon.svg"
[ -f "$ICON_SRC" ] || ICON_SRC="$SCRIPT_DIR/assets/art/logos/eternal_dominion_logo.svg"
cp -a "$ICON_SRC" "$STAGE_DIR/share/icons/hicolor/scalable/apps/$APP_ID.svg"

convert_icon() {
    local size="$1" out="$2"
    if command -v rsvg-convert >/dev/null 2>&1; then
        rsvg-convert -w "$size" -h "$size" "$ICON_SRC" -o "$out" 2>/dev/null && return 0
    fi
    if command -v magick >/dev/null 2>&1; then
        magick -background none "$ICON_SRC" -resize "${size}x${size}" "$out" 2>/dev/null && return 0
    fi
    if command -v convert >/dev/null 2>&1; then
        convert -background none "$ICON_SRC" -resize "${size}x${size}" "$out" 2>/dev/null && return 0
    fi
    return 1
}

if command -v rsvg-convert >/dev/null 2>&1 || command -v magick >/dev/null 2>&1 || command -v convert >/dev/null 2>&1; then
    for size in 16 24 32 48 64 128 256 512; do
        mkdir -p "$STAGE_DIR/share/icons/hicolor/${size}x${size}/apps"
        if ! convert_icon "$size" "$STAGE_DIR/share/icons/hicolor/${size}x${size}/apps/$APP_ID.png"; then
            warn "${size}px ikon üretilemedi; SVG ikon kullanılacak."
        fi
    done
    if [ -f "$STAGE_DIR/share/icons/hicolor/256x256/apps/$APP_ID.png" ]; then
        cp -a "$STAGE_DIR/share/icons/hicolor/256x256/apps/$APP_ID.png" "$STAGE_DIR/share/pixmaps/$APP_ID.png"
    fi
else
    warn "SVG→PNG dönüştürücü bulunamadı (rsvg-convert/imagemagick); yalnızca ölçeklenebilir SVG ikon kurulacak."
fi

# --- .desktop kaydı (KDE menüsü, KRunner ve Dolphin için) ---
DESKTOP_FILE="$STAGE_DIR/share/applications/$APP_ID.desktop"
cat > "$DESKTOP_FILE" <<EOF
[Desktop Entry]
Type=Application
Version=1.5
Name=$APP_NAME
Name[tr]=$APP_NAME
GenericName=Cosmic Sandbox Strategy
GenericName[tr]=Kozmik Sandbox Strateji
Comment=Command 60 living universes, 8 rival gods and a soul economy
Comment[tr]=60 yaşayan evreni, 8 rakip tanrıyı ve ruh ekonomisini yönet
TryExec=$PREFIX/bin/$APP_ID
Exec="$PREFIX/bin/$APP_ID"
Icon=$APP_ID
Terminal=false
Categories=Game;StrategyGame;
Keywords=kozmo;strategy;god;cosmic;universe;sandbox;soul;
Keywords[tr]=kozmo;strateji;tanrı;evren;kozmik;kum;ruh;sandbox;
StartupNotify=true
StartupWMClass=$APP_ID
X-KDE-StartupNotify=true
X-KDE-SubstituteUID=false
Actions=OpenSaveFolder;

[Desktop Action OpenSaveFolder]
Name=Open Save Folder
Name[tr]=Kayıt Klasörünü Aç
Exec="$PREFIX/bin/$APP_ID" --open-saves
Icon=folder-open
EOF
chmod 644 "$DESKTOP_FILE"

# --- AppStream metainfo (Plasma Discover / yazılım merkezleri) ---
cat > "$STAGE_DIR/share/metainfo/$METAINFO_ID.metainfo.xml" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<component type="desktop-application">
  <id>$METAINFO_ID</id>
  <metadata_license>CC0-1.0</metadata_license>
  <project_license>GPL-3.0-or-later</project_license>
  <name>$APP_NAME</name>
  <summary>Cosmic sandbox strategy across 60 living universes</summary>
  <summary xml:lang="tr">60 yaşayan evrende kozmik sandbox strateji</summary>
  <description>
    <p>KozMO is a cosmic sandbox strategy game. You are a deity without a body: shape a pool of 60 independently evolving universes, outmaneuver 8 rival gods, harvest souls and decide the fate of mortal civilizations.</p>
    <p xml:lang="tr">KozMO bir kozmik sandbox strateji oyunudur. Bedeni olmayan bir tanrı olarak 60 bağımsız evrenin oluşturduğu havuzu şekillendirir, 8 rakip tanrıyı alt eder, ruh hasadı yapar ve ölümlü uygarlıkların kaderini belirlersiniz.</p>
  </description>
  <launchable type="desktop-id">$APP_ID.desktop</launchable>
  <provides>
    <binary>$APP_ID</binary>
  </provides>
  <url type="homepage">$REPO_URL</url>
  <url type="bugtracker">$REPO_URL/issues</url>
  <developer id="io.github.padrosum">
    <name>Eternal Dominion Contributors</name>
  </developer>
  <categories>
    <category>Game</category>
    <category>StrategyGame</category>
  </categories>
  <keywords>
    <keyword>strategy</keyword>
    <keyword>god game</keyword>
    <keyword>sandbox</keyword>
    <keyword>space</keyword>
    <keyword>simulation</keyword>
  </keywords>
  <content_rating type="oars-1.1"/>
  <releases>
    <release version="$APP_VERSION" date="$(date +%Y-%m-%d)"/>
  </releases>
</component>
EOF
chmod 644 "$STAGE_DIR/share/metainfo/$METAINFO_ID.metainfo.xml"

# ---------------------------------------------------------------- kurulum

log "kuruluyor: $PREFIX"

run_root install -d -m 755 \
    "$PREFIX/bin" \
    "$PREFIX/lib/$APP_ID" \
    "$PREFIX/share/$APP_ID"

if [ "$DO_KDE" = 1 ]; then
    run_root install -d -m 755 \
        "$PREFIX/share/applications" \
        "$PREFIX/share/metainfo" \
        "$PREFIX/share/icons/hicolor/scalable/apps" \
        "$PREFIX/share/pixmaps"
fi

run_root install -m 755 "$STAGE_DIR/lib/$APP_ID/$BIN_NAME" "$PREFIX/lib/$APP_ID/$BIN_NAME"
run_root install -m 755 "$STAGE_DIR/bin/$APP_ID" "$PREFIX/bin/$APP_ID"

# Varlıklar: eski kurulumdan kalanları temizleyip taze kopya
run_root rm -rf -- "$PREFIX/share/$APP_ID/assets"
run_root cp -a "$STAGE_DIR/share/$APP_ID/assets" "$PREFIX/share/$APP_ID/assets"

if [ "$DO_KDE" = 1 ]; then
    run_root install -m 644 "$DESKTOP_FILE" "$PREFIX/share/applications/$APP_ID.desktop"
    run_root install -m 644 "$STAGE_DIR/share/metainfo/$METAINFO_ID.metainfo.xml" \
        "$PREFIX/share/metainfo/$METAINFO_ID.metainfo.xml"
    run_root install -m 644 "$STAGE_DIR/share/icons/hicolor/scalable/apps/$APP_ID.svg" \
        "$PREFIX/share/icons/hicolor/scalable/apps/$APP_ID.svg"

    for size in 16 24 32 48 64 128 256 512; do
        png="$STAGE_DIR/share/icons/hicolor/${size}x${size}/apps/$APP_ID.png"
        [ -f "$png" ] || continue
        run_root install -d -m 755 "$PREFIX/share/icons/hicolor/${size}x${size}/apps"
        run_root install -m 644 "$png" "$PREFIX/share/icons/hicolor/${size}x${size}/apps/$APP_ID.png"
    done

    if [ -f "$STAGE_DIR/share/pixmaps/$APP_ID.png" ]; then
        run_root install -m 644 "$STAGE_DIR/share/pixmaps/$APP_ID.png" "$PREFIX/share/pixmaps/$APP_ID.png"
    fi
fi

if [ "$NEED_ROOT" = 1 ]; then
    run_root chown -R 0:0 "$PREFIX/lib/$APP_ID" "$PREFIX/share/$APP_ID"
    if [ "$DO_KDE" = 1 ]; then
        run_root chown 0:0 \
            "$PREFIX/share/applications/$APP_ID.desktop" \
            "$PREFIX/share/metainfo/$METAINFO_ID.metainfo.xml" \
            "$PREFIX/share/icons/hicolor/scalable/apps/$APP_ID.svg"
    fi
fi

refresh_caches

# --- masaüstü kısayolu ---
if [ "$DO_DESKTOP_SHORTCUT" = 1 ] && [ "$DO_KDE" = 1 ]; then
    ddir="$(desktop_dir)"
    if [ -d "$ddir" ]; then
        install -m 755 "$DESKTOP_FILE" "$ddir/$APP_NAME.desktop"
        ok "masaüstü kısayolu oluşturuldu: $ddir/$APP_NAME.desktop"
        if [ -n "${XDG_CURRENT_DESKTOP:-}" ] && printf '%s' "$XDG_CURRENT_DESKTOP" | grep -qi kde; then
            log "KDE ilk tıklamada 'Güvenilir olarak işaretle' isteyebilir (sağ tık → Çalıştırmaya izin ver)."
        fi
    else
        warn "masaüstü dizini bulunamadı: $ddir"
    fi
elif [ "$DO_DESKTOP_SHORTCUT" = 1 ]; then
    warn "--desktop-shortcut, --no-kde ile birlikte kullanılamaz."
fi

# --- kurulum sonrası sağlık kontrolü ---
missing_libs="$(ldd "$PREFIX/lib/$APP_ID/$BIN_NAME" 2>/dev/null | grep 'not found' || true)"
if [ -n "$missing_libs" ]; then
    warn "eksik paylaşılan kütüphaneler tespit edildi:"
    printf '%s\n' "$missing_libs" >&2
    build_hint
fi

"$PREFIX/bin/$APP_ID" --save-dir >/dev/null 2>&1 || warn "başlatıcı doğrulaması başarısız oldu."

SAVE_DIR="${XDG_DATA_HOME:-$HOME/.local/share}/$APP_ID/saves"

printf '\n\033[1;32m%s %s kuruldu.\033[0m\n\n' "$APP_NAME" "$APP_VERSION"
printf '  Başlat    : %s\n' "$PREFIX/bin/$APP_ID"
printf '  Menüden   : Uygulamalar → Oyunlar → %s\n' "$APP_NAME"
printf '  Kayıtlar  : %s\n' "$SAVE_DIR"
printf '  Varlıklar : %s\n' "$PREFIX/share/$APP_ID/assets"
if [ "$NEED_ROOT" = 1 ]; then
    printf '  Kaldırma  : sudo %s/install.sh --uninstall\n' "$SCRIPT_DIR"
else
    printf '  Kaldırma  : %s/install.sh --uninstall\n' "$SCRIPT_DIR"
fi
printf '\n'
ok "İyi oyunlar. Evrenler senden önce evrilmeye başladı bile."
