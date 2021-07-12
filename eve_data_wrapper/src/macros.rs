/// Generates code for parsing zip files
///
/// # Parameters
///
/// * `zip`  - actual zip variable
/// * `path` - path that should be parsed
///
#[macro_export]
macro_rules! service_file_gen {
    ($zip:expr, $path:expr) => {
        {
            let mut file = $zip.by_name($path)?;
            let mut buf = Vec::with_capacity(file.size() as usize);
            file.read_to_end(&mut buf)?;
            serde_yaml::from_slice(&buf)?
        }
    }
}

/// Macro for generating a function that returns the given service for the 
/// given [crate::ServiceGroupName]
///
/// # Parameters
///
/// * `name`         - name of the function eg. `type_ids`
/// * `service_name` - name of the service eg. `TypeIds`
/// * `service`      - service struct `TypeIdService`
///
#[macro_export]
macro_rules! service_loader_gen {
    ($name:ident, $service_name:ident, $service:ident) => {
        pub async fn $name(&self) -> Result<$service, EveConnectError> {
            if let ServiceGroup::$service_name(x) = self
                .get(ServiceGroupName::$service_name)
                .await? {
                Ok(x)
            } else {
                Err(EveConnectError::LoadingService)
            }
        }
    };
}

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
/// * `typ`  - Datatype of the ID
///
#[macro_export]
macro_rules! eve_id {
    ($name:ident, $typ:ty) => {
        #[derive(
            Clone, Copy, Debug, Hash,
            PartialEq, Eq,
            PartialOrd, Ord,
            Deserialize, Serialize,
         )]
        #[serde(transparent)]
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

        impl std::str::FromStr for $name {
            type Err = EveConnectError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                s
                    .parse()
                    .map(Self)
                    .map_err(|_| EveConnectError::CannotParse)
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

        #[async_trait::async_trait]
        impl cachem::Parse for $name {
            async fn read<B>(
                buf: &mut B
            ) -> Result<Self, cachem::CachemError>
            where
                B: tokio::io::AsyncBufRead + tokio::io::AsyncRead + Send + Unpin {

                Ok(Self(<$typ>::read(buf).await?))
            }

            async fn write<B>(
                &self,
                buf: &mut B
            ) -> Result<(), cachem::CachemError>
            where
                B: tokio::io::AsyncWrite + Send + Unpin {

                self.0.write(buf).await?;
                Ok(())
            }
        }
    };
}
