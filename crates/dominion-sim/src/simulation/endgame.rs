use super::{
    GameMode, GameOutcome, SimulationState, VictoryPath, CREATION_REQUIRED_SHAPED_UNIVERSES,
    DOMINION_REQUIRED_DOMINATED, DOMINION_REQUIRED_FULL_ALLEGIANCE,
    DOMINION_REQUIRED_SEALED_RIVALS, PLAYER_DEITY_ID, TRANSCENDENCE_REQUIRED_FULL_ALLEGIANCE,
    TRANSCENDENCE_REQUIRED_POWERS,
};
use crate::deity::DiplomaticStance;

impl SimulationState {
    pub(super) fn phase_5_update_diplomacy_and_seals(&mut self) {
        for deity in &mut self.deities {
            for stance in deity.stances.values_mut() {
                if let DiplomaticStance::Pact { ticks_remaining } = stance {
                    if *ticks_remaining <= 1 {
                        *stance = DiplomaticStance::Neutral;
                    } else {
                        *ticks_remaining -= 1;
                    }
                }
            }
        }
    }

    pub(super) fn phase_6_evaluate_victory_and_defeat(&mut self) {
        if !matches!(self.outcome, GameOutcome::Ongoing) {
            return;
        }
        if self.config.game_mode == GameMode::SandboxEndlessPool {
            return;
        }

        // Single-pass ownership tally across all universes.
        let (dom_counts, full_counts) = self.all_deity_universe_counts();

        let player_dom = dom_counts
            .get(PLAYER_DEITY_ID as usize)
            .copied()
            .unwrap_or(0);
        let player_full = full_counts
            .get(PLAYER_DEITY_ID as usize)
            .copied()
            .unwrap_or(0);
        let player_any_inf = self
            .universes
            .iter()
            .any(|u| !u.is_destroyed && u.influence_of(PLAYER_DEITY_ID) >= 10.0);
        let player_res = self.deities[PLAYER_DEITY_ID as usize].resources;

        if self.deities[PLAYER_DEITY_ID as usize].is_sealed
            || (!player_any_inf && player_res.souls < 50 && player_dom == 0)
        {
            self.outcome = GameOutcome::PlayerDefeat {
                tick: self.tick,
                reason: "Son kozmik dayanaklarını ve tanrısal özünü kaybettin; varlığın hiçliğe mühürlendi."
                    .to_string(),
            };
            return;
        }

        // A rival can end the scenario by completing Transcendence or dominating the pool.
        for deity in self.deities.iter().skip(1) {
            if deity.is_sealed {
                continue;
            }
            let rival_dom = dom_counts.get(deity.id as usize).copied().unwrap_or(0);
            let rival_full = full_counts.get(deity.id as usize).copied().unwrap_or(0);
            if deity.ascension_progress >= 100.0
                && deity.unlocked_powers.len() >= TRANSCENDENCE_REQUIRED_POWERS
                && rival_full >= TRANSCENDENCE_REQUIRED_FULL_ALLEGIANCE
            {
                self.outcome = GameOutcome::PlayerDefeat {
                    tick: self.tick,
                    reason: format!(
                        "Rakip tanrı {} ({}) 10 aşamalı Aşkınlık Ritüelini ve kozmik güçlerini tamamlayarak tüm kozmosa hükmetti!",
                        deity.name, deity.title
                    ),
                };
                return;
            }
            if rival_dom >= DOMINION_REQUIRED_DOMINATED {
                self.outcome = GameOutcome::PlayerDefeat {
                    tick: self.tick,
                    reason: format!(
                        "Rakip tanrı {} ({}) {} evrende mutlak hegemonya kurarak kozmik tahtı ele geçirdi!",
                        deity.name, deity.title, rival_dom
                    ),
                };
                return;
            }
        }

        let sealed_rivals = self.sealed_rival_count();
        if player_dom >= DOMINION_REQUIRED_DOMINATED
            && player_full >= DOMINION_REQUIRED_FULL_ALLEGIANCE
            && sealed_rivals >= DOMINION_REQUIRED_SEALED_RIVALS
        {
            self.outcome = GameOutcome::PlayerVictory {
                path: VictoryPath::Dominion,
                tick: self.tick,
                summary: format!(
                    "{player_dom} evrende baskınlık ({player_full} ★ Tam Aidiyet) kurdun ve {sealed_rivals} rakip tanrıyı mühürleyerek Kozmik Tahtın tek sahibi oldun!"
                ),
            };
            return;
        }

        let player = &self.deities[PLAYER_DEITY_ID as usize];
        if player.ascension_progress >= 100.0
            && player.unlocked_powers.len() >= TRANSCENDENCE_REQUIRED_POWERS
            && player_full >= TRANSCENDENCE_REQUIRED_FULL_ALLEGIANCE
        {
            self.outcome = GameOutcome::PlayerVictory {
                path: VictoryPath::Transcendence,
                tick: self.tick,
                summary: format!(
                    "10 aşamalı Aşkınlık Ritüelini %100 tamamladın, {} ilahi gücü uyandırdın ve {} ★ Tam Aidiyet evreniyle saf kozmik bilince yükseldin!",
                    player.unlocked_powers.len(), player_full
                ),
            };
            return;
        }

        let harmonious_count = self.harmonious_shaped_universe_count(PLAYER_DEITY_ID);
        if harmonious_count >= CREATION_REQUIRED_SHAPED_UNIVERSES
            && self.has_full_creation_tree(PLAYER_DEITY_ID)
        {
            self.outcome = GameOutcome::PlayerVictory {
                path: VictoryPath::Creation,
                tick: self.tick,
                summary: format!(
                    "Yaratım Ağacında ustalaşarak kendi ilahi düzenine bağlı {harmonious_count} yüksek istikrarlı ve ★ Tam Aidiyetli evrenden oluşan kusursuz bir kozmik ağ yarattın!"
                ),
            };
        }
    }
}
