#![deny(missing_docs)]

//! API-Server for the frontend

mod alliance;
mod blueprint;
mod character;
mod corporation;
mod error;
mod eve;
mod industry;
mod item;
mod name;
mod project;

use crate::alliance::AllianceService;
use crate::blueprint::BlueprintService;
use crate::character::CharacterService;
use crate::corporation::CorporationService;
use crate::industry::IndustryService;
use crate::item::ItemService;
use crate::name::NameService;
use crate::project::ProjectService;

use self::eve::*;

use alliance::NewAllianceFitting;
use cachem::ConnectionPool;
use caph_db::CorporationBlueprintEntry;
use caph_eve_data_wrapper::{CorporationId, EveDataWrapper, TypeId};
use project::ProjectNew;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use warp::http::Response;
use warp::hyper::StatusCode;
use warp::{Filter, Rejection, Reply};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    morgan::Morgan::init(vec!["tracing".into()]);

    let pool     = ConnectionPool::new("0.0.0.0:55555", 100usize).await?;
    let eve_data = EveDataWrapper::new().await?;

    let eve_auth  = EveAuthService::new(pool.clone(), eve_data.clone());
    let industry  = IndustryService::new(eve_auth.clone(), eve_data.clone());

    let blueprint   = BlueprintService::new(pool.clone(), eve_auth.clone(), industry.clone());
    let character   = CharacterService::new(pool.clone(), eve_auth.clone(), eve_data.clone());
    let corporation = CorporationService::new(pool.clone(), eve_auth.clone());
    let item        = ItemService::new(pool.clone());
    let alliance    = AllianceService::new(pool.clone(), eve_auth.clone(), character.clone(), item.clone());
    let name        = NameService::new(pool.clone());
    let project     = ProjectService::new(pool.clone(), blueprint.clone(), character.clone(), eve_auth.clone());

    log::info!("Starting server");

    ApiServer::new(
        eve_auth,

        alliance,
        blueprint,
        character,
        corporation,
        industry,
        item,
        name,
        project,
    )
    .serve()
    .await;

    Ok(())
}

/// Contains all services and handles routing
#[derive(Clone)]
pub struct ApiServer {
    eve_auth:  EveAuthService,

    alliance:    AllianceService,
    blueprint:   BlueprintService,
    character:   CharacterService,
    corporation: CorporationService,
    industry:    IndustryService,
    item:        ItemService,
    name:        NameService,
    project:     ProjectService,
}

impl ApiServer {
    /// Creates a new api server instance
    pub fn new(
        eve_auth:  EveAuthService,

        alliance:    AllianceService,
        blueprint:   BlueprintService,
        character:   CharacterService,
        corporation: CorporationService,
        industry:    IndustryService,
        item:        ItemService,
        name:        NameService,
        project:     ProjectService,
    ) -> Self {
        Self {
            eve_auth,

            alliance,
            blueprint,
            character,
            corporation,
            industry,
            item,
            name,
            project,
        }
    }

    /// Exposes all routes
    ///
    /// This function is blocking
    pub async fn serve(&self) {
        let _self = Arc::new(self.clone());
        let log = warp::log::custom(|info| {
            log::info!(
                "{} {} {} {}ms",
                info.method(),
                info.path(),
                info.status(),
                info.elapsed().as_millis()
            );
        });

        let root = warp::any()
            .map(move || _self.clone())
            .and(warp::path!("api" / ..));

        let alliance = root
            .clone()
            .and(warp::path!("alliance" / ..));
        let alliance_get_fittings = alliance
            .clone()
            .and(warp::path!("fittings"))
            .and(warp::get())
            .and(warp::cookie("token"))
            .and_then(Self::alliance_get_fittings);
        let alliance_get_fitting = alliance
            .clone()
            .and(warp::path!("fittings" / Uuid))
            .and(warp::get())
            .and(warp::cookie("token"))
            .and_then(Self::alliance_get_fitting);
        let alliance_set_fittings = alliance
            .clone()
            .and(warp::path!("fittings"))
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::cookie("token"))
            .and_then(Self::alliance_set_fitting);
        let alliance_del_fitting = alliance
            .clone()
            .and(warp::path!("fittings" / Uuid))
            .and(warp::delete())
            .and(warp::cookie("token"))
            .and_then(Self::alliance_del_fitting);
        let alliance = alliance_get_fittings
            .or(alliance_get_fitting)
            .or(alliance_set_fittings)
            .or(alliance_del_fitting);

        let blueprint = root
            .clone()
            .and(warp::path!("blueprint" / ..));
        let blueprint_all = blueprint
            .clone()
            .and(warp::path::end())
            .and(warp::get())
            .and_then(Self::blueprint_all);
        let blueprint_by_id = blueprint
            .clone()
            .and(warp::path!(TypeId))
            .and(warp::get())
            .and_then(Self::blueprint_get);
        let blueprint = blueprint_all
            .or(blueprint_by_id);

        let character = root
            .clone()
            .and(warp::path!("character" / ..));
        let character_assets = character
            .clone()
            .and(warp::path!("assets"))
            .and(warp::get())
            .and(warp::cookie("token"))
            .and_then(Self::character_assets);
        let character_blueprints = character
            .clone()
            .and(warp::path!("blueprints"))
            .and(warp::get())
            .and(warp::cookie("token"))
            .and_then(Self::character_blueprints);
        let character_info = character
            .clone()
            .and(warp::path!("info"))
            .and(warp::get())
            .and(warp::cookie("token"))
            .and_then(Self::character_info);
        let character_item_location = character
            .clone()
            .and(warp::path!("location" / u64))
            .and(warp::get())
            .and(warp::cookie("token"))
            .and_then(Self::character_item_location);
        let character = character_assets
            .or(character_blueprints)
            .or(character_info)
            .or(character_item_location);

        let corporation = root
            .clone()
            .and(warp::path!("corporation" / ..));
        let corporation_blueprints = corporation
            .clone()
            .and(warp::path!(CorporationId / "blueprints"))
            .and(warp::get())
            .and(warp::cookie("token"))
            .and_then(Self::corporation_blueprints);
        let corporation_set_blueprints = corporation
            .clone()
            .and(warp::path!(CorporationId / "blueprints"))
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::cookie("token"))
            .and_then(Self::corporation_set_blueprints);
        let corporation_del_blueprints = corporation
            .clone()
            .and(warp::path!(CorporationId / "blueprints"))
            .and(warp::delete())
            .and(warp::cookie("token"))
            .and_then(Self::corporation_delete_blueprints);
        let corporation = corporation_blueprints
            .or(corporation_set_blueprints)
            .or(corporation_del_blueprints);

        let eve = root
            .clone()
            .and(warp::path!("eve" / ..));
        let eve_auth = eve
            .clone()
            .and(warp::path!("auth"))
            .and(warp::get())
            .and(warp::query())
            .and_then(Self::eve_auth);
        let eve_login = eve
            .clone()
            .and(warp::path!("login"))
            .and(warp::get())
            .and_then(Self::eve_login);
        let eve_login_alt = eve
            .clone()
            .and(warp::path!("login" / "alt"))
            .and(warp::get())
            .and(warp::cookie("token"))
            .and_then(Self::eve_login_alt);
        let eve_whoami = eve
            .clone()
            .and(warp::path!("whoami"))
            .and(warp::get())
            .and(warp::cookie("token"))
            .and_then(Self::eve_whoami);
        let eve = eve_auth
            .or(eve_login)
            .or(eve_login_alt)
            .or(eve_whoami);

        let item = root
            .clone()
            .and(warp::path!("items" / ..))
            .and(warp::get());
        let item_all = item
            .clone()
            .and(warp::path::end())
            .and(warp::get())
            .and_then(Self::item_all);
        let item_keys = item
            .clone()
            .and(warp::path!("keys"))
            .and(warp::get())
            .and_then(Self::item_keys);
        let item_dogma_skill = item
            .clone()
            .and(warp::path!(TypeId / "dogma" / "skill"))
            .and(warp::get())
            .and_then(Self::item_dogma_skill);
        let item = item_all
            .or(item_keys)
            .or(item_dogma_skill);

        let industry = root
            .clone()
            .and(warp::path!("industry" / ..));
        let industry_jobs = industry
            .clone()
            .and(warp::path!("jobs"))
            .and(warp::get())
            .and(warp::cookie("token"))
            .and_then(Self::industry_jobs);
        let industry_stations = industry
            .clone()
            .and(warp::path!("stations"))
            .and(warp::get())
            .and_then(Self::industry_stations);
        let industry = industry_jobs
            .or(industry_stations);

        let name = root
            .clone()
            .and(warp::path("name"));
        let name_resolve = name
            .clone()
            .and(warp::path!("resolve" / TypeId))
            .and(warp::get())
            .and_then(Self::name_resolve);
        let name_resolve_bulk = name
            .clone()
            .and(warp::path!("resolve" / "bulk"))
            .and(warp::post())
            .and(warp::body::json())
            .and_then(Self::name_resolve_bulk);
        let name_resolve_name_to_id_bulk = name
            .clone()
            .and(warp::path!("resolve" / "bulk" / "id"))
            .and(warp::post())
            .and(warp::body::json())
            .and_then(Self::name_resolve_name_to_id_bulk);
        let name = name_resolve
            .or(name_resolve_bulk)
            .or(name_resolve_name_to_id_bulk);

        let project = root
            .clone()
            .and(warp::path!("projects" / ..));
        let projects = project
            .clone()
            .and(warp::path::end())
            .and(warp::get())
            .and(warp::cookie("token"))
            .and_then(Self::projects);
        let project_id = project
            .clone()
            .and(warp::path!(Uuid))
            .and(warp::get())
            .and(warp::cookie("token"))
            .and_then(Self::project_id);
        let project_delete = project
            .clone()
            .and(warp::path!(Uuid))
            .and(warp::delete())
            .and(warp::cookie("token"))
            .and_then(Self::project_delete);
        let project_new = project
            .clone()
            .and(warp::path::end())
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::cookie("token"))
            .and_then(Self::project_new);
        let project_cost = project
            .clone()
            .and(warp::path!(Uuid / "cost"))
            .and(warp::get())
            .and(warp::cookie("token"))
            .and_then(Self::project_cost);
        let project_materials = project
            .clone()
            .and(warp::path!(Uuid / "materials"))
            .and(warp::get())
            .and(warp::cookie("token"))
            .and_then(Self::project_materials);
        let project_materials_raw = project
            .clone()
            .and(warp::path!(Uuid / "materials" / "raw"))
            .and(warp::get())
            .and(warp::cookie("token"))
            .and_then(Self::project_materials_raw);
        let project_materials_stored = project
            .clone()
            .and(warp::path!(Uuid / "materials" / "stored"))
            .and(warp::get())
            .and(warp::cookie("token"))
            .and_then(Self::project_materials_stored);
        let project_blueprints = project
            .clone()
            .and(warp::path!(Uuid / "blueprints"))
            .and(warp::get())
            .and(warp::cookie("token"))
            .and_then(Self::project_blueprints);
        let project_tree = project
            .clone()
            .and(warp::path!(Uuid / "tree"))
            .and(warp::get())
            .and(warp::cookie("token"))
            .and_then(Self::project_tree);
        let project_required_products = project
            .clone()
            .and(warp::path!(Uuid / "products"))
            .and(warp::get())
            .and(warp::cookie("token"))
            .and_then(Self::project_required_products);
        let project = projects
            .or(project_id)
            .or(project_delete)
            .or(project_new)
            .or(project_cost)
            .or(project_materials)
            .or(project_materials_raw)
            .or(project_materials_stored)
            .or(project_blueprints)
            .or(project_tree)
            .or(project_required_products);

        let api = alliance
            .or(blueprint)
            .or(character)
            .or(corporation)
            .or(eve)
            .or(industry)
            .or(item)
            .or(name)
            .or(project)
            .with(log);

        warp::serve(api)
            .run(([0, 0, 0, 0], 10101))
            .await;
    }

    async fn alliance_get_fittings(
        self:  Arc<Self>,
        token: Uuid,
    ) -> Result<impl Reply, Rejection> {
        self
            .alliance
            .get_fittings(token)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn alliance_get_fitting(
        self:  Arc<Self>,
        id:    Uuid,
        token: Uuid,
    ) -> Result<impl Reply, Rejection> {
        self
            .alliance
            .get_fitting(token, id)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn alliance_set_fitting(
        self:  Arc<Self>,
        entry: NewAllianceFitting,
        token: Uuid,
    ) -> Result<impl Reply, Rejection> {
        self
            .alliance
            .set_fitting(token, entry)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn alliance_del_fitting(
        self:  Arc<Self>,
        id:    Uuid,
        token: Uuid,
    ) -> Result<impl Reply, Rejection> {
        self
            .alliance
            .del_fitting(token, id)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn blueprint_all(
        self: Arc<Self>,
    ) -> Result<impl Reply, Rejection> {
        self
            .blueprint
            .all()
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn blueprint_get(
        self: Arc<Self>,
        bid:  TypeId,
    ) -> Result<impl Reply, Rejection> {
        self
            .blueprint
            .by_id(bid)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn character_assets(
        self:  Arc<Self>,
        token: Uuid,
    ) -> Result<impl Reply, Rejection> {
        self
            .character
            .assets(token)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn character_blueprints(
        self:  Arc<Self>,
        token: Uuid,
    ) -> Result<impl Reply, Rejection> {
        self
            .character
            .blueprints(token)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn character_info(
        self:  Arc<Self>,
        token: Uuid,
    ) -> Result<impl Reply, Rejection> {
        self
            .character
            .info(token)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn character_item_location(
        self:  Arc<Self>,
        id:    u64,
        token: Uuid,
    ) -> Result<impl Reply, Rejection> {
        self
            .character
            .item_location(token, id)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn corporation_blueprints(
        self:  Arc<Self>,
        cid:   CorporationId,
        token: Uuid,
    ) -> Result<impl Reply, Rejection> {
        self
            .corporation
            .blueprints(cid, token)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn corporation_set_blueprints(
        self:  Arc<Self>,
        cid:   CorporationId,
        body:  Vec<CorporationBlueprintEntry>,
        token: Uuid,
    ) -> Result<impl Reply, Rejection> {
        self
            .corporation
            .set_blueprints(cid, body, token)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn corporation_delete_blueprints(
        self:  Arc<Self>,
        cid:   CorporationId,
        token: Uuid,
    ) -> Result<impl Reply, Rejection> {
        self
            .corporation
            .delete_blueprints(cid, token)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn eve_auth(
        self:  Arc<Self>,
        query: EveAuthQuery,
    ) -> Result<impl Reply, Rejection> {
        let token = self.eve_auth.auth(query.code, query.state).await?;

        if let Some(token) = token {
            let cookie = format!(
                "token={}; Path=/; Secure; HttpOnly; Max-Age={}",
                token, 31557800 // 10 years
            );

            Ok(Response::builder()
                .status(StatusCode::MOVED_PERMANENTLY)
                .header("location", "https://eve.caph.xyz")
                .header("Set-Cookie", cookie)
                .body("")
                .unwrap_or_default())
        } else {
            Ok(Response::builder()
                .status(StatusCode::MOVED_PERMANENTLY)
                .header("location", "https://eve.caph.xyz")
                .body("")
                .unwrap_or_default())
        }
    }

    async fn eve_login(
        self: Arc<Self>,
    ) -> Result<impl Reply, Rejection> {
        let uri = self.eve_auth.login().await?;
        let uri = warp::http::uri::Builder::new()
            .scheme(uri.scheme())
            .authority(uri.host_str().unwrap_or_default())
            .path_and_query(&format!("{}?{}", uri.path(), uri.query().unwrap_or_default()))
            .build()
            .unwrap_or_default();
        Ok(warp::redirect::temporary(uri))
    }

    async fn eve_login_alt(
        self:  Arc<Self>,
        token: Uuid,
    ) -> Result<impl Reply, Rejection> {
        let uri = self.eve_auth.login_alt(&token).await?;
        let uri = warp::http::uri::Builder::new()
            .scheme(uri.scheme())
            .authority(uri.host_str().unwrap_or_default())
            .path_and_query(&format!("{}?{}", uri.path(), uri.query().unwrap_or_default()))
            .build()
            .unwrap_or_default();
        Ok(warp::redirect::temporary(uri))
    }

    async fn eve_whoami(
        self:  Arc<Self>,
        token: Uuid,
    ) -> Result<impl Reply, Rejection> {
        self
            .character
            .whoami(token)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn name_resolve(
        self:    Arc<Self>,
        item_id: TypeId,
    ) -> Result<impl Reply, Rejection> {
        self
            .name
            .resolve_id(item_id)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn name_resolve_bulk(
        self: Arc<Self>,
        ids:  Vec<TypeId>
    ) -> Result<impl Reply, Rejection> {
        self
            .name
            .resolve_bulk(ids)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn name_resolve_name_to_id_bulk(
        self:  Arc<Self>,
        names: Vec<String>
    ) -> Result<impl Reply, Rejection> {
        self
            .name
            .resolve_names_to_id_bulk(names)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn projects(
        self:  Arc<Self>,
        token: Uuid,
    ) -> Result<impl Reply, Rejection> {
        self
            .project
            .all(token)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn project_id(
        self:  Arc<Self>,
        id:    Uuid,
        token: Uuid,
    ) -> Result<impl Reply, Rejection> {
        self
            .project
            .by_id(id, token)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn project_delete(
        self:  Arc<Self>,
        id:    Uuid,
        token: Uuid,
    ) -> Result<impl Reply, Rejection> {
        self
            .project
            .delete(id, token)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn project_new(
        self:  Arc<Self>,
        body:  ProjectNew,
        token: Uuid,
    ) -> Result<impl Reply, Rejection> {
        self
            .project
            .create(body, token)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn project_cost(
        self:  Arc<Self>,
        id:    Uuid,
        token: Uuid,
    ) -> Result<impl Reply, Rejection> {
        self
            .project
            .cost(id, token)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn project_materials(
        self:  Arc<Self>,
        id:    Uuid,
        token: Uuid,
    ) -> Result<impl Reply, Rejection> {
        self
            .project
            .materials(id, token)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn project_materials_raw(
        self:  Arc<Self>,
        id:    Uuid,
        token: Uuid,
    ) -> Result<impl Reply, Rejection> {
        self
            .project
            .raw_materials(id, token)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn project_materials_stored(
        self:  Arc<Self>,
        id:    Uuid,
        token: Uuid,
    ) -> Result<impl Reply, Rejection> {
        self
            .project
            .stored_materials(id, token)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn project_blueprints(
        self:  Arc<Self>,
        id:    Uuid,
        token: Uuid,
    ) -> Result<impl Reply, Rejection> {
        self
            .project
            .blueprints(id, token)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn project_tree(
        self:  Arc<Self>,
        id:    Uuid,
        token: Uuid,
    ) -> Result<impl Reply, Rejection> {
        self
            .project
            .trees(id, token)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn project_required_products(
        self:  Arc<Self>,
        id:    Uuid,
        token: Uuid,
    ) -> Result<impl Reply, Rejection> {
        self
            .project
            .manufacture(id, token)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn industry_jobs(
        self:  Arc<Self>,
        token: Uuid,
    ) -> Result<impl Reply, Rejection> {
        self
            .industry
            .jobs(token)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn industry_stations(
        self: Arc<Self>,
    ) -> Result<impl Reply, Rejection> {
        let stations = self
            .industry
            .stations()?
            .iter()
            .map(|x| x.id)
            .collect::<Vec<_>>();
        Ok(warp::reply::json(&stations))
    }

    async fn item_all(
        self: Arc<Self>
    ) -> Result<impl Reply, Rejection> {
        self
            .item
            .all()
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn item_keys(
        self: Arc<Self>
    ) -> Result<impl Reply, Rejection> {
        self
            .item
            .keys()
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn item_dogma_skill(
        self:    Arc<Self>,
        type_id: TypeId,
    ) -> Result<impl Reply, Rejection> {
        self
            .item
            .dogma_skill(type_id)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }
}

#[derive(Debug, Deserialize)]
struct EveAuthQuery {
    code:  String,
    state: Uuid,
}

#[derive(Debug, Serialize)]
struct RequiredProducts {
    pub pid:       TypeId,
    pub bpid:      TypeId,
    pub quantity:  u32,
    pub stored:    u32,
    pub materials: Vec<RequiredProductsMaterial>,
    pub depth:     u8,
}

#[derive(Debug, Serialize)]
struct RequiredProductsMaterial {
    pub mid:      TypeId,
    pub quantity: u32,
    pub stored:   u32,
}
