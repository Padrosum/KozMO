use crate::civilization::Civilization;
use crate::economy::Resources;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Detailed per-turn economic yield and drain breakdown for a single universe.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct UniverseEconomyBreakdown {
    pub reservoir_souls: i64,
    pub devotion_income: Resources,
    pub passive_income: Resources,
    pub collapse_loss: Resources,
    pub rebellion_loss: Resources,
    pub upkeep_loss: Resources,
}

impl UniverseEconomyBreakdown {
    pub fn gross_income(&self) -> Resources {
        Resources::new(
            self.reservoir_souls + self.devotion_income.souls + self.passive_income.souls,
            self.devotion_income.influence + self.passive_income.influence,
            self.devotion_income.cosmic_essence + self.passive_income.cosmic_essence,
        )
    }

    pub fn total_loss(&self) -> Resources {
        Resources::new(
            self.collapse_loss.souls + self.rebellion_loss.souls + self.upkeep_loss.souls,
            self.collapse_loss.influence
                + self.rebellion_loss.influence
                + self.upkeep_loss.influence,
            self.collapse_loss.cosmic_essence
                + self.rebellion_loss.cosmic_essence
                + self.upkeep_loss.cosmic_essence,
        )
    }

    pub fn net_income(&self) -> Resources {
        let g = self.gross_income();
        let l = self.total_loss();
        Resources::new(
            g.souls - l.souls,
            g.influence - l.influence,
            g.cosmic_essence - l.cosmic_essence,
        )
    }
}

/// The six distinct universe categories defined in GDD v0.2 (`taslak.md`) and `TASARIM.md`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum UniverseType {
    /// 1. Yaşam evrenleri: High civilization density, strong soul potential, complex politics.
    Life,
    /// 2. Kaos evrenleri: Frequent wars, unstable reality, unpredictable events, high reward/risk.
    Chaos,
    /// 3. İnanç evrenleri: Strong religions, prophecies, intense devotion, major soul flow centers.
    Faith,
    /// 4. Teknoloji evrenleri: Advanced civilizations, scientific progress, resistance to divine manipulation.
    Tech,
    /// 5. İlksel evrenler: Undeveloped realities where life can be created and physics shaped.
    Primordial,
    /// 6. Çökmekte olan evrenler: Corrupted reality, souls can be lost; can be saved, exploited, or destroyed.
    Collapsing,
}

impl UniverseType {
    pub const ALL: [Self; 6] = [
        Self::Life,
        Self::Chaos,
        Self::Faith,
        Self::Tech,
        Self::Primordial,
        Self::Collapsing,
    ];

    pub fn name_tr(self) -> &'static str {
        match self {
            Self::Life => "Yaşam Evreni",
            Self::Chaos => "Kaos Evreni",
            Self::Faith => "İnanç Evreni",
            Self::Tech => "Teknoloji Evreni",
            Self::Primordial => "İlksel Evren",
            Self::Collapsing => "Çöken Evren",
        }
    }

    pub fn short_tag_tr(self) -> &'static str {
        match self {
            Self::Life => "YAŞAM",
            Self::Chaos => "KAOS",
            Self::Faith => "İNANÇ",
            Self::Tech => "TEKNOLOJİ",
            Self::Primordial => "İLKSEL",
            Self::Collapsing => "ÇÖKÜŞ",
        }
    }

    pub fn svg_asset_path(self) -> &'static str {
        match self {
            Self::Life => "art/universes/universe_life.svg",
            Self::Chaos => "art/universes/universe_chaos.svg",
            Self::Faith => "art/universes/universe_faith.svg",
            Self::Tech => "art/universes/universe_tech.svg",
            Self::Primordial => "art/universes/universe_primordial.svg",
            Self::Collapsing => "art/universes/universe_collapsing.svg",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GatewayState {
    Closed,
    Open,
    Locked { owner_deity: u32 },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayConnection {
    pub target_universe_id: u32,
    pub state: GatewayState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HistoryCategory {
    AncientOrigin,
    DivineIntervention,
    CivilizationRiseFall,
    ReligiousConflict,
    CosmicAnomaly,
    GatewayChange,
}

impl HistoryCategory {
    pub fn label_tr(self) -> &'static str {
        match self {
            Self::AncientOrigin => "Kadim Köken",
            Self::DivineIntervention => "İlahi Müdahale",
            Self::CivilizationRiseFall => "Uygarlık Tarihi",
            Self::ReligiousConflict => "İnanç Çatışması",
            Self::CosmicAnomaly => "Kozmik Anomali",
            Self::GatewayChange => "Geçit Değişimi",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UniverseHistoryEntry {
    pub tick: u64,
    pub category: HistoryCategory,
    pub description: String,
    pub deity_id: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Universe {
    pub id: u32,
    pub name: String,
    pub universe_type: UniverseType,
    /// Cosmic 2D map coordinates.
    pub position: [f32; 2],
    /// Distance ring from starting cosmic region (0 = starting core, 1..=3 = outer rings).
    pub region_tier: u8,
    /// Whether the player deity has discovered this universe on the cosmic map.
    pub discovered_by_player: bool,
    /// Stability of physical laws and reality (0.0 to 100.0).
    pub stability: f32,
    /// Danger level (0.0 to 100.0).
    pub danger_level: f32,
    /// Soul flow multiplier (0.5 to 3.0).
    pub soul_potential: f32,
    /// Cosmic essence richness (0.2 to 3.0).
    pub essence_richness: f32,
    /// Unclaimed souls lingering in the cosmic veil of this universe.
    pub soul_reservoir: i64,
    /// Influence exerted by each deity (deity_id -> 0.0..100.0).
    pub deity_influence: BTreeMap<u32, f32>,
    /// Active mortal civilizations in this universe.
    pub civilizations: Vec<Civilization>,
    /// Cosmic gateways linking to adjacent universes.
    pub gateways: Vec<GatewayConnection>,
    /// Persistent historical record of this universe.
    pub history: Vec<UniverseHistoryEntry>,
    /// If awakened/created/stabilized into a harmonious order by a deity, records that deity's ID.
    pub shaped_by_deity: Option<u32>,
    /// Whether this universe has completely collapsed into the void.
    pub is_destroyed: bool,
}

impl Universe {
    /// Returns the deity with the highest influence (if >= 15.0) and their influence value.
    pub fn dominant_deity(&self) -> Option<(u32, f32)> {
        self.deity_influence
            .iter()
            .filter(|(_, &val)| val >= 15.0)
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(&id, &val)| (id, val))
    }

    /// Returns true if `deity_id` is the dominant deity of this universe.
    pub fn is_dominated_by(&self, deity_id: u32) -> bool {
        self.dominant_deity().is_some_and(|(id, _)| id == deity_id)
    }

    /// Returns true if `deity_id` has full allegiance (Tam Aidiyet: dominant and >= 65% influence).
    pub fn has_full_allegiance(&self, deity_id: u32) -> bool {
        self.dominant_deity()
            .is_some_and(|(id, inf)| id == deity_id && inf >= 65.0)
    }

    pub fn influence_of(&self, deity_id: u32) -> f32 {
        self.deity_influence.get(&deity_id).copied().unwrap_or(0.0)
    }

    pub fn add_influence(&mut self, deity_id: u32, delta: f32) {
        let current = self.influence_of(deity_id);
        let updated = (current + delta).clamp(0.0, 100.0);
        if updated <= 0.01 {
            self.deity_influence.remove(&deity_id);
        } else {
            self.deity_influence.insert(deity_id, updated);
        }

        // Normalize if total influence across deities exceeds 160.0 so rivals compete for space
        let total: f32 = self.deity_influence.values().sum();
        if total > 160.0 {
            let scale = 160.0 / total;
            for val in self.deity_influence.values_mut() {
                *val = (*val * scale).clamp(0.0, 100.0);
            }
        }
    }

    pub fn total_population(&self) -> u64 {
        self.civilizations.iter().map(|c| c.population).sum()
    }

    pub fn life_density(&self) -> f32 {
        let pop_score = (self.total_population() as f32 / 120.0).min(70.0);
        let civ_score = (self.civilizations.len() as f32) * 10.0;
        (pop_score + civ_score).clamp(0.0, 100.0)
    }

    pub fn has_open_gateway_to(&self, other_id: u32) -> bool {
        self.gateways
            .iter()
            .any(|g| g.target_universe_id == other_id && matches!(g.state, GatewayState::Open))
    }

    pub fn record_history(
        &mut self,
        tick: u64,
        category: HistoryCategory,
        deity_id: Option<u32>,
        description: impl Into<String>,
    ) {
        self.history.push(UniverseHistoryEntry {
            tick,
            category,
            description: description.into(),
            deity_id,
        });
        if self.history.len() > 40 {
            let overflow = self.history.len() - 40;
            self.history.drain(0..overflow);
        }
    }

    /// Computes the logical per-tick economic yield and drain of this universe for its dominant deity.
    pub fn estimate_economy_breakdown(
        &self,
        tick: u64,
        is_focused: bool,
        deity: &crate::deity::Deity,
    ) -> Option<(u32, i64, UniverseEconomyBreakdown)> {
        if self.is_destroyed || deity.is_sealed {
            return None;
        }
        let (dom_id, dom_inf) = self.dominant_deity()?;
        if dom_id != deity.id {
            return None;
        }
        let is_full_allegiance = dom_inf >= 65.0;

        let has_siphon = deity.has_power(crate::deity::PowerId::SoulSiphon);
        let has_dogma = deity.has_power(crate::deity::PowerId::AbsoluteDogma);
        let has_condenser = deity.has_power(crate::deity::PowerId::EssenceCondenser);
        let has_forge = deity.has_power(crate::deity::PowerId::CosmicForge);
        let has_gw_mastery = deity.has_power(crate::deity::PowerId::GatewayMastery);
        let has_ward = deity.has_power(crate::deity::PowerId::DimensionalWard);
        let has_prophecy_voice = deity.has_power(crate::deity::PowerId::VoiceOfProphecy);

        // Tam Aidiyet (★ >= 65% influence) grants a +30% devotion & passive influence bonus
        let allegiance_mult = if is_full_allegiance { 1.30 } else { 1.0 };
        let focus_mult = if is_focused { 1.25 } else { 1.0 };
        let siphon_mult = if has_siphon { 1.40 } else { 1.0 };
        let dogma_mult = (if has_dogma { 1.45 } else { 1.0 }) * focus_mult * allegiance_mult;

        // 1. Soul Reservoir stream (higher in Chaos/Collapsing universes)
        let reservoir_rate = match self.universe_type {
            UniverseType::Chaos | UniverseType::Collapsing => 0.16,
            _ => 0.12,
        };
        let reservoir_take = if self.soul_reservoir > 0 {
            ((self.soul_reservoir as f32 * reservoir_rate).round() as i64)
                .clamp(1, self.soul_reservoir)
        } else {
            0
        };
        let reservoir_souls = (reservoir_take as f32 * siphon_mult).round() as i64;

        // 2. Civilization Devotion & Rebellion Schism Drain
        let mut devotion_income = Resources::ZERO;
        let mut rebellion_loss = Resources::ZERO;

        let life_soul_bonus = match self.universe_type {
            UniverseType::Life => 1.25,
            UniverseType::Faith => 1.15,
            _ => 1.0,
        };
        let faith_inf_bonus = match self.universe_type {
            UniverseType::Faith => 1.35,
            UniverseType::Life => 1.10,
            _ => 1.0,
        };

        for civ in &self.civilizations {
            if civ.in_rebellion {
                // Active rebellion drains Souls & Influence through religious schisms
                rebellion_loss.souls += if has_ward { 1 } else { 3 };
                rebellion_loss.influence += if has_ward { 1 } else { 2 };
            } else if civ.patron_deity == Some(dom_id) {
                let pop_f = (civ.population as f32).sqrt() * 0.44;
                let faith_f = (civ.faith_fervor / 100.0).max(0.15);
                let shield_f = if civ.divine_shield_active { 0.40 } else { 1.0 };
                let souls = (pop_f
                    * faith_f
                    * self.soul_potential
                    * dogma_mult
                    * life_soul_bonus
                    * shield_f)
                    .round() as i64;
                let inf = ((pop_f * 0.36 * faith_f * dogma_mult * faith_inf_bonus * shield_f)
                    .round() as i64)
                    .max(1);
                devotion_income.souls += souls.max(2);
                devotion_income.influence += inf;
            }
        }

        // 3. Passive Universe Influence & Cosmic Essence by Universe Specialization & Unlocked Powers
        let mut passive_income = Resources::ZERO;
        let base_inf = (((dom_inf / 24.0) * allegiance_mult).round() as i64).max(1);
        passive_income.influence += base_inf;
        if has_prophecy_voice {
            passive_income.influence += 1;
        }

        // CosmicForge bonus for shaped universes
        if has_forge && self.shaped_by_deity == Some(dom_id) {
            passive_income.souls += 4;
            passive_income.influence += 3;
            passive_income.cosmic_essence += 2;
        }

        // GatewayMastery bonus for open gateways
        if has_gw_mastery {
            let open_gws = self
                .gateways
                .iter()
                .filter(|g| matches!(g.state, GatewayState::Open))
                .count() as i64;
            passive_income.influence += open_gws.min(3);
        }

        // Enlightened Technocracy bonus in Tech universes with Full Allegiance (★)
        if self.universe_type == UniverseType::Tech && is_full_allegiance {
            passive_income.influence += 2;
            passive_income.cosmic_essence += 1;
        }

        let base_ess = if matches!(
            self.universe_type,
            UniverseType::Primordial | UniverseType::Chaos | UniverseType::Collapsing
        ) || has_condenser
        {
            let mult = match (self.universe_type, has_condenser) {
                (UniverseType::Primordial, true) => 1.85,
                (UniverseType::Primordial, false) => 1.05,
                (_, true) => 1.55,
                (_, false) => 0.85,
            };
            ((self.essence_richness * mult).round() as i64).max(1)
        } else if tick.is_multiple_of(3) {
            1
        } else {
            0
        };
        passive_income.cosmic_essence += base_ess;

        // 4. Collapsing Leak & Low-Stability Cosmic Upkeep (reduced by 75% if DimensionalWard is unlocked!)
        let mut collapse_loss = Resources::ZERO;
        let mut upkeep_loss = Resources::ZERO;
        if self.universe_type == UniverseType::Collapsing {
            collapse_loss.souls += if has_ward { 1 } else { 5 };
            collapse_loss.influence += if has_ward { 0 } else { 1 };
        } else if self.stability < 35.0 {
            upkeep_loss.souls += if has_ward { 0 } else { 2 };
            upkeep_loss.influence += if has_ward { 0 } else { 1 };
        }

        Some((
            dom_id,
            reservoir_take,
            UniverseEconomyBreakdown {
                reservoir_souls,
                devotion_income,
                passive_income,
                collapse_loss,
                rebellion_loss,
                upkeep_loss,
            },
        ))
    }
}
