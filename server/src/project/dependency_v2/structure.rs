use caph_connector::TypeId;

/// Player defined structure
pub struct Structure {
    /// [TypeId] of the structure
    pub type_id: TypeId,
    /// List of [TypeId]'s of the installed rigs
    pub rigs: Vec<TypeId>,
}

impl Structure {
    pub fn new(
        type_id: TypeId,
        rigs:    Vec<TypeId>
    ) -> Self {
        Self {
            type_id,
            rigs
        }
    }
}
