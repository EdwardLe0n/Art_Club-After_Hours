// Core directories

use turbo::*;

use crate::{turbecs, GameState};

use turbecs::entity::Entity;

#[turbo::serialize]
#[derive(PartialEq)]
pub struct PlayerGhostComponent {
    player_id : String
}

impl PlayerGhostComponent {

    pub fn new(some_id : String) -> Self {

        return Self { player_id : some_id };

    }

}

impl PlayerGhostComponent {

    pub fn on_awake(&mut self, ent : &mut Entity, state : &mut GameState) {

        log!("making ghost!");

        state.online_player_manager.add_player_ref(self.player_id.clone(), ent.locat);

    }

}