use dominion_sim::{
    deserialize_save, load_from_path, save_to_path_atomic, serialize_save, LegacySaveEnvelopeV1,
    SaveError, SimulationConfig, SimulationState, CURRENT_SAVE_SCHEMA_VERSION, SAVE_MAGIC,
};

#[test]
fn test_save_load_round_trip_and_deterministic_resume() {
    let config = SimulationConfig {
        seed: 987_654,
        ..SimulationConfig::default()
    };
    let mut sim_continuous = SimulationState::new(config);

    // Step 10 ticks
    for _ in 0..10 {
        sim_continuous.step_tick();
    }

    // Save to disk atomically and load back
    let tmp_save_path = std::env::temp_dir().join("eternal_dominion_roundtrip_test.sav");
    save_to_path_atomic(&tmp_save_path, &sim_continuous).expect("Atomic save should succeed");

    let loaded_envelope = load_from_path(&tmp_save_path).expect("Load should succeed");
    assert_eq!(loaded_envelope.schema_version, CURRENT_SAVE_SCHEMA_VERSION);
    assert_eq!(loaded_envelope.saved_at_tick, 10);

    let mut sim_resumed = loaded_envelope.state;
    assert_eq!(sim_continuous, sim_resumed);

    // Step 10 more ticks in both continuous and resumed simulations
    for _ in 0..10 {
        sim_continuous.step_tick();
        sim_resumed.step_tick();
    }

    assert_eq!(
        sim_continuous, sim_resumed,
        "Resumed simulation from save must match continuous deterministic simulation"
    );

    let _ = std::fs::remove_file(&tmp_save_path);
}

#[test]
fn test_invalid_saves_handled_safely_and_v1_schema_migrates() {
    // 1. Empty string
    assert!(matches!(
        deserialize_save(""),
        Err(SaveError::InvalidFormat(_))
    ));

    // 2. Invalid JSON
    assert!(matches!(
        deserialize_save("{not_valid_json"),
        Err(SaveError::InvalidFormat(_))
    ));

    // 3. Wrong magic header
    let wrong_magic = r#"{"magic":"WRONG_GAME","schema_version":2}"#;
    assert!(matches!(
        deserialize_save(wrong_magic),
        Err(SaveError::InvalidMagic(_))
    ));

    // 4. Unsupported future schema version
    let future_ver = r#"{"magic":"ETERNAL_DOMINION_SAVE","schema_version":999}"#;
    assert!(matches!(
        deserialize_save(future_ver),
        Err(SaveError::UnsupportedVersion(999))
    ));

    // 5. Legacy v1 schema migration to v2
    let sim = SimulationState::new(SimulationConfig::default());
    let v1_env = LegacySaveEnvelopeV1 {
        magic: SAVE_MAGIC.to_string(),
        schema_version: 1,
        state: sim.clone(),
    };
    let v1_json = serde_json::to_string(&v1_env).expect("Serialize v1");
    let migrated = deserialize_save(&v1_json).expect("v1 should migrate cleanly to v2");
    assert_eq!(migrated.schema_version, CURRENT_SAVE_SCHEMA_VERSION);
    assert!(!migrated.migration_notes.is_empty());
    assert_eq!(migrated.state, sim);

    // 6. Valid v2 serialization
    let v2_json = serialize_save(&sim).expect("Serialize v2");
    let v2_loaded = deserialize_save(&v2_json).expect("Load v2");
    assert_eq!(v2_loaded.state, sim);
}
