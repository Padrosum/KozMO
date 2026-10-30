use super::PLAYER_DEITY_ID;
use crate::deity::{Deity, DiplomaticStance, PowerId};
use crate::economy::Resources;

/// Computes per-deity income from active diplomatic pacts and EternalSeal tribute.
/// Kept pure so this part of the economy can be tested independently from tick mutation.
pub(super) fn diplomacy_income(deities: &[Deity]) -> Vec<Resources> {
    let mut income = vec![Resources::ZERO; deities.len()];

    for (deity_index, deity) in deities.iter().enumerate() {
        if deity.is_sealed {
            continue;
        }

        let deity_id = deity.id;
        let active_pacts = deity
            .stances
            .iter()
            .filter(|(&other_id, stance)| {
                matches!(stance, DiplomaticStance::Pact { .. })
                    && deities
                        .get(other_id as usize)
                        .is_some_and(|other| !other.is_sealed)
            })
            .count() as i64;
        income[deity_index].souls += active_pacts * 4;
        income[deity_index].influence += active_pacts * 3;

        if deity.has_power(PowerId::EternalSeal) {
            let sealed_tribute_count = deities
                .iter()
                .filter(|other| {
                    other.is_sealed
                        && (other.sealed_by == Some(deity_id) || deity_id == PLAYER_DEITY_ID)
                })
                .count() as i64;
            income[deity_index].souls += sealed_tribute_count * 8;
            income[deity_index].influence += sealed_tribute_count * 5;
            income[deity_index].cosmic_essence += sealed_tribute_count * 2;
        }
    }

    income
}
