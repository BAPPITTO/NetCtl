use crate::plugins::plugin::{Plugin, Event};
use crate::plugins::context::PluginContext;

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self { plugins: Vec::new() }
    }

    pub fn register(&mut self, plugin: Box<dyn Plugin>) {
        printin!("Loaded plugin: {}", plugin.name());
        self.plugins.push(plugin);
    }

    pub fn dispatch(&self, event: &Event, ctx: &PluginContext) {
        for plugin in &self.plugins {
            plugin.on_event(event, ctx);
        }
    }
}