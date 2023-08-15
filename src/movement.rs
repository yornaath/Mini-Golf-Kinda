use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

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
    )>,
) {
    for (mut velocity, controller, max_speed, acceleration, jump_height) in &mut query {
        let right: f32 = if controller.right_pressed { 1. } else { -1. };
        let left: f32 = if controller.left_pressed { 1. } else { -1. };
        let up: f32 = if controller.up_pressed { 1. } else { -1. };
        let down: f32 = if controller.down_pressed { 1. } else { -1. };

        velocity.linvel.z +=
            ((acceleration.0 / 1000.) * (down - up)).clamp(max_speed.0 * -1., max_speed.0);

        velocity.linvel.x +=
            ((acceleration.0 / 1000.) * (right - left)).clamp(max_speed.0 * -1., max_speed.0);

        if controller.jump_just_pressed {
            velocity.linvel.y = jump_height.0;
        }
    }
}
