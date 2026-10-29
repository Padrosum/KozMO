use dominion_sim::UniverseType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UniverseTypeProfile {
    pub kind: UniverseType,
    pub name_tr: &'static str,
    pub primary_color_hex: &'static str,
    pub accent_color_hex: &'static str,
    pub shape_language: &'static str,
    pub lore_summary_tr: &'static str,
    pub gameplay_hint_tr: &'static str,
    pub svg_asset: &'static str,
}

pub fn universe_type_profile(kind: UniverseType) -> UniverseTypeProfile {
    match kind {
        UniverseType::Life => UniverseTypeProfile {
            kind,
            name_tr: "Yaşam Evreni",
            primary_color_hex: "#5ED6C1",
            accent_color_hex: "#E5C77A",
            shape_language: "Organik halkalar ve dallanan yaşam düğümleri",
            lore_summary_tr: "Çok sayıda uygarlık, yüksek ruh potansiyeli ve karmaşık siyasi ilişkiler barındırır.",
            gameplay_hint_tr: "İnanç sistemi kurmak ve uzun vadeli düzenli ruh/nüfuz akışı elde etmek için en elverişli merkezdir.",
            svg_asset: "art/universes/universe_life.svg",
        },
        UniverseType::Chaos => UniverseTypeProfile {
            kind,
            name_tr: "Kaos Evreni",
            primary_color_hex: "#F06A78",
            accent_color_hex: "#9B7BFF",
            shape_language: "Asimetrik kırık üçgenler ve kararsız enerji hatları",
            lore_summary_tr: "Sık savaşlar, istikrarsız gerçeklik ve öngörülemez kozmik anomalilerle çalkalanır.",
            gameplay_hint_tr: "Ölümlü savaşları sayesinde büyük ruh ve öz fırsatları sunar fakat istikrar hızla çökebilir.",
            svg_asset: "art/universes/universe_chaos.svg",
        },
        UniverseType::Faith => UniverseTypeProfile {
            kind,
            name_tr: "İnanç Evreni",
            primary_color_hex: "#E5C77A",
            accent_color_hex: "#9B7BFF",
            shape_language: "Konsantrik mühür halkaları ve merkeze yönelen radyal ışınlar",
            lore_summary_tr: "Güçlü dinler, kehanetler ve ilahlara yoğun bağlılık üzerine kuruludur.",
            gameplay_hint_tr: "Kehanet ve mucizelerle en yüksek ibadet kaynaklı ruh ve nüfuz akışını üretir.",
            svg_asset: "art/universes/universe_faith.svg",
        },
        UniverseType::Tech => UniverseTypeProfile {
            kind,
            name_tr: "Teknoloji Evreni",
            primary_color_hex: "#38BDF8",
            accent_color_hex: "#5ED6C1",
            shape_language: "Altıgen ızgaralar, hassas düğümler ve mekanik simetri",
            lore_summary_tr: "İleri uygarlıklar, bilimsel gelişim ve tanrısal müdahaleye karşı koyabilecek kalkanlar içerir.",
            gameplay_hint_tr: "Yüksek nüfusludur ancak doğaüstü olayları araştırıp ilahi müdahaleye karşı kalkan geliştirebilir.",
            svg_asset: "art/universes/universe_tech.svg",
        },
        UniverseType::Primordial => UniverseTypeProfile {
            kind,
            name_tr: "İlksel Evren",
            primary_color_hex: "#9AA8BD",
            accent_color_hex: "#E5C77A",
            shape_language: "Büyük, ham ve yalın monolitik geometriler",
            lore_summary_tr: "Henüz gelişmemiş genç gerçeklikler; fizik yasaları ve yaşam tohumlanmaya açıktır.",
            gameplay_hint_tr: "Kozmik Öz açısından zengindir; 'Yaşam ve Düzen Tohumla' ile Yaratım Zaferine doğrudan katkı sağlar.",
            svg_asset: "art/universes/universe_primordial.svg",
        },
        UniverseType::Collapsing => UniverseTypeProfile {
            kind,
            name_tr: "Çöken Evren",
            primary_color_hex: "#F06A78",
            accent_color_hex: "#9AA8BD",
            shape_language: "Kırılmış halkalar, kopuk bağlantılar ve kızıl uyarı işaretleri",
            lore_summary_tr: "Gerçekliğin bozulduğu, ruhların boşluğa sızdığı ve eski tanrıların terk ettiği bölgeler.",
            gameplay_hint_tr: "'Gerçekliği Onar' ile kurtarılabilir veya çökmeden önce ruh ve kozmik özü sömürülebilir.",
            svg_asset: "art/universes/universe_collapsing.svg",
        },
    }
}

pub fn all_universe_type_profiles() -> Vec<UniverseTypeProfile> {
    UniverseType::ALL
        .into_iter()
        .map(universe_type_profile)
        .collect()
}
