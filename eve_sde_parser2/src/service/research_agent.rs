use crate::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ResearchAgentService(pub HashMap<AgentId, ResearchAgentEntry>);

impl ResearchAgentService {
    const PATH: &'static str = "sde/fsd/researchAgents.yaml";

    service_gen!();
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResearchAgentEntry {
    #[serde(rename = "skills")]
    pub skill: Vec<AgentSkill>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AgentSkill {
    #[serde(rename = "typeID")]
    pub type_id: TypeId
}
