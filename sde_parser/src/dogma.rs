use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;

pub fn run() -> Result<String, Box<dyn std::error::Error>> {
    let modifier      = parse_modifier_source();
    let filter        = parse_filters();
    let dogma_effects = parse_dogma_effects();
    let type_dogma    = parse_type_dogma();

    let mut dogma_attributes = HashMap::new();
    for (id, effect) in dogma_effects {
        if effect.modifier_info.is_none() {
            continue;
        }

        for modifier in effect.modifier_info.unwrap_or_default() {
            if !modifier.is_some() {
                continue;
            }

            dogma_attributes.insert(
                (id, modifier.modified_attribute_id.unwrap()),
                modifier.modifying_attribute_id.unwrap()
            );
        }
    }

    let mut entries = Vec::new();
    for (mid, modifier) in modifier {
        let dogma = type_dogma.get(&mid).unwrap();

        if let Some(x) = modifier.manufacturing {
            let x = manufacture_modifier(
                mid,
                &dogma,
                &dogma_attributes,
                &filter,
                x
            );
            entries.extend(x);
        } else if let Some(x) = modifier.reaction {
            let x = reaction_modifier(
                mid,
                &dogma,
                &dogma_attributes,
                &filter,
                x
            );
            entries.extend(x);
        } else {
            continue;
        };
    }

    let entries = entries
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let entries = format!("INSERT INTO structure_dogma VALUES {}", entries);
    let entries = vec![
        sql_header(),
        entries
    ]
    .join("\n");
    Ok(entries)
}

fn sql_header() -> String {
    r#"DELETE FROM structure_dogma CASCADE;"#.into()
}

fn manufacture_modifier(
    mid:              usize,
    dogma:            &TypeDogma,
    dogma_attributes: &HashMap<(usize, usize), usize>,
    filter:           &HashMap<usize, Filters>,
    modifier:         Modifier,
) -> Vec<DatabaseEntry> {
    let mut entries = Vec::new();

    // Manufacture
    let mut value = 0f32;
    let mut categories = Vec::new();
    let mut groups = Vec::new();
    if let Some(materials) = modifier.material && !materials.is_empty() {
        for modifier in materials {
            for effect in dogma.effects.iter() {
                if let Some(x) = dogma_attributes.get(
                    &(effect.effect_id, modifier.attribute)
                ) {
                    value = dogma
                        .attributes
                        .iter()
                        .find(|y| y.attribute_id == *x)
                        .unwrap()
                        .value;

                    if let Some(x) = modifier.filter_id {
                        let filtered = filter.get(&x).unwrap();
                        categories.extend(filtered.category_ids.clone());
                        groups.extend(filtered.group_ids.clone());
                    }
                } else if
                    // Raitaru
                    mid == 35825 ||
                    // Azbel
                    mid == 35826 ||
                    // Sotiyo
                    mid == 35827 {
                    value = 1f32;
                }
            }
        }

        entries.push(DatabaseEntry {
            type_id:    mid,
            modifier:   "MANUFACTURE_MATERIAL".into(),
            amount:     value,
            categories: categories,
            groups:     groups,
        });
    }

    let mut value = 0f32;
    let mut categories = Vec::new();
    let mut groups = Vec::new();
    if let Some(times) = modifier.time && !times.is_empty() {
        for modifier in times {
            for effect in dogma.effects.iter() {
                if let Some(x) = dogma_attributes.get(&(effect.effect_id, modifier.attribute)) {
                    value = dogma.attributes
                        .iter()
                        .find(|y| y.attribute_id == *x)
                        .unwrap()
                        .value;

                    if let Some(x) = modifier.filter_id {
                        let filtered = filter.get(&x).unwrap();
                        categories.extend(filtered.category_ids.clone());
                        groups.extend(filtered.group_ids.clone());
                    }
                } else if mid == 35825 {
                    // Raitaru
                    value = 15f32;
                } else if mid == 35826 {
                    // Azbel
                    value = 20f32;
                } else if mid == 35827 {
                    // Sotiyo
                    value = 25f32;
                } else if mid == 47512 {
                    // 'Moreau' Fortizar
                    value = 10f32;
                } else if mid == 47513 {
                    // 'Draccous' Fortizar
                    value = 15f32;
                }
            }
        }

        entries.push(DatabaseEntry {
            type_id:    mid,
            modifier:   "MANUFACTURE_TIME".into(),
            amount:     value,
            categories: categories,
            groups:     groups,
        });
    }

    entries
}

fn reaction_modifier(
    mid:              usize,
    dogma:            &TypeDogma,
    dogma_attributes: &HashMap<(usize, usize), usize>,
    filter:           &HashMap<usize, Filters>,
    modifier:         Modifier,
) -> Vec<DatabaseEntry> {
    let mut entries = Vec::new();

    let mut value = 0f32;
    let mut categories = Vec::new();
    let mut groups = Vec::new();
    if let Some(materials) = modifier.material && !materials.is_empty() {
        for modifier in materials {
            for effect in dogma.effects.iter() {
                if let Some(x) = dogma_attributes.get(&(effect.effect_id, modifier.attribute)) {
                    value = dogma.attributes
                        .iter()
                        .find(|y| y.attribute_id == *x)
                        .unwrap()
                        .value;

                    if let Some(x) = modifier.filter_id {
                        let filtered = filter.get(&x).unwrap();
                        categories.extend(filtered.category_ids.clone());
                        groups.extend(filtered.group_ids.clone());
                    }
                }
            }
        }

        entries.push(DatabaseEntry {
            type_id:    mid,
            modifier:   "REACTION_MATERIAL".into(),
            amount:     value,
            categories: categories,
            groups:     groups,
        });
    }

    let mut value = 0f32;
    let mut categories = Vec::new();
    let mut groups = Vec::new();

    if let Some(times) = modifier.time && !times.is_empty() {
        for modifier in times {
            for effect in dogma.effects.iter() {
                if let Some(x) = dogma_attributes.get(&(effect.effect_id, modifier.attribute)) {
                    value = dogma.attributes
                        .iter()
                        .find(|y| y.attribute_id == *x)
                        .unwrap()
                        .value;

                    if let Some(x) = modifier.filter_id {
                        let filtered = filter.get(&x).unwrap();
                        categories.extend(filtered.category_ids.clone());
                        groups.extend(filtered.group_ids.clone());
                    }
                } else if mid == 35836 {
                    // Tatara
                    value = 25f32;
                }
            }
        }

        entries.push(DatabaseEntry {
            type_id:    mid,
            modifier:   "REACTION_TIME".into(),
            amount:     value,
            categories: categories,
            groups:     groups,
        });
    }

    entries
}

/// TypeId, Modifier, Amount, CategoryId, GroupId
///
/// Modifier = TIME, MANUFACTURE, ISK
/// CategoryId and GroupId either empty (all) or filled with specific categories
#[derive(Debug)]
pub struct DatabaseEntry {
    type_id:    usize,
    modifier:   String,
    amount:     f32,
    categories: Vec<usize>,
    groups:     Vec<usize>,
}

impl fmt::Display for DatabaseEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, '{}', {}, '{{{}}}', '{{{}}}')",
            self.type_id,
            self.modifier,
            self.amount,
            self.categories
                .clone()
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            self.groups
                .clone()
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ModifyResource {
    manufacturing:     Option<Modifier>,
    reaction:          Option<Modifier>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Modifier {
    cost:     Option<Vec<ModifierInfo>>,
    material: Option<Vec<ModifierInfo>>,
    time:     Option<Vec<ModifierInfo>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ModifierInfo {
    #[serde(rename = "dogmaAttributeID")]
    attribute: usize,

    #[serde(rename = "filterID")]
    filter_id: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct Filters {
    #[serde(rename = "categoryIDs")]
    category_ids: Vec<usize>,
    #[serde(rename = "groupIDs")]
    group_ids:    Vec<usize>,
}

#[derive(Debug, Deserialize)]
pub struct DogmaEffect {
    #[serde(rename = "modifierInfo")]
    modifier_info: Option<Vec<DogmaEffectModifier>>
}

#[derive(Debug, Deserialize)]
pub struct DogmaEffectModifier {
    #[serde(rename = "modifiedAttributeID")]
    modified_attribute_id:  Option<usize>,
    #[serde(rename = "modifyingAttributeID")]
    modifying_attribute_id: Option<usize>,
}

impl DogmaEffectModifier {
    pub fn is_some(&self) -> bool {
        self.modified_attribute_id.is_some() &&
        self.modifying_attribute_id.is_some()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct TypeDogma {
    #[serde(rename = "dogmaAttributes")]
    attributes: Vec<TypeDogmaAttribute>,
    #[serde(rename = "dogmaEffects")]
    effects:    Vec<TypeDogmaEffect>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TypeDogmaAttribute {
    #[serde(rename = "attributeID")]
    attribute_id: usize,
    #[serde(rename = "value")]
    value:        f32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TypeDogmaEffect {
    #[serde(rename = "effectID")]
    effect_id: usize,
}

fn parse_modifier_source() -> HashMap<usize, ModifyResource> {
    let reader = File::open("input/industrymodifiersources.json").unwrap();
    let result: HashMap<usize, ModifyResource> = serde_json::from_reader(reader).unwrap();
    result
        .into_iter()
        .filter(|(_, x)| x.manufacturing.is_some() || x.reaction.is_some())
        .collect::<HashMap<_, _>>()
}

fn parse_filters() -> HashMap<usize, Filters> {
    let reader = File::open("input/industrytargetfilters.json").unwrap();
    serde_json::from_reader(reader).unwrap()
}

fn parse_dogma_effects() -> HashMap<usize, DogmaEffect> {
    let reader = File::open("input/dogmaEffects.yaml").unwrap();
    serde_yaml::from_reader(reader).unwrap()
}

fn parse_type_dogma() -> HashMap<usize, TypeDogma> {
    let reader = File::open("input/typeDogma.yaml").unwrap();
    serde_yaml::from_reader(reader).unwrap()
}

