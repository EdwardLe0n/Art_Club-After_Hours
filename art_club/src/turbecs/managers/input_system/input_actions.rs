use turbo::*;
use pointer::ScreenPointer;

use crate::turbecs::managers::input_system;

use input_system::input_states::InputStates;
use input_system::input_render_pair::InputRenderPair;

#[turbo::serialize]
#[derive(PartialEq)]
pub struct InputAction {

    pub state : InputStates,

}

impl InputAction {
    
    pub fn new() -> Self {
        return Self {
            state : InputStates::Released
        };
    }

}

impl InputAction {

    pub fn update_w_mobile(&mut self, s_pointer : &ScreenPointer, x : i32, y : i32, w : i32, h : i32) {

        if s_pointer.intersects(x, y, w, h) {

            if s_pointer.pressed() {

                if !self.pressed() {
                    self.turn_to_pressed();
                }

            }
            else if s_pointer.just_released() {
                self.turn_to_released();
            }

        }
        else if self.pressed() {
            self.turn_to_released();
        }

    }

    pub fn update_w_gamepad(&mut self, input_state : u32) {

        if input_state == 2 {
            if !self.pressed() {
                self.turn_to_pressed();
            }
        }
        else if input_state == 0 {
            self.turn_to_released();
        }

    }

    pub fn render_w_mobile(&self, x : i32, y : i32, w : i32, h : i32, color : u32, b_size : u32, b_color : u32, b_radius : i32, inp_pair : InputRenderPair) {

        rect!(
            x = x,
            y = y,
            w = w,
            h = h,
            color = color,
            rotation = 0,
            border_size = b_size,
            border_color = b_color,
            border_radius = b_radius,
            fixed = true
        );

        match inp_pair {
            InputRenderPair::Letter(text) => {

                text_box!(
                    &text,
                    font = "large",
                    w = w,
                    h = h / 2,
                    x = x,
                    y = y + (h / 4),
                    align = "center",
                    color = 0x000000ff,
                    fixed = true
                );

            },
            InputRenderPair::Image(image_ref) => {

            },
            _default => {}
        }

    }

}

impl InputAction {

    pub fn update_state(&mut self) {

        if self.just_pressed() {
            self.state = InputStates::Pressed;
        }

        if self.just_released() {
            self.state = InputStates::Released;
        }

    }

    pub fn turn_to_pressed(&mut self) {
        self.state = InputStates::JustPressed;
    }

    pub fn turn_to_released(&mut self) {
        self.state = InputStates::JustReleased;
    }
    
    pub fn pressed(&self) -> bool {

        if self.state == InputStates::Pressed || self.state == InputStates::JustPressed {

            return true;
        }

        return false;

    }

    pub fn just_pressed(&self) -> bool {

        if self.state == InputStates::JustPressed {
            return true;
        }

        return false;

    }

    pub fn released(&self) -> bool {

        if self.state == InputStates::Released || self.state == InputStates::JustReleased {
            return true;
        }

        return false;

    }

    pub fn just_released(&self) -> bool {

        if self.state == InputStates::JustReleased {
            return true;
        }

        return false;

    }

}
