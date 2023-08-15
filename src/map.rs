use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Debug, Resource)]
pub struct MapGrid(pub Vec<Vec<u32>>);

pub fn spawn_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    map: Res<MapGrid>,
) {
    let size = map.0.len() as f32;

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(size).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(6., 0.0001, 6.),
    ));

    for (row_idx, row) in map.0.iter().enumerate() {
        for (tile_idx, tile) in row.iter().enumerate() {
            match tile {
                1 => {
                    commands.spawn((
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
                _ => {}
            }
        }
    }
}
