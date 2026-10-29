use crate::plugins::GameRuntimeState;
use bevy::prelude::*;
use dominion_sim::{load_from_path, save_to_path_atomic};
use std::path::PathBuf;

pub const DEFAULT_SAVE_FILE: &str = "saves/eternal_dominion.sav";

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_save_load_hotkeys);
    }
}

pub fn save_file_path() -> PathBuf {
    PathBuf::from(DEFAULT_SAVE_FILE)
}

pub fn save_game_state(runtime: &mut GameRuntimeState) {
    let path = save_file_path();
    match save_to_path_atomic(&path, &runtime.sim) {
        Ok(()) => {
            runtime.status_message = format!(
                "Oyun başarıyla kaydedildi: {} (Tur {})",
                path.display(),
                runtime.sim.tick
            );
        }
        Err(err) => {
            runtime.status_message = format!("Kayıt hatası: {err}");
        }
    }
}

pub fn load_game_state(runtime: &mut GameRuntimeState) {
    let path = save_file_path();
    match load_from_path(&path) {
        Ok(envelope) => {
            let tick = envelope.saved_at_tick;
            runtime.sim = envelope.state;
            runtime.selected_universe_id = 0;
            runtime.tick_accumulator = 0.0;
            runtime.status_message = format!(
                "Kayıt başarıyla yüklendi (Tur {tick}, Şema v{}).",
                envelope.schema_version
            );
        }
        Err(err) => {
            runtime.status_message = format!("Yükleme hatası: {err}");
        }
    }
}

fn handle_save_load_hotkeys(
    keys: Res<ButtonInput<KeyCode>>,
    mut runtime: ResMut<GameRuntimeState>,
) {
    if keys.just_pressed(KeyCode::F5) {
        save_game_state(&mut runtime);
    } else if keys.just_pressed(KeyCode::F9) {
        load_game_state(&mut runtime);
    }
}
