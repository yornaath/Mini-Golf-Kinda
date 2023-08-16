use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use map::Tile;
use state::State;

mod camera;
mod map;
mod movement;
mod player;
mod powerup;
mod state;
mod ui;

#[rustfmt::skip]
fn main() {
    App::new()

        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            //WorldInspectorPlugin::new(),
            //RapierDebugRenderPlugin::default(),
        ))

        .insert_resource(map::MapGrid(vec![
            vec![Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall],
            vec![Tile::Wall, Tile::Empty, Tile::Hole, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Hole, Tile::Goal, Tile::Empty, Tile::Wall],
            vec![Tile::Wall, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Wall],
            vec![Tile::Wall, Tile::Empty, Tile::Empty, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Wall],
            vec![Tile::Wall, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Wall, Tile::Hole, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Wall],
            vec![Tile::Wall, Tile::Empty, Tile::Empty, Tile::Hole, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Wall, Tile::Empty, Tile::Empty, Tile::Wall],
            vec![Tile::Wall, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Wall, Tile::Empty, Tile::Empty, Tile::Wall],
            vec![Tile::Wall, Tile::Empty, Tile::Wall, Tile::Powerup(powerup::PowerupType::Antigravity), Tile::Empty, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Empty, Tile::Empty, Tile::Wall],
            vec![Tile::Wall, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Powerup(powerup::PowerupType::Boost), Tile::Empty, Tile::Empty, Tile::Empty, Tile::Wall],
            vec![Tile::Wall, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Hole, Tile::Empty, Tile::Wall],
            vec![Tile::Wall, Tile::Empty, Tile::Wall, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Wall],
            vec![Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall],
        ]))

        .add_state::<state::State>()

        .add_systems(Startup, (
                map::spawn_map, 
                camera::spawn_camera, 
                player::spawn_player
        ))

        .add_systems(Update,(
            (
                player::receive_input, 
                movement::update_movement,
                powerup::detect_pick_up,
                powerup::power_down,
                state::referee
            ).chain(),
        ).run_if(in_state(State::Playing)))
        
        .add_systems(OnEnter(State::Lost), ui::game_over)
        .add_systems(OnEnter(State::Won), ui::game_won)

        .run();
}
