pub mod game;
pub mod main_menu;

use bevy::prelude::*;
use game::GamePlugin;
use main_menu::MainMenuPlugin;

fn main() {
    App::new()
        .init_state::<AppState>()
        .add_plugins((DefaultPlugins, MainMenuPlugin, GamePlugin))
        .run();
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
