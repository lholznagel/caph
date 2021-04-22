//! Wraps the EVE Online API and EVE SDE into a single lib and exposes it
//! as a single EVE resource.
//!
//! The idea is that the data is grouped into multiple services that are managed
//! by the lib.
//! If another lib / bin wants to use a specific service, it needs to call the
//! function on a [EveDataWrapper] instance and the service is returned.
//!
//! TODO: also use sde.hoboleaks.space
//!
//! TODO: join meta_groups and groups?
//!
//! TODO: add task that periodically downloads the zip
//!
mod eve_client;
mod error;
mod macros;
mod service;

pub use self::eve_client::*;
pub use self::error::*;
pub use self::service::*;

use chrono::{NaiveDateTime, NaiveTime, Utc};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::{io::Cursor, time::Duration};
use std::sync::Arc;
use std::{collections::HashMap, io::Read};
use tokio::sync::RwLock;
use zip::ZipArchive;

/// Type alias for `ZipArchive<Cursor<Vec<u8>>>`
pub(crate) type SdeZipArchive = ZipArchive<Cursor<Vec<u8>>>;

/// Takes a path and a zip file and parses the file content into a defined
/// structure.
///
/// # Parameters
///
/// * `T`    - Type the file should be parsed to (in most cases rust figures
///            out the type)
/// * `path` - Path in the zip file for the file to parse
/// * `zip`  - Zip file that contains the file
///
/// # Returns
///
/// Parsed yaml version of the file, based on the generic parameter `T`
///
pub(crate) fn parse_zip_file<T>(
    path: &str,
    zip: &mut SdeZipArchive
) -> Result<T, EveConnectError>
    where T: DeserializeOwned {

    let mut file = zip.by_name(path)?;
    let mut buf = Vec::with_capacity(file.size() as usize);
    file.read_to_end(&mut buf)?;
    serde_yaml::from_slice(&buf).map_err(Into::into)
}

#[derive(Clone)]
pub struct EveDataWrapper {
    /// Client for communicating with eve
    eve_client: EveClient,

    /// Stores all services that are managed by this lib
    services:   Arc<RwLock<HashMap<ServiceGroupName, ServiceGroup>>>,

    /// Not all files are parsed from the zip file, so we keep it in memory
    zip:        SdeZipArchive,
}

impl EveDataWrapper {
    const ZIP_URL: &'static str = "https://eve-static-data-export.s3-eu-west-1.amazonaws.com/tranquility/sde.zip";

    /// Creates a new service loader instance.
    ///
    /// Downloads the zip archive from eve.
    pub async fn new() -> Result<Self, EveConnectError> {
        let zip = reqwest::get(Self::ZIP_URL)
            .await?
            .bytes()
            .await
            .map(|x| x.to_vec())
            .map(Cursor::new)?;
        let x = Self {
            eve_client: EveClient::new()?,
            services:   Arc::new(RwLock::new(HashMap::new())),
            zip:        ZipArchive::new(zip)?,
        };

        // Preload
        x.get(ServiceGroupName::Types).await?;

        Ok(x)
    }


    /// Creates a new duration to the next 14:30:00 time.
    ///
    /// Eve´s downtime is at 14:00, so giving them 30 minutes should be ok.
    pub async fn task() {
        // Current timestamp
        let timestamp = Utc::now().timestamp();
        // Create a naive date time and add one day to it
        let date_time = NaiveDateTime::from_timestamp(timestamp as i64, 0);
        let date_time = date_time.checked_add_signed(chrono::Duration::days(1)).unwrap();

        // Creates a new naive date time based on the date time that is one day
        // ahead. We take the date and set the hms to 14:30:00.
        let next = NaiveDateTime::new(
            date_time.date(),
            NaiveTime::from_hms(14, 30, 0)
        )
        .timestamp();

        // Execute at exactly 14:30
        let diff = next - timestamp;
        Duration::from_secs(diff as u64);
    }

    service_loader_gen!(blueprints, Blueprints, BlueprintService);
    service_loader_gen!(categories, Categories, CategoryService);
    service_loader_gen!(character, Character, CharacterService);
    service_loader_gen!(corporations, Corporations, CorporationService);
    service_loader_gen!(dogma, Dogmas, DogmaService);
    service_loader_gen!(groups, Groups, GroupService);
    service_loader_gen!(meta_groups, MetaGroups, MetaGroupService);
    service_loader_gen!(market, Market, MarketService);
    service_loader_gen!(names, Names, NameService);
    service_loader_gen!(planet_schematics, PlanetSchematics, PlanceSchematicService);
    service_loader_gen!(races, Races, RaceService);
    service_loader_gen!(research_agents, ResearchAgents, ResearchAgentService);
    service_loader_gen!(skins, Skins, SkinService);
    service_loader_gen!(stations, Stations, StationService);
    service_loader_gen!(systems, Systems, SystemService);
    service_loader_gen!(types, Types, TypeService);

    /// Gets a specific service. If the service is not loaded yet, it will
    /// be read from the zip file and stored for later use.
    ///
    async fn get(&self, service_name: ServiceGroupName) -> Result<ServiceGroup, EveConnectError> {
        let services_copy = { self.services.read().await.clone() };
        if let Some(x) = services_copy.get(&service_name) {
            Ok(x.clone())
        } else {
            let service = service_name.service(self.eve_client.clone(), self.zip.clone()).await?;
            self.services.write().await.insert(service_name, service.clone());

            Ok(service)
        }
    }
}

// TODO: validate if all are needed or if some can be merged
eve_id!(ActivityId, u32);
eve_id!(AgentId, u32);
eve_id!(AttributeId, u32);
eve_id!(CategoryId, u32);
eve_id!(ConstellationId, u32);
eve_id!(CorporationId, u32);
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
eve_id!(OperationId, u32);
eve_id!(PlanetId, u32);
eve_id!(PlayerId, u32);
eve_id!(RaceId, u32);
eve_id!(RegionId, u32);
eve_id!(StarId, u32);
eve_id!(SchematicId, u32);
eve_id!(ServiceId, u32);
eve_id!(SkinId, u32);
eve_id!(SkinLicenseId, u32);
eve_id!(SkinMaterialId, u32);
eve_id!(SolarSystemId, u32);
eve_id!(SoundId, u32);
eve_id!(StargateId, u32);
eve_id!(StationId, u32);
eve_id!(TypeId, u32);
eve_id!(UnitId, u32);
