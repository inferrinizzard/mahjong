pub mod debug_counter;
pub mod debug_tile;
pub mod main_screen;
pub use debug_counter::Counter;
pub use debug_tile::DebugTile;

#[derive(Debug, Clone)]
pub enum Screen {
    Main,
    Game,
    DebugCounter,
    DebugTile,
    Settings,
}
