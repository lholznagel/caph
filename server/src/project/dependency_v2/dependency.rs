use caph_connector::TypeId;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

use crate::Error;
use crate::structure::{Structure, BonusVariations};
use uuid::Uuid;

/// Single dependency that represents either a end product, component or
/// material
/// 
#[derive(Clone, Debug, Deserialize)]
pub struct Dependency {
    btype_id:   TypeId,
    ptype_id:   TypeId,
    #[serde(rename = "quantity")]
    needed:     f32,
    produces:   u32,
    info:       DependencyInfo,
    components: Vec<Dependency>,
    typ:        BlueprintTyp,
}

impl Dependency {
    pub fn try_from(
        quantity: u32,
        value:    serde_json::Value
    ) -> Result<Self, Error> {
        let mut dependency: Dependency = serde_json::from_value(value)
            .map_err(Error::CouldNotParseJsonToDependency)?;
        dependency.needed = dependency.needed * quantity as f32;
        Ok(dependency)
    }
}

/// Group of dependencies.
/// 
#[derive(Clone, Debug, Default)]
pub struct DependencyTree {
    tree:        HashMap<TypeId, DependencyTreeEntry>,

    structures:  Vec<Structure>,
    mapping:     Vec<StructureMapping>,
    bp_override: HashMap<TypeId, BlueprintBonus>,
}

impl DependencyTree {
    /// Creates a new Dependency tree
    /// 
    /// # Params
    /// 
    /// * `structures`  > List of strutures that should be used for calculating
    ///                   bonuses
    /// * `mapping`     > Mapping from structure to item category
    /// * `bp_override` > Overrides that ME/TE bonuses for blueprints
    /// 
    pub fn new(
        structures:           Vec<Structure>,
        mapping:              Vec<StructureMapping>,
        bp_override: HashMap<TypeId, BlueprintBonus>,
    ) -> Self {
        Self {
            tree: HashMap::new(),

            structures,
            mapping,
            bp_override,
        }
    }

    /// Creates a new dependency tree from a list of [Dependency]
    /// 
    /// # Params
    /// 
    /// * `structures`  > List of strutures that should be used for calculating
    ///                   bonuses
    /// * `mapping`     > Mapping from structure to item category
    /// * `bp_override` > Overrides that ME/TE bonuses for blueprints
    /// 
    pub fn from_dependencies(
        dependencies:         Vec<Dependency>,
        structures:           Vec<Structure>,
        mapping:              Vec<StructureMapping>,
        blueprint_overwrites: HashMap<TypeId, BlueprintBonus>,
    ) -> Self {
        let mut dp_group = Self::new(
            structures,
            mapping,
            blueprint_overwrites
        );

        for dependency in dependencies {
            dp_group.add(dependency);
        }

        dp_group
    }

    /// Adds the given [Dependency] to the tree.
    /// 
    /// # Params
    /// 
    /// * `dependency` > Dependency to add to the tree
    /// 
    pub fn add(
        &mut self,
        dependency: Dependency,
    ) -> &mut Self {
        // TODO: make it configurable
        // Skip bulding fuel blocks
        let skip = vec![4051, 4246, 4247, 4312];

        let mut queue: VecDeque<Dependency> = vec![dependency].into();

        while let Some(dep) = queue.pop_front() {
            self.add_to_tree(dep.clone());

            // Skip if the ptype_id is in the ignore list
            for component in dep.components.iter() {
                if skip.contains(&dep.ptype_id) {
                    continue;
                }

                queue.push_back(component.clone());
            }
        }

        self.full_calculation();
        self
    }

    /// Recalculates the whole tree
    pub fn full_calculation(
        &mut self,
    ) {
        let queue = self.tree
            .keys()
            .cloned()
            .collect::<VecDeque<_>>();
        self.calculate(queue);
    }

    /// Only recalculates everything that was changed by updating the given
    /// [TypeId]
    pub fn partial_calculation(
        &mut self,
        ptype_id: TypeId,
    ) {
        self.calculate(vec![ptype_id].into())
    }

    pub fn product_type_ids(
        &mut self,
    ) -> Vec<TypeId> {
        self.tree
            .iter()
            .filter(|(_, x)| x.typ != BlueprintTyp::Reaction)
            .map(|(_, x)| x.ptype_id)
            .collect::<Vec<_>>()
    }

    pub fn apply_bonus(
        &mut self,
    ) -> HashMap<TypeId, DependencyTreeEntry> {
        for ptype_id in self.product_type_ids().iter() {
            let me_bonus = if let Some(x) = self
                                                .bp_override
                                                .get(ptype_id) {
                x.material
            } else {
                10f32
            };

            self.apply_me_bonus(
                *ptype_id,
                me_bonus,
            );
            self.partial_calculation(*ptype_id);
        }

        self.apply_by_bonus_type(BlueprintTyp::Blueprint);
        self.apply_by_bonus_type(BlueprintTyp::Reaction);

        std::mem::take(&mut self.tree)
    }

    /// Adds the given [Dependency] to the tree.
    /// 
    /// # Params
    /// 
    /// * `dependency` > Dependency to add to the tree
    /// 
    fn add_to_tree(
        &mut self,
        dep: Dependency,
    ) {
        // Collect all children and the require amount
        let children = dep
            .components
            .into_iter()
            .map(|x| (x.ptype_id, x.needed))
            .collect::<HashMap<_, _>>();

        // Either insert or edit the ptype_id entry
        self.tree
            .entry(dep.ptype_id.clone())
            .and_modify(|x: &mut DependencyTreeEntry| x.needed += dep.needed)
            .or_insert(DependencyTreeEntry {
                btype_id: dep.btype_id,
                ptype_id: dep.ptype_id,
                name:     dep.info.name.clone(),
                needed:   dep.needed,
                produces: dep.produces,
                children: children,
                typ:      dep.typ,
                info:     dep.info,
            });
    }

    /// Calculates how many resources from every item is required.
    /// 
    /// The values are updated in place.
    /// 
    fn calculate(
        &mut self,
        queue: VecDeque<TypeId>,
    ) {
        let timer = std::time::Instant::now();
        let mut queue = queue.clone();

        while let Some(ptype_id) = queue.pop_front() {
            // Add all children to the queue
            if let Some(x) = self.tree.get(&ptype_id) {
                let entries = x.children.keys().collect::<Vec<_>>();
                queue.extend(entries);
            }

            let mut grouped = std::collections::HashMap::new();
            // TODO: make it configurable
            let skip = vec![4051, 4246, 4247, 4312];

            // Get all items that have the ptype_id as a children, calculate the
            // number of runs and collect them together
            self.tree
                .iter()
                .filter(|(_, e)| e.children.contains_key(&ptype_id))
                .for_each(|(p, e)| {
                    if !skip.contains(&**p) {
                        let runs = (e.needed / e.produces as f32).ceil();
                        let per_run = e.children.get(&ptype_id).unwrap_or(&0f32);
                        let per_run = runs * per_run;

                        grouped
                            .entry(p.clone())
                            .and_modify(|x: &mut f32| *x += per_run)
                            .or_insert(per_run);
                    }
                });

            // Our product, will not be in any children
            if grouped.is_empty() {
                continue;
            }

            if let Some(x) = self.tree.get_mut(&ptype_id) {
                let a = grouped
                    .into_iter()
                    .map(|(_, x)| x.ceil())
                    .sum();

                x.needed = a;
            }
        }
        dbg!(timer.elapsed().as_millis());
    }

    fn apply_by_bonus_type(
        &mut self,
        bonus_type: BlueprintTyp
    ) -> &mut Self {
        let tree_clone = self.tree.clone();
        let structures_clone = self.structures.clone();
        let blueprints = tree_clone
            .iter()
            .filter(|(_, e)| e.typ == bonus_type)
            .collect::<Vec<_>>();

        for (_, blueprint) in blueprints {
            // TODO: improve mapping, so that its not dermined by what groups
            //       it bonuses by rig but mapped by the user
            let structure = self.mapping
                .iter()
                .find(|x|
                    x.category_group.contains(&blueprint.info.category_id) ||
                    x.category_group.contains(&blueprint.info.group_id)
                );
            let structure = if let Some(s) = structure {
                // TODO: replace with hashmap
                structures_clone
                    .iter()
                    .find(|x| x.id == s.structure)
                    .unwrap()
            } else {
                continue;
            };

            let rig = structure
                .rigs()
                .iter()
                .find(|x|
                    x.has_category_or_group(blueprint.info.category_id) ||
                    x.has_category_or_group(blueprint.info.group_id)
                )
                .map(|x| (x.material, x.time));

            if let Some((Some(me), _)) = rig {
                self.apply_me_bonus(
                    blueprint.ptype_id,
                    me,
                );
                self.partial_calculation(blueprint.ptype_id);
            }

            if let Some(BonusVariations::Material(me)) = structure
                .structure
                .bonus()
                .iter()
                .find(|x| {
                    match x {
                        BonusVariations::Material(_) => true,
                        _                            => false
                    }
                }) {

                self.apply_me_bonus(
                    blueprint.ptype_id,
                    *me,
                );
                self.partial_calculation(blueprint.ptype_id);
            }
        }
        self
    }

    fn apply_me_bonus(
        &mut self,
        ptype_id: TypeId,
        bonus:    f32,
    ) {
        let entry = if let Some(x) = self.tree.get_mut(&ptype_id) {
            x
        } else {
            return;
        };

        for (_, base_val) in entry.children.iter_mut() {
            // Don´t apply bonuses if only one item is required
            if *base_val == 1f32 {
                continue;
            }

            let modifier = *base_val * (bonus as f32 / 100f32);
            *base_val = *base_val - modifier;
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DependencyInfo {
    pub name:        String,
    pub category_id: usize,
    pub group_id:    usize,
}

#[derive(Clone, Debug, Serialize)]
pub struct DependencyTreeEntry {
    pub btype_id:      TypeId,
    pub ptype_id:      TypeId,
    pub name:          String,
    pub needed:        f32,
    pub produces:      u32,
    pub children:      HashMap<TypeId, f32>,
    pub typ:           BlueprintTyp,
    pub info:          DependencyInfo,
}

#[derive(Copy, Clone, Debug)]
pub struct BlueprintBonus {
    pub ptype_id: TypeId,
    pub material: f32,
    pub time:     f32,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum BlueprintTyp {
    Blueprint,
    Reaction,
    Material,
}

#[derive(Clone, Debug)]
pub struct StructureMapping {
    pub structure:      Uuid,
    pub category_group: Vec<usize>,
}


#[cfg(test)]
mod dependency_tests {
    use sqlx::PgPool;
    use sqlx::postgres::PgPoolOptions;
    use std::collections::HashMap;
    use std::str::FromStr;
    use uuid::Uuid;

    use super::*;
    use crate::structure::structure::*;

    async fn db_pool() -> PgPool {
        dotenvy::dotenv().ok();
        let pg_addr = std::env::var("DATABASE_URL").unwrap();
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&pg_addr)
            .await
            .unwrap();
        pool
    }

    async fn dependency_group(
        bp_overwrite: HashMap<TypeId, BlueprintBonus>
    ) -> DependencyTree {
        let pool = db_pool().await;

        let manufacturing_a = Structure::new(
            Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap(),
            "Sotiyo manufacturing".into(),
            "Here".into(),
            Security::Nullsec,
            StructureType::Sotiyo,
            vec![
                StructureRig::new(&pool, TypeId::from(37180)).await.unwrap(),
                StructureRig::new(&pool, TypeId::from(37178)).await.unwrap(),
                StructureRig::new(&pool, TypeId::from(43704)).await.unwrap(),
            ]
        );

        let reaction_a = Structure::new(
            Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap(),
            "Tatara reactions".into(),
            "Here".into(),
            Security::Nullsec,
            StructureType::Tatara,
            vec![
                StructureRig::new(&pool, TypeId::from(46497)).await.unwrap(),
            ]
        );

        let mapping = vec![
            StructureMapping {
                structure:      manufacturing_a.id,
                category_group: manufacturing_a.category_groups(),
            },
            StructureMapping {
                structure:      reaction_a.id,
                category_group: reaction_a.category_groups(),
            },
        ];

        DependencyTree::new(
            vec![manufacturing_a, reaction_a],
            mapping,
            bp_overwrite,
        )
    }

    #[tokio::test]
    async fn warden() {
        let mut bp_overwrite = HashMap::new();
        bp_overwrite.insert(28209.into(), BlueprintBonus {
            ptype_id: 28209.into(),
            material: 2f32,
            time:     4f32,
        });

        let file = std::fs::File::open("./testdata/warden.json").unwrap();
        let parsed: Dependency = serde_json::from_reader(file).unwrap();

        let tree = dependency_group(bp_overwrite)
            .await
            .add(parsed)
            .apply_bonus();

        assert_eq!(tree.get(&16633.into()).unwrap().needed.ceil(),  98f32);
        assert_eq!(tree.get(&16635.into()).unwrap().needed.ceil(),  98f32);
        assert_eq!(tree.get(&16636.into()).unwrap().needed.ceil(), 196f32);
        assert_eq!(tree.get(&16638.into()).unwrap().needed.ceil(),  98f32);
        assert_eq!(tree.get(&16641.into()).unwrap().needed.ceil(), 196f32);
        assert_eq!(tree.get(&16642.into()).unwrap().needed.ceil(),  98f32);
        assert_eq!(tree.get(&16644.into()).unwrap().needed.ceil(),  98f32);
        assert_eq!(tree.get(&16646.into()).unwrap().needed.ceil(),  98f32);
        assert_eq!(tree.get(&16647.into()).unwrap().needed.ceil(),  98f32);
        assert_eq!(tree.get(&16648.into()).unwrap().needed.ceil(),  98f32);
        assert_eq!(tree.get(&16649.into()).unwrap().needed.ceil(),  98f32);
        assert_eq!(tree.get(&16650.into()).unwrap().needed.ceil(),  98f32);

        assert_eq!(tree.get(&34.into()).unwrap().needed.ceil(),     916f32);
        assert_eq!(tree.get(&35.into()).unwrap().needed.ceil(),    8855f32);
        assert_eq!(tree.get(&36.into()).unwrap().needed.ceil(),     190f32);
        assert_eq!(tree.get(&37.into()).unwrap().needed.ceil(),      70f32);
        assert_eq!(tree.get(&38.into()).unwrap().needed.ceil(),      35f32);
        assert_eq!(tree.get(&40.into()).unwrap().needed.ceil(),      16f32);
        assert_eq!(tree.get(&11399.into()).unwrap().needed.ceil(),    5f32);

        assert_eq!(tree.get(&4051.into()).unwrap().needed.ceil(), 10f32);
        assert_eq!(tree.get(&4246.into()).unwrap().needed.ceil(),  5f32);
        assert_eq!(tree.get(&4247.into()).unwrap().needed.ceil(), 10f32);
        assert_eq!(tree.get(&4312.into()).unwrap().needed.ceil(), 25f32);
    }

    #[tokio::test]
    async fn naglfar() {
        let file = std::fs::File::open("./testdata/naglfar.json").unwrap();
        let parsed: Dependency = serde_json::from_reader(file).unwrap();

        let tree = dependency_group(HashMap::new())
            .await
            .add(parsed)
            .apply_bonus();

        // Booster Gas Clouds
        assert_eq!(tree.get(&25278.into()).unwrap().needed.ceil(),       20f32);
        assert_eq!(tree.get(&25279.into()).unwrap().needed.ceil(),       20f32);
        assert_eq!(tree.get(&28694.into()).unwrap().needed.ceil(),      312f32);
        assert_eq!(tree.get(&28695.into()).unwrap().needed.ceil(),     1247f32);
        assert_eq!(tree.get(&28696.into()).unwrap().needed.ceil(),      312f32);
        assert_eq!(tree.get(&28697.into()).unwrap().needed.ceil(),      312f32);
        assert_eq!(tree.get(&28698.into()).unwrap().needed.ceil(),      312f32);
        assert_eq!(tree.get(&28699.into()).unwrap().needed.ceil(),      312f32);
        assert_eq!(tree.get(&28700.into()).unwrap().needed.ceil(),     1247f32);
        assert_eq!(tree.get(&28701.into()).unwrap().needed.ceil(),      312f32);

        // Fullerenes
        assert_eq!(tree.get(&30370.into()).unwrap().needed.ceil(),    11685f32);
        assert_eq!(tree.get(&30371.into()).unwrap().needed.ceil(),    11879f32);
        assert_eq!(tree.get(&30372.into()).unwrap().needed.ceil(),    12172f32);
        assert_eq!(tree.get(&30373.into()).unwrap().needed.ceil(),    11198f32);
        assert_eq!(tree.get(&30374.into()).unwrap().needed.ceil(),    11685f32);
        assert_eq!(tree.get(&30375.into()).unwrap().needed.ceil(),    11976f32);
        assert_eq!(tree.get(&30376.into()).unwrap().needed.ceil(),     1656f32);
        assert_eq!(tree.get(&30377.into()).unwrap().needed.ceil(),      585f32);
        assert_eq!(tree.get(&30378.into()).unwrap().needed.ceil(),      585f32);

        // Raw Moon Materials
        assert_eq!(tree.get(&16633.into()).unwrap().needed.ceil(),    86846f32);
        assert_eq!(tree.get(&16634.into()).unwrap().needed.ceil(),    86749f32);
        assert_eq!(tree.get(&16635.into()).unwrap().needed.ceil(),    28529f32);
        assert_eq!(tree.get(&16636.into()).unwrap().needed.ceil(),    28626f32);
        assert_eq!(tree.get(&16639.into()).unwrap().needed.ceil(),      293f32);
        assert_eq!(tree.get(&16642.into()).unwrap().needed.ceil(),      391f32);
        assert_eq!(tree.get(&16643.into()).unwrap().needed.ceil(),      196f32);
        assert_eq!(tree.get(&16644.into()).unwrap().needed.ceil(),       98f32);
        assert_eq!(tree.get(&16646.into()).unwrap().needed.ceil(),      391f32);
        assert_eq!(tree.get(&16647.into()).unwrap().needed.ceil(),       98f32);
        assert_eq!(tree.get(&16648.into()).unwrap().needed.ceil(),       98f32);
        assert_eq!(tree.get(&16649.into()).unwrap().needed.ceil(),       98f32);
        assert_eq!(tree.get(&16650.into()).unwrap().needed.ceil(),       98f32);
        assert_eq!(tree.get(&16651.into()).unwrap().needed.ceil(),      391f32);
        assert_eq!(tree.get(&16652.into()).unwrap().needed.ceil(),       98f32);
        assert_eq!(tree.get(&16653.into()).unwrap().needed.ceil(),       98f32);

        // Minerals
        assert_eq!(tree.get(&34.into()).unwrap().needed.ceil(),     3568889f32);
        assert_eq!(tree.get(&35.into()).unwrap().needed.ceil(),    10569714f32);
        assert_eq!(tree.get(&36.into()).unwrap().needed.ceil(),     2976841f32);
        assert_eq!(tree.get(&37.into()).unwrap().needed.ceil(),      816905f32);
        assert_eq!(tree.get(&38.into()).unwrap().needed.ceil(),       88037f32);
        assert_eq!(tree.get(&39.into()).unwrap().needed.ceil(),       41749f32);
        assert_eq!(tree.get(&40.into()).unwrap().needed.ceil(),       20884f32);
        assert_eq!(tree.get(&11399.into()).unwrap().needed.ceil(),     1281f32);

        // Fuel Blocks
        assert_eq!(tree.get(&4051.into()).unwrap().needed.ceil(),       855f32);
        assert_eq!(tree.get(&4246.into()).unwrap().needed.ceil(),      1202f32);
        assert_eq!(tree.get(&4247.into()).unwrap().needed.ceil(),       831f32);
        assert_eq!(tree.get(&4312.into()).unwrap().needed.ceil(),      1045f32);

        // PI
        assert_eq!(tree.get(&3645.into()).unwrap().needed.ceil(),     12421f32);
        assert_eq!(tree.get(&3683.into()).unwrap().needed.ceil(),      9731f32);
        assert_eq!(tree.get(&9842.into()).unwrap().needed.ceil(),        86f32);
        assert_eq!(tree.get(&2319.into()).unwrap().needed.ceil(),       427f32);
        assert_eq!(tree.get(&2329.into()).unwrap().needed.ceil(),       427f32);
        assert_eq!(tree.get(&2346.into()).unwrap().needed.ceil(),       342f32);
        assert_eq!(tree.get(&2348.into()).unwrap().needed.ceil(),       257f32);
        assert_eq!(tree.get(&2867.into()).unwrap().needed.ceil(),         6f32);
        assert_eq!(tree.get(&2868.into()).unwrap().needed.ceil(),        91f32);
        assert_eq!(tree.get(&2870.into()).unwrap().needed.ceil(),        13f32);
        assert_eq!(tree.get(&2871.into()).unwrap().needed.ceil(),         8f32);
        assert_eq!(tree.get(&2872.into()).unwrap().needed.ceil(),        97f32);
        assert_eq!(tree.get(&2361.into()).unwrap().needed.ceil(),        86f32);
        assert_eq!(tree.get(&2876.into()).unwrap().needed.ceil(),        40f32);
        assert_eq!(tree.get(&2393.into()).unwrap().needed.ceil(),      7785f32);
        assert_eq!(tree.get(&2395.into()).unwrap().needed.ceil(),      3893f32);
        assert_eq!(tree.get(&2401.into()).unwrap().needed.ceil(),      5122f32);
        assert_eq!(tree.get(&2463.into()).unwrap().needed.ceil(),       427f32);

        // Commodities
        assert_eq!(tree.get(&57443.into()).unwrap().needed.ceil(),        1f32);
        assert_eq!(tree.get(&57445.into()).unwrap().needed.ceil(),        8f32);
        assert_eq!(tree.get(&57446.into()).unwrap().needed.ceil(),        8f32);
        assert_eq!(tree.get(&57447.into()).unwrap().needed.ceil(),        8f32);
        assert_eq!(tree.get(&57448.into()).unwrap().needed.ceil(),       34f32);
        assert_eq!(tree.get(&57450.into()).unwrap().needed.ceil(),        1f32);
        assert_eq!(tree.get(&57452.into()).unwrap().needed.ceil(),       66f32);
    }
}
