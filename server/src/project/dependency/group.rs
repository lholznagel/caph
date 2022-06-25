use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use caph_connector::{GroupId, TypeId};

use crate::project::dependency::DependencyType;

use super::Dependency;
use headers::ContentLocation;

/// List of components that are required in a project
#[derive(Clone, Debug, Default, serde::Serialize)]
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
                x.set_product_quantity(x.products + dep.products)
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
                    x.set_product_quantity(x.products + entry.products);
                })
                .or_insert(entry);
        }
    }

    pub fn collect_components(
        &self
    ) -> DependencyGroup {
        let mut group = DependencyGroup::default();

        for (_, entry) in self.0.iter() {
            group.merge(entry.collect_components());
        }

        // Recalculate Fuel Blocks
        let mut fuel_blocks = HashMap::new();
        for (_, entry) in group.0.iter() {
            if entry.dependency_type != DependencyType::Reaction {
                continue;
            }

            for component in entry.components.iter() {
                if component.group_id != GroupId(1136) {
                    continue;
                }

                fuel_blocks
                    .entry(component.ptype_id)
                    .and_modify(|x: &mut u32| *x += component.products)
                    .or_insert(component.products);
            }
        }

        for (ptype_id, entry) in fuel_blocks {
            group.0
                .entry(ptype_id)
                .and_modify(
                    |x: &mut Dependency|
                    x.set_product_quantity(entry)
                );
        }

        group
    }

    /// Sorts the dependencies into a build order
    /// 
    /// # Returns
    /// 
    /// List of sorted dependencies
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

    #[deprecated]
    pub fn recalculate(
        &mut self
    ) {
        for (_, entry) in self.0.iter_mut() {
            entry.set_product_quantity(entry.products);
        }
    }

    #[deprecated]
    pub fn fix(
        &mut self,
        old: DependencyGroup
    ) {
        let mut result = BTreeSet::new();
        for (ptype_id, new) in self.0.iter() {
            if new.components.is_empty() {
                continue;
            }

            if let Some(old) = old.0.get(&ptype_id) {
                if new.has_diff(old.clone()) {
                    result.insert(ptype_id);
                }
            }
        }

        let mut queue = VecDeque::new();
        let mut corrections = result
            .iter()
            .map(|x| self.0.get(x))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .map(|x| {
                for component in x.components.iter() {
                    if component.components.is_empty() {
                        continue;
                    }
                    queue.push_back(component.clone());
                }

                (x.ptype_id, x.clone())
            })
            .collect::<HashMap<_, _>>();
        let mut init_changes = result.clone();

        while let Some(popped) = queue.pop_front() {
            if init_changes.contains(&popped.ptype_id) {
                corrections
                    .entry(popped.ptype_id)
                    .and_modify(|x: &mut Dependency| x.products = popped.products)
                    .or_insert(popped.clone());
                init_changes.remove(&popped.ptype_id);
            } else {
                corrections
                    .entry(popped.ptype_id)
                    .and_modify(|x: &mut Dependency| x.products += popped.products)
                    .or_insert(popped.clone());
            }

            for component in popped.components.iter() {
                if component.components.is_empty() {
                    continue;
                }
                queue.push_back(component.clone());
            }
        }

        for (ptype_id, correction) in corrections.iter() {
            if let Some(entry) = self.0.get_mut(&ptype_id) {
                entry.set_product_quantity(correction.products);
            }
        }
    }
}

#[cfg(test)]
mod dependency_group_test {
    use pretty_assertions::assert_eq;

    use crate::project::dependency::test_utils::dependency;

    use super::*;

    #[test]
    fn add_to_existing_dependency() {
        let a1 = dependency(
            "A".into(),
            0.into(),
            1i64,
            Vec::new()
        );
        let a2 = dependency(
            "A".into(),
            0.into(),
            1i64,
            Vec::new()
        );

        let mut group = DependencyGroup::default();
        group.add(a1);
        group.add(a2);
        let group = group.into_inner();

        assert_eq!(group.len(), 1);
        assert_eq!(group.get(&0.into()).unwrap().products, 2);
    }

    #[test]
    fn add_new_dependency() {
        let a = dependency(
            "A".into(),
            0.into(),
            1i64,
            Vec::new()
        );
        let b = dependency(
            "B".into(),
            1.into(),
            2i64,
            Vec::new()
        );

        let mut group = DependencyGroup::default();
        group.add(a);
        group.add(b);
        let group = group.into_inner();

        assert_eq!(group.len(), 2);
        assert_eq!(group.get(&0.into()).unwrap().products, 1);
        assert_eq!(group.get(&1.into()).unwrap().products, 2);
    }

    #[test]
    fn merge_existing_dependencies() {
        let a = dependency(
            "A".into(),
            0.into(),
            1i64,
            Vec::new()
        );
        let mut group1 = DependencyGroup::default();
        group1.add(a);

        let a = dependency(
            "A".into(),
            0.into(),
            1i64,
            Vec::new()
        );
        let mut group2 = DependencyGroup::default();
        group2.add(a);
        group1.merge(group2);

        let group = group1.into_inner();
        assert_eq!(group.len(), 1);
        assert_eq!(group.get(&0.into()).unwrap().products, 2);
    }

    #[test]
    fn merge_dependencies() {
        let a = dependency(
            "A".into(),
            0.into(),
            1i64,
            Vec::new()
        );
        let mut group1 = DependencyGroup::default();
        group1.add(a);

        let b = dependency(
            "B".into(),
            1.into(),
            2i64,
            Vec::new()
        );
        let mut group2 = DependencyGroup::default();
        group2.add(b);
        group1.merge(group2);

        let group = group1.into_inner();
        assert_eq!(group.len(), 2);
        assert_eq!(group.get(&0.into()).unwrap().products, 1);
        assert_eq!(group.get(&1.into()).unwrap().products, 2);
    }

    #[test]
    fn merge_deep() {
        let a = dependency(
            "A".into(),
            0.into(),
            1i64,
            vec![
                dependency(
                    "B".into(),
                    1.into(),
                    1i64,
                    Vec::new()
                )
            ]
        );
        let mut group1 = DependencyGroup::default();
        group1.add(a);
        let group2 = group1.clone();

        group1.merge(group2);

        let group = group1.into_inner();
        assert_eq!(group.len(), 1);
        assert_eq!(group.get(&0.into()).unwrap().products, 2);
        assert_eq!(group
            .get(&0.into())
            .unwrap()
            .components
            .get(0)
            .unwrap()
            .products, 2
        );
    }
}
