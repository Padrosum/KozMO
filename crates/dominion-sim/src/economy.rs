use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ResourceType {
    Souls,
    Influence,
    CosmicEssence,
}

impl ResourceType {
    pub fn label_tr(self) -> &'static str {
        match self {
            Self::Souls => "Ruh",
            Self::Influence => "Nüfuz",
            Self::CosmicEssence => "Kozmik Öz",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Resources {
    pub souls: i64,
    pub influence: i64,
    pub cosmic_essence: i64,
}

impl Resources {
    pub const ZERO: Self = Self {
        souls: 0,
        influence: 0,
        cosmic_essence: 0,
    };

    pub const fn new(souls: i64, influence: i64, cosmic_essence: i64) -> Self {
        Self {
            souls,
            influence,
            cosmic_essence,
        }
    }

    pub fn get(&self, kind: ResourceType) -> i64 {
        match kind {
            ResourceType::Souls => self.souls,
            ResourceType::Influence => self.influence,
            ResourceType::CosmicEssence => self.cosmic_essence,
        }
    }

    pub fn set(&mut self, kind: ResourceType, value: i64) {
        match kind {
            ResourceType::Souls => self.souls = value.max(0),
            ResourceType::Influence => self.influence = value.max(0),
            ResourceType::CosmicEssence => self.cosmic_essence = value.max(0),
        }
    }

    pub fn can_afford(&self, cost: &Resources) -> bool {
        self.souls >= cost.souls
            && self.influence >= cost.influence
            && self.cosmic_essence >= cost.cosmic_essence
    }

    pub fn add_assign(&mut self, delta: &Resources) {
        self.souls = self.souls.saturating_add(delta.souls).max(0);
        self.influence = self.influence.saturating_add(delta.influence).max(0);
        self.cosmic_essence = self
            .cosmic_essence
            .saturating_add(delta.cosmic_essence)
            .max(0);
    }

    pub fn sub_clamped(&mut self, amount: &Resources) -> Resources {
        let d_souls = amount.souls.min(self.souls).max(0);
        let d_inf = amount.influence.min(self.influence).max(0);
        let d_ess = amount.cosmic_essence.min(self.cosmic_essence).max(0);
        self.souls -= d_souls;
        self.influence -= d_inf;
        self.cosmic_essence -= d_ess;
        Resources::new(d_souls, d_inf, d_ess)
    }

    pub fn total_power_score(&self) -> i64 {
        self.souls + self.influence * 2 + self.cosmic_essence * 5
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ResourceSource {
    InitialGrant,
    MortalDeaths,
    FaithDevotion,
    UniversePassive,
    DivineHarvest,
    AnomalyDiscovery,
    DiplomacyTribute,
}

impl ResourceSource {
    pub fn label_tr(self) -> &'static str {
        match self {
            Self::InitialGrant => "Başlangıç Özü",
            Self::MortalDeaths => "Ölümlü Kayıpları",
            Self::FaithDevotion => "İnanç ve İbadet",
            Self::UniversePassive => "Evren Akışı",
            Self::DivineHarvest => "Tanrısal Hasat",
            Self::AnomalyDiscovery => "Kozmik Anomali",
            Self::DiplomacyTribute => "Diplomatik Haraç",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ResourceExpenditure {
    DivineAction,
    PowerUnlock,
    GatewayOperation,
    UniverseCreation,
    DiplomacyPact,
    AscensionRitual,
}

impl ResourceExpenditure {
    pub fn label_tr(self) -> &'static str {
        match self {
            Self::DivineAction => "Tanrısal Müdahale",
            Self::PowerUnlock => "Güç Ağacı Gelişimi",
            Self::GatewayOperation => "Geçit Müdahalesi",
            Self::UniverseCreation => "Evren Şekillendirme",
            Self::DiplomacyPact => "Diplomasi Anlaşması",
            Self::AscensionRitual => "Aşkınlık Ritüeli",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ResourceRemovalReason {
    UniverseCollapse,
    RivalSabotage,
    RebellionSchism,
    UpkeepDecay,
}

impl ResourceRemovalReason {
    pub fn label_tr(self) -> &'static str {
        match self {
            Self::UniverseCollapse => "Evren Çöküşü",
            Self::RivalSabotage => "Rakip Tanrı Saldırısı",
            Self::RebellionSchism => "İsyan ve Mezhep Bölünmesi",
            Self::UpkeepDecay => "Kozmik Aşınma",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LedgerOperation {
    Source(ResourceSource),
    Expenditure(ResourceExpenditure),
    Transfer { from_deity: u32, to_deity: u32 },
    Removal(ResourceRemovalReason),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub tick: u64,
    pub deity_id: u32,
    pub operation: LedgerOperation,
    pub delta: Resources,
    pub universe_id: Option<u32>,
    pub description: String,
}

/// Tracks the source, expenditure, transfer, and removal of all resources across the simulation
/// as required by Eternal Dominion architecture guidelines.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceLedger {
    pub totals_by_source: BTreeMap<ResourceSource, Resources>,
    pub totals_by_expenditure: BTreeMap<ResourceExpenditure, Resources>,
    pub total_transferred: Resources,
    pub totals_by_removal: BTreeMap<ResourceRemovalReason, Resources>,
    pub recent_entries: Vec<LedgerEntry>,
    pub max_recent_entries: usize,
}

impl ResourceLedger {
    pub fn new(max_recent_entries: usize) -> Self {
        Self {
            totals_by_source: BTreeMap::new(),
            totals_by_expenditure: BTreeMap::new(),
            total_transferred: Resources::ZERO,
            totals_by_removal: BTreeMap::new(),
            recent_entries: Vec::new(),
            max_recent_entries: max_recent_entries.max(32),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn record_source(
        &mut self,
        tick: u64,
        deity: &mut Resources,
        deity_id: u32,
        source: ResourceSource,
        amount: Resources,
        universe_id: Option<u32>,
        description: impl Into<String>,
    ) {
        self.record_source_ext(
            tick,
            deity,
            deity_id,
            source,
            amount,
            universe_id,
            true,
            description,
        );
    }

    /// Records a resource source into global ledger totals and optionally appends a `LedgerEntry`
    /// to `recent_entries` (used to avoid per-tick heap allocations for routine AI background ticks).
    #[allow(clippy::too_many_arguments)]
    pub fn record_source_ext(
        &mut self,
        tick: u64,
        deity: &mut Resources,
        deity_id: u32,
        source: ResourceSource,
        amount: Resources,
        universe_id: Option<u32>,
        log_recent: bool,
        description: impl Into<String>,
    ) {
        if amount == Resources::ZERO {
            return;
        }
        deity.add_assign(&amount);
        let entry_tot = self
            .totals_by_source
            .entry(source)
            .or_insert(Resources::ZERO);
        entry_tot.add_assign(&amount);
        if log_recent {
            self.push_entry(LedgerEntry {
                tick,
                deity_id,
                operation: LedgerOperation::Source(source),
                delta: amount,
                universe_id,
                description: description.into(),
            });
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn try_record_expenditure(
        &mut self,
        tick: u64,
        deity: &mut Resources,
        deity_id: u32,
        expenditure: ResourceExpenditure,
        cost: Resources,
        universe_id: Option<u32>,
        description: impl Into<String>,
    ) -> bool {
        if !deity.can_afford(&cost) {
            return false;
        }
        let spent = deity.sub_clamped(&cost);
        let entry_tot = self
            .totals_by_expenditure
            .entry(expenditure)
            .or_insert(Resources::ZERO);
        entry_tot.add_assign(&spent);
        self.push_entry(LedgerEntry {
            tick,
            deity_id,
            operation: LedgerOperation::Expenditure(expenditure),
            delta: spent,
            universe_id,
            description: description.into(),
        });
        true
    }

    #[allow(clippy::too_many_arguments)]
    pub fn record_transfer(
        &mut self,
        tick: u64,
        from_res: &mut Resources,
        from_deity: u32,
        to_res: &mut Resources,
        to_deity: u32,
        amount: Resources,
        description: impl Into<String>,
    ) -> Resources {
        let transferred = from_res.sub_clamped(&amount);
        if transferred == Resources::ZERO {
            return Resources::ZERO;
        }
        to_res.add_assign(&transferred);
        self.total_transferred.add_assign(&transferred);
        self.push_entry(LedgerEntry {
            tick,
            deity_id: from_deity,
            operation: LedgerOperation::Transfer {
                from_deity,
                to_deity,
            },
            delta: transferred,
            universe_id: None,
            description: description.into(),
        });
        transferred
    }

    #[allow(clippy::too_many_arguments)]
    pub fn record_removal(
        &mut self,
        tick: u64,
        deity: &mut Resources,
        deity_id: u32,
        reason: ResourceRemovalReason,
        amount: Resources,
        universe_id: Option<u32>,
        description: impl Into<String>,
    ) -> Resources {
        self.record_removal_ext(
            tick,
            deity,
            deity_id,
            reason,
            amount,
            universe_id,
            true,
            description,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn record_removal_ext(
        &mut self,
        tick: u64,
        deity: &mut Resources,
        deity_id: u32,
        reason: ResourceRemovalReason,
        amount: Resources,
        universe_id: Option<u32>,
        log_recent: bool,
        description: impl Into<String>,
    ) -> Resources {
        let removed = deity.sub_clamped(&amount);
        if removed == Resources::ZERO {
            return Resources::ZERO;
        }
        let entry_tot = self
            .totals_by_removal
            .entry(reason)
            .or_insert(Resources::ZERO);
        entry_tot.add_assign(&removed);
        if log_recent {
            self.push_entry(LedgerEntry {
                tick,
                deity_id,
                operation: LedgerOperation::Removal(reason),
                delta: removed,
                universe_id,
                description: description.into(),
            });
        }
        removed
    }

    fn push_entry(&mut self, entry: LedgerEntry) {
        self.recent_entries.push(entry);
        let cap = self.max_recent_entries.max(32);
        // Amortized batch compaction: only shift the vector when it exceeds cap + 64
        if self.recent_entries.len() > cap + 64 {
            let overflow = self.recent_entries.len() - cap;
            self.recent_entries.drain(0..overflow);
        }
    }
}
