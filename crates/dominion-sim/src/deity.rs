use crate::economy::Resources;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const PLAYER_DEITY_ID: u32 = 0;

/// Personality & strategic archetype for deities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum DeityArchetype {
    AscendantPlayer,
    Conqueror,
    Harvester,
    Preserver,
    Architect,
    Zealot,
    Voidbringer,
    Technocrat,
    Schemer,
}

impl DeityArchetype {
    pub fn title_tr(self) -> &'static str {
        match self {
            Self::AscendantPlayer => "Yükselen İlah",
            Self::Conqueror => "Kozmik Fatih",
            Self::Harvester => "Ruh Hasatçısı",
            Self::Preserver => "Yaşam Koruyucusu",
            Self::Architect => "Gerçeklik Mimarı",
            Self::Zealot => "Kehanet Hâkimi",
            Self::Voidbringer => "Çöküş Elçisi",
            Self::Technocrat => "Mekanik Düzen İlahı",
            Self::Schemer => "Gölge Diplomat",
        }
    }

    pub fn goal_description_tr(self) -> &'static str {
        match self {
            Self::AscendantPlayer => {
                "Kozmik havuzda hâkimiyet, aşkınlık veya yaratım yoluyla yükselmek."
            }
            Self::Conqueror => {
                "Rakip tanrıların nüfuzunu kırıp evrenleri askeri ve ilahi güçle ele geçirmek."
            }
            Self::Harvester => "Kaos ve çöken evrenlerden maksimum ruh ve kozmik öz hasat etmek.",
            Self::Preserver => "Yaşam evrenlerini korumak ve çöküşleri durdurmak.",
            Self::Architect => "İlksel evrenleri biçimlendirip düzenli bir kozmik ağ kurmak.",
            Self::Zealot => "Bütün uygarlıkları tek bir mutlak inanç altında toplamak.",
            Self::Voidbringer => "Evrenlerin istikrarını bozup çöküş enerjisinden beslenmek.",
            Self::Technocrat => "İleri teknoloji evrenlerinde sarsılmaz bir düzen kurmak.",
            Self::Schemer => "İttifaklar, haraçlar ve isyanlarla rakipleri birbirine düşürmek.",
        }
    }
}

/// The 5 Divine Power Development Trees (`taslak.md`: "5 gelişim ağacı").
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PowerTree {
    /// 1. Yaratım: Life seeding, reality stabilization, universe creation.
    Creation,
    /// 2. İnanç ve Hâkimiyet: Prophets, miracles, mass conversion, divine edicts.
    FaithDominion,
    /// 3. Hasat ve Ruh Akışı: Soul siphon, death rituals, cosmic essence extraction.
    SoulHarvest,
    /// 4. Kozmik Gezginlik ve Geçitler: Deep scouting, portal forging, dimensional locks.
    CosmicTraversal,
    /// 5. Tanrılar Arası Siyaset ve Yıkım: Rival smiting, schisms, sealing, diplomacy.
    DeityWarfare,
}

impl PowerTree {
    pub const ALL: [Self; 5] = [
        Self::Creation,
        Self::FaithDominion,
        Self::SoulHarvest,
        Self::CosmicTraversal,
        Self::DeityWarfare,
    ];

    pub fn name_tr(self) -> &'static str {
        match self {
            Self::Creation => "Yaratım Ağacı",
            Self::FaithDominion => "İnanç ve Hâkimiyet",
            Self::SoulHarvest => "Ruh Hasadı ve Öz",
            Self::CosmicTraversal => "Kozmik Geçitler",
            Self::DeityWarfare => "Tanrılar Savaşı",
        }
    }
}

/// 15 Divine Powers (3 tiers per tree across the 5 development trees).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PowerId {
    // Creation Tree
    GenesisSpark,
    RealityAnchor,
    CosmicForge,
    // Faith & Dominion Tree
    VoiceOfProphecy,
    GoldenMiracle,
    AbsoluteDogma,
    // Soul Harvest Tree
    SoulSiphon,
    ReapersToll,
    EssenceCondenser,
    // Cosmic Traversal Tree
    AstralSight,
    GatewayMastery,
    DimensionalWard,
    // Deity Warfare Tree
    ShadowSchism,
    DivineSmite,
    EternalSeal,
}

impl PowerId {
    pub const ALL: [Self; 15] = [
        Self::GenesisSpark,
        Self::RealityAnchor,
        Self::CosmicForge,
        Self::VoiceOfProphecy,
        Self::GoldenMiracle,
        Self::AbsoluteDogma,
        Self::SoulSiphon,
        Self::ReapersToll,
        Self::EssenceCondenser,
        Self::AstralSight,
        Self::GatewayMastery,
        Self::DimensionalWard,
        Self::ShadowSchism,
        Self::DivineSmite,
        Self::EternalSeal,
    ];

    pub fn tree(self) -> PowerTree {
        match self {
            Self::GenesisSpark | Self::RealityAnchor | Self::CosmicForge => PowerTree::Creation,
            Self::VoiceOfProphecy | Self::GoldenMiracle | Self::AbsoluteDogma => {
                PowerTree::FaithDominion
            }
            Self::SoulSiphon | Self::ReapersToll | Self::EssenceCondenser => PowerTree::SoulHarvest,
            Self::AstralSight | Self::GatewayMastery | Self::DimensionalWard => {
                PowerTree::CosmicTraversal
            }
            Self::ShadowSchism | Self::DivineSmite | Self::EternalSeal => PowerTree::DeityWarfare,
        }
    }

    pub fn tier(self) -> u8 {
        match self {
            Self::GenesisSpark
            | Self::VoiceOfProphecy
            | Self::SoulSiphon
            | Self::AstralSight
            | Self::ShadowSchism => 1,
            Self::RealityAnchor
            | Self::GoldenMiracle
            | Self::ReapersToll
            | Self::GatewayMastery
            | Self::DivineSmite => 2,
            Self::CosmicForge
            | Self::AbsoluteDogma
            | Self::EssenceCondenser
            | Self::DimensionalWard
            | Self::EternalSeal => 3,
        }
    }

    pub fn prerequisite(self) -> Option<PowerId> {
        match self {
            Self::RealityAnchor => Some(Self::GenesisSpark),
            Self::CosmicForge => Some(Self::RealityAnchor),
            Self::GoldenMiracle => Some(Self::VoiceOfProphecy),
            Self::AbsoluteDogma => Some(Self::GoldenMiracle),
            Self::ReapersToll => Some(Self::SoulSiphon),
            Self::EssenceCondenser => Some(Self::ReapersToll),
            Self::GatewayMastery => Some(Self::AstralSight),
            Self::DimensionalWard => Some(Self::GatewayMastery),
            Self::DivineSmite => Some(Self::ShadowSchism),
            Self::EternalSeal => Some(Self::DivineSmite),
            _ => None,
        }
    }

    pub fn unlock_cost(self) -> Resources {
        match self.tier() {
            1 => Resources::new(120, 45, 15),
            2 => Resources::new(280, 110, 45),
            _ => Resources::new(550, 220, 110),
        }
    }

    pub fn name_tr(self) -> &'static str {
        match self {
            Self::GenesisSpark => "Yaratılış Kıvılcımı",
            Self::RealityAnchor => "Gerçeklik Çapası",
            Self::CosmicForge => "Kozmik Ocak",
            Self::VoiceOfProphecy => "Kehanet Sesi",
            Self::GoldenMiracle => "Altın Mucize",
            Self::AbsoluteDogma => "Mutlak Dogma",
            Self::SoulSiphon => "Ruh Sifonu",
            Self::ReapersToll => "Biçicinin Bedeli",
            Self::EssenceCondenser => "Öz Yoğunlaştırıcı",
            Self::AstralSight => "Astral Görüş",
            Self::GatewayMastery => "Geçit Ustalığı",
            Self::DimensionalWard => "Boyutsal Mühür",
            Self::ShadowSchism => "Gölge Ayrılığı",
            Self::DivineSmite => "İlahi Gazap",
            Self::EternalSeal => "Ebedî Mühür",
        }
    }

    pub fn description_tr(self) -> &'static str {
        match self {
            Self::GenesisSpark => {
                "Anında tüm uygarlıklarına +35M nüfus verir; Yaşam Tohumlamayı +%35 ve nüfus artışını +%25 güçlendirir."
            }
            Self::RealityAnchor => {
                "Anında hâkim evrenlerine +15 İstikrar verir; Gerçeklik Onarımı maliyetini %50 düşürür ve her tur +0.6 İstikrar yeniler."
            }
            Self::CosmicForge => {
                "Anında +80 Öz verir; şekillendirdiğin her evrenden her tur ek +4 Ruh, +3 Nüfuz, +2 Öz üretir ve uygarlık sınırını 6'ya çıkarır."
            }
            Self::VoiceOfProphecy => {
                "Anında keşfedilen evrenlerde +8 Nüfuz yayar; Kehanet etkisini +%55 artırır ve hâkim evrenlerde her tur +0.7 Nüfuz üretir."
            }
            Self::GoldenMiracle => {
                "Anında tüm isyanları bitirir; Mucize etkisini 1.85x katlar, bilimsel kalkanları çözer ve isyan riskini yarıya indirir."
            }
            Self::AbsoluteDogma => {
                "Anında +120 Nüfuz verir; ibadet ruh/nüfuz gelirini +%45 artırır ve ★ Tam Aidiyet evrenlerini rakip sızmasına korur."
            }
            Self::SoulSiphon => {
                "Anında +180 Ruh ve havuzlara +50 Ruh verir; ölümlü/hasat ruhlarını +%40 artırır ve her tur havuzları +6 Ruh besler."
            }
            Self::ReapersToll => {
                "Anında +150 Ruh ve +40 Öz verir; Ruh Hasadında +15 Kozmik Öz kazandırır ve savaş/anomalilerden ek ganimet toplar."
            }
            Self::EssenceCondenser => {
                "Anında tüm hâkim evrenlere +0.40x Öz Zenginliği ve +75 Öz verir; her evrenden pasif Öz üretir ve damar yorgunluğunu %50 azaltır."
            }
            Self::AstralSight => {
                "Anında 3 gizemli evreni keşfeder; Keşif maliyetini %50 düşürür, 2 sıçrama ötesini açar ve her 4 turda 1 evreni otomatik keşfeder."
            }
            Self::GatewayMastery => {
                "Anında komşu geçitleri açar; Geçit maliyetini %50 düşürür, geçit inanç yayılımını 2x hızlandırır ve açık geçit başına +2N üretir."
            }
            Self::DimensionalWard => {
                "Anında +20 İstikrar ve +60 Öz verir; kapatılan geçitleri Elmas Mühürle kilitler, çöküş ve aşınma kayıplarını %75 azaltır."
            }
            Self::ShadowSchism => {
                "Anında +90 Nüfuz verir ve rakiplerde kargaşa çıkarır; İsyan Kışkırtma (-28 rakip nüfuzu) ve Haraç gelirini +%40 güçlendirir."
            }
            Self::DivineSmite => {
                "Anında düşman nüfuzlarını sarsar ve +150 Ruh verir; İlahi Darbe hasarını 42'ye çıkarır ve her vuruşta +30 Ruh ganimeti toplar."
            }
            Self::EternalSeal => {
                "Anında rakiplerin ritüelini -%15 geriletir ve +200R/+80N verir; 2 evrenli rakipleri mühürler ve mühür başına her tur +8R/+5N/+2Ö üretir."
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiplomaticStance {
    Hostile,
    Rival,
    Neutral,
    Pact { ticks_remaining: u32 },
    Subjugated,
}

impl DiplomaticStance {
    pub fn label_tr(self) -> &'static str {
        match self {
            Self::Hostile => "Düşman",
            Self::Rival => "Rakip",
            Self::Neutral => "Mesafeli",
            Self::Pact { .. } => "Ateşkes Paktı",
            Self::Subjugated => "Boyun Eğmiş",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Deity {
    pub id: u32,
    pub name: String,
    pub title: String,
    pub archetype: DeityArchetype,
    pub seal_id: u8,
    pub color_hex: String,
    pub resources: Resources,
    pub last_tick_income: Resources,
    pub unlocked_powers: BTreeSet<PowerId>,
    pub relations: BTreeMap<u32, i32>,
    pub stances: BTreeMap<u32, DiplomaticStance>,
    pub is_sealed: bool,
    pub sealed_by: Option<u32>,
    pub shaped_universes_count: u32,
    /// Progress toward Cosmic Transcendence (0.0 to 100.0).
    pub ascension_progress: f32,
}

impl Deity {
    pub fn has_power(&self, power: PowerId) -> bool {
        self.unlocked_powers.contains(&power)
    }

    pub fn can_unlock_power(&self, power: PowerId) -> bool {
        if self.has_power(power) {
            return false;
        }
        if let Some(prereq) = power.prerequisite() {
            if !self.has_power(prereq) {
                return false;
            }
        }
        self.resources.can_afford(&power.unlock_cost())
    }

    pub fn stance_toward(&self, other_id: u32) -> DiplomaticStance {
        self.stances
            .get(&other_id)
            .copied()
            .unwrap_or(DiplomaticStance::Rival)
    }

    pub fn relation_toward(&self, other_id: u32) -> i32 {
        self.relations.get(&other_id).copied().unwrap_or(0)
    }

    pub fn adjust_relation(&mut self, other_id: u32, delta: i32) {
        let cur = self.relation_toward(other_id);
        let next = (cur + delta).clamp(-100, 100);
        self.relations.insert(other_id, next);
        if !matches!(
            self.stance_toward(other_id),
            DiplomaticStance::Pact { .. } | DiplomaticStance::Subjugated
        ) {
            let new_stance = if next <= -45 {
                DiplomaticStance::Hostile
            } else if next >= 25 {
                DiplomaticStance::Neutral
            } else {
                DiplomaticStance::Rival
            };
            self.stances.insert(other_id, new_stance);
        }
    }
}
