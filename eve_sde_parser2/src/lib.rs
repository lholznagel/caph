mod error;
mod service;

pub use self::error::*;
pub use self::service::*;

use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::sync::Arc;
use std::{collections::HashMap, io::Read};
use tokio::sync::RwLock;
use zip::ZipArchive;

pub type SdeZipArchive = ZipArchive<Cursor<Vec<u8>>>;

/// Macro for generating a function that returns the given service for the 
/// given [SdeServiceName]
///
/// # Parameters
///
/// * `name`         - name of the function eg. `type_ids`
/// * `service_name` - name of the service eg. `TypeIds`
/// * `service`      - service struct `TypeIdService`
/// 
/// # Example
///
/// ``` rust
/// service_loader_gen!(type_ids, TypeIds, TypeIdService);
/// ```
///
macro_rules! service_loader_gen {
    ($name:ident, $service_name:ident, $service:ident) => {
        pub async fn $name(&self) -> Result<$service, EveSdeParserError> {
            if let SdeService::$service_name(x) = self
                .get(SdeServiceName::$service_name)
                .await? {
                Ok(x)
            } else {
                Err(EveSdeParserError::LoadingService)
            }
        }
    };
}

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

/// Generates the necessary code for a service implementation
#[macro_export]
macro_rules! service_gen {
    () => {
        pub(crate) fn new(mut zip: SdeZipArchive) -> Result<Self, EveSdeParserError> {
            let mut file = zip.by_name(Self::PATH)?;
            let mut buf = Vec::with_capacity(file.size() as usize);
            file.read_to_end(&mut buf)?;
            Ok(Self(serde_yaml::from_slice(&buf)?))
        }
    };
}

#[derive(Clone)]
pub struct SdeServiceLoader {
    services: Arc<RwLock<HashMap<SdeServiceName, SdeService>>>,
    zip: SdeZipArchive,
}

impl SdeServiceLoader {
    const ZIP_URL:  &'static str = "https://eve-static-data-export.s3-eu-west-1.amazonaws.com/tranquility/sde.zip";

    /// Creates a new service loader instance.
    ///
    /// Downloads the zip archive from eve.
    pub async fn new() -> Result<Self, EveSdeParserError> {
        let zip = reqwest::get(Self::ZIP_URL)
            .await?
            .bytes()
            .await
            .map(|x| x.to_vec())
            .map(Cursor::new)?;
        Ok(Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            zip:      ZipArchive::new(zip)?,
        })
    }

    service_loader_gen!(blueprints, Blueprints, BlueprintService);
    service_loader_gen!(categories, Categories, CategoryService);
    service_loader_gen!(corporations, Corporations, CorporationService);
    service_loader_gen!(dogma, Dogmas, DogmaService);
    service_loader_gen!(groups, Groups, GroupService);
    service_loader_gen!(meta_groups, MetaGroups, MetaGroupService);
    service_loader_gen!(names, Names, NameService);
    service_loader_gen!(planet_schematics, PlanetSchematics, PlanceSchematicService);
    service_loader_gen!(races, Races, RaceService);
    service_loader_gen!(research_agents, ResearchAgents, ResearchAgentService);
    service_loader_gen!(skins, Skins, SkinService);
    service_loader_gen!(stations, Stations, StationService);
    service_loader_gen!(type_ids, TypeIds, TypeIdService);
    service_loader_gen!(type_materials, TypeMaterials, TypeMaterialService);

    /// Gets a specific service. If the service is not loaded yet, it will
    /// be read from the zip file and stored for later use.
    ///
    /// The first call of a unloaded service will cost a little more, if this
    /// is a problem use [SdeServiceLoader::preload] to force load them.
    async fn get(&self, service_name: SdeServiceName) -> Result<SdeService, EveSdeParserError> {
        let services_copy = { self.services.read().await.clone() };
        if let Some(x) = services_copy.get(&service_name) {
            Ok(x.clone())
        } else {
            let service = service_name.service(self.zip.clone())?;
            self.services.write().await.insert(service_name, service.clone());

            Ok(service)
        }
    }
}

macro_rules! eve_id {
    ($name:ident, $typ:ty) => {
        #[derive(Clone, Copy, Debug, Deserialize, Serialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
        #[serde(transparent)]
        pub struct $name(pub $typ);

        impl std::ops::Deref for $name {
            type Target = $typ;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

eve_id!(ActivityId, u32);
eve_id!(AgentId, u32);
eve_id!(AttributeId, u32);
eve_id!(CategoryId, u32);
eve_id!(CorporationId, u32);
eve_id!(ConstellationId, u32);
eve_id!(DisplayNameId, u32);
eve_id!(DivisionId, u32);
eve_id!(DogmaCategoryId, u32);
eve_id!(EffectId, u32);
eve_id!(FactionId, u32);
eve_id!(GraphicId, u32);
eve_id!(GroupId, u32);
eve_id!(IconId, u32);
eve_id!(MarketGroupId, u32);
eve_id!(MaterialSetId, u32);
eve_id!(MetaGroupId, u32);
eve_id!(PlayerId, u32);
eve_id!(RaceId, u32);
eve_id!(SchematicId, u32);
eve_id!(ServiceId, u32);
eve_id!(SkinId, u32);
eve_id!(SkinLicenseId, u32);
eve_id!(SkinMaterialId, u32);
eve_id!(SolarSystemId, u32);
eve_id!(SoundId, u32);
eve_id!(StationId, u32);
eve_id!(TypeId, u32);
eve_id!(UnitId, u32);
eve_id!(OperationId, u32);
eve_id!(RegionId, u32);
