use turbo::*;

use crate::assets;

use assets::components::player_components;
use player_components::player_enums;
use player_enums::{PlayerDirection, PlayerState};

#[turbo::serialize]
pub struct NewOnlinePlayer{
    pub x : i32,
    pub y : i32,
    pub curr_dir : PlayerDirection,
    pub curr_state : PlayerState
}

impl NewOnlinePlayer {

    pub fn new() -> Self {

        return Self {
            x: 0,
            y: 0,
            curr_dir: PlayerDirection::Down,
            curr_state: PlayerState::Idle
        }

    }

}

#[turbo::serialize]
pub struct HeardOnline;

#[turbo::serialize]
pub struct LocalJoined {
    pub x : i32,
    pub y : i32,
    pub curr_dir : PlayerDirection,
    pub curr_state : PlayerState
}

#[turbo::os::channel(program = "artClubManager", name = "main")]
pub struct PlayerJoined;
impl ChannelHandler for PlayerJoined { 
    type Recv = NewOnlinePlayer; // incoming from client
    type Send = HeardOnline; // outgoing to client
    fn new() -> Self { 
        Self
    } 
    fn on_data(&mut self, user_id: &str, data: Self::Recv) -> Result<(), std::io::Error> { 
        log!("Got {:?} from {:?}", data, user_id); 
        Self::send(user_id, HeardOnline) 
    } 
}