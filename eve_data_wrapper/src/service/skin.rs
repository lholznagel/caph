mod skin_material;
mod skin_license;

use self::skin_material::*;
use self::skin_license::*;

use crate::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct SkinService {
    licenses:  HashMap<SkinLicenseId, SkinLicenseEntry>,
    materials: HashMap<SkinMaterialId, SkinMaterialEntry>,
    skins:     HashMap<SkinId, SkinEntry>,
}

impl SkinService {
    const PATH_LICENSES:  &'static str = "sde/fsd/skinLicenses.yaml";
    const PATH_MATERIALS: &'static str = "sde/fsd/skinMaterials.yaml";
    const PATH_SKINS:     &'static str = "sde/fsd/skins.yaml";

    pub(crate) fn new(mut zip: SdeZipArchive) -> Result<Self, EveConnectError> {
        Ok(Self {
            licenses:  crate::parse_zip_file(Self::PATH_LICENSES, &mut zip)?,
            materials: crate::parse_zip_file(Self::PATH_MATERIALS, &mut zip)?,
            skins:     crate::parse_zip_file(Self::PATH_SKINS, &mut zip)?,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SkinEntry {
    #[serde(rename = "allowCCPDevs")]
    pub allow_ccp_devs:      bool,
    #[serde(rename = "internalName")]
    pub internal_name:       String,
    #[serde(rename = "isStructureSkin")]
    #[serde(default)]
    pub is_structure_skin:   bool,
    #[serde(rename = "skinID")]
    pub skin_id:             SkinId,
    #[serde(rename = "skinMaterialID")]
    pub skin_material_id:    SkinMaterialId,
    #[serde(rename = "types")]
    pub type_ids:            Vec<TypeId>,
    #[serde(rename = "visibleSerenity")]
    pub visible_serenity:    bool,
    #[serde(rename = "visibleTranquility")]
    pub visible_tranquility: bool,

    #[serde(rename = "skinDescription")]
    pub skin_description:    Option<String>,
}
