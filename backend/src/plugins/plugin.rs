pub enum Event {
    Packet(Vec<u8>),
    Tick,
}

pub trait Plugin: Send + Sync {
    fn name(&self) -> &'static str;

    fn on_event(&self, event: &Event, ctx: &PluginContext);
}