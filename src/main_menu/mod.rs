mod components;
mod styles;
mod systems;

use bevy::app::{Plugin, Startup};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, main_menu);
    }
}

pub fn main_menu() {
    println!("Your are on the Main menu!")
}
