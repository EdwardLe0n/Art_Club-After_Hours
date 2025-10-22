// Core directories

use turbo::*;

use crate::{turbecs, GameState};

use turbecs::entity::Entity;

#[turbo::serialize]
#[derive(PartialEq)]
pub struct PlayerControllerComponent {
    player_speed : f32
}

impl PlayerControllerComponent {

    pub fn new() -> Self {
        return Self {player_speed : 100.0};
    }

}

impl PlayerControllerComponent {
    
    pub fn update (&mut self, ent : &mut Entity, state : &mut GameState) {

        self.handle_movement(ent, state);

    }

}

impl PlayerControllerComponent {

    pub fn handle_movement (&mut self, ent : &mut Entity, state : &mut GameState) {

        let p1_gamepad = gamepad::get(0);

        let mut moov_vec = (0.0, 0.0);

        // Handles vertical movement

        if p1_gamepad.up.pressed() {
            moov_vec.1 += self.player_speed;
        }

        if p1_gamepad.down.pressed() {
            moov_vec.1 -= self.player_speed;
        }

        // Handles horizontal movement

        if p1_gamepad.right.pressed() {
            moov_vec.0 += self.player_speed;
        }

        if p1_gamepad.left.pressed() {
            moov_vec.0 -= self.player_speed;
        }

        // moves the player based on the given inputs

        ent.transform.nudge_x((moov_vec.0 * state.time_manager.delta) as i32);
        ent.transform.nudge_y((moov_vec.1 * state.time_manager.delta) as i32);
        
        self.handle_camera(ent);

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