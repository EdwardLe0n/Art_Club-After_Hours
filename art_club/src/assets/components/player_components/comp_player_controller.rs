// Core directories

use turbo::*;

use crate::{turbecs, GameState, assets};

use turbecs::entity::Entity;

use turbecs::component_system;
use component_system::component_types::ComponentTypes;
use component_system::component::ComponentData;

use assets::components::player_components;
use player_components::comp_player_renderer::PlayerRendererComponent;
use player_components::player_enums::{PlayerState};

#[turbo::serialize]
#[derive(PartialEq)]
pub struct PlayerControllerComponent {
    player_speed : f32,
    past_movement : (f32, f32)
}

impl PlayerControllerComponent {

    pub fn new() -> Self {
        return Self {
            player_speed : 100.0,
            past_movement : (0.0, 0.0)
        };
    }

}

impl PlayerControllerComponent {
    
    pub fn update (&mut self, ent : &mut Entity, state : &mut GameState) {

        self.handle_movement(ent, state);

    }

}

impl PlayerControllerComponent {

    pub fn handle_movement (&mut self, ent : &mut Entity, state : &mut GameState) {

        let mut moov_vec = (0.0, 0.0);

        // Handles vertical movement

        if state.input_manager.up.pressed() {
            moov_vec.1 += self.player_speed;
        }

        if state.input_manager.down.pressed() {
            moov_vec.1 -= self.player_speed;
        }

        // Handles horizontal movement

        if state.input_manager.right.pressed() {
            moov_vec.0 += self.player_speed;
        }

        if state.input_manager.left.pressed() {
            moov_vec.0 -= self.player_speed;
        }

        // moves the player based on the given inputs

        ent.transform.nudge_x((moov_vec.0 * state.time_manager.delta) as i32);
        ent.transform.nudge_y((moov_vec.1 * state.time_manager.delta) as i32);
        
        self.handle_visuals(moov_vec, ent, state);
        self.handle_camera(ent);

        self.past_movement = moov_vec;

    }

    fn handle_visuals(&mut self, moov : (f32, f32), ent : &mut Entity, state : &mut GameState) {

        let dud = (0.0, 0.0);
        let mut state_to_change_to = PlayerState::Idle;

        // If we haven't moved this frame
        if moov == dud {

            // If we haven't moved last frame either, don't update the state
            if self.past_movement == dud {
                return;
            }

        }
        else {

            // If we weren't moving last frame, then don't udpate the animation (bad implementation)
            if self.past_movement != dud {
                return;
            }

            state_to_change_to = PlayerState::Walking;

        }

        // update element here

        let renderer_locat = ent.find_component_in_state(ComponentTypes::PlayerRenderer, state);

        if !renderer_locat.0 {
            return;
        }

        if let ComponentData::PlayerRenderer(pr_component) = &mut state.component_manager.components[renderer_locat.1].component_data {
            
            pr_component.update_animation(state_to_change_to);
            
        }

    }

    fn handle_camera(&mut self, ent : &mut Entity) {

        /*
        * Frame by frame camera movement
        */

        // creates a bounding box of sorts
        let cam_box = 0.5;

        // if checks if the player is within the box
        // if they are, then the camera won't move
        if (
            (camera::x() - (ent.transform.get_x() as f32)).abs() < cam_box
            ) && (
            (camera::y() - (ent.transform.get_y() as f32)).abs() < cam_box
        ) {
            return;
        }

        // Weight vars for calculation (sum must equal up to 1.0)
        let cam_weight = 0.95;
        let player_weight = 1.0 - cam_weight;

        camera::set_xy(
            (camera::x() * cam_weight + (ent.transform.get_x() as f32) * player_weight),
            (camera::y() * cam_weight + (ent.transform.get_y() as f32) * player_weight)
        );

    }

}