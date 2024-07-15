use geoprox_core::registry::PositionRegistry;
use std::sync::{Arc, RwLock};

pub type SharedState = Arc<RwLock<PositionRegistry>>;