use caph_connector::TypeId;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use super::error::StructureError;

/// Determines in what security status the system is located in
/// 
#[derive(Clone, Debug, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "SYSTEM_SECURITY")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Security {
    Highsec,
    Lowsec,
    #[serde(rename = "Nullsec/Wormhole")]
    Nullsec,
}

/// Represents the enum `SYSTEM_SECURITY` in PostgresSQL
/// 
#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "SYSTEM_SECURITY")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StructureBonusModifier {
    ManufactureMaterial,
    ManufactureTime,

    ReactionMaterial,
    ReactionTime,
}

/// Represents a structure with all its installed rigs and bonis
/// 
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Structure {
    /// Internal id of the structure
    pub id:        Uuid,
    /// Name of the structure
    /// TODO:: get ingame name of all alliance/corp structures?
    pub name:      String,
    /// Location of the strucutre
    pub system:    String,
    /// Security of the location the structure is in
    pub security:  Security,

    /// Type of structure
    pub structure: StructureType,
    /// List of all rigs that are in the structure
    pub rigs:      Vec<StructureRig>,
}

impl Structure {
    pub fn new(
        id:        Uuid,
        name:      String,
        system:    String,
        security:  Security,

        structure: StructureType,
        rigs:      Vec<StructureRig>,
    ) -> Self {
        Self {
            id,
            name,
            system,
            security,
            structure,
            rigs,
        }
    }

    /// Calculates all bonuses in the order they should be applied in
    #[deprecated]
    pub fn calculate_bonus(
        &self
    ) -> Vec<BonusVariations> {
        let security_modifier = self.rig_bonus_modifier();
        let mut structure_bonus = self.structure.bonus();

        for rig in self.rigs.iter() {
            // FIXME: resprect categories/groups? StructureRig::has_group
            let bonuses = rig
                .bonus()
                .into_iter()
                .map(|x| x.apply_security_bonus(security_modifier))
                .collect::<Vec<_>>();
            structure_bonus.extend(bonuses);
        }

        structure_bonus
    }

    /// Additional bonus appliad by the security status and the strucutre
    pub fn rig_bonus_modifier(
        &self
    ) -> f32 {
        match (&self.security, &self.structure) {
            // Refinery
            (Security::Highsec, StructureType::Athanor) |
            (Security::Highsec, StructureType::Tatara)  => 0f32,
            (Security::Lowsec,  StructureType::Athanor) |
            (Security::Lowsec,  StructureType::Tatara)  => 1.0f32,
            (Security::Nullsec, StructureType::Athanor) |
            (Security::Nullsec, StructureType::Tatara)  => 1.1f32,

            // Engineering
            (Security::Highsec, StructureType::Raitaru) |
            (Security::Highsec, StructureType::Azbel)   |
            (Security::Highsec, StructureType::Sotiyo)  => 1.0f32,
            (Security::Lowsec,  StructureType::Raitaru) |
            (Security::Lowsec,  StructureType::Azbel)   |
            (Security::Lowsec,  StructureType::Sotiyo)  => 1.9f32,
            (Security::Nullsec, StructureType::Raitaru) |
            (Security::Nullsec, StructureType::Azbel)   |
            (Security::Nullsec, StructureType::Sotiyo)  => 2.1f32,

            // Invalid
            _                                           => 0f32,
        }
    }

    // TODO: implement for structures
    pub fn category_groups(
        &self,
    ) -> Vec<usize> {
        self
            .rigs
            .iter()
            .map(|x| x.category_groups.clone())
            .flatten()
            .collect::<Vec<_>>()
    }

    pub fn rigs(
        &self,
    ) -> Vec<StructureRig> {
        self
            .rigs
            .iter()
            .map(|x| {
                x.material.map(|y| y * self.rig_bonus_modifier());
                x.time.map(|y| y * self.rig_bonus_modifier());

                StructureRig {
                    material: x.material.map(|y| y * self.rig_bonus_modifier()),
                    time:     x.time.map(|y| y * self.rig_bonus_modifier()),
                    category_groups: x.category_groups.clone(),
                }
            })
            .collect::<Vec<_>>()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum StructureType {
    /// https://everef.net/type/35835
    Athanor,
    /// https://everef.net/type/35836
    Tatara,

    /// https://everef.net/type/35825
    Raitaru,
    /// https://everef.net/type/35826
    Azbel,
    /// https://everef.net/type/35827
    Sotiyo,

    Invalid,
}

impl StructureType {
    /// Bonuses applied by the structure itself
    pub fn bonus(&self) -> Vec<BonusVariations> {
        match self {
            // Refinery
            Self::Athanor => vec![],
            Self::Tatara  => vec![
                BonusVariations::Time(25.00f32),
            ],

            // Engineering
            Self::Raitaru => vec![
                BonusVariations::Time(15.00f32),
                BonusVariations::Material(1.00f32),
                BonusVariations::Isk(3.00f32),
            ],
            Self::Azbel   => vec![
                BonusVariations::Time(20.00f32),
                BonusVariations::Material(1.00f32),
                BonusVariations::Isk(4.00f32),
            ],
            Self::Sotiyo  => vec![
                BonusVariations::Time(30.00f32),
                BonusVariations::Material(1.00f32),
                BonusVariations::Isk(5.00f32),
            ],
            _             => Vec::new()
        }
    }
}

impl From<i32> for StructureType {
    fn from(x: i32) -> Self {
        match x {
            35835 => Self::Athanor,
            35836 => Self::Tatara,

            35825 => Self::Raitaru,
            35826 => Self::Azbel,
            35827 => Self::Sotiyo,

            _     => Self::Invalid,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct StructureRig {
    pub material: Option<f32>,
    pub time:     Option<f32>,

    category_groups: Vec<usize>,
}

impl StructureRig {
    pub async fn new(
        pool: &PgPool,
        rid:  TypeId,
    ) -> Result<Self, StructureError> {
        let mut _self = Self::default();

        let bonuses = sqlx::query!(r#"
                SELECT
                    modifier AS "modifier!: StructureBonusModifier",
                    amount,
                    categories,
                    groups
                FROM structure_dogma
                WHERE ptype_id = $1
            "#,
                *rid
            )
            .fetch_all(pool)
            .await
            .map_err(|_| StructureError::RigNotFound(rid))?;

        for bonus in bonuses {
            match bonus.modifier {
                StructureBonusModifier::ManufactureMaterial |
                StructureBonusModifier::ReactionMaterial    => {
                    _self.material = Some(bonus.amount as f32);
                },
                StructureBonusModifier::ManufactureTime |
                StructureBonusModifier::ReactionTime    => {
                    _self.time = Some(bonus.amount as f32);
                }
            }

            if _self.category_groups.is_empty() {
                let mut cg = Vec::new();
                cg.extend(
                    bonus
                        .categories
                        .into_iter()
                        .map(|x| x as usize)
                        .collect::<Vec<_>>()
                );
                cg.extend(
                    bonus
                        .groups
                        .into_iter()
                        .map(|x| x as usize)
                        .collect::<Vec<_>>()
                );
                _self.category_groups = cg;
            }
        }

        Ok(_self)
    }

    pub fn bonus(
        &self,
    ) -> Vec<BonusVariations> {
        let mut bonis = Vec::new();

        if let Some(x) = self.material {
            bonis.push(BonusVariations::Material(x));
        }

        if let Some(x) = self.time {
            bonis.push(BonusVariations::Time(x));
        }

        bonis
    }

    pub fn has_category_or_group(
        &self,
        cg: usize
    ) -> bool {
        self.category_groups.is_empty() ||
        self.category_groups.contains(&cg)
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum BonusVariations {
    Time(f32),
    Material(f32),
    Isk(f32),
}

impl BonusVariations {
    pub fn apply_security_bonus(
        self,
        security: f32,
    ) -> Self {
        match self {
            Self::Time(x)     => Self::Time(
                f32::trunc(x * security * 100.0) / 100.0,
            ),
            Self::Material(x) => Self::Material(
                f32::trunc(x * security * 100.0) / 100.0,
            ),
            Self::Isk(x)      => Self::Isk(
                f32::trunc(x * security * 100.0) / 100.0,
            ),
        }
    }
}

#[cfg(test)]
mod structure_tests {
    use caph_connector::TypeId;
    use sqlx::postgres::PgPoolOptions;
    use uuid::Uuid;

    use super::*;

    #[tokio::test]
    async fn tatara_material_bonus() {
        dotenvy::dotenv().ok();
        let pg_addr = std::env::var("DATABASE_URL").unwrap();
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&pg_addr)
            .await
            .unwrap();

        let structure = Structure {
            id:        Uuid::new_v4(),
            name:      "a".into(),
            system:    "b".into(),
            security:  Security::Nullsec,
            structure: StructureType::Tatara,
            rigs:      vec![
                StructureRig::new(&pool, TypeId::from(46497)).await.unwrap()
            ]
        };

        let structure = structure.calculate_bonus();

        assert_eq!(structure.len(), 3);
        assert_eq!(structure[0], BonusVariations::Time(25.00f32));
        assert_eq!(structure[1], BonusVariations::Material(2.64f32));
        assert_eq!(structure[2], BonusVariations::Time(26.40f32));
    }
}
