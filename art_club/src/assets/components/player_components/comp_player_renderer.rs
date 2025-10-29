// Core directories

use turbo::*;

use crate::{turbecs, GameState};

use turbecs::entity::Entity;
use turbecs::helpers::transform::Transform;

const PLAYER_HEIGHT : i32 = 24;
const PLAYER_WIDTH : i32 = 16;
const ANIMATION_MOD : f32 = 3.0;

#[turbo::serialize]
#[derive(PartialEq)]
pub enum PlayerDirection {
    Up,
    Down,
    Left,
    Right
}

#[turbo::serialize]
#[derive(PartialEq)]
pub enum PlayerState {
    Idle,
    Walking
}

#[turbo::serialize]
#[derive(PartialEq)]
pub struct PlayerRendererComponent {
    direction : PlayerDirection,
    curr_state : PlayerState,
    elapsed : f32
}

impl PlayerRendererComponent {

    pub fn new() -> Self {
        return Self {
            direction : PlayerDirection::Down,
            curr_state : PlayerState::Idle,
            elapsed : 0.0
        };
    }

}

impl PlayerRendererComponent {

    pub fn update(&mut self, state : &mut GameState) {

        self.update_time(state);

    }

    pub fn update_time(&mut self, state : &mut GameState) {

        self.elapsed += state.time_manager.delta * ANIMATION_MOD;

        if self.elapsed >= 4.0 {
            self.elapsed -= 4.0
        }

    }

    pub fn render(&self, transform : Transform) {

        sprite!(
            "player_sprites/otter_test",
            x = transform.get_x(),
            y = transform.get_y(),
            w = PLAYER_WIDTH,
            h = PLAYER_HEIGHT,
            tx = -((self.elapsed) as i32) * PLAYER_WIDTH,
            ty = -1 * PLAYER_HEIGHT,
            cover = false,
            fixed = false
        );

    }
        
}