# KozMO — Arayüz, Tema ve Görsellik Tasarım Önerisi (`Arayüz.md`)

> **Durum:** `ONAYLANDI — Oyuncu Tarafından Onaylandı ve Uygulandı`  
> **Oyun Adı:** **KozMO** (*Kozmik Sandbox Strateji — 60 Evren · 9 İlah · 5 Güç Ağacı · 3 Zafer Yolu*)  
> **Onay Süreci:** Bu doküman oyunun mimarisi (`dominion-sim`, `dominion-content`, `dominion-game`) ve oynanış döngüsü çerçevesinde hazırlanmış, oyuncu tarafından onaylanmış ve doğrudan oyuna uygulanmış resmi arayüz, tema ve görsellik tasarım spesifikasyonudur.

---

## 1. Tasarım Vizyonu: "Yaşayan Usturlap & İlahi Gözlemevi"

**KozMO**, oyuncuyu 60 bağımsız evrenin, ölümlü uygarlıkların, boyutsal geçitlerin ve 8 rakip ilahın üzerinde hüküm süren bir kozmik varlık konumuna yerleştirir. Bu nedenle arayüz; sıradan bir tablo/menü görünümü yerine **"İlahi Gözlemevi ve Kozmik Usturlap (Celestial Astrolabe)"** hissiyatı vermelidir:

1. **Harita Merkezli Hiyerarşi (%70 Görsel Alan):** Ortadaki interaktif 60 evrenlik kozmik ağ her zaman sahnenin yıldızı olmalı; yan paneller yarı saydam derin uzay camı (*Obsidian Glass*) dokusuyla haritayı çerçevelemelidir.
2. **Sıfır Kayma / Sıfır Titreme (Zero-Jitter Ergonomisi):** Metin uzunluğu veya tur değişimi hiçbir zaman butonların yerini kaydırmamalı; yalnızca Ana Menü kahraman butonları ve harita üzerindeki kozmik efektler akıcı şekilde nefes almalıdır.
3. **2 Saniye Beklemeli Akıllı İpucu (Hover Tooltip) & Entegre Wiki:** Oyuncu herhangi bir interaktif butonun üzerinde ~2 saniye durduğunda, o emrin maliyetini, hâkimiyet şartını ve stratejik etkisini anlatan yüzer bilgi kartı açılmalıdır.

---

## 2. Önerilen Tema Sistemi (4 Kozmik Atmosfer Paleti)

Mevcut `T` kısayolu / `❖ Tema` butonu ile geçiş yapılan tema sisteminin, harita arka planını, geçit akış renklerini ve panel kenarlıklarını birlikte değiştiren **4 tam atmosferik temaya** dönüştürülmesi önerilmektedir:

| Tema No | Tema Adı | Arka Plan / Boşluk | Panel & Buton Dokusu | Ana Vurgu (Hâkimiyet) | İkincil Vurgu (Akış/Öz) | Tehlike / Rakip |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Tema 1** | **Safir Mor & İlahi Kehribar** *(Varsayılan)* | `#07050F` (Derin Hiçlik) | `#161026` / `#231B3A` | `#FFB84D` (İlahi Kehribar) | `#C46BFF` (Safir Ametist) | `#FF4F79` (Kızıl Yırtık) |
| **Tema 2** | **Zümrüt Yeşim & Yıldız Tozu** | `#040D10` (Nebula Yeşili) | `#0D2024` / `#143238` | `#2EE5B5` (Zümrüt Yeşim) | `#63D2FF` (Yıldız Turkuazı) | `#FF6B5B` (Mercan Alevi) |
| **Tema 3** | **Kızıl Kehribar & Süpernova** | `#11060B` (Kızıl Kozmos) | `#220E18` / `#381627` | `#FFC857` (Güneş Altını) | `#FF5D8F` (Süpernova Pembesi) | `#FF3355` (Kan Kızılı) |
| **Tema 4** | **Ebedi Mermer & Altın Usturlap** *(Yeni Öneri)* | `#0B0E17` (Gece Laciverti) | `#171C2B` / `#242C42` | `#F4D06F` (Saf Altın) | `#8CE0FF` (Aether Mavisi) | `#E84855` (Kaos Kırmızısı) |

---

## 3. Tipografi ve Sembol Dili Önerisi

- **Gömülü Çift Ağırlıklı Font Mimarisi (`DejaVuSansCondensed` & `Bold`):**
  - Çalışma zamanında dosya yolu hatası veya eksik glif (tofu kutucukları) yaşanmaması için fontlar doğrudan ikili dosyaya (`include_bytes!`) gömülüdür.
  - Tüm Türkçe karakterler (`ğ, ü, ş, ı, ö, ç, İ, Ğ, Ü, Ş, Ö, Ç`) ve kozmik semboller (`✦, ★, ♛, ◆, ⇄, ⚔, ⚙, ☀, ☸, ⚖, ◎, ⊗, ❖, ↻, ▲, ✓, ✖`) %100 desteklenir.
- **Önerilen Görsel Hiyerarşi:**
  - **Ana Başlık (`✦ KozMO ✦`):** `22px Bold`, Altın Kehribar (`#FFB84D`).
  - **Üst Kaynak Hapları (Ruh, Nüfuz, Öz, Zafer):** `11px Bold`, tur başı net değişim (`+24/tur`) renk kodlu (artışta zümrüt, düşüşte kızıl).
  - **Denetçi ve Wiki Metinleri:** `10.3px – 10.8px Regular`, yüksek okunabilirlikli Fildişi (`#F8F3E6`).

---

## 4. Sahne ve Panel Yerleşimi (HUD Mimarisi)

```
+--------------------------------------------------------------------------------------------------+
| [♛ İlah Adı · Tur · Hız] [◆ Ruh (+/t)] [♛ Nüfuz (+/t)] [✦ Öz (+/t)] [★ Zafer] | [Menü][Hız][Wiki] |
+----------------------+----------------------------------------------------+----------------------+
| SOL PANEL (244px)    |                                                    | SAĞ DENETÇİ (384px)  |
|                      |                                                    | [Evren][Güç][Tanrı]  |
| KOZMİK HAVUZ (60)    |              2D İNTERAKTİF KOZMOS                  | [Zafer][Wiki (F1)]   |
| [Tümü][★Tam][◆Bizim] |                                                    +----------------------+
| [⚔Rakip][○Nötr][▲Thl]|         • 3 Konsantrik Kozmik Halka                | [★ Görev / İkilem]   |
| [Kategori↻][Sırala↻] |         • ★ Tam Aidiyet: Altın Taç & Çift Halka   | [3 Canlı Durum Barı] |
|                      |         • ⇄ Geçitler: Akan İnanç/Ruh Küreleri      +----------------------+
| #1 [★ TAM] Evren A   |         • Uygarlık Yörünge Uyduları                | Denetçi / Wiki Metni |
| #2 [◆ BİZİM] Evren B |         • Anlık VFX Halkaları & Yüzen Bildirimler  | (Sabit Boyutlu Kutu) |
| ... (8 Yuva/Sayfa)   |                                                    +----------------------+
| [< Önceki] [Sonraki >|  +----------------------------------------------+  | Sızma Emirleri (Açık)|
|                      |  | ★ 2sn Buton Açıklama Kartı (Hover Tooltip)   |  | Hâkimiyet Emirleri   |
+----------------------+--+----------------------------------------------+--+----------------------+
| ALT KOZMİK OLAY GÜNLÜĞÜ (Son 3 Olay + Anlık Durum + [E] Son Olaya Odaklan)                       |
+--------------------------------------------------------------------------------------------------+
```

---

## 5. Kozmik Harita ve Görsellik Geliştirmeleri (Oyuna Uygulanan Maddeler)

Onayınız doğrultusunda mevcut arayüz ve harita üzerinde şu görsel zenginleştirmelerin tamamı devreye alınmıştır:

1. - [x] **Evren Türüne Göre Özel Geometrik Çekirdek, Aura ve Uygarlık Yörünge Uyduları (`map/mod.rs`):**
   - **Genç / Yaşam Evreni:** Çift katmanlı parlayan yıldız çekirdeği ve hızlı dönen yaşam halkası.
   - **Kadim / İlkel Evren:** Çokgen bilgelik mührü ve yavaş salınan altın-mor hale.
   - **Kaos / Anomali Evreni:** Düzensiz frekansta titreşen prizmatik halka ve mor/kızıl enerji kıvılcımları.
   - **Hiçlik / Çökmekte Olan Evren:** İçe doğru büzülen koyu kızıl girdap halkası.
   - **Uygarlık Yörünge Uyduları:** Her keşfedilen evrenin etrafında, barındırdığı uygarlık sayısı kadar yörünge uydusu döner (Bilimsel İlahi Kalkan açıkken çift turkuaz halka, isyan durumunda kızıl uyarı rengi).
2. - [x] **İnteraktif Buton İpucu Kartı — Canlı Maliyet ve Hâkimiyet Durumu (`ui/mod.rs`):**
   - Bir butonun üzerinde ~2 saniye beklerken açılan açıklama kartında, emrin **Hâkimiyet Durumu** (`✓ KULLANILABİLİR` / `⊗ HÂKİMİYET KİLİTLİ` / `▲ KAYNAK YETERSİZ`), **Net Kaynak Maliyeti** (açık güç indirimleri düşülmüş gerçek maliyet) ve **Kısayol Tuşu** gösterilir.
3. - [x] **Wiki İçerisinde Konu Başlığı Etiketleri (`ui/mod.rs` & `guide.rs`):**
   - Sağ paneldeki `Wiki (F1)` sekmesinde 8 ana bölüm butonunun üzerinde bölüm kısa isimleri (`1:Evrenler`, `2:Ekonomi`, `3:Emirler`, `4:Güçler`, `5:Türler`, `6:Olaylar`, `7:İlahlar`, `8:Zafer`) doğrudan yer alır.
4. - [x] **Görsel Ekonomi & Zafer Özeti Mini-Grafiği (`ui/mod.rs`):**
    - `Zafer` sekmesinde 10 bölmeli kozmik yoğunluk çubukları (`[◆◆◆◆◆◆○○○○]`) ve üst bardaki kaynak haplarında tur başı Ruh/Nüfuz/Öz akış yönünü gösteren renk kodlu göstergeler (`▲` zümrüt/altın artış, kızıl düşüş) yer alır.
5. - [x] **Kozmik Harita Görsel Cilası (`map/mod.rs`):**
    - Tema zeminini görünür bırakan şeffaf, mor/turkuaz/kehribar nebula dokusu; sabit konumlu, yumuşakça parıldayan çok katmanlı yıldız alanı.
    - Seçili evrende pusula işaretli çift vurgu halkası.
    - Keşfedilmiş evrenlerde tür rengi/hâkimiyet rengine göre düşük yoğunluklu aura, uygarlık uydularının altında ince yörünge izleri.
    - Açık geçitlerde animasyon zamanına bağlı enerji parçacığı dizileri; duraklatıldığında akış donar. Keşfedilmemiş evrenlerin türü ve hâkimiyeti görsel olarak açığa çıkarılmaz.

---

## 6. Onay Durumu

- [x] **Oyuncu Onayı:** `ONAYLANDI VE OYUN İÇİNE TAMAMEN UYGULANDI` *(4'lü Kozmik Atmosfer Teması (`T`), 4'lü Harita Analiz Katmanı (`M`), Prosedürel Kozmik Ses Sentezleyicisi (`V` / `♪ Ses`), Evren Çekirdek/Aura, Başkent Mühür Tacı ve Uygarlık Yörünge Uyduları, Canlı İndirimli Maliyetli 2sn Buton Açıklama Kartı, Konu Başlıklı 8 Bölümlü Wiki (`F1`) ve Görsel Yoğunluk Çubukları aktif durumdadır.)*

## 7. Simülasyonla arayüz arasındaki sınır

Arayüz görsel seçim, gösterim ve girdi katmanıdır; kurallar Bevy'den bağımsız `dominion-sim` içinde uygulanır. Simülasyonun sabit tur sırası; evrim, ekonomi, olaylar, rakip kararları, diplomasi ve sonuç değerlendirmesini izler. Sonuç değerlendirmesi `simulation/endgame.rs` modülüne ayrılmıştır. Arayüzde gösterilen kaynak akışları, eylem sonuçları ve zafer durumu simülasyon durumundan alınmalı; arayüz tarafında ikinci bir kural hesabı oluşturulmamalıdır.
