use dominion_sim::{PowerId, PowerTree, Resources};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PowerDefinition {
    pub id: PowerId,
    pub tree: PowerTree,
    pub tier: u8,
    pub name_tr: &'static str,
    pub description_tr: &'static str,
    pub cost: Resources,
    pub prerequisite: Option<PowerId>,
    pub svg_asset: &'static str,
}

pub fn power_definition(id: PowerId) -> PowerDefinition {
    let svg_asset = match id {
        PowerId::GenesisSpark => "art/powers/power_genesis_spark.svg",
        PowerId::RealityAnchor => "art/powers/power_reality_anchor.svg",
        PowerId::CosmicForge => "art/powers/power_cosmic_forge.svg",
        PowerId::VoiceOfProphecy => "art/powers/power_voice_prophecy.svg",
        PowerId::GoldenMiracle => "art/powers/power_golden_miracle.svg",
        PowerId::AbsoluteDogma => "art/powers/power_absolute_dogma.svg",
        PowerId::SoulSiphon => "art/powers/power_soul_drain.svg",
        PowerId::ReapersToll => "art/powers/power_reapers_toll.svg",
        PowerId::EssenceCondenser => "art/powers/power_essence_condenser.svg",
        PowerId::AstralSight => "art/powers/power_astral_sight.svg",
        PowerId::GatewayMastery => "art/powers/power_gateway_mastery.svg",
        PowerId::DimensionalWard => "art/powers/power_dimensional_ward.svg",
        PowerId::ShadowSchism => "art/powers/power_shadow_schism.svg",
        PowerId::DivineSmite => "art/powers/power_divine_smite.svg",
        PowerId::EternalSeal => "art/powers/power_eternal_seal.svg",
    };

    PowerDefinition {
        id,
        tree: id.tree(),
        tier: id.tier(),
        name_tr: id.name_tr(),
        description_tr: id.description_tr(),
        cost: id.unlock_cost(),
        prerequisite: id.prerequisite(),
        svg_asset,
    }
}

pub fn powers_for_tree(tree: PowerTree) -> Vec<PowerDefinition> {
    PowerId::ALL
        .into_iter()
        .filter(|p| p.tree() == tree)
        .map(power_definition)
        .collect()
}

pub fn all_power_definitions() -> Vec<PowerDefinition> {
    PowerId::ALL.into_iter().map(power_definition).collect()
}
