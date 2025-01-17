extern crate tokio;
extern crate async_trait;

use std::fmt::Debug;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use async_trait::async_trait;
use horizon_data_types::{ Player, PlayerManager };

// Basic types
pub type PlayerId = u64;
pub type ItemId = u32;
pub type Position = (f32, f32, f32);
pub mod components;
pub use components::{Plugin, PluginCreateFn, PluginMetadata};

/// Represents the version of the plugin API.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApiVersion {
    pub major: u32,
    pub minor: u32,
    pub hotfix: u32,
}

impl ApiVersion {
    pub const fn new(major: u32, minor: u32, hotfix: u32) -> Self {
        Self { major, minor, hotfix }
    }
}

/// The current version of the plugin API.
/// Plugins must specify this version in their metadata to ensure compatibility.
pub const PLUGIN_API_VERSION: ApiVersion = ApiVersion::new(0, 0, 0);

pub trait AsAny {
    fn as_any(&self) -> & dyn Any;
}

impl<T: Plugin + 'static> AsAny for T {
    fn as_any(&self) -> & dyn Any {
        self
    }
}

// Event types
pub enum GameEvent {
    None,
    PlayerJoined( Player ),
    PlayerLeft( Player ),
    ChatMessage { sender: Player, content: String },
    ItemPickup { player: Player, item: ItemId },
    PlayerMoved { player: Player, new_position: Position },
    DamageDealt { attacker: Player, target: Player, amount: f32 },
    // Add more event types as needed
}

pub trait SayHello {
    fn say_hello(&self) -> String;
}

pub trait PluginInformation {
    fn name(&self) -> String;
    fn get_instance(&self) -> Box<dyn SayHello>;
}

// Configuration trait for plugins
pub trait PluginConfig: Send + Sync {
    fn load(&mut self, config: &str) -> Result<(), String>;
    fn save(&self) -> Result<String, String>;
}

// Logging trait for plugins
pub trait PluginLogger: Send + Sync {
    fn log(&self, level: LogLevel, message: &str);
}

// Log levels
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

// Main plugin trait
#[async_trait]
pub trait BaseAPI: Send + Sync {
    // Define Async methods
    async fn on_game_event(&self, event: &GameEvent);

    async fn on_game_tick(&self, delta_time: f64);

    // Define optional methods with default implementations
    fn get_config(&self) -> Option<&dyn PluginConfig> { None }
    fn get_logger(&self) -> Option<&dyn PluginLogger> { None }

    // Method for dumbnamic casting
    fn as_any(&self) -> &dyn Any;
}

// Command handler trait
#[async_trait]
pub trait CommandHandler: Send + Sync {
    async fn handle_command(&self, sender: Player, command: &str, args: Vec<String>, context: &mut PluginContext) -> bool;
}

// Context provided to plugins for interacting with the game server
pub struct PluginContext {
    pub server: Arc<GameServer>,
    pub shared_data: Arc<RwLock<HashMap<String, Box<dyn Any + Send + Sync>>>>,
    pub config: Arc<RwLock<HashMap<String, String>>>,
}

// Game server struct (placeholder for actual implementation)
pub struct GameServer {
    // Add relevant game server fields here
}

impl GameServer {
    pub async fn broadcast_message(&self, _message: &str) {
        // Implementation for broadcasting a message to all players
    }

    pub async fn get_player(&self, _id: Player) -> Option<Player> {
        // Implementation for retrieving a player by ID
        None
    }

    pub async fn spawn_item(&self, _item: ItemId, _position: Position) {
        // Implementation for spawning an item in the game world
    }

    pub async fn apply_damage(&self, _target: Player, _amount: f32) {
        // Implementation for applying damage to a player
    }
}

// Player struct (placeholder for actual implementation)
pub struct PlayerDetails {
    pub player: Player,
    pub name: String,
    pub position: Position,
    pub health: f32,
    // Add more player fields as needed
}
