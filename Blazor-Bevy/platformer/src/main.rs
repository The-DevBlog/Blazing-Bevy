use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_rapier2d::{physics::*, prelude::*};
mod map;
mod player;

const SCALE: f32 = 10.0;

// components
struct Player;
struct Speed(f32);

fn main() {
    let mut app = App::build();
    // app.insert_resource(Msaa { samples: 4 })
    app.add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default()) // required for 'RapierConfiguration'
        .add_plugin(RapierRenderPlugin) // required to render items
        .add_plugin(map::MapPlugin)
        .add_plugin(player::PlayerPlugin);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands, mut config: ResMut<RapierConfiguration>) {
    config.scale = SCALE;
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(1000.0, 10.0, 2000.0)),
        ..Default::default()
    });

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
