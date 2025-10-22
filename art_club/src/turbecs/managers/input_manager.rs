use turbo::*;
use crate::{turbecs};

use turbecs::managers::input_system;
use input_system::{input_actions::InputAction, input_states::InputStates};

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

    pub up : InputAction

}

impl InputManager {
    
    pub fn new() -> Self {
        return Self {
            mobile: false,

            // All button types
            up : InputAction::new(),

        };
    }

}

impl InputManager {

    pub fn update(&mut self) {

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


        rect!(
            x = BUTTON_OFFSET,
            y = (screen().h() as i32) - BUTTON_SIZE - BUTTON_OFFSET,
            w = BUTTON_SIZE,
            h = BUTTON_SIZE,
            color = BUTTON_COLOR,
            rotation = 0,
            border_size = BUTTN_BORDER_SIZE,
            border_color = BUTTN_BORDER_COLOR,
            border_radius = BUTTN_BORDER_RADIUS,
            fixed = true
        );

        rect!(
            x = BUTTON_OFFSET * 2 + BUTTON_SIZE,
            y = (screen().h() as i32) + (-BUTTON_SIZE - BUTTON_OFFSET) * 2,
            w = BUTTON_SIZE,
            h = BUTTON_SIZE,
            color = BUTTON_COLOR,
            rotation = 0,
            border_size = BUTTN_BORDER_SIZE,
            border_color = BUTTN_BORDER_COLOR,
            border_radius = BUTTN_BORDER_RADIUS,
            fixed = true
        );

        rect!(
            x = BUTTON_OFFSET * 2 + BUTTON_SIZE,
            y = (screen().h() as i32) - BUTTON_SIZE - BUTTON_OFFSET,
            w = BUTTON_SIZE,
            h = BUTTON_SIZE,
            color = BUTTON_COLOR,
            rotation = 0,
            border_size = BUTTN_BORDER_SIZE,
            border_color = BUTTN_BORDER_COLOR,
            border_radius = BUTTN_BORDER_RADIUS,
            fixed = true
        );

        rect!(
            x = BUTTON_OFFSET * 3 + BUTTON_SIZE * 2,
            y = (screen().h() as i32) - BUTTON_SIZE - BUTTON_OFFSET,
            w = BUTTON_SIZE,
            h = BUTTON_SIZE,
            color = BUTTON_COLOR,
            rotation = 0,
            border_size = BUTTN_BORDER_SIZE,
            border_color = BUTTN_BORDER_COLOR,
            border_radius = BUTTN_BORDER_RADIUS,
            fixed = true
        );


    }

}

impl InputManager {

    fn update_button_states(&mut self) {

        self.up.update_state();

    }

    fn check_other(&mut self) {
        let p1_gamepad = gamepad::get(0);

        if p1_gamepad.up.pressed() {

            if !self.up.pressed() {
                self.up.turn_to_pressed();
            }

        }
        else if p1_gamepad.up.just_released() {
            self.up.turn_to_released();
        }
            

    }

    fn check_mobile(&mut self) {

        let screen_pointer = pointer::screen();

        // up check

        if (screen_pointer.intersects(
            BUTTON_OFFSET * 2 + BUTTON_SIZE, 
            (screen().h() as i32) + (-BUTTON_SIZE - BUTTON_OFFSET) * 2, 
            BUTTON_SIZE, 
            BUTTON_SIZE)
        ) {

            if screen_pointer.pressed() {

                if !self.up.pressed() {
                    self.up.turn_to_pressed();
                }

            }
            else if screen_pointer.just_released() {
                self.up.turn_to_released();
            }

        }

        // if screen_pointer.intersects(50, 30, 100, 20) {
        //     // Pointer is over the rectangle in screen coordinates
        // }
        // if screen_pointer.intersects(50, 30, 100, 20) && screen_pointer.just_pressed() {
        //     // Pointer just clicked/tapped the rectangle in screen coordinates
        // }

    }
}