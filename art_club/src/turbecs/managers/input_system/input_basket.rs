use turbo::*;

use crate::GameState;

#[turbo::serialize]
#[derive(PartialEq)]
pub struct InputBasket {

    pub up : bool,
    pub down : bool,
    pub left : bool,
    pub right : bool,

}

impl InputBasket {
    pub fn new() -> Self {

        return Self {
            up : false,
            down : false,
            left : false,
            right : false
        };

    }
}

impl InputBasket {

    pub fn new_w_state(state : &mut GameState) -> Self {

        let mut to_return = Self::new();

        to_return.up = state.input_manager.up.pressed();
        to_return.down = state.input_manager.down.pressed();
        to_return.left = state.input_manager.left.pressed();
        to_return.right = state.input_manager.right.pressed();

        return to_return;

    }

}