use crate::rng::DeterministicRng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CivilizationDoctrine {
    /// Uyumcu Filozoflar: +İstikrar direnci, dengeli ibadet ve bilim.
    #[default]
    HarmoniousPhilosophers,
    /// Bağnaz Tapınakçılar: +%25 İbadet Ruhu/Nüfuzu, yüksek inanç coşkusu.
    ZealousTemplars,
    /// Bilimsel Şüpheciler: +%30 Teknoloji hızı, erken Bilimsel İlahi Kalkan eğilimi.
    ScientificSkeptics,
    /// Savaşçı Hanedan: +%25 Savaş/Ölüm Ruhu, yüksek militarizm.
    WarriorDynasty,
    /// Hiçlik Mistikleri: Kaos ve Çöküş evrenlerinde istikrar kaybına dayanıklı, +Öz rezonansı.
    VoidMystics,
}

impl CivilizationDoctrine {
    pub const ALL: [Self; 5] = [
        Self::HarmoniousPhilosophers,
        Self::ZealousTemplars,
        Self::ScientificSkeptics,
        Self::WarriorDynasty,
        Self::VoidMystics,
    ];

    /// Deterministically assigns a doctrine from `(universe_id, civ_id)` so even a Tech universe
    /// can host Zealous Templars and a Faith universe can host Scientific Skeptics (`taslak.md` Sec 3).
    pub fn from_seed(universe_id: u32, civ_id: u32) -> Self {
        let hash = (universe_id
            .wrapping_mul(31)
            .wrapping_add(civ_id.wrapping_mul(17))) as usize;
        Self::ALL[hash % Self::ALL.len()]
    }

    pub fn name_tr(self) -> &'static str {
        match self {
            Self::HarmoniousPhilosophers => "Uyumcu Filozoflar",
            Self::ZealousTemplars => "Bağnaz Tapınakçılar",
            Self::ScientificSkeptics => "Bilimsel Şüpheciler",
            Self::WarriorDynasty => "Savaşçı Hanedan",
            Self::VoidMystics => "Hiçlik Mistikleri",
        }
    }

    pub fn short_tag_tr(self) -> &'static str {
        match self {
            Self::HarmoniousPhilosophers => "Uyumcu",
            Self::ZealousTemplars => "Tapınakçı",
            Self::ScientificSkeptics => "Şüpheci",
            Self::WarriorDynasty => "Savaşçı",
            Self::VoidMystics => "Mistik",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Civilization {
    pub id: u32,
    pub name: String,
    pub universe_id: u32,
    /// Cultural & ideological doctrine (`#[serde(default)]` preserves save schema compatibility).
    #[serde(default)]
    pub doctrine: CivilizationDoctrine,
    /// Population in millions of mortals.
    pub population: u64,
    /// Technological progression (0.0 to 100.0). High tech can resist divine miracles and expose deities.
    pub tech_level: f32,
    /// Religious fervor & devotion (0.0 to 100.0). Drives worship soul generation.
    pub faith_fervor: f32,
    /// Societal stability (0.0 to 100.0).
    pub stability: f32,
    /// Militarism & aggression (0.0 to 100.0).
    pub militarism: f32,
    /// Awareness of supernatural entities through scientific investigation (0.0 to 100.0).
    pub deity_awareness: f32,
    /// Dominant patron deity worshipped by this civilization.
    pub patron_deity: Option<u32>,
    /// Name of the active faith / cult in this civilization.
    pub religion_name: String,
    /// Whether this civilization is currently in active rebellion or religious schism.
    pub in_rebellion: bool,
    /// Whether this civilization has scientifically exposed divine manipulation and built anti-divine shields.
    pub divine_shield_active: bool,
}

#[derive(Debug, Clone, Default)]
pub struct CivTickReport {
    pub mortal_deaths_souls: i64,
    pub devotion_souls: i64,
    pub influence_generated: i64,
    pub patron_deity: Option<u32>,
    pub rebellion_started: bool,
    pub shield_activated: bool,
    pub collapsed: bool,
}

impl Civilization {
    pub fn step_tick(
        &mut self,
        universe_stability: f32,
        universe_soul_mult: f32,
        is_tech_universe: bool,
        is_faith_universe: bool,
        is_chaos_universe: bool,
        rng: &mut DeterministicRng,
    ) -> CivTickReport {
        self.step_tick_scaled(
            universe_stability,
            universe_soul_mult,
            is_tech_universe,
            is_faith_universe,
            is_chaos_universe,
            1.0,
            rng,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn step_tick_scaled(
        &mut self,
        universe_stability: f32,
        universe_soul_mult: f32,
        is_tech_universe: bool,
        is_faith_universe: bool,
        is_chaos_universe: bool,
        step_scale: f32,
        rng: &mut DeterministicRng,
    ) -> CivTickReport {
        let mut report = CivTickReport {
            patron_deity: self.patron_deity,
            ..Default::default()
        };

        if self.population == 0 {
            report.collapsed = true;
            return report;
        }

        let scale = step_scale.clamp(1.0, 4.0);

        // 1. Natural & crisis mortality -> produces Souls
        let base_mortality_rate = (0.008
            + ((100.0 - self.stability).max(0.0) * 0.0002)
            + ((100.0 - universe_stability).max(0.0) * 0.00025)
            + if self.in_rebellion { 0.015 } else { 0.0 })
            * scale;

        let deaths = ((self.population as f32 * base_mortality_rate).round() as u64)
            .max(1)
            .min(self.population);

        // Births & growth
        let growth_factor = (if universe_stability > 40.0 && !self.in_rebellion {
            0.014 + (self.stability / 100.0) * 0.012
        } else {
            0.004
        }) * scale;
        let births = (self.population as f32 * growth_factor).round() as u64;

        self.population = self
            .population
            .saturating_add(births)
            .saturating_sub(deaths)
            .clamp(0, 50_000);

        if self.population == 0 {
            report.collapsed = true;
            return report;
        }

        // Mortal deaths yield souls scaled by universe soul potential and WarriorDynasty doctrine
        let warrior_mult = if self.doctrine == CivilizationDoctrine::WarriorDynasty {
            1.25
        } else {
            1.0
        };
        let death_souls =
            ((deaths as f32 * 0.65 * universe_soul_mult * warrior_mult).round() as i64).max(1);
        report.mortal_deaths_souls = death_souls;

        // 2. Technological advancement & supernatural awareness
        let skeptic_mult = if self.doctrine == CivilizationDoctrine::ScientificSkeptics {
            1.30
        } else {
            1.0
        };
        let tech_gain = (if is_tech_universe {
            rng.range_f32(0.15, 0.55)
        } else {
            rng.range_f32(0.04, 0.25)
        }) * scale
            * skeptic_mult;
        self.tech_level = (self.tech_level + tech_gain).clamp(0.0, 100.0);

        if self.tech_level > 55.0 && self.patron_deity.is_some() {
            let awareness_gain = (self.tech_level - 50.0) * 0.012 * scale * skeptic_mult;
            self.deity_awareness = (self.deity_awareness + awareness_gain).clamp(0.0, 100.0);
            if self.deity_awareness > 80.0
                && !self.divine_shield_active
                && rng.chance((0.08 * scale).min(0.4))
            {
                self.divine_shield_active = true;
                self.faith_fervor = (self.faith_fervor - 25.0).max(5.0);
                report.shield_activated = true;
            }
        }

        // 3. Faith & Devotion progression
        let templar_faith_bonus = if self.doctrine == CivilizationDoctrine::ZealousTemplars {
            0.12
        } else {
            0.0
        };
        let faith_drift = (if is_faith_universe {
            rng.range_f32(0.1, 0.45)
        } else if self.divine_shield_active {
            -rng.range_f32(0.1, 0.35)
        } else {
            rng.range_f32(-0.15, 0.25)
        } + templar_faith_bonus)
            * scale;
        self.faith_fervor = (self.faith_fervor + faith_drift).clamp(0.0, 100.0);

        // 4. Stability & Rebellion / Schisms
        let doctrine_stab_bonus = match self.doctrine {
            CivilizationDoctrine::HarmoniousPhilosophers => 0.14,
            CivilizationDoctrine::VoidMystics if is_chaos_universe => 0.35,
            _ => 0.0,
        };
        let stability_drift = (if is_chaos_universe {
            rng.range_f32(-0.6, 0.25)
        } else {
            rng.range_f32(-0.2, 0.35)
        } + doctrine_stab_bonus)
            * scale;
        self.stability = (self.stability + stability_drift).clamp(5.0, 100.0);

        if !self.in_rebellion && self.stability < 32.0 && rng.chance((0.12 * scale).min(0.45)) {
            self.in_rebellion = true;
            report.rebellion_started = true;
        } else if self.in_rebellion
            && (self.stability > 55.0 || rng.chance((0.18 * scale).min(0.55)))
        {
            self.in_rebellion = false;
            self.stability = (self.stability + 10.0).min(100.0);
        }

        // 5. Devotion Souls & Influence for Patron Deity
        if self.patron_deity.is_some() && !self.in_rebellion {
            let shield_penalty = if self.divine_shield_active { 0.35 } else { 1.0 };
            let devotion_mult = match self.doctrine {
                CivilizationDoctrine::ZealousTemplars => 1.25,
                CivilizationDoctrine::VoidMystics => 1.15,
                _ => 1.0,
            };
            let pop_factor = (self.population as f32).sqrt() * 0.35;
            let fervor_factor = self.faith_fervor / 100.0;
            report.devotion_souls =
                (pop_factor * fervor_factor * universe_soul_mult * shield_penalty * devotion_mult)
                    .round() as i64;
            report.influence_generated =
                ((pop_factor * 0.45 * fervor_factor * shield_penalty * devotion_mult).round()
                    as i64)
                    .max(1);
        }

        report
    }
}
