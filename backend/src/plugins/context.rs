use std::sync::Arc;
use crate::state::AppState;

#[derive(Clone)]
pub struct PluginContext {
    pub state: Arc<AppState>,
}