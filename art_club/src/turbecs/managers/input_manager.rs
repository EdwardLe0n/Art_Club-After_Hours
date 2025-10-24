use turbo::*;
use crate::{turbecs};

use turbecs::managers::input_system;
use input_system::{input_actions::InputAction, input_render_pair::InputRenderPair, virtual_gamepad::VirtualGamepad};

#[turbo::serialize]
#[derive(PartialEq)]
pub struct InputManager {

    pub p1 : VirtualGamepad,

}

impl InputManager {
    
    pub fn new() -> Self {
        return Self {
            
            p1 : VirtualGamepad::new()

        };
    }

}

impl InputManager {

    pub fn update(&mut self) {

        self.p1.update();

    }

    pub fn render(&mut self) { 

        self.p1.render();

    }

}