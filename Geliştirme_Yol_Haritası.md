# KozMO — Faz 2.2 ve 2.3 Geliştirme Yol Haritası ve Tasarım Taslağı (`Geliştirme_Yol_Haritası.md`)

> **Durum:** `Faz 2.1–2.4 tamamlandı; simülasyon modülerleştirmesi sürüyor`
> **Kapsam:** **Adım 2.1 (Tüm Markdown Belgelerinin ve Lisans Kayıtlarının KozMO v0.2 ile Senkronizasyonu)**, **Faz 2.2 (Derinleştirilmiş Uygarlık Doktrinleri, Rakip Kültler ve Mezhep Savaşları)**, **Faz 2.3 (Başlangıç Arkeolojisi, Harita Görünüm Katmanları `M` ve Başkent Mühür Geometrisi)** ve **Faz 2.4 (Prosedürel Kozmik Ses / Sentezleyici Geri Bildirimi `V` / `♪ Ses`)** simülasyon çekirdeğine ve arayüze eksiksiz uygulanmış ve birim testleriyle doğrulanmıştır.

---

## Faz 2.2 — Derinleştirilmiş Uygarlık Doktrinleri, Rakip Kültler ve Mezhep Çatışmaları

[`taslak.md` Bölüm 2 & 3](file:///home/padros/AppImages/KozMO/taslak.md#L51-L184) içerisinde vurgulanan *"Bir teknoloji evreninde çok güçlü dinler, bir inanç evreninde ise tanrılara karşı çıkan gelişmiş uygarlıklar bulunabilir"* ve *"İnanç yayılımı, kültler, ayrılıklar ve mezhep savaşları"* vizyonunu simülasyon çekirdeğine (`dominion-sim`) taşımak için planlanan yapı:

### 2.2.1. Veri Modeli (`crates/dominion-sim/src/civilization.rs`)
Her ölümlü uygarlığa (`Civilization`), bulunduğu evren türünden bağımsız olarak sürprizli bir kimlik kazandıran **`CivilizationDoctrine` (Uygarlık Doktrini)** ve **Aktif Kült Durumu** eklenecektir:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CivilizationDoctrine {
    #[default]
    HarmoniousPhilosophers, // Uyumcu Filozoflar: +İstikrar direnci, dengeli ibadet ve bilim
    ZealousTemplars,        // Bağnaz Tapınakçılar: +%25 İbadet Ruhu/Nüfuzu, rakip kültlere sert tepki
    ScientificSkeptics,     // Bilimsel Şüpheciler: +%30 Teknoloji hızı, erken Bilimsel İlahi Kalkan
    WarriorDynasty,         // Savaşçı Hanedan: +%25 Savaş/Ölüm Ruhu, yüksek militarizm
    VoidMystics,            // Hiçlik Mistikleri: Kaos ve Çöküş evrenlerinde istikrar kaybına bağışık, +Öz rezonansı
}
```

### 2.2.2. Rakip Kültler ve Mezhep Savaşları (`crates/dominion-sim/src/simulation.rs`)
1. **Çift Kutuplu İnanç Çatışması:**
   - Bir evrende hem baskın ilahın (`dominant_deity`) hem de ikinci sıradaki rakip ilahın nüfuzu **`%25` eşiğini** aşarsa, o evrendeki uygarlıklar iki farklı ilahın kültüne bölünür (`patron_deity` rekabeti).
2. **Mezhep Savaşı (`ReligiousSchism`):**
    - Farklı `patron_deity`'ye tapan iki uygarlık aynı evrende bulunduğunda evren/tur eşleşmesine göre her 12 turda bir çatışma çözülür.
    - Çatışma çıktığında evrenin **Ruh Havuzu (`soul_reservoir`)** `+18` artar; kazanan ilah `+6` nüfuz, kaybeden ilah `-5` nüfuz alır ve olay evren tarihçesine kaydedilir.
3. **Yeni Oyuncu Etkileşimi / Stratejik Karar:**
   - Sağ denetçide (`Evren` sekmesi) her uygarlığın doktrini (`[Bağnaz Tapınakçılar]`, `[Bilimsel Şüpheciler]` vb.) ve hangi ilahın kültüne bağlı olduğu açıkça gösterilir.
   - `☀ Uygarlığı Aydınlat` ve `Mucize Bahşet` emirleri, rakip külte kapılmış uygarlıkları barışçıl yolla kendi inancınıza döndürür (`patron_deity = Some(PLAYER_DEITY_ID)`).

---

## Faz 2.3 — Zenginleştirilmiş Evren Arkeolojisi, Harita Görünüm Katmanları (`M`) ve SVG Mühürleri

[`taslak.md` Bölüm 3](file:///home/padros/AppImages/KozMO/taslak.md#L177-L184) (*"Oyuncu bir evrene ilk kez girdiğinde bile onun geçmişine dair bir şeyler keşfedebilir"*) ve [`TASARIM.md` Bölüm 5 & 6](file:///home/padros/AppImages/KozMO/TASARIM.md#L140-L170) (*"Haritada tanrısal etkinin kapsamını gösteren görsel katman"*) hedefleri için planlanan yapı:

### 2.3.1. Prosedürel Başlangıç Arkeolojisi (`UniverseHistoryEntry`)
- Evren havuzu oluşturulurken (`SimulationState::generate_universes`), her evrene yalnızca tek satırlık `"Evren kozmik havuzda belirdi"` yerine, **Tur 0 öncesi Kadim Çağ Arkeolojisi** (2–3 geçmiş olay kaydı) üretilir:
  - *Örn:* `"[Kadim Çağ] Solinvictus ile Nyxarion arasında İlk Kırılma Savaşı yaşandı; eski tapınaklar harabeye döndü."`
  - *Örn:* `"[Kadim Çağ] Ölümlü bilginler boyutsal geçidi mühürleyerek Hiçlik sızıntısını durdurdu."`
- Böylece oyuncu yeni keşfettiği bir evrene tıkladığında, o evrenin neden yüksek ruh havuzuna veya neden düşük istikrara sahip olduğunu tarihçesinden okuyabilir.

### 2.3.2. Dörtlü Harita Görünüm Katmanı (`MapOverlayMode` — Kısayol: `M`)
Üst çubuğa ve `M` kısayol tuşuna, 60 evrenlik haritayı stratejik açıdan okumayı kolaylaştıran **4 Harita Görünüm Katmanı** eklenecektir:

| Katman | Kısayol / Buton | Harita Üzerindeki Görsel Davranış |
| :--- | :--- | :--- |
| **1. Standart Kozmos** | `M` (Varsayılan) | Evren türleri, `★ Tam Aidiyet` taçları, uygarlık uyduları ve geçit akışları. |
| **2. İlahi Hâkimiyet & Bloklar** | `M` -> Katman 2 | Her evren düğümü, baskın olan ilahın (Oyuncu veya `#1..#8` Rakip) mühür renginde kalın dış halka ile boyanır; aynı ilaha ait komşu evrenler arasındaki sınır bağları belirginleşir. |
| **3. Ekonomi & Ruh Havuzu** | `M` -> Katman 3 | Evrenlerin etrafında biriken `Ruh Havuzu` (`soul_reservoir`) ve `Öz Zenginliği` (`essence_richness`) büyüklüğüne göre parlayan altın/mor bolluk halkaları gösterilir (Hasat hedeflerini tek bakışta bulmayı sağlar). |
| **4. İstikrar & Tehlike Isı Haritası** | `M` -> Katman 4 | İstikrarı yüksek (`>=%75`) evrenler zümrüt yeşili, tehlikesi yüksek veya çökmekte olan evrenler titreşen kızıl alarm halkalarıyla vurgulanır. |

### 2.3.3. Tanrısal Mühür Geometrisi (`DeitySeal` Harita & Denetçi Entegrasyonu)
- `Tanrılar` sekmesinde seçili rakip ilahın (`#1 Vaelkor` .. `#8 Vespera`) ve oyuncunun kendine özgü geometrik mühür sembolü ([`TASARIM.md` Bölüm 5](file:///home/padros/AppImages/KozMO/TASARIM.md#L140-L155) ve `assets/art/deities/`), o ilahın başkent evreni (en yüksek nüfuzlu kalesi) üzerinde özel bir **Başkent Mühür Tacı** olarak çizilecektir.

---

## Faz 2.4 — Prosedürel Kozmik Ses ve Sentezleyici Geri Bildirimi (`crates/dominion-game/src/audio/mod.rs`)

Harici ses dosyalarına bağımlı olmadan doğrudan Bevy'nin ses altyapısı (`bevy_audio::Decodable`) üzerinde çalışan bellek içi **Prosedürel Çok-Harmonikli Sentezleyici (`CosmicAudioPlugin`)**:
- **9 Prosedürel Kozmik Ses Kategorisi (`CosmicSfxKind`):**
  1. `UiClick`: Obsidyen/kehribar arayüz tıklama tınısı (`528 Hz + 792 Hz`).
  2. `UniverseSelect`: Evren düğümü seçildiğinde çalan sıcak beşli/oktav akoru (`432 Hz + 648 Hz + 864 Hz`).
  3. `ProphecyChime`: Kehanet yayma, aydınlatma ve pakt tekliflerinde yükselen pentatonik arpej (`440 Hz -> 880 Hz`).
  4. `SoulHarvest`: Ruh hasadı ve kozmik öz çıkarımında derin frekans süzülmesi (`396 Hz -> 220 Hz`).
  5. `MiracleChord`: Mucize, yaşam tohumlama, gerçeklik onarımı ve ilahi güç açılışında 5 sesli majör-9 koro akoru.
  6. `DivineStrike`: Rakip vurma, kutsal savaş ve mühürlemede düşük frekanslı kozmik gök gürültüsü darbesi (`185 Hz -> 65 Hz`).
  7. `GatewayPulse`: Kozmik geçit açma/kapatma ve keşifte boyutsal yukarı yönlü frekans dalgası.
  8. `CosmicEventAlert`: Rastgele kozmik olaylar, ikilemler ve eğitici görev tamamlanmalarında çift tonlu astral tapınak çanı.
  9. `VictoryFanfare`: Üç zafer yolundan birine ulaşıldığında çalan 5 notalı yükseliş fanfarı.
- **Kontrol:** Üst çubuktaki **`♪ Ses`** butonu, Ana Menü Ayarları veya **`[V]`** klavye kısayolu ile anında açılıp kapatılabilir.

---

## Uygulama ve Doğrulama Planı (`AGENTS.md` Kuralları)

1. **Kayıt Uyumluluğu (`#[serde(default)]`):** Yeni eklenen tüm alanlar (`CivilizationDoctrine` vb.) `#[serde(default)]` ile işaretlenerek mevcut `v1` ve `v2` kayıt dosyalarının sorunsuz yüklenmesi korunmuştur.
2. **Determinizm ve Testler:** `dominion-sim` Bevy'den tamamen bağımsız kalmış; yeni doktrin, mezhep çatışması, arkeoloji ve prosedürel ses sentezleyici mekanikleri için birim/entegrasyon testleri eklenmiştir.
3. **Kalite Kontrolü:** Son doğrulamada `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings` ve `cargo test --workspace` başarıyla tamamlanmıştır (14 test).

---

## Faz 2.5 — Simülasyon sorumluluklarını modüllere ayırma (devam ediyor)

- Tur fazlarının sırası sözleşme olarak belgelenmiştir: evren/uygarlık evrimi → ekonomi → kozmik olaylar → rakip kararları → diplomasi → sonuç değerlendirmesi.
- Diplomasi süresi güncellemesi ve zafer/yenilgi değerlendirmesi `crates/dominion-sim/src/simulation/endgame.rs` alt modülüne taşınmıştır.
- Pakt ve mühür haraç gelirinin saf hesabı `crates/dominion-sim/src/simulation/economy.rs` alt modülüne ayrılmıştır. Ekonomi fazının geri kalanı ve diğer fazlar henüz `simulation.rs` içindedir.
- Bu düzenleme oyun kuralı veya kayıt biçimi değiştirmez. Modüllere ayırırken deterministik tekrar oynatma ve kayıt/yükleme testleri korunur.

---

### Onay Durumu
- [x] **Faz 2.1, 2.2, 2.3 & 2.4 Onayı:** `ONAYLANDI VE OYUN İÇİNE TAMAMEN UYGULANDI`
