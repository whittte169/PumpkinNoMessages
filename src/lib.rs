use pumpkin_plugin_api::{
    Context, Plugin, PluginMetadata, Result, Server, 
    events::{EventData, EventHandler, EventPriority, PlayerJoinEvent, PlayerDeathEvent, PlayerLeaveEvent}, 
    register_plugin, text::TextComponent
};
use tracing::info;

struct Handler;
impl EventHandler<PlayerJoinEvent> for Handler {
    fn handle<'a>(&'a self, _server: Server, mut event: EventData<PlayerJoinEvent>) -> EventData<PlayerJoinEvent> {
        event.join_message = TextComponent::text("");
        event
    }
}

impl EventHandler<PlayerLeaveEvent> for Handler {
    fn handle<'a>(&'a self, _server: Server, mut event: EventData<PlayerLeaveEvent>) -> EventData<PlayerLeaveEvent> {
        event.leave_message = TextComponent::text("");
        event
    }
}

impl EventHandler<PlayerDeathEvent> for Handler {
    fn handle<'a>(&'a self, _server: Server, mut event: EventData<PlayerDeathEvent>) -> EventData<PlayerDeathEvent> {
        event.death_message = TextComponent::text("");
        event
    }
}

struct Main;
impl Plugin for Main {
    fn new() -> Self {
        Main
    }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "PumpkinNoMessages".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            authors: vec!["whittte169".into()],
            description: "A simple plugin that removes player join, leave and death messages in Pumpkin MC.".into(),
            dependencies: vec![],
            permissions: vec![]
        }
    }

    fn on_load(&self, context: Context) -> Result<()> {
        info!("PumpkinNoMessages was successfuly loaded!");
        context.register_event_handler::<PlayerJoinEvent, _>(Handler, EventPriority::Normal, true)?;
        context.register_event_handler::<PlayerLeaveEvent, _>(Handler, EventPriority::Normal, true)?;
        context.register_event_handler::<PlayerDeathEvent, _>(Handler, EventPriority::Normal, true)?;
        Ok(())
    }

    fn on_unload(&self, _context: Context) -> Result<()> {
        info!("PumpkinNoMessages was successfuly disabled!");
        Ok(())
    }
}

register_plugin!(Main);