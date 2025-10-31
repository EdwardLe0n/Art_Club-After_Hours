use turbo::*;

use crate::assets;

use assets::components::player_components;
use player_components::player_enums;
use player_enums::{PlayerDirection, PlayerState, PlayerSprite};

#[turbo::serialize]
pub struct OnlinePlayerData{
    pub x : i32,
    pub y : i32,
    pub curr_dir : PlayerDirection,
    pub curr_state : PlayerState,
    pub curr_sprite : PlayerSprite
}

impl OnlinePlayerData {

    pub fn new() -> Self {

        return Self {
            x: 0,
            y: 0,
            curr_dir: PlayerDirection::Down,
            curr_state: PlayerState::Idle,
            curr_sprite : PlayerSprite::Test
        }

    }

    pub fn new_w_data(some_x : i32, some_y : i32, some_dir : PlayerDirection, some_state : PlayerState, some_sprite : PlayerSprite) -> Self {

        let mut to_return = Self::new();

        to_return.x = some_x;
        to_return.y = some_y;
        to_return.curr_dir = some_dir;
        to_return.curr_state = some_state;
        to_return.curr_sprite = some_sprite;

        return to_return;

    }

}

#[turbo::serialize]
pub struct HeardOnline {

    pub player_id : String,
    pub data : OnlinePlayerData,

}

impl HeardOnline {
    
    pub fn new(id : String, some_data : OnlinePlayerData) -> Self {

        return Self {
            player_id : id,
            data : some_data
        };
        
    }

}

#[turbo::os::channel(program = "artClubManager", name = "playerJoined")]
pub struct PlayerJoined;
impl ChannelHandler for PlayerJoined { 
    type Recv = OnlinePlayerData; // incoming from client
    type Send = HeardOnline; // outgoing to client
    fn new() -> Self { 
        Self
    } 
    fn on_data(&mut self, user_id: &str, data: Self::Recv) -> Result<(), std::io::Error> {

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