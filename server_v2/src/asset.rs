use crate::eve::EveAuthService;

use cachem::v2::ConnectionPool;

#[derive(Clone)]
pub struct AssetService {
    pool:      ConnectionPool,
    eve_auth:  EveAuthService,
}

impl AssetService {
    pub fn new(
        pool:      ConnectionPool,
        eve_auth:  EveAuthService,
    ) -> Self {
        Self {
            pool,
            eve_auth,
        }
    }
}
