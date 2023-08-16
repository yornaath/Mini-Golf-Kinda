use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::powerup::{PoweredUp, PowerupType};

#[derive(Debug, Bundle)]
pub struct MovementBundle {
    pub controller: Controller,
    pub max_speed: MaxSpeed,
    pub acceleration: Acceleration,
    pub jump_height: JumpHeight,
}

impl MovementBundle {
    pub fn new(max_speed: f32, acceleration: f32, jump_height: f32) -> Self {
        Self {
            controller: Controller::default(),
            max_speed: MaxSpeed(max_speed),
            acceleration: Acceleration(acceleration),
            jump_height: JumpHeight(jump_height),
        }
    }
}

#[derive(Debug, Component)]
pub struct Controller {
    pub left_pressed: bool,
    pub right_pressed: bool,
    pub up_pressed: bool,
    pub down_pressed: bool,
    pub jump_just_pressed: bool,
}

impl Default for Controller {
    fn default() -> Self {
        Self {
            left_pressed: false,
            right_pressed: false,
            up_pressed: false,
            down_pressed: false,
            jump_just_pressed: false,
        }
    }
}

#[derive(Debug, Component)]
pub struct MaxSpeed(pub f32);

#[derive(Debug, Component)]
pub struct Acceleration(pub f32);

#[derive(Debug, Component)]
pub struct JumpHeight(pub f32);

pub fn update_movement(
    mut query: Query<(
        &mut Velocity,
        &Controller,
        &MaxSpeed,
        &Acceleration,
        &JumpHeight,
        Option<&PoweredUp>,
    )>,
) {
    for (mut velocity, controller, max_speed, acceleration, jump_height, powered_up) in &mut query {
        let right: f32 = if controller.right_pressed { 1. } else { -1. };
        let left: f32 = if controller.left_pressed { 1. } else { -1. };
        let up: f32 = if controller.up_pressed { 1. } else { -1. };
        let down: f32 = if controller.down_pressed { 1. } else { -1. };

        let (max_speed, acceleration) = if let Some(PoweredUp {
            powerup_type: PowerupType::Boost,
            ..
        }) = powered_up
        {
            (max_speed.0 * 1.5, (acceleration.0 / 700.))
        } else {
            (max_speed.0, (acceleration.0 / 1000.))
        };

        let jump_height = if let Some(PoweredUp {
            powerup_type: PowerupType::Antigravity,
            ..
        }) = powered_up
        {
            jump_height.0 * 2.
        } else {
            jump_height.0
        };

        velocity.linvel.z += ((acceleration) * (down - up)).clamp(max_speed * -1., max_speed);
        velocity.linvel.x += ((acceleration) * (right - left)).clamp(max_speed * -1., max_speed);

        if controller.jump_just_pressed {
            velocity.linvel.y = jump_height;
        }
    }
}
