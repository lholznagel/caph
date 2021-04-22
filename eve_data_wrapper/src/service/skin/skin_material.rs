use crate::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SkinMaterialEntry {
    #[serde(rename = "displayNameID")]
    pub display_name_id:  DisplayNameId,
    #[serde(rename = "materialSetID")]
    pub material_set_id:  MaterialSetId,
    #[serde(rename = "skinMaterialID")]
    pub skin_material_id: SkinMaterialId,
}
