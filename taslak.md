Hocam, bence bu değişiklik oyunun kimliğini daha da netleştiriyor. Avatarı tamamen çıkarıyoruz. Oyuncu hiçbir dünyaya fiziksel olarak inmiyor; her şeyi kozmik ölçekte yöneten, gözlemleyen ve gerektiğinde gerçekliğe müdahale eden bir tanrı.

Böylece kaynaklarımızı iki ayrı oynanış biçimine bölmek yerine, evrenler havuzunu, ruh ekonomisini, uygarlıkları ve tanrılar arası siyaseti gerçekten derinleştirebiliriz.

Bir de MVP konusunda çıtayı yükseltiyoruz: Bu bir teknik prototip veya birkaç düğmeden oluşan arayüz olmayacak. Baştan sona oynanabilen, kaydedilip yüklenebilen, kazanma ve kaybetme koşulları bulunan bağımsız bir demo olacak.
ETERNAL DOMINION
Game Design Document · v0.2 · Avatar'sız kozmik sandbox







canvas4everyone.com


1. Kesinleşen tasarım kararları
Avatar ve fiziksel kontrol yok

Bütün eylemler kozmik harita, tanrısal yetenekler, emirler, diplomasi ve simülasyon üzerinden gerçekleşir.


Geniş ve yaşayan evrenler havuzu

Birbirinden farklı evrenler, bağımsız uygarlıklar, rakip tanrılar, geçitler ve keşfedilebilir kozmik bölgeler.


Gerçek oynanabilir demo

Başlangıç, gelişim, stratejik kararlar, rakipler, rastlantısal olaylar, yenilgi veya zafer ve kayıt sistemi.


Oyuncudan bağımsız simülasyon

Oyuncu müdahale etmese de uygarlıklar gelişir, savaşlar çıkar, dinler yayılır ve tanrılar kendi hedeflerini takip eder.



2. MVP'nin kapsamı: küçük bir oyun değil, küçük ölçekte tam bir oyun
Burada önemli bir ayrım var. Tam sürümde binlerce veya milyonlarca evren hedefleyebiliriz. Demo ise bu iddiayı temsil edecek kadar geniş, ama gerçekten simüle edilebilecek kadar kontrollü olmalı.

MVP için şu başlangıç kapsamını öneriyorum:
İçerik	Oynanabilir demo hedefi
Evrenler havuzu	60 keşfedilebilir evren
Evren türü	6 farklı tip
Uygarlıklar	Başlangıçta 30–50 aktif uygarlık
Rakip tanrılar	8 bağımsız tanrı
Tanrısal güç alanları	5 gelişim ağacı
Temel kaynaklar	Ruh, nüfuz ve kozmik öz
Din ve inanç	İnanç yayılımı, kültler, ayrılıklar ve isyanlar
Kozmik olaylar	Savaşlar, çöküşler, anomaliler, keşifler ve felaketler
Kazanma koşulları	Birden fazla farklı zafer yolu
Yenilgi koşulları	Güç kaybı, mühürlenme ve nihai yenilgi
Kayıt sistemi	Kaydetme, yükleme ve devam etme
Oynanış	Baştan sona tamamlanabilir bir senaryo ve serbest oyun

Bunlar hedeflenen tasarım değerleri; test sonuçlarına göre değişebilir. Altmış evrenin her biri ayrı bir 3D dünyaya ihtiyaç duymayacak. Evrenler veri ve simülasyon üzerinden temsil edilecek, görsel harita ise bunların ilişkilerini gösterecek.
3. Evrenler havuzunu gerçekten geniş yapalım




















12








Altmış evren yalnızca haritada duran altmış nokta olmamalı. Her birinin kendi tarihi, yaşam biçimi, kaynakları ve tanrısal ilişkileri olmalı.
Altı evren kategorisi









1. Yaşam evrenleri

Çok sayıda uygarlık, yüksek ruh potansiyeli ve karmaşık siyasi ilişkiler. İnanç sistemi kurmak ve uzun vadeli kaynak elde etmek için elverişli.











2. Kaos evrenleri

Sık savaşlar, istikrarsız gerçeklik ve öngörülemez olaylar. Büyük fırsatlar, fakat aynı zamanda büyük kayıplar.











3. İnanç evrenleri

Güçlü dinler, kehanetler ve tanrılara yoğun bağlılık. Ruh akışını büyütmek için önemli merkezler.











4. Teknoloji evrenleri

İleri uygarlıklar, bilimsel gelişim ve tanrısal müdahaleye karşı koyabilecek sistemler. Bazı toplumlar doğaüstü olayları araştırıp tanrıları açığa çıkarabilir.











5. İlksel evrenler

Henüz gelişmemiş gerçeklikler. Yaşam yaratmak, fizik yasalarını şekillendirmek veya yeni uygarlıkların temelini atmak mümkün.











6. Çökmekte olan evrenler

Gerçekliğin bozulduğu, ruhların kaybolabildiği ve başka tanrıların terk ettiği bölgeler. Kurtarmak, sömürmek veya tamamen yok etmek arasında seçim yapılabilir.



Evren türü, o evrendeki her şeyi tek başına belirlemeyecek. Örneğin bir teknoloji evreninde çok güçlü dinler, bir inanç evreninde ise tanrılara karşı çıkan gelişmiş uygarlıklar bulunabilir.
Her evrenin kendine ait tarihi olacak
Bir evrenin durumunu yalnızca mevcut nüfus ve kaynaklarla değil, geçmiş olaylarla da takip edeceğiz.
- Hangi tanrılar buraya daha önce müdahale etti?
- Hangi uygarlıklar yükseldi ve çöktü?
- Hangi dinler birbirine karşı savaştı?
- Evren neden kararsız hâle geldi?
- Hangi tanrı burada hâkimiyet kurmaya çalışıyor?
Böylece oyuncu bir evrene ilk kez girdiğinde bile onun geçmişine dair bir şeyler keşfedebilir.
4. Kozmik harita ve oyuncunun kontrolü







patch-notes.com


Ana ekranımız bir aksiyon oyunu ekranı değil; güçlü bir kozmik yönetim arayüzü olacak.

Oyuncu şu işlemleri yapabilecek:
- Havuzdaki evrenleri keşfetmek ve filtrelemek.
- Evrenleri tehlike, ruh potansiyeli, nüfuz ve yaşam yoğunluğuna göre sıralamak.
- Evrenler arası geçitleri açmak ve kapatmak.
- Uygarlıkları, dinleri ve rakip tanrıların faaliyetlerini incelemek.
- Tanrısal emirler vermek ve müdahalelerin sonuçlarını izlemek.
- Bir evrene odaklanıp zamanın daha ayrıntılı akışını takip etmek.
- Zamanı durdurmak, normal hızda ilerletmek veya hızlandırmak.
- Geçmiş olayların günlüğünü incelemek.
Önemli karar: Oyuncu evrenleri keşfettikçe bütün havuz bir anda açılmayacak. Başlangıçta küçük bir bölge görünür olacak; yeni geçitler ve kozmik keşifler daha uzak bölgelere erişim sağlayacak.
5. Tam oynanabilir demo nasıl bitecek?
Bir demoda yalnızca sistemlerin çalışması yetmez. Oyuncunun neden devam ettiğini ve ne zaman başarılı olduğunu bilmesi gerekir.

Ben iki oynanış modu eklerdim.
Kozmik yükseliş — Senaryolu mod
Oyuncu zayıf bir tanrı olarak başlar. Başlangıç bölgesindeki sınırlı nüfuzunu kullanarak ruh toplar, yeni evrenlere erişir ve rakip tanrıların baskısıyla mücadele eder.

Zafer koşulu

Birkaç farklı yoldan biriyle belirli bir kozmik güç eşiğine ulaşmak ve yükselişini tamamlamak.

Yenilgi koşulu

Tanrısal özün tükenmesi ve oyuncunun son dayanaklarını da kaybetmesi.

Sonsuz havuz — Sandbox modu
Oyuncu başlangıç koşullarını seçer, zorluk seviyesini belirler ve kendi hedeflerini oluşturur. İster evrenler yaratır, ister rakiplerini alt eder, ister bütün havuzu yok etmeye çalışır.

Zafer koşulu

İsteğe bağlı kişisel hedefler; oyuncu dilerse hiçbir bitiş koşulu olmadan devam eder.


Senaryolu modda üç ayrı zafer yolu bulunabilir:
- Hâkimiyet: Rakip tanrıların gücünü kırıp havuzun büyük bölümünde üstünlük kurmak.
- Aşkınlık: Yeterli ruh ve kozmik öz toplayarak tanrısal varoluş biçimini aşmak.
- Yaratım: Kendi düzenine bağlı yeni evrenlerden oluşan bir alan kurmak.
Bunların hepsi aynı temel simülasyonu kullanır. Ayrı oyunlar geliştirmemize gerek kalmaz.
6. Teknik olarak oynanabilir olması için zorunlu sistemler
MVP'nin tamamlanmış sayılması için şu özelliklerin hepsi çalışmalı:
Demo kabul listesi
10/10 (Tamamlandı — v0.2 KozMO)



[x] Yeni oyun

Başlangıç seçimi, oyuncu İlah Adı belirleme ve rastgele veya sabit 60 evrenlik havuz


[x] Zaman simülasyonu

Uygarlıklar ve tanrılar bağımsız şekilde ilerler (Duraklat, 1x, 2x, 5x ve +1 Tur)


[x] Tanrısal eylemler

Oyuncu kaynak harcayıp sonuç doğuran sızma ve hâkimiyet emirleri verir


[x] Ruh ekonomisi

Kazanma, kaybetme, harcama, rezervuar yorgunluğu, ★ Tam Aidiyet bonusu ve kaynak defteri


[x] Keşif

Yeni evrenler ve tıklanabilir boyutsal geçitler açılabilir/kapatılabilir


[x] Yapay zekâ

8 bağımsız rakip tanrı kendi arketip hedefleri doğrultusunda hareket eder


[x] Kozmik olaylar

Rastgele olaylar ve etkileşimli kozmik ikilemler gerçek oyun durumunu değiştirir


[x] Zafer ve yenilgi

3 zorlu zafer yolu (Hâkimiyet, Aşkınlık, Yaratım), yenilgi ve Sonsuz Sandbox devam modu


[x] Kayıt sistemi

Atomik kaydetme (F5), yükleme (F9) ve şema göçü (v1 -> v2) ile devam etme


[x] Arayüz ve ayarlar

Oyun hızı, 4 kozmik tema (T), 8 bölümlü Wiki (F1), 2sn buton ipucu kartları ve erişilebilirlik kontrolleri



Bu listeyi bir geliştirme kabul kriteri olarak kullanabiliriz. Bir özellik yalnızca menüde görünüyorsa tamamlanmış sayılmaz; gerçekten oyun durumunu değiştirmeli, kaydedilmeli ve yeniden yüklenince korunmalı.
7. Geliştirme sırası
Önerdiğim geliştirme sırası şu:
1. Simülasyon çekirdeği: Zaman, kaynaklar, uygarlıklar ve tanrıların kararları.
2. Kozmik harita: Evren düğümleri, bağlantılar, keşif ve filtreleme.
3. Eylem sistemi: Tanrısal müdahaleler ve sonuçların hesaplanması.
4. Yapay zekâ: Rakip tanrılar, diplomasi ve kaynak yönetimi.
5. Oynanabilir senaryo: Başlangıç, görevler, yükseliş, zafer ve yenilgi.
6. Kayıt ve yükleme: Tam oyun durumunun korunması.
7. Dengeleme ve test: Kaynak ekonomisi, performans ve oynanışın tamamlanabilirliği.
Teknik açıdan simülasyon çekirdeğini görsel katmandan ayrı tutmak önemli. Böylece 60 evreni yönetmek için 60 ayrı sahne veya ayrı oyun mantığı yazmak yerine aynı sistemin farklı veri kümelerini kullanabiliriz.

Son kararım
Bu sürümde oyunun merkezinde tek bir şey var: Sen bir tanrısın; fakat oyun senin etrafında dönmek zorunda değil.

Evrenler senden bağımsız yaşayacak. Rakip tanrılar kendi planlarını yürütecek. Ruhlar kıt bir güç kaynağı olacak. Sen ise bütün bu sistemi kozmik ölçekte gözlemleyip istediğin yerde müdahale edeceksin.

Bir sonraki somut adım, bu belgeyi uygulamaya dönüştürmek için teknik mimariyi ve ilk oynanabilir sürümün dosya yapısını belirlemek. Ondan sonra simülasyon çekirdeğini kurup 60 evrenli, kayıt sistemi olan ve senaryosu tamamlanabilen gerçek bir demo geliştirebiliriz.
