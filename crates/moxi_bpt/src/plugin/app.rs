use bevy_app::App;

use crate::{
    block::Block,
    prelude::{BlockInitiallizerTrait, BlockWorldMut},
};

pub trait MoxiApp {
    fn init_block<'w, B: Block>(&'w mut self) -> BlockWorldMut<'w>;
}

impl MoxiApp for App {
    fn init_block<'w, B: Block>(&'w mut self) -> BlockWorldMut<'w> {
        self.world.init_block::<B>()
    }
}
