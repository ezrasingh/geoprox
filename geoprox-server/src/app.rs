use geoprox_core::shard::GeoShard;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct AppState{
    pub geoshard: GeoShard
}

pub type SharedState = Arc<RwLock<AppState>>;

impl Default for AppState{
    fn default() -> Self {
        AppState{
            geoshard: GeoShard::default(),
        }
    }
}