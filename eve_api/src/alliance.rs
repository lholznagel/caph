use crate::eve_client::*;
use crate::fetch;
use crate::universe::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Alliance {
    pub creator_corporation_id: CorporationId,
    pub creator_id: u32,
    pub date_founded: String,
    pub name: String,
    pub ticker: String,

    pub executor_corporation_id: Option<CorporationId>,
    pub faction_id: Option<FactionId>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Icon {
    pub px128x128: String,
    pub px64x64: String,
}

impl EveClient {
    fetch!(fetch_alliance, "alliance", AllianceId, Alliance);

    fetch!(
        fetch_alliance_corporations,
        "alliance",
        "corporations",
        AllianceId,
        Vec<CorporationId>
    );
    fetch!(fetch_alliance_icons, "alliance", "icons", AllianceId, Icon);
}
