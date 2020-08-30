use crate::eve_client::*;
use crate::fetch;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Graphic {
    pub graphic_file: String,
    pub graphic_id: GraphicId,
}

impl EveClient {
    fetch!(fetch_graphic, "universe/graphics", GraphicId, Graphic);
}
