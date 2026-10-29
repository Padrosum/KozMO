use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuideChapter {
    pub index: usize,
    pub title_tr: &'static str,
    pub subtitle_tr: &'static str,
    pub body_tr: &'static str,
    pub pro_tip_tr: &'static str,
}

pub fn built_in_guide_chapters() -> Vec<GuideChapter> {
    vec![
        GuideChapter {
            index: 0,
            title_tr: "1. KozMO Evren Yapısı ve Hâkimiyet Kuralları",
            subtitle_tr: "60 Evren Havuzu · Aidiyet Kademeleri · Söz Sahibi Olma",
            body_tr: "\
KozMO'da fiziksel bir bedenin yoktur; 60 bağımsız evrenden oluşan yaşayan bir çoklu evren havuzunu yöneten yükselen bir ilahsın.\n\n\
AİDİYET VE YETKİ KADEMELERİ:\n\
• ○ NÖTR / ⚔ RAKİP EVREN (Söz Sahibi Değilsin): Bu evrenlerde iç yönetim emirleri KİLİTLİDİR. Yalnızca Sızma & Mücadele emirlerini (Kehanet Yay, Rakip Vur, İsyan, Geçit Aç) kullanarak nüfuzunu artırabilirsin.\n\
• ◆ BASKIN İLAH (%35+ ve En Yüksek Nüfuz): Evrenin söz sahibi olursun! Tüm Hâkimiyet ve Evren Etkileşim emirleri açılır ve her tur düzenli gelir alırsın.\n\
• ★ TAM AİDİYET (%65+ Nüfuz ve Baskın): Evrenin üzerinde haritada Altın Taç ve Çift Mühür Halkası belirir. Tüm kaynak getirisi +%30 artar ve zafer hedeflerine sayılır.\n\
• ◎ İLAHİ ODAK ([F] Tuşu): Seçili baskın evreni odak yörüngesine alarak gelirini +%25 artırır ve her tur istikrarını korursun.",
            pro_tip_tr: "WİKİ İPUCU: Herhangi bir butonun üzerinde fareyle 2 saniye beklersen o butonun ne işe yaradığını anlatan detaylı bilgi kutusu açılır!",
        },
        GuideChapter {
            index: 1,
            title_tr: "2. Üç Kutsal Kaynak ve Kozmik Ekonomi",
            subtitle_tr: "◆ Ruh · ♛ Nüfuz · ✦ Kozmik Öz · Rezerv ve Bakım Dengesi",
            body_tr: "\
KozMO ekonomisi gerçekçi üretim, rezerv tükenmesi ve imparatorluk bakımına dayanır:\n\n\
• ◆ RUH (Mor): Ölümlülerin doğal ölümlerinden, savaşlardan ve tapınak ibadetlerinden doğar. Her evrenin bir 'Ruh Havuzu' vardır; baskın olduğun evrenlerden her tur pasif akar veya 'Ruh Hasadı' ile çekilir.\n\
• ♛ NÜFUZ (Altın): İnançlı uygarlıklardan, ★ Tam Aidiyet evrenlerinden ve açık geçit ticaretinden kazanılır. Kehanet yaymak, geçit açmak ve diplomasi yürütmek için harcanır.\n\
• ✦ KOZMİK ÖZ (Turkuaz): Evrenlerin gerçeklik dokusundan ve anomalilerden damıtılan nadir cevherdir. Her 'Kozmik Öz Çıkar' emri evrenin Öz Zenginliğini biraz düşürür; 'Gerçekliği Onar' veya 'Dokuyu Dönüştür' ile yenilenir.\n\
• BAKIM VE KAYIPLAR: Çok fazla evrene yayıldığında veya istikrarı %35'in altına düşen isyankâr evrenlerde tur başı Ruh/Nüfuz aşınması yaşanır.",
            pro_tip_tr: "WİKİ İPUCU: 'Zafer' sekmesindeki Kaynak Defteri'nden hangi kaynaktan ne kadar kazandığını ve hangi sebeple kayıp yaşadığını anlık takip edebilirsin.",
        },
        GuideChapter {
            index: 2,
            title_tr: "3. Evren Emirleri: Sızma, Hâkimiyet ve Dönüşüm",
            subtitle_tr: "Sızma Hamleleri · İç Yönetim · Doğrudan Evren Etkileşimi",
            body_tr: "\
Sağ paneldeki 'Evren' sekmesinde emirler iki yetki grubuna ayrılır:\n\n\
⚔ SIZMA & MÜCADELE (Her Keşfedilen Evrende Açık):\n\
• Kehanet Yay (20R 25N): Evrende +18 Nüfuz ve uygarlıklarda inanç coşkusu sağlar.\n\
• Rakip Vur (75R 35N 10Ö): Rakip ilahın nüfuzunu -22 kırar ve ilişkiyi düşürür.\n\
• İsyan (30R 40N): Rakip evrendeki uygarlıkları ayaklandırıp üretimi felç eder.\n\
• Geçit Aç (40R 20N 10Ö): Bağlı gizemli evreni keşfeder ve inanç akışını başlatır.\n\n\
♛ HÂKİMİYET & EVREN ETKİLEŞİM EMİRLERİ (Yalnızca Söz Sahibi Evrenlerde):\n\
• ☀ Uygarlığı Aydınlat (35R 20N): [BİLİMSEL KALKAN] ve [İSYAN] engellerini kaldırır, +60M nüfus ve +22 inanç verir.\n\
• ☸ Dokuyu Dönüştür (55R 25N 20Ö): Evrenin türünü dönüştürür, Ruh Çarpanı ve Öz Zenginliğini kalıcı olarak +0.22x artırır.\n\
• ⚡ Kozmik Rezonans (30N 10Ö): Ortam enerjisinden anında +75..105 Ruh üretir ve açık geçitlerden komşu evrenlere nüfuz yayar.\n\
• Ruh Hasadı, Mucize Bahşet, Kozmik Öz Çıkar, Yaşam Tohumla, Gerçekliği Onar.",
            pro_tip_tr: "WİKİ İPUCU: Kaos veya Çöken bir evreni ele geçirdikten sonra '☸ Dokuyu Dönüştür' emriyle onu bereketli bir Yaşam veya İnanç evrenine çevirebilirsin!",
        },
        GuideChapter {
            index: 3,
            title_tr: "4. Beş Tanrısal Güç Ağacı ve 15 Gücün Etkileri",
            subtitle_tr: "Anında Kozmik Patlama · Kalıcı Pasif Bonuslar · Aktif Çarpanlar",
            body_tr: "\
'Güçler' sekmesindeki 5 ağaçta toplam 15 ilahi güç bulunur. Bir gücü açtığın anda hem ANINDA KOZMİK PATLAMA gerçekleşir hem de kalıcı pasif etki başlar:\n\n\
• YARATIM AĞACI: K1 Yaratılış Kıvılcımı (anında +45M nüfus, hızlı büyüme), K2 Gerçeklik Çapası (anında +16 istikrar, her tur pasif onarım ve -%35 bakım), K3 Kozmik Ocak (anında +40 Öz, şekillendirilmiş evrenlerden her tur +4R +3N +2Ö).\n\
• İNANÇ VE HÂKİMİYET: K1 Kehanet Sesi (anında +14 nüfuz yayılımı, her tur pasif nüfuz), K2 Altın Mucize (tüm isyan ve kalkanları temizler), K3 Mutlak Dogma (rakipleri -12 kırar, ★ Tam Aidiyet gelirini +%45 artırır).\n\
• RUH HASADI VE ÖZ: K1 Ruh Sifonu (anında +90..300 Ruh, pasif ruh +%35), K2 Öz Yoğunlaştırıcı (+35 Öz ve +0.20x zenginlik), K3 Hasatçı Üstünlüğü (+180 Ruh, 2x ölümlü ruhu).\n\
• KOZMİK GEÇİTLER: K1 Astral Görüş (anında 4 yeni evren keşfeder, 3 turda bir otomatik keşif), K2 Geçit Ustalığı (açık geçitlerden her tur ticaret geliri), K3 Boyutsal Koruma (rakip sızıntısına %65 kalkan).\n\
• TANRILAR SAVAŞI: K1 İlahi Darbe, K2 Rakip Sabotajı (rakip ritüelini -%15 kırar ve kaynak çalar), K3 Ebedî Mühür (her tur rakiplerden haraç toplar, 2 evrenli rakipleri mühürleme imkânı verir).",
            pro_tip_tr: "WİKİ İPUCU: Erken oyunda 'Ruh Sifonu' ve 'Astral Görüş' açmak hem ekonomini hem de harita görüşünü hızla büyütür.",
        },
        GuideChapter {
            index: 4,
            title_tr: "5. Altı Evren Türü ve Ölümlü Uygarlık Evrimi",
            subtitle_tr: "Yaşam · Kaos · İnanç · Teknoloji · İlksel · Çöküş · Kalkanlar",
            body_tr: "\
Haritadaki 60 evren 6 farklı kozmik kategoride evrimleşir:\n\n\
• YAŞAM EVRENİ (Turkuaz Halka): Yüksek nüfus ve istikrarlı Ruh/Nüfuz üretimi sağlar.\n\
• KAOS EVRENİ (Kızıl/Mor Üçgen): Savaşlar nedeniyle ruh havuzu hızlı dolar ve Öz zengindir; ancak istikrarı hızla düşer.\n\
• İNANÇ EVRENİ (Altın Mühür): Kehanet ve mucizelere en yüksek tepkiyi verir; tapınak gelirinin kalbidir.\n\
• TEKNOLOJİ EVRENİ (Altıgen): Bilimsel ilerlemesi yüksektir. Teknoloji 55'i aşıp İnanç 35'in altına inerse uygarlıklar [BİLİMSEL KALKAN] kurarak pasif inanç gelirini %60 keser!\n\
• İLKSEL EVREN (Altın Elmas): Henüz şekillenmemiştir; 'Yaşam Tohumla' ve 'Dokuyu Dönüştür' için en uygun evrendir.\n\
• ÇÖKEN EVREN (Beşgen/Üçgen): İstikrarı 0'a inerse tamamen yok olur! 'Gerçekliği Onar' ile kurtarılmalıdır.\n\n\
5 UYGARLIK DOKTRİNİ VE RAKİP KÜLTLER:\n\
• Her uygarlık 5 doktrinden birine sahiptir: Bilge (+İstikrar), Tapınakçı (+%25 İbadet Ruhu/Nüfuz), Kuşkucu (+%30 Teknoloji/Kalkan), Hanedan (+%25 Savaş Ruhu) veya Mistik (Kaos direnci).\n\
• Aynı evrende iki ilah %25+ nüfuza ulaşırsa uygarlıklar rakip mezheplere bölünür ve mezhep çatışmaları (+18 Ruh Havuzu, +6%/-5% Nüfuz) başlar!",
            pro_tip_tr: "WİKİ İPUCU: Bir uygarlık [BİLİMSEL KALKAN] açtığında veya [İSYAN] çıkardığında '☀ Uygarlığı Aydınlat' emriyle tek tıkla sadakate döndürebilirsin.",
        },
        GuideChapter {
            index: 5,
            title_tr: "6. Rastgele Kozmik Olaylar, Geçitler ve İkilemler",
            subtitle_tr: "2-6 Turda Bir Rastgele Olaylar · ✦ OLAY İşaretçisi · [E] Tuşu",
            body_tr: "\
KozMO'da evrenler durağan değildir; 2 ila 6 tur arasında rastgele değişen sürelerde 8 farklı kozmik olay ve etkileşimli ikilem meydana gelir:\n\n\
• ✦ ALTIN ÇAĞ PEYGAMBERİ: Uygarlıkta karizmatik bir peygamber doğar; inanç ve nüfuz patlaması yaşanır (İkilem doğurur).\n\
• ⇄ EVRENLER ARASI BÜYÜK GÖÇ: Açık geçitlerden komşu evrenlere milyonlarca nüfus ve ruh göç eder.\n\
• ▲ TEKİLLİK DENEYİ: İleri teknoloji uygarlığı boyutsal deney yapar; kutsayabilir veya özünü sömürebilirsin.\n\
• ◆ ASTRAL RUH FIRTINASI: Evrenin ruh havuzu taşar; hasat veya istikrar arasında seçim yaparsın.\n\
• ★ KADİM İLAHİ KALINTI: Baskın ilaha doğrudan Ruh, Nüfuz ve Kozmik Öz hazinesi kazandırır.\n\
• Boyutsal Anomali, Uygarlık Savaşı ve Mezhep Bölünmesi.\n\n\
HARİTADA TAKİP: Olayın çıktığı evrende '✦ OLAY' veya '★ İKİLEM' elması parlar. '◎ Son Olay (E)' butonuyla kamerayı anında oraya götürebilirsin!",
            pro_tip_tr: "WİKİ İPUCU: Haritada evrenler arasındaki çizgilere ve akan kürelere doğrudan tıklayarak iki evren arasındaki inanç/ruh akışını inceleyebilirsin.",
        },
        GuideChapter {
            index: 6,
            title_tr: "7. Sekiz Rakip İlah, Arketipler ve Diplomasi",
            subtitle_tr: "Tanrılar Meclisi · Tutum Kademeleri · Pakt, Haraç ve Mühürleme",
            body_tr: "\
Kozmosta seninle yarışan 8 bağımsız rakip ilah vardır (#1 Vaelkor, #2 Morgeth, #3 Sylvara, #4 Aurelion, #5 Solinvictus, #6 Nyxarion, #7 Mechanon, #8 Vespera).\n\n\
DİPLOMASİ VE SAVAŞ EMİRLERİ ('Tanrılar' Sekmesi):\n\
• ✦ Ruh Armağanı (70R 5Ö): Rakiple ilişkiyi +28 iyileştirir.\n\
• ⚖ Ateşkes Paktı (35R 45N): Karşılıklı saldırıları durdurur ve her tur diplomatik ticaret geliri (+Ruh/+Nüfuz) sağlar.\n\
• ♛ Haraç İste (20R 35N): Güçlüysen rakipten anında Ruh ve Öz koparır.\n\
• ⚔ Kutsal Rekabet (60R 40N 10Ö): Ortak evrenlerdeki rakip nüfuzunu kırar ve +45 Nüfuz kazandırır.\n\
• ⚡ Ritüeli Sabote Et (220R 110N 45Ö): Aşkınlık Zaferine yaklaşan rakibin ritüelini -%20 geri düşürür!\n\
• ★ Zayıflamış Tanrıyı Mühürle (320R 150N 80Ö): Evren sayısı 1'e (Ebedî Mühür gücüyle 2'ye) düşen rakibi ebedî boşluğa hapseder.",
            pro_tip_tr: "WİKİ İPUCU: '◎ Rakibin Evrenini Bul' butonuna tıklayarak seçili rakip ilahın ana evrenine haritada anında odaklanabilirsin.",
        },
        GuideChapter {
            index: 7,
            title_tr: "8. Üç Zafer Yolu, Kısayollar ve Arayüz Rehberi",
            subtitle_tr: "Hâkimiyet · Aşkınlık · Yaratım · Klavye ve Fare Kısayolları",
            body_tr: "\
Senaryolu oyunda KozMO'yu üç büyük zafer yolundan biriyle kazanabilirsin:\n\n\
1) HÂKİMİYET ZAFERİ: En az 35 Baskın Evren + 18 ★ Tam Aidiyet Evreni + 5 Mühürlenmiş Rakip Tanrı.\n\
2) AŞKINLIK ZAFERİ: En az 10 İlahi Güç + 10 ★ Tam Aidiyet Evreni + 10 aşamalı Aşkınlık Ritüelini %100'e ulaştırmak.\n\
3) YARATIM ZAFERİ: Yaratım Ağacını (3/3) tamamlamak + 15 evrende ★ Tam Aidiyet, %75+ İstikrar ve şekillendirilmiş düzen kurmak.\n\n\
KLAVYE VE FARE KISAYOLLARI:\n\
• Harita Gezinme: [W][A][S][D] / Ok Tuşları / Sağ veya Orta Fare Tuşuyla Sürükleme\n\
• Yakınlaştırma: Fare Tekerleği veya [-]/[+], Kamerayı Merkeze Sıfırla: [R] veya [Home]\n\
• Seçili Evrene Odaklan: [F]  |  Son Olay Evrenine Git: [E]  |  Harita Katmanı: [M]  |  Tema: [T]  |  Ses: [V]\n\
• Wiki Sekmesi: [H] veya [F1]  |  Sekme Değiştir: [Tab]  |  Ana Menü: [Esc]\n\
• Zaman: [Space] Duraklat/Devam, [1]/[2]/[3] Hız (1x/2x/5x), [N] +1 Tur  |  Kaydet/Yükle: [F5]/[F9]",
            pro_tip_tr: "WİKİ İPUCU: Üst bardaki '◎ Katman' ([M]) ile 4 harita analiz katmanı, '❖ Tema' ([T]) ile 4 kozmik renk teması ve '♪ Ses' ([V]) ile prosedürel ses sentezleyicisini yönetebilirsin!",
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::built_in_guide_chapters;

    #[test]
    fn test_built_in_guide_chapters_are_complete_and_utf8_turkish() {
        let chapters = built_in_guide_chapters();
        assert_eq!(chapters.len(), 8);
        for (idx, chapter) in chapters.iter().enumerate() {
            assert_eq!(chapter.index, idx);
            assert!(!chapter.title_tr.is_empty());
            assert!(!chapter.subtitle_tr.is_empty());
            assert!(!chapter.body_tr.is_empty());
            assert!(!chapter.pro_tip_tr.is_empty());
        }
        // Verify Turkish diacritics are intact in guide texts
        let combined = chapters
            .iter()
            .map(|c| format!("{} {} {}", c.title_tr, c.body_tr, c.pro_tip_tr))
            .collect::<Vec<_>>()
            .join(" ");
        for ch in ['ğ', 'ş', 'ı', 'İ', 'ö', 'ü', 'ç', 'â', 'î'] {
            assert!(
                combined.contains(ch),
                "Expected Turkish character '{ch}' in built-in guide chapters"
            );
        }
    }
}
