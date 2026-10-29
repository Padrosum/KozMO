use crate::{
    audio::CosmicSfxKind,
    plugins::{
        GameRuntimeState, InspectorTab, MapVfxKind, OwnershipFilter, SimSpeed, UiFonts,
        UniverseSortMode,
    },
    save::{load_game_state, save_game_state},
};
use bevy::{
    app::AppExit,
    picking::hover::Hovered,
    prelude::*,
    ui::Pressed,
    ui_widgets::{Activate, Button},
};
use dominion_content::{
    built_in_guide_chapters, built_in_scenarios, powers_for_tree, universe_type_profile,
};
use dominion_sim::{
    DivineAction, GameOutcome, GatewayState, PowerId, PowerTree, ResourceExpenditure,
    ResourceRemovalReason, ResourceSource, TutorialStep, UniverseType,
    CREATION_REQUIRED_SHAPED_UNIVERSES, DOMINION_REQUIRED_DOMINATED,
    DOMINION_REQUIRED_FULL_ALLEGIANCE, DOMINION_REQUIRED_SEALED_RIVALS, PLAYER_DEITY_ID,
    TRANSCENDENCE_REQUIRED_FULL_ALLEGIANCE, TRANSCENDENCE_REQUIRED_POWERS,
};

pub const POOL_SLOTS_PER_PAGE: usize = 8;

pub struct CosmicUiPlugin;

impl Plugin for CosmicUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_cosmic_ui)
            .add_observer(on_ui_button_activate)
            .add_systems(
                Update,
                (
                    apply_custom_font_to_added_texts,
                    update_button_visuals,
                    refresh_button_hover_tooltip,
                    refresh_top_bar_cards,
                    refresh_left_panel_text,
                    refresh_right_inspector_text,
                    refresh_bottom_event_log_text,
                    refresh_endgame_modal,
                    refresh_main_menu_overlay,
                ),
            );
    }
}

// Palette constants — Celestial Amber, Emerald Jade, Royal Amethyst & Obsidian Sanctum
const COL_BG_VOID: Color = Color::srgb_u8(0x07, 0x05, 0x0F);
const COL_BG_DEEP: Color = Color::srgb_u8(0x11, 0x0D, 0x1E);
const COL_BG_PANEL: Color = Color::srgb_u8(0x1B, 0x14, 0x2E);
const COL_BTN_NORMAL: Color = Color::srgb_u8(0x23, 0x1B, 0x3A);
const COL_BTN_ACTIVE: Color = Color::srgb_u8(0x37, 0x26, 0x59);
const COL_BTN_HOVER: Color = Color::srgb_u8(0x3F, 0x2B, 0x66);
const COL_BTN_PRESSED: Color = Color::srgb_u8(0x64, 0x38, 0xA8);
const COL_BTN_LOCKED: Color = Color::srgb_u8(0x14, 0x0E, 0x21);
const COL_TEXT_PRIMARY: Color = Color::srgb_u8(0xF8, 0xF3, 0xE6);
const COL_TEXT_SECONDARY: Color = Color::srgb_u8(0xB9, 0xAD, 0xD2);
const COL_ACCENT_PURPLE: Color = Color::srgb_u8(0xC4, 0x6B, 0xFF);
const COL_ACCENT_GOLD: Color = Color::srgb_u8(0xFF, 0xB8, 0x4D);
const COL_ACCENT_TURQUOISE: Color = Color::srgb_u8(0x2E, 0xE5, 0xB5);
const COL_ACCENT_CRIMSON: Color = Color::srgb_u8(0xFF, 0x4F, 0x79);

fn apply_custom_font_to_added_texts(
    fonts: Res<UiFonts>,
    mut q_fonts: Query<&mut TextFont, Added<TextFont>>,
) {
    for mut tf in &mut q_fonts {
        let is_heading = matches!(tf.font_size, FontSize::Px(sz) if sz >= 11.2);
        tf.font = if is_heading {
            fonts.bold.clone().into()
        } else {
            fonts.body.clone().into()
        };
    }
}

fn theme_btn_palette(theme_idx: usize) -> (Color, Color, Color) {
    match theme_idx % 4 {
        0 => (
            COL_BTN_NORMAL,
            COL_BTN_ACTIVE,
            COL_ACCENT_PURPLE.with_alpha(0.52),
        ),
        1 => (
            Color::srgb_u8(0x12, 0x28, 0x2D),
            Color::srgb_u8(0x1D, 0x3E, 0x44),
            COL_ACCENT_TURQUOISE.with_alpha(0.55),
        ),
        2 => (
            Color::srgb_u8(0x2C, 0x15, 0x28),
            Color::srgb_u8(0x45, 0x20, 0x3E),
            COL_ACCENT_CRIMSON.with_alpha(0.55),
        ),
        _ => (
            Color::srgb_u8(0x17, 0x1C, 0x2B),
            Color::srgb_u8(0x24, 0x2C, 0x42),
            COL_ACCENT_GOLD.with_alpha(0.58),
        ),
    }
}

#[derive(Component, Clone, Copy)]
struct AnimatedButtonSpec {
    base_width: f32,
    hover_expand_px: f32,
}

#[derive(Component)]
struct TutorialBannerContainer;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
enum TopResourceCardKind {
    DeityIdentityAndTurn,
    Souls,
    Influence,
    Essence,
    DominionAndVictory,
}

#[derive(Component)]
struct LeftPoolSummaryText;

#[derive(Component)]
struct LeftQuickSlotButton(usize);

#[derive(Component)]
struct LeftQuickSlotLabel(usize);

#[derive(Component)]
struct RightTutorialBannerText;

#[derive(Component)]
struct RightTutorialButtonsRow;

#[derive(Component)]
struct InspectorBarLabel(usize);

#[derive(Component)]
struct InspectorBarFill(usize);

#[derive(Component)]
struct RightInspectorBodyText;

#[derive(Component)]
struct UniverseSovereigntyBannerText;

#[derive(Component)]
struct BottomEventLogText;

#[derive(Component)]
struct TabContainer(InspectorTab);

#[derive(Component)]
struct DilemmaButtonRow;

#[derive(Component)]
struct EndgameModalOverlay;

#[derive(Component)]
struct EndgameModalBodyText;

#[derive(Component)]
struct MainMenuOverlay;

#[derive(Component)]
struct MainMenuDeityNameText;

#[derive(Component)]
struct MainMenuSettingsContainer;

#[derive(Component)]
struct MainMenuSettingsText;

#[derive(Component)]
struct ButtonHoverTooltipOverlay;

#[derive(Component)]
struct ButtonHoverTooltipText;

#[derive(Component, Clone, Copy, Debug)]
enum UiAction {
    // Main Menu & Application Controls
    ToggleMainMenu,
    MainMenuStartNewGame,
    ToggleMainMenuSettings,
    CycleRandomDeityName,
    CycleUiTheme,
    CycleMapOverlayMode,
    ToggleAudio,
    ToggleBackgroundOptimization,
    ExitGame,
    // Time & Save
    SetSpeed(SimSpeed),
    StepOneTick,
    QuickSave,
    QuickLoad,
    // Left Panel Filters, Sorting & Pagination
    SetOwnershipFilter(OwnershipFilter),
    CycleTypeFilter,
    CycleSortMode,
    PrevPoolPage,
    NextPoolPage,
    SelectFilteredSlot(usize),
    // Right Panel Tab Switching
    SelectTab(InspectorTab),
    // Interactive Tutorial Controls
    ExecuteOrShowTutorialStep,
    ToggleTutorialCollapse,
    ResetTutorial,
    // Universe & Gateway Tab Actions
    SpreadProphecy,
    HarvestSouls,
    BestowMiracle,
    SeedLifeAndOrder,
    StabilizeReality,
    ExtractCosmicEssence,
    EnlightenCivilizations,
    ReshapeUniverseDomain,
    ChannelAmbientEnergy,
    SmiteRival,
    InciteRebellion,
    ScoutRegion,
    OpenConnectedGateways,
    CloseConnectedGateways,
    FocusSelectedUniverse,
    FocusLatestEventUniverse,
    ClearInspectedGateway,
    ResolveDilemmaChoice(bool),
    // Powers Tab: Unlock next tier in tree
    UnlockNextInTree(PowerTree),
    // Deities & Diplomacy Tab
    PrevRivalDeity,
    NextRivalDeity,
    SelectRivalDeity(u32),
    FocusRivalUniverse,
    SendDivineGift,
    OfferPact,
    DemandTribute,
    DeclareHolyWar,
    DisruptRivalRitual,
    SealRival,
    // Victory, Ledger & New Game Tab
    AdvanceAscension,
    ContinueInSandbox,
    DismissEndgameModal,
    StartScenario(usize),
    ToggleHighContrastSymbols,
    // Guide & Tutorial Tab
    PrevGuideChapter,
    NextGuideChapter,
    SelectGuideChapter(usize),
}

fn setup_cosmic_ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            Pickable::IGNORE,
        ))
        .with_children(|root| {
            // ==================== 1. TOP BAR (Görsel Kaynak Kartları, İlah Kimliği ve Zaman Kontrolü) ====================
            root.spawn((
                Node {
                    width: percent(100),
                    height: px(56),
                    padding: UiRect::axes(px(10), px(5)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    border: UiRect::bottom(px(2)),
                    ..default()
                },
                BackgroundColor(COL_BG_DEEP),
                BorderColor::all(COL_ACCENT_PURPLE.with_alpha(0.45)),
            ))
            .with_children(|top| {
                // Visual Resource Pills Row
                top.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: px(6),
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|cards| {
                    spawn_top_resource_card(
                        cards,
                        TopResourceCardKind::DeityIdentityAndTurn,
                        "♛ Aethelgard · Tur 0 · 1x",
                        COL_ACCENT_GOLD,
                        166.0,
                    );
                    spawn_top_resource_card(
                        cards,
                        TopResourceCardKind::Souls,
                        "◆ Ruh: 300 (+0)",
                        COL_ACCENT_PURPLE,
                        136.0,
                    );
                    spawn_top_resource_card(
                        cards,
                        TopResourceCardKind::Influence,
                        "♛ Nüfuz: 130 (+0)",
                        COL_ACCENT_GOLD,
                        138.0,
                    );
                    spawn_top_resource_card(
                        cards,
                        TopResourceCardKind::Essence,
                        "✦ Öz: 40 (+0)",
                        COL_ACCENT_TURQUOISE,
                        126.0,
                    );
                    spawn_top_resource_card(
                        cards,
                        TopResourceCardKind::DominionAndVictory,
                        "★ Tam: 1/18 · Baskın: 2/35",
                        COL_ACCENT_GOLD,
                        220.0,
                    );
                });

                // Time & System Controls Row
                top.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: px(4),
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|btns| {
                    spawn_action_btn(
                        btns,
                        "☰ Menü",
                        UiAction::ToggleMainMenu,
                        56.0,
                    );
                    spawn_action_btn(btns, "Duraklat", UiAction::SetSpeed(SimSpeed::Paused), 52.0);
                    spawn_action_btn(btns, "1x", UiAction::SetSpeed(SimSpeed::Normal1x), 24.0);
                    spawn_action_btn(btns, "2x", UiAction::SetSpeed(SimSpeed::Fast2x), 24.0);
                    spawn_action_btn(btns, "5x", UiAction::SetSpeed(SimSpeed::Ultra5x), 24.0);
                    spawn_action_btn(btns, "+1 Tur", UiAction::StepOneTick, 42.0);
                    spawn_action_btn(
                        btns,
                        "◎ Katman",
                        UiAction::CycleMapOverlayMode,
                        54.0,
                    );
                    spawn_action_btn(btns, "❖ Tema", UiAction::CycleUiTheme, 46.0);
                    spawn_action_btn(btns, "♪ Ses", UiAction::ToggleAudio, 42.0);
                    spawn_action_btn(
                        btns,
                        "Wiki (F1)",
                        UiAction::SelectTab(InspectorTab::GuideAndTutorial),
                        56.0,
                    );
                    spawn_action_btn(btns, "Kaydet", UiAction::QuickSave, 44.0);
                    spawn_action_btn(btns, "Yükle", UiAction::QuickLoad, 42.0);
                });
            });

            // ==================== 2. MIDDLE ROW (SOL HAVUZ + HARİTA + SAĞ DENETÇİ) ====================
            root.spawn((
                Node {
                    width: percent(100),
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    overflow: Overflow::clip(),
                    ..default()
                },
                Pickable::IGNORE,
            ))
            .with_children(|mid| {
                // -------- SOL PANEL: 60 EVREN HAVUZU, AİDİYET FİLTRELERİ VE SAYFALAMA --------
                mid.spawn((
                    Node {
                        width: px(244),
                        height: percent(100),
                        padding: UiRect::all(px(8)),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(4),
                        border: UiRect::right(px(1)),
                        ..default()
                    },
                    BackgroundColor(COL_BG_DEEP.with_alpha(0.94)),
                    BorderColor::all(COL_BG_PANEL),
                ))
                .with_children(|left| {
                    left.spawn((
                        Text::new("KOZMİK EVREN HAVUZU (60)"),
                        TextFont {
                            font_size: FontSize::Px(11.5),
                            ..default()
                        },
                        TextColor(COL_ACCENT_GOLD),
                    ));

                    // Ownership & Status Quick Filters (2 rows of 3)
                    left.spawn(Node {
                        width: percent(100),
                        flex_direction: FlexDirection::Row,
                        flex_wrap: FlexWrap::Wrap,
                        column_gap: px(3),
                        row_gap: px(3),
                        ..default()
                    })
                    .with_children(|f_row| {
                        for of in OwnershipFilter::ALL {
                            spawn_action_btn(
                                f_row,
                                of.label_tr(),
                                UiAction::SetOwnershipFilter(of),
                                71.0,
                            );
                        }
                    });

                    // Type Cycle + Sort Cycle Row
                    left.spawn(Node {
                        width: percent(100),
                        flex_direction: FlexDirection::Row,
                        column_gap: px(4),
                        ..default()
                    })
                    .with_children(|c_row| {
                        spawn_action_btn(c_row, "Kategori ↻", UiAction::CycleTypeFilter, 109.0);
                        spawn_action_btn(c_row, "Sırala ↻", UiAction::CycleSortMode, 109.0);
                    });

                    left.spawn(Node {
                        width: percent(100),
                        height: px(28),
                        overflow: Overflow::clip(),
                        ..default()
                    })
                    .with_children(|sum_box| {
                        sum_box.spawn((
                            LeftPoolSummaryText,
                            Text::new("Havuz yükleniyor..."),
                            TextFont {
                                font_size: FontSize::Px(10.0),
                                ..default()
                            },
                            TextColor(COL_TEXT_SECONDARY),
                        ));
                    });

                    // 8 Dynamic Universe Slots per page
                    for slot in 0..POOL_SLOTS_PER_PAGE {
                        spawn_dynamic_slot_btn(left, slot, UiAction::SelectFilteredSlot(slot));
                    }

                    // Pagination Row (< Önceki | Sonraki >)
                    left.spawn(Node {
                        width: percent(100),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        column_gap: px(4),
                        ..default()
                    })
                    .with_children(|p_row| {
                        spawn_action_btn(p_row, "< Önceki Sayfa", UiAction::PrevPoolPage, 109.0);
                        spawn_action_btn(p_row, "Sonraki Sayfa >", UiAction::NextPoolPage, 109.0);
                    });
                });

                // -------- MERKEZ: ZAFER / YENİLGİ SON-OYUN PENCERESİ (Başlangıçta Gizli) --------
                mid.spawn((
                    EndgameModalOverlay,
                    Node {
                        width: px(460),
                        padding: UiRect::all(px(16)),
                        border: UiRect::all(px(2)),
                        border_radius: BorderRadius::all(px(8)),
                        align_self: AlignSelf::Center,
                        display: Display::None,
                        flex_direction: FlexDirection::Column,
                        row_gap: px(10),
                        ..default()
                    },
                    BackgroundColor(COL_BG_DEEP.with_alpha(0.97)),
                    BorderColor::all(COL_ACCENT_GOLD),
                ))
                .with_children(|modal| {
                    modal.spawn((
                        EndgameModalBodyText,
                        Text::new("Kozmik Sonuç"),
                        TextFont {
                            font_size: FontSize::Px(11.8),
                            ..default()
                        },
                        TextColor(COL_TEXT_PRIMARY),
                    ));

                    modal
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            flex_wrap: FlexWrap::Wrap,
                            column_gap: px(6),
                            row_gap: px(6),
                            ..default()
                        })
                        .with_children(|m_btns| {
                            spawn_action_btn(
                                m_btns,
                                "★ Sonsuz Havuz Modunda Devam Et (Sandbox)",
                                UiAction::ContinueInSandbox,
                                420.0,
                            );
                            spawn_action_btn(
                                m_btns,
                                "Yeni Oyun: Kozmik Yükseliş",
                                UiAction::StartScenario(0),
                                207.0,
                            );
                            spawn_action_btn(
                                m_btns,
                                "Yeni Oyun: Zor Senaryo",
                                UiAction::StartScenario(1),
                                207.0,
                            );
                            spawn_action_btn(
                                m_btns,
                                "Pencereyi Kapat (Haritayı İncele)",
                                UiAction::DismissEndgameModal,
                                420.0,
                            );
                        });
                });

                // -------- SAĞ PANEL: 5 SEKMELİ DENETÇİ, GÖRSEL ÇUBUKLAR & EMİRLER --------
                mid.spawn((
                    Node {
                        width: px(384),
                        height: percent(100),
                        padding: UiRect::all(px(8)),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(5),
                        border: UiRect::left(px(1)),
                        ..default()
                    },
                    BackgroundColor(COL_BG_DEEP.with_alpha(0.95)),
                    BorderColor::all(COL_BG_PANEL),
                ))
                .with_children(|right| {
                    // 5-Tab Single Row Selector (NoWrap + safe widths so tabs never wrap or shift)
                    right
                        .spawn(Node {
                            width: percent(100),
                            flex_direction: FlexDirection::Row,
                            flex_wrap: FlexWrap::NoWrap,
                            column_gap: px(3),
                            ..default()
                        })
                        .with_children(|tabs| {
                            for tab in InspectorTab::ALL {
                                spawn_action_btn(
                                    tabs,
                                    tab.label_tr(),
                                    UiAction::SelectTab(tab),
                                    69.0,
                                );
                            }
                        });

                    // Collapsible Interactive Tutorial Banner + "▶ Uygula" & "Daralt/Aç" (Fixed height per state so text never shifts layout)
                    right
                        .spawn((
                            TutorialBannerContainer,
                            Node {
                                width: percent(100),
                                height: px(60),
                                padding: UiRect::all(px(6)),
                                border: UiRect::all(px(1)),
                                border_radius: BorderRadius::all(px(5)),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::SpaceBetween,
                                overflow: Overflow::clip(),
                                ..default()
                            },
                            BackgroundColor(COL_BG_PANEL),
                            BorderColor::all(COL_ACCENT_GOLD.with_alpha(0.55)),
                        ))
                        .with_children(|t_box| {
                            t_box.spawn((
                                RightTutorialBannerText,
                                Text::new("Eğitici görev yükleniyor..."),
                                TextFont {
                                    font_size: FontSize::Px(10.0),
                                    ..default()
                                },
                                TextColor(COL_ACCENT_GOLD),
                            ));

                            t_box
                                .spawn((
                                    RightTutorialButtonsRow,
                                    Node {
                                        flex_direction: FlexDirection::Row,
                                        column_gap: px(4),
                                        ..default()
                                    },
                                ))
                                .with_children(|t_btns| {
                                    spawn_action_btn(
                                        t_btns,
                                        "▶ Görevi Şimdi Uygula",
                                        UiAction::ExecuteOrShowTutorialStep,
                                        142.0,
                                    );
                                    spawn_action_btn(
                                        t_btns,
                                        "Wiki (F1)",
                                        UiAction::SelectTab(InspectorTab::GuideAndTutorial),
                                        106.0,
                                    );
                                    spawn_action_btn(
                                        t_btns,
                                        "Daralt/Aç",
                                        UiAction::ToggleTutorialCollapse,
                                        86.0,
                                    );
                                });
                        });

                    // Visual Status Progress Bars Card (3 Bars: Stability/Dominion, Danger/Transcendence, Allegiance/Creation)
                    right
                        .spawn((
                            Node {
                                width: percent(100),
                                padding: UiRect::all(px(6)),
                                border: UiRect::all(px(1)),
                                border_radius: BorderRadius::all(px(5)),
                                flex_direction: FlexDirection::Column,
                                row_gap: px(3),
                                ..default()
                            },
                            BackgroundColor(COL_BG_PANEL),
                            BorderColor::all(COL_ACCENT_TURQUOISE.with_alpha(0.35)),
                        ))
                        .with_children(|bars_box| {
                            for idx in 0..3 {
                                spawn_visual_progress_bar(bars_box, idx);
                            }
                        });

                    // Inspector Main Details Box (flex_basis: 0 + min_height: 0 so text changes NEVER push buttons below!)
                    right
                        .spawn((
                            Node {
                                width: percent(100),
                                padding: UiRect::all(px(8)),
                                border: UiRect::all(px(1)),
                                border_radius: BorderRadius::all(px(6)),
                                flex_grow: 1.0,
                                flex_basis: px(0),
                                min_height: px(0),
                                overflow: Overflow::clip(),
                                ..default()
                            },
                            BackgroundColor(COL_BG_PANEL),
                            BorderColor::all(COL_ACCENT_PURPLE.with_alpha(0.35)),
                        ))
                        .with_children(|box_node| {
                            box_node.spawn((
                                RightInspectorBodyText,
                                Text::new("Denetçi yükleniyor..."),
                                TextFont {
                                    font_size: FontSize::Px(10.3),
                                    ..default()
                                },
                                TextColor(COL_TEXT_PRIMARY),
                            ));
                        });

                    // Contextual Action Containers per Tab
                    // Tab 1: Universe & Gateway Actions (Split into Infiltration vs. Sovereign Authority!)
                    right
                        .spawn((
                            TabContainer(InspectorTab::Universe),
                            Node {
                                width: percent(100),
                                flex_direction: FlexDirection::Column,
                                row_gap: px(3),
                                ..default()
                            },
                        ))
                        .with_children(|u_col| {
                            // Dilemma Row (Always present so buttons never jump when a dilemma triggers)
                            u_col
                                .spawn((
                                    DilemmaButtonRow,
                                    Node {
                                        width: percent(100),
                                        flex_direction: FlexDirection::Row,
                                        column_gap: px(4),
                                        ..default()
                                    },
                                ))
                                .with_children(|d_row| {
                                    spawn_action_btn(
                                        d_row,
                                        "★ İkilem A: Kutsama (+İstikrar)",
                                        UiAction::ResolveDilemmaChoice(true),
                                        176.0,
                                    );
                                    spawn_action_btn(
                                        d_row,
                                        "★ İkilem B: Sömür (+Ruh/Öz)",
                                        UiAction::ResolveDilemmaChoice(false),
                                        176.0,
                                    );
                                });

                            // Section 1: Infiltration & Contestation (Available in ALL discovered universes)
                            u_col.spawn((
                                Text::new("⚔ SIZMA & MÜCADELE (Her Keşfedilen Evrende Açık)"),
                                TextFont {
                                    font_size: FontSize::Px(9.4),
                                    ..default()
                                },
                                TextColor(COL_ACCENT_TURQUOISE),
                            ));

                            u_col
                                .spawn(Node {
                                    width: percent(100),
                                    flex_direction: FlexDirection::Row,
                                    flex_wrap: FlexWrap::Wrap,
                                    column_gap: px(4),
                                    row_gap: px(3),
                                    ..default()
                                })
                                .with_children(|inf_btns| {
                                    spawn_action_btn(
                                        inf_btns,
                                        "Kehanet Yay (20R 25N)",
                                        UiAction::SpreadProphecy,
                                        176.0,
                                    );
                                    spawn_action_btn(
                                        inf_btns,
                                        "Rakip Vur (75R 35N 10Ö)",
                                        UiAction::SmiteRival,
                                        176.0,
                                    );
                                    spawn_action_btn(
                                        inf_btns,
                                        "İsyan (30R 40N)",
                                        UiAction::InciteRebellion,
                                        116.0,
                                    );
                                    spawn_action_btn(
                                        inf_btns,
                                        "Geçit Aç (40R 20N 10Ö)",
                                        UiAction::OpenConnectedGateways,
                                        116.0,
                                    );
                                    spawn_action_btn(
                                        inf_btns,
                                        "◎ Son Olay (E)",
                                        UiAction::FocusLatestEventUniverse,
                                        116.0,
                                    );
                                });

                            // Section 2: Sovereign Authority Commands (ONLY unlocked in Player-Dominated universes!)
                            u_col.spawn((
                                UniverseSovereigntyBannerText,
                                Text::new(
                                    "♛ HÂKİMİYET & EVREN ETKİLEŞİM EMİRLERİ (Söz Sahibi Evrenlerde)",
                                ),
                                TextFont {
                                    font_size: FontSize::Px(9.4),
                                    ..default()
                                },
                                TextColor(COL_ACCENT_GOLD),
                            ));

                            u_col
                                .spawn(Node {
                                    width: percent(100),
                                    flex_direction: FlexDirection::Row,
                                    flex_wrap: FlexWrap::Wrap,
                                    column_gap: px(4),
                                    row_gap: px(3),
                                    ..default()
                                })
                                .with_children(|sov_btns| {
                                    spawn_action_btn(
                                        sov_btns,
                                        "Ruh Hasadı (25N -> +Ruh)",
                                        UiAction::HarvestSouls,
                                        176.0,
                                    );
                                    spawn_action_btn(
                                        sov_btns,
                                        "Mucize Bahşet (60R 25N)",
                                        UiAction::BestowMiracle,
                                        176.0,
                                    );
                                    spawn_action_btn(
                                        sov_btns,
                                        "Kozmik Öz Çıkar (65R 25N)",
                                        UiAction::ExtractCosmicEssence,
                                        176.0,
                                    );
                                    spawn_action_btn(
                                        sov_btns,
                                        "☀ Uygarlığı Aydınlat (35R 20N)",
                                        UiAction::EnlightenCivilizations,
                                        176.0,
                                    );
                                    spawn_action_btn(
                                        sov_btns,
                                        "☸ Dokuyu Dönüştür (55R 25N 20Ö)",
                                        UiAction::ReshapeUniverseDomain,
                                        176.0,
                                    );
                                    spawn_action_btn(
                                        sov_btns,
                                        "⚡ Kozmik Rezonans (30N 10Ö)",
                                        UiAction::ChannelAmbientEnergy,
                                        176.0,
                                    );
                                    spawn_action_btn(
                                        sov_btns,
                                        "Yaşam Tohumla (110R 40N 25Ö)",
                                        UiAction::SeedLifeAndOrder,
                                        176.0,
                                    );
                                    spawn_action_btn(
                                        sov_btns,
                                        "Gerçekliği Onar (85R 35N 20Ö)",
                                        UiAction::StabilizeReality,
                                        176.0,
                                    );
                                    spawn_action_btn(
                                        sov_btns,
                                        "Keşif (20R 20N)",
                                        UiAction::ScoutRegion,
                                        86.0,
                                    );
                                    spawn_action_btn(
                                        sov_btns,
                                        "Geçit Kapat",
                                        UiAction::CloseConnectedGateways,
                                        86.0,
                                    );
                                    spawn_action_btn(
                                        sov_btns,
                                        "★ Odaklan (F)",
                                        UiAction::FocusSelectedUniverse,
                                        86.0,
                                    );
                                    spawn_action_btn(
                                        sov_btns,
                                        "Evrene Dön",
                                        UiAction::ClearInspectedGateway,
                                        86.0,
                                    );
                                });
                        });

                    // Tab 2: Power Trees Actions
                    right
                        .spawn((
                            TabContainer(InspectorTab::Powers),
                            Node {
                                width: percent(100),
                                display: Display::None,
                                flex_direction: FlexDirection::Column,
                                row_gap: px(3),
                                ..default()
                            },
                        ))
                        .with_children(|p_btns| {
                            for tree in PowerTree::ALL {
                                spawn_action_btn(
                                    p_btns,
                                    &format!("{} Ağacını Geliştir", tree.name_tr()),
                                    UiAction::UnlockNextInTree(tree),
                                    356.0,
                                );
                            }
                        });

                    // Tab 3: Upgraded Deities, Direct Selection, Territory Locator & Diplomacy Menu
                    right
                        .spawn((
                            TabContainer(InspectorTab::Deities),
                            Node {
                                width: percent(100),
                                display: Display::None,
                                flex_direction: FlexDirection::Column,
                                row_gap: px(3),
                                ..default()
                            },
                        ))
                        .with_children(|d_col| {
                            // Direct clickable buttons for all 8 Rival Deities (2 rows of 4)
                            d_col
                                .spawn(Node {
                                    width: percent(100),
                                    flex_direction: FlexDirection::Row,
                                    flex_wrap: FlexWrap::Wrap,
                                    column_gap: px(3),
                                    row_gap: px(3),
                                    ..default()
                                })
                                .with_children(|r_grid| {
                                    let rival_labels = [
                                        (1u32, "#1 Vaelkor"),
                                        (2u32, "#2 Morgeth"),
                                        (3u32, "#3 Sylvara"),
                                        (4u32, "#4 Aurelion"),
                                        (5u32, "#5 Solinvic"),
                                        (6u32, "#6 Nyxarion"),
                                        (7u32, "#7 Mechanon"),
                                        (8u32, "#8 Vespera"),
                                    ];
                                    for (r_id, lbl) in rival_labels {
                                        spawn_action_btn(
                                            r_grid,
                                            lbl,
                                            UiAction::SelectRivalDeity(r_id),
                                            86.0,
                                        );
                                    }
                                });

                            // Interactive Diplomacy, Territory Focus & Warfare Menu
                            d_col
                                .spawn(Node {
                                    width: percent(100),
                                    flex_direction: FlexDirection::Row,
                                    flex_wrap: FlexWrap::Wrap,
                                    column_gap: px(4),
                                    row_gap: px(3),
                                    ..default()
                                })
                                .with_children(|d_btns| {
                                    spawn_action_btn(
                                        d_btns,
                                        "< Önceki Rakip",
                                        UiAction::PrevRivalDeity,
                                        106.0,
                                    );
                                    spawn_action_btn(
                                        d_btns,
                                        "◎ Rakibin Evrenini Bul",
                                        UiAction::FocusRivalUniverse,
                                        136.0,
                                    );
                                    spawn_action_btn(
                                        d_btns,
                                        "Sonraki Rakip >",
                                        UiAction::NextRivalDeity,
                                        106.0,
                                    );
                                    spawn_action_btn(
                                        d_btns,
                                        "✦ Ruh Armağanı (70R 5Ö)",
                                        UiAction::SendDivineGift,
                                        176.0,
                                    );
                                    spawn_action_btn(
                                        d_btns,
                                        "⚖ Ateşkes Paktı (35R 45N)",
                                        UiAction::OfferPact,
                                        176.0,
                                    );
                                    spawn_action_btn(
                                        d_btns,
                                        "♛ Haraç İste (20R 35N)",
                                        UiAction::DemandTribute,
                                        176.0,
                                    );
                                    spawn_action_btn(
                                        d_btns,
                                        "⚔ Kutsal Rekabet (60R 40N 10Ö)",
                                        UiAction::DeclareHolyWar,
                                        176.0,
                                    );
                                    spawn_action_btn(
                                        d_btns,
                                        "⚡ Ritüeli Sabote Et (-%20 · 220R 110N 45Ö)",
                                        UiAction::DisruptRivalRitual,
                                        356.0,
                                    );
                                    spawn_action_btn(
                                        d_btns,
                                        "★ Zayıflamış Tanrıyı Mühürle (320R 150N 80Ö)",
                                        UiAction::SealRival,
                                        356.0,
                                    );
                                });
                        });

                    // Tab 4: Victory, Ledger & New Game Actions
                    right
                        .spawn((
                            TabContainer(InspectorTab::VictoryAndLedger),
                            Node {
                                width: percent(100),
                                display: Display::None,
                                flex_direction: FlexDirection::Row,
                                flex_wrap: FlexWrap::Wrap,
                                column_gap: px(4),
                                row_gap: px(3),
                                ..default()
                            },
                        ))
                        .with_children(|v_btns| {
                            spawn_action_btn(
                                v_btns,
                                "★ Aşkınlık Ritüelini İlerlet (+%10 · 10 Aşama)",
                                UiAction::AdvanceAscension,
                                356.0,
                            );
                            spawn_action_btn(
                                v_btns,
                                "Yeni: Kozmik Yükseliş",
                                UiAction::StartScenario(0),
                                116.0,
                            );
                            spawn_action_btn(
                                v_btns,
                                "Yeni: Zor Senaryo",
                                UiAction::StartScenario(1),
                                116.0,
                            );
                            spawn_action_btn(
                                v_btns,
                                "Yeni: Serbest Mod",
                                UiAction::StartScenario(2),
                                116.0,
                            );
                            spawn_action_btn(
                                v_btns,
                                "Tehlike Uyarı Sembolleri Aç/Kapat",
                                UiAction::ToggleHighContrastSymbols,
                                356.0,
                            );
                        });

                    // Tab 5: Interactive Tutorial & Guide Actions
                    right
                        .spawn((
                            TabContainer(InspectorTab::GuideAndTutorial),
                            Node {
                                width: percent(100),
                                display: Display::None,
                                flex_direction: FlexDirection::Row,
                                flex_wrap: FlexWrap::Wrap,
                                column_gap: px(4),
                                row_gap: px(3),
                                ..default()
                            },
                        ))
                        .with_children(|g_btns| {
                            spawn_action_btn(
                                g_btns,
                                "▶ Aktif Görevi Uygula",
                                UiAction::ExecuteOrShowTutorialStep,
                                176.0,
                            );
                            spawn_action_btn(
                                g_btns,
                                "Eğitimi Sıfırla",
                                UiAction::ResetTutorial,
                                176.0,
                            );
                            spawn_action_btn(
                                g_btns,
                                "< Önceki Wiki Konusu",
                                UiAction::PrevGuideChapter,
                                176.0,
                            );
                            spawn_action_btn(
                                g_btns,
                                "Sonraki Wiki Konusu >",
                                UiAction::NextGuideChapter,
                                176.0,
                            );
                            let wiki_labels = [
                                (0usize, "1:Evrenler"),
                                (1usize, "2:Ekonomi"),
                                (2usize, "3:Emirler"),
                                (3usize, "4:Güçler"),
                                (4usize, "5:Türler"),
                                (5usize, "6:Olaylar"),
                                (6usize, "7:İlahlar"),
                                (7usize, "8:Zafer"),
                            ];
                            for (idx, lbl) in wiki_labels {
                                spawn_action_btn(
                                    g_btns,
                                    lbl,
                                    UiAction::SelectGuideChapter(idx),
                                    86.0,
                                );
                            }
                        });
                });
            });

            // ==================== 3. BOTTOM EVENT LOG & STATUS BAR ====================
            root.spawn((
                Node {
                    width: percent(100),
                    height: px(90),
                    padding: UiRect::axes(px(14), px(6)),
                    border: UiRect::top(px(2)),
                    ..default()
                },
                BackgroundColor(COL_BG_DEEP),
                BorderColor::all(COL_BG_PANEL),
            ))
            .with_children(|bottom| {
                bottom.spawn((
                    BottomEventLogText,
                    Text::new("Kozmik Olay Günlüğü hazır."),
                    TextFont {
                        font_size: FontSize::Px(10.8),
                        ..default()
                    },
                    TextColor(COL_TEXT_PRIMARY),
                ));
            });

            // ==================== 4. FULL-SCREEN ANIMATED MAIN MENU OVERLAY ====================
            root.spawn((
                MainMenuOverlay,
                Node {
                    position_type: PositionType::Absolute,
                    left: px(0),
                    top: px(0),
                    width: percent(100),
                    height: percent(100),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ZIndex(60),
                BackgroundColor(COL_BG_VOID.with_alpha(0.92)),
            ))
            .with_children(|overlay| {
                overlay
                    .spawn((
                        Node {
                            width: px(510),
                            padding: UiRect::all(px(22)),
                            border: UiRect::all(px(2)),
                            border_radius: BorderRadius::all(px(12)),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            row_gap: px(12),
                            ..default()
                        },
                        BackgroundColor(COL_BG_DEEP.with_alpha(0.98)),
                        BorderColor::all(COL_ACCENT_GOLD),
                    ))
                    .with_children(|card| {
                        // Game Title & Subtitle
                        card.spawn((
                            Text::new("✦ KozMO ✦"),
                            TextFont {
                                font_size: FontSize::Px(22.0),
                                ..default()
                            },
                            TextColor(COL_ACCENT_GOLD),
                        ));
                        card.spawn((
                            Text::new("KOZMİK SANDBOX STRATEJİ · 60 EVREN · 9 İLAH"),
                            TextFont {
                                font_size: FontSize::Px(11.0),
                                ..default()
                            },
                            TextColor(COL_ACCENT_TURQUOISE),
                        ));

                        // Player Deity Name Input & Randomizer Box
                        card.spawn((
                            Node {
                                width: percent(100),
                                padding: UiRect::all(px(10)),
                                border: UiRect::all(px(1)),
                                border_radius: BorderRadius::all(px(8)),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                row_gap: px(6),
                                ..default()
                            },
                            BackgroundColor(COL_BG_PANEL),
                            BorderColor::all(COL_ACCENT_PURPLE.with_alpha(0.70)),
                        ))
                        .with_children(|name_box| {
                            name_box.spawn((
                                MainMenuDeityNameText,
                                Text::new("♛ İLAH ADIN: Aethelgard|"),
                                TextFont {
                                    font_size: FontSize::Px(13.5),
                                    ..default()
                                },
                                TextColor(COL_TEXT_PRIMARY),
                            ));
                            name_box.spawn((
                                Text::new(
                                    "Klavyeden harf yazarak (Backspace ile silerek) veya alttaki butonla ilah adını belirle:",
                                ),
                                TextFont {
                                    font_size: FontSize::Px(9.8),
                                    ..default()
                                },
                                TextColor(COL_TEXT_SECONDARY),
                            ));
                            spawn_action_btn(
                                name_box,
                                "↻ Rastgele Kozmik İlah Adı Seç",
                                UiAction::CycleRandomDeityName,
                                260.0,
                            );
                        });

                        // Vertical Top-to-Bottom Animated Main Menu Buttons:
                        // 1. YENİ OYUN
                        // 2. AYARLAR
                        // 3. ÇIKIŞ
                        // (+ Mevcut Kozmosa Dön)
                        card.spawn(Node {
                            width: percent(100),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            row_gap: px(8),
                            ..default()
                        })
                        .with_children(|v_menu| {
                            spawn_main_menu_btn(
                                v_menu,
                                "✦ YENİ OYUN BAŞLAT",
                                UiAction::MainMenuStartNewGame,
                                390.0,
                            );
                            spawn_main_menu_btn(
                                v_menu,
                                "⚙ AYARLAR",
                                UiAction::ToggleMainMenuSettings,
                                390.0,
                            );
                            spawn_main_menu_btn(
                                v_menu,
                                "✖ ÇIKIŞ",
                                UiAction::ExitGame,
                                390.0,
                            );
                            spawn_action_btn(
                                v_menu,
                                "▶ Mevcut Kozmosa Dön / Devam Et (Esc)",
                                UiAction::ToggleMainMenu,
                                310.0,
                            );
                        });

                        // Collapsible Settings Sub-Panel inside Main Menu
                        card.spawn((
                            MainMenuSettingsContainer,
                            Node {
                                width: percent(100),
                                padding: UiRect::all(px(10)),
                                border: UiRect::all(px(1)),
                                border_radius: BorderRadius::all(px(8)),
                                display: Display::None,
                                flex_direction: FlexDirection::Column,
                                row_gap: px(6),
                                ..default()
                            },
                            BackgroundColor(COL_BG_PANEL),
                            BorderColor::all(COL_ACCENT_TURQUOISE.with_alpha(0.65)),
                        ))
                        .with_children(|s_box| {
                            s_box.spawn((
                                MainMenuSettingsText,
                                Text::new("Ayarlar yükleniyor..."),
                                TextFont {
                                    font_size: FontSize::Px(10.5),
                                    ..default()
                                },
                                TextColor(COL_TEXT_PRIMARY),
                            ));

                            s_box
                                .spawn(Node {
                                    width: percent(100),
                                    flex_direction: FlexDirection::Row,
                                    flex_wrap: FlexWrap::Wrap,
                                    column_gap: px(5),
                                    row_gap: px(4),
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                })
                                .with_children(|s_btns| {
                                    spawn_action_btn(
                                        s_btns,
                                        "Senaryo 1: Dengeli Yükseliş",
                                        UiAction::StartScenario(0),
                                        142.0,
                                    );
                                    spawn_action_btn(
                                        s_btns,
                                        "Senaryo 2: Büyük Üstat (Zor)",
                                        UiAction::StartScenario(1),
                                        142.0,
                                    );
                                    spawn_action_btn(
                                        s_btns,
                                        "Senaryo 3: Sonsuz Sandbox",
                                        UiAction::StartScenario(2),
                                        142.0,
                                    );
                                    spawn_action_btn(
                                        s_btns,
                                        "Tehlike Sembolleri (▲) Aç/Kapat",
                                        UiAction::ToggleHighContrastSymbols,
                                        142.0,
                                    );
                                    spawn_action_btn(
                                        s_btns,
                                        "Uzak Evren Optimizasyonu",
                                        UiAction::ToggleBackgroundOptimization,
                                        142.0,
                                    );
                                    spawn_action_btn(
                                        s_btns,
                                        "❖ Tema Değiştir (T)",
                                        UiAction::CycleUiTheme,
                                        142.0,
                                    );
                                    spawn_action_btn(
                                        s_btns,
                                        "♪ Ses Aç/Kapat (V)",
                                        UiAction::ToggleAudio,
                                        142.0,
                                    );
                                });
                        });
                    });
            });

            // ==================== 5. FLOATING 2-SECOND BUTTON HOVER TOOLTIP OVERLAY ====================
            root.spawn((
                ButtonHoverTooltipOverlay,
                Node {
                    position_type: PositionType::Absolute,
                    left: px(258),
                    bottom: px(98),
                    width: px(540),
                    padding: UiRect::all(px(10)),
                    border: UiRect::all(px(1.5)),
                    border_radius: BorderRadius::all(px(6)),
                    display: Display::None,
                    ..default()
                },
                ZIndex(95),
                Pickable::IGNORE,
                BackgroundColor(COL_BG_DEEP.with_alpha(0.97)),
                BorderColor::all(COL_ACCENT_GOLD),
            ))
            .with_children(|tip| {
                tip.spawn((
                    ButtonHoverTooltipText,
                    Text::new(""),
                    TextFont {
                        font_size: FontSize::Px(10.6),
                        ..default()
                    },
                    TextColor(COL_TEXT_PRIMARY),
                ));
            });
        });
}

fn spawn_top_resource_card(
    parent: &mut ChildSpawnerCommands,
    kind: TopResourceCardKind,
    initial_text: &str,
    accent: Color,
    min_width_px: f32,
) {
    parent
        .spawn((
            Node {
                min_width: px(min_width_px),
                height: px(28),
                padding: UiRect::axes(px(8), px(4)),
                border: UiRect::all(px(1)),
                border_radius: BorderRadius::all(px(5)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(COL_BG_PANEL),
            BorderColor::all(accent.with_alpha(0.65)),
        ))
        .with_children(|card| {
            card.spawn((
                kind,
                Text::new(initial_text),
                TextFont {
                    font_size: FontSize::Px(11.0),
                    ..default()
                },
                TextColor(COL_TEXT_PRIMARY),
            ));
        });
}

fn spawn_visual_progress_bar(parent: &mut ChildSpawnerCommands, index: usize) {
    parent
        .spawn(Node {
            width: percent(100),
            flex_direction: FlexDirection::Column,
            row_gap: px(2),
            ..default()
        })
        .with_children(|col| {
            col.spawn((
                InspectorBarLabel(index),
                Text::new("Durum Çubuğu"),
                TextFont {
                    font_size: FontSize::Px(9.8),
                    ..default()
                },
                TextColor(COL_TEXT_SECONDARY),
            ));

            col.spawn((
                Node {
                    width: percent(100),
                    height: px(7),
                    border_radius: BorderRadius::all(px(3)),
                    overflow: Overflow::clip(),
                    ..default()
                },
                BackgroundColor(COL_BG_DEEP),
            ))
            .with_children(|track| {
                track.spawn((
                    InspectorBarFill(index),
                    Node {
                        width: percent(50),
                        height: percent(100),
                        border_radius: BorderRadius::all(px(3)),
                        ..default()
                    },
                    BackgroundColor(COL_ACCENT_TURQUOISE),
                ));
            });
        });
}

fn spawn_action_btn(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    action: UiAction,
    width_px: f32,
) {
    parent
        .spawn((
            Button,
            Hovered::default(),
            action,
            Node {
                width: px(width_px),
                height: px(22),
                padding: UiRect::axes(px(4), px(2)),
                border: UiRect::all(px(1)),
                border_radius: BorderRadius::all(px(4)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(COL_BTN_NORMAL),
            BorderColor::all(COL_ACCENT_PURPLE.with_alpha(0.45)),
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                TextFont {
                    font_size: FontSize::Px(9.7),
                    ..default()
                },
                TextColor(COL_TEXT_PRIMARY),
            ));
        });
}

fn spawn_main_menu_btn(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    action: UiAction,
    width_px: f32,
) {
    parent
        .spawn((
            Button,
            Hovered::default(),
            action,
            AnimatedButtonSpec {
                base_width: width_px,
                hover_expand_px: 14.0,
            },
            Node {
                width: px(width_px),
                height: px(36),
                padding: UiRect::axes(px(14), px(6)),
                border: UiRect::all(px(2)),
                border_radius: BorderRadius::all(px(7)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(COL_BTN_ACTIVE),
            BorderColor::all(COL_ACCENT_GOLD.with_alpha(0.85)),
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                TextFont {
                    font_size: FontSize::Px(13.0),
                    ..default()
                },
                TextColor(COL_TEXT_PRIMARY),
            ));
        });
}

fn spawn_dynamic_slot_btn(parent: &mut ChildSpawnerCommands, slot_idx: usize, action: UiAction) {
    parent
        .spawn((
            Button,
            Hovered::default(),
            LeftQuickSlotButton(slot_idx),
            action,
            Node {
                width: percent(100),
                height: px(23),
                padding: UiRect::axes(px(6), px(2)),
                border: UiRect::all(px(1)),
                border_radius: BorderRadius::all(px(4)),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(COL_BTN_NORMAL),
            BorderColor::all(COL_ACCENT_TURQUOISE.with_alpha(0.35)),
        ))
        .with_children(|btn| {
            btn.spawn((
                LeftQuickSlotLabel(slot_idx),
                Text::new(format!("#{} Evren", slot_idx + 1)),
                TextFont {
                    font_size: FontSize::Px(10.0),
                    ..default()
                },
                TextColor(COL_TEXT_PRIMARY),
            ));
        });
}

fn on_ui_button_activate(
    activate: On<Activate>,
    q_actions: Query<&UiAction>,
    mut runtime: ResMut<GameRuntimeState>,
    mut q_tab_containers: Query<(&TabContainer, &mut Node)>,
    mut app_exit: MessageWriter<AppExit>,
) {
    let Ok(&action) = q_actions.get(activate.entity) else {
        return;
    };

    let sel_u = runtime.selected_universe_id;
    let sel_rival = runtime.selected_rival_deity_id;
    let prev_tutorial = runtime.sim.tutorial_step;
    let prev_outcome_ongoing = matches!(runtime.sim.outcome, GameOutcome::Ongoing);

    match action {
        UiAction::ToggleMainMenu => {
            runtime.show_main_menu = !runtime.show_main_menu;
            runtime.status_message = if runtime.show_main_menu {
                "Ana Menü açıldı.".to_string()
            } else {
                format!(
                    "Kozmik haritaya dönüldü — İlah: {}",
                    runtime.sim.player_deity().name
                )
            };
        }
        UiAction::MainMenuStartNewGame => {
            let idx = runtime.active_scenario_index;
            runtime.start_scenario(idx);
        }
        UiAction::ToggleMainMenuSettings => {
            runtime.main_menu_settings_open = !runtime.main_menu_settings_open;
        }
        UiAction::CycleRandomDeityName => {
            runtime.cycle_random_deity_name();
        }
        UiAction::CycleUiTheme => {
            runtime.cycle_ui_theme();
        }
        UiAction::CycleMapOverlayMode => {
            runtime.cycle_map_overlay_mode();
        }
        UiAction::ToggleAudio => {
            runtime.toggle_audio();
        }
        UiAction::ToggleBackgroundOptimization => {
            runtime.sim.enable_background_optimization =
                !runtime.sim.enable_background_optimization;
            runtime.status_message = format!(
                "Uzak evren arka plan optimizasyonu: {}",
                if runtime.sim.enable_background_optimization {
                    "AÇIK"
                } else {
                    "KAPALI"
                }
            );
        }
        UiAction::ExitGame => {
            app_exit.write(AppExit::Success);
        }
        UiAction::SetSpeed(spd) => {
            runtime.speed = spd;
            runtime.status_message = format!("Zaman akışı: {}", spd.label_tr());
        }
        UiAction::StepOneTick => {
            runtime.step_tick_with_visual_feedback();
            runtime.anim_time += 0.45;
            runtime.status_message =
                format!("Simülasyon 1 tur ilerletildi (Tur {}).", runtime.sim.tick);
        }
        UiAction::QuickSave => save_game_state(&mut runtime),
        UiAction::QuickLoad => load_game_state(&mut runtime),
        UiAction::SetOwnershipFilter(of) => {
            runtime.ownership_filter = of;
            runtime.pool_page = 0;
            runtime.status_message = format!("Aidiyet/Durum filtresi: {}", of.label_tr());
        }
        UiAction::CycleTypeFilter => {
            runtime.cycle_type_filter();
            runtime.status_message = match runtime.type_filter {
                Some(t) => format!("Evren kategorisi filtresi: {}", t.name_tr()),
                None => "Evren kategorisi filtresi: Tüm Türler".to_string(),
            };
        }
        UiAction::CycleSortMode => {
            runtime.sort_mode = runtime.sort_mode.next();
            runtime.pool_page = 0;
            runtime.status_message = format!("Sıralama ölçütü: {}", runtime.sort_mode.label_tr());
        }
        UiAction::PrevPoolPage => {
            let total = runtime.filtered_sorted_universes().len();
            let max_page = total.saturating_sub(1) / POOL_SLOTS_PER_PAGE;
            runtime.pool_page = if runtime.pool_page == 0 {
                max_page
            } else {
                runtime.pool_page - 1
            };
        }
        UiAction::NextPoolPage => {
            let total = runtime.filtered_sorted_universes().len();
            let max_page = total.saturating_sub(1) / POOL_SLOTS_PER_PAGE;
            runtime.pool_page = if runtime.pool_page >= max_page {
                0
            } else {
                runtime.pool_page + 1
            };
        }
        UiAction::SelectFilteredSlot(slot) => {
            let list = runtime.filtered_sorted_universes();
            let abs_idx = runtime.pool_page * POOL_SLOTS_PER_PAGE + slot;
            if let Some(&u_id) = list.get(abs_idx) {
                runtime.selected_universe_id = u_id;
                runtime.inspected_gateway = None;
                let u_pos = Vec2::new(
                    runtime.sim.universes[u_id as usize].position[0],
                    runtime.sim.universes[u_id as usize].position[1],
                );
                runtime.camera_snap_target = Some(u_pos);
                runtime.spawn_universe_vfx(u_id, MapVfxKind::SelectionPulse, 0.9);
                let u_name = runtime.sim.universes[u_id as usize].name.clone();
                runtime.status_message = format!("{u_name} seçildi.");
            }
        }
        UiAction::SelectTab(tab) => {
            runtime.active_tab = tab;
            for (tc, mut node) in &mut q_tab_containers {
                node.display = if tc.0 == tab {
                    Display::Flex
                } else {
                    Display::None
                };
            }
        }
        UiAction::ExecuteOrShowTutorialStep => {
            runtime.execute_or_highlight_tutorial_step();
        }
        UiAction::ToggleTutorialCollapse => {
            runtime.tutorial_collapsed = !runtime.tutorial_collapsed;
        }
        UiAction::ResetTutorial => {
            runtime.tutorial_collapsed = false;
            runtime.reset_tutorial();
        }
        UiAction::ClearInspectedGateway => {
            runtime.inspected_gateway = None;
            runtime.status_message = "Evren denetçisi görünümüne dönüldü.".to_string();
        }
        UiAction::SpreadProphecy => {
            runtime.inspected_gateway = None;
            let res = runtime.sim.execute_action(
                PLAYER_DEITY_ID,
                DivineAction::SpreadProphecy { universe_id: sel_u },
            );
            if res.is_ok() {
                runtime.ui_resource_flash_timer = 0.5;
                runtime.spawn_universe_vfx(sel_u, MapVfxKind::ProphecyRings, 1.25);
                runtime.spawn_universe_toast(sel_u, "+Kehanet & İnanç", COL_ACCENT_GOLD);
            }
            runtime.status_message = res.unwrap_or_else(|e| e);
        }
        UiAction::HarvestSouls => {
            runtime.inspected_gateway = None;
            let res = runtime.sim.execute_action(
                PLAYER_DEITY_ID,
                DivineAction::HarvestSouls { universe_id: sel_u },
            );
            if res.is_ok() {
                runtime.ui_resource_flash_timer = 0.65;
                runtime.spawn_universe_vfx(sel_u, MapVfxKind::SoulHarvestSwirl, 1.25);
                runtime.spawn_universe_toast(sel_u, "+Ruh Hasadı", COL_ACCENT_PURPLE);
            }
            runtime.status_message = res.unwrap_or_else(|e| e);
        }
        UiAction::BestowMiracle => {
            runtime.inspected_gateway = None;
            let res = runtime.sim.execute_action(
                PLAYER_DEITY_ID,
                DivineAction::BestowMiracle { universe_id: sel_u },
            );
            if res.is_ok() {
                runtime.ui_resource_flash_timer = 0.5;
                runtime.spawn_universe_vfx(sel_u, MapVfxKind::HexagonOrder, 1.3);
                runtime.spawn_universe_toast(sel_u, "+İlahi Mucize", COL_ACCENT_TURQUOISE);
            }
            runtime.status_message = res.unwrap_or_else(|e| e);
        }
        UiAction::SeedLifeAndOrder => {
            runtime.inspected_gateway = None;
            let res = runtime.sim.execute_action(
                PLAYER_DEITY_ID,
                DivineAction::SeedLifeAndOrder { universe_id: sel_u },
            );
            if res.is_ok() {
                runtime.ui_resource_flash_timer = 0.65;
                runtime.spawn_universe_vfx(sel_u, MapVfxKind::HexagonOrder, 1.4);
                runtime.spawn_universe_toast(sel_u, "+Yaşam & Düzen", COL_ACCENT_TURQUOISE);
            }
            runtime.status_message = res.unwrap_or_else(|e| e);
        }
        UiAction::StabilizeReality => {
            runtime.inspected_gateway = None;
            let res = runtime.sim.execute_action(
                PLAYER_DEITY_ID,
                DivineAction::StabilizeReality { universe_id: sel_u },
            );
            if res.is_ok() {
                runtime.ui_resource_flash_timer = 0.55;
                runtime.spawn_universe_vfx(sel_u, MapVfxKind::HexagonOrder, 1.35);
                runtime.spawn_universe_toast(sel_u, "+Gerçeklik Onarıldı", COL_ACCENT_TURQUOISE);
            }
            runtime.status_message = res.unwrap_or_else(|e| e);
        }
        UiAction::ExtractCosmicEssence => {
            runtime.inspected_gateway = None;
            let res = runtime.sim.execute_action(
                PLAYER_DEITY_ID,
                DivineAction::ExtractCosmicEssence { universe_id: sel_u },
            );
            if res.is_ok() {
                runtime.ui_resource_flash_timer = 0.65;
                runtime.spawn_universe_vfx(sel_u, MapVfxKind::SoulHarvestSwirl, 1.3);
                runtime.spawn_universe_toast(sel_u, "+Kozmik Öz Damıtıldı", COL_ACCENT_PURPLE);
            }
            runtime.status_message = res.unwrap_or_else(|e| e);
        }
        UiAction::EnlightenCivilizations => {
            runtime.inspected_gateway = None;
            let res = runtime.sim.execute_action(
                PLAYER_DEITY_ID,
                DivineAction::EnlightenCivilizations { universe_id: sel_u },
            );
            if res.is_ok() {
                runtime.ui_resource_flash_timer = 0.65;
                runtime.spawn_universe_vfx(sel_u, MapVfxKind::ProphecyRings, 1.35);
                runtime.spawn_universe_toast(sel_u, "☀ Uygarlık Aydınlatıldı!", COL_ACCENT_GOLD);
            }
            runtime.status_message = res.unwrap_or_else(|e| e);
        }
        UiAction::ReshapeUniverseDomain => {
            runtime.inspected_gateway = None;
            let res = runtime.sim.execute_action(
                PLAYER_DEITY_ID,
                DivineAction::ReshapeUniverseDomain { universe_id: sel_u },
            );
            if res.is_ok() {
                runtime.ui_resource_flash_timer = 0.7;
                runtime.spawn_universe_vfx(sel_u, MapVfxKind::HexagonOrder, 1.45);
                runtime.spawn_universe_toast(
                    sel_u,
                    "☸ Evren Dokusu Dönüştürüldü!",
                    COL_ACCENT_TURQUOISE,
                );
            }
            runtime.status_message = res.unwrap_or_else(|e| e);
        }
        UiAction::ChannelAmbientEnergy => {
            runtime.inspected_gateway = None;
            let res = runtime.sim.execute_action(
                PLAYER_DEITY_ID,
                DivineAction::ChannelAmbientEnergy { universe_id: sel_u },
            );
            if res.is_ok() {
                runtime.ui_resource_flash_timer = 0.65;
                runtime.spawn_universe_vfx(sel_u, MapVfxKind::SoulHarvestSwirl, 1.35);
                let targets: Vec<Vec2> = runtime.sim.universes[sel_u as usize]
                    .gateways
                    .iter()
                    .filter(|g| matches!(g.state, GatewayState::Open))
                    .filter_map(|g| runtime.sim.universes.get(g.target_universe_id as usize))
                    .map(|u| Vec2::new(u.position[0], u.position[1]))
                    .collect();
                for to_pos in targets {
                    runtime.spawn_universe_vfx(sel_u, MapVfxKind::GatewayBeam { to_pos }, 1.2);
                }
                runtime.spawn_universe_toast(
                    sel_u,
                    "⚡ Kozmik Rezonans (+Ruh & Akış)",
                    COL_ACCENT_PURPLE,
                );
            }
            runtime.status_message = res.unwrap_or_else(|e| e);
        }
        UiAction::SmiteRival => {
            runtime.inspected_gateway = None;
            let target_deity = runtime.sim.universes[sel_u as usize]
                .dominant_deity()
                .map(|(id, _)| id)
                .filter(|&id| id != PLAYER_DEITY_ID)
                .unwrap_or(sel_rival);
            let res = runtime.sim.execute_action(
                PLAYER_DEITY_ID,
                DivineAction::SmiteRivalInfluence {
                    universe_id: sel_u,
                    target_deity_id: target_deity,
                },
            );
            if res.is_ok() {
                runtime.ui_resource_flash_timer = 0.5;
                runtime.spawn_universe_vfx(sel_u, MapVfxKind::DivineStrike, 1.2);
                runtime.spawn_universe_toast(sel_u, "✖ İlahi Darbe!", COL_ACCENT_CRIMSON);
            }
            runtime.status_message = res.unwrap_or_else(|e| e);
        }
        UiAction::InciteRebellion => {
            runtime.inspected_gateway = None;
            let res = runtime.sim.execute_action(
                PLAYER_DEITY_ID,
                DivineAction::InciteRebellion { universe_id: sel_u },
            );
            if res.is_ok() {
                runtime.ui_resource_flash_timer = 0.5;
                runtime.spawn_universe_vfx(sel_u, MapVfxKind::DivineStrike, 1.2);
                runtime.spawn_universe_toast(sel_u, "✖ İsyan Başlatıldı!", COL_ACCENT_CRIMSON);
            }
            runtime.status_message = res.unwrap_or_else(|e| e);
        }
        UiAction::ScoutRegion => {
            let res = runtime.sim.execute_action(
                PLAYER_DEITY_ID,
                DivineAction::ScoutRegion { universe_id: sel_u },
            );
            if res.is_ok() {
                runtime.ui_resource_flash_timer = 0.5;
                let targets: Vec<Vec2> = runtime.sim.universes[sel_u as usize]
                    .gateways
                    .iter()
                    .filter_map(|g| runtime.sim.universes.get(g.target_universe_id as usize))
                    .map(|u| Vec2::new(u.position[0], u.position[1]))
                    .collect();
                for to_pos in targets {
                    runtime.spawn_universe_vfx(sel_u, MapVfxKind::GatewayBeam { to_pos }, 1.25);
                }
                runtime.spawn_universe_toast(sel_u, "+Kozmik Keşif", COL_ACCENT_TURQUOISE);
            }
            runtime.status_message = res.unwrap_or_else(|e| e);
        }
        UiAction::OpenConnectedGateways => {
            let (from_u, target_opt) = if let Some((a, b)) = runtime.inspected_gateway {
                (a, Some(b))
            } else {
                let t = runtime.sim.universes[sel_u as usize]
                    .gateways
                    .iter()
                    .find(|g| !matches!(g.state, GatewayState::Open))
                    .or_else(|| runtime.sim.universes[sel_u as usize].gateways.first())
                    .map(|g| g.target_universe_id);
                (sel_u, t)
            };
            if let Some(to_id) = target_opt {
                let res = runtime.sim.execute_action(
                    PLAYER_DEITY_ID,
                    DivineAction::OpenGateway {
                        from_universe_id: from_u,
                        to_universe_id: to_id,
                    },
                );
                if res.is_ok() {
                    let to_pos = Vec2::new(
                        runtime.sim.universes[to_id as usize].position[0],
                        runtime.sim.universes[to_id as usize].position[1],
                    );
                    runtime.spawn_universe_vfx(from_u, MapVfxKind::GatewayBeam { to_pos }, 1.25);
                    runtime.spawn_universe_toast(from_u, "⇄ Geçit Açıldı", COL_ACCENT_TURQUOISE);
                }
                runtime.status_message = res.unwrap_or_else(|e| e);
            } else {
                runtime.status_message = "Bu evrende açılacak geçit bulunamadı.".to_string();
            }
        }
        UiAction::CloseConnectedGateways => {
            let (from_u, target_opt) = if let Some((a, b)) = runtime.inspected_gateway {
                (a, Some(b))
            } else {
                let t = runtime.sim.universes[sel_u as usize]
                    .gateways
                    .iter()
                    .find(|g| matches!(g.state, GatewayState::Open))
                    .map(|g| g.target_universe_id);
                (sel_u, t)
            };
            if let Some(to_id) = target_opt {
                let res = runtime.sim.execute_action(
                    PLAYER_DEITY_ID,
                    DivineAction::CloseGateway {
                        from_universe_id: from_u,
                        to_universe_id: to_id,
                    },
                );
                if res.is_ok() {
                    let to_pos = Vec2::new(
                        runtime.sim.universes[to_id as usize].position[0],
                        runtime.sim.universes[to_id as usize].position[1],
                    );
                    runtime.spawn_universe_vfx(from_u, MapVfxKind::GatewayBeam { to_pos }, 1.0);
                    runtime.spawn_universe_toast(from_u, "✖ Geçit Kapatıldı", COL_ACCENT_GOLD);
                }
                runtime.status_message = res.unwrap_or_else(|e| e);
            } else {
                runtime.status_message = "Bu evrende açık geçit bulunmuyor.".to_string();
            }
        }
        UiAction::FocusSelectedUniverse => {
            runtime.focus_selected_universe();
        }
        UiAction::FocusLatestEventUniverse => {
            runtime.focus_latest_event_universe();
        }
        UiAction::ResolveDilemmaChoice(choose_first) => {
            let dilemma_u = runtime
                .sim
                .active_dilemma
                .as_ref()
                .map(|d| d.universe_id)
                .unwrap_or(sel_u);
            let res = runtime.sim.execute_action(
                PLAYER_DEITY_ID,
                DivineAction::ResolveDilemma { choose_first },
            );
            if res.is_ok() {
                runtime.ui_resource_flash_timer = 0.65;
                runtime.ui_banner_flash_timer = 0.75;
                runtime.spawn_universe_vfx(dilemma_u, MapVfxKind::ProphecyRings, 1.3);
                runtime.spawn_universe_toast(dilemma_u, "★ İkilem Kararı Verildi", COL_ACCENT_GOLD);
            }
            runtime.status_message = res.unwrap_or_else(|e| e);
        }
        UiAction::UnlockNextInTree(tree) => {
            let next_power = PowerId::ALL
                .into_iter()
                .find(|p| p.tree() == tree && !runtime.sim.player_deity().has_power(*p));
            if let Some(power) = next_power {
                let res = runtime
                    .sim
                    .execute_action(PLAYER_DEITY_ID, DivineAction::UnlockPower { power });
                if res.is_ok() {
                    runtime.ui_resource_flash_timer = 0.7;
                    runtime.spawn_universe_vfx(sel_u, MapVfxKind::ProphecyRings, 1.35);
                    runtime.spawn_universe_toast(
                        sel_u,
                        format!("★ Güç Açıldı: {}", power.name_tr()),
                        COL_ACCENT_GOLD,
                    );
                }
                runtime.status_message = res.unwrap_or_else(|e| e);
            } else {
                runtime.status_message =
                    format!("{} ağacındaki tüm güçler zaten açıldı!", tree.name_tr());
            }
        }
        UiAction::PrevRivalDeity => {
            let max_rival = (runtime.sim.deities.len().saturating_sub(1)).max(1) as u32;
            runtime.selected_rival_deity_id = if sel_rival <= 1 {
                max_rival
            } else {
                sel_rival - 1
            };
        }
        UiAction::NextRivalDeity => {
            let max_rival = (runtime.sim.deities.len().saturating_sub(1)).max(1) as u32;
            runtime.selected_rival_deity_id = if sel_rival >= max_rival {
                1
            } else {
                sel_rival + 1
            };
        }
        UiAction::SelectRivalDeity(r_id) => {
            let max_rival = (runtime.sim.deities.len().saturating_sub(1)).max(1) as u32;
            runtime.selected_rival_deity_id = r_id.clamp(1, max_rival);
            if let Some(r_deity) = runtime
                .sim
                .deities
                .get(runtime.selected_rival_deity_id as usize)
            {
                runtime.status_message =
                    format!("Rakip ilah seçildi: {} ({})", r_deity.name, r_deity.title);
            }
        }
        UiAction::FocusRivalUniverse => {
            runtime.focus_rival_deity_universe();
        }
        UiAction::SendDivineGift => {
            let res = runtime.sim.execute_action(
                PLAYER_DEITY_ID,
                DivineAction::SendDivineGift {
                    target_deity_id: sel_rival,
                },
            );
            if res.is_ok() {
                runtime.ui_resource_flash_timer = 0.55;
                runtime.spawn_universe_vfx(sel_u, MapVfxKind::HexagonOrder, 1.15);
                runtime.spawn_universe_toast(sel_u, "✦ Ruh Armağanı (+28 İlişki)", COL_ACCENT_GOLD);
            }
            runtime.status_message = res.unwrap_or_else(|e| e);
        }
        UiAction::OfferPact => {
            let res = runtime.sim.execute_action(
                PLAYER_DEITY_ID,
                DivineAction::OfferDiplomaticPact {
                    target_deity_id: sel_rival,
                },
            );
            if res.is_ok() {
                runtime.ui_resource_flash_timer = 0.5;
                runtime.spawn_universe_vfx(sel_u, MapVfxKind::HexagonOrder, 1.15);
            }
            runtime.status_message = res.unwrap_or_else(|e| e);
        }
        UiAction::DemandTribute => {
            let res = runtime.sim.execute_action(
                PLAYER_DEITY_ID,
                DivineAction::DemandTribute {
                    target_deity_id: sel_rival,
                },
            );
            if res.is_ok() {
                runtime.ui_resource_flash_timer = 0.65;
                runtime.spawn_universe_vfx(sel_u, MapVfxKind::SoulHarvestSwirl, 1.2);
            }
            runtime.status_message = res.unwrap_or_else(|e| e);
        }
        UiAction::DeclareHolyWar => {
            let res = runtime.sim.execute_action(
                PLAYER_DEITY_ID,
                DivineAction::DeclareHolyWar {
                    target_deity_id: sel_rival,
                },
            );
            if res.is_ok() {
                runtime.ui_resource_flash_timer = 0.65;
                runtime.spawn_universe_vfx(sel_u, MapVfxKind::DivineStrike, 1.3);
                runtime.spawn_universe_toast(
                    sel_u,
                    "⚔ Kutsal Rekabet İlan Edildi!",
                    COL_ACCENT_CRIMSON,
                );
            }
            runtime.status_message = res.unwrap_or_else(|e| e);
        }
        UiAction::DisruptRivalRitual => {
            let res = runtime.sim.execute_action(
                PLAYER_DEITY_ID,
                DivineAction::DisruptRivalAscension {
                    target_deity_id: sel_rival,
                },
            );
            if res.is_ok() {
                runtime.ui_resource_flash_timer = 0.75;
                runtime.spawn_universe_vfx(sel_u, MapVfxKind::DivineStrike, 1.35);
                runtime.spawn_universe_toast(
                    sel_u,
                    "⚡ Rakip Ritüeli Sabote Edildi!",
                    COL_ACCENT_CRIMSON,
                );
            }
            runtime.status_message = res.unwrap_or_else(|e| e);
        }
        UiAction::SealRival => {
            let res = runtime.sim.execute_action(
                PLAYER_DEITY_ID,
                DivineAction::SealWeakenedDeity {
                    target_deity_id: sel_rival,
                },
            );
            if res.is_ok() {
                runtime.ui_resource_flash_timer = 0.85;
                runtime.spawn_universe_vfx(sel_u, MapVfxKind::DivineStrike, 1.5);
                runtime.spawn_universe_toast(sel_u, "★ Rakip Tanrı Mühürlendi!", COL_ACCENT_GOLD);
            }
            runtime.status_message = res.unwrap_or_else(|e| e);
        }
        UiAction::AdvanceAscension => {
            let res = runtime
                .sim
                .execute_action(PLAYER_DEITY_ID, DivineAction::AdvanceAscensionRitual);
            if res.is_ok() {
                runtime.ui_resource_flash_timer = 0.8;
                runtime.spawn_universe_vfx(sel_u, MapVfxKind::ProphecyRings, 1.5);
                runtime.spawn_universe_toast(sel_u, "★ Aşkınlık Ritüeli +%10", COL_ACCENT_GOLD);
            }
            runtime.status_message = res.unwrap_or_else(|e| e);
        }
        UiAction::ContinueInSandbox => {
            runtime.sim.continue_in_sandbox_mode();
            runtime.endgame_modal_dismissed = true;
            runtime.speed = SimSpeed::Normal1x;
            runtime.status_message =
                "Sonsuz Havuz (Sandbox) moduna geçildi; kozmik tarihin akmaya devam ediyor!"
                    .to_string();
        }
        UiAction::DismissEndgameModal => {
            runtime.endgame_modal_dismissed = true;
        }
        UiAction::PrevGuideChapter => {
            let total = built_in_guide_chapters().len().max(1);
            runtime.guide_chapter_index = if runtime.guide_chapter_index == 0 {
                total - 1
            } else {
                runtime.guide_chapter_index - 1
            };
        }
        UiAction::NextGuideChapter => {
            let total = built_in_guide_chapters().len().max(1);
            runtime.guide_chapter_index = (runtime.guide_chapter_index + 1) % total;
        }
        UiAction::SelectGuideChapter(idx) => {
            let total = built_in_guide_chapters().len().max(1);
            runtime.guide_chapter_index = idx.min(total - 1);
        }
        UiAction::StartScenario(idx) => {
            runtime.start_scenario(idx);
        }
        UiAction::ToggleHighContrastSymbols => {
            runtime.high_contrast_symbols = !runtime.high_contrast_symbols;
            runtime.status_message = format!(
                "Yüksek kontrastlı tehlike sembolleri: {}",
                if runtime.high_contrast_symbols {
                    "AÇIK"
                } else {
                    "KAPALI"
                }
            );
        }
    }

    if prev_outcome_ongoing && !matches!(runtime.sim.outcome, GameOutcome::Ongoing) {
        runtime.endgame_modal_dismissed = false;
        if matches!(runtime.sim.outcome, GameOutcome::PlayerVictory { .. }) {
            runtime.play_sfx(CosmicSfxKind::VictoryFanfare);
        } else {
            runtime.play_sfx(CosmicSfxKind::DivineStrike);
        }
    }

    // Trigger procedural synthesizer feedback tailored to the action category
    let sfx_kind = match action {
        UiAction::ToggleAudio | UiAction::CycleMapOverlayMode => None,
        UiAction::SpreadProphecy
        | UiAction::EnlightenCivilizations
        | UiAction::SendDivineGift
        | UiAction::OfferPact => Some(CosmicSfxKind::ProphecyChime),
        UiAction::HarvestSouls | UiAction::ExtractCosmicEssence | UiAction::DemandTribute => {
            Some(CosmicSfxKind::SoulHarvest)
        }
        UiAction::BestowMiracle
        | UiAction::SeedLifeAndOrder
        | UiAction::StabilizeReality
        | UiAction::ReshapeUniverseDomain
        | UiAction::UnlockNextInTree(_)
        | UiAction::AdvanceAscension => Some(CosmicSfxKind::MiracleChord),
        UiAction::SmiteRival
        | UiAction::InciteRebellion
        | UiAction::DeclareHolyWar
        | UiAction::DisruptRivalRitual
        | UiAction::SealRival => Some(CosmicSfxKind::DivineStrike),
        UiAction::ScoutRegion
        | UiAction::OpenConnectedGateways
        | UiAction::CloseConnectedGateways
        | UiAction::ChannelAmbientEnergy => Some(CosmicSfxKind::GatewayPulse),
        UiAction::SelectFilteredSlot(_)
        | UiAction::FocusSelectedUniverse
        | UiAction::FocusLatestEventUniverse
        | UiAction::FocusRivalUniverse => Some(CosmicSfxKind::UniverseSelect),
        UiAction::ResolveDilemmaChoice(_) | UiAction::ExecuteOrShowTutorialStep => {
            Some(CosmicSfxKind::CosmicEventAlert)
        }
        _ => Some(CosmicSfxKind::UiClick),
    };
    if let Some(kind) = sfx_kind {
        runtime.play_sfx(kind);
    }

    // Celebrate tutorial step advancement with banner flash + floating map toast
    if runtime.sim.tutorial_step != prev_tutorial {
        runtime.ui_banner_flash_timer = 1.1;
        runtime.ui_resource_flash_timer = 0.85;
        runtime.play_sfx(CosmicSfxKind::CosmicEventAlert);
        runtime.spawn_universe_toast(
            sel_u,
            "★ Eğitici Görev Tamamlandı! (+Ödül)",
            COL_ACCENT_GOLD,
        );
    }
}

fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    let ca = a.to_srgba();
    let cb = b.to_srgba();
    let k = t.clamp(0.0, 1.0);
    Color::srgba(
        ca.red + (cb.red - ca.red) * k,
        ca.green + (cb.green - ca.green) * k,
        ca.blue + (cb.blue - ca.blue) * k,
        ca.alpha + (cb.alpha - ca.alpha) * k,
    )
}

fn set_text_if_changed(text: &mut Text, new_val: String) {
    if **text != new_val {
        **text = new_val;
    }
}

fn color_differs(a: Color, b: Color) -> bool {
    let ca = a.to_srgba();
    let cb = b.to_srgba();
    (ca.red - cb.red).abs() > 0.004
        || (ca.green - cb.green).abs() > 0.004
        || (ca.blue - cb.blue).abs() > 0.004
        || (ca.alpha - cb.alpha).abs() > 0.004
}

fn is_action_currently_selected(action: &UiAction, runtime: &GameRuntimeState) -> bool {
    match action {
        UiAction::SetSpeed(spd) => runtime.speed == *spd,
        UiAction::SelectTab(tab) => runtime.active_tab == *tab,
        UiAction::SetOwnershipFilter(of) => runtime.ownership_filter == *of,
        UiAction::SelectGuideChapter(idx) => runtime.guide_chapter_index == *idx,
        UiAction::SelectRivalDeity(r_id) => runtime.selected_rival_deity_id == *r_id,
        UiAction::StartScenario(idx) => runtime.active_scenario_index == *idx,
        UiAction::ToggleMainMenuSettings => runtime.main_menu_settings_open,
        UiAction::ToggleAudio => runtime.audio_enabled,
        UiAction::FocusSelectedUniverse => {
            runtime.sim.focused_universe_id == Some(runtime.selected_universe_id)
        }
        UiAction::ResolveDilemmaChoice(_) => runtime.sim.active_dilemma.is_some(),
        _ => false,
    }
}

/// Returns true if this UI button represents a Sovereign universe action AND the player
/// does NOT currently dominate the selected universe (`!u.is_dominated_by(PLAYER_DEITY_ID)`),
/// or if it is a Dilemma Choice button while no active dilemma is present.
fn is_ui_action_sovereign_locked(action: &UiAction, runtime: &GameRuntimeState) -> bool {
    if matches!(action, UiAction::ResolveDilemmaChoice(_)) {
        return runtime.sim.active_dilemma.is_none();
    }
    let is_sovereign_btn = matches!(
        action,
        UiAction::HarvestSouls
            | UiAction::BestowMiracle
            | UiAction::SeedLifeAndOrder
            | UiAction::StabilizeReality
            | UiAction::ExtractCosmicEssence
            | UiAction::EnlightenCivilizations
            | UiAction::ReshapeUniverseDomain
            | UiAction::ChannelAmbientEnergy
            | UiAction::ScoutRegion
            | UiAction::CloseConnectedGateways
            | UiAction::FocusSelectedUniverse
    );
    if !is_sovereign_btn {
        return false;
    }
    let sel_u = runtime.selected_universe_id as usize;
    !runtime
        .sim
        .universes
        .get(sel_u)
        .is_some_and(|u| u.discovered_by_player && u.is_dominated_by(PLAYER_DEITY_ID))
}

fn ui_action_to_divine_action(
    action: &UiAction,
    runtime: &GameRuntimeState,
) -> Option<DivineAction> {
    let player = runtime.sim.player_deity();
    let sel_u = runtime.selected_universe_id;
    let sel_rival = runtime.selected_rival_deity_id;
    match action {
        UiAction::SpreadProphecy => Some(DivineAction::SpreadProphecy { universe_id: sel_u }),
        UiAction::HarvestSouls => Some(DivineAction::HarvestSouls { universe_id: sel_u }),
        UiAction::BestowMiracle => Some(DivineAction::BestowMiracle { universe_id: sel_u }),
        UiAction::SeedLifeAndOrder => Some(DivineAction::SeedLifeAndOrder { universe_id: sel_u }),
        UiAction::StabilizeReality => Some(DivineAction::StabilizeReality { universe_id: sel_u }),
        UiAction::ExtractCosmicEssence => {
            Some(DivineAction::ExtractCosmicEssence { universe_id: sel_u })
        }
        UiAction::EnlightenCivilizations => {
            Some(DivineAction::EnlightenCivilizations { universe_id: sel_u })
        }
        UiAction::ReshapeUniverseDomain => {
            Some(DivineAction::ReshapeUniverseDomain { universe_id: sel_u })
        }
        UiAction::ChannelAmbientEnergy => {
            Some(DivineAction::ChannelAmbientEnergy { universe_id: sel_u })
        }
        UiAction::SmiteRival => Some(DivineAction::SmiteRivalInfluence {
            universe_id: sel_u,
            target_deity_id: sel_rival,
        }),
        UiAction::InciteRebellion => Some(DivineAction::InciteRebellion { universe_id: sel_u }),
        UiAction::ScoutRegion => Some(DivineAction::ScoutRegion { universe_id: sel_u }),
        UiAction::OpenConnectedGateways => Some(DivineAction::OpenGateway {
            from_universe_id: sel_u,
            to_universe_id: 0,
        }),
        UiAction::CloseConnectedGateways => Some(DivineAction::CloseGateway {
            from_universe_id: sel_u,
            to_universe_id: 0,
        }),
        UiAction::SendDivineGift => Some(DivineAction::SendDivineGift {
            target_deity_id: sel_rival,
        }),
        UiAction::OfferPact => Some(DivineAction::OfferDiplomaticPact {
            target_deity_id: sel_rival,
        }),
        UiAction::DemandTribute => Some(DivineAction::DemandTribute {
            target_deity_id: sel_rival,
        }),
        UiAction::DeclareHolyWar => Some(DivineAction::DeclareHolyWar {
            target_deity_id: sel_rival,
        }),
        UiAction::DisruptRivalRitual => Some(DivineAction::DisruptRivalAscension {
            target_deity_id: sel_rival,
        }),
        UiAction::SealRival => Some(DivineAction::SealWeakenedDeity {
            target_deity_id: sel_rival,
        }),
        UiAction::AdvanceAscension => Some(DivineAction::AdvanceAscensionRitual),
        UiAction::UnlockNextInTree(tree) => PowerId::ALL
            .into_iter()
            .find(|p| p.tree() == *tree && !player.has_power(*p))
            .map(|power| DivineAction::UnlockPower { power }),
        _ => None,
    }
}

fn can_afford_ui_action(action: &UiAction, runtime: &GameRuntimeState) -> bool {
    let player = runtime.sim.player_deity();
    if let Some(act) = ui_action_to_divine_action(action, runtime) {
        player.resources.can_afford(&act.base_cost(player))
    } else {
        true
    }
}

#[derive(Default, PartialEq, Eq, Clone, Copy)]
struct FilteredPoolCacheKey {
    initialized: bool,
    tick: u64,
    ledger_len: usize,
    next_event_id: u64,
    type_filter: Option<UniverseType>,
    ownership_filter: OwnershipFilter,
    sort_mode: UniverseSortMode,
}

#[derive(Default)]
struct FilteredPoolCache {
    key: FilteredPoolCacheKey,
    ids: Vec<u32>,
}

impl FilteredPoolCache {
    fn current_key(runtime: &GameRuntimeState) -> FilteredPoolCacheKey {
        FilteredPoolCacheKey {
            initialized: true,
            tick: runtime.sim.tick,
            ledger_len: runtime.sim.ledger.recent_entries.len(),
            next_event_id: runtime.sim.next_event_id,
            type_filter: runtime.type_filter,
            ownership_filter: runtime.ownership_filter,
            sort_mode: runtime.sort_mode,
        }
    }

    fn get<'a>(&'a mut self, runtime: &GameRuntimeState) -> &'a [u32] {
        let cur_key = Self::current_key(runtime);
        if self.key != cur_key {
            self.key = cur_key;
            self.ids = runtime.filtered_sorted_universes();
        }
        &self.ids
    }
}

#[derive(Default, PartialEq, Eq, Clone, Copy)]
struct RightInspectorCacheKey {
    initialized: bool,
    tick: u64,
    ledger_len: usize,
    next_event_id: u64,
    selected_universe_id: u32,
    inspected_gateway: Option<(u32, u32)>,
    selected_rival_deity_id: u32,
    active_tab: InspectorTab,
    guide_chapter_index: usize,
    tutorial_step: TutorialStep,
    tutorial_collapsed: bool,
    active_scenario_index: usize,
    focused_universe_id: Option<u32>,
    ui_theme_index: usize,
}

#[derive(Default, PartialEq, Eq, Clone)]
struct BottomLogCacheKey {
    initialized: bool,
    tick: u64,
    next_event_id: u64,
    status_message: String,
}

type ButtonVisualQuery<'a> = (
    &'a Hovered,
    Has<Pressed>,
    Option<&'a UiAction>,
    Option<&'a LeftQuickSlotButton>,
    Option<&'a AnimatedButtonSpec>,
    &'a mut Node,
    &'a mut BackgroundColor,
    &'a mut BorderColor,
);

fn update_button_visuals(
    time: Res<Time>,
    runtime: Res<GameRuntimeState>,
    mut pool_cache: Local<FilteredPoolCache>,
    mut q_buttons: Query<ButtonVisualQuery, With<Button>>,
) {
    let dt = time.delta_secs();
    let blend = (dt * 14.0).min(1.0);
    let ui_pulse = (runtime.ui_anim_time * 3.0).sin() * 0.5 + 0.5;
    let (theme_normal_bg, theme_active_bg, theme_normal_border) =
        theme_btn_palette(runtime.ui_theme_index);
    let filtered = pool_cache.get(&runtime);

    for (hovered, pressed, action_opt, slot_opt, anim_spec_opt, mut node, mut bg, mut border) in
        &mut q_buttons
    {
        let is_active_toggle =
            action_opt.is_some_and(|act| is_action_currently_selected(act, &runtime));
        let is_sovereign_locked =
            action_opt.is_some_and(|act| is_ui_action_sovereign_locked(act, &runtime));
        let is_affordable = action_opt.is_none_or(|act| can_afford_ui_action(act, &runtime));

        // Only Main Menu hero buttons (`AnimatedButtonSpec`) animate their width on hover/press;
        // regular UI buttons never mutate `Node::width` so layout never shifts or jitters!
        if let Some(spec) = anim_spec_opt {
            let target_w = if pressed {
                (spec.base_width - 2.0).max(20.0)
            } else if hovered.get() {
                spec.base_width + spec.hover_expand_px
            } else {
                spec.base_width
            };
            let cur_w = match node.width {
                Val::Px(v) => v,
                _ => spec.base_width,
            };
            let next_w = cur_w + (target_w - cur_w) * blend;
            if (next_w - cur_w).abs() > 0.15 {
                node.width = px(next_w);
            } else if (cur_w - target_w).abs() > 0.01 {
                node.width = px(target_w);
            }
        }

        // Check if this button is a universe quick-slot button
        let slot_highlight = slot_opt.and_then(|s| {
            let abs_idx = runtime.pool_page * POOL_SLOTS_PER_PAGE + s.0;
            filtered.get(abs_idx).copied()
        });

        let is_hero = anim_spec_opt.is_some();

        let (target_bg, target_border) = if is_sovereign_locked {
            (
                COL_BTN_LOCKED,
                COL_ACCENT_CRIMSON.with_alpha(if hovered.get() { 0.55 } else { 0.25 }),
            )
        } else if pressed {
            (COL_BTN_PRESSED, COL_ACCENT_GOLD)
        } else if hovered.get() {
            (
                COL_BTN_HOVER,
                if is_hero {
                    lerp_color(COL_ACCENT_TURQUOISE, COL_ACCENT_GOLD, ui_pulse * 0.65)
                } else {
                    COL_ACCENT_TURQUOISE
                },
            )
        } else if is_hero {
            (
                COL_BTN_ACTIVE,
                lerp_color(COL_ACCENT_GOLD, COL_ACCENT_PURPLE, ui_pulse * 0.45),
            )
        } else if let Some(u_id) = slot_highlight {
            let u = &runtime.sim.universes[u_id as usize];
            let is_sel = u_id == runtime.selected_universe_id;
            let border_col = if is_sel {
                COL_ACCENT_GOLD
            } else if u.discovered_by_player && u.has_full_allegiance(PLAYER_DEITY_ID) {
                COL_ACCENT_GOLD.with_alpha(0.75)
            } else if u.discovered_by_player && u.is_dominated_by(PLAYER_DEITY_ID) {
                COL_ACCENT_PURPLE.with_alpha(0.70)
            } else if u.discovered_by_player && (u.danger_level >= 65.0 || u.stability <= 30.0) {
                COL_ACCENT_CRIMSON.with_alpha(0.75)
            } else {
                COL_ACCENT_TURQUOISE.with_alpha(0.30)
            };
            (
                if is_sel {
                    theme_active_bg
                } else {
                    theme_normal_bg
                },
                border_col,
            )
        } else if is_active_toggle {
            (theme_active_bg, COL_ACCENT_GOLD.with_alpha(0.92))
        } else if !is_affordable {
            (
                Color::srgb_u8(0x14, 0x10, 0x22),
                COL_TEXT_SECONDARY.with_alpha(0.22),
            )
        } else {
            (theme_normal_bg, theme_normal_border)
        };

        let next_bg = lerp_color(bg.0, target_bg, blend);
        if color_differs(bg.0, next_bg) {
            bg.0 = next_bg;
        }
        let cur_border = border.top;
        let next_border = lerp_color(cur_border, target_border, blend);
        if color_differs(cur_border, next_border) {
            border.set_all(next_border);
        }
    }
}

fn refresh_top_bar_cards(
    time: Res<Time>,
    mut runtime: ResMut<GameRuntimeState>,
    mut q_cards: Query<(&TopResourceCardKind, &mut Text, &mut TextColor)>,
) {
    let dt = time.delta_secs();
    if runtime.ui_resource_flash_timer > 0.0 {
        runtime.ui_resource_flash_timer = (runtime.ui_resource_flash_timer - dt).max(0.0);
    }

    let flash_t = (runtime.ui_resource_flash_timer / 0.65).clamp(0.0, 1.0);
    let player = runtime.sim.player_deity();
    let res = player.resources;
    let inc = player.last_tick_income;
    let dom_count = runtime.sim.dominated_universe_count(PLAYER_DEITY_ID);
    let full_count = runtime.sim.full_allegiance_universe_count(PLAYER_DEITY_ID);

    for (kind, mut text, mut color) in &mut q_cards {
        let base_col = match kind {
            TopResourceCardKind::Souls if inc.souls < 0 => COL_ACCENT_CRIMSON,
            TopResourceCardKind::Souls if inc.souls > 0 => {
                lerp_color(COL_TEXT_PRIMARY, COL_ACCENT_TURQUOISE, 0.35)
            }
            TopResourceCardKind::Influence if inc.influence < 0 => COL_ACCENT_CRIMSON,
            TopResourceCardKind::Influence if inc.influence > 0 => {
                lerp_color(COL_TEXT_PRIMARY, COL_ACCENT_GOLD, 0.35)
            }
            TopResourceCardKind::Essence if inc.cosmic_essence < 0 => COL_ACCENT_CRIMSON,
            TopResourceCardKind::Essence if inc.cosmic_essence > 0 => {
                lerp_color(COL_TEXT_PRIMARY, COL_ACCENT_TURQUOISE, 0.40)
            }
            _ => COL_TEXT_PRIMARY,
        };
        let target_col = lerp_color(base_col, COL_ACCENT_GOLD, flash_t * 0.85);
        if color_differs(color.0, target_col) {
            color.0 = target_col;
        }
        let next_str = match kind {
            TopResourceCardKind::DeityIdentityAndTurn => {
                format!(
                    "♛ {} · Tur {} · {}",
                    player.name,
                    runtime.sim.tick,
                    runtime.speed.label_tr()
                )
            }
            TopResourceCardKind::Souls => {
                let arrow = if inc.souls > 0 { "▲" } else { "" };
                format!("◆ Ruh: {} ({}{:+}/tur)", res.souls, arrow, inc.souls)
            }
            TopResourceCardKind::Influence => {
                let arrow = if inc.influence > 0 { "▲" } else { "" };
                format!(
                    "♛ Nüfuz: {} ({}{:+}/tur)",
                    res.influence, arrow, inc.influence
                )
            }
            TopResourceCardKind::Essence => {
                let arrow = if inc.cosmic_essence > 0 { "▲" } else { "" };
                format!(
                    "✦ Öz: {} ({}{:+}/tur)",
                    res.cosmic_essence, arrow, inc.cosmic_essence
                )
            }
            TopResourceCardKind::DominionAndVictory => {
                format!(
                    "★ Tam: {}/{} · Baskın: {}/{} · Ritüel: %{:.0}",
                    full_count,
                    DOMINION_REQUIRED_FULL_ALLEGIANCE,
                    dom_count,
                    DOMINION_REQUIRED_DOMINATED,
                    player.ascension_progress
                )
            }
        };
        set_text_if_changed(&mut text, next_str);
    }
}

fn refresh_left_panel_text(
    runtime: Res<GameRuntimeState>,
    mut pool_cache: Local<FilteredPoolCache>,
    mut last_page: Local<Option<usize>>,
    mut last_key: Local<FilteredPoolCacheKey>,
    mut q_summary: Single<&mut Text, (With<LeftPoolSummaryText>, Without<LeftQuickSlotLabel>)>,
    mut q_slots: Query<(&LeftQuickSlotLabel, &mut Text), Without<LeftPoolSummaryText>>,
) {
    let cur_key = FilteredPoolCache::current_key(&runtime);
    if *last_key == cur_key && *last_page == Some(runtime.pool_page) {
        return;
    }
    *last_key = cur_key;
    *last_page = Some(runtime.pool_page);
    let filtered = pool_cache.get(&runtime);

    let discovered_total = runtime
        .sim
        .universes
        .iter()
        .filter(|u| u.discovered_by_player && !u.is_destroyed)
        .count();
    let full_allegiance_total = runtime.sim.full_allegiance_universe_count(PLAYER_DEITY_ID);
    let total_pages = (filtered.len().saturating_sub(1) / POOL_SLOTS_PER_PAGE) + 1;
    let cur_page = (runtime.pool_page + 1).min(total_pages);

    let type_name = runtime
        .type_filter
        .map(|t| t.short_tag_tr())
        .unwrap_or("Tümü");

    set_text_if_changed(
        &mut q_summary,
        format!(
            "Durum: {} · Tür: {} · Sıra: {}\nSayfa: {}/{} ({} Evren) · Keşif: {}/60 · ★ Tam: {}",
            runtime.ownership_filter.label_tr(),
            type_name,
            runtime.sort_mode.label_tr(),
            cur_page,
            total_pages,
            filtered.len(),
            discovered_total,
            full_allegiance_total,
        ),
    );

    for (slot_marker, mut slot_text) in &mut q_slots {
        let abs_idx = runtime.pool_page * POOL_SLOTS_PER_PAGE + slot_marker.0;
        let next_slot = if let Some(&u_id) = filtered.get(abs_idx) {
            let u = &runtime.sim.universes[u_id as usize];
            if u.discovered_by_player {
                let inf = u.influence_of(PLAYER_DEITY_ID);
                let badge = if u.has_full_allegiance(PLAYER_DEITY_ID) {
                    "★ TAM"
                } else if u.is_dominated_by(PLAYER_DEITY_ID) {
                    "◆ BİZİM"
                } else if u.dominant_deity().is_some() {
                    "⚔ RAKİP"
                } else {
                    "○ NÖTR"
                };
                format!("[{}] {} (%{:.0})", badge, u.name, inf)
            } else {
                format!("[? GİZEMLİ] Bölge #{}", u.id + 1)
            }
        } else {
            "- Boş Yuva -".to_string()
        };
        set_text_if_changed(&mut slot_text, next_slot);
    }
}

type BannerFilter = (
    With<RightTutorialBannerText>,
    Without<RightInspectorBodyText>,
    Without<InspectorBarLabel>,
    Without<UniverseSovereigntyBannerText>,
);

type BodyFilter = (
    With<RightInspectorBodyText>,
    Without<RightTutorialBannerText>,
    Without<InspectorBarLabel>,
    Without<UniverseSovereigntyBannerText>,
);

type SovereigntyBannerFilter = (
    With<UniverseSovereigntyBannerText>,
    Without<RightTutorialBannerText>,
    Without<RightInspectorBodyText>,
    Without<InspectorBarLabel>,
);

type BarLabelFilter = (
    Without<RightTutorialBannerText>,
    Without<RightInspectorBodyText>,
    Without<UniverseSovereigntyBannerText>,
);

type TabContainerFilter = (
    Without<DilemmaButtonRow>,
    Without<RightTutorialButtonsRow>,
    Without<InspectorBarFill>,
    Without<TutorialBannerContainer>,
);

type TutorialBoxFilter = (
    With<TutorialBannerContainer>,
    Without<TabContainer>,
    Without<DilemmaButtonRow>,
    Without<RightTutorialButtonsRow>,
    Without<InspectorBarFill>,
);

#[allow(clippy::too_many_arguments)]
fn refresh_right_inspector_text(
    time: Res<Time>,
    mut runtime: ResMut<GameRuntimeState>,
    mut last_key: Local<RightInspectorCacheKey>,
    mut q_banner: Single<(&mut Text, &mut TextColor), BannerFilter>,
    mut q_banner_box: Single<&mut Node, TutorialBoxFilter>,
    mut q_body: Single<&mut Text, BodyFilter>,
    mut q_sov_banner: Single<(&mut Text, &mut TextColor), SovereigntyBannerFilter>,
    mut q_bar_labels: Query<(&InspectorBarLabel, &mut Text), BarLabelFilter>,
    mut q_bar_fills: Query<(&InspectorBarFill, &mut Node, &mut BackgroundColor)>,
    mut q_tab_containers: Query<(&TabContainer, &mut Node), TabContainerFilter>,
) {
    let dt = time.delta_secs();
    if runtime.ui_banner_flash_timer > 0.0 {
        runtime.ui_banner_flash_timer = (runtime.ui_banner_flash_timer - dt).max(0.0);
    }

    let (ref mut banner_text, ref mut banner_color) = *q_banner;
    let banner_flash = (runtime.ui_banner_flash_timer / 0.85).clamp(0.0, 1.0);
    let target_banner_col = if banner_flash > 0.05 {
        lerp_color(COL_ACCENT_GOLD, COL_ACCENT_TURQUOISE, banner_flash)
    } else if runtime.sim.active_dilemma.is_some() {
        COL_ACCENT_TURQUOISE
    } else {
        COL_ACCENT_GOLD
    };
    if color_differs(banner_color.0, target_banner_col) {
        banner_color.0 = target_banner_col;
    }

    let cur_key = RightInspectorCacheKey {
        initialized: true,
        tick: runtime.sim.tick,
        ledger_len: runtime.sim.ledger.recent_entries.len(),
        next_event_id: runtime.sim.next_event_id,
        selected_universe_id: runtime.selected_universe_id,
        inspected_gateway: runtime.inspected_gateway,
        selected_rival_deity_id: runtime.selected_rival_deity_id,
        active_tab: runtime.active_tab,
        guide_chapter_index: runtime.guide_chapter_index,
        tutorial_step: runtime.sim.tutorial_step,
        tutorial_collapsed: runtime.tutorial_collapsed,
        active_scenario_index: runtime.active_scenario_index,
        focused_universe_id: runtime.sim.focused_universe_id,
        ui_theme_index: runtime.ui_theme_index,
    };

    if *last_key == cur_key {
        return;
    }
    *last_key = cur_key;

    for (tc, mut node) in &mut q_tab_containers {
        let target_disp = if tc.0 == runtime.active_tab {
            Display::Flex
        } else {
            Display::None
        };
        if node.display != target_disp {
            node.display = target_disp;
        }
    }

    let target_banner_h = if runtime.tutorial_collapsed {
        px(40)
    } else {
        px(60)
    };
    if q_banner_box.height != target_banner_h {
        q_banner_box.height = target_banner_h;
    }

    // Update top Tutorial & Active Dilemma Banner
    let tut = runtime.sim.tutorial_step;
    let reward = tut.reward();
    let reward_str = if tut == TutorialStep::Completed {
        String::new()
    } else {
        format!(
            " (+{}R, +{}N, +{}Ö)",
            reward.souls, reward.influence, reward.cosmic_essence
        )
    };

    let next_banner_str = if let Some(d) = &runtime.sim.active_dilemma {
        if runtime.tutorial_collapsed {
            format!("★ AKTİF İKİLEM: {}", d.title)
        } else {
            format!("★ AKTİF İKİLEM: {}\n{}", d.title, d.description)
        }
    } else if runtime.tutorial_collapsed {
        format!("EĞİTİCİ: {}", tut.title_tr())
    } else {
        format!(
            "EĞİTİCİ — {}{}\n{}",
            tut.title_tr(),
            reward_str,
            tut.instruction_tr()
        )
    };
    set_text_if_changed(banner_text, next_banner_str);

    // Update Visual Progress Bars (3 bars) + Sovereignty Banner
    let u_idx =
        (runtime.selected_universe_id as usize).min(runtime.sim.universes.len().saturating_sub(1));
    let u = &runtime.sim.universes[u_idx];
    let (ref mut sov_text, ref mut sov_color) = *q_sov_banner;
    if u.discovered_by_player && u.is_dominated_by(PLAYER_DEITY_ID) {
        set_text_if_changed(
            sov_text,
            "♛ HÂKİMİYET EMİRLERİ (✓ Söz Sahibisin — Tüm Güçler Açık)".to_string(),
        );
        if color_differs(sov_color.0, COL_ACCENT_GOLD) {
            sov_color.0 = COL_ACCENT_GOLD;
        }
    } else {
        set_text_if_changed(
            sov_text,
            "⊗ HÂKİMİYET EMİRLERİ (KİLİTLİ — Önce Baskın İlah Olmalısın!)".to_string(),
        );
        if color_differs(sov_color.0, COL_ACCENT_CRIMSON) {
            sov_color.0 = COL_ACCENT_CRIMSON;
        }
    }

    let (dom_ratio, trans_ratio, cre_ratio) = runtime.sim.victory_progress_ratios(PLAYER_DEITY_ID);

    let bar_specs: [(String, f32, Color); 3] =
        if runtime.active_tab == InspectorTab::Universe && runtime.inspected_gateway.is_none() {
            if u.discovered_by_player {
                let my_inf = u.influence_of(PLAYER_DEITY_ID);
                [
                    (
                        format!("Evren İstikrarı: %{:.0}", u.stability),
                        (u.stability / 100.0).clamp(0.0, 1.0),
                        if u.stability < 35.0 {
                            COL_ACCENT_CRIMSON
                        } else {
                            COL_ACCENT_TURQUOISE
                        },
                    ),
                    (
                        format!("Tehlike Seviyesi: %{:.0}", u.danger_level),
                        (u.danger_level / 100.0).clamp(0.0, 1.0),
                        COL_ACCENT_CRIMSON,
                    ),
                    (
                        format!("Aidiyetimiz: %{:.0} (Hedef: %65 ★ Tam Aidiyet)", my_inf),
                        (my_inf / 100.0).clamp(0.0, 1.0),
                        if u.has_full_allegiance(PLAYER_DEITY_ID) {
                            COL_ACCENT_GOLD
                        } else {
                            COL_ACCENT_PURPLE
                        },
                    ),
                ]
            } else {
                [
                    (
                        "İstikrar: ? (Keşfedilmedi)".to_string(),
                        0.0,
                        COL_TEXT_SECONDARY,
                    ),
                    (
                        "Tehlike: ? (Keşfedilmedi)".to_string(),
                        0.0,
                        COL_TEXT_SECONDARY,
                    ),
                    (
                        "Aidiyet: ? (Keşfedilmedi)".to_string(),
                        0.0,
                        COL_TEXT_SECONDARY,
                    ),
                ]
            }
        } else {
            [
                (
                    format!(
                        "1. Hâkimiyet Zaferi: %{:.0} (35 Baskın · 18 ★ Tam · 5 Mühür)",
                        dom_ratio * 100.0
                    ),
                    dom_ratio,
                    COL_ACCENT_GOLD,
                ),
                (
                    format!(
                        "2. Aşkınlık Zaferi: %{:.0} (%100 Ritüel · 10 Güç · 10 ★ Tam)",
                        trans_ratio * 100.0
                    ),
                    trans_ratio,
                    COL_ACCENT_PURPLE,
                ),
                (
                    format!(
                        "3. Yaratım Zaferi: %{:.0} (15 Düzenli Evren · 3/3 Yaratım)",
                        cre_ratio * 100.0
                    ),
                    cre_ratio,
                    COL_ACCENT_TURQUOISE,
                ),
            ]
        };

    for (lbl, mut text) in &mut q_bar_labels {
        if let Some((title, _, _)) = bar_specs.get(lbl.0) {
            set_text_if_changed(&mut text, title.clone());
        }
    }
    for (fill, mut node, mut bg) in &mut q_bar_fills {
        if let Some((_, ratio, col)) = bar_specs.get(fill.0) {
            let target_w = percent((ratio * 100.0).clamp(2.0, 100.0));
            if node.width != target_w {
                node.width = target_w;
            }
            if color_differs(bg.0, *col) {
                bg.0 = *col;
            }
        }
    }

    let next_body = match runtime.active_tab {
        InspectorTab::Universe => {
            // If the player clicked on an inter-universe gateway / animation stream, explain it!
            if let Some((u_a, u_b)) = runtime.inspected_gateway {
                let ua = &runtime.sim.universes[u_a as usize];
                let ub = &runtime.sim.universes[u_b as usize];
                let name_a = if ua.discovered_by_player {
                    ua.name.clone()
                } else {
                    format!("Gizemli Bölge #{}", ua.id + 1)
                };
                let name_b = if ub.discovered_by_player {
                    ub.name.clone()
                } else {
                    format!("Gizemli Bölge #{}", ub.id + 1)
                };
                let gw_state = ua
                    .gateways
                    .iter()
                    .find(|g| g.target_universe_id == u_b)
                    .map(|g| g.state)
                    .unwrap_or(GatewayState::Closed);

                let (state_title, anim_explanation) = match gw_state {
                    GatewayState::Open => (
                        "AÇIK KOZMİK GEÇİT (Aktif Akış)",
                        "• Çizgi Üzerinde Akan Altın ve Turkuaz Küreler Nedir?\n  İki evren arasındaki açık boyutsal geçitten akan İnanç Yayılımı, Ruh Göçü ve Kozmik Enerji akışını temsil eder.\n• Oyun Duraklatıldığında: Zaman durduğu için kürelerin akışı da tamamen durur.",
                    ),
                    GatewayState::Closed => (
                        "KAPALI KOZMİK GEÇİT (Akış Yok)",
                        "• Kesik Çizgi Nedir?\n  İki evren arasında doğal bir bağ olduğunu ancak geçidin kapalı olduğunu gösterir.\n• 'Geçit Aç' butonuna tıklayarak bağlantıyı aktif hale getirebilir ve hedef evreni keşfedebilirsin.",
                    ),
                    GatewayState::Locked { owner_deity } => (
                        "MÜHÜRLÜ GEÇİT (Elmas Mühür)",
                        if owner_deity == PLAYER_DEITY_ID {
                            "• Ortadaki Altın Elmas Mühür Nedir?\n  Bu geçidin senin tarafından mühürlendiğini ve rakip sızıntılarına kapatıldığını gösterir."
                        } else {
                            "• Ortadaki Altın Elmas Mühür Nedir?\n  Bu geçidin rakip bir ilah tarafından kilitlendiğini gösterir."
                        },
                    ),
                };

                // Calculate live cross-gateway faith flow explanation
                let mut flow_detail = String::new();
                if matches!(gw_state, GatewayState::Open) {
                    if let Some((dom_a, inf_a)) = ua.dominant_deity() {
                        if inf_a >= 50.0 {
                            let d_name = &runtime.sim.deities[dom_a as usize].name;
                            flow_detail.push_str(&format!(
                                "  • {name_a} -> {name_b}: Baskın ilah {d_name} (%{inf_a:.0}) her tur bu geçitten komşu evrene inanç yayıyor.\n"
                            ));
                        }
                    }
                    if let Some((dom_b, inf_b)) = ub.dominant_deity() {
                        if inf_b >= 50.0 {
                            let d_name = &runtime.sim.deities[dom_b as usize].name;
                            flow_detail.push_str(&format!(
                                "  • {name_b} -> {name_a}: Baskın ilah {d_name} (%{inf_b:.0}) her tur bu geçitten komşu evrene inanç yayıyor.\n"
                            ));
                        }
                    }
                    if flow_detail.is_empty() {
                        flow_detail.push_str(
                            "  • Henüz iki evrende de %50 üzeri baskın ilah olmadığı için pasif enerji dengesi var.\n",
                        );
                    }
                } else {
                    flow_detail.push_str(
                        "  • Geçit kapalı olduğu için evrenler arası inanç sızıntısı gerçekleşmiyor.\n",
                    );
                }

                format!(
                    "=== ⇄ EVRENLER ARASI GEÇİT VE ANİMASYON ===\n\
                    Bağlantı: {} ↔ {}\n\
                    Durum: {}\n\n\
                    --- Görsel ve Animasyon Açıklaması ---\n{}\n\n\
                    --- Canlı Etkileşim ve Akış ---\n{}\n\
                    İPUCU: Alttaki 'Geçit Aç' veya 'Geçit Kapat' butonlarıyla bu bağlantıyı yönetebilir veya 'Evren Görünümüne Dön' ile evren detaylarına dönebilirsin.",
                    name_a, name_b, state_title, anim_explanation, flow_detail
                )
            } else if !u.discovered_by_player {
                format!(
                    "=== ? KEŞFEDİLMEMİŞ KOZMİK BÖLGE #{} ===\n\n\
                    Katman: {}. Kozmik Halka\n\
                    Durum: Henüz keşfedilmedi.\n\n\
                    Bu gizemli evreni açmak için söz sahibi olduğun komşu bir evreni seçip \
                    'Keşif (20R 20N)' veya 'Geçit Aç (40R 20N 10Ö)' emrini verin.\n\n\
                    İPUCU: Haritada evrenler arasındaki çizgilere ve akan kürelere tıklayarak bağlantıların ne olduğunu da inceleyebilirsin!",
                    u.id + 1,
                    u.region_tier + 1
                )
            } else {
                let profile = universe_type_profile(u.universe_type);
                let my_inf = u.influence_of(PLAYER_DEITY_ID);
                let is_sovereign = u.is_dominated_by(PLAYER_DEITY_ID);
                let allegiance_status = if u.has_full_allegiance(PLAYER_DEITY_ID) {
                    format!("★ TAM AİDİYET (Mutlak Söz Sahibisin: %{my_inf:.0} · +%30 Getiri)")
                } else if is_sovereign {
                    format!(
                        "◆ BASKIN İLAH SENSİN (Söz Sahibisin: %{my_inf:.0} — %65'te ★ Tam Aidiyet)"
                    )
                } else if let Some((d_id, val)) = u.dominant_deity() {
                    let d_name = &runtime.sim.deities[d_id as usize].name;
                    format!("⊗ Rakip Hâkim: {d_name} (%{val:.0}) | Senin: %{my_inf:.0} (Söz Sahibi Değilsin)")
                } else {
                    format!(
                        "⊗ Sahipsiz / Nötr | Senin: %{my_inf:.0} (Söz Sahibi Olmak İçin Kehanet Yay!)"
                    )
                };

                let econ_line = if let Some((dom_id, bd)) =
                    runtime.sim.universe_economy_breakdown(u.id)
                {
                    let net = bd.net_income();
                    let loss = bd.total_loss();
                    let owner_tag = if dom_id == PLAYER_DEITY_ID {
                        "Sana Getirisi"
                    } else {
                        "Rakibe Getirisi"
                    };
                    if loss.souls > 0 || loss.influence > 0 {
                        format!(
                            "Tur Başı Ekonomi ({owner_tag}): {:+}R, {:+}N, {:+}Ö (Kayıp/Aşınma: -{}R -{}N)",
                            net.souls, net.influence, net.cosmic_essence, loss.souls, loss.influence
                        )
                    } else {
                        format!(
                            "Tur Başı Ekonomi ({owner_tag}): {:+} Ruh, {:+} Nüfuz, {:+} Kozmik Öz / tur",
                            net.souls, net.influence, net.cosmic_essence
                        )
                    }
                } else {
                    "Tur Başı Ekonomi: Henüz baskın ilah yok (Ruh Havuzu birikiyor)".to_string()
                };

                let authority_note = if is_sovereign {
                    "✓ Söz Sahibisin: Ruh Hasadı, Mucize, Öz Çıkarımı, Uygarlığı Aydınlat, Dokuyu Dönüştür, Kozmik Rezonans, Yaşam Tohumla ve Gerçekliği Onar emirlerini kullanabilirsin."
                } else {
                    "⊗ Hâkimiyetin Yok: Yalnızca Sızma & Mücadele emirlerini (Kehanet Yay, Rakip Vur, İsyan, Geçit Aç, Son Olay) kullanabilirsin!"
                };

                let mut civ_lines = String::new();
                if u.civilizations.is_empty() {
                    civ_lines
                        .push_str("  • Aktif uygarlık yok ('Yaşam Tohumla' ile kurabilirsin)\n");
                } else {
                    for civ in &u.civilizations {
                        let tags = match (civ.divine_shield_active, civ.in_rebellion) {
                            (true, true) => " [KALKAN | İSYAN]",
                            (true, false) => " [BİLİMSEL KALKAN]",
                            (false, true) => " [İSYAN]",
                            (false, false) => "",
                        };
                        civ_lines.push_str(&format!(
                            "  • {} [{} · {}]: {}M | Tek {:.0} | İnanç {:.0}{}\n",
                            civ.name,
                            civ.doctrine.short_tag_tr(),
                            civ.religion_name,
                            civ.population,
                            civ.tech_level,
                            civ.faith_fervor,
                            tags
                        ));
                    }
                }

                let mut hist_lines = String::new();
                for h in u.history.iter().rev().take(3) {
                    hist_lines.push_str(&format!("  • [Tur {}] {}\n", h.tick, h.description));
                }

                format!(
                    "=== {} ({}) ===\n\
                    Aidiyet: {}\n\
                    {}\n\
                    Yetki: {}\n\
                    Ruh Havuzu: {}  |  Ruh Çarpanı: x{:.2}  |  Öz Zenginliği: x{:.2}\n\n\
                    --- Uygarlıklar ({}) ---\n{}\n\
                    --- Son Tarih Kayıtları ---\n{}",
                    u.name,
                    profile.name_tr,
                    allegiance_status,
                    econ_line,
                    authority_note,
                    u.soul_reservoir,
                    u.soul_potential,
                    u.essence_richness,
                    u.civilizations.len(),
                    civ_lines,
                    hist_lines
                )
            }
        }

        InspectorTab::Powers => {
            let player = runtime.sim.player_deity();
            let mut out = format!(
                "=== 5 TANRISAL GÜÇ AĞACI ({}/15 Açık · Aşkınlık İçin: 10 Gerekli) ===\n",
                player.unlocked_powers.len()
            );
            out.push_str(
                "Her güç açıldığında anında KOZMİK PATLAMA etkisi yaratır ve kalıcı pasif/aktif bonus verir!\n\n",
            );

            for tree in PowerTree::ALL {
                out.push_str(&format!("■ {}:\n", tree.name_tr()));
                for p_def in powers_for_tree(tree) {
                    let is_unlocked = player.has_power(p_def.id);
                    let is_next = !is_unlocked && player.can_unlock_power(p_def.id);
                    let status = if is_unlocked {
                        "★ AÇIK"
                    } else if is_next {
                        "▶ AÇILABİLİR"
                    } else {
                        "⊗ KİLİTLİ"
                    };
                    out.push_str(&format!(
                        "  K{}: {} [{}] ({}R/{}N/{}Ö)\n",
                        p_def.tier,
                        p_def.name_tr,
                        status,
                        p_def.cost.souls,
                        p_def.cost.influence,
                        p_def.cost.cosmic_essence,
                    ));
                    if is_unlocked || is_next {
                        out.push_str(&format!("    ↳ {}\n", p_def.id.description_tr()));
                    }
                }
            }
            out
        }

        InspectorTab::Deities => {
            let player = runtime.sim.player_deity();
            let (dom_counts, full_counts) = runtime.sim.all_deity_universe_counts();
            let my_dom = dom_counts
                .get(PLAYER_DEITY_ID as usize)
                .copied()
                .unwrap_or(0);
            let my_full = full_counts
                .get(PLAYER_DEITY_ID as usize)
                .copied()
                .unwrap_or(0);

            let r_idx = (runtime.selected_rival_deity_id as usize)
                .clamp(1, runtime.sim.deities.len().saturating_sub(1));
            let rival = &runtime.sim.deities[r_idx];
            let dom_u = dom_counts.get(rival.id as usize).copied().unwrap_or(0);
            let full_u = full_counts.get(rival.id as usize).copied().unwrap_or(0);
            let rel = rival.relation_toward(PLAYER_DEITY_ID);
            let stance = rival.stance_toward(PLAYER_DEITY_ID);
            let max_seal_dom = if player.has_power(PowerId::EternalSeal) {
                2
            } else {
                1
            };
            let seal_ready = if rival.is_sealed {
                "✓ ZATEN MÜHÜRLENDİ".to_string()
            } else if dom_u <= max_seal_dom {
                format!("★ MÜHÜRLENEBİLİR! (Evren sayısı {dom_u} <= {max_seal_dom})")
            } else {
                format!("Kilitli (Önce evren sayısını {max_seal_dom} veya altına düşür)")
            };

            let mut all_list = String::new();
            for d in runtime.sim.deities.iter().skip(1) {
                let marker = if d.id == rival.id { "▶" } else { " " };
                let seal_tag = if d.is_sealed { " [MÜHÜRLÜ]" } else { "" };
                let d_dom = dom_counts.get(d.id as usize).copied().unwrap_or(0);
                let d_stance = d.stance_toward(PLAYER_DEITY_ID).label_tr();
                all_list.push_str(&format!(
                    "{marker} #{}: {} ({d_stance}) | {d_dom} Evren | Güç: {}/15 | Ritüel: %{:.0}{seal_tag}\n",
                    d.id,
                    d.name,
                    d.unlocked_powers.len(),
                    d.ascension_progress
                ));
            }

            format!(
                "=== TANRILAR MECLİSİ & DİPLOMASİ (Sen: ♛ {} [{} Evren, {} ★]) ===\n\
                Seçili Rakip: #{} {} ({})\n\
                Arketip Hedefi: {}\n\
                Rakip Evrenleri: {} (★ Tam: {})  |  Ruh: {}  |  Tutum: {} (İlişki: {:+})\n\
                Aşkınlık Ritüeli: %{:.0}/100  |  Mühür Durumu: {}\n\n\
                --- 8 Rakip İlah (Alttaki #1..#8 butonlarıyla doğrudan seç!) ---\n{}",
                player.name,
                my_dom,
                my_full,
                rival.id,
                rival.name,
                rival.title,
                rival.archetype.goal_description_tr(),
                dom_u,
                full_u,
                rival.resources.souls,
                stance.label_tr(),
                rel,
                rival.ascension_progress,
                seal_ready,
                all_list
            )
        }

        InspectorTab::VictoryAndLedger => {
            let player = runtime.sim.player_deity();
            let (dom_counts, full_counts) = runtime.sim.all_deity_universe_counts();
            let dom_u = dom_counts
                .get(PLAYER_DEITY_ID as usize)
                .copied()
                .unwrap_or(0);
            let full_u = full_counts
                .get(PLAYER_DEITY_ID as usize)
                .copied()
                .unwrap_or(0);
            let sealed = runtime.sim.sealed_rival_count();
            let shaped = runtime
                .sim
                .harmonious_shaped_universe_count(PLAYER_DEITY_ID);
            let creation_tree_ok = runtime.sim.has_full_creation_tree(PLAYER_DEITY_ID);
            let next_ritual_cost = DivineAction::AdvanceAscensionRitual.base_cost(player);
            let scenarios = built_in_scenarios();
            let cur_preset = &scenarios[runtime
                .active_scenario_index
                .min(scenarios.len().saturating_sub(1))];

            let make_bar = |r: f32| -> String {
                let filled = ((r.clamp(0.0, 1.0) * 10.0).round() as usize).min(10);
                let mut s = String::with_capacity(32);
                s.push('[');
                for i in 0..10 {
                    s.push(if i < filled { '◆' } else { '○' });
                }
                s.push(']');
                s
            };
            let inc = player.last_tick_income;

            let mut out = format!(
                "=== ZORLU ZAFER YOLLARI ({}) ===\n\
                1. HÂKİMİYET ZAFERİ {} %{:.0}:\n\
                   • Baskın Evren: {}/{}  |  ★ Tam Aidiyet: {}/{}\n\
                   • Mühürlenen Rakip Tanrı: {}/{}\n\
                2. AŞKINLIK ZAFERİ {} %{:.0}:\n\
                   • Ritüel: %{:.0}/100 (Sonraki Adım: {}R / {}N / {}Ö)\n\
                   • İlahi Güç: {}/{}  |  ★ Tam Aidiyet: {}/{}\n\
                3. YARATIM ZAFERİ {} %{:.0}:\n\
                   • Kusursuz Düzenli Evren (★ Tam Aidiyet + %75 İstikrar): {}/{}\n\
                   • Yaratım Ağacı (3/3): {}\n\n\
                === KOZMİK EKONOMİ VE KAYNAK DEFTERİ ===\n\
                • Net Tur Akışı: {:+} Ruh/t | {:+} Nüfuz/t | {:+} Öz/t\n",
                cur_preset.title_tr,
                make_bar(dom_ratio),
                dom_ratio * 100.0,
                dom_u,
                DOMINION_REQUIRED_DOMINATED,
                full_u,
                DOMINION_REQUIRED_FULL_ALLEGIANCE,
                sealed,
                DOMINION_REQUIRED_SEALED_RIVALS,
                make_bar(trans_ratio),
                trans_ratio * 100.0,
                player.ascension_progress,
                next_ritual_cost.souls,
                next_ritual_cost.influence,
                next_ritual_cost.cosmic_essence,
                player.unlocked_powers.len(),
                TRANSCENDENCE_REQUIRED_POWERS,
                full_u,
                TRANSCENDENCE_REQUIRED_FULL_ALLEGIANCE,
                make_bar(cre_ratio),
                cre_ratio * 100.0,
                shaped,
                CREATION_REQUIRED_SHAPED_UNIVERSES,
                if creation_tree_ok {
                    "✓ TAMAMLANDI"
                } else {
                    "Eksik (3 Kademeyi Aç)"
                },
                inc.souls,
                inc.influence,
                inc.cosmic_essence,
            );

            for src in [
                ResourceSource::MortalDeaths,
                ResourceSource::FaithDevotion,
                ResourceSource::UniversePassive,
                ResourceSource::DiplomacyTribute,
                ResourceSource::DivineHarvest,
                ResourceSource::AnomalyDiscovery,
            ] {
                let tot = runtime
                    .sim
                    .ledger
                    .totals_by_source
                    .get(&src)
                    .copied()
                    .unwrap_or_default();
                out.push_str(&format!(
                    "  + {}: {}R / {}N / {}Ö\n",
                    src.label_tr(),
                    tot.souls,
                    tot.influence,
                    tot.cosmic_essence
                ));
            }
            for exp in [
                ResourceExpenditure::DivineAction,
                ResourceExpenditure::PowerUnlock,
                ResourceExpenditure::AscensionRitual,
            ] {
                let tot = runtime
                    .sim
                    .ledger
                    .totals_by_expenditure
                    .get(&exp)
                    .copied()
                    .unwrap_or_default();
                out.push_str(&format!(
                    "  - {}: {}R / {}N / {}Ö\n",
                    exp.label_tr(),
                    tot.souls,
                    tot.influence,
                    tot.cosmic_essence
                ));
            }
            for rem in [
                ResourceRemovalReason::UniverseCollapse,
                ResourceRemovalReason::RebellionSchism,
                ResourceRemovalReason::UpkeepDecay,
            ] {
                let tot = runtime
                    .sim
                    .ledger
                    .totals_by_removal
                    .get(&rem)
                    .copied()
                    .unwrap_or_default();
                if tot.souls > 0 || tot.influence > 0 || tot.cosmic_essence > 0 {
                    out.push_str(&format!(
                        "  ! {}: -{}R / -{}N / -{}Ö\n",
                        rem.label_tr(),
                        tot.souls,
                        tot.influence,
                        tot.cosmic_essence
                    ));
                }
            }

            out
        }

        InspectorTab::GuideAndTutorial => {
            let chapters = built_in_guide_chapters();
            let ch_idx = runtime
                .guide_chapter_index
                .min(chapters.len().saturating_sub(1));
            let ch = &chapters[ch_idx];

            let all_steps = [
                TutorialStep::Step1Prophecy,
                TutorialStep::Step2Explore,
                TutorialStep::Step3HarvestOrEssence,
                TutorialStep::Step4UnlockPower,
                TutorialStep::Step5CreateOrStabilize,
                TutorialStep::Step6RivalInteraction,
            ];
            let cur_num = if runtime.sim.tutorial_step == TutorialStep::Completed {
                7
            } else {
                runtime.sim.tutorial_step.step_number()
            };

            let mut checklist = String::new();
            for st in all_steps {
                let n = st.step_number();
                let tag = if n < cur_num {
                    "✓"
                } else if n == cur_num {
                    "▶ AKTİF:"
                } else {
                    "○"
                };
                checklist.push_str(&format!("  {} {}\n", tag, st.title_tr()));
            }

            format!(
                "=== UYGULAMALI EĞİTİCİ GÖREVLER ===\n\
                {}\
                İPUCU: Alttaki '▶ Aktif Görevi Uygula' butonuyla mevcut eğitim adımını anında gerçekleştirebilirsin!\n\n\
                === HARİTA SEMBOLLERİ VE ANİMASYON SÖZLÜĞÜ ===\n\
                • ★ Altın Taç ve Çift Mühür Halkası: Tam Aidiyetimiz (%65+ nüfuz) olan evrenler.\n\
                • ⊗ Hâkimiyet Kuralı: Yalnızca baskın olduğun (◆ BİZİM / ★ TAM) evrenlerde Ruh Hasadı, Mucize, Öz Çıkarımı ve Yaşam Tohumlama yapabilirsin!\n\
                • ⇄ Çizgiler ve Akan Küreler: Evrenler arası geçitler ve inanç/ruh akışı (Çizginin üstüne tıklayarak ayrıntısını görebilirsin!).\n\
                • Duraklatma: Oyun duraklatıldığında tüm harita animasyonları da durur.\n\n\
                === KOZMO ANSİKLOPEDİSİ & WİKİ ({}/{}: {}) ===\n\
                {}\n\
                ★ {}",
                checklist,
                ch_idx + 1,
                chapters.len(),
                ch.title_tr,
                ch.body_tr,
                ch.pro_tip_tr,
            )
        }
    };
    set_text_if_changed(&mut q_body, next_body);
}

fn refresh_bottom_event_log_text(
    runtime: Res<GameRuntimeState>,
    mut last_key: Local<BottomLogCacheKey>,
    mut q_log: Single<&mut Text, With<BottomEventLogText>>,
) {
    if last_key.initialized
        && last_key.tick == runtime.sim.tick
        && last_key.next_event_id == runtime.sim.next_event_id
        && last_key.status_message == runtime.status_message
    {
        return;
    }
    last_key.initialized = true;
    last_key.tick = runtime.sim.tick;
    last_key.next_event_id = runtime.sim.next_event_id;
    last_key.status_message.clone_from(&runtime.status_message);

    let mut lines = format!("BİLGİ: {}\n", runtime.status_message);

    match &runtime.sim.outcome {
        GameOutcome::Ongoing => {}
        GameOutcome::PlayerVictory { path, summary, .. } => {
            lines.push_str(&format!(
                "★ KOZMİK ZAFER ({}) — {}\n",
                path.name_tr(),
                summary
            ));
        }
        GameOutcome::PlayerDefeat { reason, .. } => {
            lines.push_str(&format!("✖ KOZMİK YENİLGİ — {reason}\n"));
        }
    }

    for ev in runtime.sim.events.iter().rev().take(3) {
        lines.push_str(&format!(
            "• [Tur {}] {} {} — {}\n",
            ev.tick,
            ev.kind.symbol_tag(),
            ev.title,
            ev.description
        ));
    }

    set_text_if_changed(&mut q_log, lines);
}

fn refresh_endgame_modal(
    runtime: Res<GameRuntimeState>,
    mut q_modal: Single<(&mut Node, &mut BorderColor), With<EndgameModalOverlay>>,
    mut q_body: Single<&mut Text, With<EndgameModalBodyText>>,
) {
    let (ref mut node, ref mut border) = *q_modal;
    if matches!(runtime.sim.outcome, GameOutcome::Ongoing) || runtime.endgame_modal_dismissed {
        if node.display != Display::None {
            node.display = Display::None;
        }
        return;
    }

    if node.display != Display::Flex {
        node.display = Display::Flex;
    }
    let player = runtime.sim.player_deity();
    let dom_u = runtime.sim.dominated_universe_count(PLAYER_DEITY_ID);
    let full_u = runtime.sim.full_allegiance_universe_count(PLAYER_DEITY_ID);
    let sealed = runtime.sim.sealed_rival_count();
    let shaped = runtime
        .sim
        .harmonious_shaped_universe_count(PLAYER_DEITY_ID);

    match &runtime.sim.outcome {
        GameOutcome::Ongoing => {}
        GameOutcome::PlayerVictory {
            path,
            tick,
            summary,
        } => {
            border.set_all(COL_ACCENT_GOLD);
            set_text_if_changed(
                &mut q_body,
                format!(
                    "★★★ KOZMİK ZAFER: {} ★★★\n\n\
                    {}\n\n\
                    --- Kozmik İmparatorluk İstatistiklerin ({}) ---\n\
                    • Tamamlanan Tur: {}\n\
                    • ★ Tam Aidiyet Kurulan Evrenler: {}\n\
                    • Toplam Baskın Evrenler: {} / 60\n\
                    • Mühürlenen Rakip Tanrılar: {} / 8\n\
                    • Açılan İlahi Güçler: {} / 15\n\
                    • Kusursuz Düzenli Evrenler: {}\n\n\
                    Dilersen 'Sonsuz Havuz Modunda Devam Et' butonuna tıklayarak mevcut evren havuzunda sınırsız oynamaya devam edebilirsin!",
                    path.name_tr(),
                    summary,
                    player.name,
                    tick,
                    full_u,
                    dom_u,
                    sealed,
                    player.unlocked_powers.len(),
                    shaped
                ),
            );
        }
        GameOutcome::PlayerDefeat { tick, reason } => {
            border.set_all(COL_ACCENT_CRIMSON);
            set_text_if_changed(
                &mut q_body,
                format!(
                    "✖✖✖ KOZMİK YENİLGİ (Tur {}) ✖✖✖\n\n\
                    {}\n\n\
                    --- Son Durum Özeti ({}) ---\n\
                    • ★ Tam Aidiyet Evrenlerin: {}\n\
                    • Baskın Evrenlerin: {}\n\
                    • Mühürlenen Rakipler: {}\n\
                    • Açılan İlahi Güçlerin: {} / 15\n\n\
                    Dilersen 'Sonsuz Havuz Modunda Devam Et' ile bu evrende devam edebilir veya yeni bir senaryo başlatabilirsin.",
                    tick,
                    reason,
                    player.name,
                    full_u,
                    dom_u,
                    sealed,
                    player.unlocked_powers.len()
                ),
            );
        }
    }
}

type MainMenuDeityNameFilter = (With<MainMenuDeityNameText>, Without<MainMenuSettingsText>);

type MainMenuSettingsTextFilter = (With<MainMenuSettingsText>, Without<MainMenuDeityNameText>);

fn refresh_main_menu_overlay(
    runtime: Res<GameRuntimeState>,
    mut q_overlay: Single<&mut Node, (With<MainMenuOverlay>, Without<MainMenuSettingsContainer>)>,
    mut q_settings_box: Single<
        &mut Node,
        (With<MainMenuSettingsContainer>, Without<MainMenuOverlay>),
    >,
    mut q_name_text: Single<&mut Text, MainMenuDeityNameFilter>,
    mut q_settings_text: Single<&mut Text, MainMenuSettingsTextFilter>,
) {
    let target_overlay_disp = if runtime.show_main_menu {
        Display::Flex
    } else {
        Display::None
    };
    if q_overlay.display != target_overlay_disp {
        q_overlay.display = target_overlay_disp;
    }

    if !runtime.show_main_menu {
        return;
    }

    let target_settings_disp = if runtime.main_menu_settings_open {
        Display::Flex
    } else {
        Display::None
    };
    if q_settings_box.display != target_settings_disp {
        q_settings_box.display = target_settings_disp;
    }

    let shown_name = if runtime.player_name_input.is_empty() {
        "Aethelgard"
    } else {
        &runtime.player_name_input
    };
    set_text_if_changed(
        &mut q_name_text,
        format!("♛ İLAH ADIN (OYUNCU KİMLİĞİ): [ {} ]", shown_name),
    );

    let scenarios = built_in_scenarios();
    let cur_scen = &scenarios[runtime
        .active_scenario_index
        .min(scenarios.len().saturating_sub(1))];
    set_text_if_changed(
        &mut q_settings_text,
        format!(
            "=== ⚙ OYUN VE GÖRÜNÜM AYARLARI ===\n\
            • Seçili Başlangıç Senaryosu: {} ({})\n\
            • Aktif Arayüz & Harita Teması: {}\n\
            • Prosedürel Kozmik Ses Sentezleyicisi (♪): {}\n\
            • Yüksek Kontrastlı Tehlike Sembolleri (▲): {}\n\
            • Uzak Evren Arka Plan Optimizasyonu: {}",
            cur_scen.title_tr,
            cur_scen.subtitle_tr,
            runtime.theme_name_tr(),
            if runtime.audio_enabled {
                "AÇIK (Kısayol: V)"
            } else {
                "KAPALI (Kısayol: V)"
            },
            if runtime.high_contrast_symbols {
                "AÇIK"
            } else {
                "KAPALI"
            },
            if runtime.sim.enable_background_optimization {
                "AÇIK"
            } else {
                "KAPALI"
            }
        ),
    );
}

#[derive(Default)]
struct HoverTooltipTracker {
    hovered_entity: Option<Entity>,
    elapsed_secs: f32,
}

type HoverButtonQuery<'a> = (
    Entity,
    &'a Hovered,
    Option<&'a UiAction>,
    Option<&'a LeftQuickSlotButton>,
);

fn refresh_button_hover_tooltip(
    time: Res<Time>,
    runtime: Res<GameRuntimeState>,
    mut tracker: Local<HoverTooltipTracker>,
    mut pool_cache: Local<FilteredPoolCache>,
    q_buttons: Query<HoverButtonQuery, With<Button>>,
    mut q_overlay: Single<&mut Node, With<ButtonHoverTooltipOverlay>>,
    mut q_text: Single<&mut Text, With<ButtonHoverTooltipText>>,
) {
    let mut active_hover: Option<(Entity, Option<UiAction>, Option<usize>)> = None;
    for (ent, hovered, action_opt, slot_opt) in &q_buttons {
        if hovered.get() {
            active_hover = Some((ent, action_opt.copied(), slot_opt.map(|s| s.0)));
            break;
        }
    }

    let Some((ent, action_opt, slot_idx_opt)) = active_hover else {
        tracker.hovered_entity = None;
        tracker.elapsed_secs = 0.0;
        if q_overlay.display != Display::None {
            q_overlay.display = Display::None;
        }
        return;
    };

    if tracker.hovered_entity == Some(ent) {
        tracker.elapsed_secs += time.delta_secs();
    } else {
        tracker.hovered_entity = Some(ent);
        tracker.elapsed_secs = 0.0;
    }

    // Show detailed explanation card when cursor stays on an interactive button for ~2 seconds
    if tracker.elapsed_secs < 1.85 {
        if q_overlay.display != Display::None {
            q_overlay.display = Display::None;
        }
        return;
    }

    let filtered = pool_cache.get(&runtime);
    let tip_str = if let Some(slot_idx) = slot_idx_opt {
        let abs_idx = runtime.pool_page * POOL_SLOTS_PER_PAGE + slot_idx;
        if let Some(&u_id) = filtered.get(abs_idx) {
            let u = &runtime.sim.universes[u_id as usize];
            if u.discovered_by_player {
                let my_inf = u.influence_of(PLAYER_DEITY_ID);
                format!(
                    "★ EVREN YUVASI #{} — {}\n\
                    Bu butona tıkladığında harita kamerası doğrudan '{}' evrenine odaklanır ve sağ denetçi panelinde bu evrenin uygarlıklarını, ekonomisini ve emirlerini açar. (Mevcut Nüfuzun: %{:.0} · İstikrar: %{:.0})",
                    slot_idx + 1,
                    u.name,
                    u.name,
                    my_inf,
                    u.stability
                )
            } else {
                format!(
                    "★ GİZEMLİ EVREN YUVASI #{}\n\
                    Henüz keşfedilmemiş kozmik bölge (#{}). Tıklayarak haritada yerini görebilir, komşu bir hâkim evrenden 'Keşif' veya 'Geçit Aç' emriyle bu evreni açabilirsin.",
                    slot_idx + 1,
                    u.id + 1
                )
            }
        } else {
            "★ BOŞ HAVUZ YUVASI\nMevcut filtre/sayfa seçiminde bu yuvada gösterilecek evren bulunmuyor.".to_string()
        }
    } else if let Some(action) = action_opt {
        describe_ui_action_tooltip(action, &runtime)
    } else {
        "★ ETKİLEŞİMLİ BUTON\nSol tıklayarak bu komutu çalıştırabilirsin.".to_string()
    };

    if q_overlay.display != Display::Flex {
        q_overlay.display = Display::Flex;
    }
    set_text_if_changed(&mut q_text, tip_str);
}

fn describe_ui_action_tooltip(action: UiAction, runtime: &GameRuntimeState) -> String {
    let locked = is_ui_action_sovereign_locked(&action, runtime);
    let affordable = can_afford_ui_action(&action, runtime);
    let player = runtime.sim.player_deity();
    let live_cost_badge = if let Some(div_act) = ui_action_to_divine_action(&action, runtime) {
        let c = div_act.base_cost(player);
        format!(
            "\n[Net Maliyet (Açık Güç İndirimli): {} Ruh · {} Nüfuz · {} Kozmik Öz]",
            c.souls, c.influence, c.cosmic_essence
        )
    } else {
        String::new()
    };
    let status_badge = if locked {
        "\n[Durum: ⊗ HÂKİMİYET KİLİTLİ — Seçili evrende önce Baskın İlah (◆ BİZİM / ★ TAM) olmalısın]"
    } else if !affordable {
        "\n[Durum: ▲ KAYNAK YETERSİZ — Bu emri vermek için biraz daha Ruh / Nüfuz / Öz biriktirmelisin]"
    } else {
        "\n[Durum: ✓ KULLANILABİLİR — Sol tıklayarak hemen uygulayabilirsin]"
    };

    let base = match action {
        UiAction::ToggleMainMenu => {
            "★ ANA MENÜ (Kısayol: Esc)\nKozMO Ana Menüsünü açar veya kapatır. Buradan ilah adını değiştirebilir, yeni senaryo başlatabilir ve görünüm ayarlarını düzenleyebilirsin.".to_string()
        }
        UiAction::MainMenuStartNewGame => {
            "★ YENİ OYUN BAŞLAT\nSeçili senaryo ve belirlediğin İlah Adı ile 60 evrenlik yeni bir KozMO kozmosu oluşturur ve oyunu başlatır.".to_string()
        }
        UiAction::ToggleMainMenuSettings => {
            "★ OYUN VE GÖRÜNÜM AYARLARI\nBaşlangıç senaryosunu (Dengeli, Büyük Üstat, Serbest Mod), arayüz temasını ve uzak evren performans optimizasyonunu yapılandırır.".to_string()
        }
        UiAction::CycleRandomDeityName => {
            "★ RASTGELE İLAH ADI SEÇ\nOyuncu ilah kimliğin için kadim kozmik isim havuzundan rastgele yeni bir isim seçer. Dilersen klavyeden doğrudan yazarak da değiştirebilirsin.".to_string()
        }
        UiAction::CycleUiTheme => {
            format!(
                "★ TEMA DEĞİŞTİR (Kısayol: T · Aktif: {})\nArayüz ve kozmik harita renk paletini 4 farklı kozmik atmosfer teması (Safir Mor, Zümrüt Yeşim, Kızıl Kehribar, Ebedi Mermer & Altın Usturlap) arasında anında değiştirir.",
                runtime.theme_name_tr()
            )
        }
        UiAction::CycleMapOverlayMode => {
            format!(
                "★ HARİTA KATMANI DEĞİŞTİR (Kısayol: M · Aktif: {})\nKozmik haritayı 4 farklı analitik katman (1. Standart Kozmos, 2. İlahi Hâkimiyet & Bloklar, 3. Ekonomi & Ruh Havuzu, 4. İstikrar & Tehlike Isısı) arasında geçirir.",
                runtime.map_overlay_mode.label_tr()
            )
        }
        UiAction::ToggleAudio => {
            format!(
                "★ PROSEDÜREL KOZMİK SES SENTEZLEYİCİSİ (Kısayol: V · Durum: {})\nBellek içi çok-harmonikli prosedürel ses sentezleyicisini açar veya kapatır. Kehanet çanları, mucize akorları, ruh sifonu, ilahi darbe ve geçit rezonanslarını gerçek zamanlı üretir.",
                if runtime.audio_enabled { "AÇIK" } else { "KAPALI" }
            )
        }
        UiAction::ToggleBackgroundOptimization => {
            "★ UZAK EVREN ARKA PLAN OPTİMİZASYONU\nOdak dışındaki uzak evrenlerin simülasyonunu deterministik toplu adımlarla çalıştırarak 60 evrende yüksek FPS ve düşük işlemci kullanımı sağlar.".to_string()
        }
        UiAction::ExitGame => {
            "★ OYUNDAN ÇIKIŞ\nKozMO simülasyonunu güvenli biçimde kapatır ve masaüstüne döner.".to_string()
        }
        UiAction::SetSpeed(spd) => {
            format!(
                "★ ZAMAN AKIŞ HIZI: {}\nKozmik simülasyonun akış hızını ayarlar. Duraklatıldığında tüm harita ve geçit animasyonları da tam olarak durur (Kısayollar: Space, 1, 2, 3).",
                spd.label_tr()
            )
        }
        UiAction::StepOneTick => {
            "★ +1 TUR İLERLET (Kısayol: . veya Enter)\nOyun duraklatılmış olsa bile simülasyonu tam olarak 1 kozmik tur ilerletir; ekonomi ve olayları adım adım incelemek için idealdir.".to_string()
        }
        UiAction::QuickSave => {
            "★ HIZLI KAYDET (Kısayol: F5)\nMevcut 60 evreni, 9 ilahı, kaynak defterini ve tarihçeyi diske güvenli (atomik) olarak kaydeder.".to_string()
        }
        UiAction::QuickLoad => {
            "★ HIZLI YÜKLE (Kısayol: F9)\nEn son kaydedilen KozMO oyun durumunu diskten geri yükler.".to_string()
        }
        UiAction::SetOwnershipFilter(of) => {
            format!(
                "★ HAVUZ FİLTRESİ: {}\nSol paneldeki 60 evrenlik listeyi yalnızca '{}' durumundaki evrenleri gösterecek şekilde filtreler.",
                of.label_tr(),
                of.label_tr()
            )
        }
        UiAction::CycleTypeFilter => {
            "★ EVREN KATEGORİSİ FİLTRESİ\nSol havuz listesini evren türlerine göre (Genç, Yükselen, Kadim, Çökmekte Olan, Anomali, Hiçlik) filtreler.".to_string()
        }
        UiAction::CycleSortMode => {
            "★ EVREN SIRALAMA ÖLÇÜTÜ\nSol havuzdaki evrenleri Nüfuzumuz, Ruh Havuzu, İstikrar, Tehlike veya Evren No ölçütüne göre sıralar.".to_string()
        }
        UiAction::PrevPoolPage | UiAction::NextPoolPage => {
            "★ EVREN HAVUZU SAYFALAMA (Kısayol: [ ve ])\nFiltrelenmiş evren listesinin önceki veya sonraki 8'lik sayfasına geçer.".to_string()
        }
        UiAction::SelectFilteredSlot(slot) => {
            format!(
                "★ HIZLI EVREN YUVASI #{}\nBu yuvadaki evreni seçer ve harita kamerasını üzerine odaklar.",
                slot + 1
            )
        }
        UiAction::SelectTab(tab) => {
            format!(
                "★ DENETÇİ SEKMESİ: {}\nSağ panelde '{}' görünümünü ve ilgili etkileşim butonlarını açar.",
                tab.label_tr(),
                tab.label_tr()
            )
        }
        UiAction::ExecuteOrShowTutorialStep => {
            "★ AKTİF EĞİTİCİ GÖREVİ UYGULA\nMevcut eğitici adımı senin yerine otomatik olarak gerçekleştirir veya ilgili evreni/sekmeyi açarak ödülü (+Ruh, +Nüfuz, +Öz) kazandırır.".to_string()
        }
        UiAction::ToggleTutorialCollapse => {
            "★ GÖREV KUTUSUNU DARALT / AÇ\nSağ üstteki eğitici görev ve ikilem duyuru kutusunu daraltarak denetçi metnine daha fazla yer açar veya tekrar genişletir.".to_string()
        }
        UiAction::ResetTutorial => {
            "★ EĞİTİCİ GÖREVLERİ SIFIRLA\n6 adımlı uygulamalı başlangıç görev serisini 1. adımdan tekrar başlatır.".to_string()
        }
        UiAction::SpreadProphecy => {
            "★ KEHANET YAY (Maliyet: 20 Ruh, 25 Nüfuz)\n[Sızma Emri — Her Keşfedilen Evrende Açık]\nSeçili evrende ilahi kehanetler yayarak nüfuzunu +%16 artırır, halkın inanç coşkusunu yükseltir ve evreni hâkimiyetine yaklaştırır.".to_string()
        }
        UiAction::SmiteRival => {
            "★ RAKİP İLAHI VUR (Maliyet: 75 Ruh, 35 Nüfuz, 10 Öz)\n[Sızma Emri — Her Keşfedilen Evrende Açık]\nSeçili evrendeki baskın rakip ilahın nüfuzunu -%20 kırar ve kendi nüfuzunu +%8 artırır; rakip evrenleri ele geçirmek için kullanılır.".to_string()
        }
        UiAction::InciteRebellion => {
            "★ İSYAN ÇIKAR (Maliyet: 30 Ruh, 40 Nüfuz)\n[Sızma Emri — Her Keşfedilen Evrende Açık]\nEvrendeki ölümlü uygarlıkları mevcut hâkim ilaha karşı ayaklandırır; rakibin nüfuzunu sarsar ve tehlike seviyesini artırır.".to_string()
        }
        UiAction::OpenConnectedGateways => {
            "★ BOYUTSAL GEÇİT AÇ (Maliyet: 40 Ruh, 20 Nüfuz, 10 Öz)\nSeçili evrenden komşu bir evrene kapalı olan boyutsal geçidi açar, hedef evreni keşfeder ve evrenler arası inanç/ruh akışını başlatır.".to_string()
        }
        UiAction::FocusLatestEventUniverse => {
            "★ SON OLAY EVRENİNE GİT (Kısayol: E)\nEn son kozmik olayın veya anomalinin gerçekleştiği evreni anında seçer ve kamerayı oraya odaklar.".to_string()
        }
        UiAction::HarvestSouls => {
            "★ RUH HASADI (Maliyet: 25 Nüfuz -> Anında +Ruh)\n[Hâkimiyet Emri — Söz Sahibi Evrenlerde]\nEvrende biriken Ruh Havuzunu (Rezervuar) ve ölümlü nüfusun bir kısmını hasat ederek hazinene anında yüksek miktarda Ruh kazandırır.".to_string()
        }
        UiAction::BestowMiracle => {
            "★ MUCİZE BAHŞET (Maliyet: 60 Ruh, 25 Nüfuz)\n[Hâkimiyet Emri — Söz Sahibi Evrenlerde]\nEvrene ilahi mucize indirir: İstikrarı +%18, nüfuzunu +%12 artırır, tehlikeyi düşürür ve devam eden isyanları bastırır.".to_string()
        }
        UiAction::ExtractCosmicEssence => {
            "★ KOZMİK ÖZ ÇIKAR (Maliyet: 65 Ruh, 25 Nüfuz -> Anında +Öz)\n[Hâkimiyet Emri — Söz Sahibi Evrenlerde]\nEvrenin boyutsal dokusunu damıtarak güç ağaçları ve aşkınlık ritüeli için gereken nadir Kozmik Öz'ü elde eder (İstikrarı biraz düşürür).".to_string()
        }
        UiAction::EnlightenCivilizations => {
            "★ UYGARLIĞI AYDINLAT (Maliyet: 35 Ruh, 20 Nüfuz -> +Öz & +Teknoloji)\n[Hâkimiyet Emri — Söz Sahibi Evrenlerde]\nEvrendeki uygarlıklara ilahi bilgelik verir: Teknoloji +16, İnanç +14, İstikrar +8 artar; Bilimsel İlahi Kalkan açar ve anında Kozmik Öz üretir.".to_string()
        }
        UiAction::ReshapeUniverseDomain => {
            "★ EVREN DOKUSUNU DÖNÜŞTÜR (Maliyet: 55 Ruh, 25 Nüfuz, 20 Öz)\n[Hâkimiyet Emri — Söz Sahibi Evrenlerde]\nEvrenin kozmik türünü bir üst evreye dönüştürür (Hiçlik -> Çökmekte -> Genç -> Yükselen -> Kadim) ve kalıcı Ruh/Öz çarpanlarını büyütür!".to_string()
        }
        UiAction::ChannelAmbientEnergy => {
            "★ KOZMİK REZONANS (Maliyet: 30 Nüfuz, 10 Öz -> +Ruh & Komşu Yayılımı)\n[Hâkimiyet Emri — Söz Sahibi Evrenlerde]\nEvrenden anında Ruh hasat eder, nüfuzunu +%10 artırır ve açık geçitlerden tüm komşu evrenlere otomatik olarak +%6 inanç yayar.".to_string()
        }
        UiAction::SeedLifeAndOrder => {
            "★ YAŞAM VE DÜZEN TOHUMLA (Maliyet: 110 Ruh, 40 Nüfuz, 25 Öz)\n[Hâkimiyet Emri — Söz Sahibi Evrenlerde]\nEvrende yeni bir ölümlü uygarlık kurar veya mevcut uygarlıkların nüfusunu büyütür; İstikrarı +%20 artırır (Yaratım Zaferi için anahtar emirdir).".to_string()
        }
        UiAction::StabilizeReality => {
            "★ GERÇEKLİĞİ ONAR (Maliyet: 85 Ruh, 35 Nüfuz, 20 Öz)\n[Hâkimiyet Emri — Söz Sahibi Evrenlerde]\nBoyutsal yırtıkları kapatarak İstikrarı +%26 yükseltir ve Tehlikeyi -%22 düşürür; evrenin hiçliğe çökmesini engeller.".to_string()
        }
        UiAction::ScoutRegion => {
            "★ KOZMİK KEŞİF (Maliyet: 20 Ruh, 20 Nüfuz)\n[Hâkimiyet Emri — Söz Sahibi Evrenlerde]\nSeçili evrene bağlı tüm komşu gizemli evrenleri keşfeder ve anında keşif ödülü (+Kozmik Öz) kazandırır.".to_string()
        }
        UiAction::CloseConnectedGateways => {
            "★ BOYUTSAL GEÇİDİ KAPAT (Maliyet: 25 Ruh, 25 Nüfuz)\n[Hâkimiyet Emri — Söz Sahibi Evrenlerde]\nSeçili evrendeki açık geçidi kapatarak rakip ilahların bu bağlantı üzerinden evrenine inanç sızdırmasını engeller.".to_string()
        }
        UiAction::FocusSelectedUniverse => {
            "★ İLAHİ ODAK KUR / KALDIR (Kısayol: F)\n[Hâkimiyet Emri — Söz Sahibi Evrenlerde]\nSeçili evreni İlahi Odak noktası yapar: O evrenden elde edilen tüm tur başı Ruh, Nüfuz ve Öz gelirlerini +%25 artırır ve istikrarını korur.".to_string()
        }
        UiAction::ClearInspectedGateway => {
            "★ EVREN GÖRÜNÜMÜNE DÖN\nGeçit inceleme ekranını kapatır ve seçili evrenin genel detaylarına geri döner.".to_string()
        }
        UiAction::ResolveDilemmaChoice(first) => {
            if let Some(d) = &runtime.sim.active_dilemma {
                if first {
                    format!("★ İKİLEM KARARI A: {}\nAktif kozmik ikilemde 1. seçeneği uygular: {}", d.choice_a_label, d.description)
                } else {
                    format!("★ İKİLEM KARARI B: {}\nAktif kozmik ikilemde 2. seçeneği uygular: {}", d.choice_b_label, d.description)
                }
            } else {
                "★ KOZMİK İKİLEM KARARI\nŞu anda bekleyen bir kozmik ikilem yok. Evrenlerde rastgele bir ikilem çıktığında bu butonlarla karar verebilirsin.".to_string()
            }
        }
        UiAction::UnlockNextInTree(tree) => {
            format!(
                "★ {} AĞACINI GELİŞTİR\nBu güç ağacındaki sıradaki ilahi gücü açar. Her yeni güç açıldığında tüm evrenlerinde anında tek seferlik Kozmik Patlama etkisi yaratır ve kalıcı bonus verir!",
                tree.name_tr().to_uppercase()
            )
        }
        UiAction::PrevRivalDeity | UiAction::NextRivalDeity => {
            "★ RAKİP İLAH DEĞİŞTİR\nTanrılar Meclisi'ndeki 8 rakip ilah arasında sırayla geçiş yapar.".to_string()
        }
        UiAction::SelectRivalDeity(r_id) => {
            if let Some(d) = runtime.sim.deities.get(r_id as usize) {
                format!(
                    "★ RAKİP İLAH #{}: {} ({})\nBu rakip ilahı doğrudan seçer; arketip hedefini, evren sayısını ve diplomatik seçeneklerini gösterir.",
                    r_id, d.name, d.title
                )
            } else {
                format!("★ RAKİP İLAH #{r_id}\nBu rakip ilahı seçer.")
            }
        }
        UiAction::FocusRivalUniverse => {
            "★ RAKİBİN EVRENİNİ HARİTADA BUL\nSeçili rakip ilahın hâkim olduğu evrenlerden birini bulur ve harita kamerasını doğrudan o evrene odaklar.".to_string()
        }
        UiAction::SendDivineGift => {
            "★ RUH ARMAĞANI GÖNDER (Maliyet: 70 Ruh, 5 Öz)\nSeçili rakip ilaha diplomatik armağan göndererek ikili ilişkiyi +28 iyileştirir ve düşmanlığı azaltır.".to_string()
        }
        UiAction::OfferPact => {
            "★ ATEŞKES PAKTI TEKLİF ET (Maliyet: 35 Ruh, 45 Nüfuz)\nİlişkinin -15 veya üzerinde olduğu rakip ilahla karşılıklı saldırmazlık ve işbirliği paktı (+35 İlişki) kurar.".to_string()
        }
        UiAction::DemandTribute => {
            "★ HARAÇ İSTE (Maliyet: 20 Ruh, 35 Nüfuz -> Anında +Ruh & +Öz)\nZayıflamış veya çekinen rakip ilahtan zorla Ruh ve Kozmik Öz haracı tahsil eder (İlişkiyi düşürür).".to_string()
        }
        UiAction::DeclareHolyWar => {
            "★ KUTSAL REKABET İLAN ET (Maliyet: 60 Ruh, 40 Nüfuz, 10 Öz)\nSeçili rakip ilaha karşı topyekûn kozmik savaş başlatır ve çekişmeli evrenlerdeki nüfuzunu sarsar.".to_string()
        }
        UiAction::DisruptRivalRitual => {
            "★ RAKİP RİTÜELİNİ SABOTE ET (Maliyet: 220 Ruh, 110 Nüfuz, 45 Öz)\nAşkınlık zaferine yaklaşan rakip ilahın ritüel ilerlemesini anında -%20 geri düşürür.".to_string()
        }
        UiAction::SealRival => {
            "★ ZAYIFLAMIŞ TANRIYI MÜHÜRLE (Maliyet: 320 Ruh, 150 Nüfuz, 80 Öz)\nEvren sayısı 1 veya altına (Ebedi Mühür gücüyle 2 veya altına) düşmüş rakip ilahı kalıcı olarak mühürler ve kalan evrenlerini sana devreder!".to_string()
        }
        UiAction::AdvanceAscension => {
            "★ AŞKINLIK RİTÜELİNİ İLERLET (+%10 İlerleme · Toplam 10 Aşama)\nAşkınlık Zaferi için kozmik ritüeli +%10 ilerletir. %100'e ulaştığında (10 Güç ve 10 ★ Tam Aidiyet ile) oyunu zaferle taçlandırır.".to_string()
        }
        UiAction::ContinueInSandbox => {
            "★ SONSUZ HAVUZ (SANDBOX) MODUNDA DEVAM ET\nZafer veya yenilgi sonrasında mevcut kozmosta hiçbir tur sınırı olmadan oynamaya devam etmeni sağlar.".to_string()
        }
        UiAction::DismissEndgameModal => {
            "★ SONUÇ PENCERESİNİ KAPAT\nZafer/Yenilgi bilgi penceresini kapatarak arka plandaki kozmik haritayı incelemene olanak tanır.".to_string()
        }
        UiAction::StartScenario(idx) => {
            let scenarios = built_in_scenarios();
            if let Some(sc) = scenarios.get(idx) {
                format!(
                    "★ SENARYO BAŞLAT: {} ({})\n{}",
                    sc.title_tr, sc.subtitle_tr, sc.description_tr
                )
            } else {
                "★ YENİ SENARYO BAŞLAT\nSeçili senaryo ayarlarıyla yeni bir oyun başlatır.".to_string()
            }
        }
        UiAction::ToggleHighContrastSymbols => {
            "★ YÜKSEK KONTRASTLI TEHLİKE SEMBOLLERİ (▲)\nHaritadaki tehlikeli veya istikrarsız evrenlerin üzerinde ek görsel uyarı üçgenleri gösterilmesini açar/kapatır.".to_string()
        }
        UiAction::PrevGuideChapter | UiAction::NextGuideChapter => {
            "★ WİKİ KONUSU DEĞİŞTİR\nKozMO Ansiklopedisi & Wiki içerisindeki 8 detaylı rehber bölümü arasında ileri/geri geçiş yapar.".to_string()
        }
        UiAction::SelectGuideChapter(idx) => {
            let chapters = built_in_guide_chapters();
            if let Some(ch) = chapters.get(idx) {
                format!(
                    "★ WİKİ BÖLÜM {}: {}\nBu butona tıklayarak '{}' başlıklı detaylı Wiki maddesini okuyabilirsin.",
                    idx + 1,
                    ch.title_tr,
                    ch.title_tr
                )
            } else {
                format!("★ WİKİ BÖLÜM {}\nİlgili Wiki konusunu açar.", idx + 1)
            }
        }
    };

    format!("{base}{live_cost_badge}{status_badge}")
}
