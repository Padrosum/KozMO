use crate::audio::CosmicSfxKind;
use crate::plugins::{GameRuntimeState, InspectorTab, MapOverlayMode, MapVfxKind};
use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    math::Isometry2d,
    prelude::*,
};
use dominion_sim::{GatewayState, UniverseType, PLAYER_DEITY_ID};
use std::f32::consts::{FRAC_PI_4, FRAC_PI_6, PI, TAU};

pub struct CosmicMapPlugin;

#[derive(Component)]
pub struct CosmicMapCamera;

#[derive(Component)]
pub struct UniverseMapLabel(pub u32);

#[derive(Component)]
pub struct GatewayHoverLabel;

#[derive(Component)]
pub struct FloatingMapToast {
    pub elapsed: f32,
    pub duration: f32,
    pub base_color: Color,
}

impl Plugin for CosmicMapPlugin {
    fn build(&self, app: &mut App) {
        // Semavi Kehribar & Obsidyen Tapınak arka planı (#07050F)
        app.insert_resource(ClearColor(Color::srgb_u8(0x07, 0x05, 0x0F)))
            .add_systems(Startup, setup_cosmic_map_camera)
            .add_systems(
                Update,
                (
                    control_cosmic_camera,
                    pick_universe_node_on_click,
                    draw_cosmic_map_gizmos,
                    update_and_draw_map_vfx,
                    animate_floating_map_toasts,
                    sync_universe_map_labels,
                ),
            );
    }
}

fn setup_cosmic_map_camera(
    mut commands: Commands,
    runtime: Res<GameRuntimeState>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera2d,
        CosmicMapCamera,
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // A transparent, pre-rendered nebula layer gives the map real background color
    // while leaving the theme-dependent clear color visible through the empty areas.
    commands.spawn((
        Sprite::from_image(asset_server.load("art/backgrounds/cosmic_nebula.png")),
        Transform::from_xyz(0.0, 0.0, -100.0),
    ));

    for u in &runtime.sim.universes {
        commands.spawn((
            UniverseMapLabel(u.id),
            Text2d::new(""),
            TextFont {
                font_size: FontSize::Px(10.5),
                ..default()
            },
            TextColor(Color::srgb_u8(0xF8, 0xF3, 0xE6)),
            Transform::from_xyz(u.position[0], u.position[1] - 21.0, 5.0),
        ));
    }

    commands.spawn((
        GatewayHoverLabel,
        Text2d::new(""),
        TextFont {
            font_size: FontSize::Px(11.0),
            ..default()
        },
        TextColor(Color::srgb_u8(0xFF, 0xB8, 0x4D)),
        Transform::from_xyz(0.0, 0.0, 9.0),
        Visibility::Hidden,
    ));
}

type UniverseLabelQuery<'a> = (
    &'a UniverseMapLabel,
    &'a mut Text2d,
    &'a mut TextColor,
    &'a mut Transform,
    &'a mut Visibility,
);

type GatewayLabelQuery<'a> = (
    &'a mut Text2d,
    &'a mut TextColor,
    &'a mut Transform,
    &'a mut Visibility,
);

fn sync_universe_map_labels(
    time: Res<Time>,
    runtime: Res<GameRuntimeState>,
    mut clear_color: ResMut<ClearColor>,
    mut q_labels: Query<UniverseLabelQuery, Without<GatewayHoverLabel>>,
    mut q_gw_label: Single<GatewayLabelQuery, With<GatewayHoverLabel>>,
) {
    let dt = time.delta_secs();
    let target_clear = match runtime.ui_theme_index % 4 {
        0 => Color::srgb_u8(0x07, 0x05, 0x0F),
        1 => Color::srgb_u8(0x04, 0x0D, 0x10),
        2 => Color::srgb_u8(0x11, 0x06, 0x0B),
        _ => Color::srgb_u8(0x0B, 0x0E, 0x17),
    };
    if clear_color.0 != target_clear {
        clear_color.0 = target_clear;
    }

    let col_primary = Color::srgb_u8(0xF8, 0xF3, 0xE6);
    let col_secondary = Color::srgb_u8(0xB9, 0xAD, 0xD2);
    let col_gold = Color::srgb_u8(0xFF, 0xB8, 0x4D);
    let col_turquoise = Color::srgb_u8(0x2E, 0xE5, 0xB5);
    let col_crimson = Color::srgb_u8(0xFF, 0x4F, 0x79);

    for (label_id, mut text, mut color, mut transform, mut vis) in &mut q_labels {
        let idx = label_id.0 as usize;
        let Some(u) = runtime.sim.universes.get(idx) else {
            *vis = Visibility::Hidden;
            continue;
        };
        if u.is_destroyed {
            *vis = Visibility::Hidden;
            continue;
        }

        let is_selected = u.id == runtime.selected_universe_id;
        let is_hovered = runtime.hovered_universe_id == Some(u.id);
        let full_allegiance = u.has_full_allegiance(PLAYER_DEITY_ID);
        let player_dominant = u.is_dominated_by(PLAYER_DEITY_ID);
        let has_dilemma = runtime
            .sim
            .active_dilemma
            .as_ref()
            .is_some_and(|d| d.universe_id == u.id);
        let is_last_event = runtime.sim.last_event_universe_id == Some(u.id);

        let target_scale = if is_selected {
            1.14
        } else if is_hovered {
            1.08
        } else {
            1.0
        };
        let cur_scale = transform.scale.x;
        let next_scale = cur_scale + (target_scale - cur_scale) * (dt * 12.0).min(1.0);
        transform.scale = Vec3::splat(next_scale);

        transform.translation.x = u.position[0];
        transform.translation.y = u.position[1]
            - if is_hovered || is_selected {
                23.5
            } else {
                21.0
            };

        let filtered_out = runtime.type_filter.is_some_and(|ft| ft != u.universe_type);
        if filtered_out {
            *vis = Visibility::Hidden;
            continue;
        }
        *vis = Visibility::Inherited;

        if u.discovered_by_player {
            let inf = u.influence_of(PLAYER_DEITY_ID);
            let badge = if full_allegiance {
                "★ "
            } else if player_dominant {
                "◆ "
            } else {
                ""
            };
            let event_suffix = if has_dilemma {
                " · ★ İKİLEM"
            } else if is_last_event {
                " · ✦ OLAY"
            } else {
                ""
            };
            if is_hovered && !is_selected {
                let allegiance_tag = if full_allegiance {
                    " · TAM AİDİYET"
                } else {
                    ""
                };
                **text = format!(
                    "{}{} [%{:.0} | İst: %{:.0}{}{}]",
                    badge, u.name, inf, u.stability, allegiance_tag, event_suffix
                );
            } else if full_allegiance {
                **text = format!("★ {} [%{:.0} · BİZİM{}]", u.name, inf, event_suffix);
            } else if inf >= 10.0 || is_selected || has_dilemma || is_last_event {
                **text = format!("{}{} [%{:.0}{}]", badge, u.name, inf, event_suffix);
            } else {
                **text = u.name.clone();
            }
            *color = TextColor(if has_dilemma {
                col_crimson
            } else if is_selected || full_allegiance || is_last_event {
                col_gold
            } else if is_hovered || player_dominant {
                col_turquoise
            } else {
                col_primary
            });
        } else if is_selected || is_hovered {
            **text = format!("? Gizemli Bölge #{}", u.id + 1);
            *color = TextColor(col_secondary);
        } else {
            **text = String::new();
        }
    }

    // Update floating tooltip on hovered/inspected inter-universe gateway
    let (ref mut gw_text, ref mut gw_color, ref mut gw_tf, ref mut gw_vis) = *q_gw_label;
    let active_gw = runtime.hovered_gateway.or(runtime.inspected_gateway);
    if let Some((u_a, u_b)) = active_gw {
        let ua = &runtime.sim.universes[u_a as usize];
        let ub = &runtime.sim.universes[u_b as usize];
        let mid = Vec2::new(
            (ua.position[0] + ub.position[0]) * 0.5,
            (ua.position[1] + ub.position[1]) * 0.5 + 14.0,
        );
        gw_tf.translation.x = mid.x;
        gw_tf.translation.y = mid.y;
        let gw_state = ua
            .gateways
            .iter()
            .find(|g| g.target_universe_id == u_b)
            .map(|g| g.state)
            .unwrap_or(GatewayState::Closed);
        let state_str = match gw_state {
            GatewayState::Open => "AÇIK GEÇİT (İnanç & Ruh Akışı — Tıkla: Ayrıntı)",
            GatewayState::Closed => "KAPALI GEÇİT (Tıkla: Ayrıntı)",
            GatewayState::Locked { .. } => "MÜHÜRLÜ GEÇİT (Tıkla: Ayrıntı)",
        };
        ***gw_text = format!("⇄ {state_str}");
        **gw_color = TextColor(col_gold);
        **gw_vis = Visibility::Inherited;
    } else {
        **gw_vis = Visibility::Hidden;
    }
}

fn control_cosmic_camera(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
    mut runtime: ResMut<GameRuntimeState>,
    mut camera_q: Single<(&mut Transform, &mut Projection), With<CosmicMapCamera>>,
) {
    if runtime.show_main_menu {
        return;
    }
    let (ref mut transform, ref mut projection) = *camera_q;
    let dt = time.delta_secs();

    let manual_pan_input = keys.pressed(KeyCode::KeyW)
        || keys.pressed(KeyCode::ArrowUp)
        || keys.pressed(KeyCode::KeyS)
        || keys.pressed(KeyCode::ArrowDown)
        || keys.pressed(KeyCode::KeyA)
        || keys.pressed(KeyCode::ArrowLeft)
        || keys.pressed(KeyCode::KeyD)
        || keys.pressed(KeyCode::ArrowRight)
        || mouse_buttons.pressed(MouseButton::Right)
        || mouse_buttons.pressed(MouseButton::Middle);

    if manual_pan_input {
        runtime.camera_snap_target = None;
    }

    // Smooth camera glide toward target universe or origin
    if let Some(target_pos) = runtime.camera_snap_target {
        let cur = Vec2::new(transform.translation.x, transform.translation.y);
        let diff = target_pos - cur;
        if diff.length_squared() < 1.5 {
            transform.translation.x = target_pos.x;
            transform.translation.y = target_pos.y;
            runtime.camera_snap_target = None;
        } else {
            let alpha = (dt * 7.5).min(1.0);
            let next = cur.lerp(target_pos, alpha);
            transform.translation.x = next.x;
            transform.translation.y = next.y;
        }
    }

    let mut zoom_scale = 1.0f32;
    if let Projection::Orthographic(ortho) = &mut **projection {
        if keys.pressed(KeyCode::Equal) || keys.pressed(KeyCode::NumpadAdd) {
            runtime.camera_zoom_target = None;
            ortho.scale = (ortho.scale * (1.0 - 1.2 * dt)).clamp(0.45, 2.6);
        }
        if keys.pressed(KeyCode::Minus) || keys.pressed(KeyCode::NumpadSubtract) {
            runtime.camera_zoom_target = None;
            ortho.scale = (ortho.scale * (1.0 + 1.2 * dt)).clamp(0.45, 2.6);
        }
        if mouse_scroll.delta.y.abs() > 0.001 {
            let factor = (1.0 - mouse_scroll.delta.y * 0.11).clamp(0.75, 1.3);
            let base = runtime.camera_zoom_target.unwrap_or(ortho.scale);
            runtime.camera_zoom_target = Some((base * factor).clamp(0.45, 2.6));
        }
        if keys.just_pressed(KeyCode::Home) || keys.just_pressed(KeyCode::KeyR) {
            runtime.camera_snap_target = Some(Vec2::ZERO);
            runtime.camera_zoom_target = Some(1.0);
        }
        if let Some(target_z) = runtime.camera_zoom_target {
            let diff = target_z - ortho.scale;
            if diff.abs() < 0.004 {
                ortho.scale = target_z;
                runtime.camera_zoom_target = None;
            } else {
                ortho.scale += diff * (dt * 9.0).min(1.0);
            }
        }
        zoom_scale = ortho.scale;
    }

    let pan_speed = 420.0 * zoom_scale * dt;
    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        transform.translation.y += pan_speed;
    }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        transform.translation.y -= pan_speed;
    }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= pan_speed;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        transform.translation.x += pan_speed;
    }

    if mouse_buttons.pressed(MouseButton::Right) || mouse_buttons.pressed(MouseButton::Middle) {
        transform.translation.x -= mouse_motion.delta.x * zoom_scale;
        transform.translation.y += mouse_motion.delta.y * zoom_scale;
    }

    transform.translation.x = transform.translation.x.clamp(-750.0, 750.0);
    transform.translation.y = transform.translation.y.clamp(-750.0, 750.0);
}

fn point_to_segment_distance(p: Vec2, a: Vec2, b: Vec2) -> f32 {
    let ab = b - a;
    let len_sq = ab.length_squared();
    if len_sq < 1.0e-4 {
        return p.distance(a);
    }
    let proj = ((p - a).dot(ab) / len_sq).clamp(0.05, 0.95);
    let closest = a + ab * proj;
    p.distance(closest)
}

fn pick_universe_node_on_click(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    window_q: Single<&Window>,
    camera_q: Single<(&Camera, &GlobalTransform), With<CosmicMapCamera>>,
    mut runtime: ResMut<GameRuntimeState>,
) {
    if runtime.show_main_menu {
        runtime.hovered_universe_id = None;
        runtime.hovered_gateway = None;
        return;
    }
    let window = *window_q;
    let Some(cursor_pos) = window.cursor_position() else {
        runtime.hovered_universe_id = None;
        runtime.hovered_gateway = None;
        return;
    };

    // Ignore cursor inside the Top Bar, Left Panel, Right Panel, or Bottom Event Log
    let win_w = window.width();
    let win_h = window.height();
    let left_panel_w = 230.0;
    let right_panel_w = 370.0;
    let top_bar_h = 56.0;
    let bottom_bar_h = 98.0;

    if cursor_pos.x < left_panel_w
        || cursor_pos.x > (win_w - right_panel_w)
        || cursor_pos.y < top_bar_h
        || cursor_pos.y > (win_h - bottom_bar_h)
    {
        runtime.hovered_universe_id = None;
        runtime.hovered_gateway = None;
        return;
    }

    let (camera, cam_transform) = *camera_q;
    let Ok(world_pos) = camera.viewport_to_world_2d(cam_transform, cursor_pos) else {
        runtime.hovered_universe_id = None;
        runtime.hovered_gateway = None;
        return;
    };

    // 1. Find closest universe node within hover/pick radius
    let mut best_match: Option<(u32, f32)> = None;
    for u in &runtime.sim.universes {
        if u.is_destroyed {
            continue;
        }
        if let Some(ft) = runtime.type_filter {
            if u.universe_type != ft {
                continue;
            }
        }
        let u_pos = Vec2::new(u.position[0], u.position[1]);
        let dist = world_pos.distance(u_pos);
        if dist <= 24.0 && best_match.is_none_or(|(_, best_d)| dist < best_d) {
            best_match = Some((u.id, dist));
        }
    }

    runtime.hovered_universe_id = best_match.map(|(id, _)| id);

    // 2. If not hovering directly over a universe node, check inter-universe gateway lines & flowing animations
    let mut best_gateway: Option<((u32, u32), f32)> = None;
    if best_match.is_none() {
        for u in &runtime.sim.universes {
            if u.is_destroyed {
                continue;
            }
            let pos_a = Vec2::new(u.position[0], u.position[1]);
            for gw in &u.gateways {
                if gw.target_universe_id <= u.id {
                    continue;
                }
                let target = &runtime.sim.universes[gw.target_universe_id as usize];
                if target.is_destroyed {
                    continue;
                }
                if !u.discovered_by_player && !target.discovered_by_player {
                    continue;
                }
                let pos_b = Vec2::new(target.position[0], target.position[1]);
                let seg_dist = point_to_segment_distance(world_pos, pos_a, pos_b);
                if seg_dist <= 11.5 && best_gateway.is_none_or(|(_, best_d)| seg_dist < best_d) {
                    best_gateway = Some(((u.id, target.id), seg_dist));
                }
            }
        }
    }
    runtime.hovered_gateway = best_gateway.map(|(pair, _)| pair);

    if !mouse_buttons.just_pressed(MouseButton::Left) {
        return;
    }

    if let Some((picked_id, _)) = best_match {
        let already_selected =
            runtime.selected_universe_id == picked_id && runtime.inspected_gateway.is_none();
        runtime.selected_universe_id = picked_id;
        runtime.inspected_gateway = None;
        runtime.spawn_universe_vfx(picked_id, MapVfxKind::SelectionPulse, 0.85);

        let (u_pos, status_msg) = {
            let u = &runtime.sim.universes[picked_id as usize];
            let pos = Vec2::new(u.position[0], u.position[1]);
            let msg = if u.discovered_by_player {
                let mut badge_note = String::new();
                if u.has_full_allegiance(PLAYER_DEITY_ID) {
                    badge_note.push_str(" · ★ TAM AİDİYET (Altın Taç & Çift Mühür)");
                } else if u.is_dominated_by(PLAYER_DEITY_ID) {
                    badge_note.push_str(" · ◆ Baskın İlah Sensin");
                }
                if runtime.sim.focused_universe_id == Some(u.id) {
                    badge_note.push_str(" · ◎ İlahi Odak Yörüngesi (+%25 Akış)");
                }
                format!(
                    "{} seçildi ({} · İstikrar: %{:.0}{})",
                    u.name,
                    u.universe_type.name_tr(),
                    u.stability,
                    badge_note
                )
            } else {
                format!(
                    "Keşfedilmemiş Kozmik Bölge (#{}). Bağlantılı bir evrenden Geçit Aç veya Keşif yap.",
                    u.id + 1
                )
            };
            (pos, msg)
        };
        if already_selected {
            // Clicking the already selected node smoothly glides the camera to center on it
            runtime.camera_snap_target = Some(u_pos);
        }
        runtime.play_sfx(CosmicSfxKind::UniverseSelect);
        runtime.status_message = status_msg;
    } else if let Some(((u_a, u_b), _)) = best_gateway {
        runtime.inspected_gateway = Some((u_a, u_b));
        runtime.active_tab = InspectorTab::Universe;
        runtime.play_sfx(CosmicSfxKind::GatewayPulse);
        let msg = {
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
            match gw_state {
                GatewayState::Open => format!(
                    "⇄ KOZMİK GEÇİT SEÇİLDİ ({name_a} ↔ {name_b}): Çizgi üzerindeki akan küreler iki evren arasındaki İnanç Yayılımı ve Ruh/Enerji akışını gösterir. Sağ panelde ayrıntıları incele."
                ),
                GatewayState::Closed => format!(
                    "⇄ KAPALI GEÇİT SEÇİLDİ ({name_a} ↔ {name_b}): Bağlantı şu an kapalı; inanç veya ruh geçişi olmuyor. 'Geçit Aç' ile aktifleşebilir."
                ),
                GatewayState::Locked { .. } => format!(
                    "⇄ MÜHÜRLÜ GEÇİT SEÇİLDİ ({name_a} ↔ {name_b}): Elmas mühür bu geçidin kilitlendiğini gösterir."
                ),
            }
        };
        runtime.status_message = msg;
    }
}

fn theme_map_palette(theme_idx: usize) -> (Color, Color, Color, Color, Color) {
    match theme_idx % 4 {
        // Tema 1: Safir Mor & İlahi Kehribar
        0 => (
            Color::srgb_u8(0x28, 0x1D, 0x42),
            Color::srgb_u8(0xC4, 0x6B, 0xFF),
            Color::srgb_u8(0xFF, 0xB8, 0x4D),
            Color::srgb_u8(0x2E, 0xE5, 0xB5),
            Color::srgb_u8(0xFF, 0x4F, 0x79),
        ),
        // Tema 2: Zümrüt Yeşim & Yıldız Tozu
        1 => (
            Color::srgb_u8(0x14, 0x32, 0x38),
            Color::srgb_u8(0x63, 0xD2, 0xFF),
            Color::srgb_u8(0x2E, 0xE5, 0xB5),
            Color::srgb_u8(0x8C, 0xFF, 0xDB),
            Color::srgb_u8(0xFF, 0x6B, 0x5B),
        ),
        // Tema 3: Kızıl Kehribar & Süpernova
        2 => (
            Color::srgb_u8(0x38, 0x16, 0x27),
            Color::srgb_u8(0xFF, 0x5D, 0x8F),
            Color::srgb_u8(0xFF, 0xC8, 0x57),
            Color::srgb_u8(0xFF, 0x9E, 0x6D),
            Color::srgb_u8(0xFF, 0x33, 0x55),
        ),
        // Tema 4: Ebedi Mermer & Altın Usturlap
        _ => (
            Color::srgb_u8(0x24, 0x2C, 0x42),
            Color::srgb_u8(0x8C, 0xE0, 0xFF),
            Color::srgb_u8(0xF4, 0xD0, 0x6F),
            Color::srgb_u8(0x5E, 0xE2, 0xC6),
            Color::srgb_u8(0xE8, 0x48, 0x55),
        ),
    }
}

fn draw_cosmic_map_gizmos(mut gizmos: Gizmos, runtime: Res<GameRuntimeState>) {
    // Use `runtime.anim_time` so ALL map animations genuinely STOP when the game is paused!
    let t = runtime.anim_time;

    // 4-Theme Atmosphere Palette (Arayüz.md Section 2)
    let (col_panel_line, col_purple, col_gold, col_turquoise, col_crimson) =
        theme_map_palette(runtime.ui_theme_index);
    let col_text_primary = Color::srgb_u8(0xF8, 0xF3, 0xE6);
    let col_text_secondary = Color::srgb_u8(0xB9, 0xAD, 0xD2);

    // 1. Concentric cosmic ring guides and a deterministic, softly twinkling starfield.
    for &radius in &[130.0f32, 265.0, 405.0, 545.0] {
        gizmos
            .circle_2d(
                Isometry2d::IDENTITY,
                radius,
                col_panel_line.with_alpha(0.27),
            )
            .resolution(96);
    }

    // Integer hashing keeps the star positions stable across frames and save replays.
    for i in 0..220u32 {
        let mut hash = i.wrapping_add(0x9E37_79B9);
        hash ^= hash >> 16;
        hash = hash.wrapping_mul(0x7FEB_352D);
        hash ^= hash >> 15;
        hash = hash.wrapping_mul(0x846C_A68B);
        hash ^= hash >> 16;
        let angle = (hash as f32 / u32::MAX as f32) * TAU;
        let radius_hash = hash.rotate_left(13);
        let radius = 34.0 + (radius_hash % 625) as f32;
        let star_pos = Vec2::new(angle.cos() * radius, angle.sin() * radius);
        let size = 0.45 + ((hash >> 8) % 100) as f32 / 100.0;
        let phase = ((hash >> 16) % 628) as f32 / 100.0;
        let twinkle = 0.72 + (t * 0.7 + phase).sin().abs() * 0.28;
        let star_color = match i % 17 {
            0 => col_gold,
            1..=3 => col_turquoise,
            _ => col_text_secondary,
        };
        gizmos.circle_2d(
            Isometry2d::from_translation(star_pos),
            size,
            star_color.with_alpha((0.16 + size * 0.22) * twinkle),
        );

        // Sparse four-point glints add a little scale without turning the map noisy.
        if i % 29 == 0 {
            let glint = 2.0 + size;
            let glint_color = star_color.with_alpha(0.28 * twinkle);
            gizmos.line_2d(
                star_pos + Vec2::new(-glint, 0.0),
                star_pos + Vec2::new(glint, 0.0),
                glint_color,
            );
            gizmos.line_2d(
                star_pos + Vec2::new(0.0, -glint),
                star_pos + Vec2::new(0.0, glint),
                glint_color,
            );
        }
    }

    // 2. Draw Gateways (Evrenler arası bağlantılar ve akan enerji küreleri)
    for u in &runtime.sim.universes {
        if u.is_destroyed {
            continue;
        }
        let pos_a = Vec2::new(u.position[0], u.position[1]);
        for gw in &u.gateways {
            if gw.target_universe_id <= u.id {
                continue;
            }
            let target = &runtime.sim.universes[gw.target_universe_id as usize];
            if target.is_destroyed {
                continue;
            }
            let pos_b = Vec2::new(target.position[0], target.position[1]);
            let both_hidden = !u.discovered_by_player && !target.discovered_by_player;

            if both_hidden {
                gizmos.line_2d(pos_a, pos_b, col_panel_line.with_alpha(0.25));
                continue;
            }

            let pair = (u.id, target.id);
            let is_inspected_gw = runtime.inspected_gateway == Some(pair);
            let is_hovered_gw = runtime.hovered_gateway == Some(pair);
            let is_highlighted_edge = is_inspected_gw
                || is_hovered_gw
                || u.id == runtime.selected_universe_id
                || target.id == runtime.selected_universe_id
                || runtime.hovered_universe_id == Some(u.id)
                || runtime.hovered_universe_id == Some(target.id);

            let mid = (pos_a + pos_b) * 0.5;

            match gw.state {
                GatewayState::Open => {
                    let line_col = if is_inspected_gw || is_hovered_gw {
                        col_gold
                    } else if is_highlighted_edge {
                        col_gold.with_alpha(0.85)
                    } else if u.discovered_by_player && target.discovered_by_player {
                        col_turquoise.with_alpha(0.55)
                    } else {
                        col_purple.with_alpha(0.35)
                    };
                    gizmos.line_2d(pos_a, pos_b, line_col);

                    // A short train of energy motes makes gateway direction easier to read.
                    // `runtime.anim_time` keeps the flow frozen while simulation is paused.
                    let speed = if is_highlighted_edge { 0.65 } else { 0.35 };
                    let mote_count = if is_highlighted_edge { 4 } else { 3 };
                    for mote_idx in 0..mote_count {
                        let phase = (t * speed
                            + (u.id as f32) * 0.17
                            + mote_idx as f32 / mote_count as f32)
                            .fract();
                        let flow_pos = pos_a.lerp(pos_b, phase);
                        let is_leading_mote = mote_idx == 0;
                        let mote_color = if mote_idx % 2 == 0 {
                            col_gold
                        } else {
                            col_turquoise
                        };
                        let mote_radius = if is_inspected_gw || is_hovered_gw {
                            if is_leading_mote {
                                3.8
                            } else {
                                2.4
                            }
                        } else if is_highlighted_edge {
                            if is_leading_mote {
                                3.1
                            } else {
                                2.0
                            }
                        } else if is_leading_mote {
                            2.3
                        } else {
                            1.5
                        };
                        gizmos.circle_2d(
                            Isometry2d::from_translation(flow_pos),
                            mote_radius,
                            mote_color.with_alpha(if is_leading_mote { 0.9 } else { 0.54 }),
                        );
                    }
                    if is_highlighted_edge {
                        let phase2 = (t * speed + (u.id as f32) * 0.17 + 0.5).fract();
                        let flow_pos2 = pos_b.lerp(pos_a, phase2);
                        gizmos.circle_2d(
                            Isometry2d::from_translation(flow_pos2),
                            2.2,
                            col_turquoise.with_alpha(0.88),
                        );
                    }
                }
                GatewayState::Closed => {
                    let col_closed = if is_inspected_gw || is_hovered_gw {
                        col_gold.with_alpha(0.85)
                    } else if is_highlighted_edge {
                        col_purple.with_alpha(0.55)
                    } else {
                        col_text_secondary.with_alpha(0.22)
                    };
                    gizmos.line_2d(pos_a, pos_a.lerp(mid, 0.72), col_closed);
                    gizmos.line_2d(pos_b, pos_b.lerp(mid, 0.72), col_closed);
                }
                GatewayState::Locked { .. } => {
                    gizmos.line_2d(pos_a, pos_b, col_gold.with_alpha(0.5));
                    draw_diamond(&mut gizmos, mid, 5.0, col_gold);
                }
            }

            // Highlight ring on hovered or clicked inter-universe gateway
            if is_inspected_gw || is_hovered_gw {
                gizmos
                    .circle_2d(
                        Isometry2d::from_translation(mid),
                        7.5,
                        if is_inspected_gw {
                            col_gold
                        } else {
                            col_turquoise
                        },
                    )
                    .resolution(28);
            }
        }
    }

    // 3. Draw Universe Nodes (60 Universes with TASARIM.md geometric shape language)
    let pulse = (t * 2.4).sin() * 0.5 + 0.5;

    // Precompute each active deity's primary Stronghold Capital universe (Faz 2.3.3)
    let mut deity_capitals: Vec<(u32, u32)> = Vec::new();
    for d in &runtime.sim.deities {
        if d.is_sealed {
            continue;
        }
        if let Some(cap_u) = runtime
            .sim
            .universes
            .iter()
            .filter(|u| !u.is_destroyed && u.discovered_by_player && u.is_dominated_by(d.id))
            .max_by(|a, b| {
                a.influence_of(d.id)
                    .partial_cmp(&b.influence_of(d.id))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
        {
            deity_capitals.push((cap_u.id, d.id));
        }
    }

    for u in &runtime.sim.universes {
        if u.is_destroyed {
            continue;
        }
        let filtered_out = runtime.type_filter.is_some_and(|ft| ft != u.universe_type);
        let alpha_mult = if filtered_out { 0.18 } else { 1.0 };

        let pos = Vec2::new(u.position[0], u.position[1]);
        let is_selected = u.id == runtime.selected_universe_id;
        let is_hovered = runtime.hovered_universe_id == Some(u.id);

        // Layered, low-alpha aura separates each node from the starfield. Ownership
        // takes precedence; neutral worlds use their universe-type accent instead.
        let aura_color = u.dominant_deity().map_or_else(
            || match u.universe_type {
                UniverseType::Life | UniverseType::Tech => col_turquoise,
                UniverseType::Faith | UniverseType::Primordial => col_gold,
                UniverseType::Chaos | UniverseType::Collapsing => col_crimson,
            },
            |(deity_id, _)| deity_color(deity_id),
        );
        let aura_emphasis = if is_selected {
            1.0
        } else if is_hovered {
            0.8
        } else {
            0.55
        };
        if u.discovered_by_player {
            for (radius, alpha) in [(17.0 + pulse, 0.14), (21.0 + pulse * 1.5, 0.075)] {
                gizmos
                    .circle_2d(
                        Isometry2d::from_translation(pos),
                        radius,
                        aura_color.with_alpha(alpha * aura_emphasis * alpha_mult),
                    )
                    .resolution(40);
            }
        }

        if !u.discovered_by_player {
            // Undiscovered universe: mysterious low-detail circle
            let dim_col = col_text_secondary.with_alpha(0.32 * alpha_mult);
            gizmos.circle_2d(Isometry2d::from_translation(pos), 9.0, dim_col);
            gizmos.circle_2d(Isometry2d::from_translation(pos), 2.5, dim_col);
            if is_hovered {
                gizmos.circle_2d(
                    Isometry2d::from_translation(pos),
                    13.5 + pulse * 1.5,
                    col_turquoise.with_alpha(0.75),
                );
            }
            if is_selected {
                gizmos.circle_2d(
                    Isometry2d::from_translation(pos),
                    16.0 + pulse * 2.5,
                    col_gold,
                );
            }
            continue;
        }

        // Dominant Deity Influence Halo (TASARIM.md Section 5)
        if let Some((dom_id, dom_inf)) = u.dominant_deity() {
            let deity_col = deity_color(dom_id).with_alpha(0.65 * alpha_mult);
            let halo_radius = 15.0 + (dom_inf / 100.0) * 6.5;
            gizmos
                .circle_2d(Isometry2d::from_translation(pos), halo_radius, deity_col)
                .resolution(36);
        }

        // Faz 2.3.2: Four-Layer Map Overlay Mode (Katman Modu — Kısayol: M)
        match runtime.map_overlay_mode {
            MapOverlayMode::StandardCosmos => {}
            MapOverlayMode::DeityDominionBlocks => {
                if let Some((dom_id, dom_inf)) = u.dominant_deity() {
                    let block_col = deity_color(dom_id).with_alpha(0.92 * alpha_mult);
                    let block_r = 22.0 + (dom_inf / 100.0) * 7.5;
                    draw_regular_polygon(&mut gizmos, pos, block_r, 6, t * 0.12, block_col);
                }
            }
            MapOverlayMode::EconomyAndReservoir => {
                let res_ratio = (u.soul_reservoir.clamp(0, 600) as f32) / 600.0;
                let econ_r = 18.5 + res_ratio * 11.5;
                gizmos
                    .circle_2d(
                        Isometry2d::from_translation(pos),
                        econ_r,
                        col_gold.with_alpha((0.55 + res_ratio * 0.4) * alpha_mult),
                    )
                    .resolution(40);
                if u.essence_richness >= 1.35 {
                    draw_diamond(
                        &mut gizmos,
                        pos + Vec2::new(0.0, -21.0),
                        4.2,
                        col_turquoise.with_alpha(alpha_mult),
                    );
                }
            }
            MapOverlayMode::StabilityHeatmap => {
                let heat_col = if u.stability >= 75.0 && u.danger_level < 35.0 {
                    col_turquoise.with_alpha(0.92 * alpha_mult)
                } else if u.stability < 40.0 || u.danger_level >= 60.0 {
                    col_crimson.with_alpha(0.95 * alpha_mult)
                } else {
                    col_gold.with_alpha(0.88 * alpha_mult)
                };
                let heat_r = 21.5 + pulse * 2.2;
                gizmos
                    .circle_2d(Isometry2d::from_translation(pos), heat_r, heat_col)
                    .resolution(44);
                gizmos
                    .circle_2d(
                        Isometry2d::from_translation(pos),
                        heat_r + 2.5,
                        heat_col.with_alpha(0.45 * alpha_mult),
                    )
                    .resolution(44);
            }
        }

        // Faz 2.3.3: Tanrısal Mühür Geometrisi (Deity Capital Stronghold Seal)
        if let Some(&(_, cap_deity_id)) = deity_capitals.iter().find(|&&(uid, _)| uid == u.id) {
            let seal_col = deity_color(cap_deity_id).with_alpha(alpha_mult);
            draw_deity_capital_seal(
                &mut gizmos,
                pos + Vec2::new(0.0, 27.5),
                cap_deity_id,
                t,
                seal_col,
                col_gold.with_alpha(alpha_mult),
            );
        }

        // TAM AİDİYET BELİRTECİ (Full Player Allegiance Indicator: Double Covenant Ring + Golden Crown)
        if u.has_full_allegiance(PLAYER_DEITY_ID) {
            let cov_r = 17.2;
            gizmos
                .circle_2d(
                    Isometry2d::from_translation(pos),
                    cov_r,
                    col_gold.with_alpha(0.92 * alpha_mult),
                )
                .resolution(48);
            for k in 0..4 {
                let a = (k as f32) * (TAU / 4.0) + t * 0.25;
                let stud = pos + Vec2::new(a.cos() * cov_r, a.sin() * cov_r);
                draw_diamond(&mut gizmos, stud, 2.3, col_gold.with_alpha(alpha_mult));
            }
            draw_allegiance_crown(
                &mut gizmos,
                pos + Vec2::new(0.0, 22.0),
                col_gold.with_alpha(alpha_mult),
                col_purple.with_alpha(alpha_mult),
            );
        } else if u.is_dominated_by(PLAYER_DEITY_ID) {
            // Partial dominance indicator (single diamond above node)
            draw_diamond(
                &mut gizmos,
                pos + Vec2::new(0.0, 20.5),
                3.5,
                col_purple.with_alpha(alpha_mult),
            );
        }

        // Universe Category Geometric Core & Aura (Arayüz.md Section 5.1)
        match u.universe_type {
            UniverseType::Life => {
                // Çift katmanlı parlayan yıldız çekirdeği ve hızlı dönen yaşam halkası
                let c = col_turquoise.with_alpha(alpha_mult);
                gizmos.circle_2d(Isometry2d::from_translation(pos), 12.0, c);
                gizmos.circle_2d(
                    Isometry2d::from_translation(pos),
                    6.5,
                    col_gold.with_alpha(alpha_mult),
                );
                gizmos.circle_2d(
                    Isometry2d::from_translation(pos),
                    2.6 + pulse * 0.8,
                    col_text_primary.with_alpha(alpha_mult),
                );
                for k in 0..4 {
                    let a = (k as f32) * (TAU / 4.0) + t * 0.45;
                    let bud = pos + Vec2::new(a.cos() * 12.0, a.sin() * 12.0);
                    gizmos.circle_2d(Isometry2d::from_translation(bud), 2.3, c);
                }
            }
            UniverseType::Chaos => {
                // Düzensiz frekansta titreşen prizmatik halka ve mor/kızıl enerji kıvılcımları
                let c1 = col_crimson.with_alpha(alpha_mult);
                let c2 = col_purple.with_alpha(alpha_mult);
                let jitter = (t * 5.2 + u.id as f32).sin() * 1.1;
                draw_regular_polygon(&mut gizmos, pos, 13.0 + jitter, 3, t * 0.65, c1);
                draw_regular_polygon(&mut gizmos, pos, 11.5 - jitter * 0.6, 3, -t * 0.55 + PI, c2);
                gizmos.circle_2d(
                    Isometry2d::from_translation(pos),
                    3.2,
                    col_gold.with_alpha(0.85 * alpha_mult),
                );
            }
            UniverseType::Faith => {
                // Çokgen bilgelik/inanç mührü ve salınan altın-mor hale
                let c = col_gold.with_alpha(alpha_mult);
                gizmos.circle_2d(Isometry2d::from_translation(pos), 12.5, c);
                draw_regular_polygon(
                    &mut gizmos,
                    pos,
                    8.5,
                    8,
                    t * 0.18,
                    col_purple.with_alpha(alpha_mult),
                );
                for k in 0..4 {
                    let a = (k as f32) * FRAC_PI_4 + t * 0.15;
                    let d = Vec2::new(a.cos() * 14.2, a.sin() * 14.2);
                    gizmos.line_2d(pos - d, pos + d, c.with_alpha(0.68 * alpha_mult));
                }
            }
            UniverseType::Tech => {
                // Çift yönlü dönen altıgen usturlap çekirdeği
                let c = col_turquoise.with_alpha(alpha_mult);
                draw_regular_polygon(&mut gizmos, pos, 13.0, 6, FRAC_PI_6 + t * 0.12, c);
                draw_regular_polygon(
                    &mut gizmos,
                    pos,
                    7.4,
                    6,
                    FRAC_PI_6 - t * 0.28,
                    col_text_primary.with_alpha(0.8 * alpha_mult),
                );
                gizmos.circle_2d(
                    Isometry2d::from_translation(pos),
                    2.5,
                    col_gold.with_alpha(alpha_mult),
                );
            }
            UniverseType::Primordial => {
                // Kadim çokgen mühür ve nefes alan elmas çekirdek
                let c = col_text_secondary.with_alpha(alpha_mult);
                gizmos.circle_2d(Isometry2d::from_translation(pos), 12.2, c);
                draw_regular_polygon(
                    &mut gizmos,
                    pos,
                    10.2,
                    6,
                    -t * 0.14,
                    col_purple.with_alpha(0.75 * alpha_mult),
                );
                draw_diamond(
                    &mut gizmos,
                    pos,
                    8.0 + pulse * 1.3,
                    col_gold.with_alpha(alpha_mult),
                );
            }
            UniverseType::Collapsing => {
                // İçe doğru büzülen koyu kızıl girdap halkası
                let c = col_crimson.with_alpha(alpha_mult);
                let shrink = 1.0 - pulse * 0.16;
                draw_regular_polygon(&mut gizmos, pos, 12.8 * shrink, 5, t * 0.45, c);
                draw_regular_polygon(
                    &mut gizmos,
                    pos,
                    7.2 * shrink,
                    3,
                    -t * 0.8 + PI * 0.5,
                    col_purple.with_alpha(alpha_mult),
                );
            }
        }

        // Uygarlık Yörünge Uyduları (Arayüz.md Section 4: Civilization Orbital Satellites)
        let civ_count = u.civilizations.len();
        for (c_idx, civ) in u.civilizations.iter().enumerate() {
            let orbit_r = 15.6 + (c_idx as f32) * 2.1;
            let speed_dir = if c_idx % 2 == 0 { 0.55 } else { -0.45 };
            let angle =
                t * speed_dir + (c_idx as f32) * (TAU / (civ_count.max(1) as f32)) + u.id as f32;
            let sat_pos = pos + Vec2::new(angle.cos() * orbit_r, angle.sin() * orbit_r);
            gizmos
                .circle_2d(
                    Isometry2d::from_translation(pos),
                    orbit_r,
                    col_text_secondary.with_alpha(0.11 * alpha_mult),
                )
                .resolution(32);
            let sat_col = if civ.in_rebellion {
                col_crimson.with_alpha(alpha_mult)
            } else if civ.divine_shield_active {
                col_turquoise.with_alpha(alpha_mult)
            } else {
                col_text_primary.with_alpha(0.88 * alpha_mult)
            };
            gizmos.circle_2d(Isometry2d::from_translation(sat_pos), 1.9, sat_col);
            if civ.divine_shield_active {
                gizmos.circle_2d(
                    Isometry2d::from_translation(sat_pos),
                    3.4,
                    col_turquoise.with_alpha(0.55 * alpha_mult),
                );
            }
        }

        // Danger / Collapse Warning Indicator (TASARIM.md Section 6: shape + color)
        if (u.danger_level >= 65.0 || u.stability <= 30.0) && runtime.high_contrast_symbols {
            let warn_pos = pos + Vec2::new(-16.0, 16.0 + pulse * 1.2);
            draw_regular_polygon(
                &mut gizmos,
                warn_pos,
                4.8 + pulse * 0.8,
                3,
                PI * 0.5,
                col_crimson.with_alpha(alpha_mult),
            );
        }

        // Focused Universe Orbital Ring (taslak.md Section 4)
        if runtime.sim.focused_universe_id == Some(u.id) {
            let focus_r = 25.5;
            gizmos
                .circle_2d(
                    Isometry2d::from_translation(pos),
                    focus_r,
                    col_purple.with_alpha(0.8 * alpha_mult),
                )
                .resolution(48);
            for k in 0..2 {
                let orbit_angle = t * 1.4 + (k as f32) * PI;
                let sat_pos =
                    pos + Vec2::new(orbit_angle.cos() * focus_r, orbit_angle.sin() * focus_r);
                gizmos.circle_2d(
                    Isometry2d::from_translation(sat_pos),
                    2.8,
                    col_turquoise.with_alpha(alpha_mult),
                );
            }
        }

        // Active Cosmic Dilemma or Recent Random Event Beacon
        let has_dilemma = runtime
            .sim
            .active_dilemma
            .as_ref()
            .is_some_and(|d| d.universe_id == u.id);
        let is_last_event = runtime.sim.last_event_universe_id == Some(u.id);
        if has_dilemma || is_last_event {
            let d_pos = pos + Vec2::new(16.0, 16.0);
            let beacon_col = if has_dilemma { col_crimson } else { col_gold };
            draw_diamond(&mut gizmos, d_pos, 5.2 + pulse * 1.8, beacon_col);
            gizmos
                .circle_2d(
                    Isometry2d::from_translation(d_pos),
                    8.0 + pulse * 3.0,
                    beacon_col.with_alpha(0.65 * alpha_mult),
                )
                .resolution(24);
        }

        // Interactive Hover Reticle (TASARIM.md Section 3.1: Halkalar ve ince çizgiler)
        if is_hovered && !is_selected {
            let hov_r = 18.5 + pulse * 1.5;
            gizmos
                .circle_2d(
                    Isometry2d::from_translation(pos),
                    hov_r,
                    col_turquoise.with_alpha(0.85),
                )
                .resolution(40);
            for k in 0..4 {
                let a = (k as f32) * (TAU / 4.0) + t * 0.8;
                let p_in = pos + Vec2::new(a.cos() * (hov_r - 2.5), a.sin() * (hov_r - 2.5));
                let p_out = pos + Vec2::new(a.cos() * (hov_r + 3.5), a.sin() * (hov_r + 3.5));
                gizmos.line_2d(p_in, p_out, col_gold.with_alpha(0.9));
            }
        }

        // Selected Universe Highlight Ring
        if is_selected {
            let sel_r = 20.0 + pulse * 3.0;
            gizmos
                .circle_2d(Isometry2d::from_translation(pos), sel_r, col_gold)
                .resolution(48);
            gizmos
                .circle_2d(
                    Isometry2d::from_translation(pos),
                    sel_r + 3.0,
                    col_text_primary.with_alpha(0.7),
                )
                .resolution(48);
            for mark in 0..8 {
                let angle = mark as f32 * (TAU / 8.0);
                let direction = Vec2::new(angle.cos(), angle.sin());
                gizmos.line_2d(
                    pos + direction * (sel_r + 4.0),
                    pos + direction * (sel_r + 7.0),
                    col_gold.with_alpha(0.7),
                );
            }
        }
    }
}

/// Draws a geometric 3-peaked Divine Crown emblem above universes with Full Player Allegiance (`Tam Aidiyet`).
fn draw_allegiance_crown(gizmos: &mut Gizmos, center: Vec2, col_gold: Color, col_purple: Color) {
    let base_l = center + Vec2::new(-5.5, -2.5);
    let base_r = center + Vec2::new(5.5, -2.5);
    let peak_l = center + Vec2::new(-6.5, 3.0);
    let valley_l = center + Vec2::new(-2.5, -0.5);
    let peak_m = center + Vec2::new(0.0, 5.2);
    let valley_r = center + Vec2::new(2.5, -0.5);
    let peak_r = center + Vec2::new(6.5, 3.0);

    gizmos.line_2d(base_l, base_r, col_gold);
    gizmos.line_2d(base_l, peak_l, col_gold);
    gizmos.line_2d(peak_l, valley_l, col_gold);
    gizmos.line_2d(valley_l, peak_m, col_gold);
    gizmos.line_2d(peak_m, valley_r, col_gold);
    gizmos.line_2d(valley_r, peak_r, col_gold);
    gizmos.line_2d(peak_r, base_r, col_gold);
    gizmos.circle_2d(Isometry2d::from_translation(peak_m), 1.5, col_purple);
}

/// Faz 2.3.3: Draws a distinct geometric Deity Stronghold Seal above each deity's primary capital universe.
fn draw_deity_capital_seal(
    gizmos: &mut Gizmos,
    center: Vec2,
    deity_id: u32,
    t: f32,
    seal_col: Color,
    accent_col: Color,
) {
    let sides = 3 + ((deity_id as usize) % 5);
    draw_regular_polygon(gizmos, center, 5.8, sides, t * 0.35, seal_col);
    draw_diamond(gizmos, center, 2.8, accent_col);
}

fn update_and_draw_map_vfx(
    mut gizmos: Gizmos,
    time: Res<Time>,
    mut runtime: ResMut<GameRuntimeState>,
) {
    // Freeze active map VFX progression when the game is paused!
    let is_paused = runtime.speed.anim_multiplier() == 0.0;
    let dt = if is_paused { 0.0 } else { time.delta_secs() };
    let col_purple = Color::srgb_u8(0xC4, 0x6B, 0xFF);
    let col_gold = Color::srgb_u8(0xFF, 0xB8, 0x4D);
    let col_turquoise = Color::srgb_u8(0x2E, 0xE5, 0xB5);
    let col_crimson = Color::srgb_u8(0xFF, 0x4F, 0x79);
    let col_white = Color::srgb_u8(0xF8, 0xF3, 0xE6);

    runtime.active_vfx.retain_mut(|vfx| {
        // Selection pulse can still complete when clicking while paused, all other cosmos VFX pause
        let step_dt = if matches!(vfx.kind, MapVfxKind::SelectionPulse) {
            time.delta_secs()
        } else {
            dt
        };
        vfx.elapsed += step_dt;
        if vfx.elapsed >= vfx.duration {
            return false;
        }
        let progress = (vfx.elapsed / vfx.duration).clamp(0.0, 1.0);
        // Smooth ease-out curve: 1 - (1 - p)^2
        let ease_out = 1.0 - (1.0 - progress) * (1.0 - progress);
        let fade = 1.0 - progress;
        let center = vfx.origin;

        match vfx.kind {
            MapVfxKind::SelectionPulse => {
                let r1 = 14.0 + ease_out * 28.0;
                let r2 = 10.0 + ease_out * 18.0;
                gizmos
                    .circle_2d(
                        Isometry2d::from_translation(center),
                        r1,
                        col_gold.with_alpha(fade * 0.85),
                    )
                    .resolution(48);
                gizmos
                    .circle_2d(
                        Isometry2d::from_translation(center),
                        r2,
                        col_white.with_alpha(fade * 0.65),
                    )
                    .resolution(36);
            }
            MapVfxKind::ProphecyRings => {
                // Concentric faith rings + 8 radial rays (TASARIM.md 4.3)
                for ring_idx in 0..3 {
                    let ring_p = (progress - (ring_idx as f32) * 0.16).clamp(0.0, 1.0);
                    if ring_p > 0.0 {
                        let r = 12.0 + ring_p * 42.0;
                        let c = if ring_idx % 2 == 0 {
                            col_gold.with_alpha((1.0 - ring_p) * 0.9)
                        } else {
                            col_purple.with_alpha((1.0 - ring_p) * 0.85)
                        };
                        gizmos
                            .circle_2d(Isometry2d::from_translation(center), r, c)
                            .resolution(48);
                    }
                }
                for ray in 0..8 {
                    let angle = (ray as f32) * (TAU / 8.0) + progress * 0.6;
                    let r_in = 14.0 + ease_out * 10.0;
                    let r_out = 20.0 + ease_out * 28.0;
                    let p1 = center + Vec2::new(angle.cos() * r_in, angle.sin() * r_in);
                    let p2 = center + Vec2::new(angle.cos() * r_out, angle.sin() * r_out);
                    gizmos.line_2d(p1, p2, col_gold.with_alpha(fade * 0.8));
                }
            }
            MapVfxKind::SoulHarvestSwirl => {
                // Orbiting soul point clusters converging inward (TASARIM.md 3.1: Nokta kümeleri)
                let radius = 44.0 * (1.0 - ease_out * 0.82);
                for k in 0..10 {
                    let angle = (k as f32) * (TAU / 10.0) + progress * TAU * 1.4;
                    let pt = center + Vec2::new(angle.cos() * radius, angle.sin() * radius);
                    let col = if k % 2 == 0 {
                        col_purple.with_alpha(fade * 0.95)
                    } else {
                        col_turquoise.with_alpha(fade * 0.95)
                    };
                    gizmos.circle_2d(Isometry2d::from_translation(pt), 2.8, col);
                }
                gizmos
                    .circle_2d(
                        Isometry2d::from_translation(center),
                        8.0 + (progress * PI).sin() * 10.0,
                        col_purple.with_alpha(fade * 0.85),
                    )
                    .resolution(36);
            }
            MapVfxKind::HexagonOrder => {
                // Counter-rotating hexagons of stability & creation (TASARIM.md 3.1: Altıgenler)
                let r_outer = 14.0 + ease_out * 34.0;
                let r_inner = 10.0 + ease_out * 22.0;
                draw_regular_polygon(
                    &mut gizmos,
                    center,
                    r_outer,
                    6,
                    progress * 1.2,
                    col_turquoise.with_alpha(fade * 0.9),
                );
                draw_regular_polygon(
                    &mut gizmos,
                    center,
                    r_inner,
                    6,
                    -progress * 1.2 + FRAC_PI_6,
                    col_gold.with_alpha(fade * 0.85),
                );
            }
            MapVfxKind::DivineStrike => {
                // Sharp triangles for power, rebellion & smite (TASARIM.md 3.1: Üçgenler)
                let r1 = 12.0 + ease_out * 36.0;
                draw_regular_polygon(
                    &mut gizmos,
                    center,
                    r1,
                    3,
                    progress * 2.2,
                    col_crimson.with_alpha(fade * 0.95),
                );
                draw_regular_polygon(
                    &mut gizmos,
                    center,
                    r1 * 0.82,
                    3,
                    -progress * 2.2 + PI,
                    col_purple.with_alpha(fade * 0.9),
                );
            }
            MapVfxKind::GatewayBeam { to_pos } => {
                // Energy surge along gateway connection (TASARIM.md 3.1 & 6)
                gizmos.line_2d(center, to_pos, col_turquoise.with_alpha(fade * 0.9));
                let head = center.lerp(to_pos, ease_out);
                draw_diamond(&mut gizmos, head, 6.5, col_gold.with_alpha(fade));
                gizmos
                    .circle_2d(
                        Isometry2d::from_translation(to_pos),
                        10.0 + ease_out * 20.0,
                        col_turquoise.with_alpha(fade * 0.8),
                    )
                    .resolution(36);
            }
        }

        true
    });
}

fn animate_floating_map_toasts(
    mut commands: Commands,
    time: Res<Time>,
    mut runtime: ResMut<GameRuntimeState>,
    mut q_toasts: Query<(
        Entity,
        &mut FloatingMapToast,
        &mut Transform,
        &mut TextColor,
    )>,
) {
    let is_paused = runtime.speed.anim_multiplier() == 0.0;
    let dt = if is_paused { 0.0 } else { time.delta_secs() };

    // Spawn any newly queued world-space floating toasts
    for pending in runtime.pending_toasts.drain(..) {
        commands.spawn((
            FloatingMapToast {
                elapsed: 0.0,
                duration: 1.45,
                base_color: pending.color,
            },
            Text2d::new(pending.text),
            TextFont {
                font_size: FontSize::Px(11.5),
                ..default()
            },
            TextColor(pending.color),
            Transform::from_xyz(pending.world_pos.x, pending.world_pos.y, 12.0),
        ));
    }

    if dt == 0.0 {
        return;
    }

    for (entity, mut toast, mut transform, mut color) in &mut q_toasts {
        toast.elapsed += dt;
        if toast.elapsed >= toast.duration {
            commands.entity(entity).despawn();
            continue;
        }
        let progress = (toast.elapsed / toast.duration).clamp(0.0, 1.0);
        transform.translation.y += 22.0 * dt * (1.0 - progress * 0.4);
        let alpha = (1.0 - progress * progress).clamp(0.0, 1.0);
        *color = TextColor(toast.base_color.with_alpha(alpha));
    }
}

fn draw_regular_polygon(
    gizmos: &mut Gizmos,
    center: Vec2,
    radius: f32,
    sides: usize,
    rotation: f32,
    color: Color,
) {
    if sides < 3 {
        return;
    }
    for i in 0..sides {
        let a1 = rotation + (i as f32) * TAU / (sides as f32);
        let a2 = rotation + ((i + 1) as f32) * TAU / (sides as f32);
        let p1 = center + Vec2::new(a1.cos() * radius, a1.sin() * radius);
        let p2 = center + Vec2::new(a2.cos() * radius, a2.sin() * radius);
        gizmos.line_2d(p1, p2, color);
    }
}

fn draw_diamond(gizmos: &mut Gizmos, center: Vec2, radius: f32, color: Color) {
    draw_regular_polygon(gizmos, center, radius, 4, 0.0, color);
}

fn deity_color(deity_id: u32) -> Color {
    match deity_id {
        PLAYER_DEITY_ID => Color::srgb_u8(0xC4, 0x6B, 0xFF), // Royal Amethyst
        1 => Color::srgb_u8(0xFF, 0x4F, 0x79),               // Vaelkor - Solar Crimson
        2 => Color::srgb_u8(0xE8, 0x4A, 0xF0),               // Morgeth - Astral Magenta
        3 => Color::srgb_u8(0x2E, 0xE5, 0xB5),               // Sylvara - Emerald Jade
        4 => Color::srgb_u8(0xFF, 0xB8, 0x4D),               // Aurelion - Celestial Amber
        5 => Color::srgb_u8(0xFF, 0x8A, 0x33),               // Solinvictus - Solar Flame
        6 => Color::srgb_u8(0xFF, 0x73, 0x99),               // Nyxarion - Nebula Rose
        7 => Color::srgb_u8(0x38, 0xD2, 0xFF),               // Mechanon - Cyber Cyan
        _ => Color::srgb_u8(0xAE, 0x8C, 0xFF),               // Vespera - Starlight Violet
    }
}
