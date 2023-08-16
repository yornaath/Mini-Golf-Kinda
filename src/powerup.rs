use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{map::Tile, player::Player};

#[derive(Debug, Clone, Component)]
pub enum PowerupType {
    Boost,
    Antigravity,
}

#[derive(Component, Debug)]
pub struct PoweredUp {
    pub timer: Timer,
    pub powerup_type: PowerupType,
}

pub fn detect_pick_up(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut player_query: Query<(Entity), With<Player>>,
    powerups_query: Query<(Entity, &PowerupType)>,
) {
    for (powerup_entity, powerup_type) in powerups_query.iter() {
        match powerup_type {
            PowerupType::Antigravity => {
                for player_entity in player_query.iter_mut() {
                    if rapier_context
                        .intersection_pair(player_entity, powerup_entity)
                        .is_some()
                    {
                        commands.entity(player_entity).insert(PoweredUp {
                            timer: Timer::new(Duration::from_secs(5), TimerMode::Once),
                            powerup_type: powerup_type.clone(),
                        });
                        commands.entity(powerup_entity).despawn();
                    }
                }
            }
            PowerupType::Boost => {
                for player_entity in player_query.iter_mut() {
                    if rapier_context
                        .intersection_pair(player_entity, powerup_entity)
                        .is_some()
                    {
                        commands.entity(player_entity).insert(PoweredUp {
                            timer: Timer::new(Duration::from_secs(5), TimerMode::Once),
                            powerup_type: powerup_type.clone(),
                        });
                        commands.entity(powerup_entity).despawn();
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn power_down(
    mut commands: Commands,
    mut powered_up_player_query: Query<(Entity, &mut PoweredUp), With<Player>>,
    time: Res<Time>,
) {
    for (player_entity, mut powered_up) in powered_up_player_query.iter_mut() {
        powered_up.timer.tick(time.delta());

        if powered_up.timer.finished() {
            commands.entity(player_entity).remove::<PoweredUp>();
        }
    }
}
