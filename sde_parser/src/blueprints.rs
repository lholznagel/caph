//! Creates the SQL-Code for blueprints
use crate::FOLDER_INPUT;

use crate::items::{GroupEntry, TypeEntry};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use uuid::Uuid;

/// Wrapper for TypeId
type TypeId = i32;
/// Wrapper for GroupId
type GroupId = i32;

/// Parses the input file and exports it as SQL
pub fn run() -> Result<String, Box<dyn std::error::Error>> {
    tracing::info!("Starting blueprint parsing");

    let current = std::env::current_dir()?;
    let current = current.to_str().unwrap_or_default();
    let path = format!("{}/{}/blueprints.yaml", current, FOLDER_INPUT);
    let file = File::open(&path)?;

    let path_type_ids = format!("{}/{}/type_ids.yaml", current, FOLDER_INPUT);
    let file_type_ids = File::open(&path_type_ids)?;

    let path_group_ids = format!("{}/{}/group_ids.yaml", current, FOLDER_INPUT);
    let file_group_ids = File::open(&path_group_ids)?;

    // Map with the blueprint as key
    let blueprints: HashMap<TypeId, Blueprint> = serde_yaml::from_reader(file)?;
    let type_ids: HashMap<TypeId, TypeEntry> = serde_yaml::from_reader(file_type_ids)?;
    let group_ids: HashMap<GroupId, GroupEntry> = serde_yaml::from_reader(file_group_ids)?;

    // Map with the product as key
    let products = blueprints
        .clone()
        .into_iter()
        .map(|(_, e)| e)
        .filter(|e| e.product().is_some())
        .filter(|e| e.materials().len() > 1)
        .map(|e| {
            // Unwrap is save because of the filter
            let ptype_id = e.product().unwrap();
            (ptype_id, e)
        })
        .collect::<HashMap<_, _>>();

    let entries = vec![
        sql_header(),
        sql_manufacture(&blueprints),
        sql_manufacture_components(&blueprints, &products),
        sql_research(&blueprints),
        sql_invention(&blueprints),
        sql_raw(&blueprints, &products),
        sql_json(&type_ids, &group_ids, &blueprints, &products),
    ];

    Ok(entries.join("\n"))
}

/// Generates the basic SQL-Query that is required for blueprints.
///
/// # Returns
///
/// String containing the SQL-Query.
///
fn sql_header() -> String {
    r#"DELETE FROM blueprint_raw                    CASCADE;
DELETE FROM blueprint_manufacture            CASCADE;
DELETE FROM blueprint_manufacture_components CASCADE;
DELETE FROM blueprint_inventions             CASCADE;
DELETE FROM blueprint_research               CASCADE;
DELETE FROM blueprint_materials              CASCADE;
DELETE FROM blueprint_json                   CASCADE;"#
        .into()
}

/// Generates the SQL-Code for inserting all blueprint research entries.
///
/// Combines the activities [ActivityName::ResearchMaterial],
/// [ActivityName::ResearchTime] and [ActivityName::Copying].
///
/// # Params
///
/// * `bps` -> Map of the parsed `blueprint.yaml` file
///
/// # Returns
///
/// String containing the SQL-Query.
///
fn sql_research(bps: &HashMap<TypeId, Blueprint>) -> String {
    let mut entries = Vec::new();

    for (btype_id, entry) in bps {
        if !entry.has_job() {
            continue;
        }

        let btype_id = *btype_id;

        let ptype_id = if let Some(x) = entry.product() {
            x
        } else {
            continue;
        };

        let material = if let Some(x) = entry.research_time(ActivityName::ResearchMaterial) {
            x
        } else {
            continue;
        };

        let time = if let Some(x) = entry.research_time(ActivityName::ResearchTime) {
            x
        } else {
            continue;
        };

        let copy = if let Some(x) = entry.research_time(ActivityName::Copying) {
            x
        } else {
            continue;
        };

        let sql_entry = BlueprintResearch {
            btype_id,
            ptype_id,
            material,
            time,
            copy,
        };
        entries.push(sql_entry.into_sql());
    }

    format!(
        "
INSERT INTO blueprint_research VALUES {};",
        entries.join(", ")
    )
}

/// Generates a SQL-Query containing all blueprints
///
/// # Returns
///
/// String containing the value-tuple
///
fn sql_manufacture(blueprints: &HashMap<TypeId, Blueprint>) -> String {
    let excluded_type_ids = vec![
        2738, 2742, 2743, 2744, 2745, 2746, 2747, 2748, 2749, 2751, 2753, 2754, 2756, 2758, 2760,
        2762, 2764, 2765, 2766, 2767, 2768, 2769, 2770, 2771, 2772, 2773, 2786, 2788, 2789, 2790,
        2791, 2793, 2795, 2797, 2800, 2820, 2821, 21943, 21944, 21945, 21946, 28605, 32800, 32802,
        32804, 33515, 33582, 33584, 33868, 34222, 42134, 42135,
    ];

    let mut bps = Vec::new();
    let mut materials = Vec::new();

    for (btype_id, entry) in blueprints {
        let bp_id = Uuid::new_v4();

        if excluded_type_ids.contains(btype_id) {
            continue;
        }

        let mut queue = VecDeque::from(entry.materials());
        let mut compounds = HashMap::new();

        while let Some(e) = queue.pop_front() {
            let quantity = e.quantity;
            let mtype_id = e.type_id;
            let produces = entry.product_quantity().unwrap_or_default();
            let time = entry.manufacture_time().unwrap_or_default();

            let bpm = BlueprintMaterial {
                bp_id,
                quantity,
                mtype_id,
                produces,
                time,
            };
            compounds
                .entry(e.type_id)
                .and_modify(|x: &mut BlueprintMaterial| x.quantity += bpm.quantity)
                .or_insert(bpm);
        }

        let compounds = compounds
            .into_iter()
            .map(|(_, x)| x.into_sql())
            .collect::<Vec<_>>();
        materials.extend(compounds);

        let btype_id = *btype_id;
        let ptype_id = if let Some(x) = entry.product() {
            x
        } else {
            continue;
        };
        let quantity = if let Some(x) = entry.product_quantity() {
            x
        } else {
            continue;
        };
        let time = if let Some(x) = entry.manufacture_time() {
            x
        } else {
            continue;
        };
        let reaction = entry.is_reaction();
        let bp = BlueprintManufacture {
            bp_id,
            btype_id,
            ptype_id,
            quantity,
            time,
            reaction,
        };
        bps.push(bp.into_sql());
    }

    format!(
        "
INSERT INTO blueprint_manufacture VALUES {};
INSERT INTO blueprint_materials VALUES {};",
        bps.join(", "),
        materials.join(", ")
    )
}

/// Generates a SQL-Query containing all blueprints components
///
/// # Returns
///
/// String containing the value-tuple
///
fn sql_manufacture_components(
    blueprints: &HashMap<TypeId, Blueprint>,
    products: &HashMap<TypeId, Blueprint>,
) -> String {
    let mut bps = Vec::new();
    let mut materials = Vec::new();

    for (btype_id, entry) in blueprints {
        let bp_id = Uuid::new_v4();

        let mut queue = VecDeque::from(entry.materials());
        let mut components = HashMap::new();

        while let Some(e) = queue.pop_front() {
            let product = if let Some(x) = products.get(&e.type_id) {
                x
            } else {
                continue;
            };

            let materials = product.materials();
            // Skip base materials
            if materials.is_empty() {
                continue;
            }
            queue.extend(materials);

            let quantity = e.quantity;
            let mtype_id = e.type_id;
            let produces = product.product_quantity().unwrap_or_default();
            let time = product.manufacture_time().unwrap_or_default();

            let bpm = BlueprintMaterial {
                bp_id,
                quantity,
                mtype_id,
                produces,
                time,
            };
            components
                .entry(e.type_id)
                .and_modify(|x: &mut BlueprintMaterial| x.quantity += bpm.quantity)
                .or_insert(bpm);
        }

        let components = components
            .into_iter()
            .map(|(_, x)| x.into_sql())
            .collect::<Vec<_>>();
        materials.extend(components);

        let btype_id = *btype_id;
        let ptype_id = if let Some(x) = entry.product() {
            x
        } else {
            continue;
        };
        let quantity = if let Some(x) = entry.product_quantity() {
            x
        } else {
            continue;
        };
        let bp = BlueprintManufactureComponent {
            bp_id,
            btype_id,
            ptype_id,
            quantity,
        };
        bps.push(bp.into_sql());
    }

    format!(
        "
INSERT INTO blueprint_manufacture_components VALUES {};
INSERT INTO blueprint_materials VALUES {};",
        bps.join(", "),
        materials.join(", ")
    )
}

/// Generates the SQL-Code for inserting all blueprint invention entries.
///
/// Contains the activity [ActivityName::Invention].
///
/// # Params
///
/// * `bps` -> Map of the parsed `blueprint.yaml` file
///
/// # Returns
///
/// String containing the SQL-Query.
///
fn sql_invention(bps: &HashMap<TypeId, Blueprint>) -> String {
    let mut inventions = Vec::new();
    let mut materials = Vec::new();

    for (btype_id, entry) in bps {
        let ptype_id = if let Some(x) = entry.product() {
            x
        } else {
            continue;
        };

        let activity = if let Some(x) = entry.activities.get(&ActivityName::Invention) {
            x
        } else {
            continue;
        };
        let time = if let Some(x) = entry.activities.get(&ActivityName::Invention) {
            if !x.materials.is_empty() {
                x.time
            } else {
                continue;
            }
        } else {
            continue;
        };

        for i in activity.products.iter() {
            let bp_id = Uuid::new_v4();
            let btype_id = *btype_id;
            let itype_id = i.type_id;
            let ttype_id = if let Some(x) = bps.get(&itype_id) {
                if let Some(x) = x.product() {
                    x
                } else {
                    continue;
                }
            } else {
                continue;
            };
            let probability = i.probability.unwrap_or_default();

            let invention = BlueprintInvention {
                bp_id,
                btype_id,
                ptype_id,
                itype_id,
                ttype_id,
                time,
                probability,
            };
            inventions.push(invention.into_sql());

            for i in activity.materials.iter() {
                let quantity = i.quantity;
                let mtype_id = i.type_id;
                let produces = entry.product_quantity().unwrap_or_default();
                let time = entry.manufacture_time().unwrap_or_default();

                let material = BlueprintMaterial {
                    bp_id,
                    quantity,
                    mtype_id,
                    produces,
                    time,
                };
                materials.push(material.into_sql());
            }
        }
    }

    format!(
        "
INSERT INTO blueprint_inventions VALUES {};
INSERT INTO blueprint_materials VALUES {};",
        inventions.join(", "),
        materials.join(", ")
    )
}

/// Generates the SQL-Code for inserting all raw entries that are required for
/// a blueprint or reaction.
///
/// # Params
///
/// * `bps` -> Map of the parsed `blueprint.yaml` file
///
/// # Returns
///
/// String containing the SQL-Query.
///
fn sql_raw(
    blueprints: &HashMap<TypeId, Blueprint>,
    products: &HashMap<TypeId, Blueprint>,
) -> String {
    let mut entries = Vec::new();
    let mut materials = Vec::new();

    for (bp_id, bp) in blueprints {
        let mut raw = HashMap::new();

        let mut todo = VecDeque::new();
        todo.extend(bp.materials());

        while let Some(e) = todo.pop_front() {
            if let Some(x) = products.get(&e.type_id) {
                todo.extend(x.materials());
            } else {
                raw.entry(e.type_id)
                    .and_modify(|r: &mut Material| r.quantity += e.quantity)
                    .or_insert(e);
            }
        }

        let btype_id = *bp_id;
        let ptype_id = bp.product().unwrap_or_default();
        let bp_id = Uuid::new_v4();
        let quantity = bp.product_quantity().unwrap_or_default();

        entries.push(
            BlueprintRaw {
                bp_id,
                btype_id,
                ptype_id,
                quantity,
            }
            .into_sql(),
        );

        for (_, raw) in raw {
            let quantity = raw.quantity;
            let mtype_id = raw.type_id;
            let produces = bp.product_quantity().unwrap_or_default();
            let time = bp.manufacture_time().unwrap_or_default();

            materials.push(
                BlueprintMaterial {
                    bp_id,
                    quantity,
                    mtype_id,
                    produces,
                    time,
                }
                .into_sql(),
            );
        }
    }

    format!(
        "
INSERT INTO blueprint_raw VALUES {};
INSERT INTO blueprint_materials VALUES {};",
        entries.join(", "),
        materials.join(", ")
    )
}

/// Generates the SQL-Code for inserting all raw entries that are required for
/// a blueprint or reaction.
///
/// # Params
///
/// * `bps` -> Map of the parsed `blueprint.yaml` file
///
/// # Returns
///
/// String containing the SQL-Query.
///
fn sql_json(
    items: &HashMap<TypeId, TypeEntry>,
    groups: &HashMap<GroupId, GroupEntry>,
    blueprints: &HashMap<TypeId, Blueprint>,
    products: &HashMap<TypeId, Blueprint>,
) -> String {
    #[derive(Clone, Debug, Serialize)]
    enum DependencyType {
        Blueprint,
        Reaction,

        Material,
    }
    impl DependencyType {
        pub fn reaction(is: bool) -> Self {
            if is {
                Self::Reaction
            } else {
                Self::Blueprint
            }
        }
    }
    #[derive(Clone, Debug, Serialize)]
    struct Dependency {
        btype_id: TypeId,
        blueprint_name: String,
        ptype_id: TypeId,
        time: u32,
        quantity: u32,
        produces: u32,
        info: DependencyInfo,
        typ: DependencyType,
        components: Vec<Dependency>,
    }
    impl Dependency {
        pub fn to_sql(self) -> String {
            format!(
                "({}, '{}')",
                &self.ptype_id,
                serde_json::to_string(&self).unwrap()
            )
        }
    }

    #[derive(Clone, Debug, Serialize)]
    struct DependencyInfo {
        ptype_id: TypeId,
        category_id: u32,
        group_id: u32,
        name: String,
    }

    let find_btype_id = |ptype_id: TypeId| {
        blueprints
            .iter()
            .filter(|(_, x)| x.product().is_some())
            .find(|(_, x)| x.product().unwrap() == ptype_id)
            .map(|(y, _)| y)
            .unwrap()
            .clone()
    };

    let mut entries: HashMap<TypeId, Dependency> = HashMap::new();
    let mut queue: VecDeque<Dependency> = VecDeque::new();

    for (ptype_id, pentry) in products {
        let bentry = items.get(&find_btype_id(*ptype_id)).unwrap();
        let bname = bentry.name.get("en").unwrap().replace('\'', "''");

        if let None = items.get(&ptype_id) {
            continue;
        }

        let ientry = items.get(&ptype_id).unwrap();
        if !ientry.published {
            continue;
        }

        let iname = ientry.name.get("en").unwrap().replace('\'', "''");
        let igroup_id = ientry.group_id;
        let icategory_id = groups.get(&igroup_id).unwrap().category_id;

        let mut dependency = Dependency {
            blueprint_name: bname.clone(),
            ptype_id: *ptype_id,
            btype_id: find_btype_id(*ptype_id),
            info: DependencyInfo {
                ptype_id: *ptype_id,
                category_id: icategory_id as u32,
                group_id: igroup_id as u32,
                name: iname.clone(),
            },
            time: pentry.manufacture_time().unwrap() as u32,
            quantity: pentry.product_quantity().unwrap() as u32,
            produces: pentry.product_quantity().unwrap() as u32,
            typ: DependencyType::reaction(pentry.is_reaction()),
            components: Vec::new(),
        };

        let mut components = Vec::new();
        for material in pentry.materials() {
            if products.contains_key(&material.type_id) && entries.contains_key(&material.type_id) {
                let mut entry = entries.get(&material.type_id).unwrap().clone();
                entry.quantity = material.quantity as u32;
                components.push(entry);
            } else if !products.contains_key(&material.type_id) {
                let ientry = items.get(&material.type_id).unwrap();
                let iname = ientry.name.get("en").unwrap().replace('\'', "''");
                let igroup_id = ientry.group_id;
                let icategory_id = groups.get(&igroup_id).unwrap().category_id;

                let dependency = Dependency {
                    blueprint_name: String::new(),
                    ptype_id: material.type_id,
                    btype_id: 0,
                    time: 0,
                    quantity: material.quantity as u32,
                    produces: 0,
                    info: DependencyInfo {
                        ptype_id: *ptype_id,
                        category_id: icategory_id as u32,
                        group_id: igroup_id as u32,
                        name: iname.clone(),
                    },
                    typ: DependencyType::Material,
                    components: Vec::new(),
                };
                components.push(dependency);
            } else {
                queue.push_back(dependency.clone());
                break;
            }
        }

        if components.len() == pentry.materials().len() {
            dependency.components = components;
            entries.insert(*ptype_id, dependency);
        } else {
            queue.push_back(dependency);
        }
    }

    while let Some(pentry) = queue.pop_front() {
        let mut entry = pentry;
        let materials = products.get(&entry.ptype_id).unwrap().materials();

        let mut components = Vec::new();
        for material in materials.iter() {
            if products.contains_key(&material.type_id) && entries.contains_key(&material.type_id) {
                let mut entry = entries.get(&material.type_id).unwrap().clone();
                entry.quantity = material.quantity as u32;
                components.push(entry);
            } else if !products.contains_key(&material.type_id) {
                let ientry = items.get(&material.type_id).unwrap();
                let iname = ientry.name.get("en").unwrap().replace('\'', "''");
                let igroup_id = ientry.group_id;
                let icategory_id = groups.get(&igroup_id).unwrap().category_id;

                let dependency = Dependency {
                    blueprint_name: String::new(),
                    ptype_id: material.type_id,
                    btype_id: 0,
                    time: 0,
                    quantity: material.quantity as u32,
                    produces: 0,
                    info: DependencyInfo {
                        ptype_id: material.type_id,
                        category_id: icategory_id as u32,
                        group_id: igroup_id as u32,
                        name: iname.clone(),
                    },
                    typ: DependencyType::Material,
                    components: Vec::new(),
                };
                components.push(dependency);
            } else {
                queue.push_back(entry.clone());
                break;
            }
        }

        if components.len() == materials.len() {
            entry.components = components;
            entries.insert(entry.ptype_id, entry);
        } else {
            queue.push_back(entry);
        }
    }

    let entries = entries
        .values()
        .cloned()
        .map(|x| x.to_sql())
        .collect::<Vec<_>>();
    format!("INSERT INTO blueprint_json VALUES {};", entries.join(", "),)
}

/// Represents a single blueprint
#[derive(Clone, Debug, Default)]
struct BlueprintRaw {
    /// Uniqe id
    bp_id: Uuid,
    /// Blueprint type id
    btype_id: TypeId,
    /// Product type id
    ptype_id: TypeId,
    /// Quantity that is produced with each run
    quantity: i32,
}

impl BlueprintRaw {
    /// Converts the struct into a SQL-Insert Query.
    ///
    /// # Returns
    ///
    /// SQL-Value tuple for inserting.
    ///
    pub fn into_sql(self) -> String {
        format!(
            "('{}', {}, {}, {})",
            self.bp_id, self.btype_id, self.ptype_id, self.quantity
        )
    }
}

/// Represetns a single manufacture job
#[derive(Clone, Debug, Default)]
struct BlueprintManufacture {
    /// Uniqe id
    bp_id: Uuid,
    /// Blueprint type id
    btype_id: TypeId,
    /// Product type id
    ptype_id: TypeId,
    /// Quantity that is produced with each run
    quantity: i32,
    /// Time it takes to construct
    time: i32,
    /// Determines if this entry is a reaction
    reaction: bool,
}

impl BlueprintManufacture {
    /// Converts the struct into a SQL-Insert Query.
    ///
    /// # Returns
    ///
    /// SQL-Value tuple for inserting.
    ///
    pub fn into_sql(self) -> String {
        format!(
            "('{}', {}, {}, {}, {}, {})",
            self.bp_id, self.btype_id, self.ptype_id, self.time, self.reaction, self.quantity,
        )
    }
}

/// Represetns a single manufacture component
#[derive(Clone, Debug, Default)]
struct BlueprintManufactureComponent {
    /// Uniqe id
    bp_id: Uuid,
    /// Blueprint type id
    btype_id: TypeId,
    /// Product type id
    ptype_id: TypeId,
    /// Quantity that is produced with each run
    quantity: i32,
}

impl BlueprintManufactureComponent {
    /// Converts the struct into a SQL-Insert Query.
    ///
    /// # Returns
    ///
    /// SQL-Value tuple for inserting.
    ///
    pub fn into_sql(self) -> String {
        format!(
            "('{}', {}, {}, {})",
            self.bp_id, self.btype_id, self.ptype_id, self.quantity,
        )
    }
}

/// Represetns a single invention
#[derive(Clone, Debug, Default)]
struct BlueprintInvention {
    /// Unique id of the invention
    bp_id: Uuid,
    /// Blueprint type id
    btype_id: TypeId,
    /// Tier 1 product type id
    ttype_id: TypeId,
    /// Product type id
    ptype_id: TypeId,
    /// TypeId of the invented blueprint
    itype_id: TypeId,

    /// Time it takes to invent
    time: i32,
    /// Probability that the invention works
    probability: f32,
}

impl BlueprintInvention {
    /// Converts the struct into a SQL-Insert Query.
    ///
    /// # Returns
    ///
    /// SQL-Value tuple for inserting.
    ///
    pub fn into_sql(self) -> String {
        format!(
            "('{}', {}, {}, {}, {}, {}, {})",
            self.bp_id,
            self.btype_id,
            self.ptype_id,
            self.itype_id,
            self.ttype_id,
            self.time,
            self.probability
        )
    }
}

/// Represents a single blueprint
#[derive(Clone, Debug, Default)]
struct BlueprintResearch {
    /// Blueprint type id
    btype_id: TypeId,
    /// Product type id
    ptype_id: TypeId,

    /// Time to research material efficiency
    material: i32,
    /// Time to research time efficiency
    time: i32,
    /// Time to make a blueprint copy
    copy: i32,
}

impl BlueprintResearch {
    /// Converts the struct into a SQL-Insert Query.
    ///
    /// # Example
    ///
    /// ```
    /// (955, 608, 2100, 2100, 4800)
    /// ```
    ///
    /// # Returns
    ///
    /// SQL-Value tuple for inserting.
    ///
    pub fn into_sql(self) -> String {
        format!(
            "({}, {}, {}, {}, {})",
            self.btype_id, self.ptype_id, self.material, self.time, self.copy
        )
    }
}

/// Represents a material required for an invention
#[derive(Clone, Debug, Default)]
struct BlueprintMaterial {
    /// Unique id that references to [BlueprintInvention]
    bp_id: Uuid,
    /// Required quantity
    quantity: i32,
    /// TypeId of the material
    mtype_id: TypeId,
    /// Quantity that is produced by the product
    produces: i32,
    /// Time to research time efficiency
    time: i32,
}

impl BlueprintMaterial {
    /// Converts the struct into a SQL-Insert Query.
    ///
    /// # Returns
    ///
    /// SQL-Value tuple for inserting.
    ///
    pub fn into_sql(self) -> String {
        format!(
            "('{}', {}, {}, {}, {})",
            self.bp_id, self.mtype_id, self.produces, self.time, self.quantity,
        )
    }
}

/// Represents a blueprint taken from SDE
#[derive(Clone, Debug, Deserialize)]
struct Blueprint {
    /// Holds all activities that are possible with that blueprint
    activities: HashMap<ActivityName, Activity>,
}

impl Blueprint {
    /// Checks if the activity has reaction.
    ///
    /// # Returns
    ///
    /// * `true`  -> If the entry is a reaction
    /// * `false` -> If there are not reactions
    ///
    pub fn is_reaction(&self) -> bool {
        self.activities.get(&ActivityName::Reaction).is_some()
    }

    /// Checks if the blueprint has a manufacture or reaction job.
    ///
    /// # Returns
    ///
    /// * `true`  -> If there is either a manufacture or reaction job
    /// * `false` -> If there are not jobs
    ///
    pub fn has_job(&self) -> bool {
        let manufacture = self.activities.get(&ActivityName::Manufacturing);
        let reaction = self.activities.get(&ActivityName::Reaction);
        manufacture.is_some() || reaction.is_some()
    }

    /// Gets the product either from the manufacture job or the reaction job.
    ///
    /// # Returns
    ///
    /// * `None` -> If there is no product
    /// * `Some` -> TypeId of the product
    ///
    pub fn product(&self) -> Option<TypeId> {
        if let Some(x) = self.activities.get(&ActivityName::Manufacturing) {
            Some(x.products.get(0)?.type_id)
        } else if let Some(x) = self.activities.get(&ActivityName::Reaction) {
            Some(x.products.get(0)?.type_id)
        } else {
            None
        }
    }

    /// Gets the produced quantity of either a manufacturing or reaction job.
    ///
    /// # Returns
    ///
    /// * `None` -> If there is no product
    /// * `Some` -> Quantity of the produced product
    ///
    pub fn product_quantity(&self) -> Option<i32> {
        if let Some(x) = self.activities.get(&ActivityName::Manufacturing) {
            Some(x.products.get(0)?.quantity)
        } else if let Some(x) = self.activities.get(&ActivityName::Reaction) {
            Some(x.products.get(0)?.quantity)
        } else {
            None
        }
    }

    /// Gets the materials required for either manufacturing or reaction.
    ///
    /// # Returns
    ///
    /// List of all required materials. If there is no manufacturing or reaction
    /// job, an empty vec is returned.
    ///
    pub fn materials(&self) -> Vec<Material> {
        if let Some(x) = self.activities.get(&ActivityName::Manufacturing) {
            x.materials.clone()
        } else if let Some(x) = self.activities.get(&ActivityName::Reaction) {
            x.materials.clone()
        } else {
            Vec::new()
        }
    }

    /// Gets the time for a manufacture job.
    ///
    /// # Returns
    ///
    /// - `None` -> If the BP has no manufacture job
    /// - `Some` -> Time of the action
    ///
    pub fn manufacture_time(&self) -> Option<i32> {
        if !self.has_job() {
            return None;
        }

        if let Some(x) = self.activities.get(&ActivityName::Manufacturing) {
            Some(x.time)
        } else if let Some(x) = self.activities.get(&ActivityName::Reaction) {
            Some(x.time)
        } else {
            None
        }
    }

    /// Gets the time for a research job.
    ///
    /// # Returns
    ///
    /// - `None` -> If the BP has no research
    /// - `Some` -> Time of the action
    ///
    pub fn research_time(&self, activity: ActivityName) -> Option<i32> {
        if let Some(x) = self.activities.get(&activity) {
            if x.materials.is_empty() {
                Some(x.time)
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// All possible activity that a blueprint can have
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum ActivityName {
    /// Copy
    Copying,
    /// Invention
    Invention,
    /// Manufacture
    Manufacturing,
    /// Reaction
    Reaction,
    /// Material research
    ResearchMaterial,
    /// Time research
    ResearchTime,
}

/// Represents a sinble blueprints activity
#[derive(Clone, Debug, Deserialize)]
struct Activity {
    /// Time it takes to perform the activity
    time: i32,
    /// Required materials for the activity, will be an empty Vector if not
    /// materials are required
    #[serde(default)]
    materials: Vec<Material>,
    /// Products that are produced by this blueprint, will be an empty Vec if
    /// nothing is produced by this activity
    #[serde(default)]
    products: Vec<Material>,
}

/// Represents a material required for an activity
#[derive(Clone, Debug, Deserialize)]
struct Material {
    /// Quantity that is required
    quantity: i32,
    /// TypeId of the material that is required
    #[serde(rename = "typeID")]
    type_id: TypeId,

    /// This field is only set when the activity is an invention and there only
    /// for products
    probability: Option<f32>,
}
