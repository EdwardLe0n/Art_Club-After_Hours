use turbo::*;

use std::collections::HashMap;

use crate::{GameState, turbecs, assets};

use assets::game_state;
use game_state::online_calls::channels::{NewLocalPlayer, HeardLocal, LocalJoined};
use game_state::online_calls::channels::{NewOnlinePlayer, HeardOnline, PlayerJoined};

use assets::prefabs::character_prefabs;

#[turbo::serialize]
#[derive(PartialEq)]
pub struct OnlinePlayerManager {

    pub player_id : String,
    pub online_players : HashMap<String, usize>
    
}
impl OnlinePlayerManager {
    
    pub fn new() -> Self {
        return Self {
            player_id : "".to_string(),
            online_players : HashMap::new()
        };
    }

}

impl OnlinePlayerManager {

    pub fn check_for_player(&self, some_id : &String) -> bool {
        return self.online_players.contains_key(some_id);
    }

    pub fn add_player_ref(&mut self, some_id : String, some_usize : usize) {

        self.online_players.insert(some_id, some_usize);

    }

}

impl GameState {

    pub fn handle_online(&mut self) {
        
        if self.handle_is_connected() {

            self.handle_player_joined();

        }

    }

    fn handle_is_connected(&mut self) -> bool {

        if self.online_player_manager.player_id != "" {
            return true;
        }

        if let Some(conn) = LocalJoined::subscribe("default") {

            while let Ok(msg) = conn.recv() {
                self.online_player_manager.player_id = msg.player_id;
            }

            if self.online_player_manager.player_id == "" {
                let _ = conn.send(&NewLocalPlayer);
            }

        }

        return false;

    }

    fn handle_player_joined(&mut self) {

        if let Some(conn) = PlayerJoined::subscribe("default") {

            while let Ok(msg) = conn.recv() {

                if msg.player_id == self.online_player_manager.player_id {
                    continue;
                }

                if self.online_player_manager.check_for_player(&msg.player_id) {
                    continue;
                }

                log!("just received new player info");

                self.make_new_ghost(&msg);

            }

            if self.input_manager.a.just_pressed() {
                let _ = conn.send(&NewOnlinePlayer::new());
            }

        }

    }

    fn make_new_ghost(&mut self, some_data : &HeardOnline) {

        log!("Trying to make a new player");
        
        self.new_entity_w_comp(&mut character_prefabs::new_online_player(some_data.player_id.clone()));

    }

}