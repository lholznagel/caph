use crate::{EveApiError, EveClient};

impl EveClient {
    /// Fetches all region ids
    pub async fn fetch_regions(
        &self,
    ) -> Result<Vec<u32>, EveApiError> {
        self.fetch_page("universe/regions").await
    }
}
