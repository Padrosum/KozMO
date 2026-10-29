use crate::simulation::SimulationState;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::io::Write;
use std::path::Path;

pub const SAVE_MAGIC: &str = "ETERNAL_DOMINION_SAVE";
pub const CURRENT_SAVE_SCHEMA_VERSION: u32 = 2;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SaveError {
    Io(String),
    InvalidFormat(String),
    InvalidMagic(String),
    UnsupportedVersion(u32),
    CorruptedState(String),
}

impl fmt::Display for SaveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(msg) => write!(f, "Dosya hatası: {msg}"),
            Self::InvalidFormat(msg) => write!(f, "Geçersiz kayıt biçimi: {msg}"),
            Self::InvalidMagic(msg) => write!(f, "Tanınmayan kayıt başlığı: {msg}"),
            Self::UnsupportedVersion(ver) => {
                write!(f, "Desteklenmeyen kayıt şeması sürümü: v{ver}")
            }
            Self::CorruptedState(msg) => write!(f, "Bozuk oyun durumu: {msg}"),
        }
    }
}

impl std::error::Error for SaveError {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveEnvelope {
    pub magic: String,
    pub schema_version: u32,
    pub saved_at_tick: u64,
    #[serde(default)]
    pub migration_notes: Vec<String>,
    pub state: SimulationState,
}

/// Legacy v1 envelope structure used to verify and execute schema migrations safely.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacySaveEnvelopeV1 {
    pub magic: String,
    pub schema_version: u32,
    pub state: SimulationState,
}

pub fn serialize_save(state: &SimulationState) -> Result<String, SaveError> {
    validate_simulation_state(state)?;
    let envelope = SaveEnvelope {
        magic: SAVE_MAGIC.to_string(),
        schema_version: CURRENT_SAVE_SCHEMA_VERSION,
        saved_at_tick: state.tick,
        migration_notes: Vec::new(),
        state: state.clone(),
    };
    serde_json::to_string_pretty(&envelope).map_err(|e| SaveError::InvalidFormat(e.to_string()))
}

pub fn deserialize_save(raw: &str) -> Result<SaveEnvelope, SaveError> {
    let raw_trimmed = raw.trim();
    if raw_trimmed.is_empty() {
        return Err(SaveError::InvalidFormat("Boş kayıt verisi".to_string()));
    }

    let val: serde_json::Value =
        serde_json::from_str(raw_trimmed).map_err(|e| SaveError::InvalidFormat(e.to_string()))?;

    let magic = val
        .get("magic")
        .and_then(|v| v.as_str())
        .ok_or_else(|| SaveError::InvalidMagic("Başlık alanı eksik".to_string()))?;

    if magic != SAVE_MAGIC {
        return Err(SaveError::InvalidMagic(magic.to_string()));
    }

    let version = val
        .get("schema_version")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| SaveError::InvalidFormat("schema_version alanı eksik".to_string()))?
        as u32;

    let envelope = match version {
        1 => {
            let v1: LegacySaveEnvelopeV1 =
                serde_json::from_value(val).map_err(|e| SaveError::InvalidFormat(e.to_string()))?;
            migrate_v1_to_v2(v1)
        }
        CURRENT_SAVE_SCHEMA_VERSION => serde_json::from_value::<SaveEnvelope>(val)
            .map_err(|e| SaveError::InvalidFormat(e.to_string()))?,
        other => return Err(SaveError::UnsupportedVersion(other)),
    };

    validate_simulation_state(&envelope.state)?;
    Ok(envelope)
}

pub fn migrate_v1_to_v2(v1: LegacySaveEnvelopeV1) -> SaveEnvelope {
    let saved_at_tick = v1.state.tick;
    SaveEnvelope {
        magic: SAVE_MAGIC.to_string(),
        schema_version: CURRENT_SAVE_SCHEMA_VERSION,
        saved_at_tick,
        migration_notes: vec!["Migrated from schema v1 to v2".to_string()],
        state: v1.state,
    }
}

pub fn validate_simulation_state(state: &SimulationState) -> Result<(), SaveError> {
    if state.universes.is_empty() {
        return Err(SaveError::CorruptedState(
            "Evren havuzu boş olamaz".to_string(),
        ));
    }
    if state.deities.is_empty() {
        return Err(SaveError::CorruptedState(
            "Tanrı listesi boş olamaz".to_string(),
        ));
    }
    Ok(())
}

/// Atomically writes the save file by first writing and flushing a temporary `.tmp` file
/// in the same directory and then renaming it over the target path, protecting valid saves
/// from partial writes.
pub fn save_to_path_atomic(
    path: impl AsRef<Path>,
    state: &SimulationState,
) -> Result<(), SaveError> {
    let path = path.as_ref();
    let payload = serialize_save(state)?;

    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).map_err(|e| SaveError::Io(e.to_string()))?;
        }
    }

    let tmp_path = path.with_extension("sav.tmp");
    {
        let mut file = fs::File::create(&tmp_path).map_err(|e| SaveError::Io(e.to_string()))?;
        file.write_all(payload.as_bytes())
            .map_err(|e| SaveError::Io(e.to_string()))?;
        file.sync_all().map_err(|e| SaveError::Io(e.to_string()))?;
    }

    fs::rename(&tmp_path, path).map_err(|e| SaveError::Io(e.to_string()))?;
    Ok(())
}

pub fn load_from_path(path: impl AsRef<Path>) -> Result<SaveEnvelope, SaveError> {
    let raw = fs::read_to_string(path.as_ref()).map_err(|e| SaveError::Io(e.to_string()))?;
    deserialize_save(&raw)
}
