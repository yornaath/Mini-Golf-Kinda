use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Debug, Bundle)]
pub struct MovementBundle {
    pub controller: Controller,
    pub run_speed: RunSpeed,
    pub jump_height: JumpHeight,
}

impl MovementBundle {
    pub fn new(run_speed: f32, jump_height: f32) -> Self {
        Self {
            controller: Controller::default(),
            run_speed: RunSpeed(run_speed),
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
pub struct RunSpeed(pub f32);

#[derive(Debug, Component)]
pub struct JumpHeight(pub f32);

const MIN_VELOCITY: f32 = -4.;
const MAX_VELOCITY: f32 = 4.;

pub fn update_movement(mut query: Query<(&mut Velocity, &Controller, &RunSpeed, &JumpHeight)>) {
    for (mut velocity, controller, run_speed, jump_height) in &mut query {
        let right: f32 = if controller.right_pressed { 1. } else { -1. };
        let left: f32 = if controller.left_pressed { 1. } else { -1. };
        let up: f32 = if controller.up_pressed { 1. } else { -1. };
        let down: f32 = if controller.down_pressed { 1. } else { -1. };

        velocity.linvel.z += ((run_speed.0 / 100.) * (down - up)).clamp(MIN_VELOCITY, MAX_VELOCITY);
        velocity.linvel.x += (0.04 * (right - left)).clamp(MIN_VELOCITY, MAX_VELOCITY);

        if controller.jump_just_pressed {
            velocity.linvel.y = jump_height.0;
        }
    }
}
