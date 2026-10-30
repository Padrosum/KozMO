use crate::civilization::{Civilization, CivilizationDoctrine};
use crate::deity::{Deity, DeityArchetype, DiplomaticStance, PowerId, PLAYER_DEITY_ID};
use crate::economy::{
    ResourceExpenditure, ResourceLedger, ResourceRemovalReason, ResourceSource, Resources,
};
use crate::events::{CosmicEvent, CosmicEventKind, EventSeverity};
use crate::rng::DeterministicRng;
use crate::universe::{
    GatewayConnection, GatewayState, HistoryCategory, Universe, UniverseEconomyBreakdown,
    UniverseType,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[path = "simulation/economy.rs"]
mod economy_phase;
#[path = "simulation/endgame.rs"]
mod endgame;

pub const DEFAULT_UNIVERSE_COUNT: usize = 60;
pub const DEFAULT_RIVAL_DEITY_COUNT: usize = 8;

// Challenging multi-pillar Victory & Endgame thresholds
pub const DOMINION_REQUIRED_DOMINATED: usize = 35;
pub const DOMINION_REQUIRED_FULL_ALLEGIANCE: usize = 18;
pub const DOMINION_REQUIRED_SEALED_RIVALS: usize = 5;

pub const TRANSCENDENCE_REQUIRED_POWERS: usize = 10;
pub const TRANSCENDENCE_REQUIRED_FULL_ALLEGIANCE: usize = 10;
pub const TRANSCENDENCE_MIN_POWERS_TO_START: usize = 4;
pub const TRANSCENDENCE_MIN_FULL_ALLEGIANCE_TO_START: usize = 3;
pub const TRANSCENDENCE_STEP_GAIN: f32 = 10.0;

pub const CREATION_REQUIRED_SHAPED_UNIVERSES: usize = 15;
pub const CREATION_REQUIRED_STABILITY: f32 = 75.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameMode {
    /// Kozmik Yükseliş — Senaryolu mod (has victory & defeat conditions).
    ScenarioCosmicAscension,
    /// Sonsuz Havuz — Sandbox modu (endless play or optional goals).
    SandboxEndlessPool,
}

impl GameMode {
    pub fn name_tr(self) -> &'static str {
        match self {
            Self::ScenarioCosmicAscension => "Kozmik Yükseliş (Senaryo)",
            Self::SandboxEndlessPool => "Sonsuz Havuz (Sandbox)",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DifficultyLevel {
    Story,
    Balanced,
    Grandmaster,
}

impl DifficultyLevel {
    pub fn name_tr(self) -> &'static str {
        match self {
            Self::Story => "Gözlemci (Kolay)",
            Self::Balanced => "Kozmik Denge (Normal)",
            Self::Grandmaster => "Ebedî Kaos (Zor)",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VictoryPath {
    /// Hâkimiyet: Rakip tanrıların gücünü kırıp havuzun büyük bölümünde üstünlük kurmak.
    Dominion,
    /// Aşkınlık: Yeterli ruh ve kozmik öz toplayarak tanrısal varoluş biçimini aşmak.
    Transcendence,
    /// Yaratım: Kendi düzenine bağlı yeni/onarılmış evrenlerden oluşan bir alan kurmak.
    Creation,
}

impl VictoryPath {
    pub fn name_tr(self) -> &'static str {
        match self {
            Self::Dominion => "Hâkimiyet Zaferi",
            Self::Transcendence => "Aşkınlık Zaferi",
            Self::Creation => "Yaratım Zaferi",
        }
    }

    pub fn description_tr(self) -> &'static str {
        match self {
            Self::Dominion => {
                "En az 35 evrende baskın ol, 18 evrende ★ Tam Aidiyet (%65+) kur ve 5 rakip tanrıyı mühürle."
            }
            Self::Transcendence => {
                "10 aşamalı Aşkınlık Ritüelini %100'e ulaştır, en az 10 ilahi güç aç ve 10 ★ Tam Aidiyet evrenine sahip ol."
            }
            Self::Creation => {
                "Yaratım Ağacını tamamla (3/3) ve en az 15 evrende ★ Tam Aidiyet, %75+ İstikrar ve aktif uygarlıkla düzen kur."
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum TutorialStep {
    #[default]
    Step1Prophecy,
    Step2Explore,
    Step3HarvestOrEssence,
    Step4UnlockPower,
    Step5CreateOrStabilize,
    Step6RivalInteraction,
    Completed,
}

impl TutorialStep {
    pub fn step_number(self) -> u8 {
        match self {
            Self::Step1Prophecy => 1,
            Self::Step2Explore => 2,
            Self::Step3HarvestOrEssence => 3,
            Self::Step4UnlockPower => 4,
            Self::Step5CreateOrStabilize => 5,
            Self::Step6RivalInteraction => 6,
            Self::Completed => 6,
        }
    }

    pub fn title_tr(self) -> &'static str {
        match self {
            Self::Step1Prophecy => "Adım 1/6: İlk Kehanet",
            Self::Step2Explore => "Adım 2/6: Kozmik Keşif ve Geçitler",
            Self::Step3HarvestOrEssence => "Adım 3/6: Ruh Hasadı veya Öz Çıkarımı",
            Self::Step4UnlockPower => "Adım 4/6: İlahi Güç Ağacı",
            Self::Step5CreateOrStabilize => "Adım 5/6: Evren Şekillendirme",
            Self::Step6RivalInteraction => "Adım 6/6: Tanrılar Arası Mücadele ve Diplomasi",
            Self::Completed => "Başlangıç Eğitimi Tamamlandı!",
        }
    }

    pub fn instruction_tr(self) -> &'static str {
        match self {
            Self::Step1Prophecy => {
                "Seçili başlangıç evreninde 'Kehanet Yay' emrine tıklayarak ölümlüler arasında inancını güçlendir."
            }
            Self::Step2Explore => {
                "'Kozmik Keşif' veya 'Geçitleri Aç' emrini kullanarak dış halkadaki gizemli evrenleri haritada görünür kıl."
            }
            Self::Step3HarvestOrEssence => {
                "'Ruh Hasadı' veya 'Kozmik Öz Çıkar' emrini vererek tanrısal hazineni büyüt."
            }
            Self::Step4UnlockPower => {
                "Sağ paneldeki 'Güç Ağaçları' sekmesine geç ve 5 gelişim ağacından ilk ilahi gücünü aç."
            }
            Self::Step5CreateOrStabilize => {
                "Bir evrende 'Yaşam Tohumla', 'Gerçekliği Onar' veya 'Mucize Bahşet' emrini uygula."
            }
            Self::Step6RivalInteraction => {
                "Bir rakip tanrıya karşı 'Rakip Vur', 'İsyan Çıkar' veya 'Tanrılar' sekmesinden 'Ateşkes Paktı Teklif Et' uygula."
            }
            Self::Completed => {
                "Tüm temel sistemleri öğrendin! Şimdi zorlu Hâkimiyet (35 Evren), Aşkınlık (10 Aşama) veya Yaratım (15 Evren) zaferine ulaş."
            }
        }
    }

    pub fn reward(self) -> Resources {
        match self {
            Self::Step1Prophecy => Resources::new(60, 25, 0),
            Self::Step2Explore => Resources::new(80, 0, 20),
            Self::Step3HarvestOrEssence => Resources::new(0, 40, 25),
            Self::Step4UnlockPower => Resources::new(120, 35, 0),
            Self::Step5CreateOrStabilize => Resources::new(100, 0, 40),
            Self::Step6RivalInteraction => Resources::new(150, 50, 50),
            Self::Completed => Resources::ZERO,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CosmicDilemma {
    pub id: u64,
    pub universe_id: u32,
    pub title: String,
    pub description: String,
    pub choice_a_label: String,
    pub choice_b_label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameOutcome {
    Ongoing,
    PlayerVictory {
        path: VictoryPath,
        tick: u64,
        summary: String,
    },
    PlayerDefeat {
        tick: u64,
        reason: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DivineAction {
    /// Kehanet Yay: Evrende nüfuz ve inanç ateşini artırır.
    SpreadProphecy { universe_id: u32 },
    /// Ruh Hasadı: Evrendeki birikmiş ruhları ve ölümlü yaşam enerjisini toplar.
    HarvestSouls { universe_id: u32 },
    /// Mucize Bahşet: İstikrarı, nüfusu ve bağlılığı güçlendirir, isyanları dindirir.
    BestowMiracle { universe_id: u32 },
    /// Yaşam ve Düzen Tohumla: İlksel evreni canlandırır veya yeni uygarlık kurar.
    SeedLifeAndOrder { universe_id: u32 },
    /// Gerçekliği Onar: Çöken veya kaos içindeki evreni istikrara kavuşturur.
    StabilizeReality { universe_id: u32 },
    /// Kozmik Öz Çıkar: İlksel, çöken veya kaos evrenlerinden Kozmik Öz damıtır.
    ExtractCosmicEssence { universe_id: u32 },
    /// Rakip Nüfuzunu Vur: Evrendeki bir rakip tanrının etkisini kırar.
    SmiteRivalInfluence {
        universe_id: u32,
        target_deity_id: u32,
    },
    /// Ayrılık ve İsyan Çıkar: Rakip tanrıya bağlı uygarlıklarda isyan başlatır.
    InciteRebellion { universe_id: u32 },
    /// Geçit Aç: İki bağlantılı evren arasındaki geçidi açar ve hedefi keşfeder.
    OpenGateway {
        from_universe_id: u32,
        to_universe_id: u32,
    },
    /// Geçit Kapat / Kilitle: Evrenler arası bağlantıyı kapatır.
    CloseGateway {
        from_universe_id: u32,
        to_universe_id: u32,
    },
    /// Kozmik Keşif: Seçili evrenin komşu evrenlerini haritada görünür kılar.
    ScoutRegion { universe_id: u32 },
    /// Tanrısal Güç Aç: 5 gelişim ağacından bir yeteneği kalıcı olarak açar.
    UnlockPower { power: PowerId },
    /// Ateşkes Paktı Teklif Et: Rakip tanrı ile geçici saldırmazlık anlaşması kurar.
    OfferDiplomaticPact { target_deity_id: u32 },
    /// Ruh Armağanı Gönder: Rakip tanrıya ruh sunarak ilişkiyi düzeltir ve diplomatik nüfuz kazanır.
    SendDivineGift { target_deity_id: u32 },
    /// Haraç Talep Et: Zayıf bir rakip tanrıdan ruh ve nüfuz talep eder.
    DemandTribute { target_deity_id: u32 },
    /// Kutsal Rekabet / Savaş İlan Et: Rakip tanrıya meydan okuyarak kendi müritlerinde şevki ateşler.
    DeclareHolyWar { target_deity_id: u32 },
    /// Zayıflamış Tanrıyı Mühürle: Evren hâkimiyetini yitirmiş bir rakibi mühürler.
    SealWeakenedDeity { target_deity_id: u32 },
    /// Rakip Aşkınlık Ritüelini Sabote Et: Rakip tanrının ritüel ilerlemesini -%20 düşürür.
    DisruptRivalAscension { target_deity_id: u32 },
    /// Aşkınlık Ritüelini İlerlet: Aşkınlık zaferi için ruh ve kozmik öz kanalize eder (+%10).
    AdvanceAscensionRitual,
    /// Evrene Odaklan: Seçili evreni ilahi odak altına alır (+%25 ibadet geliri ve istikrar desteği).
    FocusUniverse { universe_id: Option<u32> },
    /// Kozmik İkilem Kararı Ver: Aktif kozmik olaya ilahi yanıt seçer.
    ResolveDilemma { choose_first: bool },
    /// Uygarlıkları Aydınlat ve Bilimsel Kalkanı Çöz: Evrendeki uygarlıkların kalkanını çözer, isyanları dindirir ve inançlarını yükseltir.
    EnlightenCivilizations { universe_id: u32 },
    /// Evren Dokusunu Dönüştür: Evrenin uzmanlık dokusunu dönüştürür, ruh potansiyeli ve kozmik öz zenginliğini kalıcı artırır.
    ReshapeUniverseDomain { universe_id: u32 },
    /// Kozmik Rezonans Başlat: Evrenin ortam enerjisini kanalize ederek ruh üretir ve açık geçitlerden komşulara nüfuz yayar.
    ChannelAmbientEnergy { universe_id: u32 },
}

impl DivineAction {
    /// Returns `Some(universe_id)` if this action requires the actor deity to be the dominant deity
    /// ("söz ve güç sahibi") in that universe.
    pub fn requires_dominion_in(&self) -> Option<u32> {
        match *self {
            Self::HarvestSouls { universe_id }
            | Self::BestowMiracle { universe_id }
            | Self::SeedLifeAndOrder { universe_id }
            | Self::StabilizeReality { universe_id }
            | Self::ExtractCosmicEssence { universe_id }
            | Self::ScoutRegion { universe_id }
            | Self::EnlightenCivilizations { universe_id }
            | Self::ReshapeUniverseDomain { universe_id }
            | Self::ChannelAmbientEnergy { universe_id }
            | Self::CloseGateway {
                from_universe_id: universe_id,
                ..
            } => Some(universe_id),
            Self::FocusUniverse {
                universe_id: Some(u_id),
            } => Some(u_id),
            _ => None,
        }
    }

    pub fn base_cost(&self, deity: &Deity) -> Resources {
        let gw_discount = if deity.has_power(PowerId::GatewayMastery) {
            2
        } else {
            1
        };
        let anchor_discount = if deity.has_power(PowerId::RealityAnchor) {
            2
        } else {
            1
        };
        let astral_discount = if deity.has_power(PowerId::AstralSight) {
            2
        } else {
            1
        };
        let stage = (deity.ascension_progress / TRANSCENDENCE_STEP_GAIN).floor() as i64;

        match self {
            Self::SpreadProphecy { .. } => Resources::new(20, 25, 0),
            Self::HarvestSouls { .. } => Resources::new(0, 25, 0),
            Self::BestowMiracle { .. } => Resources::new(60, 25, 0),
            Self::SeedLifeAndOrder { .. } => Resources::new(110, 40, 25),
            Self::StabilizeReality { .. } => Resources::new(
                85 / anchor_discount,
                35 / anchor_discount,
                20 / anchor_discount,
            ),
            Self::ExtractCosmicEssence { .. } => Resources::new(65, 25, 0),
            Self::SmiteRivalInfluence { .. } => Resources::new(75, 35, 10),
            Self::InciteRebellion { .. } => Resources::new(30, 40, 0),
            Self::OpenGateway { .. } => {
                Resources::new(40 / gw_discount, 20 / gw_discount, 10 / gw_discount)
            }
            Self::CloseGateway { .. } => {
                Resources::new(25 / gw_discount, 15 / gw_discount, 5 / gw_discount)
            }
            Self::ScoutRegion { .. } => {
                Resources::new(20 / astral_discount, 20 / astral_discount, 0)
            }
            Self::UnlockPower { power } => power.unlock_cost(),
            Self::OfferDiplomaticPact { .. } => Resources::new(35, 45, 0),
            Self::SendDivineGift { .. } => Resources::new(70, 0, 5),
            Self::DemandTribute { .. } => Resources::new(20, 35, 0),
            Self::DeclareHolyWar { .. } => Resources::new(60, 40, 10),
            Self::SealWeakenedDeity { .. } => Resources::new(320, 150, 80),
            Self::DisruptRivalAscension { .. } => Resources::new(220, 110, 45),
            Self::AdvanceAscensionRitual => {
                Resources::new(450 + stage * 45, 140 + stage * 15, 100 + stage * 12)
            }
            Self::EnlightenCivilizations { .. } => Resources::new(35, 20, 0),
            Self::ReshapeUniverseDomain { .. } => Resources::new(
                55 / anchor_discount,
                25 / anchor_discount,
                20 / anchor_discount,
            ),
            Self::ChannelAmbientEnergy { .. } => Resources::new(0, 30, 10),
            Self::FocusUniverse { .. } | Self::ResolveDilemma { .. } => Resources::ZERO,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub seed: u64,
    pub universe_count: usize,
    pub rival_deity_count: usize,
    pub game_mode: GameMode,
    pub difficulty: DifficultyLevel,
    pub player_name: String,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            seed: 42_026,
            universe_count: DEFAULT_UNIVERSE_COUNT,
            rival_deity_count: DEFAULT_RIVAL_DEITY_COUNT,
            game_mode: GameMode::ScenarioCosmicAscension,
            difficulty: DifficultyLevel::Balanced,
            player_name: "Aethelgard".to_string(),
        }
    }
}

/// Complete, serializable, Bevy-independent simulation state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationState {
    pub config: SimulationConfig,
    pub tick: u64,
    pub rng: DeterministicRng,
    pub universes: Vec<Universe>,
    pub deities: Vec<Deity>,
    pub ledger: ResourceLedger,
    pub events: Vec<CosmicEvent>,
    pub next_event_id: u64,
    pub next_civ_id: u32,
    pub outcome: GameOutcome,
    /// Enables lower-frequency background updates on distant unobserved universes while preserving consistency.
    pub enable_background_optimization: bool,
    /// Current step of the interactive in-game tutorial questline.
    #[serde(default)]
    pub tutorial_step: TutorialStep,
    /// Universe currently under the player's direct cosmic focus (+25% devotion yield & stability support).
    #[serde(default)]
    pub focused_universe_id: Option<u32>,
    /// Active interactive cosmic dilemma awaiting player choice.
    #[serde(default)]
    pub active_dilemma: Option<CosmicDilemma>,
    /// Next simulation tick at which a randomized cosmic event will trigger (interval: 2..=6 ticks).
    #[serde(default)]
    pub next_random_event_tick: u64,
    /// Universe ID of the most recent random cosmic event (used for visual map beacon & quick jump).
    #[serde(default)]
    pub last_event_universe_id: Option<u32>,
}

impl SimulationState {
    pub fn new(config: SimulationConfig) -> Self {
        let mut rng = DeterministicRng::new(config.seed);
        let mut ledger = ResourceLedger::new(250);
        let mut next_civ_id = 1u32;

        // 1. Create Player Deity + 8 Rival Deities
        let mut deities = Self::generate_deities(&config, &mut rng, &mut ledger);

        // 2. Create 60 Universes across 4 concentric cosmic rings with 6 categories
        let mut universes = Self::generate_universes(
            config.universe_count.max(12),
            &mut rng,
            &mut next_civ_id,
            &deities,
        );

        // 3. Assign starting home universes and initial influence
        Self::assign_starting_positions(&mut universes, &mut deities, &mut rng);

        // 4. Record opening cosmic event
        let mut state = Self {
            config,
            tick: 0,
            rng,
            universes,
            deities,
            ledger,
            events: Vec::new(),
            next_event_id: 1,
            next_civ_id,
            outcome: GameOutcome::Ongoing,
            enable_background_optimization: true,
            tutorial_step: TutorialStep::Step1Prophecy,
            focused_universe_id: Some(0),
            active_dilemma: None,
            next_random_event_tick: 2,
            last_event_universe_id: None,
        };

        state.push_event(
            Some(0),
            Some(PLAYER_DEITY_ID),
            CosmicEventKind::AscensionMilestone,
            EventSeverity::Positive,
            "Kozmik Uyanış",
            "Evrenler havuzunun merkezinde bilincin uyandı. Çevrendeki geçitler ve uygarlıklar ilahi iradeni bekliyor.",
        );

        state
    }

    fn generate_deities(
        config: &SimulationConfig,
        rng: &mut DeterministicRng,
        ledger: &mut ResourceLedger,
    ) -> Vec<Deity> {
        let mut deities = Vec::new();

        // Player Deity (ID 0)
        let mut player_res = Resources::ZERO;
        let initial_player_grant = match config.difficulty {
            DifficultyLevel::Story => Resources::new(420, 180, 60),
            DifficultyLevel::Balanced => Resources::new(300, 130, 40),
            DifficultyLevel::Grandmaster => Resources::new(220, 95, 25),
        };
        ledger.record_source(
            0,
            &mut player_res,
            PLAYER_DEITY_ID,
            ResourceSource::InitialGrant,
            initial_player_grant,
            Some(0),
            "Yükseliş başlangıç özü",
        );

        deities.push(Deity {
            id: PLAYER_DEITY_ID,
            name: config.player_name.clone(),
            title: DeityArchetype::AscendantPlayer.title_tr().to_string(),
            archetype: DeityArchetype::AscendantPlayer,
            seal_id: 1,
            color_hex: "#9B7BFF".to_string(),
            resources: player_res,
            last_tick_income: Resources::ZERO,
            unlocked_powers: BTreeSet::new(),
            relations: BTreeMap::new(),
            stances: BTreeMap::new(),
            is_sealed: false,
            sealed_by: None,
            shaped_universes_count: 0,
            ascension_progress: 0.0,
        });

        let rival_specs = [
            ("Vaelkor", DeityArchetype::Conqueror, 2, "#F06A78"),
            ("Morgeth", DeityArchetype::Harvester, 3, "#D946EF"),
            ("Sylvara", DeityArchetype::Preserver, 4, "#5ED6C1"),
            ("Aurelion", DeityArchetype::Architect, 5, "#E5C77A"),
            ("Solinvictus", DeityArchetype::Zealot, 6, "#F59E0B"),
            ("Nyxarion", DeityArchetype::Voidbringer, 7, "#FB7185"),
            ("Mechanon", DeityArchetype::Technocrat, 8, "#38BDF8"),
            ("Vespera", DeityArchetype::Schemer, 9, "#A78BFA"),
        ];

        let rival_count = config.rival_deity_count.min(rival_specs.len());
        let ai_mult = match config.difficulty {
            DifficultyLevel::Story => 0.85,
            DifficultyLevel::Balanced => 1.0,
            DifficultyLevel::Grandmaster => 1.35,
        };

        for (idx, &(name, archetype, seal_id, color_hex)) in
            rival_specs.iter().enumerate().take(rival_count)
        {
            let deity_id = (idx + 1) as u32;
            let mut ai_res = Resources::ZERO;
            let grant = Resources::new(
                (280.0 * ai_mult) as i64,
                (120.0 * ai_mult) as i64,
                (35.0 * ai_mult) as i64,
            );
            ledger.record_source(
                0,
                &mut ai_res,
                deity_id,
                ResourceSource::InitialGrant,
                grant,
                None,
                format!("{name} başlangıç özü"),
            );

            deities.push(Deity {
                id: deity_id,
                name: name.to_string(),
                title: archetype.title_tr().to_string(),
                archetype,
                seal_id,
                color_hex: color_hex.to_string(),
                resources: ai_res,
                last_tick_income: Resources::ZERO,
                unlocked_powers: BTreeSet::new(),
                relations: BTreeMap::new(),
                stances: BTreeMap::new(),
                is_sealed: false,
                sealed_by: None,
                shaped_universes_count: 0,
                ascension_progress: 0.0,
            });
        }

        // Initialize diplomatic relations between all deities
        let n = deities.len();
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                let other_id = deities[j].id;
                let base_rel = rng.range_u32(0, 50) as i32 - 30;
                deities[i].relations.insert(other_id, base_rel);
                let stance = if base_rel <= -20 {
                    DiplomaticStance::Hostile
                } else if base_rel >= 10 {
                    DiplomaticStance::Neutral
                } else {
                    DiplomaticStance::Rival
                };
                deities[i].stances.insert(other_id, stance);
            }
        }

        deities
    }

    fn generate_universes(
        count: usize,
        rng: &mut DeterministicRng,
        next_civ_id: &mut u32,
        deities: &[Deity],
    ) -> Vec<Universe> {
        let prefixes = [
            "Aethel", "Vora", "Kael", "Nyx", "Sola", "Thal", "Oron", "Zeph", "Kryo", "Myr",
            "Vesper", "Chron", "Astra", "Pyra", "Lyra", "Ereb", "Xeno", "Val", "Drak", "Seraph",
        ];
        let suffixes = [
            "Prime", "Nexus", "Sanctum", "Vortex", "Reach", "Abyss", "Cradle", "Spire", "Veil",
            "Echo", "Forge", "Shard", "Crown", "Hollow", "Horizon",
        ];

        // Distribute universe types evenly across the 6 categories, then shuffle
        let mut u_types = Vec::with_capacity(count);
        for i in 0..count {
            u_types.push(UniverseType::ALL[i % UniverseType::ALL.len()]);
        }
        rng.shuffle(&mut u_types);
        // Ensure universe 0 (player starting universe) is a Life or Faith universe
        u_types[0] = UniverseType::Life;

        // Ring definitions: (ring_tier, count_in_ring, radius)
        let mut universes = Vec::with_capacity(count);

        for (i, &u_type) in u_types.iter().enumerate().take(count) {
            let (region_tier, ring_idx, ring_total, base_radius) = if i < 8 {
                (0u8, i, 8usize, 130.0f32)
            } else if i < 22 {
                (1u8, i - 8, 14usize, 265.0f32)
            } else if i < 40 {
                (2u8, i - 22, 18usize, 405.0f32)
            } else {
                (3u8, i - 40, count.saturating_sub(40).max(1), 545.0f32)
            };

            let angle_step = std::f32::consts::TAU / (ring_total as f32);
            let angle_offset = (region_tier as f32) * 0.37 + rng.range_f32(-0.14, 0.14);
            let angle = (ring_idx as f32) * angle_step + angle_offset;
            let radius = base_radius + rng.range_f32(-22.0, 22.0);
            let x = angle.cos() * radius;
            let y = angle.sin() * radius;

            let p_idx = (i * 7 + rng.range_usize(0, prefixes.len())) % prefixes.len();
            let s_idx = (i * 11 + rng.range_usize(0, suffixes.len())) % suffixes.len();
            let name = format!("{}-{} #{}", prefixes[p_idx], suffixes[s_idx], i + 1);

            let (stability, danger_level, soul_potential, essence_richness, civ_count) =
                match u_type {
                    UniverseType::Life => (
                        rng.range_f32(68.0, 92.0),
                        rng.range_f32(10.0, 35.0),
                        rng.range_f32(1.4, 2.2),
                        rng.range_f32(0.5, 1.2),
                        rng.range_usize(1, 3),
                    ),
                    UniverseType::Chaos => (
                        rng.range_f32(28.0, 55.0),
                        rng.range_f32(60.0, 90.0),
                        rng.range_f32(1.3, 2.5),
                        rng.range_f32(1.2, 2.4),
                        rng.range_usize(1, 2),
                    ),
                    UniverseType::Faith => (
                        rng.range_f32(60.0, 88.0),
                        rng.range_f32(15.0, 45.0),
                        rng.range_f32(1.8, 2.8),
                        rng.range_f32(0.7, 1.5),
                        rng.range_usize(1, 2),
                    ),
                    UniverseType::Tech => (
                        rng.range_f32(65.0, 95.0),
                        rng.range_f32(25.0, 55.0),
                        rng.range_f32(1.0, 1.7),
                        rng.range_f32(0.8, 1.8),
                        rng.range_usize(1, 2),
                    ),
                    UniverseType::Primordial => (
                        rng.range_f32(50.0, 80.0),
                        rng.range_f32(20.0, 50.0),
                        rng.range_f32(0.8, 1.5),
                        rng.range_f32(1.8, 3.0),
                        if rng.chance(0.25) { 1 } else { 0 },
                    ),
                    UniverseType::Collapsing => (
                        rng.range_f32(15.0, 38.0),
                        rng.range_f32(72.0, 96.0),
                        rng.range_f32(1.1, 2.1),
                        rng.range_f32(1.5, 2.8),
                        if rng.chance(0.55) { 1 } else { 0 },
                    ),
                };

            let mut civilizations = Vec::new();
            for c_i in 0..civ_count {
                let civ_id = *next_civ_id;
                *next_civ_id += 1;

                let tech_base = match u_type {
                    UniverseType::Tech => rng.range_f32(55.0, 85.0),
                    UniverseType::Primordial => rng.range_f32(5.0, 20.0),
                    _ => rng.range_f32(18.0, 58.0),
                };
                let faith_base = match u_type {
                    UniverseType::Faith => rng.range_f32(65.0, 95.0),
                    UniverseType::Tech => rng.range_f32(20.0, 55.0),
                    _ => rng.range_f32(35.0, 75.0),
                };

                let doctrine = CivilizationDoctrine::from_seed(i as u32, civ_id);
                civilizations.push(Civilization {
                    id: civ_id,
                    name: format!(
                        "{} Uygarlığı {}",
                        prefixes[(p_idx + c_i) % prefixes.len()],
                        c_i + 1
                    ),
                    universe_id: i as u32,
                    doctrine,
                    population: rng.range_u32(90, 650) as u64,
                    tech_level: tech_base,
                    faith_fervor: faith_base,
                    stability: stability.clamp(25.0, 90.0),
                    militarism: if u_type == UniverseType::Chaos {
                        rng.range_f32(55.0, 90.0)
                    } else {
                        rng.range_f32(15.0, 60.0)
                    },
                    deity_awareness: if tech_base > 60.0 {
                        rng.range_f32(15.0, 45.0)
                    } else {
                        rng.range_f32(0.0, 15.0)
                    },
                    patron_deity: None,
                    religion_name: format!("{} İnanışı", doctrine.name_tr()),
                    in_rebellion: false,
                    divine_shield_active: false,
                });
            }

            // Generate rich initial lore/history per GDD section 3 ("Her evrenin kendine ait tarihi olacak")
            let mut u = Universe {
                id: i as u32,
                name,
                universe_type: u_type,
                position: [x, y],
                region_tier,
                // Initially only Core ring (tier 0) is discovered by the player!
                discovered_by_player: region_tier == 0,
                stability,
                danger_level,
                soul_potential,
                essence_richness,
                soul_reservoir: rng.range_u32(40, 220) as i64,
                deity_influence: BTreeMap::new(),
                civilizations,
                gateways: Vec::new(),
                history: Vec::new(),
                shaped_by_deity: None,
                is_destroyed: false,
            };

            let origin_lore = match u_type {
                UniverseType::Life => "Kadim çağlarda bereketli yıldız tozuyla yoğruldu; çok sayıda ölümlü uygarlığa beşiklik ediyor.",
                UniverseType::Chaos => "İki rakip ilahın kadim savaşı gerçeklik dokusunu parçaladı; savaşlar hiç dinmiyor.",
                UniverseType::Faith => "Gökyüzündeki altın mühürler ölümlüleri yüzyıllardır kehanetlere ve tapınaklara yönlendiriyor.",
                UniverseType::Tech => "Ölümlü bilginler doğaüstü olayları analiz eden sismik ve astral gözlemevleri inşa etti.",
                UniverseType::Primordial => "Henüz genç ve biçimlenmemiş bir gerçeklik; kozmik öz nehirleri ilahi bir mimar bekliyor.",
                UniverseType::Collapsing => "Eski koruyucu tanrısı tarafından terk edildi; gerçeklik halkaları çatlıyor ve ruhlar boşluğa sızıyor.",
            };
            u.record_history(0, HistoryCategory::AncientOrigin, None, origin_lore);

            let arch_lore = match i % 5 {
                0 => "[Kadim Çağ] Yörüngede unutulmuş bir yıldız tapınağının kalıntıları hâlâ zayıf ilahi yankılar yayıyor.",
                1 => "[Kadim Çağ] Ölümlüler ilk yazıtlarında gökyüzünü bölen devasa bir kozmik geçitten söz ediyor.",
                2 => "[Kadim Çağ] Gezegen çekirdeklerinde kristalize olmuş saf kozmik öz damarları keşfedildi.",
                3 => "[Kadim Çağ] Yok olmuş öncül bir uygarlığın bıraktığı dikilitaşlar tanrıların dönüşünü müjdeliyor.",
                _ => "[Kadim Çağ] Boşlukta süzülen dev usturlap halkaları, evrenin bir zamanlar büyük bir mimar tarafından ölçüldüğünü gösteriyor.",
            };
            u.record_history(0, HistoryCategory::AncientOrigin, None, arch_lore);

            if !deities.is_empty() && rng.chance(0.45) {
                let past_deity = &deities[rng.range_usize(1, deities.len())];
                u.record_history(
                    0,
                    HistoryCategory::DivineIntervention,
                    Some(past_deity.id),
                    format!(
                        "Geçmiş çağda {} ({}) bu evrende iz bıraktı.",
                        past_deity.name, past_deity.title
                    ),
                );
            }

            universes.push(u);
        }

        // Build cosmic gateway connections based on spatial proximity so the graph is connected
        let n = universes.len();
        for i in 0..n {
            // Connect to 2..=3 nearest neighbors that aren't already connected
            let mut dists: Vec<(usize, f32)> = (0..n)
                .filter(|&j| j != i)
                .map(|j| {
                    let dx = universes[i].position[0] - universes[j].position[0];
                    let dy = universes[i].position[1] - universes[j].position[1];
                    (j, (dx * dx + dy * dy).sqrt())
                })
                .collect();
            dists.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

            let target_links = 3;
            for &(neighbor, _) in dists.iter().take(target_links) {
                let is_core = universes[i].region_tier == 0 && universes[neighbor].region_tier <= 1;
                let state = if is_core || rng.chance(0.55) {
                    GatewayState::Open
                } else {
                    GatewayState::Closed
                };
                Self::ensure_bidirectional_gateway(
                    &mut universes,
                    i as u32,
                    neighbor as u32,
                    state,
                );
            }
        }

        universes
    }

    fn ensure_bidirectional_gateway(
        universes: &mut [Universe],
        a: u32,
        b: u32,
        state: GatewayState,
    ) {
        let ua = &mut universes[a as usize];
        if !ua.gateways.iter().any(|g| g.target_universe_id == b) {
            ua.gateways.push(GatewayConnection {
                target_universe_id: b,
                state,
            });
        }
        let ub = &mut universes[b as usize];
        if !ub.gateways.iter().any(|g| g.target_universe_id == a) {
            ub.gateways.push(GatewayConnection {
                target_universe_id: a,
                state,
            });
        }
    }

    fn assign_starting_positions(
        universes: &mut [Universe],
        deities: &mut [Deity],
        rng: &mut DeterministicRng,
    ) {
        if universes.is_empty() || deities.is_empty() {
            return;
        }

        // Player starts with strong influence in Universe 0 and mild influence in Universe 1
        universes[0].discovered_by_player = true;
        universes[0].add_influence(PLAYER_DEITY_ID, 65.0);
        for civ in &mut universes[0].civilizations {
            civ.patron_deity = Some(PLAYER_DEITY_ID);
            civ.religion_name = format!("{} Yolu", deities[0].name);
        }
        universes[0].record_history(
            0,
            HistoryCategory::DivineIntervention,
            Some(PLAYER_DEITY_ID),
            format!(
                "{} bu evreni ilk kozmik tahtı olarak seçti.",
                deities[0].name
            ),
        );

        if universes.len() > 1 {
            universes[1].discovered_by_player = true;
            universes[1].add_influence(PLAYER_DEITY_ID, 30.0);
        }

        // Assign each rival deity 2 starting universes spread across the pool
        let u_len = universes.len();
        for d_idx in 1..deities.len() {
            let deity_id = deities[d_idx].id;
            let deity_name = deities[d_idx].name.clone();
            let primary_u =
                (d_idx * (u_len / deities.len().max(2))).clamp(2, u_len.saturating_sub(1));
            let secondary_u = (primary_u + rng.range_usize(2, 6)) % u_len;

            for &u_idx in &[primary_u, secondary_u] {
                if u_idx == 0 {
                    continue;
                }
                universes[u_idx].add_influence(deity_id, 62.0);
                for civ in &mut universes[u_idx].civilizations {
                    civ.patron_deity = Some(deity_id);
                    civ.religion_name = format!("{deity_name} Kültü");
                }
                universes[u_idx].record_history(
                    0,
                    HistoryCategory::DivineIntervention,
                    Some(deity_id),
                    format!("{deity_name} bu evrende erken hâkimiyet kurdu."),
                );
            }
        }
    }

    pub fn player_deity(&self) -> &Deity {
        &self.deities[PLAYER_DEITY_ID as usize]
    }

    pub fn player_deity_mut(&mut self) -> &mut Deity {
        &mut self.deities[PLAYER_DEITY_ID as usize]
    }

    pub fn dominated_universe_count(&self, deity_id: u32) -> usize {
        self.universes
            .iter()
            .filter(|u| !u.is_destroyed && u.dominant_deity().map(|(id, _)| id) == Some(deity_id))
            .count()
    }

    pub fn full_allegiance_universe_count(&self, deity_id: u32) -> usize {
        self.universes
            .iter()
            .filter(|u| !u.is_destroyed && u.has_full_allegiance(deity_id))
            .count()
    }

    /// Computes `(dominated_counts, full_allegiance_counts)` for all deities in a single `O(U)` pass.
    pub fn all_deity_universe_counts(&self) -> (Vec<usize>, Vec<usize>) {
        let len = self.deities.len();
        let mut dom = vec![0usize; len];
        let mut full = vec![0usize; len];
        for u in &self.universes {
            if u.is_destroyed {
                continue;
            }
            if let Some((d_id, inf)) = u.dominant_deity() {
                let idx = d_id as usize;
                if idx < len {
                    dom[idx] += 1;
                    if inf >= 65.0 {
                        full[idx] += 1;
                    }
                }
            }
        }
        (dom, full)
    }

    /// Returns the exact per-turn economic breakdown of `universe_id` for its dominant deity.
    pub fn universe_economy_breakdown(
        &self,
        universe_id: u32,
    ) -> Option<(u32, UniverseEconomyBreakdown)> {
        let u = self.universes.get(universe_id as usize)?;
        let (dom_id, _) = u.dominant_deity()?;
        let d = self.deities.get(dom_id as usize)?;
        if d.is_sealed {
            return None;
        }
        let is_focused = dom_id == PLAYER_DEITY_ID && self.focused_universe_id == Some(u.id);
        let (_, _, breakdown) = u.estimate_economy_breakdown(self.tick, is_focused, d)?;
        Some((dom_id, breakdown))
    }

    pub fn sealed_rival_count(&self) -> usize {
        self.deities
            .iter()
            .filter(|d| d.id != PLAYER_DEITY_ID && d.is_sealed)
            .count()
    }

    pub fn has_full_creation_tree(&self, deity_id: u32) -> bool {
        self.deities.get(deity_id as usize).is_some_and(|d| {
            d.has_power(PowerId::GenesisSpark)
                && d.has_power(PowerId::RealityAnchor)
                && d.has_power(PowerId::CosmicForge)
        })
    }

    pub fn harmonious_shaped_universe_count(&self, deity_id: u32) -> usize {
        self.universes
            .iter()
            .filter(|u| {
                !u.is_destroyed
                    && u.shaped_by_deity == Some(deity_id)
                    && u.stability >= CREATION_REQUIRED_STABILITY
                    && u.has_full_allegiance(deity_id)
                    && u.civilizations.iter().any(|c| !c.in_rebellion)
            })
            .count()
    }

    /// Returns `(dominion_ratio, transcendence_ratio, creation_ratio)` in `0.0..=1.0` for UI progress bars.
    pub fn victory_progress_ratios(&self, deity_id: u32) -> (f32, f32, f32) {
        let dom_u = self.dominated_universe_count(deity_id) as f32;
        let full_u = self.full_allegiance_universe_count(deity_id) as f32;
        let sealed = self.sealed_rival_count() as f32;
        let dom_ratio = ((dom_u / DOMINION_REQUIRED_DOMINATED as f32).min(1.0)
            + (full_u / DOMINION_REQUIRED_FULL_ALLEGIANCE as f32).min(1.0)
            + (sealed / DOMINION_REQUIRED_SEALED_RIVALS as f32).min(1.0))
            / 3.0;

        let deity = &self.deities[deity_id as usize];
        let powers = deity.unlocked_powers.len() as f32;
        let trans_ratio = ((deity.ascension_progress / 100.0).min(1.0)
            + (powers / TRANSCENDENCE_REQUIRED_POWERS as f32).min(1.0)
            + (full_u / TRANSCENDENCE_REQUIRED_FULL_ALLEGIANCE as f32).min(1.0))
            / 3.0;

        let shaped = self.harmonious_shaped_universe_count(deity_id) as f32;
        let creation_tree_count = [
            PowerId::GenesisSpark,
            PowerId::RealityAnchor,
            PowerId::CosmicForge,
        ]
        .into_iter()
        .filter(|p| deity.has_power(*p))
        .count() as f32;
        let creation_ratio = ((shaped / CREATION_REQUIRED_SHAPED_UNIVERSES as f32).min(1.0) * 0.7
            + (creation_tree_count / 3.0) * 0.3)
            .clamp(0.0, 1.0);

        (dom_ratio, trans_ratio, creation_ratio)
    }

    /// Switches an ended Scenario game into Endless Sandbox Mode so the player can keep playing.
    pub fn continue_in_sandbox_mode(&mut self) {
        self.config.game_mode = GameMode::SandboxEndlessPool;
        self.outcome = GameOutcome::Ongoing;
        self.push_event(
            None,
            Some(PLAYER_DEITY_ID),
            CosmicEventKind::AscensionMilestone,
            EventSeverity::Positive,
            "Sonsuz Havuz Moduna Geçildi",
            "Kozmik tarih bitiş koşulu olmadan sonsuz sandbox modunda akmaya devam ediyor.",
        );
    }

    /// Updates the player's divine name in config, deity record, and patronized civilizations.
    pub fn rename_player_deity(&mut self, new_name: impl Into<String>) {
        let trimmed = new_name.into().trim().to_string();
        if trimmed.is_empty() {
            return;
        }
        self.config.player_name = trimmed.clone();
        if let Some(player) = self.deities.get_mut(PLAYER_DEITY_ID as usize) {
            player.name = trimmed.clone();
        }
        for u in &mut self.universes {
            for civ in &mut u.civilizations {
                if civ.patron_deity == Some(PLAYER_DEITY_ID) {
                    civ.religion_name = format!("{trimmed} Yolu");
                }
            }
        }
    }

    pub fn push_event(
        &mut self,
        universe_id: Option<u32>,
        deity_id: Option<u32>,
        kind: CosmicEventKind,
        severity: EventSeverity,
        title: impl Into<String>,
        description: impl Into<String>,
    ) {
        let id = self.next_event_id;
        self.next_event_id += 1;
        self.events.push(CosmicEvent {
            id,
            tick: self.tick,
            universe_id,
            deity_id,
            kind,
            severity,
            title: title.into(),
            description: description.into(),
        });
        if self.events.len() > 160 {
            let overflow = self.events.len() - 160;
            self.events.drain(0..overflow);
        }
    }

    /// Executes a divine action on behalf of `deity_id` (Player or AI).
    /// Returns `Ok(String)` with a human-readable outcome summary or `Err(String)` if invalid/unaffordable.
    pub fn execute_action(
        &mut self,
        deity_id: u32,
        action: DivineAction,
    ) -> Result<String, String> {
        let d_idx = deity_id as usize;
        if d_idx >= self.deities.len() {
            return Err("Geçersiz tanrı kimliği.".to_string());
        }
        if self.deities[d_idx].is_sealed {
            return Err("Mühürlenmiş bir tanrı eylemde bulunamaz.".to_string());
        }

        let cost = action.base_cost(&self.deities[d_idx]);
        if !self.deities[d_idx].resources.can_afford(&cost) {
            return Err(format!(
                "Yetersiz kaynak! Gereken: {} Ruh, {} Nüfuz, {} Öz",
                cost.souls, cost.influence, cost.cosmic_essence
            ));
        }

        let deity_name = self.deities[d_idx].name.clone();
        let has_prophecy_power = self.deities[d_idx].has_power(PowerId::VoiceOfProphecy);
        let has_miracle_power = self.deities[d_idx].has_power(PowerId::GoldenMiracle);
        let has_siphon_power = self.deities[d_idx].has_power(PowerId::SoulSiphon);
        let has_condenser_power = self.deities[d_idx].has_power(PowerId::EssenceCondenser);
        let has_reaper_power = self.deities[d_idx].has_power(PowerId::ReapersToll);
        let has_genesis_power = self.deities[d_idx].has_power(PowerId::GenesisSpark);
        let has_forge_power = self.deities[d_idx].has_power(PowerId::CosmicForge);
        let has_smite_power = self.deities[d_idx].has_power(PowerId::DivineSmite);
        let has_schism_power = self.deities[d_idx].has_power(PowerId::ShadowSchism);
        let has_seal_power = self.deities[d_idx].has_power(PowerId::EternalSeal);
        let has_astral_power = self.deities[d_idx].has_power(PowerId::AstralSight);
        let has_gateway_power = self.deities[d_idx].has_power(PowerId::GatewayMastery);
        let action_for_tutorial = action.clone();

        let action_result = match action {
            DivineAction::SpreadProphecy { universe_id } => {
                let u_idx = self.validate_universe(universe_id, deity_id == PLAYER_DEITY_ID)?;
                self.spend_for_action(
                    deity_id,
                    ResourceExpenditure::DivineAction,
                    cost,
                    Some(universe_id),
                    "Kehanet yayılımı",
                );

                let inf_gain = if has_prophecy_power { 28.0 } else { 18.0 };
                let u = &mut self.universes[u_idx];
                u.add_influence(deity_id, inf_gain);
                let high_influence = u.influence_of(deity_id) >= 35.0;
                for civ in &mut u.civilizations {
                    let shield_mult = if civ.divine_shield_active { 0.45 } else { 1.0 };
                    civ.faith_fervor = (civ.faith_fervor + 14.0 * shield_mult).clamp(0.0, 100.0);
                    if high_influence {
                        civ.patron_deity = Some(deity_id);
                        civ.religion_name = format!("{deity_name} Kehaneti");
                    }
                }
                let u_name = u.name.clone();
                u.record_history(
                    self.tick,
                    HistoryCategory::DivineIntervention,
                    Some(deity_id),
                    format!("{deity_name} ölümlülere kehanetler göndererek nüfuzunu artırdı."),
                );
                let msg =
                    format!("{u_name} evreninde kehanet yayıldı (+{inf_gain:.0} Nüfuz Etkisi).");
                if deity_id == PLAYER_DEITY_ID {
                    self.push_event(
                        Some(universe_id),
                        Some(deity_id),
                        CosmicEventKind::ProphecyFulfilled,
                        EventSeverity::Positive,
                        "Kehanet Yayıldı",
                        &msg,
                    );
                }
                Ok(msg)
            }

            DivineAction::HarvestSouls { universe_id } => {
                let u_idx = self.validate_dominated_universe(universe_id, deity_id)?;
                if self.universes[u_idx].soul_reservoir < 15
                    && self.universes[u_idx].total_population() < 30
                {
                    return Err(
                        "Bu evrenin ruh havuzu tükenmiş durumda (En az 15 Ruh Havuzu veya 30M ölümlü nüfus gerekir)."
                            .to_string(),
                    );
                }
                self.spend_for_action(
                    deity_id,
                    ResourceExpenditure::DivineAction,
                    cost,
                    Some(universe_id),
                    "Ruh hasadı",
                );

                let u = &mut self.universes[u_idx];
                let reservoir_take = if u.soul_reservoir > 0 {
                    ((u.soul_reservoir * 65) / 100)
                        .max(10)
                        .min(u.soul_reservoir)
                } else {
                    0
                };
                u.soul_reservoir = (u.soul_reservoir - reservoir_take).max(0);
                let pop_total = u.total_population();
                let pop_take = if pop_total > 0 {
                    ((pop_total / 10) as i64) + 20
                } else {
                    0
                };
                for civ in &mut u.civilizations {
                    civ.population = civ.population.saturating_sub((civ.population / 16).max(2));
                    civ.stability = (civ.stability - 8.0).max(5.0);
                    civ.faith_fervor = (civ.faith_fervor - 6.0).max(5.0);
                }
                u.stability = (u.stability - 6.0).max(5.0);
                u.danger_level = (u.danger_level + 7.0).min(100.0);

                let mult = if has_siphon_power { 1.40 } else { 1.0 };
                let harvested_souls =
                    (((reservoir_take + pop_take) as f32) * u.soul_potential * mult)
                        .round()
                        .max(15.0) as i64;
                let bonus_essence = if has_reaper_power { 12 } else { 0 };
                let u_name = u.name.clone();
                u.record_history(
                    self.tick,
                    HistoryCategory::DivineIntervention,
                    Some(deity_id),
                    format!("{deity_name} bu evrende büyük bir ruh hasadı gerçekleştirdi ({harvested_souls} Ruh)."),
                );

                self.ledger.record_source(
                    self.tick,
                    &mut self.deities[d_idx].resources,
                    deity_id,
                    ResourceSource::DivineHarvest,
                    Resources::new(harvested_souls, 0, bonus_essence),
                    Some(universe_id),
                    format!("{u_name} ruh hasadı"),
                );

                let msg = format!(
                    "{u_name} evreninden {harvested_souls} Ruh hasat edildi{} (Kalan Havuz: {}).",
                    if bonus_essence > 0 {
                        format!(" (+{bonus_essence} Kozmik Öz)")
                    } else {
                        String::new()
                    },
                    self.universes[u_idx].soul_reservoir
                );
                if deity_id == PLAYER_DEITY_ID {
                    self.push_event(
                        Some(universe_id),
                        Some(deity_id),
                        CosmicEventKind::DivineIntervention,
                        EventSeverity::Info,
                        "Ruh Hasadı Tamamlandı",
                        &msg,
                    );
                }
                Ok(msg)
            }

            DivineAction::BestowMiracle { universe_id } => {
                let u_idx = self.validate_dominated_universe(universe_id, deity_id)?;
                self.spend_for_action(
                    deity_id,
                    ResourceExpenditure::DivineAction,
                    cost,
                    Some(universe_id),
                    "İlahi mucize",
                );

                let mult = if has_miracle_power { 1.85 } else { 1.0 };
                let u = &mut self.universes[u_idx];
                u.stability = (u.stability + 15.0 * mult).min(100.0);
                u.danger_level = (u.danger_level - 12.0 * mult).max(0.0);
                u.soul_potential = (u.soul_potential + 0.08 * mult).min(3.0);
                u.soul_reservoir += (25.0 * mult) as i64;
                u.add_influence(deity_id, 14.0 * mult);

                for civ in &mut u.civilizations {
                    civ.population = (civ.population + (45.0 * mult) as u64).min(50_000);
                    civ.stability = (civ.stability + 18.0 * mult).min(100.0);
                    civ.faith_fervor = (civ.faith_fervor + 12.0 * mult).min(100.0);
                    civ.in_rebellion = false;
                    civ.patron_deity = Some(deity_id);
                    civ.religion_name = format!("{deity_name} Kültü");
                    if has_miracle_power {
                        civ.divine_shield_active = false;
                    }
                }
                let u_name = u.name.clone();
                u.record_history(
                    self.tick,
                    HistoryCategory::DivineIntervention,
                    Some(deity_id),
                    format!("{deity_name} altın bir mucize bahşederek evreni kutsadı."),
                );
                Ok(format!(
                    "{u_name} evreninde mucize gerçekleşti; istikrar, nüfus ve ruh akışı yükseldi."
                ))
            }

            DivineAction::SeedLifeAndOrder { universe_id } => {
                let u_idx = self.validate_dominated_universe(universe_id, deity_id)?;
                self.spend_for_action(
                    deity_id,
                    ResourceExpenditure::UniverseCreation,
                    cost,
                    Some(universe_id),
                    "Yaşam ve düzen tohumlama",
                );

                let civ_id = self.next_civ_id;
                self.next_civ_id += 1;
                let bonus = if has_genesis_power { 1.35 } else { 1.0 };
                let max_civs = if has_forge_power { 6 } else { 4 };

                let u = &mut self.universes[u_idx];
                if u.universe_type == UniverseType::Primordial {
                    u.universe_type = UniverseType::Life;
                }
                u.stability = (u.stability + 22.0 * bonus).min(100.0);
                u.soul_potential = (u.soul_potential + 0.35 * bonus).min(3.0);
                u.add_influence(deity_id, 32.0 * bonus);
                let newly_shaped = u.shaped_by_deity != Some(deity_id);
                u.shaped_by_deity = Some(deity_id);

                if u.civilizations.len() < max_civs {
                    u.civilizations.push(Civilization {
                        id: civ_id,
                        name: format!("{deity_name} Soyu #{civ_id}"),
                        universe_id,
                        doctrine: CivilizationDoctrine::from_seed(universe_id, civ_id),
                        population: (180.0 * bonus) as u64,
                        tech_level: 20.0,
                        faith_fervor: 82.0,
                        stability: 85.0,
                        militarism: 20.0,
                        deity_awareness: 10.0,
                        patron_deity: Some(deity_id),
                        religion_name: format!("{deity_name} Düzeni"),
                        in_rebellion: false,
                        divine_shield_active: false,
                    });
                }

                let u_name = u.name.clone();
                u.record_history(
                    self.tick,
                    HistoryCategory::CivilizationRiseFall,
                    Some(deity_id),
                    format!("{deity_name} bu evrende yeni yaşam ve kozmik düzen tohumladı."),
                );
                if newly_shaped {
                    self.deities[d_idx].shaped_universes_count += 1;
                }

                let msg = format!("{u_name} evreninde yeni yaşam ve kozmik düzen kuruldu!");
                if deity_id == PLAYER_DEITY_ID {
                    self.push_event(
                        Some(universe_id),
                        Some(deity_id),
                        CosmicEventKind::NewCivilizationBorn,
                        EventSeverity::Positive,
                        "Yaşam Tohumlandı",
                        &msg,
                    );
                }
                Ok(msg)
            }

            DivineAction::StabilizeReality { universe_id } => {
                let u_idx = self.validate_dominated_universe(universe_id, deity_id)?;
                self.spend_for_action(
                    deity_id,
                    ResourceExpenditure::UniverseCreation,
                    cost,
                    Some(universe_id),
                    "Gerçeklik onarımı",
                );

                let u = &mut self.universes[u_idx];
                let was_collapsing = u.universe_type == UniverseType::Collapsing;
                if was_collapsing {
                    u.universe_type = UniverseType::Faith;
                }
                u.stability = (u.stability + 30.0).min(100.0);
                u.danger_level = (u.danger_level - 25.0).max(5.0);
                u.essence_richness = (u.essence_richness + 0.25).min(3.0);
                u.add_influence(deity_id, 25.0);
                let newly_shaped = u.shaped_by_deity != Some(deity_id);
                u.shaped_by_deity = Some(deity_id);

                let u_name = u.name.clone();
                u.record_history(
                    self.tick,
                    HistoryCategory::DivineIntervention,
                    Some(deity_id),
                    format!("{deity_name} kırılan gerçeklik halkalarını onararak çöküşü durdurdu."),
                );
                if newly_shaped {
                    self.deities[d_idx].shaped_universes_count += 1;
                }
                Ok(format!(
                    "{u_name} evreninin gerçeklik dokusu ve öz damarları onarıldı (+30 İstikrar, +0.25 Öz Zenginliği)."
                ))
            }

            DivineAction::ExtractCosmicEssence { universe_id } => {
                let u_idx = self.validate_dominated_universe(universe_id, deity_id)?;
                if self.universes[u_idx].essence_richness < 0.45 {
                    return Err(
                        "Bu evrenin kozmik öz damarları geçici olarak yorgun (Min 0.45 Öz Zenginliği gerekir; zamanla veya Gerçeklik Onarımı ile yenilenir)."
                            .to_string(),
                    );
                }
                self.spend_for_action(
                    deity_id,
                    ResourceExpenditure::DivineAction,
                    cost,
                    Some(universe_id),
                    "Kozmik öz çıkarımı",
                );

                let u = &mut self.universes[u_idx];
                let type_bonus = match u.universe_type {
                    UniverseType::Primordial | UniverseType::Collapsing | UniverseType::Chaos => {
                        1.5
                    }
                    _ => 1.0,
                };
                let condenser_mult = if has_condenser_power { 1.25 } else { 1.0 };
                let extracted = ((24.0 * u.essence_richness * type_bonus * condenser_mult).round()
                    as i64)
                    .max(10);
                let richness_drain = if has_condenser_power { 0.10 } else { 0.22 };
                u.essence_richness = (u.essence_richness - richness_drain).max(0.35);
                u.stability = (u.stability - 8.0).max(5.0);
                let new_richness = u.essence_richness;
                let u_name = u.name.clone();
                u.record_history(
                    self.tick,
                    HistoryCategory::DivineIntervention,
                    Some(deity_id),
                    format!("{deity_name} evrenin çekirdeğinden {extracted} Kozmik Öz damıttı."),
                );

                self.ledger.record_source(
                    self.tick,
                    &mut self.deities[d_idx].resources,
                    deity_id,
                    ResourceSource::DivineHarvest,
                    Resources::new(0, 0, extracted),
                    Some(universe_id),
                    format!("{u_name} kozmik öz çıkarımı"),
                );

                Ok(format!(
                    "{u_name} evreninden +{extracted} Kozmik Öz elde edildi (Kalan Öz Zenginliği: {new_richness:.2}x)."
                ))
            }

            DivineAction::SmiteRivalInfluence {
                universe_id,
                target_deity_id,
            } => {
                let u_idx = self.validate_universe(universe_id, deity_id == PLAYER_DEITY_ID)?;
                if target_deity_id == deity_id || target_deity_id as usize >= self.deities.len() {
                    return Err("Geçersiz hedef tanrı.".to_string());
                }
                self.spend_for_action(
                    deity_id,
                    ResourceExpenditure::DivineAction,
                    cost,
                    Some(universe_id),
                    "İlahi darbe",
                );

                let dmg = if has_smite_power { 42.0 } else { 24.0 };
                let target_name = self.deities[target_deity_id as usize].name.clone();
                let u = &mut self.universes[u_idx];
                u.add_influence(target_deity_id, -dmg);
                u.add_influence(deity_id, 12.0);
                u.danger_level = (u.danger_level + 8.0).min(100.0);
                let u_name = u.name.clone();
                u.record_history(
                    self.tick,
                    HistoryCategory::ReligiousConflict,
                    Some(deity_id),
                    format!("{deity_name}, {target_name} tapınaklarını ilahi gazapla vurdu."),
                );

                if has_smite_power {
                    self.ledger.record_source(
                        self.tick,
                        &mut self.deities[d_idx].resources,
                        deity_id,
                        ResourceSource::DivineHarvest,
                        Resources::new(30, 0, 0),
                        Some(universe_id),
                        "İlahi gazap ruh ganimeti",
                    );
                }

                self.deities[target_deity_id as usize].adjust_relation(deity_id, -25);
                self.deities[d_idx].adjust_relation(target_deity_id, -15);

                Ok(format!(
                    "{u_name} evreninde {target_name} nüfuzu kırıldı (-{dmg:.0}){}.",
                    if has_smite_power {
                        " (+30 Ruh Ganimeti)"
                    } else {
                        ""
                    }
                ))
            }

            DivineAction::InciteRebellion { universe_id } => {
                let u_idx = self.validate_universe(universe_id, deity_id == PLAYER_DEITY_ID)?;
                self.spend_for_action(
                    deity_id,
                    ResourceExpenditure::DivineAction,
                    cost,
                    Some(universe_id),
                    "İsyan kışkırtması",
                );

                let rival_loss = if has_schism_power { -28.0 } else { -18.0 };
                let own_gain = if has_schism_power { 16.0 } else { 10.0 };
                let u = &mut self.universes[u_idx];
                let rival = u.dominant_deity().map(|(id, _)| id);
                for civ in &mut u.civilizations {
                    if civ.patron_deity != Some(deity_id) {
                        civ.in_rebellion = true;
                        civ.stability = (civ.stability - 20.0).max(5.0);
                    }
                }
                if let Some(rival_id) = rival {
                    if rival_id != deity_id {
                        u.add_influence(rival_id, rival_loss);
                    }
                }
                u.add_influence(deity_id, own_gain);
                let u_name = u.name.clone();
                u.record_history(
                    self.tick,
                    HistoryCategory::ReligiousConflict,
                    Some(deity_id),
                    format!(
                        "{deity_name} fısıltılarıyla evrende mezhep ayrılığı ve isyan başlattı."
                    ),
                );
                Ok(format!(
                    "{u_name} evrenindeki rakip uygarlıklarda isyan çıktı! ({rival_loss:.0} Rakip, +{own_gain:.0} Nüfuz)"
                ))
            }

            DivineAction::OpenGateway {
                from_universe_id,
                to_universe_id,
            } => {
                let from_idx =
                    self.validate_universe(from_universe_id, deity_id == PLAYER_DEITY_ID)?;
                if to_universe_id as usize >= self.universes.len() {
                    return Err("Hedef evren bulunamadı.".to_string());
                }
                self.spend_for_action(
                    deity_id,
                    ResourceExpenditure::GatewayOperation,
                    cost,
                    Some(from_universe_id),
                    "Geçit açma",
                );

                for g in &mut self.universes[from_idx].gateways {
                    if g.target_universe_id == to_universe_id {
                        g.state = GatewayState::Open;
                    }
                }
                for g in &mut self.universes[to_universe_id as usize].gateways {
                    if g.target_universe_id == from_universe_id {
                        g.state = GatewayState::Open;
                    }
                }
                if deity_id == PLAYER_DEITY_ID {
                    self.universes[to_universe_id as usize].discovered_by_player = true;
                    // If AstralSight is unlocked, also reveal neighbors of `to_universe_id`
                    if has_astral_power {
                        let neighbors: Vec<u32> = self.universes[to_universe_id as usize]
                            .gateways
                            .iter()
                            .map(|g| g.target_universe_id)
                            .collect();
                        for n_id in neighbors {
                            self.universes[n_id as usize].discovered_by_player = true;
                        }
                    }
                }

                let to_name = self.universes[to_universe_id as usize].name.clone();
                self.universes[from_idx].record_history(
                    self.tick,
                    HistoryCategory::GatewayChange,
                    Some(deity_id),
                    format!("{deity_name}, {to_name} evrenine giden kozmik geçidi açtı."),
                );

                let msg =
                    format!("{to_name} evrenine giden kozmik geçit açıldı ve bölge keşfedildi.");
                if deity_id == PLAYER_DEITY_ID {
                    self.push_event(
                        Some(to_universe_id),
                        Some(deity_id),
                        CosmicEventKind::GatewayOpened,
                        EventSeverity::Positive,
                        "Kozmik Geçit Açıldı",
                        &msg,
                    );
                }
                Ok(msg)
            }

            DivineAction::CloseGateway {
                from_universe_id,
                to_universe_id,
            } => {
                let from_idx = self.validate_dominated_universe(from_universe_id, deity_id)?;
                if to_universe_id as usize >= self.universes.len() {
                    return Err("Hedef evren bulunamadı.".to_string());
                }
                self.spend_for_action(
                    deity_id,
                    ResourceExpenditure::GatewayOperation,
                    cost,
                    Some(from_universe_id),
                    "Geçit kapatma",
                );

                let new_state = if self.deities[d_idx].has_power(PowerId::DimensionalWard) {
                    GatewayState::Locked {
                        owner_deity: deity_id,
                    }
                } else {
                    GatewayState::Closed
                };

                for g in &mut self.universes[from_idx].gateways {
                    if g.target_universe_id == to_universe_id {
                        g.state = new_state;
                    }
                }
                for g in &mut self.universes[to_universe_id as usize].gateways {
                    if g.target_universe_id == from_universe_id {
                        g.state = new_state;
                    }
                }

                let to_name = self.universes[to_universe_id as usize].name.clone();
                Ok(format!("{to_name} evrenine giden geçit mühürlendi."))
            }

            DivineAction::ScoutRegion { universe_id } => {
                let u_idx = self.validate_dominated_universe(universe_id, deity_id)?;
                self.spend_for_action(
                    deity_id,
                    ResourceExpenditure::GatewayOperation,
                    cost,
                    Some(universe_id),
                    "Kozmik keşif",
                );

                let neighbors: Vec<u32> = self.universes[u_idx]
                    .gateways
                    .iter()
                    .map(|g| g.target_universe_id)
                    .collect();

                let mut newly_discovered = 0usize;
                for &n_id in &neighbors {
                    let nu = &mut self.universes[n_id as usize];
                    if !nu.discovered_by_player {
                        nu.discovered_by_player = true;
                        newly_discovered += 1;
                    }
                    if has_astral_power {
                        let second_hop: Vec<u32> =
                            nu.gateways.iter().map(|g| g.target_universe_id).collect();
                        for s_id in second_hop {
                            if !self.universes[s_id as usize].discovered_by_player {
                                self.universes[s_id as usize].discovered_by_player = true;
                                newly_discovered += 1;
                            }
                        }
                    }
                }
                self.universes[u_idx].add_influence(deity_id, 8.0);
                Ok(format!(
                    "Kozmik keşif tamamlandı: {newly_discovered} yeni evren haritada belirdi."
                ))
            }

            DivineAction::UnlockPower { power } => {
                if !self.deities[d_idx].can_unlock_power(power) {
                    return Err(
                        "Bu ilahi güç zaten açık, ön koşulu eksik veya kaynak yetersiz."
                            .to_string(),
                    );
                }
                self.spend_for_action(
                    deity_id,
                    ResourceExpenditure::PowerUnlock,
                    cost,
                    None,
                    format!("Güç açıldı: {}", power.name_tr()),
                );
                self.deities[d_idx].unlocked_powers.insert(power);
                let burst_summary = self.apply_power_unlock_burst(deity_id, power);
                let msg = format!(
                    "{} ({}) ilahi gücü uyandırıldı! {}",
                    power.name_tr(),
                    power.tree().name_tr(),
                    burst_summary
                );
                if deity_id == PLAYER_DEITY_ID {
                    self.push_event(
                        None,
                        Some(deity_id),
                        CosmicEventKind::AscensionMilestone,
                        EventSeverity::Positive,
                        format!("Yeni İlahi Güç: {}", power.name_tr()),
                        &msg,
                    );
                }
                Ok(msg)
            }

            DivineAction::OfferDiplomaticPact { target_deity_id } => {
                if target_deity_id == deity_id || target_deity_id as usize >= self.deities.len() {
                    return Err("Geçersiz hedef tanrı.".to_string());
                }
                self.spend_for_action(
                    deity_id,
                    ResourceExpenditure::DiplomacyPact,
                    cost,
                    None,
                    "Diplomatik pakt teklifi",
                );

                let target_idx = target_deity_id as usize;
                self.deities[target_idx].adjust_relation(deity_id, 30);
                self.deities[d_idx].adjust_relation(target_deity_id, 30);
                self.deities[target_idx].stances.insert(
                    deity_id,
                    DiplomaticStance::Pact {
                        ticks_remaining: 25,
                    },
                );
                self.deities[d_idx].stances.insert(
                    target_deity_id,
                    DiplomaticStance::Pact {
                        ticks_remaining: 25,
                    },
                );

                let target_name = self.deities[target_idx].name.clone();
                Ok(format!(
                    "{target_name} ile 25 tur sürecek Kozmik Ateşkes Paktı imzalandı."
                ))
            }

            DivineAction::SendDivineGift { target_deity_id } => {
                if target_deity_id == deity_id || target_deity_id as usize >= self.deities.len() {
                    return Err("Geçersiz hedef tanrı.".to_string());
                }
                let target_idx = target_deity_id as usize;
                if self.deities[target_idx].is_sealed {
                    return Err("Bu tanrı zaten mühürlenmiş.".to_string());
                }

                let target_name = self.deities[target_idx].name.clone();
                let gift_amount = Resources::new(70, 0, 5);
                let (actor_res, target_res) = if d_idx < target_idx {
                    let (left, right) = self.deities.split_at_mut(target_idx);
                    (&mut left[d_idx].resources, &mut right[0].resources)
                } else {
                    let (left, right) = self.deities.split_at_mut(d_idx);
                    (&mut right[0].resources, &mut left[target_idx].resources)
                };
                self.ledger.record_transfer(
                    self.tick,
                    actor_res,
                    deity_id,
                    target_res,
                    target_deity_id,
                    gift_amount,
                    format!("{target_name} ruh armağanı"),
                );

                // Also gain +12 diplomatic influence prestige for generosity
                self.ledger.record_source(
                    self.tick,
                    &mut self.deities[d_idx].resources,
                    deity_id,
                    ResourceSource::UniversePassive,
                    Resources::new(0, 12, 0),
                    None,
                    format!("{target_name} diplomatik saygınlık getirisi"),
                );

                self.deities[target_idx].adjust_relation(deity_id, 28);
                self.deities[d_idx].adjust_relation(target_deity_id, 18);
                let new_rel = self.deities[target_idx].relation_toward(deity_id);
                if new_rel >= 15
                    && !matches!(
                        self.deities[target_idx].stance_toward(deity_id),
                        DiplomaticStance::Pact { .. }
                    )
                {
                    self.deities[target_idx]
                        .stances
                        .insert(deity_id, DiplomaticStance::Neutral);
                    self.deities[d_idx]
                        .stances
                        .insert(target_deity_id, DiplomaticStance::Neutral);
                }

                Ok(format!(
                    "{target_name} ilahına ruh armağanı sunuldu (+28 İlişki, +12 Diplomatik Nüfuz)."
                ))
            }

            DivineAction::DeclareHolyWar { target_deity_id } => {
                if target_deity_id == deity_id || target_deity_id as usize >= self.deities.len() {
                    return Err("Geçersiz hedef tanrı.".to_string());
                }
                let target_idx = target_deity_id as usize;
                if self.deities[target_idx].is_sealed {
                    return Err("Bu tanrı zaten mühürlenmiş.".to_string());
                }

                self.spend_for_action(
                    deity_id,
                    ResourceExpenditure::DivineAction,
                    cost,
                    None,
                    "Kutsal rekabet ilanı",
                );

                let target_name = self.deities[target_idx].name.clone();
                self.deities[target_idx].adjust_relation(deity_id, -40);
                self.deities[d_idx].adjust_relation(target_deity_id, -30);
                self.deities[target_idx]
                    .stances
                    .insert(deity_id, DiplomaticStance::Hostile);
                self.deities[d_idx]
                    .stances
                    .insert(target_deity_id, DiplomaticStance::Hostile);

                let mut affected_universes = 0usize;
                for u in &mut self.universes {
                    if !u.is_destroyed && u.influence_of(target_deity_id) > 0.0 {
                        u.add_influence(target_deity_id, -10.0);
                        if u.discovered_by_player || deity_id != PLAYER_DEITY_ID {
                            u.add_influence(deity_id, 6.0);
                        }
                        affected_universes += 1;
                    }
                }

                let msg = format!(
                    "{target_name} ilahına Kutsal Rekabet ilan edildi! {affected_universes} evrende nüfuzu sarsıldı (-10 Rakip Nüfuzu)."
                );
                self.push_event(
                    None,
                    Some(deity_id),
                    CosmicEventKind::DivineIntervention,
                    EventSeverity::Warning,
                    "Kutsal Rekabet İlan Edildi",
                    &msg,
                );
                Ok(msg)
            }

            DivineAction::DemandTribute { target_deity_id } => {
                if target_deity_id == deity_id || target_deity_id as usize >= self.deities.len() {
                    return Err("Geçersiz hedef tanrı.".to_string());
                }
                let target_idx = target_deity_id as usize;
                if self.deities[target_idx].is_sealed {
                    return Err("Bu tanrı zaten mühürlenmiş.".to_string());
                }
                self.spend_for_action(
                    deity_id,
                    ResourceExpenditure::DiplomacyPact,
                    cost,
                    None,
                    "Haraç talebi",
                );

                let my_dom = self.dominated_universe_count(deity_id);
                let their_dom = self.dominated_universe_count(target_deity_id);
                let target_name = self.deities[target_idx].name.clone();

                if my_dom >= their_dom {
                    let tribute = if has_schism_power {
                        Resources::new(125, 42, 14)
                    } else {
                        Resources::new(90, 30, 10)
                    };
                    let (actor_res, target_res) = if d_idx < target_idx {
                        let (left, right) = self.deities.split_at_mut(target_idx);
                        (&mut left[d_idx].resources, &mut right[0].resources)
                    } else {
                        let (left, right) = self.deities.split_at_mut(d_idx);
                        (&mut right[0].resources, &mut left[target_idx].resources)
                    };
                    let transferred = self.ledger.record_transfer(
                        self.tick,
                        target_res,
                        target_deity_id,
                        actor_res,
                        deity_id,
                        tribute,
                        format!("{target_name} haracı"),
                    );
                    self.deities[target_idx].adjust_relation(deity_id, -20);
                    Ok(format!(
                        "{target_name} baskına boyun eğdi: +{} Ruh, +{} Nüfuz, +{} Öz haraç alındı.",
                        transferred.souls, transferred.influence, transferred.cosmic_essence
                    ))
                } else {
                    self.deities[target_idx].adjust_relation(deity_id, -30);
                    Ok(format!(
                        "{target_name} haraç talebini reddetti! (Hâkimiyetin ondan yüksek olmalı)"
                    ))
                }
            }

            DivineAction::SealWeakenedDeity { target_deity_id } => {
                if target_deity_id == deity_id || target_deity_id as usize >= self.deities.len() {
                    return Err("Geçersiz hedef tanrı.".to_string());
                }
                let target_idx = target_deity_id as usize;
                if self.deities[target_idx].is_sealed {
                    return Err("Bu tanrı zaten mühürlenmiş.".to_string());
                }
                let their_dom = self.dominated_universe_count(target_deity_id);
                let max_allowed_dom = if has_seal_power { 2 } else { 1 };
                if their_dom > max_allowed_dom {
                    return Err(format!(
                        "Hedef tanrıyı mühürlemek için baskın olduğu evren sayısı en fazla {max_allowed_dom} olmalı (Şu an: {their_dom})."
                    ));
                }

                self.spend_for_action(
                    deity_id,
                    ResourceExpenditure::DivineAction,
                    cost,
                    None,
                    "Tanrı mühürleme",
                );

                let target_name = self.deities[target_idx].name.clone();
                self.deities[target_idx].is_sealed = true;
                self.deities[target_idx].sealed_by = Some(deity_id);

                // Absorb remaining essence & souls via ledger transfer
                let remaining = self.deities[target_idx].resources;
                let (actor_res, target_res) = if d_idx < target_idx {
                    let (left, right) = self.deities.split_at_mut(target_idx);
                    (&mut left[d_idx].resources, &mut right[0].resources)
                } else {
                    let (left, right) = self.deities.split_at_mut(d_idx);
                    (&mut right[0].resources, &mut left[target_idx].resources)
                };
                self.ledger.record_transfer(
                    self.tick,
                    target_res,
                    target_deity_id,
                    actor_res,
                    deity_id,
                    remaining,
                    format!("{target_name} mühürleme ganimeti"),
                );

                for u in &mut self.universes {
                    if u.influence_of(target_deity_id) > 0.0 {
                        u.deity_influence.remove(&target_deity_id);
                        u.add_influence(deity_id, 20.0);
                    }
                }

                let msg =
                    format!("{target_name} ebedî boşluğa mühürlendi ve tüm kozmik gücü soğuruldu!");
                self.push_event(
                    None,
                    Some(deity_id),
                    CosmicEventKind::DeitySealed,
                    EventSeverity::Critical,
                    "Tanrı Mühürlendi!",
                    &msg,
                );
                Ok(msg)
            }

            DivineAction::DisruptRivalAscension { target_deity_id } => {
                if target_deity_id == deity_id || target_deity_id as usize >= self.deities.len() {
                    return Err("Geçersiz hedef tanrı.".to_string());
                }
                let target_idx = target_deity_id as usize;
                if self.deities[target_idx].is_sealed {
                    return Err("Bu tanrı zaten mühürlenmiş.".to_string());
                }
                if self.deities[target_idx].ascension_progress <= 0.0 {
                    return Err(
                        "Hedef tanrının aktif bir Aşkınlık Ritüeli ilerlemesi yok.".to_string()
                    );
                }

                self.spend_for_action(
                    deity_id,
                    ResourceExpenditure::DivineAction,
                    cost,
                    None,
                    "Rakip aşkınlık ritüeli sabotajı",
                );

                let target_name = self.deities[target_idx].name.clone();
                let before = self.deities[target_idx].ascension_progress;
                let drop_amt = if has_seal_power { 35.0 } else { 20.0 };
                let after = (before - drop_amt).max(0.0);
                self.deities[target_idx].ascension_progress = after;
                self.deities[target_idx].adjust_relation(deity_id, -35);

                let msg = format!(
                    "{target_name} Aşkınlık Ritüeli sabote edildi (%{before:.0} -> %{after:.0})!"
                );
                self.push_event(
                    None,
                    Some(deity_id),
                    CosmicEventKind::DivineIntervention,
                    EventSeverity::Positive,
                    "Rakip Ritüeli Sabote Edildi",
                    &msg,
                );
                Ok(msg)
            }

            DivineAction::AdvanceAscensionRitual => {
                let unlocked_powers = self.deities[d_idx].unlocked_powers.len();
                let full_allegiance = self.full_allegiance_universe_count(deity_id);
                if unlocked_powers < TRANSCENDENCE_MIN_POWERS_TO_START
                    || full_allegiance < TRANSCENDENCE_MIN_FULL_ALLEGIANCE_TO_START
                {
                    return Err(format!(
                        "Aşkınlık Ritüeli için en az {} İlahi Güç (Şu an: {}) ve {} ★ Tam Aidiyet Evreni (Şu an: {}) gerekir!",
                        TRANSCENDENCE_MIN_POWERS_TO_START,
                        unlocked_powers,
                        TRANSCENDENCE_MIN_FULL_ALLEGIANCE_TO_START,
                        full_allegiance
                    ));
                }

                self.spend_for_action(
                    deity_id,
                    ResourceExpenditure::AscensionRitual,
                    cost,
                    None,
                    "Aşkınlık ritüeli adımı",
                );
                self.deities[d_idx].ascension_progress =
                    (self.deities[d_idx].ascension_progress + TRANSCENDENCE_STEP_GAIN).min(100.0);
                let progress = self.deities[d_idx].ascension_progress;

                // Rival deities react with alarm when someone nears Transcendence!
                if progress >= 50.0 {
                    for other_idx in 0..self.deities.len() {
                        if other_idx != d_idx && !self.deities[other_idx].is_sealed {
                            self.deities[other_idx].adjust_relation(deity_id, -12);
                        }
                    }
                }

                let msg = format!(
                    "{deity_name} Aşkınlık Ritüelini ilerletti (+%{TRANSCENDENCE_STEP_GAIN:.0} -> İlerleme: %{progress:.0}/100)."
                );
                let sev = if deity_id != PLAYER_DEITY_ID && progress >= 60.0 {
                    EventSeverity::Critical
                } else {
                    EventSeverity::Positive
                };
                self.push_event(
                    None,
                    Some(deity_id),
                    CosmicEventKind::AscensionMilestone,
                    sev,
                    "Aşkınlık Ritüeli",
                    &msg,
                );
                self.phase_6_evaluate_victory_and_defeat();
                Ok(msg)
            }

            DivineAction::FocusUniverse { universe_id } => {
                if let Some(u_id) = universe_id {
                    let u_idx = self.validate_dominated_universe(u_id, deity_id)?;
                    self.focused_universe_id = Some(u_id);
                    let u_name = self.universes[u_idx].name.clone();
                    Ok(format!(
                        "{u_name} ilahi odak altına alındı (+%25 ibadet akışı ve istikrar desteği)."
                    ))
                } else {
                    self.focused_universe_id = None;
                    Ok("Evren odağı kaldırıldı.".to_string())
                }
            }

            DivineAction::ResolveDilemma { choose_first } => {
                let Some(dilemma) = self.active_dilemma.take() else {
                    return Err("Şu anda yanıt bekleyen aktif bir kozmik ikilem yok.".to_string());
                };
                let u_idx =
                    (dilemma.universe_id as usize).min(self.universes.len().saturating_sub(1));
                let u_name = self.universes[u_idx].name.clone();

                let summary = if choose_first {
                    self.universes[u_idx].stability =
                        (self.universes[u_idx].stability + 16.0).min(100.0);
                    self.universes[u_idx].add_influence(deity_id, 18.0);
                    self.ledger.record_source(
                        self.tick,
                        &mut self.deities[d_idx].resources,
                        deity_id,
                        ResourceSource::FaithDevotion,
                        Resources::new(65, 30, 5),
                        Some(dilemma.universe_id),
                        "Kozmik ikilem: Düzen ve kutsama kararı",
                    );
                    format!(
                        "{u_name} ikileminde '{}' seçildi (+65 Ruh, +30 Nüfuz, +5 Öz, +16 İstikrar).",
                        dilemma.choice_a_label
                    )
                } else {
                    self.universes[u_idx].stability =
                        (self.universes[u_idx].stability - 5.0).max(10.0);
                    self.ledger.record_source(
                        self.tick,
                        &mut self.deities[d_idx].resources,
                        deity_id,
                        ResourceSource::AnomalyDiscovery,
                        Resources::new(105, 0, 35),
                        Some(dilemma.universe_id),
                        "Kozmik ikilem: Öz ve kaos hasadı kararı",
                    );
                    format!(
                        "{u_name} ikileminde '{}' seçildi (+105 Ruh, +35 Kozmik Öz).",
                        dilemma.choice_b_label
                    )
                };

                self.universes[u_idx].record_history(
                    self.tick,
                    HistoryCategory::DivineIntervention,
                    Some(deity_id),
                    &summary,
                );
                self.push_event(
                    Some(dilemma.universe_id),
                    Some(deity_id),
                    CosmicEventKind::DivineIntervention,
                    EventSeverity::Positive,
                    "Kozmik Karar Verildi",
                    &summary,
                );
                Ok(summary)
            }

            DivineAction::EnlightenCivilizations { universe_id } => {
                let u_idx = self.validate_dominated_universe(universe_id, deity_id)?;
                self.spend_for_action(
                    deity_id,
                    ResourceExpenditure::DivineAction,
                    cost,
                    Some(universe_id),
                    "Uygarlıkları aydınlatma ve kalkan çözme",
                );

                let civ_id_if_empty = self.next_civ_id;
                let u = &mut self.universes[u_idx];
                u.stability = (u.stability + 10.0).min(100.0);
                u.add_influence(deity_id, 14.0);

                if u.civilizations.is_empty() {
                    self.next_civ_id += 1;
                    u.civilizations.push(Civilization {
                        id: civ_id_if_empty,
                        name: format!("{deity_name} Aydınları #{civ_id_if_empty}"),
                        universe_id,
                        doctrine: CivilizationDoctrine::from_seed(universe_id, civ_id_if_empty),
                        population: 140,
                        tech_level: 32.0,
                        faith_fervor: 88.0,
                        stability: 86.0,
                        militarism: 18.0,
                        deity_awareness: 5.0,
                        patron_deity: Some(deity_id),
                        religion_name: format!("{deity_name} Aydınlanması"),
                        in_rebellion: false,
                        divine_shield_active: false,
                    });
                } else {
                    for civ in &mut u.civilizations {
                        civ.divine_shield_active = false;
                        civ.in_rebellion = false;
                        civ.deity_awareness = (civ.deity_awareness - 35.0).max(0.0);
                        civ.faith_fervor = (civ.faith_fervor + 22.0).min(100.0);
                        civ.stability = (civ.stability + 16.0).min(100.0);
                        civ.population = (civ.population + 60).min(50_000);
                        civ.patron_deity = Some(deity_id);
                        civ.religion_name = format!("{deity_name} Aydınlanması");
                    }
                }

                let u_name = u.name.clone();
                u.record_history(
                    self.tick,
                    HistoryCategory::DivineIntervention,
                    Some(deity_id),
                    format!(
                        "{deity_name} uygarlıkları ilahi bilgelikle aydınlattı; bilimsel kalkanlar ve isyanlar sona erdi."
                    ),
                );
                let msg = format!(
                    "{u_name} uygarlıkları aydınlatıldı: Bilimsel kalkanlar çözüldü, isyanlar dindi (+22 İnanç, +60M Nüfus)."
                );
                if deity_id == PLAYER_DEITY_ID {
                    self.push_event(
                        Some(universe_id),
                        Some(deity_id),
                        CosmicEventKind::DivineIntervention,
                        EventSeverity::Positive,
                        "Uygarlıklar Aydınlatıldı",
                        &msg,
                    );
                }
                Ok(msg)
            }

            DivineAction::ReshapeUniverseDomain { universe_id } => {
                let u_idx = self.validate_dominated_universe(universe_id, deity_id)?;
                self.spend_for_action(
                    deity_id,
                    ResourceExpenditure::UniverseCreation,
                    cost,
                    Some(universe_id),
                    "Evren dokusunu dönüştürme",
                );

                let u = &mut self.universes[u_idx];
                let old_type = u.universe_type;
                let new_type = match old_type {
                    UniverseType::Primordial | UniverseType::Collapsing | UniverseType::Chaos => {
                        UniverseType::Life
                    }
                    UniverseType::Life => UniverseType::Faith,
                    UniverseType::Faith => UniverseType::Tech,
                    UniverseType::Tech => UniverseType::Life,
                };
                u.universe_type = new_type;
                u.stability = (u.stability + 18.0).min(100.0);
                u.danger_level = (u.danger_level - 18.0).max(5.0);
                u.soul_potential = (u.soul_potential + 0.22).min(3.0);
                u.essence_richness = (u.essence_richness + 0.22).min(3.0);
                u.add_influence(deity_id, 18.0);
                let newly_shaped = u.shaped_by_deity != Some(deity_id);
                u.shaped_by_deity = Some(deity_id);

                let u_name = u.name.clone();
                u.record_history(
                    self.tick,
                    HistoryCategory::DivineIntervention,
                    Some(deity_id),
                    format!(
                        "{deity_name} evrenin dokusunu {} türünden {} türüne dönüştürdü.",
                        old_type.name_tr(),
                        new_type.name_tr()
                    ),
                );
                if newly_shaped {
                    self.deities[d_idx].shaped_universes_count += 1;
                }

                let msg = format!(
                    "{u_name} dokusu '{}' -> '{}' olarak dönüştürüldü (+0.22x Ruh & Öz Çarpanı, +18 İstikrar).",
                    old_type.name_tr(),
                    new_type.name_tr()
                );
                if deity_id == PLAYER_DEITY_ID {
                    self.push_event(
                        Some(universe_id),
                        Some(deity_id),
                        CosmicEventKind::DivineIntervention,
                        EventSeverity::Positive,
                        "Evren Dokusu Dönüştürüldü",
                        &msg,
                    );
                }
                Ok(msg)
            }

            DivineAction::ChannelAmbientEnergy { universe_id } => {
                let u_idx = self.validate_dominated_universe(universe_id, deity_id)?;
                self.spend_for_action(
                    deity_id,
                    ResourceExpenditure::DivineAction,
                    cost,
                    Some(universe_id),
                    "Kozmik rezonans başlatma",
                );

                let soul_gain = if has_siphon_power { 105 } else { 75 };
                let neighbor_inf = if has_gateway_power { 10.0 } else { 6.0 };

                let open_neighbors: Vec<u32> = self.universes[u_idx]
                    .gateways
                    .iter()
                    .filter(|g| matches!(g.state, GatewayState::Open))
                    .map(|g| g.target_universe_id)
                    .collect();

                let u = &mut self.universes[u_idx];
                u.stability = (u.stability + 12.0).min(100.0);
                u.soul_reservoir += 35;
                u.add_influence(deity_id, 10.0);
                let u_name = u.name.clone();
                u.record_history(
                    self.tick,
                    HistoryCategory::DivineIntervention,
                    Some(deity_id),
                    format!(
                        "{deity_name} kozmik rezonans başlatarak ortam enerjisini ruha ve geçit dalgalarına dönüştürdü."
                    ),
                );

                for &n_id in &open_neighbors {
                    if let Some(nu) = self.universes.get_mut(n_id as usize) {
                        if !nu.is_destroyed {
                            nu.add_influence(deity_id, neighbor_inf);
                        }
                    }
                }

                self.ledger.record_source(
                    self.tick,
                    &mut self.deities[d_idx].resources,
                    deity_id,
                    ResourceSource::UniversePassive,
                    Resources::new(soul_gain, 0, 0),
                    Some(universe_id),
                    format!("{u_name} kozmik rezonans enerjisi"),
                );

                let msg = format!(
                    "{u_name} kozmik rezonansı: +{soul_gain} Ruh üretildi, +12 İstikrar ve {} komşu evrene +{neighbor_inf:.0} Nüfuz yayıldı.",
                    open_neighbors.len()
                );
                if deity_id == PLAYER_DEITY_ID {
                    self.push_event(
                        Some(universe_id),
                        Some(deity_id),
                        CosmicEventKind::DivineIntervention,
                        EventSeverity::Positive,
                        "Kozmik Rezonans",
                        &msg,
                    );
                }
                Ok(msg)
            }
        };

        if let Ok(mut msg) = action_result {
            if deity_id == PLAYER_DEITY_ID {
                if let Some(tut_bonus_msg) = self.advance_tutorial_if_matched(&action_for_tutorial)
                {
                    msg = format!("{msg} | {tut_bonus_msg}");
                }
            }
            Ok(msg)
        } else {
            action_result
        }
    }

    /// Triggers an immediate cosmic burst effect when `deity_id` unlocks `power`, making every
    /// single power tree node tangibly impact the universe pool and economy right away.
    fn apply_power_unlock_burst(&mut self, deity_id: u32, power: PowerId) -> String {
        let d_idx = deity_id as usize;
        let deity_name = self.deities[d_idx].name.clone();

        match power {
            PowerId::GenesisSpark => {
                let mut affected = 0usize;
                for u in &mut self.universes {
                    if !u.is_destroyed && u.is_dominated_by(deity_id) {
                        affected += 1;
                        u.stability = (u.stability + 12.0).min(100.0);
                        u.soul_potential = (u.soul_potential + 0.15).min(3.0);
                        for civ in &mut u.civilizations {
                            civ.population = (civ.population + 45).min(50_000);
                            civ.stability = (civ.stability + 10.0).min(100.0);
                        }
                        if u.civilizations.is_empty() {
                            let cid = self.next_civ_id;
                            self.next_civ_id += 1;
                            u.civilizations.push(Civilization {
                                id: cid,
                                name: format!("{deity_name} Kıvılcımı #{cid}"),
                                universe_id: u.id,
                                doctrine: CivilizationDoctrine::from_seed(u.id, cid),
                                population: 120,
                                tech_level: 18.0,
                                faith_fervor: 80.0,
                                stability: 82.0,
                                militarism: 15.0,
                                deity_awareness: 5.0,
                                patron_deity: Some(deity_id),
                                religion_name: format!("{deity_name} Düzeni"),
                                in_rebellion: false,
                                divine_shield_active: false,
                            });
                        }
                    }
                }
                format!(
                    "[Anında Etki: {affected} baskın evrende +12 İstikrar, +0.15x Ruh Potansiyeli ve +45M Nüfus patlaması!]"
                )
            }
            PowerId::RealityAnchor => {
                let mut affected = 0usize;
                for u in &mut self.universes {
                    if !u.is_destroyed && u.is_dominated_by(deity_id) {
                        affected += 1;
                        if u.universe_type == UniverseType::Collapsing {
                            u.universe_type = UniverseType::Faith;
                        }
                        u.stability = (u.stability + 18.0).min(100.0);
                        u.danger_level = (u.danger_level - 15.0).max(5.0);
                        u.essence_richness = (u.essence_richness + 0.20).min(3.0);
                    }
                }
                format!(
                    "[Anında Etki: {affected} baskın evrende +18 İstikrar, +0.20x Öz Zenginliği ve her tur pasif onarım!]"
                )
            }
            PowerId::CosmicForge => {
                let mut newly_shaped = 0usize;
                for u in &mut self.universes {
                    if !u.is_destroyed && u.is_dominated_by(deity_id) {
                        u.soul_potential = (u.soul_potential + 0.20).min(3.0);
                        u.essence_richness = (u.essence_richness + 0.20).min(3.0);
                        if u.shaped_by_deity != Some(deity_id) && newly_shaped < 2 {
                            u.shaped_by_deity = Some(deity_id);
                            newly_shaped += 1;
                        }
                    }
                }
                self.deities[d_idx].shaped_universes_count += newly_shaped as u32;
                self.ledger.record_source(
                    self.tick,
                    &mut self.deities[d_idx].resources,
                    deity_id,
                    ResourceSource::UniversePassive,
                    Resources::new(0, 0, 40),
                    None,
                    "Kozmik Ocak uyanış özü",
                );
                "[Anında Etki: +40 Kozmik Öz kazanıldı ve şekillendirilmiş evrenlerde her tur +4R +3N +2Ö ek gelir başladı!]"
                    .to_string()
            }
            PowerId::VoiceOfProphecy => {
                let mut affected = 0usize;
                for u in &mut self.universes {
                    if !u.is_destroyed
                        && ((deity_id == PLAYER_DEITY_ID && u.discovered_by_player)
                            || u.influence_of(deity_id) > 0.0)
                    {
                        u.add_influence(deity_id, 14.0);
                        affected += 1;
                    }
                }
                self.ledger.record_source(
                    self.tick,
                    &mut self.deities[d_idx].resources,
                    deity_id,
                    ResourceSource::FaithDevotion,
                    Resources::new(0, 35, 0),
                    None,
                    "Kehanet Sesi yankısı",
                );
                format!(
                    "[Anında Etki: {affected} evrende +14 İlahi Nüfuz yayıldı ve +35 Nüfuz kazanıldı!]"
                )
            }
            PowerId::GoldenMiracle => {
                let mut affected = 0usize;
                for u in &mut self.universes {
                    if !u.is_destroyed && u.is_dominated_by(deity_id) {
                        affected += 1;
                        u.stability = (u.stability + 15.0).min(100.0);
                        for civ in &mut u.civilizations {
                            civ.in_rebellion = false;
                            civ.divine_shield_active = false;
                            civ.faith_fervor = (civ.faith_fervor + 18.0).min(100.0);
                            civ.stability = (civ.stability + 15.0).min(100.0);
                        }
                    }
                }
                format!(
                    "[Anında Etki: {affected} baskın evrende tüm isyanlar ve bilimsel kalkanlar kaldırıldı (+18 İnanç)!]"
                )
            }
            PowerId::AbsoluteDogma => {
                let mut affected = 0usize;
                for u in &mut self.universes {
                    if !u.is_destroyed && u.is_dominated_by(deity_id) {
                        affected += 1;
                        u.add_influence(deity_id, 18.0);
                        let rivals: Vec<u32> = u
                            .deity_influence
                            .keys()
                            .copied()
                            .filter(|&id| id != deity_id)
                            .collect();
                        for r_id in rivals {
                            u.add_influence(r_id, -12.0);
                        }
                    }
                }
                format!(
                    "[Anında Etki: {affected} evrende +18 Nüfuz kazanıldı, rakip etkileri -12 kırıldı ve ★ Tam Aidiyet ibadet geliri +%45 arttı!]"
                )
            }
            PowerId::SoulSiphon => {
                let mut drained = 0i64;
                for u in &mut self.universes {
                    if !u.is_destroyed && u.is_dominated_by(deity_id) {
                        let take = u.soul_reservoir.clamp(15, 35);
                        u.soul_reservoir = (u.soul_reservoir - take).max(0);
                        drained += take;
                    }
                }
                let grant = drained.max(90);
                self.ledger.record_source(
                    self.tick,
                    &mut self.deities[d_idx].resources,
                    deity_id,
                    ResourceSource::DivineHarvest,
                    Resources::new(grant, 0, 0),
                    None,
                    "Ruh Sifonu uyanış çekimi",
                );
                format!(
                    "[Anında Etki: Evren havuzlarından +{grant} Ruh çekildi ve tüm ruh gelirleri +%40 arttı!]"
                )
            }
            PowerId::EssenceCondenser => {
                for u in &mut self.universes {
                    if !u.is_destroyed && u.is_dominated_by(deity_id) {
                        u.essence_richness = (u.essence_richness + 0.25).min(3.0);
                    }
                }
                self.ledger.record_source(
                    self.tick,
                    &mut self.deities[d_idx].resources,
                    deity_id,
                    ResourceSource::UniversePassive,
                    Resources::new(0, 0, 35),
                    None,
                    "Öz Yoğunlaştırıcı kristalleşmesi",
                );
                "[Anında Etki: +35 Kozmik Öz kazanıldı, baskın evrenlerde +0.25x Öz Zenginliği ve her tur pasif öz üretimi başladı!]".to_string()
            }
            PowerId::ReapersToll => {
                self.ledger.record_source(
                    self.tick,
                    &mut self.deities[d_idx].resources,
                    deity_id,
                    ResourceSource::DivineHarvest,
                    Resources::new(110, 0, 20),
                    None,
                    "Ölüm Çanı kozmik hasadı",
                );
                "[Anında Etki: +110 Ruh ve +20 Kozmik Öz toplandı; artık her Ruh Hasadı +12 Kozmik Öz kazandırır!]".to_string()
            }
            PowerId::AstralSight => {
                let mut revealed = 0usize;
                if deity_id == PLAYER_DEITY_ID {
                    let candidates: Vec<u32> = self
                        .universes
                        .iter()
                        .filter(|u| u.discovered_by_player)
                        .flat_map(|u| u.gateways.iter().map(|g| g.target_universe_id))
                        .collect();
                    for cid in candidates {
                        if let Some(cu) = self.universes.get_mut(cid as usize) {
                            if !cu.discovered_by_player && revealed < 5 {
                                cu.discovered_by_player = true;
                                revealed += 1;
                            }
                        }
                    }
                }
                self.ledger.record_source(
                    self.tick,
                    &mut self.deities[d_idx].resources,
                    deity_id,
                    ResourceSource::UniversePassive,
                    Resources::new(0, 30, 0),
                    None,
                    "Astral Görüş ufku",
                );
                format!(
                    "[Anında Etki: {revealed} yeni komşu evren keşfedildi, +30 Nüfuz kazanıldı ve her 4 turda otomatik keşif başladı!]"
                )
            }
            PowerId::GatewayMastery => {
                let dom_ids: Vec<u32> = self
                    .universes
                    .iter()
                    .filter(|u| !u.is_destroyed && u.is_dominated_by(deity_id))
                    .map(|u| u.id)
                    .collect();
                let mut opened = 0usize;
                for uid in dom_ids {
                    let targets: Vec<u32> = self.universes[uid as usize]
                        .gateways
                        .iter()
                        .filter(|g| matches!(g.state, GatewayState::Closed))
                        .map(|g| g.target_universe_id)
                        .collect();
                    for tid in targets {
                        for g in &mut self.universes[uid as usize].gateways {
                            if g.target_universe_id == tid {
                                g.state = GatewayState::Open;
                            }
                        }
                        for g in &mut self.universes[tid as usize].gateways {
                            if g.target_universe_id == uid {
                                g.state = GatewayState::Open;
                            }
                        }
                        if deity_id == PLAYER_DEITY_ID {
                            self.universes[tid as usize].discovered_by_player = true;
                        }
                        opened += 1;
                    }
                }
                self.ledger.record_source(
                    self.tick,
                    &mut self.deities[d_idx].resources,
                    deity_id,
                    ResourceSource::UniversePassive,
                    Resources::new(0, 25, 15),
                    None,
                    "Geçit Ustalığı rezonansı",
                );
                format!(
                    "[Anında Etki: {opened} kapalı geçit açıldı, +25 Nüfuz & +15 Öz kazanıldı ve her açık geçit +1 Nüfuz/tur üretmeye başladı!]"
                )
            }
            PowerId::DimensionalWard => {
                let mut warded = 0usize;
                for u in &mut self.universes {
                    if !u.is_destroyed && u.is_dominated_by(deity_id) {
                        u.stability = (u.stability + 15.0).min(100.0);
                        u.danger_level = (u.danger_level - 15.0).max(5.0);
                        warded += 1;
                    }
                }
                format!(
                    "[Anında Etki: {warded} baskın evren boyutsal kalkanla korundu (+15 İstikrar) ve istikrarsızlık/isyan kayıpları %75 azaltıldı!]"
                )
            }
            PowerId::DivineSmite => {
                let mut smitten = 0usize;
                for u in &mut self.universes {
                    if !u.is_destroyed && u.is_dominated_by(deity_id) {
                        let rivals: Vec<u32> = u
                            .deity_influence
                            .keys()
                            .copied()
                            .filter(|&id| id != deity_id)
                            .collect();
                        for r_id in rivals {
                            u.add_influence(r_id, -20.0);
                            smitten += 1;
                        }
                    }
                }
                self.ledger.record_source(
                    self.tick,
                    &mut self.deities[d_idx].resources,
                    deity_id,
                    ResourceSource::DivineHarvest,
                    Resources::new(70, 0, 0),
                    None,
                    "İlahi Gazap ganimeti",
                );
                format!(
                    "[Anında Etki: Baskın evrenlerdeki {smitten} rakip izi temizlendi (-20 Nüfuz) ve +70 Ruh kazanıldı!]"
                )
            }
            PowerId::ShadowSchism => {
                let mut schisms = 0usize;
                for u in &mut self.universes {
                    if !u.is_destroyed && !u.is_dominated_by(deity_id) && schisms < 3 {
                        if let Some((dom_id, _)) = u.dominant_deity() {
                            u.add_influence(dom_id, -16.0);
                            u.add_influence(deity_id, 12.0);
                            for civ in &mut u.civilizations {
                                civ.in_rebellion = true;
                            }
                            schisms += 1;
                        }
                    }
                }
                self.ledger.record_source(
                    self.tick,
                    &mut self.deities[d_idx].resources,
                    deity_id,
                    ResourceSource::UniversePassive,
                    Resources::new(0, 35, 0),
                    None,
                    "Gölge Ayrılığı entrikası",
                );
                format!(
                    "[Anında Etki: {schisms} rakip evrende mezhep isyanı patlak verdi ve +35 Nüfuz kazanıldı!]"
                )
            }
            PowerId::EternalSeal => {
                for other_idx in 0..self.deities.len() {
                    if other_idx != d_idx && !self.deities[other_idx].is_sealed {
                        self.deities[other_idx].ascension_progress =
                            (self.deities[other_idx].ascension_progress - 10.0).max(0.0);
                    }
                }
                self.ledger.record_source(
                    self.tick,
                    &mut self.deities[d_idx].resources,
                    deity_id,
                    ResourceSource::UniversePassive,
                    Resources::new(80, 0, 25),
                    None,
                    "Ebedî Mühür otoritesi",
                );
                "[Anında Etki: Tüm rakiplerin Aşkınlık Ritüeli -%10 geriletildi, +80 Ruh & +25 Öz kazanıldı ve mühürlenen tanrı başına her tur haraç geliri başladı!]".to_string()
            }
        }
    }

    fn advance_tutorial_if_matched(&mut self, action: &DivineAction) -> Option<String> {
        let matched_next = match (self.tutorial_step, action) {
            (TutorialStep::Step1Prophecy, DivineAction::SpreadProphecy { .. }) => {
                Some(TutorialStep::Step2Explore)
            }
            (
                TutorialStep::Step2Explore,
                DivineAction::ScoutRegion { .. } | DivineAction::OpenGateway { .. },
            ) => Some(TutorialStep::Step3HarvestOrEssence),
            (
                TutorialStep::Step3HarvestOrEssence,
                DivineAction::HarvestSouls { .. }
                | DivineAction::ExtractCosmicEssence { .. }
                | DivineAction::ChannelAmbientEnergy { .. },
            ) => Some(TutorialStep::Step4UnlockPower),
            (TutorialStep::Step4UnlockPower, DivineAction::UnlockPower { .. }) => {
                Some(TutorialStep::Step5CreateOrStabilize)
            }
            (
                TutorialStep::Step5CreateOrStabilize,
                DivineAction::SeedLifeAndOrder { .. }
                | DivineAction::StabilizeReality { .. }
                | DivineAction::BestowMiracle { .. }
                | DivineAction::EnlightenCivilizations { .. }
                | DivineAction::ReshapeUniverseDomain { .. },
            ) => Some(TutorialStep::Step6RivalInteraction),
            (
                TutorialStep::Step6RivalInteraction,
                DivineAction::SmiteRivalInfluence { .. }
                | DivineAction::InciteRebellion { .. }
                | DivineAction::OfferDiplomaticPact { .. }
                | DivineAction::SendDivineGift { .. }
                | DivineAction::DeclareHolyWar { .. }
                | DivineAction::DemandTribute { .. },
            ) => Some(TutorialStep::Completed),
            _ => None,
        }?;

        let completed_step = self.tutorial_step;
        let reward = completed_step.reward();
        self.tutorial_step = matched_next;

        if reward != Resources::ZERO {
            self.ledger.record_source(
                self.tick,
                &mut self.deities[PLAYER_DEITY_ID as usize].resources,
                PLAYER_DEITY_ID,
                ResourceSource::InitialGrant,
                reward,
                None,
                format!("Eğitici görev ödülü: {}", completed_step.title_tr()),
            );
        }

        let note = format!(
            "EĞİTİCİ GÖREV TAMAMLANDI ({})! Ödül: +{} Ruh, +{} Nüfuz, +{} Öz",
            completed_step.title_tr(),
            reward.souls,
            reward.influence,
            reward.cosmic_essence
        );
        self.push_event(
            None,
            Some(PLAYER_DEITY_ID),
            CosmicEventKind::AscensionMilestone,
            EventSeverity::Positive,
            "Eğitici Görev Tamamlandı",
            &note,
        );
        Some(note)
    }

    fn validate_universe(
        &self,
        universe_id: u32,
        require_player_discovered: bool,
    ) -> Result<usize, String> {
        let idx = universe_id as usize;
        if idx >= self.universes.len() {
            return Err("Geçersiz evren kimliği.".to_string());
        }
        if self.universes[idx].is_destroyed {
            return Err("Bu evren tamamen çökmüş durumda.".to_string());
        }
        if require_player_discovered && !self.universes[idx].discovered_by_player {
            return Err(
                "Bu evren henüz keşfedilmedi. Önce geçit açın veya keşif yapın.".to_string(),
            );
        }
        Ok(idx)
    }

    /// Ensures the universe is valid, discovered (if player), AND currently dominated by `deity_id`.
    /// Non-dominated universes do not grant sovereign authority ("güç ve söz sahibi")!
    fn validate_dominated_universe(
        &self,
        universe_id: u32,
        deity_id: u32,
    ) -> Result<usize, String> {
        let idx = self.validate_universe(universe_id, deity_id == PLAYER_DEITY_ID)?;
        if !self.universes[idx].is_dominated_by(deity_id) {
            return Err(format!(
                "{} evreninde henüz hâkimiyetin yok! Söz sahibi olmak için önce Kehanet Yay, Rakip Vur veya İsyan Çıkar ile baskın tanrı (◆ BİZİM / ★ TAM AİDİYET) olmalısın.",
                self.universes[idx].name
            ));
        }
        Ok(idx)
    }

    fn spend_for_action(
        &mut self,
        deity_id: u32,
        expenditure: ResourceExpenditure,
        cost: Resources,
        universe_id: Option<u32>,
        description: impl Into<String>,
    ) {
        let d_idx = deity_id as usize;
        self.ledger.try_record_expenditure(
            self.tick,
            &mut self.deities[d_idx].resources,
            deity_id,
            expenditure,
            cost,
            universe_id,
            description,
        );
    }

    /// Advances the cosmic simulation by one fixed timestep tick.
    ///
    /// The phase order below is part of the simulation contract: universe/civilization
    /// evolution, economy, cosmic events, rival decisions, diplomacy, then outcomes.
    /// Keep this order stable; changing it can change deterministic save replays and
    /// gameplay results even when the RNG seed is unchanged.
    pub fn step_tick(&mut self) {
        if !matches!(self.outcome, GameOutcome::Ongoing)
            && self.config.game_mode == GameMode::ScenarioCosmicAscension
        {
            return;
        }

        self.tick += 1;

        self.phase_1_evolve_universes_and_civilizations();
        self.phase_2_resolve_economy_and_souls();
        self.phase_3_generate_cosmic_events();
        self.phase_4_run_rival_ai_decisions();
        self.phase_5_update_diplomacy_and_seals();
        self.phase_6_evaluate_victory_and_defeat();
    }

    fn phase_1_evolve_universes_and_civilizations(&mut self) {
        let tick = self.tick;
        let mut pending_events = Vec::new();

        for u_idx in 0..self.universes.len() {
            if self.universes[u_idx].is_destroyed {
                continue;
            }

            // Background approximation for distant unobserved universes with no player influence:
            // Evolve every 2nd tick with 2.0x scaled continuous updates to cut CPU work in half.
            // This is deterministic for a given seed, but is not guaranteed to match the exact
            // state of an unoptimized run because nonlinear/clamped updates and random events
            // do not necessarily commute with the skipped tick.
            let is_distant_unobserved = self.enable_background_optimization
                && !self.universes[u_idx].discovered_by_player
                && self.universes[u_idx].region_tier >= 2
                && self.universes[u_idx].influence_of(PLAYER_DEITY_ID) == 0.0;

            if is_distant_unobserved && !(tick + u_idx as u64).is_multiple_of(2) {
                continue;
            }
            let step_scale = if is_distant_unobserved { 2.0 } else { 1.0 };

            let u_type = self.universes[u_idx].universe_type;
            let u_id = self.universes[u_idx].id;

            // Natural cosmic essence regeneration (prevents permanent depletion after extraction)
            let essence_regen = match u_type {
                UniverseType::Primordial | UniverseType::Chaos | UniverseType::Collapsing => 0.015,
                _ => 0.006,
            } * step_scale;
            if self.universes[u_idx].essence_richness < 1.35 {
                self.universes[u_idx].essence_richness =
                    (self.universes[u_idx].essence_richness + essence_regen).min(1.35);
            }

            // Natural universe stability drift by type
            let stab_delta = (match u_type {
                UniverseType::Collapsing => -self.rng.range_f32(0.4, 1.3),
                UniverseType::Chaos => self.rng.range_f32(-0.8, 0.5),
                UniverseType::Life | UniverseType::Faith | UniverseType::Tech => {
                    self.rng.range_f32(-0.15, 0.35)
                }
                UniverseType::Primordial => self.rng.range_f32(-0.1, 0.2),
            }) * step_scale;
            self.universes[u_idx].stability =
                (self.universes[u_idx].stability + stab_delta).clamp(0.0, 100.0);

            if self.universes[u_idx].stability <= 0.5 {
                let u_name = self.universes[u_idx].name.clone();
                self.universes[u_idx].is_destroyed = true;
                self.universes[u_idx].civilizations.clear();
                self.universes[u_idx].record_history(
                    tick,
                    HistoryCategory::CosmicAnomaly,
                    None,
                    "Gerçeklik tamamen çöktü; evren hiçliğe gömüldü.",
                );
                pending_events.push((
                    Some(u_id),
                    None,
                    CosmicEventKind::UniverseCollapsed,
                    EventSeverity::Critical,
                    format!("{u_name} Çöktü!"),
                    format!("{u_name} evreninin gerçeklik dokusu tamamen parçalandı."),
                ));
                continue;
            } else if self.universes[u_idx].stability < 22.0
                && u_type != UniverseType::Collapsing
                && self.rng.chance((0.15 * step_scale).min(0.45))
            {
                self.universes[u_idx].universe_type = UniverseType::Collapsing;
                self.universes[u_idx].record_history(
                    tick,
                    HistoryCategory::CosmicAnomaly,
                    None,
                    "Aşırı istikrarsızlık nedeniyle evren çöküş sürecine girdi.",
                );
                if self.universes[u_idx].discovered_by_player {
                    let u_name = self.universes[u_idx].name.clone();
                    pending_events.push((
                        Some(u_id),
                        None,
                        CosmicEventKind::UniverseCollapseWarning,
                        EventSeverity::Warning,
                        format!("{u_name} Çöküş Tehlikesinde"),
                        format!("{u_name} evreni istikrarını kaybediyor! Müdahale edilmezse yok olabilir."),
                    ));
                }
            }

            let u_stab = self.universes[u_idx].stability;
            let u_soul_mult = self.universes[u_idx].soul_potential;
            let dom_deity = self.universes[u_idx].dominant_deity().map(|(id, _)| id);

            // Apply continuous passive power effects of the dominant deity
            if let Some(d_id) = dom_deity {
                if let Some(dom_d) = self.deities.get(d_id as usize) {
                    if !dom_d.is_sealed {
                        let has_anchor = dom_d.has_power(PowerId::RealityAnchor);
                        let has_prophecy = dom_d.has_power(PowerId::VoiceOfProphecy);
                        let has_siphon = dom_d.has_power(PowerId::SoulSiphon);
                        let has_genesis = dom_d.has_power(PowerId::GenesisSpark);
                        let has_dogma = dom_d.has_power(PowerId::AbsoluteDogma);

                        if has_anchor {
                            self.universes[u_idx].stability =
                                (self.universes[u_idx].stability + 0.65 * step_scale).min(100.0);
                            self.universes[u_idx].essence_richness =
                                (self.universes[u_idx].essence_richness + 0.008 * step_scale)
                                    .min(2.5);
                        }
                        if has_prophecy {
                            self.universes[u_idx].add_influence(d_id, 0.75 * step_scale);
                        }
                        if has_siphon {
                            self.universes[u_idx].soul_reservoir += (6.0 * step_scale) as i64;
                        }
                        if has_genesis
                            && matches!(u_type, UniverseType::Life | UniverseType::Primordial)
                        {
                            for civ in &mut self.universes[u_idx].civilizations {
                                civ.population =
                                    (civ.population + (12.0 * step_scale) as u64).min(50_000);
                                civ.stability = (civ.stability + 1.2 * step_scale).min(100.0);
                            }
                        }
                        if has_dogma && self.universes[u_idx].has_full_allegiance(d_id) {
                            let rivals: Vec<u32> = self.universes[u_idx]
                                .deity_influence
                                .keys()
                                .copied()
                                .filter(|&id| id != d_id)
                                .collect();
                            for r_id in rivals {
                                self.universes[u_idx].add_influence(r_id, -0.8 * step_scale);
                            }
                        }
                    }
                }
            }

            let mut idx_c = 0;
            while idx_c < self.universes[u_idx].civilizations.len() {
                // If universe has a dominant deity and civ has no patron, adopt dominant deity
                if self.universes[u_idx].civilizations[idx_c]
                    .patron_deity
                    .is_none()
                {
                    if let Some(d_id) = dom_deity {
                        self.universes[u_idx].civilizations[idx_c].patron_deity = Some(d_id);
                    }
                }

                let rep = self.universes[u_idx].civilizations[idx_c].step_tick_scaled(
                    u_stab,
                    u_soul_mult,
                    u_type == UniverseType::Tech,
                    u_type == UniverseType::Faith,
                    u_type == UniverseType::Chaos,
                    step_scale,
                    &mut self.rng,
                );

                // Add unclaimed mortal death souls to universe reservoir
                self.universes[u_idx].soul_reservoir +=
                    (rep.mortal_deaths_souls / 2).max(step_scale as i64);

                if rep.shield_activated && self.universes[u_idx].discovered_by_player {
                    let u_name = self.universes[u_idx].name.clone();
                    let civ_name = self.universes[u_idx].civilizations[idx_c].name.clone();
                    pending_events.push((
                        Some(u_id),
                        rep.patron_deity,
                        CosmicEventKind::ScientificAwakening,
                        EventSeverity::Warning,
                        "Tanrısal Müdahale Deşifre Edildi".to_string(),
                        format!(
                            "{u_name} evrenindeki {civ_name}, bilimsel araştırmalarla ilahi müdahaleyi tespit etti ve kalkan kurdu!"
                        ),
                    ));
                }

                if rep.rebellion_started && self.universes[u_idx].discovered_by_player {
                    let u_name = self.universes[u_idx].name.clone();
                    let civ_name = self.universes[u_idx].civilizations[idx_c].name.clone();
                    pending_events.push((
                        Some(u_id),
                        rep.patron_deity,
                        CosmicEventKind::CivilizationRebellion,
                        EventSeverity::Warning,
                        "Uygarlık İsyanı".to_string(),
                        format!("{u_name} evrenindeki {civ_name} içinde dini ayaklanma başladı."),
                    ));
                }

                if rep.collapsed {
                    let civ_name = self.universes[u_idx].civilizations[idx_c].name.clone();
                    self.universes[u_idx].record_history(
                        tick,
                        HistoryCategory::CivilizationRiseFall,
                        None,
                        format!("{civ_name} uygarlığı tarihten silindi."),
                    );
                    self.universes[u_idx].civilizations.remove(idx_c);
                } else {
                    idx_c += 1;
                }
            }

            // Faz 2.2.2: Rival Cults & Sectarian Schism (Rakip Kültler ve Mezhep Savaşları)
            if self.universes[u_idx].civilizations.len() >= 2 {
                let mut contenders: Vec<(u32, f32)> = self.universes[u_idx]
                    .deity_influence
                    .iter()
                    .filter(|(_, &inf)| inf >= 25.0)
                    .map(|(&id, &inf)| (id, inf))
                    .collect();
                contenders.sort_by(|a, b| {
                    b.1.partial_cmp(&a.1)
                        .unwrap_or(std::cmp::Ordering::Equal)
                        .then_with(|| a.0.cmp(&b.0))
                });

                if contenders.len() >= 2 {
                    let d1 = contenders[0].0;
                    let d2 = contenders[1].0;
                    let d1_name = self
                        .deities
                        .get(d1 as usize)
                        .map(|d| d.name.clone())
                        .unwrap_or_else(|| format!("İlah #{d1}"));
                    let d2_name = self
                        .deities
                        .get(d2 as usize)
                        .map(|d| d.name.clone())
                        .unwrap_or_else(|| format!("İlah #{d2}"));

                    let has_d2_cult = self.universes[u_idx]
                        .civilizations
                        .iter()
                        .any(|c| c.patron_deity == Some(d2));
                    if !has_d2_cult {
                        if let Some(challenger_civ) = self.universes[u_idx].civilizations.get_mut(1)
                        {
                            challenger_civ.patron_deity = Some(d2);
                            challenger_civ.religion_name = format!("{d2_name} Mezhebi");
                        }
                        if let Some(primary_civ) = self.universes[u_idx].civilizations.get_mut(0) {
                            primary_civ.patron_deity = Some(d1);
                            primary_civ.religion_name = format!("{d1_name} Kültü");
                        }
                    }
                }

                // Check if two civilizations follow rival patron deities
                let mut rival_pair: Option<(usize, u32, usize, u32)> = None;
                let civ_len = self.universes[u_idx].civilizations.len();
                'outer: for i_a in 0..civ_len {
                    if let Some(da) = self.universes[u_idx].civilizations[i_a].patron_deity {
                        for i_b in (i_a + 1)..civ_len {
                            if let Some(db) = self.universes[u_idx].civilizations[i_b].patron_deity
                            {
                                if da != db {
                                    rival_pair = Some((i_a, da, i_b, db));
                                    break 'outer;
                                }
                            }
                        }
                    }
                }

                if let Some((i_a, da, i_b, db)) = rival_pair {
                    if (tick + u_id as u64).is_multiple_of(12) {
                        let score_a = self.universes[u_idx].civilizations[i_a].faith_fervor * 0.6
                            + self.universes[u_idx].civilizations[i_a].militarism * 0.4;
                        let score_b = self.universes[u_idx].civilizations[i_b].faith_fervor * 0.6
                            + self.universes[u_idx].civilizations[i_b].militarism * 0.4;
                        let (win_d, lose_d, win_civ_name, lose_civ_name) = if score_a >= score_b {
                            (
                                da,
                                db,
                                self.universes[u_idx].civilizations[i_a].name.clone(),
                                self.universes[u_idx].civilizations[i_b].name.clone(),
                            )
                        } else {
                            (
                                db,
                                da,
                                self.universes[u_idx].civilizations[i_b].name.clone(),
                                self.universes[u_idx].civilizations[i_a].name.clone(),
                            )
                        };

                        self.universes[u_idx].soul_reservoir += 18;
                        self.universes[u_idx].add_influence(win_d, 6.0);
                        self.universes[u_idx].add_influence(lose_d, -5.0);
                        let u_name = self.universes[u_idx].name.clone();
                        let desc = format!(
                            "{u_name} evreninde mezhep çatışması: {win_civ_name} kültü, {lose_civ_name} mezhebini bastırdı (+6% Nüfuz, +18 Ruh Havuzu)."
                        );
                        self.universes[u_idx].record_history(
                            tick,
                            HistoryCategory::ReligiousConflict,
                            Some(win_d),
                            &desc,
                        );
                        if self.universes[u_idx].discovered_by_player
                            && (win_d == PLAYER_DEITY_ID || lose_d == PLAYER_DEITY_ID)
                        {
                            pending_events.push((
                                Some(u_id),
                                Some(win_d),
                                CosmicEventKind::ReligiousSchism,
                                EventSeverity::Warning,
                                "Mezhep Çatışması".to_string(),
                                desc,
                            ));
                        }
                    }
                }
            }
        }

        for (u_id, d_id, kind, sev, title, desc) in pending_events {
            self.push_event(u_id, d_id, kind, sev, title, desc);
        }

        // Cross-gateway faith & influence propagation: strong deities spread faith through Open Gateways
        let mut gateway_spillovers: Vec<(usize, u32, f32)> = Vec::new();
        for u in &self.universes {
            if u.is_destroyed {
                continue;
            }
            if let Some((dom_id, dom_inf)) = u.dominant_deity() {
                if dom_inf >= 50.0 {
                    let gw_mult = if self
                        .deities
                        .get(dom_id as usize)
                        .is_some_and(|d| d.has_power(PowerId::GatewayMastery))
                    {
                        2.0
                    } else {
                        1.0
                    };
                    for gw in &u.gateways {
                        if matches!(gw.state, GatewayState::Open) {
                            gateway_spillovers.push((
                                gw.target_universe_id as usize,
                                dom_id,
                                (dom_inf - 45.0) * 0.025 * gw_mult,
                            ));
                        }
                    }
                }
            }
        }
        for (target_idx, d_id, inf_delta) in gateway_spillovers {
            if let Some(target_u) = self.universes.get_mut(target_idx) {
                if !target_u.is_destroyed {
                    target_u.add_influence(d_id, inf_delta);
                }
            }
        }

        // AstralSight passive cosmic revelation: every 4 ticks, automatically reveal 1 connected undiscovered universe
        if tick.is_multiple_of(4)
            && self
                .deities
                .get(PLAYER_DEITY_ID as usize)
                .is_some_and(|d| d.has_power(PowerId::AstralSight))
        {
            let next_unrevealed = self
                .universes
                .iter()
                .filter(|u| !u.is_destroyed && u.discovered_by_player)
                .flat_map(|u| u.gateways.iter().map(|g| g.target_universe_id))
                .find(|&tid| {
                    self.universes
                        .get(tid as usize)
                        .is_some_and(|tu| !tu.is_destroyed && !tu.discovered_by_player)
                });
            if let Some(tid) = next_unrevealed {
                self.universes[tid as usize].discovered_by_player = true;
                let t_name = self.universes[tid as usize].name.clone();
                self.push_event(
                    Some(tid),
                    Some(PLAYER_DEITY_ID),
                    CosmicEventKind::GatewayOpened,
                    EventSeverity::Info,
                    "Astral Görüş Keşfi",
                    format!("Astral Görüş gücü sayesinde {t_name} evreni haritada belirdi."),
                );
            }
        }

        // Focused universe stability support
        if let Some(focus_id) = self.focused_universe_id {
            if let Some(fu) = self.universes.get_mut(focus_id as usize) {
                if !fu.is_destroyed {
                    fu.stability = (fu.stability + 0.45).min(100.0);
                    fu.add_influence(PLAYER_DEITY_ID, 0.8);
                }
            }
        }
    }

    fn phase_2_resolve_economy_and_souls(&mut self) {
        let tick = self.tick;
        let deity_count = self.deities.len();
        let mut death_income = vec![Resources::ZERO; deity_count];
        let mut devotion_income = vec![Resources::ZERO; deity_count];
        let mut passive_income = vec![Resources::ZERO; deity_count];
        let diplomacy_income = economy_phase::diplomacy_income(&self.deities);
        let mut collapse_loss = vec![Resources::ZERO; deity_count];
        let mut rebellion_loss = vec![Resources::ZERO; deity_count];
        let mut upkeep_loss = vec![Resources::ZERO; deity_count];
        let mut dom_counts = vec![0usize; deity_count];

        let focused_id = self.focused_universe_id;

        for u in &mut self.universes {
            if u.is_destroyed {
                continue;
            }

            if let Some((dom_id, _)) = u.dominant_deity() {
                let d_idx = dom_id as usize;
                if d_idx < deity_count && !self.deities[d_idx].is_sealed {
                    dom_counts[d_idx] += 1;
                    let is_focused = dom_id == PLAYER_DEITY_ID && focused_id == Some(u.id);

                    if let Some((_, reservoir_take, breakdown)) =
                        u.estimate_economy_breakdown(tick, is_focused, &self.deities[d_idx])
                    {
                        u.soul_reservoir = (u.soul_reservoir - reservoir_take).max(0);
                        death_income[d_idx].souls += breakdown.reservoir_souls;
                        devotion_income[d_idx].add_assign(&breakdown.devotion_income);
                        passive_income[d_idx].add_assign(&breakdown.passive_income);
                        collapse_loss[d_idx].add_assign(&breakdown.collapse_loss);
                        rebellion_loss[d_idx].add_assign(&breakdown.rebellion_loss);
                        upkeep_loss[d_idx].add_assign(&breakdown.upkeep_loss);

                        // Imperial overextension upkeep: beyond 8 dominated universes, unstable nodes (< 55% stability) add extra upkeep
                        if dom_counts[d_idx] > 8 && u.stability < 55.0 {
                            upkeep_loss[d_idx].souls += 1;
                            upkeep_loss[d_idx].influence += 1;
                        }
                    }
                }
            }
        }

        for d_idx in 0..deity_count {
            if self.deities[d_idx].is_sealed {
                self.deities[d_idx].last_tick_income = Resources::ZERO;
                continue;
            }
            let d_id = self.deities[d_idx].id;
            let is_player = d_id == PLAYER_DEITY_ID;

            // Base divine spark so no active deity ever permanently stalls at 0
            passive_income[d_idx].souls += 6;
            passive_income[d_idx].influence += 3;
            if tick.is_multiple_of(2) {
                passive_income[d_idx].cosmic_essence += 1;
            }

            let mut net = Resources::ZERO;
            net.add_assign(&death_income[d_idx]);
            net.add_assign(&devotion_income[d_idx]);
            net.add_assign(&passive_income[d_idx]);
            net.add_assign(&diplomacy_income[d_idx]);

            self.ledger.record_source_ext(
                tick,
                &mut self.deities[d_idx].resources,
                d_id,
                ResourceSource::MortalDeaths,
                death_income[d_idx],
                None,
                is_player,
                "Ölümlü ruh akışı",
            );
            self.ledger.record_source_ext(
                tick,
                &mut self.deities[d_idx].resources,
                d_id,
                ResourceSource::FaithDevotion,
                devotion_income[d_idx],
                None,
                is_player,
                "İnanç ve tapınak akışı",
            );
            self.ledger.record_source_ext(
                tick,
                &mut self.deities[d_idx].resources,
                d_id,
                ResourceSource::UniversePassive,
                passive_income[d_idx],
                None,
                is_player,
                "Kozmik öz ve nüfuz akışı",
            );
            if diplomacy_income[d_idx] != Resources::ZERO {
                self.ledger.record_source_ext(
                    tick,
                    &mut self.deities[d_idx].resources,
                    d_id,
                    ResourceSource::DiplomacyTribute,
                    diplomacy_income[d_idx],
                    None,
                    is_player,
                    "Diplomatik pakt ticaret akışı",
                );
            }

            if collapse_loss[d_idx] != Resources::ZERO {
                let removed = self.ledger.record_removal_ext(
                    tick,
                    &mut self.deities[d_idx].resources,
                    d_id,
                    ResourceRemovalReason::UniverseCollapse,
                    collapse_loss[d_idx],
                    None,
                    is_player,
                    "Çöken evren ruh sızıntısı",
                );
                net.souls -= removed.souls;
                net.influence -= removed.influence;
            }
            if rebellion_loss[d_idx] != Resources::ZERO {
                let removed = self.ledger.record_removal_ext(
                    tick,
                    &mut self.deities[d_idx].resources,
                    d_id,
                    ResourceRemovalReason::RebellionSchism,
                    rebellion_loss[d_idx],
                    None,
                    is_player,
                    "İsyancı uygarlık mezhep kaybı",
                );
                net.souls -= removed.souls;
                net.influence -= removed.influence;
            }
            if upkeep_loss[d_idx] != Resources::ZERO {
                let removed = self.ledger.record_removal_ext(
                    tick,
                    &mut self.deities[d_idx].resources,
                    d_id,
                    ResourceRemovalReason::UpkeepDecay,
                    upkeep_loss[d_idx],
                    None,
                    is_player,
                    "İstikrarsız evren kozmik aşınma gideri",
                );
                net.souls -= removed.souls;
                net.influence -= removed.influence;
            }

            self.deities[d_idx].last_tick_income = net;
        }
    }

    fn phase_3_generate_cosmic_events(&mut self) {
        // Trigger random cosmic events at randomized intervals (every 2..=6 ticks)
        if self.tick < self.next_random_event_tick {
            return;
        }
        let interval = self.rng.range_u32(2, 6) as u64;
        self.next_random_event_tick = self.tick + interval;

        let u_len = self.universes.len();
        if u_len == 0 {
            return;
        }

        // Prefer discovered active universes 75% of the time so the player can see and interact with random events
        let discovered_indices: Vec<usize> = self
            .universes
            .iter()
            .enumerate()
            .filter(|(_, u)| !u.is_destroyed && u.discovered_by_player)
            .map(|(idx, _)| idx)
            .collect();

        let u_idx = if !discovered_indices.is_empty() && self.rng.chance(0.75) {
            discovered_indices[self.rng.range_usize(0, discovered_indices.len())]
        } else {
            self.rng.range_usize(0, u_len)
        };

        if self.universes[u_idx].is_destroyed {
            return;
        }

        let u_id = self.universes[u_idx].id;
        let u_name = self.universes[u_idx].name.clone();
        let discovered = self.universes[u_idx].discovered_by_player;
        self.last_event_universe_id = Some(u_id);
        let roll = self.rng.range_u32(0, 100);

        if roll < 14 {
            // 1. Golden Age Prophet (Altın Çağ Peygamberi)
            self.universes[u_idx].stability = (self.universes[u_idx].stability + 10.0).min(100.0);
            self.universes[u_idx].soul_reservoir += 65;
            for civ in &mut self.universes[u_idx].civilizations {
                civ.faith_fervor = (civ.faith_fervor + 18.0).min(100.0);
                civ.population = (civ.population + 45).min(50_000);
                civ.in_rebellion = false;
            }
            if let Some((dom_id, _)) = self.universes[u_idx].dominant_deity() {
                self.universes[u_idx].add_influence(dom_id, 12.0);
            }
            self.universes[u_idx].record_history(
                self.tick,
                HistoryCategory::DivineIntervention,
                None,
                "Ölümlüler arasında yükselen bir Altın Çağ Peygamberi tapınakları ve nüfusu coşturdu.",
            );
            if discovered {
                self.push_event(
                    Some(u_id),
                    None,
                    CosmicEventKind::GoldenAgeProphet,
                    EventSeverity::Positive,
                    format!("{u_name}: Altın Çağ Peygamberi"),
                    format!(
                        "{u_name} evreninde karizmatik bir peygamber doğdu; inanç (+18) ve ruh havuzu (+65) yükseldi."
                    ),
                );
                if self.active_dilemma.is_none() {
                    self.active_dilemma = Some(CosmicDilemma {
                        id: self.next_event_id,
                        universe_id: u_id,
                        title: format!("{u_name}: Peygamberin İlahi Vizyonu"),
                        description: format!(
                            "{u_name} peygamberi gökyüzüne seslenerek ilahi bir işaret talep ediyor."
                        ),
                        choice_a_label: "Peygamberi Kutsa (+65R +30N +5Ö +16 İstikrar)".to_string(),
                        choice_b_label: "Vecd Enerjisini Hasat Et (+105R +35Ö)".to_string(),
                    });
                }
            }
        } else if roll < 26 {
            // 2. Interstellar Migration (Yıldızlararası Göç Dalgası)
            self.universes[u_idx].stability = (self.universes[u_idx].stability + 8.0).min(100.0);
            self.universes[u_idx].soul_reservoir += 50;
            if self.universes[u_idx].civilizations.is_empty() {
                let cid = self.next_civ_id;
                self.next_civ_id += 1;
                let dom_id = self.universes[u_idx].dominant_deity().map(|(id, _)| id);
                self.universes[u_idx].civilizations.push(Civilization {
                    id: cid,
                    name: format!("Göçebe Koloni #{cid}"),
                    universe_id: u_id,
                    doctrine: CivilizationDoctrine::from_seed(u_id, cid),
                    population: 150,
                    tech_level: 35.0,
                    faith_fervor: 68.0,
                    stability: 78.0,
                    militarism: 22.0,
                    deity_awareness: 10.0,
                    patron_deity: dom_id,
                    religion_name: "Gezgin Yıldız Kültü".to_string(),
                    in_rebellion: false,
                    divine_shield_active: false,
                });
            } else {
                for civ in &mut self.universes[u_idx].civilizations {
                    civ.population = (civ.population + 85).min(50_000);
                }
            }
            self.universes[u_idx].record_history(
                self.tick,
                HistoryCategory::CivilizationRiseFall,
                None,
                "Geçitlerden akan yıldızlararası göç filoları evrenin nüfusunu ve ticaretini büyüttü.",
            );
            if discovered {
                self.push_event(
                    Some(u_id),
                    None,
                    CosmicEventKind::InterstellarMigration,
                    EventSeverity::Positive,
                    format!("{u_name}: Yıldızlararası Göç"),
                    format!(
                        "{u_name} evrenine ulaşan göçmen filolar nüfusu (+85M) ve ruh akışını güçlendirdi."
                    ),
                );
            }
        } else if roll < 38 {
            // 3. Singularity Experiment (Tekillik Deneyi)
            self.universes[u_idx].essence_richness =
                (self.universes[u_idx].essence_richness + 0.28).min(3.0);
            self.universes[u_idx].stability = (self.universes[u_idx].stability - 5.0).max(10.0);
            for civ in &mut self.universes[u_idx].civilizations {
                civ.tech_level = (civ.tech_level + 12.0).min(100.0);
            }
            self.universes[u_idx].record_history(
                self.tick,
                HistoryCategory::CosmicAnomaly,
                None,
                "Ölümlü bilginlerin gerçekleştirdiği kuantum tekillik deneyi gerçeklik dokusunda öz kristalleri oluşturdu.",
            );
            if discovered {
                self.push_event(
                    Some(u_id),
                    None,
                    CosmicEventKind::SingularityExperiment,
                    EventSeverity::Info,
                    format!("{u_name}: Tekillik Deneyi"),
                    format!(
                        "{u_name} bilginleri yapay bir tekillik yarattı (+0.28x Öz Zenginliği, +12 Teknoloji)."
                    ),
                );
                if self.active_dilemma.is_none() {
                    self.active_dilemma = Some(CosmicDilemma {
                        id: self.next_event_id,
                        universe_id: u_id,
                        title: format!("{u_name}: Kuantum Tekillik Çekirdeği"),
                        description: format!(
                            "{u_name} laboratuvarlarında dönen tekillik çekirdeği hem tehlikeli hem de saf öz dolu."
                        ),
                        choice_a_label: "Deneyi Kutsayıp İstikrara Kavuştur (+65R +30N +5Ö)".to_string(),
                        choice_b_label: "Tekilliği Soğur (+105R +35 Kozmik Öz)".to_string(),
                    });
                }
            }
        } else if roll < 50 {
            // 4. Cosmic Soul Storm (Kozmik Ruh Fırtınası)
            self.universes[u_idx].soul_reservoir += 110;
            self.universes[u_idx].soul_potential =
                (self.universes[u_idx].soul_potential + 0.15).min(3.0);
            if let Some((dom_id, _)) = self.universes[u_idx].dominant_deity() {
                let d_idx = dom_id as usize;
                self.ledger.record_source(
                    self.tick,
                    &mut self.deities[d_idx].resources,
                    dom_id,
                    ResourceSource::AnomalyDiscovery,
                    Resources::new(35, 0, 0),
                    Some(u_id),
                    format!("{u_name} kozmik ruh fırtınası"),
                );
            }
            self.universes[u_idx].record_history(
                self.tick,
                HistoryCategory::CosmicAnomaly,
                None,
                "Astral rüzgârların taşıdığı kozmik ruh fırtınası evrenin ruh havuzunu doldurdu.",
            );
            if discovered {
                self.push_event(
                    Some(u_id),
                    None,
                    CosmicEventKind::SoulStorm,
                    EventSeverity::Positive,
                    format!("{u_name}: Kozmik Ruh Fırtınası"),
                    format!(
                        "{u_name} üzerinden geçen astral fırtına +110 Ruh Havuzu ve +0.15x Ruh Potansiyeli bıraktı!"
                    ),
                );
            }
        } else if roll < 62 {
            // 5. Ancient Relic Discovered (Kadim Tanrı Kalıntısı)
            if let Some((dom_id, _)) = self.universes[u_idx].dominant_deity() {
                let d_idx = dom_id as usize;
                self.ledger.record_source(
                    self.tick,
                    &mut self.deities[d_idx].resources,
                    dom_id,
                    ResourceSource::AnomalyDiscovery,
                    Resources::new(25, 18, 18),
                    Some(u_id),
                    format!("{u_name} kadim kalıntı keşfi"),
                );
            }
            self.universes[u_idx].essence_richness =
                (self.universes[u_idx].essence_richness + 0.20).min(3.0);
            self.universes[u_idx].record_history(
                self.tick,
                HistoryCategory::AncientOrigin,
                None,
                "Evrenin derinliklerinde unutulmuş bir ilk çağ ilahına ait mühür ve öz kalıntısı bulundu.",
            );
            if discovered {
                self.push_event(
                    Some(u_id),
                    None,
                    CosmicEventKind::AncientRelicDiscovered,
                    EventSeverity::Positive,
                    format!("{u_name}: Kadim Tanrı Kalıntısı"),
                    format!(
                        "{u_name} evreninde bulunan kadim mühür baskın tanrıya +25 Ruh, +18 Nüfuz ve +18 Öz kazandırdı!"
                    ),
                );
                if self.active_dilemma.is_none() {
                    self.active_dilemma = Some(CosmicDilemma {
                        id: self.next_event_id,
                        universe_id: u_id,
                        title: format!("{u_name}: Kadim İlahın Mirası"),
                        description: format!(
                            "{u_name} yüzeyinde parlayan kadim anıt ölümlüleri büyülemeye devam ediyor."
                        ),
                        choice_a_label: "Anıtı Kutsal Tapınak Yap (+65R +30N +16 İstikrar)".to_string(),
                        choice_b_label: "Kalıntıyı Eritip Öz Çıkar (+105R +35Ö)".to_string(),
                    });
                }
            }
        } else if roll < 76 {
            // 6. Cosmic Reality Anomaly (Boyutsal Anomali)
            self.universes[u_idx].essence_richness =
                (self.universes[u_idx].essence_richness + 0.35).min(3.0);
            self.universes[u_idx].soul_reservoir += 60;
            if let Some((dom_id, _)) = self.universes[u_idx].dominant_deity() {
                let d_idx = dom_id as usize;
                self.ledger.record_source(
                    self.tick,
                    &mut self.deities[d_idx].resources,
                    dom_id,
                    ResourceSource::AnomalyDiscovery,
                    Resources::new(25, 0, 15),
                    Some(u_id),
                    format!("{u_name} kozmik anomali keşfi"),
                );
            }
            self.universes[u_idx].record_history(
                self.tick,
                HistoryCategory::CosmicAnomaly,
                None,
                "Boyutlar arası bir anomali evrenin kozmik öz yoğunluğunu artırdı.",
            );
            if discovered {
                self.push_event(
                    Some(u_id),
                    None,
                    CosmicEventKind::RealityAnomaly,
                    EventSeverity::Positive,
                    format!("{u_name}: Kozmik Anomali"),
                    format!(
                        "{u_name} evreninde parlayan bir anomali Kozmik Öz zenginliğini artırdı."
                    ),
                );
                if self.active_dilemma.is_none() {
                    self.active_dilemma = Some(CosmicDilemma {
                        id: self.next_event_id,
                        universe_id: u_id,
                        title: format!("{u_name}: Boyutsal Yarığın Çağrısı"),
                        description: format!(
                            "{u_name} evreninde açılan kozmik yarık ölümlüleri korkutuyor fakat saf öz saçıyor."
                        ),
                        choice_a_label: "Kutsama ile Yatıştır (+65R +30N +16 İstikrar)".to_string(),
                        choice_b_label: "Yarığı Sömür (+105R +35 Kozmik Öz)".to_string(),
                    });
                }
            }
        } else if roll < 90 && !self.universes[u_idx].civilizations.is_empty() {
            // 7. Inter-civilization war or conflict -> spike in mortal deaths & soul reservoir
            let war_souls = self.rng.range_u32(45, 130) as i64;
            self.universes[u_idx].soul_reservoir += war_souls;
            self.universes[u_idx].stability = (self.universes[u_idx].stability - 6.0).max(10.0);
            for civ in &mut self.universes[u_idx].civilizations {
                civ.population = civ.population.saturating_sub(12);
            }
            self.universes[u_idx].record_history(
                self.tick,
                HistoryCategory::CivilizationRiseFall,
                None,
                format!("Büyük bir yıkım savaşı yaşandı; {war_souls} ruh kozmik havuza karıştı."),
            );
            if discovered {
                self.push_event(
                    Some(u_id),
                    None,
                    CosmicEventKind::CivilizationWar,
                    EventSeverity::Info,
                    format!("{u_name}: Uygarlıklar Savaşı"),
                    format!("{u_name} evreninde patlak veren savaş {war_souls} yeni ruhu serbest bıraktı."),
                );
            }
        } else {
            // 8. Religious schism
            if let Some((dom_id, _)) = self.universes[u_idx].dominant_deity() {
                self.universes[u_idx].add_influence(dom_id, -10.0);
                self.universes[u_idx].record_history(
                    self.tick,
                    HistoryCategory::ReligiousConflict,
                    Some(dom_id),
                    "Mezhep ayrılığı baskın inancın gücünü sarsarak yeni kültler doğurdu.",
                );
                if discovered {
                    self.push_event(
                        Some(u_id),
                        Some(dom_id),
                        CosmicEventKind::ReligiousSchism,
                        EventSeverity::Warning,
                        format!("{u_name}: Mezhep Ayrılığı"),
                        format!("{u_name} evreninde sapkın kültler baskın nüfuzu zayıflattı."),
                    );
                }
            }
        }
    }

    fn phase_4_run_rival_ai_decisions(&mut self) {
        // Each rival deity acts according to its archetype every 2 ticks
        let deity_count = self.deities.len();
        let (_, full_counts) = self.all_deity_universe_counts();
        let (p_dom_ratio, p_trans_ratio, p_cre_ratio) =
            self.victory_progress_ratios(PLAYER_DEITY_ID);
        let player_nearing_victory =
            p_dom_ratio >= 0.65 || p_trans_ratio >= 0.65 || p_cre_ratio >= 0.65;

        for d_idx in 1..deity_count {
            if self.deities[d_idx].is_sealed {
                continue;
            }
            let deity_id = self.deities[d_idx].id;
            if !(self.tick + deity_id as u64).is_multiple_of(2) {
                continue;
            }

            // 1. Try unlocking an affordable power aligned with archetype
            let preferred_tree = match self.deities[d_idx].archetype {
                DeityArchetype::Conqueror | DeityArchetype::Voidbringer => {
                    crate::deity::PowerTree::DeityWarfare
                }
                DeityArchetype::Harvester => crate::deity::PowerTree::SoulHarvest,
                DeityArchetype::Preserver | DeityArchetype::Architect => {
                    crate::deity::PowerTree::Creation
                }
                DeityArchetype::Zealot => crate::deity::PowerTree::FaithDominion,
                DeityArchetype::Technocrat | DeityArchetype::Schemer => {
                    crate::deity::PowerTree::CosmicTraversal
                }
                DeityArchetype::AscendantPlayer => crate::deity::PowerTree::FaithDominion,
            };

            let candidate_power = PowerId::ALL
                .into_iter()
                .find(|p| p.tree() == preferred_tree && self.deities[d_idx].can_unlock_power(*p))
                .or_else(|| {
                    PowerId::ALL
                        .into_iter()
                        .find(|p| self.deities[d_idx].can_unlock_power(*p))
                });
            if let Some(power) = candidate_power {
                let _ = self.execute_action(deity_id, DivineAction::UnlockPower { power });
                continue;
            }

            // 2. If rich enough and prerequisites met, advance Ascension Ritual
            let ai_full_allegiance = full_counts.get(d_idx).copied().unwrap_or(0);
            if self.deities[d_idx].unlocked_powers.len() >= TRANSCENDENCE_MIN_POWERS_TO_START
                && ai_full_allegiance >= TRANSCENDENCE_MIN_FULL_ALLEGIANCE_TO_START
                && self.deities[d_idx]
                    .resources
                    .can_afford(&Resources::new(720, 260, 190))
            {
                let _ = self.execute_action(deity_id, DivineAction::AdvanceAscensionRitual);
                continue;
            }

            // 3. Endgame Escalation: if Player is nearing victory, hostile rivals target player universes!
            let u_len = self.universes.len();
            if u_len == 0 {
                continue;
            }
            let target_u = if player_nearing_victory && self.rng.chance(0.55) {
                self.universes
                    .iter()
                    .find(|u| !u.is_destroyed && u.is_dominated_by(PLAYER_DEITY_ID))
                    .map(|u| u.id)
                    .unwrap_or_else(|| self.rng.range_usize(0, u_len) as u32)
            } else {
                self.rng.range_usize(0, u_len) as u32
            };
            if self.universes[target_u as usize].is_destroyed {
                continue;
            }

            let archetype = self.deities[d_idx].archetype;
            let action = match archetype {
                DeityArchetype::Conqueror => {
                    if let Some((dom_id, _)) = self.universes[target_u as usize].dominant_deity() {
                        if dom_id != deity_id
                            && !matches!(
                                self.deities[d_idx].stance_toward(dom_id),
                                DiplomaticStance::Pact { .. }
                            )
                        {
                            DivineAction::SmiteRivalInfluence {
                                universe_id: target_u,
                                target_deity_id: dom_id,
                            }
                        } else {
                            DivineAction::SpreadProphecy {
                                universe_id: target_u,
                            }
                        }
                    } else {
                        DivineAction::SpreadProphecy {
                            universe_id: target_u,
                        }
                    }
                }
                DeityArchetype::Harvester | DeityArchetype::Voidbringer => {
                    if self.rng.chance(0.5) {
                        DivineAction::HarvestSouls {
                            universe_id: target_u,
                        }
                    } else {
                        DivineAction::ExtractCosmicEssence {
                            universe_id: target_u,
                        }
                    }
                }
                DeityArchetype::Preserver => DivineAction::BestowMiracle {
                    universe_id: target_u,
                },
                DeityArchetype::Architect => {
                    if self.universes[target_u as usize].stability < 50.0 {
                        DivineAction::StabilizeReality {
                            universe_id: target_u,
                        }
                    } else {
                        DivineAction::SeedLifeAndOrder {
                            universe_id: target_u,
                        }
                    }
                }
                DeityArchetype::Zealot => DivineAction::SpreadProphecy {
                    universe_id: target_u,
                },
                DeityArchetype::Technocrat => {
                    if let Some(gw) = self.universes[target_u as usize].gateways.first() {
                        DivineAction::OpenGateway {
                            from_universe_id: target_u,
                            to_universe_id: gw.target_universe_id,
                        }
                    } else {
                        DivineAction::SpreadProphecy {
                            universe_id: target_u,
                        }
                    }
                }
                DeityArchetype::Schemer => DivineAction::InciteRebellion {
                    universe_id: target_u,
                },
                DeityArchetype::AscendantPlayer => DivineAction::SpreadProphecy {
                    universe_id: target_u,
                },
            };

            let action = if let Some(req_u) = action.requires_dominion_in() {
                if self.universes[req_u as usize].is_dominated_by(deity_id) {
                    action
                } else {
                    DivineAction::SpreadProphecy {
                        universe_id: target_u,
                    }
                }
            } else {
                action
            };

            let _ = self.execute_action(deity_id, action);
        }
    }
}
