use geoprox_core::registry::PositionRegistry;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct AppState{
    pub position_registry: PositionRegistry,
    pub precision:usize,
}

pub type SharedState = Arc<RwLock<AppState>>;

impl Default for AppState{
    fn default() -> Self {
        AppState{
            position_registry: PositionRegistry::default(),
            precision: 10
        }
    }
}