# KozMO — Kozmik Sandbox Strateji (v0.2)

**KozMO** (*Eternal Dominion*), Rust ve Bevy 0.20 ile geliştirilen, oyuncunun fiziksel bir avatar olmadan kozmik ölçekte 60 evrenlik yaşayan bir havuzu yönettiği, 8 rakip tanrıyla mücadele ettiği, ruh ekonomisini yönlendirdiği ve ölümlü uygarlıkların tarihini şekillendirdiği bir kozmik strateji ve sandbox oyunudur.

---

## Mimari ve Crate Yapısı (`AGENTS.md`, `prompt.md`, `Arayüz.md` & `Geliştirme_Yol_Haritası.md`)

Proje, simülasyon çekirdeğini grafik çalışma zamanından (`Bevy`) tamamen bağımsız tutan modüler bir Cargo workspace olarak yapılandırılmıştır:

- `crates/dominion-sim`: Saf Rust ile yazılmış, deterministik simülasyon çekirdeği (`Bevy` bağımlılığı yoktur).
  - `rng.rs`: Tohumlanabilir (`seedable`) deterministik `Xoroshiro128++` rastgele sayı üreteci.
  - `universe.rs`: 6 evren kategorisi, kozmik geçitler, tanrısal etki haritası ve prosedürel `[Kadim Çağ]` arkeolojisi içeren kalıcı evren tarihi (`UniverseHistoryEntry`).
  - `civilization.rs`: Ölümlü uygarlıklar, **5 Uygarlık Doktrini (`CivilizationDoctrine`: `Bilge`, `Tapınakçı`, `Kuşkucu`, `Hanedan`, `Mistik`)**, nüfus değişimi, teknoloji gelişimi, bilimsel ilahi kalkanlar, rakip kültler ve mezhep savaşları.
  - `deity.rs`: Oyuncu ve 8 bağımsız rakip tanrı arketipleri, 5 gelişim ağacı (`PowerTree`), 15 ilahi güç (`PowerId` — açıldığında anlık kozmik patlama + kalıcı pasif bonus + emir indirimi) ve diplomasi.
  - `economy.rs`: **Ruh**, **Nüfuz** ve **Kozmik Öz** kaynakları ile tüm kaynak girişlerini, harcamalarını, transferlerini ve kayıplarını izleyen `ResourceLedger`.
  - `events.rs`: Savaşlar, mezhep ayrılıkları (`ReligiousSchism`), anomaliler, kehanetler, 2–6 turda bir tetiklenen rastgele kozmik olaylar ve gerçeklik çöküşleri.
  - `save.rs`: Sürümlendirilmiş kayıt şeması (`v2` + `v1 -> v2` şema göçü), bozuk kayıt koruması ve kısmi yazmaya karşı atomik dosya kaydı (`save_to_path_atomic`).
  - `simulation.rs`: 6 aşamalı sabit güncelleme sırası (`step_tick`), uzak evren arka plan optimizasyonu, rakip kült/mezhep çatışmaları, rakip yapay zekâ karar döngüsü ve zafer/yenilgi değerlendirmesi.
- `crates/dominion-content`: Veri odaklı evren türü profilleri, ilahi güç tanımları, oynanabilir senaryo şablonları (`Kozmik Yükseliş`, `İlahların Alacakaranlığı`, `Sonsuz Havuz Sandbox`) ve 8 bölümlük kapsamlı oyun içi ansiklopedi (**KozMO Wiki** — `guide.rs`).
- `crates/dominion-game`: Bevy 0.20 tabanlı görsel kozmik harita ve arayüz katmanı:
  - `map/mod.rs`: 4 kozmik atmosfer teması (`T`), **4 analitik harita görünüm katmanı (`M` / `◎ Katman`)**, evren türüne özel geometrik çekirdek/aura, **Tanrısal Başkent Mühür Tacı** ve **Uygarlık Yörünge Uyduları**.
  - `audio/mod.rs`: Harici ses dosyası gerektirmeyen, bellek içi çok-harmonikli **Prosedürel Kozmik Ses Sentezleyicisi (`CosmicAudioPlugin` — `V` / `♪ Ses`)**; kehanet çanları, mucize akorları, ruh sifonu, ilahi darbe, geçit rezonansı ve zafer fanfarını gerçek zamanlı üretir.
  - `ui/mod.rs`: Gömülü tam Türkçe karakter ve sembol destekli (`DejaVuSansCondensed.ttf`) kullanıcı arayüzü, ~2 saniye beklemeli canlı indirimli maliyet ve yetki rozetli buton açıklama kartı (`ButtonHoverTooltipOverlay`), etkileşimli eğitici görev sistemi ve **Wiki (`F1`)**.
  - `save/mod.rs`: Hızlı kaydetme (`F5`) ve hızlı yükleme (`F9`) köprüsü.
- `assets/`: `TASARIM.md` kurallarına uygun SVG sembolleri (`art/universes/`, `art/deities/`, `art/powers/`, `art/events/`, `art/ui/`, `art/logos/`), gömülü Türkçe destekli yazı tipleri (`fonts/DejaVuSans*.ttf`) ve içerik tanımları.
- `tests/`: Grafik motorundan bağımsız ekonomi (`economy_tests.rs`), deterministik simülasyon (`simulation_tests.rs`) ve kayıt/yükleme (`save_load_tests.rs`) testleri.

---

## Oynanış Özellikleri (`taslak.md`, `Arayüz.md` & `Geliştirme_Yol_Haritası.md`)

### 1. Evrenler Havuzu (60 Evren, 6 Kategori) ve 5 Uygarlık Doktrini
- **6 Evren Kategorisi:** **Yaşam (`Life`)**, **Kaos (`Chaos`)**, **İnanç (`Faith`)**, **Teknoloji (`Tech`)**, **İlksel (`Primordial`)** ve **Çöken (`Collapsing`)** evrenler.
- **5 Uygarlık Doktrini (`CivilizationDoctrine`):** Her uygarlık evren türünden bağımsız olarak **Uyumcu Filozoflar (`Bilge`)**, **Bağnaz Tapınakçılar (`Tapınakçı` — +%25 İbadet Ruhu/Nüfuz)**, **Bilimsel Kuşkucular (`Kuşkucu` — +%30 Teknoloji/Kalkan)**, **Savaşçı Hanedan (`Hanedan` — +%25 Savaş Ruhu)** veya **Hiçlik Mistikleri (`Mistik` — Kaos direnci)** doktrinlerinden birine sahiptir.
- **Rakip Kültler ve Mezhep Çatışmaları:** Bir evrende iki ilah `%25+` nüfuza ulaştığında uygarlıklar rakip mezheplere bölünür ve mezhep savaşları (`+18` Ruh Havuzu, `+%6 / -%5` Nüfuz değişimi) başlar.

### 2. Dörtlü Harita Görünüm Katmanı (`M`) ve Prosedürel Ses Sentezleyicisi (`V`)
- **Harita Katmanları (`M` veya `◎ Katman`):** `1. Standart Kozmos`, `2. İlahi Hâkimiyet & Bloklar`, `3. Ekonomi & Ruh Havuzu` ve `4. İstikrar & Tehlike Isısı`.
- **Kozmik Atmosfer Temaları (`T` veya `❖ Tema`):** `Safir Mor & İlahi Kehribar`, `Zümrüt Yeşim & Yıldız Tozu`, `Kızıl Kehribar & Süpernova` ve `Ebedi Mermer & Altın Usturlap`.
- **Prosedürel Ses Sentezleyicisi (`V` veya `♪ Ses`):** Her ilahi emir, geçit etkileşimi, rastgele kozmik olay ve zafer için gerçek zamanlı sentezlenen çok-harmonikli ses geri bildirimi.

---

## Derleme (Build), Paketleme ve Çalıştırma Yönergeleri

### 1. Ön Gereksinimler
- **Rust Araç Zinciri (`1.80+`):** [`rustup`](https://rustup.rs/) üzerinden kararlı (`stable`) Rust sürümü kurulu olmalıdır.
- **Linux Sistem Kütüphaneleri (Bevy Pencere ve Ses Desteği İçin):**
  - **Ubuntu / Debian / Linux Mint:**
    ```bash
    sudo apt-get update
    sudo apt-get install -y pkg-config libx11-dev libxi-dev libxtst-dev libxkbcommon-dev libwayland-dev libasound2-dev libudev-dev
    ```
  - **Fedora / RHEL:**
    ```bash
    sudo dnf install -y pkgconf-pkg-config libX11-devel libXi-devel libxkbcommon-devel wayland-devel alsa-lib-devel systemd-devel
    ```
  - **Arch Linux / Manjaro:**
    ```bash
    sudo pacman -S --needed pkgconf libx11 libxi libxkbcommon wayland alsa-lib systemd-libs
    ```

### 2. Projeyi Derleme (Build) Komutları

```bash
# 1) Geliştirme (Debug) modunda tüm çalışma alanını (workspace) derlemek için:
cargo build --workspace

# 2) Tam optimize edilmiş (Release) oyun ikilisini (binary) derlemek için (Önerilen):
cargo build --release -p dominion-game

# 3) İnternet bağlantısı olmadan (yerel önbellek ile çevrimdışı) derlemek için:
cargo build --offline -p dominion-game
```

Derleme tamamlandığında çalıştırılabilir oyun dosyaları şu konumlarda oluşturulur:
- **Debug Binary:** `./target/debug/dominion-game`
- **Release Binary:** `./target/release/dominion-game`

> **Not:** Yazı tipleri (`DejaVuSans*.ttf`) ve prosedürel ses sentezleyicisi doğrudan çalıştırılabilir ikili dosyaya (`dominion-game`) gömülüdür; oyun hem proje kök dizininden hem de `target/release/` dizininden doğrudan çalıştırılabilir.

### 3. Oyunu Çalıştırma Komutları

```bash
# Geliştirme modunda çalıştırmak için:
cargo run -p dominion-game

# Yüksek performanslı Release modunda çalıştırmak için:
cargo run --release -p dominion-game
```

### 4. Test, Biçimlendirme ve Kalite Doğrulama Komutları

```bash
# Tüm simülasyon, ekonomi, ses sentezleyici ve kayıt/yükleme birim testlerini çalıştırmak için:
cargo test --workspace

# Kod biçimlendirme ve Clippy lint kontrolleri için:
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
```

### 5. GitHub Actions CI/CD ve Çoklu Platform Paketleme ([`.github/workflows/ci.yml`](file:///home/padros/AppImages/KozMO/.github/workflows/ci.yml))
- Her `push` (`main` / `master`) ve `pull_request` işleminde otomatik olarak **`cargo fmt`**, **`cargo clippy`** ve **`cargo test --workspace`** kontrolleri çalıştırılır.
- Ardından **Linux (`KozMO-linux-x86_64.tar.gz`)**, **Windows (`KozMO-windows-x86_64.zip`)** ve **macOS (`KozMO-macos-arm64.tar.gz`)** için optimize edilmiş Release paketleri (`dominion-game` + `assets/` + dokümantasyon) derlenip GitHub Actions Artifact olarak yüklenir.
- `v*` formatında bir sürüm etiketi (`git tag v0.2.0 && git push origin v0.2.0`) gönderildiğinde paketler otomatik olarak **GitHub Releases** sayfasında yayınlanır.

---

## Lisans
Kod tabanı **GNU GPL-3.0-or-later** (`LICENSE`) lisansıyla dağıtılmaktadır. Sanat ve yazı tipi varlıklarının lisans bilgileri [`assets/ASSET_LICENSES.md`](file:///home/padros/AppImages/KozMO/assets/ASSET_LICENSES.md) dosyasında kayıtlıdır.
