use crate::movement::{Controller, MovementBundle};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Debug, Component)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Player"),
        Player,
        MovementBundle::new(5., 5.),
        Collider::ball(0.2),
        RigidBody::Dynamic,
        Friction::coefficient(0.7),
        Restitution::coefficient(1.),
        Velocity::zero(),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.2,
                ..Default::default()
            })),
            material: materials.add(Color::rgb(0.01, 0.7, 0.2).into()),
            transform: Transform::from_xyz(-4., 0.2, 3.),
            ..default()
        },
    ));
}
pub fn receive_input(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Controller, &Transform), With<Player>>,
) {
    for (mut controller, player_transform) in &mut query {
        controller.right_pressed = input.pressed(KeyCode::D);
        controller.left_pressed = input.pressed(KeyCode::A);
        controller.up_pressed = input.pressed(KeyCode::W);
        controller.down_pressed = input.pressed(KeyCode::S);
        controller.jump_just_pressed =
            input.just_pressed(KeyCode::Space) && player_transform.translation.y < 2.4;
    }
}
