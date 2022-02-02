//! Creates the SQL-Code for blueprints
use crate::FOLDER_INPUT;

use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;

/// Wrapper for CategoryId
type CategoryId = i32;
/// Wrapper for GroupId
type GroupId     = i32;
/// Wrapper for TypeId
type TypeId     = i32;

/// Parses the input file and exports it as SQL
pub fn run() -> Result<String, Box<dyn std::error::Error>> {
    tracing::info!("Starting asset parsing");

    let current = std::env::current_dir()?;
    let current = current
        .to_str()
        .unwrap_or_default();
    let path_type_ids = format!(
        "{}/{}/type_ids.yaml",
        current,
        FOLDER_INPUT
    );
    let file_type_ids = File::open(&path_type_ids)?;

    let path_group_ids = format!(
        "{}/{}/group_ids.yaml",
        current,
        FOLDER_INPUT
    );
    let file_group_ids = File::open(&path_group_ids)?;

    let type_ids: HashMap<TypeId, TypeEntry> = serde_yaml::from_reader(file_type_ids)?;
    let group_ids: HashMap<GroupId, GroupEntry> = serde_yaml::from_reader(file_group_ids)?;

    let entries = vec![
        sql_header(),
        sql_items(&type_ids, &group_ids)
    ];

    Ok(entries.join("\n"))
}

/// Generates the basic SQL-Query that is required for items.
///
/// # Returns
///
/// String containing the SQL-Query.
///
fn sql_header() -> String {
    r#"DELETE FROM items CASCADE;"#.into()
}

/// Generates a SQL-Query containing all game items
/// 
/// # Returns
/// 
/// String containing the value-tuple
/// 
fn sql_items(
    type_ids:  &HashMap<TypeId, TypeEntry>,
    group_ids: &HashMap<GroupId, GroupEntry>,
) -> String {
    let mut items = Vec::new();

    for (type_id, entry) in type_ids {
        let type_id = *type_id;
        let group_id = entry.group_id;
        let category_id = group_ids
            .get(&group_id)
            .map(|x| x.category_id)
            .expect("Every entry should have a categroy id");
        let volume = entry.volume.unwrap_or(0f32);
        let meta_group_id = entry.meta_group_id;
        let name = entry
            .name()
            .unwrap_or(
                format!("Unknown name {}", type_id)
            );

        let item= Item {
            type_id,
            group_id,
            meta_group_id,
            category_id,
            volume,
            name,
        };
        items.push(item.into_sql());
    }

    format!("INSERT INTO items VALUES {};",
        items.join(", "),
    )
}

/// Represents a single item entry
#[derive(Clone, Debug)]
struct Item {
    /// TypeId of the item
    type_id:       TypeId,
    /// CategoryId of the item
    category_id:   TypeId,
    /// GroupId of the item
    group_id:      TypeId,
    /// MetaGroupId of the item
    meta_group_id: Option<TypeId>,
    /// Volume
    volume:        f32,
    /// English name of the item
    name:          String
}

impl Item {
    /// Converts the struct into a SQL-Insert Query.
    ///
    /// # Example
    ///
    /// ```text
    /// (35834, 65, 1657, 1, 800000.0, 'Keepstar')
    /// ```
    ///
    /// # Returns
    ///
    /// SQL-Value tuple for inserting.
    ///
    pub fn into_sql(self) -> String {
        format!(
            "({}, {}, {}, {}, {}, '{}')",
            self.type_id,
            self.category_id,
            self.group_id,
            self.meta_group_id.map_or("NULL".into(), |x| format!("{}", x)),
            self.volume,
            self.name.replace('\'', "''")
        )
    }
}

/// Represents a single entry in the yaml for a type
#[derive(Clone, Debug, Deserialize)]
pub struct TypeEntry {
    /// ID of the group this type belongs to
    #[serde(rename = "groupID")]
    pub group_id:                 GroupId,
    /// Name of the item in different languages
    #[serde(rename = "name")]
    pub name:                     HashMap<String, String>,
    /// ID of the group this type belongs to
    #[serde(rename = "metaGroupID")]
    pub meta_group_id:            Option<GroupId>,
    /// Volume of the type
    #[serde(rename = "volume")]
    pub volume:                   Option<f32>,
}

impl TypeEntry {
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
#[derive(Clone, Debug, Deserialize)]
pub struct GroupEntry {
    /// Id of the category
    #[serde(rename = "categoryID")]
    pub category_id: CategoryId,
}
