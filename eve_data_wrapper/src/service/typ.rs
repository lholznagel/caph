use crate::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Service for wrapping EVE types.
#[derive(Clone, Debug)]
pub struct TypeService {
    /// All available EVE types
    types:     HashMap<TypeId, TypeIdEntry>,
    /// Contains items like Plagioclase that can be reprocessed
    materials: HashMap<TypeId, TypeMaterialEntry>,
}

impl TypeService {
    const PATH_ID:       &'static str = "sde/fsd/typeIDs.yaml";
    const PATH_MATERIAL: &'static str = "sde/fsd/typeMaterials.yaml";

    /// Creates a new service instant.
    ///
    /// # Parameters
    ///
    /// * `zip` - Current SDE-Zip archive
    ///
    /// # Returns
    ///
    /// Instance of itself with parsed fields.
    ///
    pub fn new(mut zip: SdeZipArchive) -> Result<Self, EveConnectError> {
        Ok(Self {
            types:     crate::parse_zip_file(Self::PATH_ID, &mut zip)?,
            materials: crate::parse_zip_file(Self::PATH_MATERIAL, &mut zip)?
        })
    }

    /// Returns all types
    pub fn types(&self) -> &HashMap<TypeId, TypeIdEntry> {
        &self.types
    }

    pub fn type_by_id<T: Into<TypeId>>(&self, id: T) -> Option<&TypeIdEntry> {
        self.types.get(&id.into())
    }

    /// Returns all materials
    pub fn materials(&self) -> &HashMap<TypeId, TypeMaterialEntry> {
        &self.materials
    }

    /// Collects all names
    pub fn collect_names(&self) -> HashMap<TypeId, String> {
        self
            .types
            .iter()
            .map(|(id, entry)| (*id, entry.name().unwrap_or_default()))
            .collect::<HashMap<_, _>>()
    }

    /// Checks if the given [TypeId] can be reprocessed
    ///
    /// # Parameters
    ///
    /// * `id` - Id to check, can be [TypeId] or a [u32]
    ///
    /// # Returns
    ///
    /// * `true`  - when the type can be reprocessed
    /// * `false` - when it cannot be reprocessed
    ///
    pub fn type_can_be_reprocessed<T: Into<TypeId>>(&self, id: T) -> bool {
        self.materials.contains_key(&id.into())
    }
}

/// Represents a single entry in the yaml for a type
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TypeIdEntry {
    #[serde(rename = "groupID")]
    pub group_id:                 GroupId,
    #[serde(rename = "description")]
    #[serde(default)]
    pub description:              HashMap<String, String>,
    #[serde(rename = "name")]
    pub name:                     HashMap<String, String>,
    #[serde(rename = "masteries")]
    #[serde(default)]
    pub masteries:                HashMap<u16, Vec<u16>>,
    #[serde(rename = "portionSize")]
    pub portion_size:             usize,
    #[serde(rename = "published")]
    pub published:                bool,

    #[serde(rename = "basePrice")]
    pub base_price:               Option<f64>,
    #[serde(rename = "capacity")]
    pub capacity:                 Option<f64>,
    #[serde(rename = "sofFactionName")]
    pub faction_name:             Option<String>,
    #[serde(rename = "factionID")]
    pub faction_id:               Option<FactionId>,
    #[serde(rename = "graphicID")]
    pub graphic_id:               Option<GraphicId>,
    #[serde(rename = "iconID")]
    pub icon_id:                  Option<IconId>,
    #[serde(rename = "marketGroupID")]
    pub market_group_id:          Option<MarketGroupId>,
    #[serde(rename = "mass")]
    pub mass:                     Option<f32>,
    #[serde(rename = "sofMaterialSetID")]
    pub material_set_id:          Option<u32>,
    #[serde(rename = "metaGroupID")]
    pub meta_group_id:            Option<MetaGroupId>,
    #[serde(rename = "raceID")]
    pub race_id:                  Option<RaceId>,
    #[serde(rename = "radius")]
    pub radius:                   Option<f32>,
    #[serde(rename = "soundID")]
    pub sound_id:                 Option<SoundId>,
    #[serde(rename = "traits")]
    pub traits:                   Option<Trait>,
    #[serde(rename = "volume")]
    pub volume:                   Option<f32>,
    #[serde(rename = "variationParentTypeID")]
    pub variation_parent_type_id: Option<TypeId>,
}

impl TypeIdEntry {
    /// Gets the english description for a type.
    ///
    /// # Returns
    ///
    /// If the english translation exists, it is returned, if not [None] is
    /// returned.
    pub fn description(&self) -> Option<String> {
        self
            .description
            .get("en")
            .cloned()
    }

    /// Gets the english name for a type.
    ///
    /// # Returns
    ///
    /// If the english translation exists, it is returned, if not [None] is
    /// returned.
    pub fn name(&self) -> Option<String> {
        self
            .name
            .get("en")
            .cloned()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Trait {
    #[serde(rename = "miscBonuses")]
    pub misc_bonuses: Option<Vec<Bonus>>,
    #[serde(rename = "iconID")]
    pub icon_id :     Option<IconId>,
    #[serde(rename = "roleBonuses")]
    pub role_bonuses: Option<Vec<Bonus>>,
    #[serde(rename = "types")]
    #[serde(default)]
    pub types:        HashMap<usize, Vec<Bonus>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bonus {
    #[serde(rename = "bonusText")]
    pub bonus_text:  HashMap<String, String>,
    #[serde(rename = "importance")]
    pub importance:  usize,

    #[serde(rename = "bonus")]
    pub bonus:       Option<f32>,
    #[serde(rename = "unitID")]
    pub unit_id:     Option<UnitId>,
    #[serde(rename = "isPositive")]
    pub is_positive: Option<bool>,
}

/// Represents the materials for a type.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TypeMaterialEntry {
    #[serde(rename = "materials")]
    pub materials: Vec<Material>,
}

/// Description of a type material
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Material {
    #[serde(rename = "materialTypeID")]
    pub material_type_id: TypeId,
    #[serde(rename = "quantity")]
    pub quantity:         u32
}
