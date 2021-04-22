use crate::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NpcCorporationEntry {
    #[serde(rename = "corporationTrades")]
    #[serde(default)]
    pub corporation_trades:            HashMap<TypeId, f32>,
    #[serde(rename = "deleted")]
    pub deleted:                       bool,
    #[serde(rename = "descriptionID")]
    #[serde(default)]
    pub description:                   HashMap<String, String>,
    #[serde(rename = "divisions")]
    #[serde(default)]
    pub divisions:                     HashMap<DivisionId, NpcDivision>,
    #[serde(rename = "extent")]
    pub extend:                        String,
    #[serde(rename = "hasPlayerPersonnelManager")]
    pub has_player_personnel_manager:  bool,
    #[serde(rename = "initialPrice")]
    pub initial_price:                 u32,
    #[serde(rename = "memberLimit")]
    pub member_limit:                  i16,
    #[serde(rename = "minSecurity")]
    pub min_security:                  f32,
    #[serde(rename = "minimumJoinStanding")]
    pub min_join_standing:             u16,
    #[serde(rename = "nameID")]
    pub name:                          HashMap<String, String>,
    #[serde(rename = "publicShares")]
    pub public_shares:                 u32,
    #[serde(rename = "sendCharTerminationMessage")]
    pub send_char_termination_message: bool,
    #[serde(rename = "shares")]
    pub shares:                        u64,
    #[serde(rename = "size")]
    pub size:                          String,
    #[serde(rename = "taxRate")]
    pub tax_rate:                      f32,
    #[serde(rename = "tickerName")]
    pub ticker_name:                   String,
    #[serde(rename = "uniqueName")]
    pub unique_name:                   bool,
    #[serde(rename = "url")]
    #[serde(default)]
    pub url:                           String,
    #[serde(rename = "exchangeRates")]
    #[serde(default)]
    pub exchange_rates:                HashMap<CorporationId, f32>,

    #[serde(rename = "ceoID")]
    pub ceo_id:                        Option<PlayerId>,
    #[serde(rename = "allowedMemberRaces")]
    pub allowed_member_races:          Option<Vec<RaceId>>,
    #[serde(rename = "enemyID")]
    pub enemy_id:                      Option<CorporationId>,
    #[serde(rename = "friendID")]
    pub friend_id:                     Option<CorporationId>,
    #[serde(rename = "factionID")]
    pub faction_id:                    Option<FactionId>,
    #[serde(rename = "iconID")]
    pub icon_id:                       Option<IconId>,
    #[serde(rename = "investors")]
    pub investors:                     Option<HashMap<PlayerId, u32>>,
    #[serde(rename = "lpOfferTables")]
    pub lpoffer_tables:                Option<Vec<u16>>,
    #[serde(rename = "mainActivityID")]
    pub main_activity_id:              Option<ActivityId>,
    #[serde(rename = "secondaryActivityID")]
    pub secondary_activity_id:         Option<ActivityId>,
    #[serde(rename = "raceID")]
    pub race_id:                       Option<RaceId>,
    #[serde(rename = "sizeFactor")]
    pub size_factor:                   Option<f32>,
    #[serde(rename = "stationID")]
    pub station_id:                    Option<StationId>,
    #[serde(rename = "solarSystemID")]
    pub solar_system_id:               Option<SolarSystemId>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NpcDivision {
    #[serde(rename = "divisionNumber")]
    pub division_number: u8,
    #[serde(rename = "leaderID")]
    pub leader_id:       PlayerId,
    #[serde(rename = "size")]
    pub size:            u32,
}
