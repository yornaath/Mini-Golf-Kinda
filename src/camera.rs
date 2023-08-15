use bevy::{prelude::*, render::camera::ScalingMode};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        projection: OrthographicProjection {
            scale: 4.0,
            scaling_mode: ScalingMode::FixedVertical(2.0),
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(3.0, 8.0, 5.0),
        ..default()
    });
}
