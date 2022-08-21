mod database;
mod group;
mod typ;

use caph_connector::{CategoryId, GroupId, TypeId};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub use self::database::*;
pub use self::group::*;
pub use self::typ::*;
use crate::Structure;

/// Represents a single dependency and its required materials
#[derive(Clone, Debug, Deserialize, Serialize)]
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

    /// FIXME: Rework products
    ///        For calculation purposes we need the produced amount per run that
    ///        are build with each run.
    ///        With total_products and products_per_run we can calculate the
    ///        number of runs required. (serde hook during serialize maybe)
    ///        Information required:
    ///            total_products   -> Total number of products that are required
    ///            products_per_run -> Represents the number of products that
    ///                                are produced with each run
    ///            total_time       -> Total time it takes to build all products
    ///            time_per_run     -> Time it takes for one run
    /// Number of total products
    products:              f32,
    /// Base requirements for building a single run
    products_base:         u32,
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
    /// Returns the product rounded to the nearest upper value.
    /// 
    pub fn product(&self) -> u32 {
        self.products.ceil() as u32
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
        self.products = amount as f32;

        let runs = (
            self.products as f64 / self.products_per_run as f64
        ).ceil() as u32;

        // FIXME: with the rework no longer needed
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
        self.products = (runs * self.products_base) as f32;

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
    #[deprecated(note = "Use [DependencyGroup::apply_bonus]")]
    pub fn apply_material_bonus(
        &mut self,
        bonus: &HashMap<TypeId, f32>
    ) {
        if let Some(b) = bonus.get(&self.ptype_id) {
            for material in self.components.iter_mut() {
                if material.products > 1f32 {
                    material.products = material.products - (
                        material.products * (*b / 100f32)
                    );
                }

                material.apply_material_bonus(&bonus);
            }
        } else {
            for material in self.components.iter_mut() {
                material.apply_material_bonus(&bonus);
            }
        }
    }

    pub fn round_material_bonus(
        &mut self
    ) {
        for material in self.components.iter_mut() {
            if material.products > 1f32 {
                material.products = material.products.ceil();
            }

            material.round_material_bonus();
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

    /// Creates a empty dependency containing only the given list of
    /// dependencies.
    /// 
    /// The intent is to create an artificial root dependency.
    /// 
    /// # Params
    /// 
    /// * `deps` -> List of dependencies
    /// 
    /// # Returns
    /// 
    /// Empty dependency containing only the given dependencies
    /// 
    pub(crate) fn with_dependencies(
        deps: Vec<Dependency>
    ) -> Self {
        let mut s = Self::default();
        s.components = deps;
        s
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

impl From<DatabaseDependency> for Dependency {
    fn from(dependency: DatabaseDependency) -> Self {
        Self {
            name:              dependency.product_name,
            blueprint_name:    dependency.blueprint_name,
            btype_id:          dependency.btype_id,
            ptype_id:          dependency.ptype_id,
            category_id:       dependency.category_id,
            group_id:          dependency.group_id,

            products:          dependency.produces as f32,
            products_base:     dependency.quantity,
            products_per_run:  dependency.produces,
            time:              dependency.time,
            time_per_run:      dependency.time,

            dependency_type:   dependency.typ,

            components:        dependency
                                    .components
                                    .into_iter()
                                    .map(Self::from)
                                    .collect::<Vec<_>>(),
        }
    }
}

impl Default for Dependency {
    fn default() -> Self {
        Self {
            name:              String::new(),
            blueprint_name:    String::new(),
            btype_id:          0.into(),
            ptype_id:          0.into(),
            category_id:       0.into(),
            group_id:          0.into(),

            products:          0f32,
            products_base:     0u32,
            products_per_run:  0u32,
            time:              0u32,
            time_per_run:      0u32,

            dependency_type:   DependencyType::default(),

            components:        Vec::new(),
        }
    }
}
