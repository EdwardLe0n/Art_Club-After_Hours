use turbo::*;

use crate::{GameState, turbecs, assets};

#[turbo::serialize]
#[derive(PartialEq)]
pub enum Minigames { 
    Crochet,
    None
}

#[turbo::serialize]
#[derive(PartialEq)]
pub struct MinigameManager {
    pub is_active : bool,
    pub curr_game : Minigames
}

impl MinigameManager {
    
    pub fn new() -> Self {
        return Self {
            is_active : false,
            curr_game : Minigames::None
        };
    } 

}

impl MinigameManager {
    pub fn start_minigame(&mut self, some_minigame : Minigames) {

        self.is_active = true;
        self.curr_game = some_minigame;

    }
}

impl GameState {
    
    pub fn is_minigame_active(&self) -> bool {
        return self.minigame_manager.is_active;
    }

}