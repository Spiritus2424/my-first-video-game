pub mod enemy;
pub mod events;
pub mod player;
pub mod score;
pub mod star;
mod systems;

use bevy::app::Plugin;
use bevy::prelude::*;

use crate::AppState;
use enemy::EnemyPlugin;
use events::*;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_state::<SimulationState>()
            .add_event::<GameOverEvent>()
            .add_plugins((PlayerPlugin, EnemyPlugin, ScorePlugin, StarPlugin))
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, exit_game)
            .add_systems(Update, handle_game_over)
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}
