use caph_connector::TypeId;
use serde::Serialize;
use std::collections::{BTreeMap, HashMap};

use super::Dependency;

/// List of components that are required in a project
#[derive(Clone, Debug, Default, Serialize)]
pub struct DependencyGroup(pub(crate) BTreeMap<TypeId, Dependency>);

impl DependencyGroup {
    /// Consumes itself and returns the inner [BTreeMap].
    /// 
    /// # Returns
    /// 
    /// Inner [BTreeMap]
    /// 
    pub fn into_inner(self) -> BTreeMap<TypeId, Dependency> {
        self.0
    }

    /// Constructes a [DependencyGroup] from a list of dependencies
    /// 
    /// # Params
    /// 
    /// * `dependencies` -> List of dependencies
    /// 
    /// # Returens
    /// 
    /// New [DependencyGroup]
    /// 
    pub fn from_dependencies(
        dependencies: Vec<Dependency>
    ) -> Self {
        let dependencies = dependencies
            .into_iter()
            .map(|x| (x.ptype_id, x))
            .collect::<BTreeMap<_, _>>();
        Self(dependencies)
    }

    /// Adds a new [Dependency] to the list.
    /// If the dependency is already in the list, the number of products will
    /// be added to the existing entry.
    /// 
    /// # Params
    /// 
    /// * `dep` -> [Dependency] to add
    /// 
    pub fn add(
        &mut self,
        dep: Dependency
    ) {
        self.0
            .entry(dep.ptype_id)
            .and_modify(|x: &mut Dependency|
                x.set_product_quantity(x.product() + dep.product())
            )
            .or_insert(dep.clone());
    }

    /// Merges two [DependencyGroup] together.
    /// 
    /// # Params
    /// 
    /// * `dep` -> [DependencyGroup] to merge
    /// 
    pub fn merge(
        &mut self,
        dep: DependencyGroup
    ) {
        for (type_id, entry) in dep.into_inner() {
            self.0
                .entry(type_id)
                .and_modify(|x: &mut Dependency| {
                    //x.products += entry.products;
                    // TODO: validate that this is the way
                    x.set_product_quantity(x.product() + entry.product());
                })
                .or_insert(entry);
        }
    }

    /// TODO: this flattens the tree, validate. Old function name was `collect_components`
    /// Flattens the Group into a single layer
    /// 
    /// # Returns
    /// 
    /// Flattened [DependencyGroup]
    /// 
    pub fn collect_components(
        &self
    ) -> DependencyGroup {
        let mut group = DependencyGroup::default();

        for (_, entry) in self.0.iter() {
            group.merge(entry.collect_components());
        }

        group
    }

    /// TODO: add more comments
    /// Recalculates the complete [DependencyGroup] in place
    /// 
    pub fn recalculate(
        &mut self,
    ) {
        let mut old = Vec::new();
        let mut new = Vec::new();

        while old != new || old.len() == 0 {
            old = new;

            let mut changes = HashMap::new();

            for (_, entry) in self.0.iter() {
                for component in entry.components.iter() {
                    changes
                        .entry(component.ptype_id)
                        .and_modify(|x: &mut f32| *x += component.products)
                        .or_insert(component.products);
                }
            }

            for (pid, entry) in changes {
                self.0
                    .entry(pid)
                    .and_modify(|x: &mut Dependency| x.set_product_quantity(entry as u32));
            }
            let mut changes = HashMap::new();

            for (_, entry) in self.0.iter() {
                for component in entry.components.iter() {
                    changes
                        .entry(component.ptype_id)
                        .and_modify(|x: &mut f32| *x += component.products)
                        .or_insert(component.products);
                }
            }

            for (pid, entry) in changes {
                self.0
                    .entry(pid)
                    .and_modify(|x: &mut Dependency| x.set_product_quantity(entry as u32));
            }

            new = self.0
                .clone()
                .into_iter()
                .map(|(pid, p)| (pid, p.products))
                .collect::<Vec<_>>();
        }
    }

    /// Sorts the dependencies into a build order
    /// 
    /// # Returns
    /// 
    /// List of sorted [Dependency]
    /// 
    pub fn build_order(
        &mut self,
    ) -> Vec<Dependency> {
        let dependencies = self.0
            .values()
            .cloned()
            .collect::<Vec<_>>();

        let corrections = self.0
            .clone()
            .into_iter()
            .collect::<HashMap<_, _>>();

        let mut virtual_dependency = Dependency::with_dependencies(
            dependencies
        );
        virtual_dependency.sort(corrections);
        virtual_dependency.components
    }
}
