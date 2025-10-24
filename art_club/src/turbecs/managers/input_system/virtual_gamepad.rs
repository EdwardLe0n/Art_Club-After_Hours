use turbo::*;

use crate::{turbecs};

use turbecs::managers::input_system;
use input_system::{input_actions::InputAction, input_render_pair::InputRenderPair};

const BUTTON_SIZE : i32 = 20; 
const BUTTON_OFFSET : i32 = 4;
const BUTTON_COLOR : u32 = 0xffffff99;
const BUTTN_BORDER_SIZE : u32 = 0;
const BUTTN_BORDER_COLOR : u32 = 0x00000000;
const BUTTN_BORDER_RADIUS : i32 = 4;

#[turbo::serialize]
#[derive(PartialEq)]
pub struct VirtualGamepad {

    pub mobile : bool,

    pub up : InputAction,
    pub left : InputAction,
    pub down : InputAction,
    pub right : InputAction,

}

impl VirtualGamepad {
    pub fn new() -> Self {
        return Self {
            mobile: true,

            // All button types
            up : InputAction::new(),
            left : InputAction::new(),
            down : InputAction::new(),
            right : InputAction::new(),

        };
    }
}

impl VirtualGamepad {
    pub fn update(&mut self) {

        self.update_button_states();

        if self.mobile {
            self.check_mobile();
        }
        else {
            self.check_other();
        }

    }

    pub fn render(&mut self) { 

        if !self.mobile {
            return;
        }

        self.up.render_w_mobile(
            BUTTON_OFFSET * 2 + BUTTON_SIZE,
            (screen().h() as i32) + (-BUTTON_SIZE - BUTTON_OFFSET) * 2,
            BUTTON_SIZE,
            BUTTON_SIZE,
            BUTTON_COLOR,
            BUTTN_BORDER_SIZE,
            BUTTN_BORDER_COLOR,
            BUTTN_BORDER_RADIUS,
            InputRenderPair::Letter("w".to_string())
        );

        self.left.render_w_mobile(
            BUTTON_OFFSET,
            (screen().h() as i32) - BUTTON_SIZE - BUTTON_OFFSET,
            BUTTON_SIZE,
            BUTTON_SIZE,
            BUTTON_COLOR,
            BUTTN_BORDER_SIZE,
            BUTTN_BORDER_COLOR,
            BUTTN_BORDER_RADIUS,
            InputRenderPair::Letter("a".to_string())
        );

        self.down.render_w_mobile(
            BUTTON_OFFSET * 2 + BUTTON_SIZE,
            (screen().h() as i32) + (-BUTTON_SIZE - BUTTON_OFFSET),
            BUTTON_SIZE,
            BUTTON_SIZE,
            BUTTON_COLOR,
            BUTTN_BORDER_SIZE,
            BUTTN_BORDER_COLOR,
            BUTTN_BORDER_RADIUS,
            InputRenderPair::Letter("s".to_string())
        );

        self.right.render_w_mobile(
            BUTTON_OFFSET * 3 + BUTTON_SIZE * 2,
            (screen().h() as i32) + (-BUTTON_SIZE - BUTTON_OFFSET),
            BUTTON_SIZE,
            BUTTON_SIZE,
            BUTTON_COLOR,
            BUTTN_BORDER_SIZE,
            BUTTN_BORDER_COLOR,
            BUTTN_BORDER_RADIUS,
            InputRenderPair::Letter("d".to_string())
        );

    }

}

impl VirtualGamepad {

    fn update_button_states(&mut self) {

        self.up.update_state();
        self.left.update_state();
        self.down.update_state();
        self.right.update_state();

    }

    fn check_other(&mut self) {
        let p1_gamepad = gamepad::get(0);

        self.up.update_w_gamepad(p1_gamepad.up as u32);
        self.left.update_w_gamepad(p1_gamepad.left as u32);
        self.down.update_w_gamepad(p1_gamepad.down as u32);
        self.right.update_w_gamepad(p1_gamepad.right as u32);

    }

    fn check_mobile(&mut self) {

        let screen_pointer = pointer::screen();

        // up check

        self.up.update_w_mobile(
            &screen_pointer,
            BUTTON_OFFSET * 2 + BUTTON_SIZE,
            (screen().h() as i32) + (-BUTTON_SIZE - BUTTON_OFFSET) * 2,
            BUTTON_SIZE, BUTTON_SIZE
        );

        // left check

        self.left.update_w_mobile(
            &screen_pointer,
            BUTTON_OFFSET,
            (screen().h() as i32) + (-BUTTON_SIZE - BUTTON_OFFSET),
            BUTTON_SIZE, BUTTON_SIZE
        );

        // down check

        self.down.update_w_mobile(
            &screen_pointer,
            BUTTON_OFFSET * 2 + BUTTON_SIZE,
            (screen().h() as i32) + (-BUTTON_SIZE - BUTTON_OFFSET),
            BUTTON_SIZE, BUTTON_SIZE
        );

        // right check

        self.right.update_w_mobile(
            &screen_pointer,
            BUTTON_OFFSET * 3 + BUTTON_SIZE * 2,
            (screen().h() as i32) + (-BUTTON_SIZE - BUTTON_OFFSET),
            BUTTON_SIZE, BUTTON_SIZE
        );

    }

}