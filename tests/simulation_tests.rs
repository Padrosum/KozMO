use dominion_sim::{
    CivilizationDoctrine, CosmicDilemma, DivineAction, GameOutcome, GatewayConnection,
    GatewayState, HistoryCategory, PowerId, ResourceRemovalReason, ResourceSource,
    SimulationConfig, SimulationState, TutorialStep, UniverseType, VictoryPath,
    DEFAULT_RIVAL_DEITY_COUNT, DEFAULT_UNIVERSE_COUNT, PLAYER_DEITY_ID,
};

#[test]
fn test_initial_pool_has_60_universes_6_types_and_8_rival_deities() {
    let config = SimulationConfig::default();
    let state = SimulationState::new(config);

    assert_eq!(state.universes.len(), DEFAULT_UNIVERSE_COUNT);
    assert_eq!(state.deities.len(), DEFAULT_RIVAL_DEITY_COUNT + 1);

    for u_type in UniverseType::ALL {
        assert!(
            state.universes.iter().any(|u| u.universe_type == u_type),
            "Missing universe type {:?}",
            u_type
        );
    }

    let total_civs: usize = state.universes.iter().map(|u| u.civilizations.len()).sum();
    assert!(
        total_civs >= 30,
        "Expected at least 30 active civilizations, got {total_civs}"
    );
}

#[test]
fn test_deterministic_simulation_and_unobserved_universe_evolution() {
    let config = SimulationConfig {
        seed: 133_742,
        ..SimulationConfig::default()
    };

    let mut sim_a = SimulationState::new(config.clone());
    let mut sim_b = SimulationState::new(config);

    // Find an unobserved outer-ring universe
    let unobserved_id = sim_a
        .universes
        .iter()
        .find(|u| !u.discovered_by_player)
        .expect("Should have undiscovered universes")
        .id;
    let initial_stability = sim_a.universes[unobserved_id as usize].stability;

    for _ in 0..20 {
        sim_a.step_tick();
        sim_b.step_tick();
    }

    // Both runs with identical seed must produce identical state
    assert_eq!(sim_a.tick, 20);
    assert_eq!(sim_a.deities, sim_b.deities);
    assert_eq!(sim_a.universes, sim_b.universes);

    // Unobserved universe must have evolved independently
    let after_stability = sim_a.universes[unobserved_id as usize].stability;
    assert_ne!(
        initial_stability, after_stability,
        "Unobserved universe stability should evolve over 20 ticks"
    );
}

#[test]
fn test_divine_actions_and_victory_paths() {
    let mut sim = SimulationState::new(SimulationConfig::default());

    // Grant resources to test unlocking powers and divine interventions
    sim.player_deity_mut().resources.souls = 50_000;
    sim.player_deity_mut().resources.influence = 25_000;
    sim.player_deity_mut().resources.cosmic_essence = 25_000;

    // Early Ascension Ritual attempt must fail before prerequisites (4 powers & 3 Full Allegiance) are met
    assert!(sim
        .execute_action(PLAYER_DEITY_ID, DivineAction::AdvanceAscensionRitual)
        .is_err());

    // Unlock 10 powers across trees in tier order (required for Transcendence Victory)
    let powers_to_unlock = [
        PowerId::GenesisSpark,
        PowerId::RealityAnchor,
        PowerId::CosmicForge,
        PowerId::VoiceOfProphecy,
        PowerId::GoldenMiracle,
        PowerId::AbsoluteDogma,
        PowerId::SoulSiphon,
        PowerId::ReapersToll,
        PowerId::AstralSight,
        PowerId::GatewayMastery,
    ];
    for p in powers_to_unlock {
        let res = sim.execute_action(PLAYER_DEITY_ID, DivineAction::UnlockPower { power: p });
        assert!(res.is_ok(), "Failed to unlock {p:?}: {res:?}");
    }

    // Establish Full Allegiance (★ >= 65% influence) in 10 universes
    for u_idx in 0..10 {
        sim.universes[u_idx].discovered_by_player = true;
        sim.universes[u_idx].deity_influence.clear();
        sim.universes[u_idx].add_influence(PLAYER_DEITY_ID, 80.0);
        assert!(sim.universes[u_idx].has_full_allegiance(PLAYER_DEITY_ID));
    }

    // Test DisruptRivalAscension counter-play
    sim.deities[1].ascension_progress = 40.0;
    assert!(sim
        .execute_action(
            PLAYER_DEITY_ID,
            DivineAction::DisruptRivalAscension { target_deity_id: 1 }
        )
        .is_ok());
    assert_eq!(sim.deities[1].ascension_progress, 20.0);

    // Advance Ascension Ritual 9 times (90%) — game should still be Ongoing!
    for _ in 0..9 {
        let res = sim.execute_action(PLAYER_DEITY_ID, DivineAction::AdvanceAscensionRitual);
        assert!(res.is_ok(), "Ritual step failed: {res:?}");
    }
    assert!(matches!(sim.outcome, GameOutcome::Ongoing));

    // 10th step reaches 100% and triggers Transcendence Victory!
    assert!(sim
        .execute_action(PLAYER_DEITY_ID, DivineAction::AdvanceAscensionRitual)
        .is_ok());

    assert!(matches!(
        sim.outcome,
        GameOutcome::PlayerVictory {
            path: VictoryPath::Transcendence,
            ..
        }
    ));

    // Verify player can continue in Endless Sandbox Mode after victory
    sim.continue_in_sandbox_mode();
    assert!(matches!(sim.outcome, GameOutcome::Ongoing));
}

#[test]
fn test_tutorial_progression_and_focus_universe() {
    let mut sim = SimulationState::new(SimulationConfig::default());
    assert_eq!(sim.tutorial_step, TutorialStep::Step1Prophecy);
    assert_eq!(sim.focused_universe_id, Some(0));

    // Focus another discovered universe
    assert!(sim
        .execute_action(
            PLAYER_DEITY_ID,
            DivineAction::FocusUniverse {
                universe_id: Some(1)
            }
        )
        .is_ok());
    assert_eq!(sim.focused_universe_id, Some(1));

    // Step 1 -> 2: SpreadProphecy completes Step1Prophecy and grants reward
    let souls_before = sim.player_deity().resources.souls;
    assert!(sim
        .execute_action(
            PLAYER_DEITY_ID,
            DivineAction::SpreadProphecy { universe_id: 0 }
        )
        .is_ok());
    assert_eq!(sim.tutorial_step, TutorialStep::Step2Explore);
    assert!(
        sim.player_deity().resources.souls > souls_before - 35,
        "Completing tutorial step 1 should grant bonus souls"
    );

    // Step 2 -> 3: ScoutRegion completes Step2Explore
    assert!(sim
        .execute_action(
            PLAYER_DEITY_ID,
            DivineAction::ScoutRegion { universe_id: 0 }
        )
        .is_ok());
    assert_eq!(sim.tutorial_step, TutorialStep::Step3HarvestOrEssence);

    // Step 3 -> 4: HarvestSouls completes Step3HarvestOrEssence
    assert!(sim
        .execute_action(
            PLAYER_DEITY_ID,
            DivineAction::HarvestSouls { universe_id: 0 }
        )
        .is_ok());
    assert_eq!(sim.tutorial_step, TutorialStep::Step4UnlockPower);

    // Step 4 -> 5: UnlockPower completes Step4UnlockPower
    sim.player_deity_mut().resources.souls += 500;
    sim.player_deity_mut().resources.influence += 500;
    sim.player_deity_mut().resources.cosmic_essence += 500;
    assert!(sim
        .execute_action(
            PLAYER_DEITY_ID,
            DivineAction::UnlockPower {
                power: PowerId::VoiceOfProphecy
            }
        )
        .is_ok());
    assert_eq!(sim.tutorial_step, TutorialStep::Step5CreateOrStabilize);

    // Step 5 -> 6: BestowMiracle completes Step5CreateOrStabilize
    assert!(sim
        .execute_action(
            PLAYER_DEITY_ID,
            DivineAction::BestowMiracle { universe_id: 0 }
        )
        .is_ok());
    assert_eq!(sim.tutorial_step, TutorialStep::Step6RivalInteraction);

    // Step 6 -> Completed: OfferDiplomaticPact completes Step6RivalInteraction
    assert!(sim
        .execute_action(
            PLAYER_DEITY_ID,
            DivineAction::OfferDiplomaticPact { target_deity_id: 1 }
        )
        .is_ok());
    assert_eq!(sim.tutorial_step, TutorialStep::Completed);
}

#[test]
fn test_cosmic_dilemma_resolution_and_gateway_faith_spread() {
    let mut sim = SimulationState::new(SimulationConfig::default());

    // Inject an active cosmic dilemma and resolve it
    sim.active_dilemma = Some(CosmicDilemma {
        id: 1,
        universe_id: 0,
        title: "Test İkilemi".to_string(),
        description: "Test açıklaması".to_string(),
        choice_a_label: "Seçenek A".to_string(),
        choice_b_label: "Seçenek B".to_string(),
    });

    let souls_before = sim.player_deity().resources.souls;
    assert!(sim
        .execute_action(
            PLAYER_DEITY_ID,
            DivineAction::ResolveDilemma { choose_first: true }
        )
        .is_ok());
    assert!(sim.active_dilemma.is_none());
    assert!(sim.player_deity().resources.souls > souls_before);

    // Verify full allegiance on starting home universe (#0 starts at 65% influence)
    assert!(sim.universes[0].has_full_allegiance(PLAYER_DEITY_ID));
    assert!(sim.universes[0].is_dominated_by(PLAYER_DEITY_ID));
    assert!(sim.universes[1].is_dominated_by(PLAYER_DEITY_ID));
    assert!(!sim.universes[1].has_full_allegiance(PLAYER_DEITY_ID));

    // Verify cross-gateway faith propagation from universe 0 (dominant player) to neighbor 1
    sim.universes[0]
        .deity_influence
        .insert(PLAYER_DEITY_ID, 85.0);
    sim.universes[1].deity_influence.remove(&PLAYER_DEITY_ID);
    sim.universes[0].gateways.push(GatewayConnection {
        target_universe_id: 1,
        state: GatewayState::Open,
    });

    sim.step_tick();

    let spread_influence = sim.universes[1].influence_of(PLAYER_DEITY_ID);
    assert!(
        spread_influence > 0.0,
        "Expected player faith to spread across open gateway into universe 1, got {spread_influence}"
    );
}

#[test]
fn test_universe_sovereignty_and_expanded_deity_diplomacy() {
    let mut sim = SimulationState::new(SimulationConfig::default());

    // Rename player deity and verify persistence
    sim.rename_player_deity("Kaelion");
    assert_eq!(sim.player_deity().name, "Kaelion");

    // Universe #2 is discovered where rival deity #1 is dominant (not player)
    sim.universes[2].discovered_by_player = true;
    sim.universes[2].deity_influence.insert(1, 55.0);
    assert!(!sim.universes[2].is_dominated_by(PLAYER_DEITY_ID));

    // Give player plenty of resources to test action permissions
    sim.deities[0].resources.souls = 1000;
    sim.deities[0].resources.influence = 1000;
    sim.deities[0].resources.cosmic_essence = 1000;

    // Sovereign actions in non-dominated universe #2 must fail with an authority error
    assert!(sim
        .execute_action(
            PLAYER_DEITY_ID,
            DivineAction::HarvestSouls { universe_id: 2 }
        )
        .is_err());
    assert!(sim
        .execute_action(
            PLAYER_DEITY_ID,
            DivineAction::BestowMiracle { universe_id: 2 }
        )
        .is_err());
    assert!(sim
        .execute_action(
            PLAYER_DEITY_ID,
            DivineAction::FocusUniverse {
                universe_id: Some(2)
            }
        )
        .is_err());

    // Infiltration actions in non-dominated universe #2 must succeed
    assert!(sim
        .execute_action(
            PLAYER_DEITY_ID,
            DivineAction::SpreadProphecy { universe_id: 2 }
        )
        .is_ok());
    let rival_id = sim.universes[2]
        .dominant_deity()
        .map(|(id, _)| id)
        .unwrap_or(1);
    assert!(sim
        .execute_action(
            PLAYER_DEITY_ID,
            DivineAction::SmiteRivalInfluence {
                universe_id: 2,
                target_deity_id: rival_id
            }
        )
        .is_ok());

    // Once player becomes dominant in universe #2, sovereign actions succeed
    sim.universes[2]
        .deity_influence
        .insert(PLAYER_DEITY_ID, 75.0);
    assert!(sim.universes[2].is_dominated_by(PLAYER_DEITY_ID));
    assert!(sim
        .execute_action(
            PLAYER_DEITY_ID,
            DivineAction::HarvestSouls { universe_id: 2 }
        )
        .is_ok());

    // Test new Deity diplomacy and warfare actions (SendDivineGift & DeclareHolyWar)
    let rel_before = sim.deities[1].relation_toward(PLAYER_DEITY_ID);
    assert!(sim
        .execute_action(
            PLAYER_DEITY_ID,
            DivineAction::SendDivineGift { target_deity_id: 1 }
        )
        .is_ok());
    assert!(sim.deities[1].relation_toward(PLAYER_DEITY_ID) > rel_before);

    assert!(sim
        .execute_action(
            PLAYER_DEITY_ID,
            DivineAction::DeclareHolyWar { target_deity_id: 2 }
        )
        .is_ok());
}

#[test]
fn test_logical_economy_depletion_allegiance_bonus_and_upkeep() {
    let mut sim = SimulationState::new(SimulationConfig::default());
    sim.deities[0].resources.souls = 5_000;
    sim.deities[0].resources.influence = 5_000;
    sim.deities[0].resources.cosmic_essence = 5_000;

    // 1. Verify Full Allegiance (★ >= 65%) gives a higher economic yield than partial dominance (45%)
    sim.universes[0]
        .deity_influence
        .insert(PLAYER_DEITY_ID, 45.0);
    let (_, partial_bd) = sim
        .universe_economy_breakdown(0)
        .expect("Universe 0 should have breakdown");
    sim.universes[0]
        .deity_influence
        .insert(PLAYER_DEITY_ID, 75.0);
    let (_, full_bd) = sim
        .universe_economy_breakdown(0)
        .expect("Universe 0 should have breakdown");
    assert!(
        full_bd.gross_income().souls > partial_bd.gross_income().souls
            && full_bd.gross_income().influence > partial_bd.gross_income().influence,
        "Full allegiance (★) must yield higher souls and influence than partial dominance"
    );

    // 2. Verify ExtractCosmicEssence depletes essence_richness and blocks spamming when exhausted
    sim.universes[0].essence_richness = 0.50;
    assert!(sim
        .execute_action(
            PLAYER_DEITY_ID,
            DivineAction::ExtractCosmicEssence { universe_id: 0 }
        )
        .is_ok());
    assert!(sim.universes[0].essence_richness < 0.45);
    assert!(
        sim.execute_action(
            PLAYER_DEITY_ID,
            DivineAction::ExtractCosmicEssence { universe_id: 0 }
        )
        .is_err(),
        "Depleted universe (< 0.45 richness) must reject immediate repeat essence extraction"
    );

    // StabilizeReality restores essence_richness
    assert!(sim
        .execute_action(
            PLAYER_DEITY_ID,
            DivineAction::StabilizeReality { universe_id: 0 }
        )
        .is_ok());
    assert!(sim.universes[0].essence_richness >= 0.55);

    // 3. Verify HarvestSouls requires actual reservoir or mortal population
    sim.universes[0].soul_reservoir = 5;
    for civ in &mut sim.universes[0].civilizations {
        civ.population = 5;
    }
    assert!(
        sim.execute_action(
            PLAYER_DEITY_ID,
            DivineAction::HarvestSouls { universe_id: 0 }
        )
        .is_err(),
        "Empty soul reservoir and tiny population must reject HarvestSouls"
    );

    // 4. Verify Diplomatic Pact generates DiplomacyTribute and Rebellion/Unstable universe records removals
    assert!(sim
        .execute_action(
            PLAYER_DEITY_ID,
            DivineAction::OfferDiplomaticPact { target_deity_id: 1 }
        )
        .is_ok());
    sim.universes[0].stability = 25.0;
    if let Some(civ) = sim.universes[0].civilizations.first_mut() {
        civ.population = 120;
        civ.stability = 18.0;
        civ.in_rebellion = true;
    }

    sim.step_tick();

    assert!(
        sim.ledger
            .totals_by_source
            .get(&ResourceSource::DiplomacyTribute)
            .is_some_and(|r| r.souls > 0 && r.influence > 0),
        "Active diplomatic pact should generate DiplomacyTribute resources"
    );
    assert!(
        sim.ledger
            .totals_by_removal
            .get(&ResourceRemovalReason::RebellionSchism)
            .is_some_and(|r| r.souls > 0),
        "Rebellious civilization in dominated universe should record RebellionSchism drain"
    );
    assert!(
        sim.ledger
            .totals_by_removal
            .get(&ResourceRemovalReason::UpkeepDecay)
            .is_some_and(|r| r.souls > 0),
        "Low-stability dominated universe should record UpkeepDecay drain"
    );
}

#[test]
fn test_simulation_performance_budget_and_background_consistency() {
    let mut sim_opt = SimulationState::new(SimulationConfig {
        seed: 99_101,
        ..SimulationConfig::default()
    });
    sim_opt.enable_background_optimization = true;

    let start = std::time::Instant::now();
    let ticks = 300u64;
    for _ in 0..ticks {
        sim_opt.step_tick();
    }
    let elapsed = start.elapsed();
    let avg_ms_per_tick = elapsed.as_secs_f64() * 1000.0 / (ticks as f64);

    // Budget check: even in unoptimized debug test builds, a tick across 60 universes & 9 deities
    // must complete well under 3.0 ms/tick (leaving >80% frame budget at 60 FPS).
    assert!(
        avg_ms_per_tick < 3.0,
        "Simulation step exceeded performance budget: {avg_ms_per_tick:.3} ms/tick"
    );

    // Distant unobserved universes must have accumulated soul reservoir & evolved civilizations
    let distant_reservoir_sum: i64 = sim_opt
        .universes
        .iter()
        .filter(|u| !u.discovered_by_player && u.region_tier >= 2 && !u.is_destroyed)
        .map(|u| u.soul_reservoir)
        .sum();
    assert!(
        distant_reservoir_sum > 200,
        "Distant unobserved universes should actively accumulate souls under background optimization"
    );
}

#[test]
fn test_power_unlock_effects_random_events_and_interactive_universe_actions() {
    let mut sim = SimulationState::new(SimulationConfig::default());
    sim.deities[0].resources.souls = 10_000;
    sim.deities[0].resources.influence = 10_000;
    sim.deities[0].resources.cosmic_essence = 10_000;

    // 1. Unlocking powers has immediate burst effects AND passive economy effects
    let (_, bd_before) = sim
        .universe_economy_breakdown(0)
        .expect("Universe 0 should have breakdown");
    let pop_before: u64 = sim.universes[0]
        .civilizations
        .iter()
        .map(|c| c.population)
        .sum();
    let discovered_before = sim
        .universes
        .iter()
        .filter(|u| u.discovered_by_player)
        .count();

    assert!(sim
        .execute_action(
            PLAYER_DEITY_ID,
            DivineAction::UnlockPower {
                power: PowerId::GenesisSpark,
            }
        )
        .is_ok());
    let pop_after: u64 = sim.universes[0]
        .civilizations
        .iter()
        .map(|c| c.population)
        .sum();
    assert!(
        pop_after > pop_before,
        "GenesisSpark unlock must immediately boost mortal population in dominated universes"
    );

    assert!(sim
        .execute_action(
            PLAYER_DEITY_ID,
            DivineAction::UnlockPower {
                power: PowerId::SoulSiphon,
            }
        )
        .is_ok());
    let (_, bd_after) = sim
        .universe_economy_breakdown(0)
        .expect("Universe 0 should have breakdown");
    assert!(
        bd_after.gross_income().souls > bd_before.gross_income().souls,
        "SoulSiphon power must increase passive soul income per tick"
    );

    assert!(sim
        .execute_action(
            PLAYER_DEITY_ID,
            DivineAction::UnlockPower {
                power: PowerId::AstralSight,
            }
        )
        .is_ok());
    let discovered_after = sim
        .universes
        .iter()
        .filter(|u| u.discovered_by_player)
        .count();
    assert!(
        discovered_after > discovered_before,
        "AstralSight unlock must immediately discover new cosmic regions"
    );

    // 2. Interactive Universe Actions: EnlightenCivilizations, ReshapeUniverseDomain, ChannelAmbientEnergy
    if let Some(civ) = sim.universes[0].civilizations.first_mut() {
        civ.divine_shield_active = true;
        civ.in_rebellion = true;
    }
    assert!(sim
        .execute_action(
            PLAYER_DEITY_ID,
            DivineAction::EnlightenCivilizations { universe_id: 0 }
        )
        .is_ok());
    assert!(
        sim.universes[0]
            .civilizations
            .iter()
            .all(|c| !c.divine_shield_active && !c.in_rebellion),
        "EnlightenCivilizations must dismantle scientific shields and end rebellions"
    );

    let pot_before = sim.universes[0].soul_potential;
    assert!(sim
        .execute_action(
            PLAYER_DEITY_ID,
            DivineAction::ReshapeUniverseDomain { universe_id: 0 }
        )
        .is_ok());
    assert!(
        sim.universes[0].soul_potential > pot_before
            && sim.universes[0].shaped_by_deity == Some(PLAYER_DEITY_ID),
        "ReshapeUniverseDomain must boost soul_potential and mark shaped_by_deity"
    );

    let souls_before = sim.deities[0].resources.souls;
    assert!(sim
        .execute_action(
            PLAYER_DEITY_ID,
            DivineAction::ChannelAmbientEnergy { universe_id: 0 }
        )
        .is_ok());
    assert!(
        sim.deities[0].resources.souls > souls_before,
        "ChannelAmbientEnergy must net positive souls from ambient cosmic energy"
    );

    // 3. Random interval events trigger across ticks and update next_random_event_tick & last_event_universe_id
    let mut observed_schedule_changes = 0usize;
    let mut prev_sched = sim.next_random_event_tick;
    for _ in 0..18 {
        sim.step_tick();
        if sim.next_random_event_tick != prev_sched {
            observed_schedule_changes += 1;
            prev_sched = sim.next_random_event_tick;
        }
    }
    assert!(
        observed_schedule_changes >= 3 && sim.last_event_universe_id.is_some(),
        "Random cosmic events must trigger at randomized intervals and record last_event_universe_id"
    );
}

#[test]
fn test_civilization_doctrines_rival_cult_schisms_and_ancient_archaeology() {
    let mut sim = SimulationState::new(SimulationConfig::default());

    // 1. Faz 2.3.1: Every generated universe must have at least 2 AncientOrigin history entries (including [Kadim Çağ] archaeology)
    for u in &sim.universes {
        let ancient_count = u
            .history
            .iter()
            .filter(|h| h.category == HistoryCategory::AncientOrigin)
            .count();
        assert!(
            ancient_count >= 2,
            "Universe #{} must start with at least 2 AncientOrigin history entries",
            u.id
        );
        assert!(
            u.history
                .iter()
                .any(|h| h.description.contains("[Kadim Çağ]")),
            "Universe #{} must include procedural [Kadim Çağ] archaeology lore",
            u.id
        );
    }

    // 2. Faz 2.2.1: Verify all 5 CivilizationDoctrines are represented across the cosmic pool
    for doc in CivilizationDoctrine::ALL {
        assert!(
            sim.universes
                .iter()
                .flat_map(|u| &u.civilizations)
                .any(|c| c.doctrine == doc),
            "Expected doctrine {:?} to appear in initial civilization pool",
            doc
        );
    }

    // 3. Faz 2.2.2: Rival Cults & Sectarian Schism when two deities have >= 25% influence in a multi-civ universe
    sim.deities[0].resources.souls = 2000;
    sim.deities[0].resources.influence = 2000;
    sim.deities[0].resources.cosmic_essence = 2000;

    // Ensure universe 0 has at least 2 civilizations
    if sim.universes[0].civilizations.len() < 2 {
        assert!(sim
            .execute_action(
                PLAYER_DEITY_ID,
                DivineAction::SeedLifeAndOrder { universe_id: 0 }
            )
            .is_ok());
    }
    assert!(sim.universes[0].civilizations.len() >= 2);

    // Give Rival Deity #1 >= 30% influence alongside Player Deity #0 in universe 0
    sim.universes[0]
        .deity_influence
        .insert(PLAYER_DEITY_ID, 60.0);
    sim.universes[0].deity_influence.insert(1, 35.0);

    // Advance 12 ticks so sectarian split and a sectarian clash occur
    for _ in 0..12 {
        sim.step_tick();
    }

    let has_conflict_history = sim.universes[0]
        .history
        .iter()
        .any(|h| h.category == HistoryCategory::ReligiousConflict);
    assert!(
        has_conflict_history,
        "Expected sectarian conflict (ReligiousConflict) in contested multi-civilization universe"
    );
}
