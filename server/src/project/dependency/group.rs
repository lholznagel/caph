use std::collections::BTreeMap;
use caph_connector::TypeId;

use super::Dependency;

/// List of components that are required in a project
#[derive(Clone, Debug, Default)]
pub struct DependencyGroup(BTreeMap<TypeId, Dependency>);

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
            .and_modify(|x: &mut Dependency| x.products += dep.products)
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
                .and_modify(|x: &mut Dependency| x.products += entry.products)
                .or_insert(entry);
        }
    }

    pub fn build_order(
        &mut self
    ) -> Vec<Dependency> {
        let dependencies = self.0
            .values()
            .cloned()
            .collect::<Vec<_>>();

        let mut virtual_dependency = Dependency::with_dependencies(
            dependencies
        );
        virtual_dependency.sort();
        virtual_dependency.components
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
}
