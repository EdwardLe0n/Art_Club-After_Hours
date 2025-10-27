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
pub struct InputManager {

    pub mobile : bool,

    pub up : InputAction,
    pub left : InputAction,
    pub down : InputAction,
    pub right : InputAction,

    pub a : InputAction,
    pub b : InputAction,
    pub x : InputAction,
    pub y : InputAction,

    pub select : InputAction,
    pub start : InputAction,

}

impl InputManager {
    
    pub fn new() -> Self {
        return Self {
            mobile: true,

            // All button types
            up : InputAction::new(),
            left : InputAction::new(),
            down : InputAction::new(),
            right : InputAction::new(),

            a : InputAction::new(),
            b : InputAction::new(),
            x : InputAction::new(),
            y : InputAction::new(),

            select : InputAction::new(),
            start : InputAction::new()

        };
    }

}

impl InputManager {

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

        // main non - wasd buttons 

        self.a.render_w_mobile(
            screen().w() as i32 - (BUTTON_OFFSET + BUTTON_SIZE) * 2,
            (screen().h() as i32) - BUTTON_SIZE - BUTTON_OFFSET,
            BUTTON_SIZE,
            BUTTON_SIZE,
            BUTTON_COLOR,
            BUTTN_BORDER_SIZE,
            BUTTN_BORDER_COLOR,
            BUTTN_BORDER_RADIUS,
            InputRenderPair::Letter("a".to_string())
        );

        self.b.render_w_mobile(
            screen().w() as i32 - (BUTTON_OFFSET + BUTTON_SIZE),
            (screen().h() as i32) + ((- BUTTON_SIZE - BUTTON_OFFSET) as f32 * 1.5) as i32,
            BUTTON_SIZE,
            BUTTON_SIZE,
            BUTTON_COLOR,
            BUTTN_BORDER_SIZE,
            BUTTN_BORDER_COLOR,
            BUTTN_BORDER_RADIUS,
            InputRenderPair::Letter("b".to_string())
        );

        self.y.render_w_mobile(
            screen().w() as i32 - (BUTTON_OFFSET + BUTTON_SIZE) * 2,
            (screen().h() as i32)  + (- BUTTON_SIZE - BUTTON_OFFSET) * 2,
            BUTTON_SIZE,
            BUTTON_SIZE,
            BUTTON_COLOR,
            BUTTN_BORDER_SIZE,
            BUTTN_BORDER_COLOR,
            BUTTN_BORDER_RADIUS,
            InputRenderPair::Letter("y".to_string())
        );

        self.x.render_w_mobile(
            screen().w() as i32 - (BUTTON_OFFSET + BUTTON_SIZE) * 3,
            (screen().h() as i32) + ((- BUTTON_SIZE - BUTTON_OFFSET) as f32 * 1.5) as i32,
            BUTTON_SIZE,
            BUTTON_SIZE,
            BUTTON_COLOR,
            BUTTN_BORDER_SIZE,
            BUTTN_BORDER_COLOR,
            BUTTN_BORDER_RADIUS,
            InputRenderPair::Letter("x".to_string())
        );

        self.select.render_w_mobile(
            (screen().w() / 2) as i32 - (BUTTON_OFFSET + BUTTON_SIZE * 2),
            BUTTON_SIZE / 8,
            BUTTON_SIZE * 2,
            BUTTON_SIZE,
            BUTTON_COLOR,
            BUTTN_BORDER_SIZE,
            BUTTN_BORDER_COLOR,
            BUTTN_BORDER_RADIUS,
            InputRenderPair::Letter("slct".to_string())
        );

        self.start.render_w_mobile(
            (screen().w() / 2) as i32 + (BUTTON_OFFSET),
            BUTTON_SIZE / 8,
            BUTTON_SIZE * 2,
            BUTTON_SIZE,
            BUTTON_COLOR,
            BUTTN_BORDER_SIZE,
            BUTTN_BORDER_COLOR,
            BUTTN_BORDER_RADIUS,
            InputRenderPair::Letter("strt".to_string())
        );


    }

}

impl InputManager {

    fn update_button_states(&mut self) {

        self.up.update_state();
        self.left.update_state();
        self.down.update_state();
        self.right.update_state();

        self.a.update_state();
        self.b.update_state();
        self.x.update_state();
        self.y.update_state();

        self.select.update_state();
        self.start.update_state();

    }

    fn check_other(&mut self) {
        let p1_gamepad = gamepad::get(0);

        self.up.update_w_gamepad(p1_gamepad.up as u32);
        self.left.update_w_gamepad(p1_gamepad.left as u32);
        self.down.update_w_gamepad(p1_gamepad.down as u32);
        self.right.update_w_gamepad(p1_gamepad.right as u32);

        self.a.update_w_gamepad(p1_gamepad.a as u32);
        self.b.update_w_gamepad(p1_gamepad.b as u32);
        self.x.update_w_gamepad(p1_gamepad.x as u32);
        self.y.update_w_gamepad(p1_gamepad.y as u32);

        self.select.update_w_gamepad(p1_gamepad.select as u32);
        self.start.update_w_gamepad(p1_gamepad.start as u32);

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


        // non wasd inputs

        self.a.update_w_mobile(
            &screen_pointer,
            screen().w() as i32 - (BUTTON_OFFSET + BUTTON_SIZE) * 2,
            (screen().h() as i32) - BUTTON_SIZE - BUTTON_OFFSET,
            BUTTON_SIZE,
            BUTTON_SIZE,
        );

        self.b.update_w_mobile(
            &screen_pointer,
            screen().w() as i32 - (BUTTON_OFFSET + BUTTON_SIZE),
            (screen().h() as i32) + ((- BUTTON_SIZE - BUTTON_OFFSET) as f32 * 1.5) as i32,
            BUTTON_SIZE,
            BUTTON_SIZE,
        );

        self.y.update_w_mobile(
            &screen_pointer,
            screen().w() as i32 - (BUTTON_OFFSET + BUTTON_SIZE) * 2,
            (screen().h() as i32)  + (- BUTTON_SIZE - BUTTON_OFFSET) * 2,
            BUTTON_SIZE,
            BUTTON_SIZE,
        );

        self.x.update_w_mobile(
            &screen_pointer,
            screen().w() as i32 - (BUTTON_OFFSET + BUTTON_SIZE) * 3,
            (screen().h() as i32) + ((- BUTTON_SIZE - BUTTON_OFFSET) as f32 * 1.5) as i32,
            BUTTON_SIZE,
            BUTTON_SIZE,
        );

        self.select.update_w_mobile(
            &screen_pointer,
            (screen().w() / 2) as i32 - (BUTTON_OFFSET + BUTTON_SIZE * 2),
            BUTTON_SIZE / 8,
            BUTTON_SIZE * 2,
            BUTTON_SIZE,
        );

        self.start.update_w_mobile(
            &screen_pointer,
            (screen().w() / 2) as i32 + (BUTTON_OFFSET),
            BUTTON_SIZE / 8,
            BUTTON_SIZE * 2,
            BUTTON_SIZE,
        );

    }
}