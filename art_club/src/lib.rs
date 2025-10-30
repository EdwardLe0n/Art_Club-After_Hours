
// Core Turbo functionality
use turbo::*;

// Links the driver file to the main lib file
mod ecs_gamestate;

// Initial TurbECS import
mod turbecs;

// Manager imports
use turbecs::{managers};

use managers::entity_manager::EntityManager;
use managers::component_manager::ComponentManager;
use managers::scene_manager::SceneManager;

use managers::time_manager::TimeManager;
use managers::input_manager::InputManager;

// Community work/manager imports
use managers::particlemanager::ParticleManager;

// Game specific elements that need to be in the GameState

mod assets;
use assets::game_state::{run_data::RunData};

use assets::game_state;
use game_state::online_player_manager::OnlinePlayerManager;
use game_state::online_calls::channels::{NewOnlinePlayer, HeardOnline, PlayerJoined};

#[turbo::game]
struct GameState {
    
    // Core managers for turbecs

    pub scene_manager : SceneManager,
    pub entity_manager : EntityManager,
    pub component_manager : ComponentManager,
    pub render_manager : Vec<Vec<usize>>,

    pub time_manager : TimeManager,
    pub input_manager : InputManager,

    // Community integrated work/managers

    pub particle_manager : ParticleManager,

    // Project specific elements

    pub run_data : RunData,
    pub can_interact : bool

}

impl GameState {
    fn new() -> Self {

        camera::set_xy(0, 0);

        Self {
            
            // Core managers for turbecs

            scene_manager : SceneManager::new(),
            entity_manager : EntityManager::new(),
            component_manager : ComponentManager::new(),
            render_manager : Vec::with_capacity(10),

            time_manager : TimeManager::new(),
            input_manager : InputManager::new(),
            
            // Community integrated work/managers

            particle_manager : ParticleManager::new(),

            // Project specific elements

            run_data : RunData::new(),
            can_interact : true
        }
    
    }

    fn update(&mut self) {

        // Will move later
        
        if let Some(conn) = PlayerJoined::subscribe("default") {

            while let Ok(msg) = conn.recv() {
                log!("Received message from server!");
            }
            if self.input_manager.a.pressed() {
                // Send a Ping message using the `ChannelConnection::send` method
                let _ = conn.send(&NewOnlinePlayer::new());
                log!("Sent ping to the server!");
            }

        }

        // Updates the time every frame

        self.time_manager.update();

        // Gathers the inputs every frame

        self.input_manager.update();

        // Checks the scene state before continuing

        self.check_scene_state();

        // From here TurbECS will run it's lifetime functions!

        self.run_lifetime();

        // renders the 'time' since last frame

        self.time_manager.render();

        // Renders the controls to the player

        self.input_manager.render();

    }
}