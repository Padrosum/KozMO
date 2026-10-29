use dominion_sim::{
    DifficultyLevel, GameMode, SimulationConfig, DEFAULT_RIVAL_DEITY_COUNT, DEFAULT_UNIVERSE_COUNT,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScenarioPreset {
    pub id: &'static str,
    pub title_tr: &'static str,
    pub subtitle_tr: &'static str,
    pub description_tr: &'static str,
    pub config: SimulationConfig,
}

pub fn built_in_scenarios() -> Vec<ScenarioPreset> {
    vec![
        ScenarioPreset {
            id: "cosmic_ascension",
            title_tr: "Kozmik Yükseliş (Ana Senaryo)",
            subtitle_tr: "60 Evren · 8 Rakip Tanrı · 3 Zafer Yolu",
            description_tr: "Genç bir ilah olarak başlangıç bölgesindeki sınırlı nüfuzunu kullan, ruh topla, yeni geçitler aç ve rakip tanrıların baskısına karşı Hâkimiyet, Aşkınlık veya Yaratım zaferine ulaş.",
            config: SimulationConfig {
                seed: 202_610,
                universe_count: DEFAULT_UNIVERSE_COUNT,
                rival_deity_count: DEFAULT_RIVAL_DEITY_COUNT,
                game_mode: GameMode::ScenarioCosmicAscension,
                difficulty: DifficultyLevel::Balanced,
                player_name: "Aethelgard".to_string(),
            },
        },
        ScenarioPreset {
            id: "twilight_of_gods",
            title_tr: "İlahların Alacakaranlığı (Zorlu Senaryo)",
            subtitle_tr: "60 Evren · 8 Agresif Tanrı · Yüksek Tehlike",
            description_tr: "Rakip tanrılar güçlü bir başlangıç özüyle uyanır. Geçitleri stratejik olarak kilitle, isyanlar çıkar ve yok olmadan önce kozmik tahtı ele geçir.",
            config: SimulationConfig {
                seed: 994_108,
                universe_count: DEFAULT_UNIVERSE_COUNT,
                rival_deity_count: DEFAULT_RIVAL_DEITY_COUNT,
                game_mode: GameMode::ScenarioCosmicAscension,
                difficulty: DifficultyLevel::Grandmaster,
                player_name: "Valkorion".to_string(),
            },
        },
        ScenarioPreset {
            id: "endless_pool_sandbox",
            title_tr: "Sonsuz Havuz (Serbest Sandbox)",
            subtitle_tr: "60 Evren · Bitiş Koşulu Yok · Tam Özgürlük",
            description_tr: "Hiçbir bitiş koşulu olmadan evrenler havuzunu gözlemle, yeni yaşam biçimleri tohumla, uygarlıkların yükselişini izle veya bütün kozmosu yeniden şekillendir.",
            config: SimulationConfig {
                seed: 777_001,
                universe_count: DEFAULT_UNIVERSE_COUNT,
                rival_deity_count: DEFAULT_RIVAL_DEITY_COUNT,
                game_mode: GameMode::SandboxEndlessPool,
                difficulty: DifficultyLevel::Story,
                player_name: "Luminary".to_string(),
            },
        },
    ]
}
