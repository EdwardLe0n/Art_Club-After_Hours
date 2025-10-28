use turbo::*;

#[turbo::serialize]
pub struct OnlineJoined;

#[turbo::serialize]
pub struct LocalJoined;

#[turbo::os::channel(program = "artClubManager", name = "playerJoined")]
pub struct PlayerJoined;
impl ChannelHandler for PlayerJoined { 
    type Recv = OnlineJoined; // incoming from client
    type Send = LocalJoined; // outgoing to client
    fn new() -> Self { 
        Self
    } 
    fn on_data(&mut self, user_id: &str, data: Self::Recv) -> Result<(), std::io::Error> { 
        log!("Got {:?} from {:?}", data, user_id); 
        Self::send(user_id, LocalJoined) 
    } 
}