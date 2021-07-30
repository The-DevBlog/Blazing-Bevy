use crate::SCALE;
use bevy::prelude::*;
use bevy_rapier2d::{
    physics::{ColliderBundle, ColliderPositionSync},
    prelude::*,
    render::ColliderDebugRender,
};
use std::collections::HashMap;
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(background.system())
            .add_startup_system(boundaries.system())
            .add_startup_system(map_matrix.system());
    }
}

fn map_matrix(
    mut commands: Commands,
    windows: Res<Windows>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = windows.get_primary().unwrap();

    let map = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];

    let mut xy_conversion = HashMap::new();
    xy_conversion.insert(1, -6);
    xy_conversion.insert(2, -5);
    xy_conversion.insert(3, -4);
    xy_conversion.insert(4, -3);
    xy_conversion.insert(5, -2);
    xy_conversion.insert(6, -1);
    xy_conversion.insert(7, 1);
    xy_conversion.insert(8, 2);
    xy_conversion.insert(9, 3);
    xy_conversion.insert(10, 4);
    xy_conversion.insert(11, 5);
    xy_conversion.insert(12, 6);

    let unit_width = window.width() / SCALE / 12.0; // 10.666667
    let unit_height = window.height() / SCALE / 12.0; // 12

    for (i, row) in map.iter().enumerate() {
        // collider width in units as defined above
        let mut units = 0;
        let mut starting_idx = 0;
        for (j, col) in row.iter().enumerate() {
            if col != &0 {
                units += 1;
                if starting_idx == 0 {
                    starting_idx = j + 1;
                }
            }

            if &units > &0 && col == &0 || &units > &0 && j == map[i].len() - 1 {
                let collider_width = units as f32 * unit_width;
                let collider_height = unit_height;

                let collider_x =
                    xy_conversion.get(&starting_idx).unwrap().clone() as f32 * unit_width;

                let collider_y =
                    (xy_conversion.get(&(i + 1)).unwrap().clone() * -1) as f32 * unit_height;

                let sprite = SpriteBundle {
                    material: materials.add(Color::rgb(0.08, 0.58, 0.0).into()),
                    // the sprite vector is directly proportionate to the collider size.
                    // eg 1: a new vec of 'x: 1.0, y: 1.0' is the same exact size as the collider
                    // eg 2: a vec of 'x: 2.0, y: 2.0' is twice as large as the collider
                    sprite: Sprite::new(Vec2::new(1.0, 1.0)),
                    ..Default::default()
                };

                starting_idx = col.clone();
                units = 0;

                commands
                    .spawn()
                    .insert_bundle(sprite)
                    .insert_bundle(ColliderBundle {
                        position: [collider_x, collider_y].into(),
                        shape: ColliderShape::cuboid(collider_width, collider_height),
                        material: ColliderMaterial {
                            friction: 0.0,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(ColliderDebugRender::default())
                    .insert(ColliderPositionSync::Discrete);
            }
        }
    }
}

fn boundaries(mut commands: Commands, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();

    // a closure to use for the left, right, bottom and top colliders of the map
    let collider = |x: f32, y: f32, w: f32, h: f32| -> ColliderBundle {
        ColliderBundle {
            position: [x, y].into(),
            shape: ColliderShape::cuboid(w, h),
            material: ColliderMaterial {
                friction: 0.0,
                ..Default::default()
            },
            ..Default::default()
        }
    };

    // width and height
    let width = 1.0;
    let height = window.height();

    let x = window.width() / 2.0 / SCALE + width;
    let y = 0.0;

    // right
    commands.spawn_bundle(collider(x, y, width, height));

    // left
    commands.spawn_bundle(collider(-x, y, width, height));

    // top
    let width = window.width() / 2.0;
    let height = 1.0;
    let x = 0.0;
    let y = window.height() / 2.0 / SCALE + height;

    commands.spawn_bundle(collider(x, y, width, height));
}

fn background(
    mut commands: Commands,
    windows: Res<Windows>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = windows.get_primary().unwrap();

    let width = window.width();
    let height = window.height();

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(0.4, 0.8, 1.0).into()),
        sprite: Sprite::new(Vec2::new(width, height)),
        ..Default::default()
    });
}
