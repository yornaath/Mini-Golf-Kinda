use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::powerup::PowerupType;

#[derive(Debug, Resource)]
pub struct MapGrid(pub Vec<Vec<Tile>>);

#[derive(Debug, Component)]
pub enum Tile {
    Wall,
    Empty,
    Goal,
    Hole,
    Powerup(PowerupType),
}

#[derive(Debug, Component)]
pub struct Floor;

pub fn spawn_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    map: Res<MapGrid>,
) {
    let size = map.0.len() as f32;

    // Spawn the ground based on the map size.
    commands.spawn((
        Floor,
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(size).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(6., 0.0, 6.),
    ));

    // Iterate over the rows of tiles in the map grid
    // and spawn the tiles based on the tile type.

    for (row_idx, row) in map.0.iter().enumerate() {
        for (tile_idx, tile) in row.iter().enumerate() {
            match tile {
                // Spawn the walls for every tile that is Wall.
                // Insert rigid bodies and colliders for every wall tile.
                Tile::Wall => {
                    commands.spawn((
                        Tile::Wall,
                        PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube { size: 1. })),
                            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                            transform: Transform::from_xyz(
                                ((0.5 - (size / 2.)) + tile_idx as f32) as f32,
                                0.5,
                                ((0.5 - (size / 2.)) + row_idx as f32) as f32,
                            ),
                            ..default()
                        },
                        RigidBody::Fixed,
                        Collider::cuboid(0.5, 0.5, 0.5),
                    ));
                }

                // Spawn the holes for every tile that is Hole.
                // Add a sensor component to the holes so we can detect when the ball enters them.
                Tile::Hole => {
                    commands.spawn((
                        Tile::Hole,
                        PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Circle {
                                radius: 0.2,
                                vertices: 20,
                            })),
                            material: materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
                            transform: Transform::from_xyz(
                                ((0.5 - (size / 2.)) + tile_idx as f32) as f32,
                                0.01,
                                ((0.5 - (size / 2.)) + row_idx as f32) as f32,
                            )
                            .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
                            ..default()
                        },
                        Collider::ball(0.2),
                        Sensor,
                    ));
                }

                // Spawn the goal for every tile that is Goal.
                // Add a sensor component to the goal so we can detect when the ball enters it.
                Tile::Goal => {
                    commands.spawn((
                        Tile::Goal,
                        PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Circle {
                                radius: 0.2,
                                vertices: 20,
                            })),
                            material: materials.add(Color::rgb(0.9, 0.8, 0.1).into()),
                            transform: Transform::from_xyz(
                                ((0.5 - (size / 2.)) + tile_idx as f32) as f32,
                                0.02,
                                ((0.5 - (size / 2.)) + row_idx as f32) as f32,
                            )
                            .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
                            ..default()
                        },
                        Collider::ball(0.2),
                        Sensor,
                    ));
                }

                Tile::Powerup(PowerupType) => {
                    let color = match PowerupType {
                        PowerupType::Boost => Color::ORANGE,
                        PowerupType::Antigravity => Color::rgb(0.9, 0.02, 0.9),
                    };
                    commands.spawn((
                        PowerupType.clone(),
                        PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::UVSphere {
                                radius: 0.2,
                                ..default()
                            })),
                            material: materials.add(color.into()),
                            transform: Transform::from_xyz(
                                ((0.5 - (size / 2.)) + tile_idx as f32) as f32,
                                0.02,
                                ((0.5 - (size / 2.)) + row_idx as f32) as f32,
                            )
                            .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
                            ..default()
                        },
                        Collider::ball(0.2),
                        Sensor,
                    ));
                }

                // Spawn nothing for every tile that is Empty.
                Tile::Empty => {}
            }
        }
    }
}
