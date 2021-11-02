use crate::ConnectError;
use crate::{CategoryId, GroupId, TypeId};
use crate::zip::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Wrapper for all type and group ids
pub struct ConnectAssetService {
    /// All type id entries that are in the file
    type_ids:  HashMap<TypeId, TypeIdEntry>,
    /// All group id entries that are in the file
    group_ids: HashMap<GroupId, GroupEntry>
}

impl ConnectAssetService {
    /// Path to the typeID file
    const PATH_TYPE:   &'static str = "sde/fsd/typeIDs.yaml";
    /// Path to the groupID file
    const PATH_GROUPS: &'static str = "sde/fsd/groupIDs.yaml";

    /// Creates a new instance of the service
    ///
    /// # Params
    ///
    /// * `zip` -> Service for the zip file
    ///
    /// # Errors
    ///
    /// Fails when the file is not in the zip file or parsing the file fails.
    ///
    /// # Returns
    ///
    /// New instance
    ///
    pub fn new(zip: &mut SdeService) -> Result<Self, ConnectError> {
        let type_ids  = zip.get_file(Self::PATH_TYPE)?;
        let group_ids = zip.get_file(Self::PATH_GROUPS)?;

        Ok(ConnectAssetService {
            type_ids,
            group_ids
        })
    }

    /// Gets the list of all types
    ///
    /// # Returns
    ///
    /// List of all types
    ///
    pub fn type_ids(&self) -> &HashMap<TypeId, TypeIdEntry> {
        &self.type_ids
    }

    /// Gets the list of all groups
    ///
    /// # Returns
    ///
    /// List of all groups
    ///
    pub fn group_ids(&self) -> &HashMap<GroupId, GroupEntry> {
        &self.group_ids
    }
}

/// Represents a single entry in the yaml for a type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TypeIdEntry {
    /// ID of the group this type belongs to
    #[serde(rename = "groupID")]
    pub group_id:                 GroupId,
    /// Name of the item in different languages
    #[serde(rename = "name")]
    pub name:                     HashMap<String, String>,
    /// Volume of the type
    #[serde(rename = "volume")]
    pub volume:                   Option<f32>,
}

impl TypeIdEntry {
    /// Gets the english name for a type.
    ///
    /// # Returns
    ///
    /// If the english translation exists, it is returned, if not [None] is
    /// returned.
    ///
    pub fn name(&self) -> Option<String> {
        self
            .name
            .get("en")
            .cloned()
    }
}

/// Represents a single group entry
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GroupEntry {
    /// Id of the category
    #[serde(rename = "categoryID")]
    pub category_id: CategoryId,
}
