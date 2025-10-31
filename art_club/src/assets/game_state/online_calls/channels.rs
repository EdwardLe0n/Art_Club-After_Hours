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
pub struct HeardOnline {

    pub player_id : String,
    pub data : NewOnlinePlayer,

}

impl HeardOnline {
    
    pub fn new(id : String, some_data : NewOnlinePlayer) -> Self {

        return Self {
            player_id : id,
            data : some_data
        };
        
    }

}

#[turbo::os::channel(program = "artClubManager", name = "playerJoined")]
pub struct PlayerJoined;
impl ChannelHandler for PlayerJoined { 
    type Recv = NewOnlinePlayer; // incoming from client
    type Send = HeardOnline; // outgoing to client
    fn new() -> Self { 
        Self
    } 
    fn on_data(&mut self, user_id: &str, data: Self::Recv) -> Result<(), std::io::Error> {
        // Self::send(user_id, HeardOnline::new(user_id.to_string(), data))

        Self::broadcast(HeardOnline::new(user_id.to_string(), data))

    }
}

#[turbo::serialize]
pub struct NewLocalPlayer;

#[turbo::serialize]
pub struct HeardLocal {

    pub player_id : String

}

#[turbo::os::channel(program = "artClubManager", name = "localJoined")]
pub struct LocalJoined;
impl ChannelHandler for LocalJoined { 
    type Recv = NewLocalPlayer; // incoming from client
    type Send = HeardLocal; // outgoing to client
    fn new() -> Self { 
        Self
    } 
    fn on_data(&mut self, user_id: &str, data: Self::Recv) -> Result<(), std::io::Error> {
        Self::send(user_id, HeardLocal{player_id : user_id.to_string()})
    }
}