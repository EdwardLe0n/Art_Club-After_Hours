// Core directories

use turbo::*;

use crate::{turbecs, GameState};

use turbecs::managers;


#[turbo::serialize]
#[derive(PartialEq)]
pub struct CrochetHandlerComponent {
    
}

impl CrochetHandlerComponent {
    pub fn new() -> Self {
        return Self{
            
        };
    }
}

impl CrochetHandlerComponent {
    
    pub fn update(&mut self, state :&mut GameState) {

    }

    pub fn render(&self, state :&mut GameState) {

        rect!(
            x = screen().w() as f32 * 0.1,
            y = screen().h() as f32 * 0.1,
            w = screen().w() as f32 * 0.8,
            h = screen().h() as f32 * 0.8,
            color = 0x0000ffff,
            fixed = true
        );

    }

}