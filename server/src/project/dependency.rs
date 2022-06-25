mod cache;
mod group;

#[cfg(test)]
mod test_utils;

use std::collections::{HashMap, HashSet};

use caph_connector::CategoryId;
use caph_connector::GroupId;
use caph_connector::TypeId;
use serde::Deserialize;
use serde::Serialize;

pub use self::cache::*;
pub use self::group::*;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use crate::ContainerId;

/// Different types a dependency can be
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Eq, Serialize)]
pub enum DependencyType {
    /// The dependency is a blueprint
    Blueprint,
    /// The dependency is a reaction
    Reaction,
    /// The dependency is a material
    Material,
}

impl DependencyType {
    /// Constructes the enum from a bool.
    /// 
    /// # Params
    /// 
    /// * `is_reaction` -> Determines what type the dependency is
    /// 
    /// # Returns
    /// 
    /// - [DependencyType::Blueprint] if input is false
    /// - [DependencyType::Reaction]  if input is true
    /// 
    pub fn from_is_reaction(is_reaction: bool) -> Self {
        match is_reaction {
            false => DependencyType::Blueprint,
            true  => DependencyType::Reaction,
        }
    }

    /// Returns true if the type is blueprint
    pub fn is_blueprint(&self) -> bool {
        match self {
            DependencyType::Blueprint => true,
            _                         => false
        }
    }

    /// Returns true if the type is material
    pub fn is_material(&self) -> bool {
        match self {
            DependencyType::Material => true,
            _                        => false
        }
    }

    /// Returns true if the type is reaction
    pub fn is_reaction(&self) -> bool {
        match self {
            DependencyType::Reaction => true,
            _                        => false
        }
    }
}

/// Represents a single dependency and its required materials
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct Dependency {
    /// Name of the product
    pub name:              String,
    /// Name of the bluepritn
    pub blueprint_name:    String,
    /// [TypeId] of the blueprint
    pub btype_id:          TypeId,
    /// [TypeId] of the product
    pub ptype_id:          TypeId,
    /// [CategoryId] of the item
    pub category_id:       CategoryId,
    /// [GroupId] of the item
    pub group_id:          GroupId,

    /// Number of products to produce
    pub products:          u32,
    /// Base requirements for building a single run
    pub products_base:     u32,
    /// Quantity that is produced with each iteration
    pub products_per_run:  u32,
    /// Total time it takes for all runs
    pub time:              u32,
    /// Time it takes to run one production cycle
    pub time_per_run:      u32,

    /// Type of the dependency
    pub dependency_type:   DependencyType,

    /// Materials that are required to build this component
    pub components:        Vec<Dependency>,
}

impl Dependency {
    /// Creates a new dependency instance using the given cache.
    /// 
    /// # Params
    /// 
    /// * `ptype_id` -> [TypeId] of the product
    /// * `cache`    -> Cache that holds all blueprints and products
    /// 
    /// # Returns
    /// 
    /// New dependency instance
    /// 
    #[deprecated]
    pub fn from_cache(
        cache:    &DependencyCache,
        ptype_id: TypeId,
    ) -> Self {
        let mut root = cache.get(&ptype_id).unwrap().clone();

        for material in root.components.iter_mut() {
            Self::component_from_cache(
                cache,
                material.ptype_id,
                material
            );
        }

        root
    }

    pub(crate) fn from_database_dependency(
        dependency: DatabaseDependency,
    ) -> Self {
        Dependency {
            name:              dependency.product_name,
            blueprint_name:    dependency.blueprint_name,
            btype_id:          dependency.btype_id,
            ptype_id:          dependency.ptype_id,
            category_id:       dependency.category_id.into(),
            group_id:          dependency.group_id.into(),

            products:          dependency.produces,
            products_base:     dependency.quantity,
            products_per_run:  dependency.produces,
            time:              dependency.time,
            time_per_run:      dependency.time,

            dependency_type:   dependency.typ,

            components:        dependency
                                    .components
                                    .into_iter()
                                    .map(Dependency::from_database_dependency)
                                    .collect::<Vec<_>>(),
        }
    }

    pub(crate) fn with_dependencies(
        deps: Vec<Dependency>
    ) -> Self {
        Self {
            name:             String::new(),
            blueprint_name:   String::new(),
            btype_id:         0.into(),
            ptype_id:         0.into(),
            category_id:      0.into(),
            group_id:         0.into(),
            products:         0,
            products_base:    0,
            products_per_run: 0,
            time:             0,
            time_per_run:     0,
            dependency_type:  DependencyType::Material,
            components:       deps
        }
    }

    /// Creates a new component dependency.
    /// 
    /// # Params
    /// 
    /// * `cache`    -> Cache that holds all blueprints and products
    /// * `ptype_id` -> [TypeId] of the component
    /// * `material` -> [Dependency] to modify
    /// 
    pub fn component_from_cache(
        cache:    &DependencyCache,
        ptype_id: TypeId,
        material: &mut Dependency
    ) {
        if let Some(x) = cache.get(&ptype_id) {
            material.products_per_run = x.products_per_run;
            material.components = x.components.clone();
            material.components.sort_by_key(|x| x.ptype_id);

            for material in material.components.iter_mut() {
                Self::component_from_cache(
                    cache,
                    material.ptype_id,
                    material
                );
            }
        }
    }

    /// Adds the given amount to the total number of products.
    /// 
    /// Recalculates the dependencies afterwards.
    /// 
    /// # Params
    /// 
    /// * `amount` -> Required amount that should be added
    /// 
    pub fn add_product_quantity(
        &mut self,
        amount: u32
    ) {
        self.products += amount;

        let runs = (
            self.products as f64 / self.products_per_run as f64
        ).ceil() as u32;

        self.time = runs * self.time_per_run;

        // Recalculate dependencies
        for material in self.components.iter_mut() {
            material.recalculate_runs(runs);
        }
    }

    /// Sets the given amount to the total number of products.
    /// 
    /// Recalculates the dependencies afterwards.
    /// 
    /// # Params
    /// 
    /// * `amount` -> Required amount that should be added
    /// 
    pub fn set_product_quantity(
        &mut self,
        amount: u32,
    ) {
        self.products = amount;

        let runs = (
            self.products as f64 / self.products_per_run as f64
        ).ceil() as u32;

        self.time = runs * self.time_per_run;

        // Recalculate dependencies
        for material in self.components.iter_mut() {
            material.recalculate_runs(runs);
        }
    }

    /// Propagates the number of runs down from the end product and recalculates
    /// the number of required runs for all components
    /// 
    /// # Params
    /// 
    /// * `runs` -> Number of runs that the parent requires
    /// 
    pub fn recalculate_runs(
        &mut self,
        runs: u32
    ) {
        self.time     = runs * self.time_per_run;
        self.products = runs * self.products_base;

        let runs = (
            self.products as f64 / self.products_per_run as f64
        ).ceil() as u32;

        // Recalculates dependencies
        for material in self.components.iter_mut() {
            material.recalculate_runs(runs);
        }
    }

    /// Calculates the number of runs required
    /// 
    pub fn runs(
        &self
    ) -> u32 {
        (
            self.products as f64 / self.products_per_run as f64
        ).ceil() as u32
    }

    /// Collects all required materials into a single map, mapping the product
    /// [TypeId] to the amount that is required
    /// 
    pub fn collect_raw_materials(
        &self
    ) -> DependencyGroup {
        let mut result = DependencyGroup::default();

        for material in self.components.iter() {
            if material.components.is_empty() {
                result.add(material.clone());
            } else {
                let dependencies = material.collect_raw_materials();
                result.merge(dependencies);
            }
        }

        result
    }

    /// Collects all required components into a single map, mapping the product
    /// [TypeId] to the amount that is required
    /// 
    pub fn collect_components(
        &self
    ) -> DependencyGroup {
        let mut result = DependencyGroup::default();
        // add our self to include the final product
        result.add(self.clone());

        // go through all components
        for material in self.components.iter() {
            if material.components.is_empty() {
                continue;
            }

            let dependencies = material.collect_components();
            result.merge(dependencies);
        }

        result
    }

    /// Collects all product [TypeId]s that are required to build the dependency.
    /// Includes the product [TypeId] of the dependency itself.
    /// 
    /// # Returns
    /// 
    /// List of [TypeId]s
    /// 
    pub fn collect_ptype_ids(
        &self
    ) -> Vec<TypeId> {
        let mut ptype_ids = self.components
            .iter()
            .map(|x| x.collect_ptype_ids())
            .flatten()
            .collect::<HashSet<_>>();
        ptype_ids.insert(self.ptype_id);

        ptype_ids
            .into_iter()
            .collect::<Vec<_>>()
    }

    /// Adds a material bonus to all required materials.
    /// 
    /// Limitations: This must be done AFTER the required amount of items is
    ///              added
    /// 
    /// # Params
    /// 
    /// * `bonus` -> Maps product [TypeId] to material efficiency
    /// 
    pub fn apply_material_bonus(
        &mut self,
        bonus: &HashMap<TypeId, u8>
    ) {
        if let Some(b) = bonus.get(&self.ptype_id) {
            for material in self.components.iter_mut() {
                if material.products >= 10 {
                    material.products = material.products - (
                        (material.products as f64 * (*b as f64 / 100f64)).floor()
                    ).round() as u32;
                }

                material.apply_material_bonus(&bonus);
            }
        } else {
            for material in self.components.iter_mut() {
                material.apply_material_bonus(&bonus);
            }
        }
    }

    /// Applies time bonuses.
    /// 
    /// Limitations: This must be done AFTER the required amount of items is
    ///              added
    /// 
    /// # Params
    /// 
    /// * `bonus` -> Maps product [TypeId] to time efficiency
    /// 
    pub fn apply_time_bonus(
        &mut self,
        bonus: &HashMap<TypeId, u8>
    ) {
        if let Some(b) = bonus.get(&self.ptype_id) {
            for material in self.components.iter_mut() {
                let runs = (
                    self.products as f64 / self.products_per_run as f64
                ).ceil() as u32;
                let runs = if runs == 0 { 1 } else { runs };

                material.time_per_run = runs * material.time_per_run - (
                    runs as f64 * material.time_per_run as f64 * (*b as f64 / 100f64)
                ).round() as u32;

                material.apply_time_bonus(&bonus);
            }
        } else {
            for material in self.components.iter_mut() {
                material.apply_time_bonus(&bonus);
            }
        }
    }

    pub fn has_diff(
        &self,
        old: Dependency
    ) -> bool {
        let mut result = false;

        if !self.components.is_empty() {
            if self.products != old.products {
                return true;
            }

            self.components
                .iter()
                .zip(old.components)
                .for_each(|(new, old)| {
                    if new.has_diff(old.clone()) {
                        result = true;
                    }
                });
        }

        result
    }

    /// Sorts the dependencies by their prequisite
    /// 
    pub fn sort(
        &mut self,
        corrections: HashMap<TypeId, Dependency>
    ) {
        let mut empty = HashSet::new();
        let mut sorted = self.inner_sort(&mut empty);

        for sort in sorted.iter_mut() {
            if let Some(x) = corrections.get(&sort.ptype_id) {
                sort.products = x.products;
                sort.components = x.components.clone();
            }
        }

        self.components = sorted.into_iter().collect::<Vec<_>>();
    }

    fn inner_sort(
        &self,
        seen: &mut HashSet<TypeId>
    ) -> Vec<Dependency> {
        let mut inverted = Vec::new();
        for dep in self.components.iter() {
            if seen.contains(&dep.ptype_id) {
                continue;
            } else if dep.components.len() == 0 {
                continue;
            } else {
                seen.insert(dep.ptype_id);
                inverted.extend(dep.inner_sort(seen));
                inverted.push(dep.clone());
            }
        }

        inverted
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseDependency {
    pub btype_id:       TypeId,
    pub blueprint_name: String,
    pub ptype_id:       TypeId,
    pub category_id:    u32,
    pub group_id:       u32,
    pub product_name:   String,
    pub time:           u32,
    pub quantity:       u32,
    pub produces:       u32,
    pub typ:            DependencyType,
    pub components:     Vec<DatabaseDependency>,
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use sqlx::postgres::PgPoolOptions;

    use crate::project::dependency::test_utils::fuel_block;

    use super::*;

    #[test]
    #[ignore = "Testdata is wrong"]
    fn add_runs() {
        let mut dep = dependency_titanium_chromide();

        dep.add_product_quantity(100);
        assert_eq!(dep.runs(), 1);
        assert_eq!(dep.components[0].products, 5);

        dep.add_product_quantity(100);
        assert_eq!(dep.runs(), 1);
        assert_eq!(dep.components[0].products, 5);

        dep.add_product_quantity(100);
        assert_eq!(dep.runs(), 2);
        assert_eq!(dep.components[0].products, 10);

        dep.add_product_quantity(1300);
        assert_eq!(dep.runs(), 8);
        assert_eq!(dep.components[0].products, 40);

        dep.add_product_quantity(100);
        assert_eq!(dep.runs(), 9);
        assert_eq!(dep.components[0].products, 45);
        assert_eq!(dep.components[0].runs(), 2);
    }

    #[test]
    fn apply_material_bonus_fuel_blocks() {
        let mut dep = dependency_fuel_block();

        let mut bonus: HashMap<TypeId, u8> = HashMap::new();
        bonus.insert(4312.into(), 10);

        assert_eq!(dep.components[8].ptype_id, 17887.into());
        assert_eq!(dep.components[8].products, 450);

        dep.apply_material_bonus(&bonus);

        assert_eq!(dep.components[8].ptype_id, 17887.into());
        assert_eq!(dep.components[8].products, 405);
    }

    #[test]
    fn apply_material_bonus_fuel_blocks_add_structure_bonus() {
        let mut dep = dependency_fuel_block();
        dep.add_product_quantity(40);

        let mut bonus: HashMap<TypeId, u8> = HashMap::new();
        bonus.insert(4312.into(), 10);

        assert_eq!(dep.components[8].ptype_id, 17887.into());
        assert_eq!(dep.components[8].products, 450);

        dep.apply_material_bonus(&bonus);
        assert_eq!(dep.components[8].ptype_id, 17887.into());
        assert_eq!(dep.components[8].products, 405);

        let mut bonus: HashMap<TypeId, u8> = HashMap::new();
        bonus.insert(4312.into(), 1);

        dep.apply_material_bonus(&bonus);
        assert_eq!(dep.components[8].ptype_id, 17887.into());
        assert_eq!(dep.components[8].products, 401);
    }

    #[test]
    #[ignore = "Testdata is wrong"]
    fn apply_material_bonus_titanium_chromide() {
        let mut dep = dependency_titanium_chromide();
        dep.add_product_quantity(200);

        let mut bonus: HashMap<TypeId, u8> = HashMap::new();
        bonus.insert(4312.into(), 10);

        assert_eq!(dep.components[0].ptype_id, 4312.into());
        assert_eq!(dep.components[0].components[8].ptype_id, 17887.into());
        assert_eq!(dep.components[0].components[8].products, 450);

        dep.apply_material_bonus(&bonus);

        assert_eq!(dep.components[0].ptype_id, 4312.into());
        assert_eq!(dep.components[0].components[8].ptype_id, 17887.into());
        assert_eq!(dep.components[0].components[8].products, 405);
    }

    #[tokio::test]
    #[ignore = "Testdata is wrong"]
    async fn build_titanium_chromide_from_cache() {
        let cache = cache_instance().await;

        let a = dependency_titanium_chromide();
        let mut b = Dependency::from_cache(&cache, 16654.into());
        b.components.sort_by_key(|x| x.ptype_id);

        assert_eq!(a, b);
    }

    #[tokio::test]
    #[ignore = "Testdata is wrong"]
    async fn build_fullerides_from_cache() {
        let cache = cache_instance().await;

        let a = dependency_fullerides();
        let mut b = Dependency::from_cache(&cache, 16679.into());
        b.components.sort_by_key(|x| x.ptype_id);

        assert_eq!(a, b);
    }

    async fn cache_instance() -> DependencyCache {
        dotenv::dotenv().ok();

        let pg_addr = std::env::var("DATABASE_URL")
            .unwrap();
        let pool = PgPoolOptions::new()
            .connect(&pg_addr)
            .await
            .unwrap();

        DependencyCache::new(pool).await.unwrap()
    }

    fn dependency_fuel_block() -> Dependency {
        fuel_block("Oxygen Fuel Block".into(), 4312.into())
    }

    fn dependency_titanium_chromide() -> Dependency {
        Dependency {
            name:             "Titanium Chromide".into(),
            blueprint_name:   "Titanium Chromide Reaction Formula".into(),
            btype_id:         46182.into(),
            ptype_id:         16654.into(),
            category_id:      4.into(),
            group_id:         428.into(),
            products_per_run: 200,
            products:         0i64,
            products_base:    0i64,
            time:             10800i64,
            time_per_run:     10800i64,
            dependency_type:  DependencyType::Reaction,
            components:       vec![
                fuel_block("Oxygen Fuel Block".into(), 4312.into()),
                Dependency {
                    name:             "Titanium".into(),
                    blueprint_name:   "".into(),
                    btype_id:         0.into(),
                    ptype_id:         16638.into(),
                    category_id:      4.into(),
                    group_id:         427.into(),
                    products_per_run: 0i64,
                    products:         100i64,
                    products_base:    100i64,
                    time:             0i64,
                    time_per_run:     0i64,
                    dependency_type:  DependencyType::Material,
                    components:       Vec::new()
                },
                Dependency {
                    name:             "Chromium".into(),
                    blueprint_name:   "".into(),
                    btype_id:         0.into(),
                    ptype_id:         16641.into(),
                    category_id:      4.into(),
                    group_id:         427.into(),
                    products_per_run: 0i64,
                    products:         100i64,
                    products_base:    100i64,
                    time:             0i64,
                    time_per_run:     0i64,
                    dependency_type:  DependencyType::Material,
                    components:       Vec::new()
                }
            ]
        }
    }

    fn dependency_fullerides() -> Dependency {
        Dependency {
            name:             "Fullerides".into(),
            blueprint_name:   "Fulleride Reaction Formula".into(),
            btype_id:         46209.into(),
            ptype_id:         16679.into(),
            category_id:      4.into(),
            group_id:         429.into(),
            products_per_run: 3000,
            products:         0i64,
            products_base:    0i64,
            time:             10800i64,
            time_per_run:     10800i64,
            dependency_type:  DependencyType::Reaction,
            components:      vec![
                fuel_block("Nitrogen Fuel Block".into(), 4051.into()),
                Dependency {
                    name:             "Carbon Polymers".into(),
                    blueprint_name:   "Carbon Polymers Reaction Formula".into(),
                    btype_id:         46167.into(),
                    ptype_id:         16659.into(),
                    category_id:      4.into(),
                    group_id:         428.into(),
                    products_per_run: 200i64,
                    products:         100i64,
                    products_base:    100i64,
                    time:             10800i64,
                    time_per_run:     10800i64,
                    dependency_type:  DependencyType::Reaction,
                    components:       vec![
                        fuel_block("Helium Fuel Block".into(), 4247.into()),
                        Dependency {
                            name:             "Hydrocarbons".into(),
                            blueprint_name:   "".into(),
                            btype_id:         0.into(),
                            ptype_id:         16633.into(),
                            category_id:      4.into(),
                            group_id:         427.into(),
                            products_per_run: 0i64,
                            products:         100i64,
                            products_base:    100i64,
                            time:             0i64,
                            time_per_run:     0i64,
                            dependency_type:  DependencyType::Material,
                            components:       Vec::new()
                        },
                        Dependency {
                            name:             "Silicates".into(),
                            blueprint_name:   "".into(),
                            btype_id:         0.into(),
                            ptype_id:         16636.into(),
                            category_id:      4.into(),
                            group_id:         427.into(),
                            products_per_run: 0i64,
                            products:         100i64,
                            products_base:    100i64,
                            time:             0i64,
                            time_per_run:     0i64,
                            dependency_type:  DependencyType::Material,
                            components:       Vec::new()
                        },
                    ]
                },
                Dependency {
                    name:             "Platinum Technite".into(),
                    blueprint_name:   "Platinum Technite Reaction Formula".into(),
                    btype_id:         46177.into(),
                    ptype_id:         16662.into(),
                    category_id:      4.into(),
                    group_id:         428.into(),
                    products_per_run: 200i64,
                    products:         100i64,
                    products_base:    100i64,
                    time:             10800i64,
                    time_per_run:     10800i64,
                    dependency_type:  DependencyType::Reaction,
                    components:       vec![
                        fuel_block("Nitrogen Fuel Block".into(), 4051.into()),
                        Dependency {
                            name:             "Platinum".into(),
                            blueprint_name:   "".into(),
                            btype_id:         0.into(),
                            ptype_id:         16644.into(),
                            category_id:      4.into(),
                            group_id:         427.into(),
                            products_per_run: 0i64,
                            products:         100i64,
                            products_base:    100i64,
                            time:             0i64,
                            time_per_run:     0i64,
                            dependency_type:  DependencyType::Material,
                            components:       Vec::new()
                        },
                        Dependency {
                            name:             "Technetium".into(),
                            blueprint_name:   "".into(),
                            btype_id:         0.into(),
                            ptype_id:         16649.into(),
                            category_id:      4.into(),
                            group_id:         427.into(),
                            products_per_run: 0i64,
                            products:         100i64,
                            products_base:    100i64,
                            time:             0i64,
                            time_per_run:     0i64,
                            dependency_type:  DependencyType::Material,
                            components:       Vec::new()
                        },
                    ]
                }
            ]
        }
    }
}
