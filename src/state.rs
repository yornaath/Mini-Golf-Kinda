use crate::{
    map::{Floor, Tile},
    player::Player,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum State {
    #[default]
    Playing,
    Lost,
    Won,
}

pub fn referee(
    mut commands: Commands,
    mut next_state: ResMut<NextState<State>>,
    rapier_context: Res<RapierContext>,
    mut player_query: Query<(Entity), With<Player>>,
    tiles_query: Query<(Entity, &Tile)>,
    mut floor_query: Query<(Entity), With<Floor>>,
) {
    for (tile_entity, tile) in tiles_query.iter() {
        match tile {
            Tile::Hole => {
                for player_entity in player_query.iter_mut() {
                    if rapier_context
                        .intersection_pair(player_entity, tile_entity)
                        .is_some()
                    {
                        for (floor_entity) in floor_query.iter_mut() {
                            commands
                                .entity(floor_entity)
                                .remove::<RigidBody>()
                                .remove::<Collider>();

                            next_state.set(State::Lost);
                        }
                    }
                }
            }
            Tile::Goal => {
                for player_entity in player_query.iter_mut() {
                    if rapier_context
                        .intersection_pair(player_entity, tile_entity)
                        .is_some()
                    {
                        commands.entity(player_entity).despawn_recursive();
                        next_state.set(State::Won);
                    }
                }
            }
            _ => {}
        }
    }
}
