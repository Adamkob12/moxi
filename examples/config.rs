//! This example shows how to configure the plugin with a custom chunk size.

use bevy::prelude::*;
pub use bevy_moxi as moxi; // Import moxi, the alias is optional
use moxi::prelude::*;

pub const CHUNK_DIMS: Dimensions = Dimensions::new(16, 16, 16); // The dimensions of a chunk

config_from_dimensions!(CHUNK_DIMS); // This macro configures a lot of types that take
                                     // a generic const. The macro is optional but highly recommended.
fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, MoxiBptaPlugin::default())); // add the plugin

    app.run();
}
