// Core directories

use turbo::*;

use crate::{turbecs, GameState};

use turbecs::managers;


#[turbo::serialize]
#[derive(PartialEq)]
pub struct MinigameHandlerComponent {
    
}

impl MinigameHandlerComponent {
    pub fn new() -> Self {
        return Self{
            
        };
    }
}

impl MinigameHandlerComponent {
    
    pub fn update(&mut self, state :&mut GameState) {

    }

    pub fn render(&self, state :&mut GameState) {
        state.minigame_manager.render_minigame();
    }

}