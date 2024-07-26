use geoprox_core::shard::{GeoShard, GeoShardConfig};
use std::sync::{Arc, RwLock};

#[derive(Clone, Default)]
pub struct AppState {
    pub geoshard: GeoShard,
}

impl From<GeoShardConfig> for AppState {
    fn from(shard_config: GeoShardConfig) -> Self {
        AppState {
            geoshard: GeoShard::from(shard_config),
        }
    }
}

pub type SharedState = Arc<RwLock<AppState>>;

impl From<AppState> for SharedState {
    fn from(app: AppState) -> Self {
        Arc::new(app.into())
    }
}
