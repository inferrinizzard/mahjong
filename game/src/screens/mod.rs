pub mod debug_counter;
pub mod debug_tile;
pub mod main_screen;

#[derive(Debug, Clone)]
pub enum Screen {
    Main,
    DebugCounter,
    DebugTile,
    Settings,
}
