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
    action_type: ActionType,
}

#[derive(Event)]
pub struct SecondAction {
    action_type: ActionType,
}

/// Broadcast actions from keyboard input
pub fn broadcast_actions(
    mut prime_action: EventWriter<PrimeAction>,
    mut second_action: EventWriter<SecondAction>,
    buttons: Res<Input<MouseButton>>,
    action_binds: Res<ActionKeyBinds>,
) {
    let prime_action_key = action_binds.prime_action;
    let second_action_key = action_binds.second_action;
    if buttons.just_pressed(prime_action_key) {
        prime_action.send(PrimeAction {
            action_type: ActionType::Start,
        });
    }
    if buttons.just_released(prime_action_key) {
        prime_action.send(PrimeAction {
            action_type: ActionType::Stop,
        });
    }
    if buttons.just_pressed(second_action_key) {
        second_action.send(SecondAction {
            action_type: ActionType::Start,
        });
    }
    if buttons.just_released(second_action_key) {
        second_action.send(SecondAction {
            action_type: ActionType::Stop,
        });
    }
}

pub fn handle_prime_action(
    mut blocks: BlocksMutX,
    target_block: Res<TargetBlock>,
    mut primary_actions: EventReader<PrimeAction>,
) {
    if target_block.ignore_flag {
        return;
    }
    for action in primary_actions.read() {
        if matches!(action.action_type, ActionType::Start) {
            blocks.set_block_at_name(target_block.chunk_cords, target_block.block_pos, "Air");
        }
    }
}

pub fn handle_second_action(
    mut blocks: BlocksMutX,
    target_block: Res<TargetBlock>,
    mut secondary_actions: EventReader<SecondAction>,
) {
    for action in secondary_actions.read() {
        if target_block.ignore_flag || target_block.face_hit.is_none() {
            return;
        }

        let global_block_to_place_pos = global_neighbor(
            BlockGlobalPos::new(target_block.block_pos, target_block.chunk_cords),
            target_block.face_hit.unwrap(),
            CHUNK_DIMS,
        );
        if !global_block_to_place_pos.valid {
            return;
        }
        if matches!(action.action_type, ActionType::Start) {
            blocks.set_block_at_name(
                global_block_to_place_pos.cords,
                global_block_to_place_pos.pos,
                "Sand",
            );
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
