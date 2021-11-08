/// Automatically adds some base implementations to the given enum.
///
/// # Usage
///
/// error_derive!(NameOfTheEnum);
///
#[macro_export]
macro_rules! error_derive {
    ($error_enum:ident) => {
        impl std::error::Error for $error_enum {  }

        impl std::fmt::Display for $error_enum {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }

        impl From<sqlx::Error> for $error_enum {
            fn from(x: sqlx::Error) -> Self {
                Self::DatabaseError(x)
            }
        }
    };
}
