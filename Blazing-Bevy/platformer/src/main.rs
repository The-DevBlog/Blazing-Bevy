use bevy::prelude::*;
use bevy_rapier2d::{physics::*, prelude::*};
mod map;
mod player;

const SCALE: f32 = 10.0;

// components
struct Player;
struct Speed(f32);

fn main() {
    let mut app = App::build();

    app.insert_resource(WindowDescriptor {
        width: 200.0,
        height: 100.0,
        resizable: true,
        vsync: false,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
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
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let text_style = TextStyle {
        font_size: 60.0,
        color: Color::BLACK,
        ..Default::default()
    };

    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Top,
        horizontal: HorizontalAlign::Left,
    };

    let text_bundle = Text2dBundle {
        text: Text::with_section("HELLO WORLD", text_style, text_alignment),
        ..Default::default()
    };

    commands.spawn_bundle(text_bundle);
}
