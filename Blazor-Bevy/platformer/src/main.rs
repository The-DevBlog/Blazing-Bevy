use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use bevy::window::WindowMode;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::{physics::*, prelude::*};
mod map;
mod player;

const SCALE: f32 = 10.0;

// components
struct Player;
struct Speed(f32);

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            mode: WindowMode::Fullscreen { use_size: false },
            resizable: true,
            vsync: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.47, 0.78, 1.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_webgl2::WebGL2Plugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default()) // required for 'RapierConfiguration'
        .add_plugin(RapierRenderPlugin) // required to render items
        .add_plugin(map::MapPlugin)
        .add_plugin(player::PlayerPlugin)
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
    // .insert(MainCamera);
}

// Below source used from -> https://bevy-cheatbook.github.io/cookbook/cursor2world.html
// struct MainCamera;

// gets the x y coords of the mouse at every frame
// fn my_cursor_system(
//     // need to get window dimensions
//     wnds: Res<Windows>,
//     // query to get camera transform
//     q_camera: Query<&Transform, With<MainCamera>>,
// ) {
//     // get the primary window
//     let wnd = wnds.get_primary().unwrap();

//     // check if the cursor is in the primary window
//     if let Some(pos) = wnd.cursor_position() {
//         // get the size of the window
//         let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

//         // the default orthographic projection is in pixels from the center;
//         // just undo the translation
//         let p = pos - size / 2.0;

//         // assuming there is exactly one main camera entity, so this is OK
//         let camera_transform = q_camera.single().unwrap();

//         // apply the camera transform
//         let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
//         eprintln!("World coords: {}/{}", pos_wld.x, pos_wld.y);
//     }
// }
