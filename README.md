# KozMO — *Eternal Dominion*

### 60 evren. 8 rakip tanrı. Tek bir irade: seninki.

[![CI](https://github.com/Padrosum/KozMO/actions/workflows/ci.yml/badge.svg)](https://github.com/Padrosum/KozMO/actions/workflows/ci.yml)
[![Lisans: GPL-3.0-or-later](https://img.shields.io/badge/lisans-GPL--3.0--or--later-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.80%2B-orange.svg)](https://rustup.rs)
[![Bevy](https://img.shields.io/badge/bevy-0.20-purple.svg)](https://bevyengine.org)
[![Sürüm](https://img.shields.io/badge/sürüm-v0.2.0-brightgreen.svg)](Geliştirme_Yol_Haritası.md)

**KozMO**, Rust ve Bevy ile yazılmış bir **kozmik sandbox strateji oyunu**. Bedeni yok, sınırı yok, sabrı yok: sen bir tanrısın. Fiziksel bir kahramanı değil, **yaşayan 60 evrenlik bir havuzu** yönetiyorsun. Ölümlü uygarlıklar senin adını unutmadan önce mi onları kendine bağlayacaksın, yoksa onları birer kaynak olarak mı göreceksin?

Sen yokken bile evrenler dönmeye devam eder. Uygarlıklar doğar, doktrinlerine göre bölünür, rakip kültlere sapar, birbirini katleder. Sekiz rakip tanrı seninle birlikte aynı havuz için plan yapar — ve senin uyuduğunu fark ettiklerinde hiç nazik davranmazlar.

---

## Neden KozMO?

- 🌌 **60 evren hiç durmaz.** Simülasyon yalnızca ekranda akan bir animasyon değil; uzak evrenler düşük frekansta arka planda evrilmeye devam eder. Geri döndüğünde harita seni bekliyor olmayacak — değişmiş olacak.
- 👑 **8 rakip tanrı, 8 ayrı hırs.** Her biri kendi arketipi, başkent evreni, mühür sembolü ve yapay zekâ karar döngüsüyle hareket eder. Diplomasi masasında dost olabilirler; sen zayıfladığın anda ilk mühürü onlar basar.
- 🧠 **Uygarlıklar kendi iradesine sahip.** Beş doktrin (`Bilge`, `Tapınakçı`, `Kuşkucu`, `Hanedan`, `Mistik`) sadece sayı değiştirmez; bir uygarlığın sana tapıp tapmayacağını, bilimsel kalkanını, isyan eşiğini ve mezhep savaşlarındaki tarafını belirler.
- ⚔️ **Mezhep savaşları gerçekten kan döker.** İki tanrı aynı evrende %25 nüfuzu aşarsa uygarlıklar kültlere bölünür. Ruh havuzu ölülerle dolar, kazanan ilah +%8 nüfuz kazanır — kaybeden ise tarihe gömülür.
- 🏛️ **Her evrenin senden önce bir geçmişi var.** Kadim Çağ arkeolojisi, sana yeni girdiğin bir evrenin neden harabeye döndüğünü anlatır.
- 🎼 **Sıfır ses dosyası, tam kozmik atmosfer.** Tüm sesler çalışma zamanında sentezlenir: kehanet çanları, mucize akorları, ilahi darbe gök gürültüsü, zafer fanfarı. Kurulum derdi yok, tek ikili dosya yeter.
- 💾 **Kaydın kutsaldır.** Sürümlendirilmiş kayıt şeması, otomatik şema göçü ve kısmi yazmaya karşı atomik dosya kaydı. `F5` ile kaydet, `F9` ile geri dön.

---

## Oynanış Döngüsü

> **Kehanet yay → Ruh hasat et → İlahi gücünü aç → Evreni şekillendir → Rakibi mühürle.**

Her tur bir karar, her karar bir iz bırakır. 20'den fazla ilahi emir arasında seçim yaparsın: kehanet yaymak, ruh hasadı yapmak, mucize bahşetmek, yaşam tohumlamak, gerçekliği onarmak, kozmik öz damıtmak, isyan çıkarmak, geçit açmak, haraç talep etmek, kutsal savaş ilan etmek, rakip aşkınlık ritüelini sabote etmek…

Kazanamazsın. **Üç farklı yoldan kazanırsın:**

| Zafer Yolu | Şart |
| :--- | :--- |
| **👑 Hâkimiyet** | En az **35 evrende** baskın ol, **18 evrende ★ Tam Aidiyet** kur ve **5 rakip tanrıyı mühürle**. |
| **✨ Aşkınlık** | 10 aşamalı **Aşkınlık Ritüelini %100'e** ulaştır, en az **10 ilahi güç** aç ve **10 ★ Tam Aidiyet** evrenine sahip ol. |
| **🌱 Yaratım** | **Yaratım Ağacını (3/3)** tamamla; en az **15 evrende** ★ Tam Aidiyet, **%75+ istikrar** ve aktif uygarlıkla kendi düzenini kur. |

### Seni ne bekliyor?

- **6 evren kategorisi** — Yaşam, Kaos, İnanç, Teknoloji, İlksel ve Çöken. Her kategori farklı ekonomi, farklı risk, farklı fırsat.
- **15 ilahi güç / 5 gelişim ağacı** — Her güç açıldığında anlık bir kozmik patlama, kalıcı bir pasif bonus ve emir indirimi getirir.
- **20+ ilahi emir** — Ekonomi, diplomasi, sabotaj, savaş ve yaratım aynı arayüzde.
- **Rastgele kozmik olaylar ve ikilemler** — 2–6 turda bir kehanetler, anomaliler, gerçeklik çöküşleri ve ahlaki ikilemler. Her birinin bir bedeli var.
- **Ruh ekonomisi tamamen izlenebilir** — Ruh, Nüfuz ve Kozmik Öz; her kaynağın kaynağı, harcaması, transferi ve kaybı `ResourceLedger` üzerinde kayıt altında.
- **4 analitik harita katmanı (`M`)** — Standart Kozmos, İlahi Hâkimiyet & Bloklar, Ekonomi & Ruh Havuzu, İstikrar & Tehlike Isı Haritası. Haritayı okumayı öğren, savaşı kazan.
- **4 kozmik atmosfer teması (`T`)** — Safir Mor & İlahi Kehribar, Zümrüt Yeşim & Yıldız Tozu, Kızıl Kehribar & Süpernova, Ebedi Mermer & Altın Usturlap.
- **Prosedürel ses sentezleyicisi (`V`)** — 9 farklı kozmik ses kategorisi, tamamı bellek içinde üretilir.
- **3 zorluk seviyesi** — Gözlemci (Kolay), Kozmik Denge (Normal), Ebedî Kaos (Zor).
- **2 oyun modu** — Senaryolu **Kozmik Yükseliş** ve sınırsız **Sonsuz Havuz (Sandbox)**.
- **Eğitici görev zinciri + 8 bölümlük oyun içi ansiklopedi (`F1`)** — İlk turdan itibaren ne yapacağını bilirsin; ama nasıl kazanacağını sen seçersin.

---

## Kontroller

| Tuş | İşlev |
| :--- | :--- |
| `Space` | Zamanı duraklat / devam ettir |
| `1` / `2` / `3` | 1x Normal / 2x Hızlı / 5x Kozmik hız |
| `N` | Simülasyonu tek tur ilerlet |
| `Tab` | Denetçi sekmeleri arasında geç |
| `W A S D` / `Ok tuşları` | Haritayı kaydır |
| `+` / `-` | Yakınlaş / uzaklaş |
| `F` | Seçili evrene odaklan |
| `E` | Son olayın evrenine odaklan |
| `M` | Harita görünüm katmanını değiştir |
| `T` | Arayüz temasını değiştir |
| `V` | Sesi aç / kapat |
| `F1` veya `H` | KozMO Wiki ve eğitici |
| `F5` / `F9` | Hızlı kaydet / hızlı yükle |
| `Esc` | Menü / geri |

---

## Hızlı Başlangıç

### 1. Ön gereksinimler

- **Rust 1.80+** (kararlı) — [`rustup`](https://rustup.rs/) ile kurun.
- **Linux sistem kütüphaneleri** (Bevy pencere ve ses desteği):

```bash
# Ubuntu / Debian / Linux Mint
sudo apt-get update
sudo apt-get install -y pkg-config libx11-dev libxi-dev libxtst-dev libxkbcommon-dev libwayland-dev libasound2-dev libudev-dev

# Fedora / RHEL
sudo dnf install -y pkgconf-pkg-config libX11-devel libXi-devel libxkbcommon-devel wayland-devel alsa-lib-devel systemd-devel

# Arch Linux / Manjaro
sudo pacman -S --needed pkgconf libx11 libxi libxkbcommon wayland alsa-lib systemd-libs
```

### 2. Derle ve oyna

```bash
git clone https://github.com/Padrosum/KozMO.git
cd KozMO

# Hemen oyna:
cargo run --release -p dominion-game
```

Yalnızca ikili dosyayı üretmek istersen:

```bash
cargo build --release -p dominion-game   # -> ./target/release/dominion-game
```

> Yazı tipleri ve prosedürel ses sentezleyicisi doğrudan ikili dosyaya gömülüdür; oyunu proje kökünden ya da `target/release/` içinden çalıştırabilirsin.

**İndirme paketleri:** Her sürümde Linux (`.tar.gz`), Windows (`.zip`) ve macOS ARM64 (`.tar.gz`) paketleri [Releases](https://github.com/Padrosum/KozMO/releases) sayfasında yayınlanır.

---

## Mimari

Simülasyon çekirdeği grafik çalışma zamanından tamamen bağımsızdır. Bevy'yi silip yerine başka bir görüntüleyici koysan bile evrenler aynı şekilde evrilir — çünkü oyun döngüsü Rust'ın kendisinde.

```
KozMO/
├── crates/dominion-sim/      # Deterministik simülasyon çekirdeği (Bevy bağımlılığı YOK)
├── crates/dominion-content/  # Veri odaklı içerik: evren profilleri, güçler, senaryolar, Wiki
├── crates/dominion-game/     # Bevy 0.20 sunum katmanı: harita, arayüz, ses, kayıt köprüsü
├── assets/                   # SVG semboller, gömülü yazı tipleri, içerik tanımları
└── tests/                    # Motordan bağımsız ekonomi, simülasyon ve kayıt testleri
```

| Crate | Sorumluluk |
| :--- | :--- |
| `dominion-sim` | Tohumlanabilir `Xoroshiro128++` RNG, 6 aşamalı sabit güncelleme sırası (`step_tick`), evren tarihi, uygarlıklar, tanrılar, ekonomi defteri, olaylar, sürümlendirilmiş kayıt (`v2`, `v1 -> v2` göçü) |
| `dominion-content` | Evren türü profilleri, 15 ilahi güç tanımı, 3 senaryo şablonu, 8 bölümlük oyun içi ansiklopedi |
| `dominion-game` | 4 harita katmanı, 4 atmosfer teması, prosedürel ses sentezleyici, tam Türkçe karakter destekli arayüz, `F5`/`F9` kayıt köprüsü |

Tasarımın tamamı için: [`TASARIM.md`](TASARIM.md) · [`REHBER.md`](REHBER.md) · [`Arayüz.md`](Arayüz.md) · [`Geliştirme_Yol_Haritası.md`](Geliştirme_Yol_Haritası.md)

---

## Kalite ve Test

```bash
cargo test --workspace                       # simülasyon, ekonomi, ses ve kayıt testleri
cargo fmt --all -- --check                   # biçimlendirme
cargo clippy --workspace --all-targets -- -D warnings   # lint
```

Her `push` ve `pull_request` işleminde **CI** (`cargo fmt`, `cargo clippy`, `cargo test --workspace`) çalışır; ardından üç platform için optimize edilmiş paketler üretilir. `v*` etiketi gönderdiğinde (`git tag v0.2.0 && git push origin v0.2.0`) bu paketler otomatik olarak GitHub Releases'e düşer.

Testler grafik çalışma zamanından bağımsız çalışır; simülasyon determinizmi tohumlanmış RNG ile doğrulanır.

---

## Yol Haritası

- [x] **Faz 1 — Kozmik Çekirdek:** Simülasyon, ekonomi, tanrılar, zafer yolları
- [x] **Faz 2.1 — Dokümantasyon Senkronizasyonu**
- [x] **Faz 2.2 — Doktrinler, Rakip Kültler, Mezhep Savaşları**
- [x] **Faz 2.3 — Kadim Çağ Arkeolojisi, 4 Harita Katmanı, Mühür Geometrisi**
- [x] **Faz 2.4 — Prosedürel Kozmik Ses Sentezleyicisi**
- [ ] **Sonraki:** yeni senaryolar, daha derin diplomasi, performans optimizasyonu

Ayrıntılar: [`Geliştirme_Yol_Haritası.md`](Geliştirme_Yol_Haritası.md)

---

## Katkı

Hata bildirimi, fikir ve PR'lar memnuniyetle karşılanır. Katkıda bulunmadan önce lütfen [`AGENTS.md`](AGENTS.md) içindeki geliştirme kurallarına göz atın: simülasyon çekirdeği Bevy'den bağımsız kalmalı, değişiklikler `cargo fmt` + `cargo clippy` + `cargo test` ile doğrulanmalı ve mevcut kayıtlar bozulmamalıdır.

## Lisans

Kod tabanı **GNU GPL-3.0-or-later** ([`LICENSE`](LICENSE)) ile dağıtılmaktadır. Sanat ve yazı tipi varlıklarının lisansları [`assets/ASSET_LICENSES.md`](assets/ASSET_LICENSES.md) dosyasında kayıtlıdır.

---

<p align="center"><em>Evrenler seni beklemiyor. Şimdiden evriliyorlar.</em></p>
