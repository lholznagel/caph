use crate::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SkinLicenseEntry {
    #[serde(rename = "duration")]
    pub duration:        i32,
    #[serde(rename = "isSingleUse")]
    #[serde(default)]
    pub is_single_use:   bool,
    #[serde(rename = "licenseTypeID")]
    pub skin_license_id: SkinLicenseId,
    #[serde(rename = "skinID")]
    pub skin_id:         SkinId,
}
