use turbo::*;

use crate::turbecs::managers::input_system::input_states::InputStates;

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

        log!("just pressed!");

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
