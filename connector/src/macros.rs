/// Generator for EVE ID's.
///
/// Automatically derives a bunch of useful traits.
///
/// As an addition it implements [std::ops::Deref], [std::convert::From],
/// [std::convert::Into].
///
/// The generated new type struct is marked as an serde::transparent struct.
///
/// # Parameters
///
/// * `name` - Name of the ID
/// * `typ`  - Datatype of the ID (e.g. i32)
///
#[macro_export]
macro_rules! eve_id {
    ($name:ident, $typ:ty) => {
        /// Represents an ID-Type from EVE
        #[derive(
            Clone, Copy, Debug, Hash,
            PartialEq, Eq,
            PartialOrd, Ord,
            Deserialize, Serialize,
         )]
        #[serde(transparent)]
        #[cfg_attr(feature = "sqlx_types", derive(sqlx::Type))]
        #[cfg_attr(feature = "sqlx_types", sqlx(transparent))]
        pub struct $name(pub $typ);

        impl From<$typ> for $name {
            fn from(x: $typ) -> Self {
                Self(x)
            }
        }

        impl std::ops::Deref for $name {
            type Target = $typ;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(
                &self,
                f: &mut std::fmt::Formatter<'_>
            ) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}
