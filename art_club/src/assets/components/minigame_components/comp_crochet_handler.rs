// Core directories

use turbo::*;

use crate::{turbecs, GameState};

use turbecs::managers;
use turbecs::entity::Entity;

const CURRENT_SONGS : u32 = 3;

#[turbo::serialize]
#[derive(PartialEq)]
pub struct SongSelectData {
    pub looking_at : u32,
    pub delta_time : f32
}

impl SongSelectData {

    pub fn new() -> Self {
        return Self { looking_at: 0, delta_time: 0.0 };
    }

}

#[turbo::serialize]
#[derive(PartialEq)]
pub enum SongGameState {
    Before,
    Playing,
    After
}

#[turbo::serialize]
#[derive(PartialEq)]
pub struct SongGameData {

    pub song : String,
    pub delta_time : f32,
    pub song_state : SongGameState

}

impl SongGameData {

    pub fn new(some_u32 : u32) -> Self{

        let mut some_song = "glorp";

        match some_u32 {
            0 => {some_song = "glorp";}
            _default => {}
        }

        return Self { 
            song: some_song.to_string(),
            delta_time: 0.0,
            song_state: SongGameState::Before 
        };

    }

}

#[turbo::serialize]
#[derive(PartialEq)]
pub enum CrochetMinigameState {
    Start,
    SongSelect(SongSelectData),
    Game(SongGameData),
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
            CrochetMinigameState::Game(_) => {
                self.update_game(state);
            },
            CrochetMinigameState::Results => {
                self.update_results(state);
            }
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
                self.render_song_select(select_info);
            },
            CrochetMinigameState::Game(game_data) => {
                self.render_game(game_data);
            },
            CrochetMinigameState::Results => {
                self.render_results();
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

        match &mut self.mini_state {
            CrochetMinigameState::SongSelect(select) => {

                if state.input_manager.a.just_pressed() {
                    log!("trying to play song");

                    if select.looking_at != 0 {
                        log!("song not yet implemented!");
                        return;
                    }

                    self.mini_state = CrochetMinigameState::Game(SongGameData::new(select.looking_at));

                    return;

                }

                select.delta_time += state.time_manager.delta;

                if state.input_manager.up.just_pressed() {

                    if select.looking_at == 0 {
                        select.looking_at = CURRENT_SONGS - 1;
                    }
                    else {
                        select.looking_at -= 1;
                    }

                }

                if state.input_manager.down.just_pressed() {

                    select.looking_at += 1;

                    if select.looking_at == CURRENT_SONGS {
                        select.looking_at = 0;
                    }

                }

            }
            _default => {
                return;
            }
        }
        
    }

    pub fn update_game(&mut self, state : &mut GameState) {

        match &mut self.mini_state {
            CrochetMinigameState::Game(game_data) => {

                game_data.delta_time += state.time_manager.delta;

                match game_data.song_state {
                    
                    SongGameState::Before => {

                        if game_data.delta_time >= 2.0 {

                            game_data.delta_time = 0.0;
                            game_data.song_state = SongGameState::Playing;

                            audio::play(&game_data.song);

                        }

                    }

                    SongGameState::Playing => {
                        
                        if !audio::is_playing(&game_data.song) {
                            log!("song has ended");
                            game_data.delta_time = 0.0;
                            game_data.song_state = SongGameState::After;
                            return;
                        }

                    }

                    SongGameState::After => {

                        if game_data.delta_time >= 2.0 {

                            game_data.delta_time = 0.0;
                            self.mini_state = CrochetMinigameState::Results;
                            return;

                        }
                        
                    }

                }
                
            }
            _default => {}
        }
        
    }

    pub fn update_results(&mut self, state : &mut GameState) {

        if state.input_manager.b.just_pressed() {

            log!("going back to the song select");

            self.mini_state = CrochetMinigameState::SongSelect(SongSelectData::new());
            return;

        }

    }

}

// state specific render calls

impl CrochetHandlerComponent {
    
    fn render_start(&self) {

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

    fn render_song_select(&self, some_data : &SongSelectData) {
        
        // handles the selection rectangle

        rect!(
            x = screen().w() as f32 * 0.185,
            y = screen().h() as f32 * (0.3325 + (0.1 * some_data.looking_at as f32)),
            w = screen().w() as f32 * 0.625,
            h = screen().h() as f32 * 0.075,
            color = 0xffffff00,
            border_color = 0xffffffff,
            border_size = 2,
            border_radius = 2,
            fixed = true
        );

        // renders the rest of the screen

        render_text_w_box(
            "Choose a track!",
            screen().w() as f32 * 0.2,
            screen().h() as f32 * 0.2,
            screen().w() as f32 * 0.6,
            screen().h() as f32 * 0.05,
            "large"
        );
        
        render_text_w_box(
            "meow.mp3",
            screen().w() as f32 * 0.2,
            screen().h() as f32 * 0.35,
            screen().w() as f32 * 0.6,
            screen().h() as f32 * 0.05,
            "medium"
        );

        render_text_w_box(
            "Glorps_call.mp3",
            screen().w() as f32 * 0.2,
            screen().h() as f32 * 0.45,
            screen().w() as f32 * 0.6,
            screen().h() as f32 * 0.05,
            "medium"
        );

        render_text_w_box(
            "the_faz_call.mp3",
            screen().w() as f32 * 0.2,
            screen().h() as f32 * 0.55,
            screen().w() as f32 * 0.6,
            screen().h() as f32 * 0.05,
            "medium"
        );

        render_text_w_box(
            "press B to go back...",
            screen().w() as f32 * 0.2,
            screen().h() as f32 * 0.8,
            screen().w() as f32 * 0.6,
            screen().h() as f32 * 0.05,
            "medium"
        );

    }

    fn render_game(&self, some_data : &SongGameData) {

        rect!(
            x = screen().w() as f32 * -0.05,
            y = screen().h() as f32 * 0.2,
            w = screen().w() as f32 * 1.1,
            h = screen().h() as f32 * 0.1,
            color = 0x000000aa,
            fixed = true
        );

        rect!(
            x = screen().w() as f32 * 0.075,
            y = screen().h() as f32 * 0.2,
            w = screen().w() as f32 * 0.01,
            h = screen().h() as f32 * 0.1,
            color = 0xffffffff,
            fixed = true
        );

        match &some_data.song_state {
            SongGameState::Before => {
                render_text_w_box(
                "ready?",
                screen().w() as f32 * 0.2,
                screen().h() as f32 * 0.5,
                screen().w() as f32 * 0.6,
                screen().h() as f32 * 0.05,
                "large"
            );
            }
            _default => {}
        }

    }

    fn render_results(&self) {

        render_text_w_box(
            "good job!~~~",
            screen().w() as f32 * 0.2,
            screen().h() as f32 * 0.4,
            screen().w() as f32 * 0.6,
            screen().h() as f32 * 0.05,
            "large"
        );

        render_text_w_box(
            "press b to return to the song select!",
            screen().w() as f32 * 0.1,
            screen().h() as f32 * 0.8,
            screen().w() as f32 * 0.8,
            screen().h() as f32 * 0.05,
            "medium"
        );

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