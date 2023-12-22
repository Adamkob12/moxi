use crate::blocks::BlocksX;
use crate::*;
use bevy::prelude::*;

use super::TargetBlock;

/// Start of stop the action
pub enum ActionType {
    Start,
    Stop,
}

#[derive(Resource)]
pub struct ActionKeyBinds {
    prime_action: MouseButton,
    second_action: MouseButton,
}

#[derive(Event)]
pub struct PrimeAction {
    /// In millies from start
    #[allow(dead_code)]
    time_stamp: u128,
    action_type: ActionType,
}

#[derive(Event)]
pub struct SecondAction {
    // In millies from start
    #[allow(dead_code)]
    time_stamp: u128,
    action_type: ActionType,
}

/// Broadcast actions from keyboard input
pub fn broadcast_actions(
    mut prime_action: EventWriter<PrimeAction>,
    mut second_action: EventWriter<SecondAction>,
    time: Res<Time<Virtual>>,
    buttons: Res<Input<MouseButton>>,
    action_binds: Res<ActionKeyBinds>,
) {
    let prime_action_key = action_binds.prime_action;
    let second_action_key = action_binds.second_action;
    if buttons.just_pressed(prime_action_key) {
        prime_action.send(PrimeAction {
            time_stamp: time.elapsed().as_millis(),
            action_type: ActionType::Start,
        });
    }
    if buttons.just_released(prime_action_key) {
        prime_action.send(PrimeAction {
            time_stamp: time.elapsed().as_millis(),
            action_type: ActionType::Stop,
        });
    }
    if buttons.just_pressed(second_action_key) {
        second_action.send(SecondAction {
            time_stamp: time.elapsed().as_millis(),
            action_type: ActionType::Start,
        });
    }
    if buttons.just_released(second_action_key) {
        second_action.send(SecondAction {
            time_stamp: time.elapsed().as_millis(),
            action_type: ActionType::Stop,
        });
    }
}

pub fn handle_prime_action(
    mut blocks: BlocksMutX,
    target_block: Res<TargetBlock>,
    mut primary_actions: EventReader<PrimeAction>,
) {
    for action in primary_actions.read() {
        if matches!(action.action_type, ActionType::Start) {
            blocks.set_block_at_name(target_block.chunk_cords, target_block.block_pos, "Air");
        }
    }
}

impl Default for ActionKeyBinds {
    fn default() -> Self {
        ActionKeyBinds {
            prime_action: MouseButton::Left,
            second_action: MouseButton::Right,
        }
    }
}
