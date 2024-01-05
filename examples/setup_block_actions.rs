//! This example shows how to use define block actions for blocks
//! # What are block actions?
//! Block actions, are comprised of a single trigger, which is evaluated to `true` or `false`
//! and a set of actions (= bevy systems) that are executed if the trigger is `true`.
//!
//! # How do I define block actions?
//! Block actions are defined after the `init_block` method, using the `with_block_actions`
//! method. The `with_block_actions` method takes 3 arguments:
//! - The trigger, which is a function that returns a `bool` and optionally takes a `In<BlockWorldUpdateEvent>` as an input.
//! - No input actions, a set of actions (= bevy systems) that don't take any input.
//! - Input actions, a set of actions (= bevy systems) that take a `In<BlockWorldUpdateEvent>` as an input.

use bevy::prelude::*;
pub use bevy_moxi as moxi; // Import moxi, the alias is optional
use moxi::prelude::*;

pub const CHUNK_DIMS: Dimensions = Dimensions::new(16, 16, 16); // The dimensions of a chunk

config_from_dimensions!(CHUNK_DIMS); // This macro configures a lot of types that take
                                     // a generic const. The macro is optional but highly recommended.

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, MoxiBptaPlugin::default())); // add the plugin
    app.init_block::<MyBlock>() // Init a block
        .with_block_actions(trigger1, (action2, action2, action2), ()) // define block actions
        .with_block_actions(trigger2, action2, action1) // we can add as many block actions as we
        // want, also, we can use the same trigger for multiple actions, or multiple actions for
        // several block actions
        .init_block::<MyBlock2>(); // Init another block without block actions
    app.run();
}

// Trigger that doesn't take any input
fn trigger1() -> bool {
    true
}

// Trigger that takes a `In<BlockWorldUpdateEvent>` as an input
fn trigger2(block_world_update: In<BlockWorldUpdateEvent>) -> bool {
    block_world_update.0.block_pos().x % 2 == 0
}

// Action that doesn't take any input, note actions are basically bevy systems,
// so they can do anything a bevy system can do
fn action2(all_query: Query<Entity>) {
    all_query.for_each(|ent| println!("Found entity: {:?} ", ent));
}

// Action that takes a `In<BlockWorldUpdateEvent>` as an input
fn action1(_: In<BlockWorldUpdateEvent>) {
    println!("Action 1");
}

struct MyBlock; // Define a block
struct MyBlock2; // Define another block

impl Block for MyBlock {
    fn get_name() -> &'static str {
        "MyBlock" // define the name of the block
    }
    // NOTE: We didn't define a mesh for the block, so it will be invisible (the same as Air)
}

impl Block for MyBlock2 {
    fn get_name() -> &'static str {
        "MyBlock2" // define the name of the block
    }
    // NOTE: We didn't define a mesh for the block, so it will be invisible (the same as Air)
}
