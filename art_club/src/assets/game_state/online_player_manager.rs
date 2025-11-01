use turbo::*;

use std::collections::HashMap;

use crate::{GameState, turbecs, assets};

use turbecs::component_system::component_types::ComponentTypes;
use turbecs::component_system::component::ComponentData;

use assets::game_state;
use game_state::online_calls::channels::{NewLocalPlayer, HeardLocal, LocalJoined};
use game_state::online_calls::channels::{OnlinePlayerData, HeardOnline, PlayerJoined};
use game_state::online_calls::channels::{UpdatePlayer, ExtraData};

use crate::assets::components::player_components::player_enums::PlayerState;

use assets::prefabs::character_prefabs;

use turbecs::managers::input_system::input_basket::InputBasket;

#[turbo::serialize]
#[derive(PartialEq)]
pub struct OnlinePlayerManager {

    pub player_id : String,
    pub player_render_locat : (usize, usize),
    pub online_players : HashMap<String, usize>,
    pub should_join : bool,
    pub should_update : bool,
}
impl OnlinePlayerManager {
    
    pub fn new() -> Self {
        return Self {
            player_id : "".to_string(),
            player_render_locat : (0, 0),
            online_players : HashMap::new(),
            should_join : false,
            should_update : false,
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
            self.handle_update_player();

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

            if self.online_player_manager.should_join {

                if !self.check_for_local_data() {
                    return;
                }

                let local_player = self.entity_manager.entities[self.online_player_manager.player_render_locat.0].clone();

                if let ComponentData::PlayerRenderer(p_render_component) = &self.component_manager.components[self.online_player_manager.player_render_locat.1].component_data {
                    
                    let _ = conn.send(
                        &OnlinePlayerData::new_w_data(
                            local_player.transform.get_x(),
                            -local_player.transform.get_y(),
                            p_render_component.direction.clone(),
                            p_render_component.curr_state.clone(),
                            p_render_component.sprite.clone()
                        )
                    );

                }

                self.online_player_manager.should_join = false;

            }

        }

    }

    fn handle_update_player(&mut self) {

        if let Some(conn) = UpdatePlayer::subscribe("default") {

            while let Ok(msg) = conn.recv() {

                if msg.player_id == self.online_player_manager.player_id {
                    continue;
                }

                if self.online_player_manager.check_for_player(&msg.player_id) {
                    
                    let other_player = self.entity_manager.entities[*self.online_player_manager.online_players.get_key_value(&msg.player_id).unwrap().1].clone();

                    // first check if this entity is good

                    if !other_player.has_component(ComponentTypes::PlayerGhost, self) {
                        continue;
                    }

                    // then do all le magic

                    self.entity_manager.entities[other_player.locat].transform.set_x(msg.data.x);
                    self.entity_manager.entities[other_player.locat].transform.set_y(msg.data.y);

                    // add update visual logic

                    let render_comp = other_player.find_component_in_state(ComponentTypes::PlayerRenderer, self);

                    if !render_comp.0 {
                        continue;
                    }

                    if let ComponentData::PlayerRenderer(p_render_comp_data) = &mut self.component_manager.components[render_comp.1].component_data {

                        p_render_comp_data.direction = msg.data.curr_dir;
                        p_render_comp_data.update_animation(msg.data.curr_state);

                    }

                    // updates ghost component

                    let ghost_comp = other_player.find_component_in_state(ComponentTypes::PlayerGhost, self);

                    if !ghost_comp.0 {
                        continue;
                    }

                    if let ComponentData::PlayerGhost(p_ghost_comp_data) = &mut self.component_manager.components[ghost_comp.1].component_data {

                        match &msg.data.extra {

                            ExtraData::Movement(basket) => {
                                
                                p_ghost_comp_data.update_basket(basket.clone());

                            },
                            _default => {

                                p_ghost_comp_data.update_basket(InputBasket::new());

                            }

                        }

                    }

                }
                else {

                    log!("Update Player : didn't know player existed, adding now");

                    self.make_new_ghost(&msg);
                }

            }

            if self.online_player_manager.should_update {

                if !self.check_for_local_data() {
                    return;
                }

                let player_ent = self.entity_manager.entities[self.online_player_manager.player_render_locat.0].clone();
                let player_rend = self.component_manager.components[self.online_player_manager.player_render_locat.1].clone();

                if let ComponentData::PlayerRenderer(player_rend_data) = player_rend.component_data {

                    let mut data_to_send = OnlinePlayerData::new_w_data(
                            player_ent.transform.get_x(),
                            -player_ent.transform.get_y(),
                            player_rend_data.direction,
                            player_rend_data.curr_state,
                            player_rend_data.sprite
                        );

                    if data_to_send.curr_state == PlayerState::Walking{

                        data_to_send.extra = ExtraData::Movement(
                            InputBasket::new_w_state(self)
                        );

                    }

                    let _ = conn.send(
                        &data_to_send
                    );

                }

                self.online_player_manager.should_update = false;

            }

        }

    }

    fn check_for_local_data(&mut self) -> bool {

        if self.component_manager.components.len() == 0 {
            return false;
        }

        if self.component_manager.components[self.online_player_manager.player_render_locat.1].get_comp_type() == ComponentTypes::PlayerRenderer {
            
            if self.component_manager.components[self.online_player_manager.player_render_locat.1].active {
                return true;
            }

        }

        let local_player = self.find_w_component(ComponentTypes::PlayerController);

        if !local_player.0 {
            return false;
        }

        self.online_player_manager.player_render_locat.0 = local_player.1;

        let local_player_ent = self.entity_manager.entities[local_player.1].clone();

        let local_render = local_player_ent.find_component_in_state(ComponentTypes::PlayerRenderer, self);

        if !local_render.0 {
            return false;
        }

        self.online_player_manager.player_render_locat.1 = local_render.1;

        return true;

    }

    fn make_new_ghost(&mut self, some_data : &HeardOnline) {

        log!("Trying to make a new player");
        
        self.new_entity_w_comp(&mut character_prefabs::new_online_player(some_data.clone()));

    }

}