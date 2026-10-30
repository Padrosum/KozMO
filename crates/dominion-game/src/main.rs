mod audio;
mod map;
mod plugins;
mod save;
mod ui;

use audio::CosmicAudioPlugin;
use bevy::{asset::AssetId, prelude::*, window::WindowResolution};
use map::CosmicMapPlugin;
use plugins::SimulationBridgePlugin;
use save::SavePlugin;
use std::path::Path;
use ui::CosmicUiPlugin;

/// Embedded full-Unicode font (DejaVu Sans) with complete Turkish glyph coverage
/// (ğ, Ğ, ş, Ş, ı, İ, ö, Ö, ü, Ü, ç, Ç, â, î, û) and geometric symbols.
const TURKISH_UNICODE_FONT_BYTES: &[u8] = include_bytes!("../../../assets/fonts/DejaVuSans.ttf");

fn resolve_assets_path() -> String {
    // Bevy resolves relative asset paths from the crate's manifest directory, while
    // launchers may start the game from the user's data directory. Canonicalize the
    // working-directory assets first (including install-time symlinks), then fall
    // back to the repository assets relative to this crate.
    let candidates = [
        Path::new("assets").to_path_buf(),
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets"),
    ];
    for candidate in candidates {
        if let Ok(path) = candidate.canonicalize() {
            if path.is_dir() {
                return path.to_string_lossy().into_owned();
            }
        }
    }

    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets")
        .to_string_lossy()
        .into_owned()
}

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    // WM_CLASS / Wayland app_id → .desktop dosyası ve görev çubuğu ikonu eşleşmesi
                    name: Some("kozmo".to_string()),
                    title: "KozMO — Kozmik Sandbox Strateji (v0.2)".to_string(),
                    resolution: WindowResolution::new(1440, 860),
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                file_path: resolve_assets_path(),
                ..default()
            }),
    );

    // Replace Bevy's ASCII-subset default font with full Turkish Unicode font
    // so every Text and Text2d renders Turkish characters without missing glyphs.
    {
        let mut fonts = app.world_mut().resource_mut::<Assets<Font>>();
        let turkish_font = Font::from_bytes(TURKISH_UNICODE_FONT_BYTES.to_vec());
        let _ = fonts.insert(AssetId::<Font>::default(), turkish_font);
    }

    app.add_plugins((
        SimulationBridgePlugin,
        CosmicAudioPlugin,
        SavePlugin,
        CosmicMapPlugin,
        CosmicUiPlugin,
    ))
    .run();
}
