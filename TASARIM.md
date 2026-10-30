# Eternal Dominion — TASARIM.md

**Belge sürümü:** 1.0  
**Proje:** Eternal Dominion  
**Motor:** Bevy + Rust  
**Kod lisansı:** GNU GPL-3.0-or-later  
**Birincil çizim formatı:** SVG  
**Görsel üretim aracı:** Inkscape  
**Hedef:** Tutarlı, özgün, yapay zekâyla üretilebilir ve kolayca düzenlenebilir bir görsel sistem oluşturmak.

---

## 1. Tasarım vizyonu

Eternal Dominion, çoklu evrenleri yöneten tanrıların güç mücadelesini konu alan kozmik bir strateji ve sandbox oyunudur.

Oyuncu dünyaya fiziksel bir avatar aracılığıyla değil, kozmik ölçekte müdahalelerde bulunarak hükmeder. Bu nedenle görsel tasarım, tek tek karakterlerden ziyade evrenleri, tanrıları, medeniyetleri, kaynakları ve aralarındaki ilişkileri ön plana çıkarmalıdır.

Görsel dil şu özellikleri taşımalıdır:

- Kozmik, gizemli ve yer yer okült bir atmosfer.
- Sade, geometrik ve kolay tanınabilir semboller.
- Karanlık arka planlar üzerinde kontrollü ışık kullanımı.
- Yoğun bilgi içeren ama okunabilir bir strateji arayüzü.
- Prosedürel üretime uygun, modüler görsel bileşenler.
- Yapay zekânın üretebileceği, insanın düzenleyebileceği SVG dosyaları.
- Gereksiz görsel karmaşadan kaçınan bir tasarım.

**Temel ilke:** Görseller yalnızca güzel görünmek için değil, oyun durumunu anlatmak ve oyuncunun karar vermesini kolaylaştırmak için kullanılmalıdır.

## 2. Görsel kimlik

### 2.1. Atmosfer

Ana atmosfer karanlık kozmik fantezidir. Bilimkurgu ve mistik sembolizm bir arada bulunabilir.

Görsel referans kavramları:

- Derin uzay ve kozmik boşluk.
- Kozmik ağlar ve birbirine bağlanan evrenler.
- Kadim geometrik mühürler.
- Soyut tanrısal varlıklar.
- Kozmik toz, yıldız kümeleri ve ince ışık çizgileri.
- Medeniyetlerin yükselişini ve çöküşünü simgeleyen işaretler.

Belirli bir oyunun, sanatçının veya fikrî mülkiyetin görsel kimliği kopyalanmayacaktır. Referanslar yalnızca atmosfer ve genel tasarım ilkeleri için kullanılacaktır.

### 2.2. Renk paleti

Başlangıç paleti:

| Kullanım | Renk | HEX |
|---|---|---|
| Ana arka plan | Kozmik siyah | `#080B12` |
| İkincil arka plan | Derin lacivert | `#111827` |
| Panel yüzeyi | Koyu mavi-gri | `#192333` |
| Ana metin | Soluk beyaz | `#E6EAF2` |
| İkincil metin | Gri-mavi | `#9AA8BD` |
| Kozmik vurgu | Elektrik moru | `#9B7BFF` |
| İlahi etki | Soluk altın | `#E5C77A` |
| Yaşam ve büyüme | Turkuaz | `#5ED6C1` |
| Tehlike ve çöküş | Kızıl | `#F06A78` |

Renkler başlangıç değerleridir. Oyun içi testlerle kontrast ve okunabilirlik kontrol edilecektir.

Renkler tek başına anlam taşımamalıdır. Örneğin tehlike, kırmızı rengin yanında şekil, ikon veya metinle de ifade edilmelidir.

Parlak renkler geniş alanları doldurmak için değil, önemli noktaları vurgulamak için kullanılacaktır.

## 3. Şekil dili

### 3.1. Geometrik yaklaşım

Temel şekiller:

- Daireler: evrenler, ruh havuzları ve döngüler.
- Halkalar: tanrısal etki, koruma ve güç alanları.
- Üçgenler: güç, tehlike ve yönelim.
- Altıgenler: istikrar, düzen ve medeniyet.
- Kırık geometriler: kaos, çöküş ve bozulma.
- İnce çizgiler: evrenler arası bağlantılar ve etki akışları.
- Nokta kümeleri: yıldızlar, ruhlar ve küçük kozmik varlıklar.

Şekiller mümkün olduğunca küçük boyutlarda da tanınabilir olmalıdır.

### 3.2. Çizgi kuralları

- Varsayılan çizgiler ince ve temiz olacaktır.
- İkonlarda gereksiz iç içe geçmiş çizgiler kullanılmayacaktır.
- Küçük ikonlarda aşırı ayrıntıdan kaçınılacaktır.
- Dolgu, kontur ve ışık efektleri birbirinden ayrı tutulacaktır.
- Süslemeler temel sembolün okunabilirliğini bozmamalıdır.

## 4. Evren türlerinin görsel kimliği

İlk sürümde altı evren türü bulunacaktır.

### 4.1. Yaşam evreni

- Organik, akışkan ve dallanan şekiller.
- Turkuaz ve yumuşak yeşil vurgular.
- Büyüme, üretim ve biyolojik çeşitlilik çağrışımları.

### 4.2. Kaos evreni

- Asimetrik geometriler.
- Kesintili bağlantılar ve düzensiz desenler.
- Mor ve kızıl vurgular.
- Hareketli veya kararsız görünen şekiller.

### 4.3. İnanç evreni

- Konsantrik halkalar.
- Radyal simetri ve mühür benzeri semboller.
- Altın ve mor vurgular.
- Bir merkeze yönelen çizgiler.

### 4.4. Teknoloji evreni

- Izgaralar, düzenli düğümler ve hassas geometriler.
- Soğuk mavi ve turkuaz vurgular.
- Mekanik düzen ve bağlantı hissi.

### 4.5. İlksel evren

- Büyük, basit ve ham geometriler.
- Az sayıda ayrıntı.
- Taş, kül, soluk altın ve koyu tonlar.
- Henüz biçimlenmemiş bir gerçeklik hissi.

### 4.6. Çöken evren

- Kırılmış halkalar.
- Kopan bağlantılar ve eksik geometriler.
- Solmuş renkler, kızıl uyarı vurguları.
- Görsel bütünlüğü kaybolan semboller.

Her evren türünün kendine ait bir temel sembolü, renk vurgusu ve görsel davranışı bulunacaktır.

## 5. Tanrıların tasarımı

Tanrılar ilk sürümde karakter portreleriyle değil, semboller ve görsel davranışlarla temsil edilecektir.

Her tanrı için:

- Özgün bir ana mühür.
- Tanrı kimliğini tanımlayan bir renk.
- Güçlerini temsil eden ikonlar.
- Diplomasi ve ilişki durumunu gösteren işaretler.
- Haritada tanrısal etkinin kapsamını gösteren bir görsel katman.

Tanrı sembolleri aynı temel tasarım sistemini kullanmalı, fakat birbirleriyle karıştırılmayacak kadar farklı olmalıdır.

Rastgele üretilen tanrılar için mevcut geometrik bileşenler birleştirilerek yeni mühürler oluşturulabilir. Bu üretim sistemi tanınabilir ve estetik sonuçlar vermelidir; rastgelelik tek başına kalite garantisi değildir.

## 6. Kozmik harita

Kozmik harita oyunun ana ekranıdır.

Haritada bulunacak bileşenler:

- Evren düğümleri.
- Evrenler arası bağlantılar.
- Keşfedilmemiş evren işaretleri.
- Tanrıların etki alanları.
- Önemli kozmik olayların işaretleri.
- Seçili evrenin vurgusu.
- Yakınlaştırma ve uzaklaştırma.
- Evren türü, tehlike ve tanrısal etkiye göre filtreleme.

### Görsel davranış

- Seçili evren, renk ve dış halka ile vurgulanır.
- **Tam Aidiyet Belirteci (`★`)**: Oyuncunun baskın ilah olduğu ve nüfuzunun `%65` veya üzerine çıktığı evrenlerde, düğümün tepesinde **3 sivri altın/mor İlahi Hâkimiyet Tacı**, etrafında 4 elmas mühürlü çift ahit halkası ve etiketinde `★ [Evren Adı] [%.. · BİZİM]` ibaresi gösterilir.
- Tehlikedeki evrenlerde belirgin uyarı işaretleri gösterilir.
- Keşfedilmemiş evrenler düşük ayrıntılı ve gizemli görünür.
- Bağlantılar, evrenler arasındaki ilişkinin türüne göre farklılaşabilir; açık geçit çizgilerine ve akan enerji kürelerine doğrudan tıklanarak iki evren arasındaki inanç/ruh/öz akışı incelenebilir.
- Animasyonlar yavaş, ölçülü ve dikkat dağıtmayacak biçimde kullanılmalıdır. **Oyun duraklatıldığında (`SimSpeed::Paused`) tüm harita, geçit ve efekt animasyonları olduğu yerde tamamen donar.**

### 6.1. Etkileşimli animasyonlar ve görsel geri bildirim

Oyuncunun harita ve arayüzle kurduğu her etkileşim, Bölüm 3.1'deki geometrik şekil diline sadık ölçülü animasyonlarla desteklenir:

- **Gerçek Duraklatma (`anim_time`)**: Harita dönüşleri, yörünge halkaları, geçitler üzerindeki akan enerji küreleri, müdahale dalgaları ve yükselen metin bildirimleri gerçek zaman yerine simülasyon hızına bağlı `anim_time` saatini kullanır. Oyun `Duraklat` durumundayken hiçbir animasyon hareket etmez; böylece oyuncu duraklatılmış evreni ve geçitleri rahatça inceleyebilir.
- **Tıklanabilir Evrenler Arası Geçitler ve Akan Küreler**: Evrenleri birbirine bağlayan çizgilerin ve üzerlerinde akan enerji kürelerinin üzerine gelindiğinde (`hover`) geçit hattı altın/turkuaz renkte parlar (`⇄ Geçit #A ↔ #B`). Çizgiye tıklandığında sağ denetçide bu animasyonun neyi temsil ettiği (açık/mühürlü geçit durumu, `%50` üstü baskın nüfuzdan kaynaklanan otomatik inanç yayılımı yönü ve hızı) ayrıntılı olarak açıklanır.
- **Derinlik ve yön bulma cilası**: Sabit konumlu, hafif parıldayan yıldız katmanları arka planı derinleştirir. Keşfedilmiş düğümlerde düşük yoğunluklu tür/hâkimiyet aurası ve uygarlık yörünge izleri kullanılır; seçili düğümde pusula işaretli çift halka görünür. Açık geçitlerde parçacık dizileri akış yönünü belirginleştirir. Efektler harita bilgisini örtmemeli, keşfedilmemiş evrenlerin saklı verilerini göstermemelidir.
- **Nebula arka plan katmanı**: `assets/art/backgrounds/cosmic_nebula.png`, tema rengiyle boyanmış siyah bir zemin yerine, temaya ait `ClearColor`'ı alttan gösteren şeffaf sis bulutları kullanır. Renkli bulutlar düşük kontrastlı ve haritanın dışında da yumuşakça sönümlü olmalıdır.
- **Tam Aidiyet (`★`) Tacı ve Çift Ahit Halkası**: Nüfuzun `%65+` olduğu evrenlerde düğümün üstünde altın/mor taç ve 4 ana yönde elmas mühür taşları yer alır.
- **Yumuşak Kamera Süzülmesi ve Yakınlaştırma (`Lerp`)**: Evren seçildiğinde, odaklandığında (`F`) veya sıfırlandığında (`R`/`Home`) kamera sert sıçramalar yerine üstel sönümleme ile hedefe süzülür.
- **Üzerine Gelme (Hover) Nişangâhı ve Ağ Vurgusu**: Fare imleci bir evren düğümünün üzerine geldiğinde dönen kadim nişangâh halkası (`#5ED6C1` / `#E5C77A`) belirir, etiket yumuşakça büyür ve o evrene bağlı açık geçitlerdeki enerji akışı hızlanır.
- **İlahi Müdahale Geometrik Dalgaları (`MapVfxKind`)**:
  - *Kehanet & İnanç (`ProphecyRings`)*: Konsantrik altın ve mor halkalar ile 8 radyal ışın yayılımı.
  - *Ruh Hasadı & Öz Çekimi (`SoulHarvestSwirl`)*: Merkeze doğru sarmal çizerek toplanan ruh/öz nokta kümeleri.
  - *Mucize, Yaşam & Onarım (`HexagonOrder`)*: Ters yönlerde dönerek genişleyen çift altıgen düzen dalgası.
  - *İlahi Darbe, İsyan & Mühür (`DivineStrike`)*: Kızıl ve elektrik moru üçgen şok dalgası.
  - *Geçit ve Keşif Akışı (`GatewayBeam`)*: Bağlantı hattı boyunca hedefe ilerleyen elmas mühür ve ışık darbesi.
- **Yükselen Kozmik Bildirimler (`FloatingMapToast`)**: Müdahale edilen evrenin üzerinde sonucu özetleyen (`+Kehanet`, `+Ruh Hasadı`, `★ Görev Tamamlandı`) ve yukarı süzülerek sönümlenen metin etiketleri.
- **Arayüz Geçişleri**: Butonların normal, üzerine gelme (`hover`), basılı (`pressed`) ve aktif sekme/filtre durumları arasında yumuşak renk geçişleri (`lerp_color`) ile üst kaynak çubuğu ve görev kutusunda değişim parlaması.
- **Dörtlü Harita Görünüm Katmanı (`M` / `◎ Katman`) ve Başkent Mühür Tacı**: Harita üzerinde `1. Standart Kozmos`, `2. İlahi Hâkimiyet & Bloklar`, `3. Ekonomi & Ruh Havuzu` ve `4. İstikrar & Tehlike Isısı` katmanları arasında geçiş yapılabilir; her aktif ilahın ana kale evreni üzerinde kendine özgü geometrik Başkent Mührü çizilir.
- **Prosedürel Kozmik Ses Sentezleyicisi (`V` / `♪ Ses`)**: Harici ses dosyası gerektirmeyen bellek içi çok-harmonikli sentezleyici (`CosmicAudioPlugin`), her ilahi müdahale, geçit akışı, rastgele kozmik olay ve zafer için atmosferik ses geri bildirimi üretir.

Harita mümkün olduğunca Bevy'nin çizim araçları, sprite'lar ve shader'larıyla oluşturulacaktır. Her evren için ayrı bir arka plan illüstrasyonu zorunlu değildir.

## 7. Kullanıcı arayüzü

Arayüz bilgi yoğunluğunu korurken sade ve görsel açıdan okunabilir kalmalıdır:
- **Kozmik Ana Menü (`MainMenuOverlay` - `Esc` / `☰ Ana Menü`)**:
  - Oyuncunun kendi **İlah Adını** klavyeden doğrudan yazabildiği veya `🎲 Rastgele Kozmik İlah Adı Seç` ile belirleyebildiği giriş alanı,
  - Yukarıdan aşağıya sıralı ve üzerine gelindiğinde yumuşakça genişleyip altın/mor çerçeveyle parlayan (`AnimatedButtonSpec`) 3 ana menü butonu:
    1. **`✦ YENİ OYUN BAŞLAT`**
    2. **`⚙ AYARLAR`** (Oyun modu, yüksek kontrastlı tehlike sembolleri ve arka plan simülasyon optimizasyonu ayarları)
    3. **`✖ ÇIKIŞ`**
  - Ayrıca aktif oyuna dönmek için **`▶ Mevcut Kozmosa Dön / Devam Et (Esc)`** butonu.
- **Üst Bölüm (Görsel Kaynak Kartları ve İlah Kimliği)**: Oyuncunun seçtiği `👑 [İlah Adı] · Tur/Hız`, `◆ Ruh` (`#9B7BFF`), `👑 Nüfuz` (`#E5C77A`), `✦ Kozmik Öz` (`#5ED6C1`), `★ Aidiyet & Ritüel` bilgileri ve `☰ Ana Menü` erişimi renk kodlu bağımsız kartlarda gösterilir.
- **Sol Panel (60 Evrenlik Sayfalı Havuz ve Aidiyet Filtreleri)**:
  - Sahiplik ve durum filtreleri (`Tümü`, `★ Tam`, `◆ Bizim`, `⚔ Rakip`, `▲ Tehlike`, `? Keşif`),
  - Döngüsel `Kategori ↻` ve `Sırala ↻` butonları,
  - Sayfa başına 8 evren yuvası ve `< Önceki Sayfa` / `Sonraki Sayfa >` kontrolleriyle 60 evrenin tamamına hızlı erişim.
- **Sağ Panel (5 Sekmeli Denetçi, Hâkimiyet Yetkisi ve Gelişmiş Tanrı Paneli)**:
  - Tek satırda 5 sekme (`Evren`, `Güçler`, `Tanrılar`, `Zafer & Oyun`, `Eğitici (F1)`),
  - **Evren Hâkimiyet Yetkisi Ayrımı (`Sızma & Mücadele` vs. `Hâkimiyet Emirleri`)**: Oyuncunun baskın ilah olmadığı evrenlerde egemenlik gerektiren emirler (`Ruh Hasadı`, `Mucize`, `Yaşam Tohumla`, `Gerçekliği Onar`, `Öz Çıkar`, `Keşfet`, `Geçit Kapat`, `Evrene Odaklan`) görsel olarak kilitlenir (`COL_BTN_LOCKED`) ve simülasyon çekirdeği tarafından engellenir; oyuncu önce `Kehanet Yay`, `Rakip Vur`, `İsyan Çıkar` veya `Geçit Aç` ile evrende baskın ilah olmalıdır,
  - **Gelişmiş Tanrılar Sekmesi**: `#1`..`#8` doğrudan rakip tanrı seçimi, `🔍 Rakibin Evrenini Bul` kamera odaklama, `🎁 Ruh Armağanı`, `🤝 Ateşkes Paktı`, `👑 Haraç İste`, `⚔ Kutsal Rekabet`, `⚡ Ritüeli Sabote Et` ve `★ Zayıflamış Tanrıyı Mühürle` etkileşim menüsü,
  - Küçültülebilir (`Daralt/Aç`) ve tek tıkla görev uygulayan (`▶ Görevi Şimdi Uygula`) etkileşimli eğitici kutusu,
  - Seçili evrenin İstikrar/Tehlike/Aidiyet oranlarını ve 3 Zafer Yolunun tamamlanma yüzdelerini gösteren **3 renkli görsel ilerleme çubuğu (`InspectorBarFill`)**.
- **Merkezî Zafer / Yenilgi Son-Oyun Penceresi (`EndgameModalOverlay`)**: Senaryo tamamlandığında istatistikleri özetleyen ve tek tıkla **`Sonsuz Havuz Modunda Devam Et (Sandbox)`** seçeneği sunan kozmik sonuç penceresi.

### Ana ekran

- Üst bölüm: ruhlar, nüfuz, kozmik öz ve zaman kontrolü.
- Orta bölüm: kozmik harita.
- Yan panel: seçili evrenin veya tanrının ayrıntıları.
- Alt veya yan günlük: önemli olaylar ve bildirimler.

### Arayüz ilkeleri

- Okunabilir tipografi.
- Tutarlı boşluklar ve hizalama.
- Gereksiz çerçeve ve süslemelerden kaçınma.
- Önemli kararları belirginleştirme.
- Uzun açıklamaları gerektiğinde ikincil panellerde gösterme.
- Renk körlüğünü ve farklı ekran boyutlarını dikkate alma.

Arayüz elemanlarının çoğu SVG dosyaları yerine Bevy UI ile oluşturulacaktır. SVG özellikle ikonlar, semboller, logo ve bağımsız illüstrasyonlar için kullanılacaktır.

## 8. Çizim aracı ve SVG üretim iş akışı

### 8.1. Birincil araç

Inkscape kullanılacaktır.

Inkscape; özgür, vektör tabanlı ve SVG merkezli bir çizim aracıdır. Yapay zekâ tarafından üretilen SVG dosyalarını açmak, düzeltmek, yeniden renklendirmek ve dışa aktarmak için kullanılacaktır.

Küçük raster dokular veya boyamalı illüstrasyonlar gerektiğinde Krita isteğe bağlı yardımcı araç olabilir. İlk sürümde zorunlu değildir.

### 8.2. Yapay zekâ ile çizim

Yapay zekâdan SVG biçiminde ikon, mühür, logo ve basit illüstrasyon üretmesi istenebilir.

Üretim süreci:

1. Tasarım rehberi ve ilgili varlık türü belirlenir.
2. Yapay zekâya renk, şekil, boyut ve stil kuralları verilir.
3. SVG dosyası oluşturulur.
4. Dosya Inkscape'te açılıp görsel olarak incelenir.
5. Gereksiz öğeler, aşırı ayrıntılar ve uyumsuz renkler düzeltilir.
6. SVG'nin güvenli ve geçerli olduğu kontrol edilir.
7. Oyun içinde farklı ölçeklerde test edilir.
8. Lisans ve kaynak bilgileri kaydedilir.
9. Varlık sürüm kontrolüne eklenir.

Yapay zekâ çıktıları otomatik olarak onaylanmış sayılmaz. Her varlık okunabilirlik, stil uyumu, teknik geçerlilik ve lisans politikası açısından kontrol edilmelidir.

### 8.3. SVG teknik kuralları

- SVG, XML tabanlı bir vektör formatı olarak tutulacaktır.
- Dosyalarda gereksiz metadata ve kullanılmayan nesneler temizlenecektir.
- Dışarıdan yüklenen SVG'ler güvenilir kabul edilmeyecektir.
- Harici dosyalara, uzak kaynaklara veya gömülü betiklere gereksiz bağımlılık kurulmayacaktır.
- SVG'ler oyun içinde kullanılmadan önce doğrulanacaktır.
- Bevy'nin SVG desteği ve hedeflenen render iş akışı doğrulanacak; gerekirse SVG, PNG veya başka bir uygun biçime dönüştürülecektir.
- Oyun içinde doğrudan SVG kullanımı mümkün olsa bile her durumda bunun en iyi performans seçeneği olduğu varsayılmayacaktır.

## 9. Varlık adlandırma ve klasörleme

Dosya adları küçük harf, rakam ve alt çizgi kullanmalıdır.

Örnekler:

- `universe_life.svg`
- `universe_chaos.svg`
- `deity_seal_001.svg`
- `power_soul_drain.svg`
- `event_universe_collapse.svg`
- `ui_resource_souls.svg`

Klasör yapısı:

    assets/
      art/
        universes/
        deities/
        powers/
        events/
        ui/
        logos/
      fonts/
      textures/
      shaders/
      audio/
      content/

Her varlığın amacı ve kullanım yeri açık olmalıdır.

## 10. Görsel kalite kontrolü

Bir görsel kabul edilmeden önce şu koşulları sağlamalıdır:

- Tasarım rehberine uymalıdır.
- Küçük boyutta da anlaşılabilmelidir.
- Koyu arka planda yeterli kontrasta sahip olmalıdır.
- Benzer ikonlarla karıştırılmamalıdır.
- Gereksiz karmaşa içermemelidir.
- Dosya formatı teknik olarak geçerli olmalıdır.
- Oyun içinde doğru biçimde görüntülenmelidir.
- Kaynağı ve lisansı kayıt altına alınmalıdır.

Bir varlık bu koşulları karşılamıyorsa yeniden düzenlenir veya reddedilir.

## 11. Lisans politikası

### 11.1. Kod

Projenin kodu GNU GPL-3.0-or-later lisansıyla dağıtılacaktır.

- Lisans metni depo kökünde bulunacaktır.
- Üçüncü taraf Rust bağımlılıklarının lisansları kontrol edilecektir.
- Yeni bağımlılıklar eklenmeden önce lisans uyumluluğu değerlendirilecektir.
- Dağıtım sırasında gerekli telif, lisans ve bildirimler korunacaktır.

### 11.2. Sanat varlıkları

Kod lisansı, sanat varlıklarının lisansını kendiliğinden belirlemez.

Projeye özel çizimlerin lisansı ayrıca belirlenecektir. Üçüncü taraf görseller yalnızca kullanım koşulları doğrulandıktan sonra eklenecektir.

Her dış varlık için şu bilgiler saklanacaktır:

- Dosya adı.
- Kaynak bağlantısı.
- Hak sahibi veya sanatçı.
- Lisans adı ve sürümü.
- İndirme veya edinme tarihi.
- Yapılan değişiklikler.
- Gerekli atıf metni.

Lisansı belirsiz veya kullanım hakkı doğrulanamayan varlıklar dağıtılabilir sürüme alınmayacaktır.

## 12. Tasarım kapsamı ve öncelikler

İlk oynanabilir demo için öncelik sırası:

1. Kozmik haritanın temel görünümü.
2. Evren türlerini ayırt eden semboller.
3. Tanrı mühürleri.
4. Kaynak ve zaman arayüzü.
5. Evren ayrıntı paneli.
6. Olay günlüğü ve uyarılar.
7. Tanrısal güç ikonları.
8. Daha ayrıntılı görsel efektler ve illüstrasyonlar.

Görsel üretimi simülasyon geliştirmesini geciktirmemelidir. Önce anlaşılır bir görsel prototip oluşturulacak, ardından sistemler geliştikçe görsel kalite artırılacaktır.

## 13. Temel tasarım kuralı

Eternal Dominion'ın görsel kimliği rastgele görsel biriktirerek oluşturulmayacaktır.

Her yeni çizim, ikon, efekt ve arayüz bileşeni bu belgedeki stil kurallarıyla tutarlı olmalıdır. Tasarım rehberi değiştirilecekse değişiklik bilinçli yapılmalı ve belge güncellenmelidir.

**Hedef:** Özgün, tutarlı, modüler, erişilebilir ve yapay zekâ destekli üretime uygun bir kozmik strateji görsel sistemi.

## 14. Simülasyon mimarisi ve güncelleme sözleşmesi

- `dominion-sim`, Bevy'den bağımsız kalır; görsel katman simülasyon kurallarını sahiplenmez.
- Sabit adımlı tur sırası korunur: evren/uygarlık evrimi → ekonomi → kozmik olaylar → rakip kararları → diplomasi → zafer/yenilgi değerlendirmesi.
- Tur sonu diplomasi ve zafer/yenilgi kuralları `crates/dominion-sim/src/simulation/endgame.rs` içindedir. Pakt ve mühür haraç gelirinin hesabı `simulation/economy.rs` içindedir; kalan simülasyon fazları ana `simulation.rs` dosyasındadır.
- Uzak, gözlenmeyen evrenlerin seyrek güncellenmesi aynı seed için deterministiktir; ancak atlanan adımlar, rastgele olaylar ve sınırlandırılmış/doğrusal olmayan güncellemeler nedeniyle optimizasyon kapalı çalışmayla birebir eşdeğer olduğu garanti edilmez.
- Bu sorumluluklar ayrıştırılırken güncelleme sırası, kayıt uyumluluğu ve aynı seed ile yeniden oynatma davranışı test edilmelidir.
