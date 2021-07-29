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
        app.add_startup_system(boundaries.system())
            .add_startup_system(map_matrix.system());
    }
}

fn map_matrix(
    mut commands: Commands,
    windows: Res<Windows>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    /*
    2560 / 10 (SCALE) / 12 = 21.333333
    1440 / 10 (SCALE) / 12 = 12

    1 unit width = 10.665
    1 unit length = 6
    */

    let window = windows.get_primary().unwrap();

    let map = [
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ],
    ];

    let mut x_conversion = HashMap::new();
    x_conversion.insert(1, -12);
    x_conversion.insert(2, -11);
    x_conversion.insert(3, -10);
    x_conversion.insert(4, -9);
    x_conversion.insert(5, -8);
    x_conversion.insert(6, -7);
    x_conversion.insert(7, -6);
    x_conversion.insert(8, -5);
    x_conversion.insert(9, -4);
    x_conversion.insert(10, -3);
    x_conversion.insert(11, -2);
    x_conversion.insert(12, -1);
    x_conversion.insert(13, 1);
    x_conversion.insert(14, 2);
    x_conversion.insert(15, 3);
    x_conversion.insert(16, 4);
    x_conversion.insert(17, 5);
    x_conversion.insert(18, 6);
    x_conversion.insert(19, 7);
    x_conversion.insert(20, 8);
    x_conversion.insert(21, 9);
    x_conversion.insert(22, 10);
    x_conversion.insert(23, 11);
    x_conversion.insert(24, 12);

    let unit_width = window.width() / SCALE / 12.0 / 2.0;
    let unit_height = window.height() / SCALE / 12.0 / 2.0;

    // a closure
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

    let height = 0;
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
                    x_conversion.get(&starting_idx).unwrap().clone() as f32 * (unit_width * 2.0);
                let collider_y = -((map.len() - (map.len() - i)) as f32 * unit_height);

                let sprite = SpriteBundle {
                    material: materials.add(Color::rgb(0.08, 0.58, 0.0).into()),
                    // the sprite vector is directly proportionate to the collider size.
                    // eg 1: a new vec of 'x: 1.0, y: 1.0' is the same exact size as the collider
                    // eg 2: a vec of 'x: 2.0, y: 2.0' is twice as large as the collider
                    sprite: Sprite::new(Vec2::new(1.0, 1.0)),
                    ..Default::default()
                };
                println!(
                    "Units: {}, Unit width: {}, Unit height: {}, Starting idx: {} j: {}, collider width: {}, collider x: {}",
                    units, unit_width, unit_height, starting_idx, j, collider_width, collider_x
                );
                starting_idx = col.clone();
                units = 0;

                commands
                    .spawn()
                    .insert_bundle(sprite)
                    .insert_bundle(collider(
                        collider_x,
                        collider_y,
                        collider_width,
                        collider_height,
                    ))
                    .insert(ColliderDebugRender::default())
                    .insert(ColliderPositionSync::Discrete);
            }

            // if col == &1 {
            // } else if col == &2 {
            // }
        }
    }
}

fn boundaries(
    mut commands: Commands,
    windows: Res<Windows>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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

    // bottom
    // let height = 5.0;

    // let sprite = SpriteBundle {
    //     material: materials.add(Color::rgb(0.08, 0.58, 0.0).into()),
    //     // the sprite vector is directly proportionate to the collider size.
    //     // eg 1: a new vec of 'x: 1.0, y: 1.0' is the same exact size as the collider
    //     // eg 2: a vec of 'x: 2.0, y: 2.0' is twice as large as the collider
    //     sprite: Sprite::new(Vec2::new(1.0, 1.0)),
    //     ..Default::default()
    // };

    // commands
    //     .spawn()
    //     .insert_bundle(sprite)
    //     .insert_bundle(collider(x, -y, width, height))
    //     .insert(ColliderDebugRender::default())
    //     // syncs the collider position with the sprite position
    //     .insert(ColliderPositionSync::Discrete);
}
