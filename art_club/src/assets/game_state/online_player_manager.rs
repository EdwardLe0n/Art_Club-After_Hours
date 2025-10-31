use turbo::*;

use std::collections::HashMap;

use crate::{GameState, turbecs, assets};

use assets::game_state;
use game_state::online_calls::channels::{NewOnlinePlayer, HeardOnline, PlayerJoined};

#[turbo::serialize]
#[derive(PartialEq)]
pub struct OnlinePlayerManager {


    
}

impl GameState {

    pub fn handle_online(&mut self) {
        
        self.handle_player_joined();

    }

    fn handle_player_joined(&mut self) {

        if let Some(conn) = PlayerJoined::subscribe("default") {

            while let Ok(msg) = conn.recv() {
                log!("Received message from server!");



            }
            if self.input_manager.a.just_pressed() {
                // Send a Ping message using the `ChannelConnection::send` method
                let _ = conn.send(&NewOnlinePlayer::new());
            }

        }

    }

}