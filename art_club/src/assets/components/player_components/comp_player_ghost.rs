// Core directories

use turbo::*;

use crate::{turbecs, GameState, assets};
use turbecs::entity::Entity;

use turbecs::managers::input_system::input_basket::InputBasket;
use assets::components::player_components::comp_player_controller::PLAYER_SPEED;

#[turbo::serialize]
#[derive(PartialEq)]
pub struct PlayerGhostComponent {
    player_id : String,
    held_inputs : InputBasket
}

impl PlayerGhostComponent {

    pub fn new(some_id : String) -> Self {

        return Self { player_id : some_id, held_inputs : InputBasket::new() };

    }

    pub fn new_w_basket(some_id : String, some_basket : InputBasket) -> Self {

        let mut to_return = Self::new(some_id);

        to_return.held_inputs = some_basket;

        return to_return;

    }

}

impl PlayerGhostComponent {

    pub fn on_awake(&mut self, ent : &mut Entity, state : &mut GameState) {

        log!("making ghost!");

        state.online_player_manager.add_player_ref(self.player_id.clone(), ent.locat);

    }

    pub fn update(&mut self, ent : &mut Entity, state : &mut GameState) {

        if (self.held_inputs.up || self.held_inputs.down) && !(self.held_inputs.up && self.held_inputs.down) {

            if self.held_inputs.up {

                ent.transform.nudge_y((PLAYER_SPEED * state.time_manager.delta) as i32);

            }
            else {
                ent.transform.nudge_y(-(PLAYER_SPEED * state.time_manager.delta) as i32);
            }

        }

        if (self.held_inputs.right || self.held_inputs.left) && !(self.held_inputs.right && self.held_inputs.left) {

            if self.held_inputs.right {

                ent.transform.nudge_x((PLAYER_SPEED * state.time_manager.delta) as i32);

            }
            else {
                ent.transform.nudge_x(-(PLAYER_SPEED * state.time_manager.delta) as i32);
            }

        }

    }

}

impl PlayerGhostComponent {

    pub fn update_basket(&mut self, basket : InputBasket) {

        self.held_inputs = basket;

    }

}