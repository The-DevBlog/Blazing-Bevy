use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::na::Vector2};

use crate::{Player, Speed, SCALE};
const PLAYER_SPEED: f32 = 3.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_player.system())
            .add_system(player_movement.system());
    }
}

fn spawn_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();

    let player_color = (0.69, 0.46, 0.05);
    let player_size = 4.0;
    let gravity = 50.0;

    // ensure player will start at left most position regardless of screen size
    let player_x = window.width() / 2.0 / SCALE - player_size - 5.0;

    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(player_size, player_size),
        position: [-player_x, -10.0].into(),
        material: ColliderMaterial {
            friction: 0.0,
            ..Default::default()
        },
        ..Default::default()
    };

    let rigid_body = RigidBodyBundle {
        body_type: RigidBodyType::Dynamic,
        damping: RigidBodyDamping {
            linear_damping: 5.0,
            ..Default::default()
        },
        forces: RigidBodyForces {
            gravity_scale: gravity,
            ..Default::default()
        },

        // prevent player from rotating
        mass_properties: (RigidBodyMassPropsFlags::ROTATION_LOCKED).into(),
        ..Default::default()
    };

    // handles cosmetics
    let sprite = SpriteBundle {
        material: materials.add(Color::rgb(player_color.0, player_color.1, player_color.2).into()),
        sprite: Sprite::new(Vec2::new(1.0, 1.0)),
        ..Default::default()
    };

    commands
        .spawn()
        .insert_bundle(sprite)
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        // syncs the collider position width the sprite position
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::default())
        .insert(Player)
        .insert(Speed(PLAYER_SPEED));
}

fn player_movement(
    keys: Res<Input<KeyCode>>,
    mut player_info: Query<(&Player, &mut RigidBodyVelocity)>,
) {
    if let Ok((_player, mut rb_velocity)) = player_info.single_mut() {
        let mut direction = 0.0;
        let mut jump = 0.0;

        // go left
        if (rb_velocity.linvel[1] != 0.0 && rb_velocity.linvel[0] < 0.0)
            || (rb_velocity.linvel[1] == 0.0)
        {
            // sprint left
            if keys.pressed(KeyCode::A) && keys.pressed(KeyCode::LShift) {
                direction = -1.5;
                // move left
            } else if keys.pressed(KeyCode::A) {
                direction = -1.0;
            }
        }

        // go right
        if (rb_velocity.linvel[1] != 0.0 && rb_velocity.linvel[0] > 0.0)
            || (rb_velocity.linvel[1] == 0.0)
        {
            // sprint right
            if keys.pressed(KeyCode::D) && keys.pressed(KeyCode::LShift) {
                direction = 1.5;
                // move right
            } else if keys.pressed(KeyCode::D) {
                direction = 1.0;
            }
        }

        // check if player is already jumping
        if rb_velocity.linvel[1] == 0.0 {
            // power jump
            if keys.pressed(KeyCode::LShift) && keys.just_pressed(KeyCode::Space) {
                jump = 65.0;
                // normal jump
            } else if keys.just_pressed(KeyCode::Space) {
                jump = 50.0;
            }
        }

        // eprintln!("{}", rb_velocity.linvel[0]);
        let move_delta = Vector2::new(direction, jump);
        rb_velocity.linvel += move_delta * PLAYER_SPEED;
    }
}
