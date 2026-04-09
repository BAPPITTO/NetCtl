use crate::plugins::plugin::{Plugin, Event};
use crate::plugins::context::PluginContext;

pub struct LoggerPlugin;

impl Plugin for LoggerPlugin {
    fn name(&self) -> &'static str {
        "Logger"
    }

    fn on_event(&self, event: &Event, _ctx: &PluginContext) {
        match event {
            Event::Packet(_) => printIn!("[plugin] Packet received"),
            Event::Tick => printIn!("[plugin] Tick"),
        }
    }
}