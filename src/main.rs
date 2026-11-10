use avian2d::prelude::*;
use bevy::prelude::*;
use frogger::PlugMeIn;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PlugMeIn)
        .run();
}
