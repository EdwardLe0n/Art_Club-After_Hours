// Core directories

use turbo::*;

use crate::{turbecs, GameState, assets};

use turbecs::helpers::transform::Transform;

const PLAYER_HEIGHT : i32 = 24;
const PLAYER_WIDTH : i32 = 16;
const ANIMATION_MOD : f32 = 2.5;

use assets::components::player_components::player_enums;
use player_enums::{PlayerDirection, PlayerState};

#[turbo::serialize]
#[derive(PartialEq)]
pub struct PlayerRendererComponent {
    pub direction : PlayerDirection,
    pub curr_state : PlayerState,
    pub elapsed : f32
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

    pub fn render(&self, transform : Transform) {

        let anim_data = self.get_animation_data();

        sprite!(
            "player_sprites/otter_test",
            x = transform.get_x(),
            y = transform.get_y(),
            w = PLAYER_WIDTH,
            h = PLAYER_HEIGHT,
            tx = -((self.elapsed) as i32 + anim_data.0) * PLAYER_WIDTH,
            ty = -anim_data.1 * PLAYER_HEIGHT,
            cover = false,
            fixed = false
        );

    }
        
}

impl PlayerRendererComponent {
    
    pub fn update_time(&mut self, state : &mut GameState) {

        self.elapsed += state.time_manager.delta * ANIMATION_MOD;

        let len = self.get_animation_len();

        if self.elapsed >= len {
            self.elapsed -= len;
        }

    }

    pub fn get_animation_data(&self) -> (i32, i32) {

        let mut to_return = (0, 0);

        match &self.curr_state {
            PlayerState::Idle => {

                

            }
            PlayerState::Walking => {

                to_return.1 = 1;

            }
            _default => {}
        }

        return to_return;

    }

    pub fn get_animation_len(&self) -> f32 {

        let mut val_to_return = 0.0;

        match &self.curr_state {
            PlayerState::Idle => {

                val_to_return = 4.0;

            }
            PlayerState::Walking => {

                val_to_return = 4.0;

            }
            _default => {

                val_to_return = 1.0;

            }
        }

        return val_to_return;

    }

}