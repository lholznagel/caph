use serde::{Deserialize, Serialize};

/// Different types a dependency can be
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub enum DependencyType {
    /// The dependency is a blueprint
    Blueprint,
    /// The dependency is a reaction
    Reaction,
    /// The dependency is a material
    Material,

    /// The type is not yet set
    Unknown
}

impl Default for DependencyType {
    fn default() -> Self {
        Self::Unknown
    }
}
