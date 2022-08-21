use std::collections::{BTreeMap, VecDeque};
use serde::Deserialize;
use caph_connector::TypeId;
use super::bonus::{BonusModifier, Bonus, BonusTyp};
use crate::Info;

/// Single dependency that represents either a end product, component or
/// material
/// 
#[derive(Clone, Debug, Deserialize)]
pub struct Dependency {
    ptype_id: TypeId,
    #[serde(rename = "quantity")]
    needed: f32,
    #[serde(rename = "produces")]
    base: u32,
    // TODO: add to json
    info:       DependencyInfo,
    components: Vec<Dependency>,
    typ:        BonusTyp,
}

impl Dependency {
    pub fn new<T, S>(
        ptype_id:   T,
        needed:     f32,
        base:       u32,
        info:       DependencyInfo,
        components: Vec<Dependency>,
        typ:        BonusTyp,
    ) -> Self
        where
            T: Into<TypeId>, {
        Self {
            ptype_id:   ptype_id.into(),
            needed:     needed,
            base:       base,
            info:       info,
            typ:        typ,
            components: components,
        }
    }
}

/// Group of dependencies.
/// 
#[derive(Debug, Default)]
pub struct DependencyGroup(Vec<Dependency>);

impl DependencyGroup {
    pub fn add(
        &mut self,
        dep: Dependency
    ) {
        self.0.push(dep);
    }

    pub fn flat_tree(self) -> FlatTree {
        let mut tree = FlatTree::default();
        let mut queue: VecDeque<Dependency> = self.0.into();

        while let Some(dep) = queue.pop_front() {
            tree.add(dep.clone());

            for component in dep.components.iter() {
                queue.push_back(component.clone());
            }
        }

        tree.calculate();
        tree
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct DependencyInfo {
    pub name:        String,
    pub category_id: u32,
    pub group_id:    u32,
}

#[derive(Clone, Debug, Default)]
pub struct FlatTree(pub BTreeMap<TypeId, FlatTreeEntry>);

impl FlatTree {
    pub fn add(
        &mut self,
        dep: Dependency,
    ) {
        // Collect all children and the require amount
        let children = dep
            .components
            .into_iter()
            .map(|x| (x.ptype_id, x.needed))
            .collect::<BTreeMap<_, _>>();

        // Either insert or edit the ptype_id entry
        self.0
            .entry(dep.ptype_id.clone())
            .and_modify(|x: &mut FlatTreeEntry| x.needed += dep.needed)
            .or_insert(FlatTreeEntry {
                ptype_id:      dep.ptype_id,
                name:          dep.info.name.clone(),
                needed:        dep.needed,
                base:          dep.base,
                children:      children,
                bonus_changes: Vec::new(),
                typ:           dep.typ,
                info:          dep.info,
            });
    }

    /// Calculates how many resources from every item is required.
    /// 
    /// The values are updated in place.
    /// 
    pub fn calculate(&mut self) {
        let mut queue = self.0
            .keys()
            .cloned()
            .collect::<VecDeque<_>>();

        while let Some(ptype_id) = queue.pop_front() {
            // Add all children to the queue
            if let Some(x) = self.0.get(&ptype_id) {
                x.children.keys().collect::<Vec<_>>()
            } else {
                Vec::new()
            }
            .into_iter()
            .for_each(|x| queue.push_back(*x));

            self.inner_calculate(ptype_id);
        }
    }

    /// Calculates a specific item.
    /// 
    /// Values are updated in place.
    /// 
    fn inner_calculate(&mut self, ptype_id: TypeId) {
        // Get all items that have the ptype_id as a children, calculate the
        // number of runs and collect them together
        let needed = self.0
            .iter()
            .filter(|(_, e)| e.children.contains_key(&ptype_id))
            .map(|(_, e)| {
                let runs = (e.needed / e.base as f32).ceil();
                let per_run = e.children.get(&ptype_id).unwrap_or(&0f32);
                runs * per_run
            })
            .sum();

        // Our product will not be in any children
        if needed == 0f32 {
            return;
        } else if let Some(x) = self.0.get_mut(&ptype_id) {
            x.needed = needed;
        };
    }
}

#[derive(Clone, Debug)]
pub struct FlatTreeEntry {
    pub ptype_id:      TypeId,
    pub name:          String,
    pub needed:        f32,
    pub base:          u32,
    pub children:      BTreeMap<TypeId, f32>,
    pub bonus_changes: Vec<BonusModifier>,
    pub typ:           BonusTyp,
    pub info:          DependencyInfo,
}

#[cfg(test)]
mod dependency_tests {
    use super::*;
    use crate::project::dependency_v2::bonus::{BlueprintBonus, Bonus, StructureBonus, RigBonus, BonusTyp};

    #[test]
    fn naglfar() {
        let file = std::fs::File::open("./testdata/naglfar.json").unwrap();
        let parsed: Dependency = serde_json::from_reader(file).unwrap();
        let mut tree = DependencyGroup(vec![parsed]).flat_tree();
        tree.calculate();

        let a = tree.0.get(&57481.into());
        dbg!(a);

        let a = tree.0.get(&57463.into());
        dbg!(a);

        let a = tree.0.get(&34.into());
        dbg!(a);

        let a = tree.0.get(&21009.into());
        dbg!(a);
    }

    #[test]
    fn caracal() {
        let blueprint_bonus = BlueprintBonus {
            ptype_id: 621.into(),
            material: 10f32,
            time:     20f32,
        };
        let rig_bonus = RigBonus {
            type_id:    37180.into(),
            material:   2f32 * 2.1f32,
            time:       20f32 * 2.1f32,
            categories: vec![6, 32],
            groups:     Vec::new(),
            typ:        BonusTyp::Blueprint
        };
        let structure_bonus = StructureBonus {
            type_id:    35827.into(),
            material:   1f32,
            time:       25f32,
            rigs:       vec![rig_bonus],
            categories: Vec::new(),
            groups:     Vec::new(),
            typ:        BonusTyp::Blueprint
        };
        let bonus = Bonus::new(
            vec![blueprint_bonus],
            vec![structure_bonus],
        );

        let file = std::fs::File::open("./testdata/caracal.json").unwrap();
        let parsed: Dependency = serde_json::from_reader(file).unwrap();
        let mut tree = DependencyGroup(vec![parsed]).flat_tree();
        tree.calculate();

        assert_eq!(tree.0.get(&34.into()).unwrap().needed.ceil(), 540_000f32);
        assert_eq!(tree.0.get(&35.into()).unwrap().needed.ceil(), 180_000f32);
        assert_eq!(tree.0.get(&36.into()).unwrap().needed.ceil(),  36_000f32);
        assert_eq!(tree.0.get(&37.into()).unwrap().needed.ceil(),  10_000f32);
        assert_eq!(tree.0.get(&38.into()).unwrap().needed.ceil(),   1_500f32);
        assert_eq!(tree.0.get(&39.into()).unwrap().needed.ceil(),     350f32);
        assert_eq!(tree.0.get(&40.into()).unwrap().needed.ceil(),     140f32);

        bonus.apply_blueprint_bonus(&mut tree);
        assert_eq!(tree.0.get(&34.into()).unwrap().needed.ceil(), 486_000f32);
        assert_eq!(tree.0.get(&35.into()).unwrap().needed.ceil(), 162_000f32);
        assert_eq!(tree.0.get(&36.into()).unwrap().needed.ceil(),  32_400f32);
        assert_eq!(tree.0.get(&37.into()).unwrap().needed.ceil(),   9_000f32);
        assert_eq!(tree.0.get(&38.into()).unwrap().needed.ceil(),   1_350f32);
        assert_eq!(tree.0.get(&39.into()).unwrap().needed.ceil(),     315f32);
        assert_eq!(tree.0.get(&40.into()).unwrap().needed.ceil(),     126f32);

        bonus.apply_structure_bonus(&mut tree);
        assert_eq!(tree.0.get(&34.into()).unwrap().needed.ceil(), 460_933f32);
        assert_eq!(tree.0.get(&35.into()).unwrap().needed.ceil(), 153_645f32);
        assert_eq!(tree.0.get(&36.into()).unwrap().needed.ceil(),  30_729f32);
        assert_eq!(tree.0.get(&37.into()).unwrap().needed.ceil(),   8_536f32);
        assert_eq!(tree.0.get(&38.into()).unwrap().needed.ceil(),   1_281f32);
        assert_eq!(tree.0.get(&39.into()).unwrap().needed.ceil(),     299f32);
        assert_eq!(tree.0.get(&40.into()).unwrap().needed.ceil(),     120f32);
    }

    #[test]
    fn warden() {
        let bp_bonuses = vec![
            BlueprintBonus {
                ptype_id: 11481.into(),
                material: 10f32,
                time:     20f32,
            },
            BlueprintBonus {
                ptype_id: 23559.into(),
                material: 10f32,
                time:     20f32,
            },
            BlueprintBonus {
                ptype_id: 28209.into(),
                material: 2f32,
                time:     4f32,
            },
        ];
        let blueprint_rig_bonus = RigBonus {
            type_id:    37180.into(),
            material:   2f32 * 2.1f32,
            time:       20f32 * 2.1f32,
            categories: vec![6, 32],
            groups:     Vec::new(),
            typ:        BonusTyp::Blueprint
        };
        let reaction_rig_bonus = RigBonus {
            type_id:    46497.into(),
            material:   2.4f32 * 1f32,
            time:       24f32 * 1f32,
            categories: Vec::new(),
            groups:     vec![428, 429, 712, 974, 4096],
            typ:        BonusTyp::Reaction
        };
        let blueprint_bonus = StructureBonus {
            type_id:    35827.into(),
            material:   1f32,
            time:       25f32,
            rigs:       vec![blueprint_rig_bonus],
            categories: Vec::new(),
            groups:     Vec::new(),
            typ:        BonusTyp::Blueprint
        };
        let reaction_bonus = StructureBonus {
            type_id:    35827.into(),
            material:   1f32,
            time:       25f32,
            rigs:       vec![reaction_rig_bonus],
            categories: Vec::new(),
            groups:     Vec::new(),
            typ:        BonusTyp::Reaction
        };
        let bonus = Bonus::new(
            bp_bonuses,
            vec![blueprint_bonus, reaction_bonus],
        );

        let file = std::fs::File::open("./testdata/warden.json").unwrap();
        let parsed: Dependency = serde_json::from_reader(file).unwrap();
        let mut tree = DependencyGroup(vec![parsed]).flat_tree();

        tree.calculate();
        bonus.apply_blueprint_bonus(&mut tree);
        bonus.apply_structure_bonus(&mut tree);

        // Current problem, instead of 100 moon goo only has 99 which breaks the rest

        dbg!(tree.0.into_iter().map(|(_, x)| (x.ptype_id, x.name, x.needed, x.typ)).collect::<Vec<_>>());
    }
}
