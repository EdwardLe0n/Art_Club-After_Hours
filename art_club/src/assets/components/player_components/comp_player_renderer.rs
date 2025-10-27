// Core directories

use turbo::*;

use crate::{turbecs, GameState};

use turbecs::entity::Entity;
use turbecs::helpers::transform::Transform;

const PLAYER_HEIGHT : i32 = 24;
const PLAYER_WIDTH : i32 = 16;

#[turbo::serialize]
#[derive(PartialEq)]
pub struct PlayerRendererComponent {
    
}

impl PlayerRendererComponent {

    pub fn new() -> Self {
        return Self {};
    }

}

impl PlayerRendererComponent {

    pub fn render(&self, transform : Transform) {

        sprite!(
            "player_sprites/otter_test",
            x = transform.get_x(),
            y = transform.get_y(),
            w = PLAYER_WIDTH,
            h = PLAYER_HEIGHT,
            tx = -2 * PLAYER_WIDTH,
            ty = 0,
            cover = false,
            fixed = false
        );

    }
        
}