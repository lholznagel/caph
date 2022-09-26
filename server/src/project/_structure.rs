use serde::{Deserialize, Serialize};
use crate::DependencyType;

/// General information about a structure
#[derive(Debug, Deserialize, Serialize)]
#[deprecated]
pub struct Structure {
    /// Name of the structure ingame
    /// TODO: figure out how to get ingame names
    pub name:     String,
    /// Name of the system the station is located in
    pub system:   String,
    /// Securtiy level the structure is located in
    pub security: Security,
    /// Type of structure
    pub typ:      StructureType,
    /// Rigs that the structure has
    pub rigs:     Vec<StructureRig>
}

impl Structure {
    pub fn new(
        name:     String,
        system:   String,
        security: Security,
        typ:      StructureType,
        rigs:     Vec<StructureRig>,
    ) -> Self {
        Self {
            name,
            system,
            security,
            typ,
            rigs,
        }
    }

    /// Calculates all bonuses in the order they should be applied in
    pub fn calculate_bonus(&self) -> Vec<BonusVariations> {
        let security_modifier = self.rig_bonus_modifier();
        let mut structure_bonus = self.typ.bonus();

        for rig in self.rigs.iter() {
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
    pub fn rig_bonus_modifier(&self) -> f32 {
        match (&self.security, &self.typ) {
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
        }
    }
}

/// Determines in what security status the system is located in
#[derive(Debug, Deserialize, Serialize)]
#[deprecated]
pub enum Security {
    Highsec,
    Lowsec,
    #[serde(rename = "Nullsec/Wormhole")]
    Nullsec,
}

/// Different types of structure sizes
#[derive(Debug, Deserialize, Serialize)]
#[deprecated]
pub enum StructureSize {
    M,
    L,
    XL,
}

/// Different engineering and refinery structures
#[derive(Debug, Deserialize, Serialize)]
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
}

impl StructureType {
    /// Gets the size of the structure
    pub fn size(&self) -> StructureSize {
        match self {
            // Refineries
            Self::Athanor => StructureSize::M,
            Self::Tatara  => StructureSize::L,

            // Engineering
            Self::Raitaru => StructureSize::M,
            Self::Azbel   => StructureSize::L,
            Self::Sotiyo  => StructureSize::XL,
        }
    }

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
        }
    }
}

/// Rigs that can be installed on a structure
#[derive(Debug, Deserialize, Serialize)]
#[deprecated]
pub enum StructureRig {
    // Refinery Size: L
    /// https://everef.net/type/46497
    ReactorEfficiencyII,

    // Azbel
    /// https://everef.net/type/37174
    AdvancedComponentManufacturingEfficiencyI,

    /// https://everef.net/type/37173
    CapitalShipManufacturingEfficiencyI,

    /// https://everef.net/type/43718
    CapitalComponentManufacturingEfficiencyI,
}

impl StructureRig {
    /// Bonuses that are apply by installing the rig
    pub fn bonus(&self) -> Vec<BonusVariations> {
        match self {
            Self::ReactorEfficiencyII => vec![
                BonusVariations::Time(24.00f32),
                BonusVariations::Material(2.40f32)
            ],
            Self::AdvancedComponentManufacturingEfficiencyI => vec![
                BonusVariations::Time(20.00f32),
                BonusVariations::Material(2.00f32)
            ],
            Self::CapitalShipManufacturingEfficiencyI => vec![
                BonusVariations::Time(20.00f32),
                BonusVariations::Material(2.00f32)
            ],
            Self::CapitalComponentManufacturingEfficiencyI => vec![
                BonusVariations::Time(20.00f32),
                BonusVariations::Material(2.00f32)
            ],
        }
    }

    pub fn has_group(&self, group: usize) -> bool {
        match self {
            Self::ReactorEfficiencyII => vec![
                428, 429, 712, 974, 4096
            ].contains(&group),
            Self::AdvancedComponentManufacturingEfficiencyI => vec![
                332, 334, 716, 913, 964,
            ].contains(&group),
            Self::CapitalShipManufacturingEfficiencyI => vec![
                485, 547, 883, 1538,
            ].contains(&group),
            Self::CapitalComponentManufacturingEfficiencyI => vec![
                873,
            ].contains(&group),
        }
    }
}

/// Different kind of bonuses a blueprint, structure or rigs can have
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[deprecated]
pub enum BonusVariations {
    Time(f32),
    Material(f32),
    Isk(f32),
}

impl BonusVariations {
    pub fn apply_security_bonus(
        self,
        bonus: f32,
    ) -> Self {
        match self {
            Self::Time(x)     => Self::Time(
                f32::trunc(x * bonus * 100.0) / 100.0
            ),
            Self::Material(x) => Self::Material(
                f32::trunc(x * bonus * 100.0) / 100.0
            ),
            Self::Isk(x)      => Self::Isk(
                f32::trunc(x * bonus * 100.0) / 100.0
            ),
        }
    }
}

#[cfg(test)]
mod structure_tests {
    use super::*;

    #[test]
    fn tatara_material_bonus() {
        let structure = Structure::new(
            "a".into(),
            "b".into(),
            Security::Nullsec,
            StructureType::Tatara,
            vec![StructureRig::ReactorEfficiencyII]
        )
        .calculate_bonus();

        assert_eq!(structure.len(), 3);
        assert_eq!(structure[0], BonusVariations::Time(25.00f32));
        assert_eq!(structure[1], BonusVariations::Time(26.40f32));
        assert_eq!(structure[2], BonusVariations::Material(2.64f32));
    }
}
