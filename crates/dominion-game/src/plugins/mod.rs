use crate::audio::CosmicSfxKind;
use bevy::prelude::*;
use dominion_content::built_in_scenarios;
use dominion_sim::{
    DivineAction, GameOutcome, PowerId, SimulationState, TutorialStep, UniverseType,
    PLAYER_DEITY_ID,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SimSpeed {
    Paused,
    #[default]
    Normal1x,
    Fast2x,
    Ultra5x,
}

impl SimSpeed {
    pub fn label_tr(self) -> &'static str {
        match self {
            Self::Paused => "DURAKLATILDI",
            Self::Normal1x => "1x NORMAL",
            Self::Fast2x => "2x HIZLI",
            Self::Ultra5x => "5x KOZMİK",
        }
    }

    pub fn interval_secs(self) -> Option<f32> {
        match self {
            Self::Paused => None,
            Self::Normal1x => Some(1.0),
            Self::Fast2x => Some(0.45),
            Self::Ultra5x => Some(0.18),
        }
    }

    pub fn anim_multiplier(self) -> f32 {
        match self {
            Self::Paused => 0.0,
            Self::Normal1x => 1.0,
            Self::Fast2x => 1.75,
            Self::Ultra5x => 3.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InspectorTab {
    #[default]
    Universe,
    Powers,
    Deities,
    VictoryAndLedger,
    GuideAndTutorial,
}

impl InspectorTab {
    pub const ALL: [Self; 5] = [
        Self::Universe,
        Self::Powers,
        Self::Deities,
        Self::VictoryAndLedger,
        Self::GuideAndTutorial,
    ];

    pub fn label_tr(self) -> &'static str {
        match self {
            Self::Universe => "Evren",
            Self::Powers => "Güçler",
            Self::Deities => "Tanrılar",
            Self::VictoryAndLedger => "Zafer & Oyun",
            Self::GuideAndTutorial => "Wiki (F1)",
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::Universe => Self::Powers,
            Self::Powers => Self::Deities,
            Self::Deities => Self::VictoryAndLedger,
            Self::VictoryAndLedger => Self::GuideAndTutorial,
            Self::GuideAndTutorial => Self::Universe,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OwnershipFilter {
    #[default]
    All,
    FullAllegiance,
    PlayerDominated,
    RivalDominated,
    DangerAlert,
    Undiscovered,
}

impl OwnershipFilter {
    pub const ALL: [Self; 6] = [
        Self::All,
        Self::FullAllegiance,
        Self::PlayerDominated,
        Self::RivalDominated,
        Self::DangerAlert,
        Self::Undiscovered,
    ];

    pub fn label_tr(self) -> &'static str {
        match self {
            Self::All => "Tümü",
            Self::FullAllegiance => "★ Tam",
            Self::PlayerDominated => "◆ Bizim",
            Self::RivalDominated => "⚔ Rakip",
            Self::DangerAlert => "▲ Tehlike",
            Self::Undiscovered => "? Keşif",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UniverseSortMode {
    #[default]
    Influence,
    SoulPotential,
    Danger,
    LifeDensity,
}

impl UniverseSortMode {
    pub fn label_tr(self) -> &'static str {
        match self {
            Self::Influence => "Nüfuz",
            Self::SoulPotential => "Ruh Potansiyeli",
            Self::Danger => "Tehlike",
            Self::LifeDensity => "Yaşam Yoğunluğu",
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::Influence => Self::SoulPotential,
            Self::SoulPotential => Self::Danger,
            Self::Danger => Self::LifeDensity,
            Self::LifeDensity => Self::Influence,
        }
    }
}

/// Four-layer cosmic map overlay mode (Faz 2.3.2 — Shortcut: `M` or Top Bar button).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MapOverlayMode {
    #[default]
    StandardCosmos,
    DeityDominionBlocks,
    EconomyAndReservoir,
    StabilityHeatmap,
}

impl MapOverlayMode {
    pub fn label_tr(self) -> &'static str {
        match self {
            Self::StandardCosmos => "1. Standart Kozmos",
            Self::DeityDominionBlocks => "2. İlahi Hâkimiyet & Bloklar",
            Self::EconomyAndReservoir => "3. Ekonomi & Ruh Havuzu",
            Self::StabilityHeatmap => "4. İstikrar & Tehlike Isısı",
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::StandardCosmos => Self::DeityDominionBlocks,
            Self::DeityDominionBlocks => Self::EconomyAndReservoir,
            Self::EconomyAndReservoir => Self::StabilityHeatmap,
            Self::StabilityHeatmap => Self::StandardCosmos,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MapVfxKind {
    /// Seçim ve odak halkası dalgası (Daireler & Halkalar — TASARIM.md 3.1)
    SelectionPulse,
    /// Kehanet ve inanç yayılımı konsantrik halkaları (TASARIM.md 4.3)
    ProphecyRings,
    /// Ruh hasadı ve kozmik öz çekimi nokta kümesi girdabı (TASARIM.md 3.1)
    SoulHarvestSwirl,
    /// Mucize, yaşam tohumlama ve gerçeklik onarımı altıgen düzen dalgası (TASARIM.md 3.1)
    HexagonOrder,
    /// Rakip darbesi, isyan ve mühürleme üçgen şok dalgası (TASARIM.md 3.1)
    DivineStrike,
    /// Evrenler arası geçit ve keşif enerji akışı (TASARIM.md 6)
    GatewayBeam { to_pos: Vec2 },
}

#[derive(Debug, Clone)]
pub struct ActiveMapVfx {
    pub origin: Vec2,
    pub kind: MapVfxKind,
    pub elapsed: f32,
    pub duration: f32,
}

#[derive(Debug, Clone)]
pub struct PendingFloatingToast {
    pub world_pos: Vec2,
    pub text: String,
    pub color: Color,
}

#[derive(Resource, Clone)]
pub struct UiFonts {
    pub body: Handle<Font>,
    pub bold: Handle<Font>,
}

#[derive(Resource)]
pub struct GameRuntimeState {
    pub sim: SimulationState,
    pub selected_universe_id: u32,
    pub hovered_universe_id: Option<u32>,
    /// Hovered inter-universe gateway connection `(from_id, to_id)`.
    pub hovered_gateway: Option<(u32, u32)>,
    /// Clicked/inspected inter-universe gateway connection `(from_id, to_id)`.
    pub inspected_gateway: Option<(u32, u32)>,
    pub selected_rival_deity_id: u32,
    pub active_tab: InspectorTab,
    pub speed: SimSpeed,
    pub tick_accumulator: f32,
    /// Animation clock that ONLY advances when the game is not paused (`speed != SimSpeed::Paused`).
    pub anim_time: f32,
    /// Continuous UI animation clock for smooth button hover/pulse animations even when paused or in Main Menu.
    pub ui_anim_time: f32,
    pub show_main_menu: bool,
    pub main_menu_settings_open: bool,
    pub player_name_input: String,
    pub deity_name_preset_index: usize,
    pub ui_theme_index: usize,
    pub map_overlay_mode: MapOverlayMode,
    pub type_filter: Option<UniverseType>,
    pub ownership_filter: OwnershipFilter,
    pub sort_mode: UniverseSortMode,
    pub pool_page: usize,
    pub tutorial_collapsed: bool,
    pub endgame_modal_dismissed: bool,
    pub high_contrast_symbols: bool,
    pub status_message: String,
    pub active_scenario_index: usize,
    pub guide_chapter_index: usize,
    pub camera_snap_target: Option<Vec2>,
    pub camera_zoom_target: Option<f32>,
    pub active_vfx: Vec<ActiveMapVfx>,
    pub pending_toasts: Vec<PendingFloatingToast>,
    pub pending_sfx: Vec<CosmicSfxKind>,
    pub audio_enabled: bool,
    pub audio_volume: f32,
    pub ui_resource_flash_timer: f32,
    pub ui_banner_flash_timer: f32,
}

impl Default for GameRuntimeState {
    fn default() -> Self {
        let scenarios = built_in_scenarios();
        let mut config = scenarios
            .first()
            .map(|s| s.config.clone())
            .unwrap_or_default();
        let default_name = "Aethelgard".to_string();
        config.player_name = default_name.clone();
        let sim = SimulationState::new(config);
        Self {
            sim,
            selected_universe_id: 0,
            hovered_universe_id: None,
            hovered_gateway: None,
            inspected_gateway: None,
            selected_rival_deity_id: 1,
            active_tab: InspectorTab::Universe,
            speed: SimSpeed::Normal1x,
            tick_accumulator: 0.0,
            anim_time: 0.0,
            ui_anim_time: 0.0,
            show_main_menu: true,
            main_menu_settings_open: false,
            player_name_input: default_name,
            deity_name_preset_index: 0,
            ui_theme_index: 0,
            map_overlay_mode: MapOverlayMode::StandardCosmos,
            type_filter: None,
            ownership_filter: OwnershipFilter::All,
            sort_mode: UniverseSortMode::Influence,
            pool_page: 0,
            tutorial_collapsed: false,
            endgame_modal_dismissed: false,
            high_contrast_symbols: true,
            status_message: "Kozmik Harita hazır. Evrenlere veya aralarındaki geçit çizgilerine tıklayabilirsin."
                .to_string(),
            active_scenario_index: 0,
            guide_chapter_index: 0,
            camera_snap_target: None,
            camera_zoom_target: None,
            active_vfx: Vec::new(),
            pending_toasts: Vec::new(),
            pending_sfx: Vec::new(),
            audio_enabled: true,
            audio_volume: 0.65,
            ui_resource_flash_timer: 0.0,
            ui_banner_flash_timer: 0.0,
        }
    }
}

impl GameRuntimeState {
    pub fn filtered_sorted_universes(&self) -> Vec<u32> {
        let mut ids: Vec<u32> = self
            .sim
            .universes
            .iter()
            .filter(|u| !u.is_destroyed)
            .filter(|u| {
                if let Some(ft) = self.type_filter {
                    u.universe_type == ft
                } else {
                    true
                }
            })
            .filter(|u| match self.ownership_filter {
                OwnershipFilter::All => true,
                OwnershipFilter::FullAllegiance => {
                    u.discovered_by_player && u.has_full_allegiance(PLAYER_DEITY_ID)
                }
                OwnershipFilter::PlayerDominated => {
                    u.discovered_by_player && u.is_dominated_by(PLAYER_DEITY_ID)
                }
                OwnershipFilter::RivalDominated => {
                    u.discovered_by_player
                        && u.dominant_deity()
                            .is_some_and(|(id, _)| id != PLAYER_DEITY_ID)
                }
                OwnershipFilter::DangerAlert => {
                    u.discovered_by_player && (u.danger_level >= 60.0 || u.stability <= 35.0)
                }
                OwnershipFilter::Undiscovered => !u.discovered_by_player,
            })
            .map(|u| u.id)
            .collect();

        ids.sort_by(|&a, &b| {
            let ua = &self.sim.universes[a as usize];
            let ub = &self.sim.universes[b as usize];
            match (ua.discovered_by_player, ub.discovered_by_player) {
                (true, false) => return std::cmp::Ordering::Less,
                (false, true) => return std::cmp::Ordering::Greater,
                _ => {}
            }
            let score_a = match self.sort_mode {
                UniverseSortMode::Influence => ua.influence_of(PLAYER_DEITY_ID),
                UniverseSortMode::SoulPotential => {
                    ua.soul_potential * 100.0 + ua.soul_reservoir as f32
                }
                UniverseSortMode::Danger => ua.danger_level,
                UniverseSortMode::LifeDensity => ua.life_density(),
            };
            let score_b = match self.sort_mode {
                UniverseSortMode::Influence => ub.influence_of(PLAYER_DEITY_ID),
                UniverseSortMode::SoulPotential => {
                    ub.soul_potential * 100.0 + ub.soul_reservoir as f32
                }
                UniverseSortMode::Danger => ub.danger_level,
                UniverseSortMode::LifeDensity => ub.life_density(),
            };
            score_b
                .partial_cmp(&score_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        ids
    }

    pub fn cycle_type_filter(&mut self) {
        self.type_filter = match self.type_filter {
            None => Some(UniverseType::Life),
            Some(UniverseType::Life) => Some(UniverseType::Faith),
            Some(UniverseType::Faith) => Some(UniverseType::Chaos),
            Some(UniverseType::Chaos) => Some(UniverseType::Tech),
            Some(UniverseType::Tech) => Some(UniverseType::Primordial),
            Some(UniverseType::Primordial) => Some(UniverseType::Collapsing),
            Some(UniverseType::Collapsing) => None,
        };
        self.pool_page = 0;
    }

    pub fn start_scenario(&mut self, scenario_index: usize) {
        let scenarios = built_in_scenarios();
        let idx = scenario_index.min(scenarios.len().saturating_sub(1));
        let preset = &scenarios[idx];
        let mut cfg = preset.config.clone();
        if !self.player_name_input.trim().is_empty() {
            cfg.player_name = self.player_name_input.trim().to_string();
        }
        self.sim = SimulationState::new(cfg);
        self.selected_universe_id = 0;
        self.hovered_universe_id = None;
        self.hovered_gateway = None;
        self.inspected_gateway = None;
        self.selected_rival_deity_id = 1;
        self.tick_accumulator = 0.0;
        self.anim_time = 0.0;
        self.pool_page = 0;
        self.endgame_modal_dismissed = false;
        self.show_main_menu = false;
        self.main_menu_settings_open = false;
        self.active_scenario_index = idx;
        self.camera_snap_target = Some(Vec2::ZERO);
        self.camera_zoom_target = Some(1.0);
        self.active_vfx.clear();
        self.pending_toasts.clear();
        self.spawn_universe_vfx(0, MapVfxKind::ProphecyRings, 1.4);
        self.status_message = format!(
            "Yeni oyun başlatıldı: {} (İlah: {})",
            preset.title_tr,
            self.sim.player_deity().name
        );
    }

    pub fn cycle_random_deity_name(&mut self) {
        const COSMIC_DEITY_NAMES: [&str; 10] = [
            "Aethelgard",
            "Kaelithar",
            "Zephyron",
            "Oron-Vex",
            "Seraphion",
            "Astralyn",
            "Valerion",
            "Nyxaris",
            "Solarius",
            "Thalorion",
        ];
        self.deity_name_preset_index =
            (self.deity_name_preset_index + 1) % COSMIC_DEITY_NAMES.len();
        let chosen = COSMIC_DEITY_NAMES[self.deity_name_preset_index].to_string();
        self.player_name_input = chosen.clone();
        self.sim.rename_player_deity(&chosen);
        self.status_message = format!("İlah adın '{}' olarak belirlendi.", chosen);
    }

    pub fn cycle_map_overlay_mode(&mut self) {
        self.map_overlay_mode = self.map_overlay_mode.next();
        self.play_sfx(CosmicSfxKind::UiClick);
        self.status_message = format!(
            "Harita görünüm katmanı değiştirildi: {}",
            self.map_overlay_mode.label_tr()
        );
    }

    pub fn play_sfx(&mut self, kind: CosmicSfxKind) {
        if self.audio_enabled {
            self.pending_sfx.push(kind);
        }
    }

    pub fn toggle_audio(&mut self) {
        self.audio_enabled = !self.audio_enabled;
        if self.audio_enabled {
            self.pending_sfx.push(CosmicSfxKind::ProphecyChime);
            self.status_message =
                "Prosedürel Kozmik Sentezleyici Sesi: AÇIK (Kısayol: V)".to_string();
        } else {
            self.pending_sfx.clear();
            self.status_message =
                "Prosedürel Kozmik Sentezleyici Sesi: KAPALI (Kısayol: V)".to_string();
        }
    }

    /// Finds a universe where the currently selected rival deity is dominant (or has influence),
    /// reveals/selects it if discovered, or points the camera toward it.
    pub fn focus_rival_deity_universe(&mut self) {
        let rival_id = self.selected_rival_deity_id;
        let rival_name = self
            .sim
            .deities
            .get(rival_id as usize)
            .map(|d| d.name.clone())
            .unwrap_or_else(|| "Rakip".to_string());

        // Prefer discovered universes dominated by rival, then any universe dominated by rival, then any with rival influence
        let candidate = self
            .sim
            .universes
            .iter()
            .filter(|u| !u.is_destroyed && u.discovered_by_player && u.is_dominated_by(rival_id))
            .max_by(|a, b| {
                a.influence_of(rival_id)
                    .partial_cmp(&b.influence_of(rival_id))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .or_else(|| {
                self.sim
                    .universes
                    .iter()
                    .filter(|u| !u.is_destroyed && u.is_dominated_by(rival_id))
                    .max_by(|a, b| {
                        a.influence_of(rival_id)
                            .partial_cmp(&b.influence_of(rival_id))
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
            })
            .or_else(|| {
                self.sim
                    .universes
                    .iter()
                    .filter(|u| !u.is_destroyed && u.influence_of(rival_id) > 0.0)
                    .max_by(|a, b| {
                        a.influence_of(rival_id)
                            .partial_cmp(&b.influence_of(rival_id))
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
            })
            .map(|u| (u.id, u.position, u.name.clone(), u.discovered_by_player));

        if let Some((u_id, pos, u_name, discovered)) = candidate {
            self.selected_universe_id = u_id;
            self.inspected_gateway = None;
            self.camera_snap_target = Some(Vec2::new(pos[0], pos[1]));
            self.spawn_universe_vfx(u_id, MapVfxKind::SelectionPulse, 1.25);
            if discovered {
                self.status_message = format!(
                    "{rival_name} ilahının kalesi bulundu ve odaklandı: {u_name} (Evren sekmesinden sızma emri verebilirsin)."
                );
            } else {
                self.status_message = format!(
                    "{rival_name} ilahının izi keşfedilmemiş bölgede (#{}) tespit edildi; kamera o yöne kaydırıldı.",
                    u_id + 1
                );
            }
        } else {
            self.status_message =
                format!("{rival_name} ilahının kozmosta aktif bir evren hâkimiyeti kalmadı.");
        }
    }

    pub fn spawn_universe_vfx(&mut self, universe_id: u32, kind: MapVfxKind, duration: f32) {
        if let Some(u) = self.sim.universes.get(universe_id as usize) {
            let origin = Vec2::new(u.position[0], u.position[1]);
            self.active_vfx.push(ActiveMapVfx {
                origin,
                kind,
                elapsed: 0.0,
                duration,
            });
        }
    }

    pub fn spawn_universe_toast(
        &mut self,
        universe_id: u32,
        text: impl Into<String>,
        color: Color,
    ) {
        if let Some(u) = self.sim.universes.get(universe_id as usize) {
            let world_pos = Vec2::new(u.position[0], u.position[1] + 22.0);
            self.pending_toasts.push(PendingFloatingToast {
                world_pos,
                text: text.into(),
                color,
            });
        }
    }

    pub fn focus_selected_universe(&mut self) {
        let sel = self.selected_universe_id;
        self.inspected_gateway = None;
        let res = self.sim.execute_action(
            PLAYER_DEITY_ID,
            DivineAction::FocusUniverse {
                universe_id: Some(sel),
            },
        );
        if let Some(u) = self.sim.universes.get(sel as usize) {
            self.camera_snap_target = Some(Vec2::new(u.position[0], u.position[1]));
        }
        if res.is_ok() {
            self.spawn_universe_vfx(sel, MapVfxKind::SelectionPulse, 1.15);
            self.spawn_universe_toast(
                sel,
                "★ İlahi Odak (+%25 Akış)",
                Color::srgb_u8(0x9B, 0x7B, 0xFF),
            );
        }
        self.status_message = res.unwrap_or_else(|e| e);
    }

    /// Executes or navigates to the active interactive tutorial step when the player clicks "Görevi Şimdi Uygula".
    pub fn execute_or_highlight_tutorial_step(&mut self) {
        let sel = self.selected_universe_id;
        // For sovereign actions in tutorial, use `sel` if dominated by player, otherwise fallback to a dominated universe (e.g. 0)
        let dom_sel = if self
            .sim
            .universes
            .get(sel as usize)
            .is_some_and(|u| u.discovered_by_player && u.is_dominated_by(PLAYER_DEITY_ID))
        {
            sel
        } else {
            self.sim
                .universes
                .iter()
                .find(|u| {
                    !u.is_destroyed && u.discovered_by_player && u.is_dominated_by(PLAYER_DEITY_ID)
                })
                .map(|u| u.id)
                .unwrap_or(0)
        };
        self.inspected_gateway = None;
        let prev = self.sim.tutorial_step;
        let res = match self.sim.tutorial_step {
            TutorialStep::Step1Prophecy => {
                self.active_tab = InspectorTab::Universe;
                self.selected_universe_id = 0;
                let r = self.sim.execute_action(
                    PLAYER_DEITY_ID,
                    DivineAction::SpreadProphecy { universe_id: 0 },
                );
                if r.is_ok() {
                    self.spawn_universe_vfx(0, MapVfxKind::ProphecyRings, 1.25);
                    self.spawn_universe_toast(
                        0,
                        "+Kehanet (Adım 1 Tamamlandı)",
                        Color::srgb_u8(0xE5, 0xC7, 0x7A),
                    );
                }
                r
            }
            TutorialStep::Step2Explore => {
                self.active_tab = InspectorTab::Universe;
                self.selected_universe_id = dom_sel;
                let r = self.sim.execute_action(
                    PLAYER_DEITY_ID,
                    DivineAction::ScoutRegion {
                        universe_id: dom_sel,
                    },
                );
                if r.is_ok() {
                    self.spawn_universe_vfx(dom_sel, MapVfxKind::SelectionPulse, 1.2);
                    self.spawn_universe_toast(
                        dom_sel,
                        "+Kozmik Keşif (Adım 2 Tamamlandı)",
                        Color::srgb_u8(0x5E, 0xD6, 0xC1),
                    );
                }
                r
            }
            TutorialStep::Step3HarvestOrEssence => {
                self.active_tab = InspectorTab::Universe;
                self.selected_universe_id = dom_sel;
                let r = self.sim.execute_action(
                    PLAYER_DEITY_ID,
                    DivineAction::HarvestSouls {
                        universe_id: dom_sel,
                    },
                );
                if r.is_ok() {
                    self.spawn_universe_vfx(dom_sel, MapVfxKind::SoulHarvestSwirl, 1.25);
                    self.spawn_universe_toast(
                        dom_sel,
                        "+Ruh Hasadı (Adım 3 Tamamlandı)",
                        Color::srgb_u8(0x9B, 0x7B, 0xFF),
                    );
                }
                r
            }
            TutorialStep::Step4UnlockPower => {
                self.active_tab = InspectorTab::Powers;
                let r = self.sim.execute_action(
                    PLAYER_DEITY_ID,
                    DivineAction::UnlockPower {
                        power: PowerId::VoiceOfProphecy,
                    },
                );
                if r.is_ok() {
                    self.spawn_universe_vfx(dom_sel, MapVfxKind::ProphecyRings, 1.3);
                    self.spawn_universe_toast(
                        dom_sel,
                        "★ İlahi Güç Açıldı (Adım 4 Tamamlandı)",
                        Color::srgb_u8(0xE5, 0xC7, 0x7A),
                    );
                }
                r
            }
            TutorialStep::Step5CreateOrStabilize => {
                self.active_tab = InspectorTab::Universe;
                self.selected_universe_id = dom_sel;
                let r = self.sim.execute_action(
                    PLAYER_DEITY_ID,
                    DivineAction::BestowMiracle {
                        universe_id: dom_sel,
                    },
                );
                if r.is_ok() {
                    self.spawn_universe_vfx(dom_sel, MapVfxKind::HexagonOrder, 1.3);
                    self.spawn_universe_toast(
                        dom_sel,
                        "+Mucize Bahşedildi (Adım 5 Tamamlandı)",
                        Color::srgb_u8(0x5E, 0xD6, 0xC1),
                    );
                }
                r
            }
            TutorialStep::Step6RivalInteraction => {
                self.active_tab = InspectorTab::Deities;
                let r = self.sim.execute_action(
                    PLAYER_DEITY_ID,
                    DivineAction::OfferDiplomaticPact {
                        target_deity_id: self.selected_rival_deity_id,
                    },
                );
                if r.is_ok() {
                    self.spawn_universe_vfx(dom_sel, MapVfxKind::HexagonOrder, 1.3);
                    self.spawn_universe_toast(
                        dom_sel,
                        "★ Eğitim Tamamlandı!",
                        Color::srgb_u8(0xE5, 0xC7, 0x7A),
                    );
                }
                r
            }
            TutorialStep::Completed => Ok(
                "Başlangıç eğitimi zaten tamamlandı! 'Eğitimi Sıfırla' ile tekrar başlatabilirsin."
                    .to_string(),
            ),
        };

        if self.sim.tutorial_step != prev {
            self.ui_banner_flash_timer = 1.0;
            self.ui_resource_flash_timer = 0.75;
        }
        self.status_message = res.unwrap_or_else(|e| e);
    }

    pub fn reset_tutorial(&mut self) {
        self.sim.tutorial_step = TutorialStep::Step1Prophecy;
        self.ui_banner_flash_timer = 0.8;
        self.status_message = "Eğitici görevler Adım 1'den yeniden başlatıldı.".to_string();
    }

    pub fn theme_name_tr(&self) -> &'static str {
        match self.ui_theme_index % 4 {
            0 => "Safir Mor & İlahi Kehribar",
            1 => "Zümrüt Yeşim & Yıldız Tozu",
            2 => "Kızıl Kehribar & Süpernova",
            _ => "Ebedi Mermer & Altın Usturlap",
        }
    }

    pub fn cycle_ui_theme(&mut self) {
        self.ui_theme_index = (self.ui_theme_index + 1) % 4;
        self.ui_resource_flash_timer = 0.65;
        self.ui_banner_flash_timer = 0.65;
        self.status_message = format!("Arayüz ve Harita Teması: {}", self.theme_name_tr());
    }

    /// Selects and centers the camera on the universe with an active dilemma or the latest random cosmic event.
    pub fn focus_latest_event_universe(&mut self) {
        let target_u = self
            .sim
            .active_dilemma
            .as_ref()
            .map(|d| d.universe_id)
            .or(self.sim.last_event_universe_id)
            .or_else(|| self.sim.events.iter().rev().find_map(|ev| ev.universe_id));

        if let Some(u_id) = target_u {
            if let Some(u) = self.sim.universes.get(u_id as usize) {
                let u_pos = Vec2::new(u.position[0], u.position[1]);
                let u_name = if u.discovered_by_player {
                    u.name.clone()
                } else {
                    format!("Gizemli Bölge #{}", u.id + 1)
                };
                self.selected_universe_id = u_id;
                self.inspected_gateway = None;
                self.active_tab = InspectorTab::Universe;
                self.camera_snap_target = Some(u_pos);
                self.spawn_universe_vfx(u_id, MapVfxKind::SelectionPulse, 1.25);
                self.status_message =
                    format!("Son kozmik olay / ikilem evrenine odaklanıldı: {u_name}");
                return;
            }
        }
        self.status_message = "Henüz odaklanılacak bir kozmik olay evreni yok.".to_string();
    }

    /// Advances the simulation by 1 tick and triggers map VFX + toasts for any newly spawned random events or dilemmas.
    pub fn step_tick_with_visual_feedback(&mut self) {
        let prev_dilemma = self.sim.active_dilemma.as_ref().map(|d| d.id);
        let prev_event_id = self.sim.next_event_id;
        let prev_ongoing = matches!(self.sim.outcome, GameOutcome::Ongoing);
        self.sim.step_tick();
        self.ui_resource_flash_timer = 0.35;

        if prev_ongoing && matches!(self.sim.outcome, GameOutcome::PlayerVictory { .. }) {
            self.play_sfx(CosmicSfxKind::VictoryFanfare);
        }

        if let Some(d) = self.sim.active_dilemma.clone() {
            if prev_dilemma != Some(d.id) {
                self.ui_banner_flash_timer = 1.0;
                self.play_sfx(CosmicSfxKind::CosmicEventAlert);
                self.spawn_universe_vfx(d.universe_id, MapVfxKind::ProphecyRings, 1.35);
                self.spawn_universe_toast(
                    d.universe_id,
                    "★ Kozmik İkilem Doğdu!",
                    Color::srgb_u8(0xFF, 0xB8, 0x4D),
                );
                return;
            }
        }

        if self.sim.next_event_id > prev_event_id {
            if let Some(ev) = self.sim.events.last().cloned() {
                if let Some(u_id) = ev.universe_id {
                    self.play_sfx(CosmicSfxKind::CosmicEventAlert);
                    self.spawn_universe_vfx(u_id, MapVfxKind::HexagonOrder, 1.2);
                    self.spawn_universe_toast(
                        u_id,
                        format!("{} {}", ev.kind.symbol_tag(), ev.title),
                        Color::srgb_u8(0x2E, 0xE5, 0xB5),
                    );
                }
            }
        }
    }
}

pub struct SimulationBridgePlugin;

impl Plugin for SimulationBridgePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameRuntimeState>()
            .insert_resource(Time::<Fixed>::from_hz(30.0))
            .add_systems(PreStartup, load_ui_fonts)
            .add_systems(FixedUpdate, advance_simulation_fixed)
            .add_systems(Update, (advance_animation_clock, handle_simulation_hotkeys));
    }
}

/// Embedded UI fonts (DejaVu Sans Condensed family): full Turkish glyph coverage
/// (ğ, ş, ı, İ, ö, ü, ç) AND every geometric symbol used by the UI (★ ◆ ▲ ⇄ ✦ ⚔ ⚙ ✓ ✖).
/// Embedding avoids async asset-path failures that would render text invisible.
const UI_FONT_BODY_BYTES: &[u8] =
    include_bytes!("../../../../assets/fonts/DejaVuSansCondensed.ttf");
const UI_FONT_BOLD_BYTES: &[u8] =
    include_bytes!("../../../../assets/fonts/DejaVuSansCondensed-Bold.ttf");

fn load_ui_fonts(mut commands: Commands, mut fonts: ResMut<Assets<Font>>) {
    commands.insert_resource(UiFonts {
        body: fonts.add(Font::from_bytes(UI_FONT_BODY_BYTES.to_vec())),
        bold: fonts.add(Font::from_bytes(UI_FONT_BOLD_BYTES.to_vec())),
    });
}

fn advance_animation_clock(time: Res<Time>, mut runtime: ResMut<GameRuntimeState>) {
    // UI animation clock always ticks for smooth menu/button animations
    runtime.ui_anim_time += time.delta_secs();
    if runtime.show_main_menu {
        return;
    }
    // Only advance the map animation clock when the simulation is NOT paused!
    let mult = runtime.speed.anim_multiplier();
    if mult > 0.0 {
        runtime.anim_time += time.delta_secs() * mult;
    }
}

fn advance_simulation_fixed(time: Res<Time<Fixed>>, mut runtime: ResMut<GameRuntimeState>) {
    if runtime.show_main_menu {
        return;
    }
    let Some(interval) = runtime.speed.interval_secs() else {
        return;
    };

    runtime.tick_accumulator += time.delta_secs();
    if runtime.tick_accumulator >= interval {
        runtime.tick_accumulator -= interval;
        runtime.step_tick_with_visual_feedback();
    }
}

fn handle_simulation_hotkeys(
    keys: Res<ButtonInput<KeyCode>>,
    mut runtime: ResMut<GameRuntimeState>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        runtime.show_main_menu = !runtime.show_main_menu;
        runtime.status_message = if runtime.show_main_menu {
            "Ana Menü açıldı. İlah adını düzenleyebilir veya ayarları değiştirebilirsin."
                .to_string()
        } else {
            format!("Oyuna dönüldü — İlah: {}", runtime.sim.player_deity().name)
        };
        return;
    }

    // When Main Menu is open, allow typing the player's Deity Name with keyboard!
    if runtime.show_main_menu {
        let mut changed = false;
        if keys.just_pressed(KeyCode::Backspace) {
            runtime.player_name_input.pop();
            changed = true;
        }
        let letter_map = [
            (KeyCode::KeyA, 'a', 'A'),
            (KeyCode::KeyB, 'b', 'B'),
            (KeyCode::KeyC, 'c', 'C'),
            (KeyCode::KeyD, 'd', 'D'),
            (KeyCode::KeyE, 'e', 'E'),
            (KeyCode::KeyF, 'f', 'F'),
            (KeyCode::KeyG, 'g', 'G'),
            (KeyCode::KeyH, 'h', 'H'),
            (KeyCode::KeyI, 'i', 'I'),
            (KeyCode::KeyJ, 'j', 'J'),
            (KeyCode::KeyK, 'k', 'K'),
            (KeyCode::KeyL, 'l', 'L'),
            (KeyCode::KeyM, 'm', 'M'),
            (KeyCode::KeyN, 'n', 'N'),
            (KeyCode::KeyO, 'o', 'O'),
            (KeyCode::KeyP, 'p', 'P'),
            (KeyCode::KeyQ, 'q', 'Q'),
            (KeyCode::KeyR, 'r', 'R'),
            (KeyCode::KeyS, 's', 'S'),
            (KeyCode::KeyT, 't', 'T'),
            (KeyCode::KeyU, 'u', 'U'),
            (KeyCode::KeyV, 'v', 'V'),
            (KeyCode::KeyW, 'w', 'W'),
            (KeyCode::KeyX, 'x', 'X'),
            (KeyCode::KeyY, 'y', 'Y'),
            (KeyCode::KeyZ, 'z', 'Z'),
            (KeyCode::Space, ' ', ' '),
            (KeyCode::Minus, '-', '-'),
        ];
        let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
        for (code, lower, upper) in letter_map {
            if keys.just_pressed(code) && runtime.player_name_input.chars().count() < 18 {
                let capitalize = shift
                    || runtime.player_name_input.is_empty()
                    || runtime.player_name_input.ends_with(' ')
                    || runtime.player_name_input.ends_with('-');
                runtime
                    .player_name_input
                    .push(if capitalize { upper } else { lower });
                changed = true;
            }
        }
        if changed {
            let fallback = if runtime.player_name_input.trim().is_empty() {
                "Aethelgard".to_string()
            } else {
                runtime.player_name_input.trim().to_string()
            };
            runtime.sim.rename_player_deity(fallback);
        }
        return;
    }

    if keys.just_pressed(KeyCode::Space) {
        runtime.speed = if runtime.speed == SimSpeed::Paused {
            SimSpeed::Normal1x
        } else {
            SimSpeed::Paused
        };
        runtime.status_message = format!("Zaman akışı: {}", runtime.speed.label_tr());
    } else if keys.just_pressed(KeyCode::Digit1) {
        runtime.speed = SimSpeed::Normal1x;
        runtime.status_message = "Zaman hızı: 1x Normal".to_string();
    } else if keys.just_pressed(KeyCode::Digit2) {
        runtime.speed = SimSpeed::Fast2x;
        runtime.status_message = "Zaman hızı: 2x Hızlı".to_string();
    } else if keys.just_pressed(KeyCode::Digit3) {
        runtime.speed = SimSpeed::Ultra5x;
        runtime.status_message = "Zaman hızı: 5x Kozmik".to_string();
    } else if keys.just_pressed(KeyCode::KeyN) {
        runtime.step_tick_with_visual_feedback();
        runtime.anim_time += 0.45;
        runtime.ui_resource_flash_timer = 0.45;
        runtime.status_message =
            format!("Simülasyon 1 tur ilerletildi (Tur {}).", runtime.sim.tick);
    } else if keys.just_pressed(KeyCode::Tab) {
        runtime.active_tab = runtime.active_tab.next();
    } else if keys.just_pressed(KeyCode::F1) || keys.just_pressed(KeyCode::KeyH) {
        runtime.active_tab = InspectorTab::GuideAndTutorial;
        runtime.status_message = "KozMO Ansiklopedisi ve Wiki sayfası açıldı.".to_string();
    } else if keys.just_pressed(KeyCode::KeyF) {
        runtime.focus_selected_universe();
    } else if keys.just_pressed(KeyCode::KeyE) {
        runtime.focus_latest_event_universe();
    } else if keys.just_pressed(KeyCode::KeyT) {
        runtime.cycle_ui_theme();
    } else if keys.just_pressed(KeyCode::KeyM) {
        runtime.cycle_map_overlay_mode();
    } else if keys.just_pressed(KeyCode::KeyV) {
        runtime.toggle_audio();
    }
}
