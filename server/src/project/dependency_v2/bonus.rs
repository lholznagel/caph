use super::dependency::{FlatTreeEntry, FlatTree};
use caph_connector::TypeId;
use serde::Deserialize;

/// Represents
pub struct Bonus {
    blueprints: Vec<BlueprintBonus>,
    structures: Vec<StructureBonus>,
}

impl Bonus {
    pub fn new(
        blueprints: Vec<BlueprintBonus>,
        structures: Vec<StructureBonus>,
    ) -> Self {
        Self {
            blueprints,
            structures,
        }
    }

    pub fn apply_blueprint_bonus(
        &self,
        tree: &mut FlatTree,
    ) {
        for blueprint in self.blueprints.iter() {
            self.apply_me_bonus(
                blueprint.ptype_id,
                blueprint.material,
                tree,
            );
            tree.calculate();
        }
    }

    pub fn apply_structure_bonus(
        &self,
        tree: &mut FlatTree
    ) {
        let structure = self.structures[0].clone();

        let tree_clone = tree.clone();
        let blueprints = tree_clone.0
            .iter()
            .filter(|(_, e)| e.typ == BonusTyp::Blueprint)
            .collect::<Vec<_>>();

        for (_, blueprint) in blueprints {
            let rig = structure
                .rigs
                .iter()
                .filter(|x| x.typ == blueprint.typ)
                .find(|x| {
                    if x.categories.is_empty() && x.groups.is_empty() {
                        true
                    } else if x.categories.contains(&blueprint.info.category_id) {
                        true
                    } else if x.groups.contains(&blueprint.info.group_id) {
                        true
                    } else {
                        false
                    }
                })
                .map(|x| (x.material, x.time));

            if let Some((me, _)) = rig {
                self.apply_me_bonus(
                    blueprint.ptype_id,
                    me,
                    tree,
                );
                tree.calculate();
            }

            self.apply_me_bonus(
                blueprint.ptype_id,
                structure.material,
                tree,
            );
            tree.calculate();
        }

        // duplication of the above
        let tree_clone = tree.clone();
        let reactions = tree_clone.0
            .iter()
            .filter(|(_, e)| e.typ == BonusTyp::Reaction)
            .collect::<Vec<_>>();

        //let structure = self.structures[1].clone();
        for (_, reaction) in reactions {
            let rig = structure
                .rigs
                .iter()
                .filter(|x| x.typ == reaction.typ)
                .find(|x| {
                    if x.categories.is_empty() && x.groups.is_empty() {
                        true
                    } else if x.categories.contains(&reaction.info.category_id) {
                        true
                    } else if x.groups.contains(&reaction.info.group_id) {
                        true
                    } else {
                        false
                    }
                })
                .map(|x| (x.material, x.time));

            if let Some((me, _)) = rig {
                self.apply_me_bonus(
                    reaction.ptype_id,
                    me,
                    tree,
                );
                tree.calculate();
            }

            self.apply_me_bonus(
                reaction.ptype_id,
                structure.material,
                tree,
            );
            tree.calculate();
        }
    }

    fn apply_me_bonus(
        &self,
        ptype_id: TypeId,
        bonus:    f32,
        tree:     &mut FlatTree,
    ) {
        let entry = tree.0.get_mut(&ptype_id).unwrap();

        for (_, base_val) in entry.children.iter_mut() {
            let mut bonus_modifier = BonusModifier {
                before: *base_val,
                after:  0f32,
                change: 0f32,
            };

            bonus_modifier.change = bonus;
            let modifier = *base_val * (bonus as f32 / 100f32);

            *base_val = *base_val - modifier;
            bonus_modifier.after = *base_val;
            entry.bonus_changes.push(bonus_modifier);
        }
    }
}

#[derive(Clone, Debug)]
pub struct BonusModifier {
    pub before: f32,
    pub after:  f32,
    pub change: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct BlueprintBonus {
    pub ptype_id: TypeId,
    pub material: f32,
    pub time:     f32,
}

#[derive(Clone, Debug)]
pub struct RigBonus {
    pub type_id:    TypeId,
    pub material:   f32,
    pub time:       f32,
    pub categories: Vec<u32>,
    pub groups:     Vec<u32>,
    pub typ:        BonusTyp,
}

#[derive(Clone, Debug)]
pub struct StructureBonus {
    pub type_id:    TypeId,
    pub material:   f32,
    pub time:       f32,
    pub rigs:       Vec<RigBonus>,
    pub categories: Vec<u32>,
    pub groups:     Vec<u32>,
    pub typ:        BonusTyp,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum BonusTyp {
    Blueprint,
    Reaction,
    Material,
}
