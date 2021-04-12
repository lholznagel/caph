mod dogma_attribute;
mod dogma_attribute_categorie;
mod dogma_effect;
mod type_dogma;

pub use self::dogma_attribute::*;
pub use self::dogma_attribute_categorie::*;
pub use self::dogma_effect::*;
pub use self::type_dogma::*;

use crate::*;

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct DogmaService {
    attributes: HashMap<AttributeId, DogmaAttributeEntry>,
    categories: HashMap<DogmaCategoryId, DogmaAttributeCategoryEntry>,
    effects:    HashMap<u32, DogmaEffectEntry>,
    typ:        HashMap<AttributeId, TypeDogmaEntry>,
}

impl DogmaService {
    const PATH_ATTRIBUTES: &'static str = "sde/fsd/dogmaAttributes.yaml";
    const PATH_CATEGORIES: &'static str = "sde/fsd/dogmaAttributeCategories.yaml";
    const PATH_EFFECTS:    &'static str = "sde/fsd/dogmaEffects.yaml";
    const PATH_TYPE:       &'static str = "sde/fsd/typeDogma.yaml";

    pub(crate) fn new(mut zip: SdeZipArchive) -> Result<Self, EveSdeParserError> {
        Ok(Self {
            attributes: service_file_gen!(zip, Self::PATH_ATTRIBUTES),
            categories: service_file_gen!(zip, Self::PATH_CATEGORIES),
            effects:    service_file_gen!(zip, Self::PATH_EFFECTS),
            typ:        service_file_gen!(zip, Self::PATH_TYPE),
        })
    }
}
