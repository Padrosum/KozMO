pub mod civilization;
pub mod deity;
pub mod economy;
pub mod events;
pub mod rng;
pub mod save;
pub mod simulation;
pub mod universe;

pub use civilization::{CivTickReport, Civilization, CivilizationDoctrine};
pub use deity::{Deity, DeityArchetype, DiplomaticStance, PowerId, PowerTree, PLAYER_DEITY_ID};
pub use economy::{
    LedgerEntry, LedgerOperation, ResourceExpenditure, ResourceLedger, ResourceRemovalReason,
    ResourceSource, ResourceType, Resources,
};
pub use events::{CosmicEvent, CosmicEventKind, EventSeverity};
pub use rng::DeterministicRng;
pub use save::{
    deserialize_save, load_from_path, migrate_v1_to_v2, save_to_path_atomic, serialize_save,
    LegacySaveEnvelopeV1, SaveEnvelope, SaveError, CURRENT_SAVE_SCHEMA_VERSION, SAVE_MAGIC,
};
pub use simulation::{
    CosmicDilemma, DifficultyLevel, DivineAction, GameMode, GameOutcome, SimulationConfig,
    SimulationState, TutorialStep, VictoryPath, CREATION_REQUIRED_SHAPED_UNIVERSES,
    CREATION_REQUIRED_STABILITY, DEFAULT_RIVAL_DEITY_COUNT, DEFAULT_UNIVERSE_COUNT,
    DOMINION_REQUIRED_DOMINATED, DOMINION_REQUIRED_FULL_ALLEGIANCE,
    DOMINION_REQUIRED_SEALED_RIVALS, TRANSCENDENCE_MIN_FULL_ALLEGIANCE_TO_START,
    TRANSCENDENCE_MIN_POWERS_TO_START, TRANSCENDENCE_REQUIRED_FULL_ALLEGIANCE,
    TRANSCENDENCE_REQUIRED_POWERS, TRANSCENDENCE_STEP_GAIN,
};
pub use universe::{
    GatewayConnection, GatewayState, HistoryCategory, Universe, UniverseEconomyBreakdown,
    UniverseHistoryEntry, UniverseType,
};
