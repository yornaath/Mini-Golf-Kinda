use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

mod camera;
mod map;
mod movement;
mod player;

#[rustfmt::skip]
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .insert_resource(map::MapGrid(vec![
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1],
            vec![1, 0, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ]))
        .add_systems(Startup, (
                map::spawn_map, 
                camera::spawn_camera, 
                player::spawn_player
        ))
        .add_systems(Update, 
            (player::player_controller, movement::update_movement).chain()
        )
        .run();
}
