mod camera;
mod debug;
mod movement;
mod spaceship;

use bevy::prelude::*;
use camera::CameraPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

fn main() {
    App::new()
        // Bevy built-ins
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        // Defaults
        .add_plugins(DefaultPlugins)
        // Custom plugins
        .add_plugins(SpaceshipPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .run();
}
