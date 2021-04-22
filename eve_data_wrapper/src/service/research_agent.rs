use crate::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ResearchAgentService {
    research_agents: HashMap<AgentId, ResearchAgentEntry>
}

impl ResearchAgentService {
    const PATH: &'static str = "sde/fsd/researchAgents.yaml";

    pub fn new(mut zip: SdeZipArchive) -> Result<Self, EveConnectError> {
        Ok(Self {
            research_agents: crate::parse_zip_file(Self::PATH, &mut zip)?,
        })
    }
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
