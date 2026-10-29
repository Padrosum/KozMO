use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum CosmicEventKind {
    CivilizationWar,
    ReligiousSchism,
    CivilizationRebellion,
    RealityAnomaly,
    UniverseCollapseWarning,
    UniverseCollapsed,
    ScientificAwakening,
    ProphecyFulfilled,
    NewCivilizationBorn,
    DivineIntervention,
    GatewayOpened,
    DeitySealed,
    AscensionMilestone,
    GoldenAgeProphet,
    InterstellarMigration,
    SingularityExperiment,
    SoulStorm,
    AncientRelicDiscovered,
}

impl CosmicEventKind {
    pub fn symbol_tag(self) -> &'static str {
        match self {
            Self::CivilizationWar => "[SAVAŞ]",
            Self::ReligiousSchism => "[AYRILIK]",
            Self::CivilizationRebellion => "[İSYAN]",
            Self::RealityAnomaly => "[ANOMALİ]",
            Self::UniverseCollapseWarning => "[TEHLİKE]",
            Self::UniverseCollapsed => "[ÇÖKÜŞ]",
            Self::ScientificAwakening => "[BİLİM]",
            Self::ProphecyFulfilled => "[KEHANET]",
            Self::NewCivilizationBorn => "[DOĞUŞ]",
            Self::DivineIntervention => "[MÜDAHALE]",
            Self::GatewayOpened => "[GEÇİT]",
            Self::DeitySealed => "[MÜHÜR]",
            Self::AscensionMilestone => "[YÜKSELİŞ]",
            Self::GoldenAgeProphet => "[ALTIN ÇAĞ]",
            Self::InterstellarMigration => "[GÖÇ]",
            Self::SingularityExperiment => "[TEKİLLİK]",
            Self::SoulStorm => "[FIRTINA]",
            Self::AncientRelicDiscovered => "[KALINTI]",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum EventSeverity {
    Info,
    Positive,
    Warning,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CosmicEvent {
    pub id: u64,
    pub tick: u64,
    pub universe_id: Option<u32>,
    pub deity_id: Option<u32>,
    pub kind: CosmicEventKind,
    pub severity: EventSeverity,
    pub title: String,
    pub description: String,
}
