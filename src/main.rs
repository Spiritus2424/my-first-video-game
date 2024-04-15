mod hello;
mod people;

use bevy::prelude::*;
use hello::plugin::HelloPlugin;

fn main() {
    App::new().add_plugins((DefaultPlugins, HelloPlugin)).run();
}

// fn hello_world() {
//     println!("hello world!");
// }

// #[derive(Component)]
// struct Name(String);
