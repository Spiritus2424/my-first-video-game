pub mod components;
mod systems;

use bevy::app::Plugin;
use bevy::prelude::*;

use systems::*;

use crate::AppState;

use super::SimulationState;

pub const PLAYER_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 500.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_player)
            .add_systems(
                Update,
                (
                    player_movement,
                    confine_player_movement.after(player_movement),
                    enemy_hit_player,
                    player_hit_star,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(OnExit(AppState::Game), despawn_player);
    }
}
