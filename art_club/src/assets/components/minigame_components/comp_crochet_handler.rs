// Core directories

use turbo::*;

use crate::{turbecs, GameState};

use turbecs::managers;
use turbecs::entity::Entity;

#[turbo::serialize]
#[derive(PartialEq)]
pub struct SongSelectData {
    looking_at : u32,
    delta_time : u32
}

impl SongSelectData {

    pub fn new() -> Self {
        return Self { looking_at: 0, delta_time: 0 };
    }

}

#[turbo::serialize]
#[derive(PartialEq)]
pub enum CrochetMinigameState {
    Start,
    SongSelect(SongSelectData),
    Game,
    Results
}

#[turbo::serialize]
#[derive(PartialEq)]
pub struct CrochetHandlerComponent {
    mini_state : CrochetMinigameState
}

impl CrochetHandlerComponent {
    pub fn new() -> Self {
        return Self{
            mini_state : CrochetMinigameState::Start
        };
    }
}

impl CrochetHandlerComponent {
    
    pub fn update(&mut self, ent : &mut Entity, state :&mut GameState) {

        match &self.mini_state {
            CrochetMinigameState::Start => {
                self.update_start(ent, state);
            },
            CrochetMinigameState::SongSelect(_) => {
                self.update_song_select(state);
            },
            _default => {}
        }

    }

    pub fn render(&self, state :&mut GameState) {

        rect!(
            x = screen().w() as f32 * -0.05,
            y = screen().h() as f32 * 0.1,
            w = screen().w() as f32 * 1.1,
            h = screen().h() as f32 * 0.8,
            color = 0xbf8943ff,
            border_size = 4,
            border_color = 0x9c6e33ff,
            fixed = true
        );

        // yarn

        sprite!(
            "smile",
            x = screen().w() as f32 * 0.1,
            y = screen().h() as f32 * 0.2,
            w = screen().w() as f32 * 0.3,
            h = screen().h() as f32 * 0.3,
            color = 0xaa1111ff,
            fixed = true
        );

        //scarf

        sprite!(
            "smile",
            x = screen().w() as f32 * 0.1,
            y = screen().h() as f32 * 0.7,
            w = screen().w() as f32 * 0.6,
            h = screen().h() as f32 * 0.3,
            color = 0xaa1111ff,
            fixed = true
        );

        // phone

        sprite!(
            "smile",
            x = screen().w() as f32 * 0.75,
            y = screen().h() as f32 * 0.55,
            w = screen().w() as f32 * 0.2,
            h = screen().h() as f32 * 0.3,
            color = 0x111111ff,
            fixed = true
        );

        match &self.mini_state {
            CrochetMinigameState::Start => {
                self.render_start();
            },
            CrochetMinigameState::SongSelect(select_info) => {
                self.render_song_select(select_info.clone());
            },
            _default => {}
        }

    }

}

// state specific update calls

impl CrochetHandlerComponent {

    pub fn update_start(&mut self, ent : &mut Entity, state : &mut GameState) {

        if state.input_manager.a.just_pressed() {
            
            log!("moving to song select!");
            self.mini_state = CrochetMinigameState::SongSelect(SongSelectData::new());
            return;

        }

        else if state.input_manager.b.just_pressed() {
            
            log!("ending minigame!");
            state.entity_manager.lifetime_data.new_destroy.push_back(ent.locat);
            state.minigame_manager.is_active = false;

        }

    }


    pub fn update_song_select(&mut self, state : &mut GameState) {

        if state.input_manager.b.just_pressed() {
            
            log!("moving to main menu!");
            self.mini_state = CrochetMinigameState::Start;

            return;

        }
        
    }

}

// state specific render calls

impl CrochetHandlerComponent {
    
    pub fn render_start(&self) {

        text_box!(
            "Crochet Beats",
            x = screen().w() as f32 * 0.1,
            y = screen().h() as f32 * 0.3,
            w = screen().w() as f32 * 0.8,
            h = screen().h() as f32 * 0.2,
            color = 0xffffffff,
            font = "large",
            align = "center",
            fixed = true
        );

        render_text_w_box(
            "Make scarves while bopping to some beats!",
            screen().w() as f32 * 0.15,
            screen().h() as f32 * 0.5,
            screen().w() as f32 * 0.7,
            screen().h() as f32 * 0.1,
            "medium"
        );

        render_text_w_box(
            "Press A to Start!",
            screen().w() as f32 * 0.2,
            screen().h() as f32 * 0.7,
            screen().w() as f32 * 0.6,
            screen().h() as f32 * 0.05,
            "medium"
        );

        render_text_w_box(
            "Press B to Cancel!",
            screen().w() as f32 * 0.2,
            screen().h() as f32 * 0.8,
            screen().w() as f32 * 0.6,
            screen().h() as f32 * 0.05,
            "medium"
        );
        
    }

    pub fn render_song_select(&self, some_data : SongSelectData) {



    }

}

fn render_text_w_box(some_str : &str, some_x : f32, some_y : f32, some_w : f32, some_h : f32, font : &str) {

    rect!(
        x = some_x,
        y = some_y - some_h * 0.2,
        w = some_w,
        h = some_h * 1.2,
        color = 0x000000aa,
        border_radius = 2,
        fixed = true
    );

    text_box!(
        some_str,
        x = some_x,
        y = some_y,
        w = some_w,
        h = some_h,
        color = 0xffffffff,
        font = font,
        align = "center",
        fixed = true
    );

}