// Core directories

use std::collections::VecDeque;

use turbo::*;

use crate::{turbecs, GameState};

use turbecs::managers;
use turbecs::entity::Entity;

const CURRENT_SONGS : u32 = 2;

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
    pub since_last_beat : f32,
    pub sec_per_beat : f32,
    pub song_state : SongGameState,
    pub total_score : f32,
    pub inputs : i32,
    pub correct_inputs : i32,

}

impl SongGameData {

    pub fn new(some_u32 : u32) -> Self{

        let mut some_song = "glorp";
        let mut some_spb = 120.0;

        match some_u32 {
            0 => {
                some_song = "glorp";
                some_spb = 120.0
            },
            1 => {
                some_song = "what_was_i_meowed_for";
                some_spb = 130.0
            },
            _default => {}
        }

        some_spb = 60.0 / some_spb;

        return Self { 
            song: some_song.to_string(),
            delta_time: 0.0,
            since_last_beat : 0.0,
            sec_per_beat : some_spb,
            song_state: SongGameState::Before,
            total_score : 0.0,
            inputs : 0,
            correct_inputs : 0
        };

    }

}

#[turbo::serialize]
#[derive(PartialEq)]
pub struct SongResultData {

    pub song : String,
    pub total_score : f32,
    pub inputs : i32,
    pub correct_inputs : i32

}

impl SongResultData {
    
    pub fn new(some_song : String, some_score : f32, some_inputs : i32, some_correct_inputs : i32) -> Self {
        return Self { 
            song: some_song,
            total_score: some_score,
            inputs : some_inputs,
            correct_inputs : some_correct_inputs
        };
    }

}

#[turbo::serialize]
#[derive(PartialEq)]
pub enum CrochetMinigameState {
    Start,
    SongSelect(SongSelectData),
    Game(SongGameData),
    Results(SongResultData)
}

#[turbo::serialize]
#[derive(PartialEq)]
pub enum NextButton {
    A,
    B,
    X,
    Y
}

impl NextButton {

    pub fn to_string(&self) -> String {

        match self {
            NextButton::A => {return "A".to_string();}
            NextButton::B => {return "B".to_string();}
            NextButton::X => {return "X".to_string();}
            NextButton::Y => {return "Y".to_string();}
        }

    }

}

#[turbo::serialize]
#[derive(PartialEq)]
pub struct CrochetHandlerComponent {
    mini_state : CrochetMinigameState,
    next_inputs : VecDeque<NextButton>
}

impl CrochetHandlerComponent {
    pub fn new() -> Self {
        return Self{
            mini_state : CrochetMinigameState::Start,
            next_inputs : VecDeque::with_capacity(10)
        };
    }
}

impl CrochetHandlerComponent {
    
    pub fn update(&mut self, ent : &mut Entity, state :&mut GameState) {

        if state.input_manager.select.just_pressed() {

            state.entity_manager.lifetime_data.new_destroy.push_back(ent.locat);

        }

        if self.next_inputs.is_empty() {
            self.init_inputs();
        }

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
            CrochetMinigameState::Results(_) => {
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
            "crochet/yarn",
            x = screen().w() as f32 * 0.1,
            y = screen().h() as f32 * 0.2,
            w = 70,
            h = 70,
            color = 0xffffffff,
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

        // log!("x : {} and y : {}", screen().w() as f32 * 0.2, screen().h() as f32 * 0.3);

        sprite!(
            "crochet/phone",
            x = screen().w() as f32 * 0.75,
            y = screen().h() as f32 * 0.55,
            w = 50,
            h = 75,
            color = 0xffffffff,
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
            CrochetMinigameState::Results(game_data) => {
                self.render_results(game_data);
            },
            _default => {}
        }

    }

    pub fn destroy(&mut self, state :&mut GameState) {

        state.minigame_manager.is_active = false;

    }

}

impl CrochetHandlerComponent {
    
    pub fn init_inputs(&mut self) {

        self.next_inputs.push_back(NextButton::A);
        self.next_inputs.push_back(NextButton::B);
        self.next_inputs.push_back(NextButton::X);
        self.next_inputs.push_back(NextButton::Y);
        self.next_inputs.push_back(NextButton::A);
        self.next_inputs.push_back(NextButton::B);
        self.next_inputs.push_back(NextButton::X);
        self.next_inputs.push_back(NextButton::Y);
        self.next_inputs.push_back(NextButton::A);
        self.next_inputs.push_back(NextButton::B);

    }

    pub fn add_new_input(&mut self) {

        match self.next_inputs[self.next_inputs.len() - 1] {
            NextButton::A => {self.next_inputs.push_back(NextButton::B);}
            NextButton::B => {self.next_inputs.push_back(NextButton::X);}
            NextButton::X => {self.next_inputs.push_back(NextButton::Y);}
            NextButton::Y => {self.next_inputs.push_back(NextButton::A);}
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

                    // if select.looking_at != 0 {
                    //     log!("song not yet implemented!");
                    //     return;
                    // }

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

        let mut did_an_input = false;
        let mut correct_input = false;

        let mut diff = 0.0;
        let mut is_early = false;

        let mut percent_correct = 0.0;

        match &mut self.mini_state {
            CrochetMinigameState::Game(game_data) => {

                // Handles time updates

                game_data.delta_time += state.time_manager.delta;
                game_data.since_last_beat += state.time_manager.delta;

                if game_data.since_last_beat >= game_data.sec_per_beat {
                    game_data.since_last_beat -= game_data.sec_per_beat;
                }

                diff = game_data.since_last_beat;

                if diff > game_data.sec_per_beat / 2.0 {
                    is_early = true;
                    diff = game_data.sec_per_beat - diff;
                }

                percent_correct = (diff / game_data.sec_per_beat) * 200.0;

                // Input handling section

                if state.input_manager.a.just_pressed() {

                    did_an_input = true;

                    if self.next_inputs.front().unwrap_or(&NextButton::A) == &NextButton::A {

                        correct_input = true;

                    }

                }

                if state.input_manager.b.just_pressed() {

                    did_an_input = true;

                    if self.next_inputs.front().unwrap_or(&NextButton::A) == &NextButton::B {

                        correct_input = true;

                    }

                }

                if state.input_manager.x.just_pressed() {

                    did_an_input = true;

                    if self.next_inputs.front().unwrap_or(&NextButton::A) == &NextButton::X {

                        correct_input = true;

                    }

                }

                if state.input_manager.y.just_pressed() {

                    did_an_input = true;

                    if self.next_inputs.front().unwrap_or(&NextButton::A) == &NextButton::Y {

                        correct_input = true;

                    }

                }

                // Updates the state when needed

                match game_data.song_state {
                    
                    SongGameState::Before => {

                        did_an_input = false;

                        if game_data.delta_time >= 2.0 {

                            game_data.delta_time = 0.0;
                            game_data.song_state = SongGameState::Playing;
                            game_data.since_last_beat = 0.0;

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

                        // Updates with the given results

                        if did_an_input {
                            game_data.inputs += 1;
                        }

                        if correct_input {
                            game_data.correct_inputs += 1;
                            game_data.total_score += 100.0 - percent_correct;
                        }

                    }

                    SongGameState::After => {

                        did_an_input = false;

                        if game_data.delta_time >= 2.0 {

                            game_data.delta_time = 0.0;

                            log!("total score = {}", game_data.total_score as i32);

                            self.mini_state = CrochetMinigameState::Results(
                                SongResultData::new(
                                    game_data.song.clone(),
                                    game_data.total_score.clone(),
                                    game_data.inputs.clone(),
                                    game_data.correct_inputs.clone()
                                )
                            );

                            return;

                        }
                        
                    }

                }
                
            }
            _default => {}
        }

        if !did_an_input {
            return;
        }

        // Sanity

        log!("Difference is {}", diff);

        if is_early {
            log!("early ");
        }

        log!("Percentile = {}", percent_correct);
        log!("Score is {}", 100.0 - percent_correct);

        // Correct handler!

        if !correct_input {
            log!("Incorrect!");
        }
        else {
            self.next_inputs.pop_front();
            self.add_new_input();
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

        sprite!(
            "crochet/lockScreen",
            x = screen().w() as f32 * 0.75,
            y = screen().h() as f32 * 0.55,
            w = 50,
            h = 75,
            color = 0xffffffff,
            fixed = true
        );

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

        sprite!(
            "crochet/sealify",
            x = screen().w() as f32 * 0.75,
            y = screen().h() as f32 * 0.55,
            w = 50,
            h = 75,
            color = 0xffffffff,
            fixed = true
        );
        
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
            "Glorps_call.mp3",
            screen().w() as f32 * 0.2,
            screen().h() as f32 * 0.35,
            screen().w() as f32 * 0.6,
            screen().h() as f32 * 0.05,
            "medium"
        );

        render_text_w_box(
            "what_was_i_meowed_for.mp3",
            screen().w() as f32 * 0.2,
            screen().h() as f32 * 0.45,
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

        let bar_w = screen().w() as f32 * 0.01;
        let bar_h = screen().h() as f32 * 0.1;

        rect!(
            x = screen().w() as f32 * 0.075,
            y = screen().h() as f32 * 0.2,
            w = bar_w,
            h = bar_h,
            color = 0xffffffff,
            fixed = true
        );

        let bump = 1.0 + some_data.sec_per_beat - some_data.since_last_beat;
        let mut hov_bar_w = (bar_w * bump * 1.1) as i32;
        let mut hov_bar_h = (bar_h * bump) as i32;

        if hov_bar_w % 2 == 1 {
            hov_bar_w += 1;
        }

        if hov_bar_h % 2 == 1{
            hov_bar_h += 1;
        }

        rect!(
            x = screen().w() as f32 * 0.075 - (hov_bar_w as f32 - bar_w) / 2.0,
            y = screen().h() as f32 * 0.2 - (hov_bar_h as f32 - bar_h) / 2.0,
            w = hov_bar_w,
            h = hov_bar_h,
            color = 0xffffffbb,
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
            },
            SongGameState::After => {
                render_text_w_box(
                    "done!~",
                    screen().w() as f32 * 0.2,
                    screen().h() as f32 * 0.5,
                    screen().w() as f32 * 0.6,
                    screen().h() as f32 * 0.05,
                    "large"
                );
            }
            _default => {}
        }

        let wh = 15;

        for i in 0..self.next_inputs.len() {
            
            let letter = self.next_inputs[i].to_string();

            ellipse!(
                x = screen().w() as f32 * 0.075 + (i * wh * 2) as f32 + wh as f32,
                y = screen().h() as f32 * 0.2,
                w = wh,
                h = wh,
                color = 0xaa0000ff,
                fixed = true
            );

            text_box!(
                &letter,
                font = "large",
                x = screen().w() as f32 * 0.075 + (i * wh * 2) as f32 + wh as f32,
                y = screen().h() as f32 * 0.2 + (wh / 4) as f32,
                w = wh,
                h = wh,
                align = "center",
                fixed = true
            )

        }

    }

    fn render_results(&self, some_data : &SongResultData) {

        render_text_w_box(
            "good job!~~~",
            screen().w() as f32 * 0.2,
            screen().h() as f32 * 0.3,
            screen().w() as f32 * 0.6,
            screen().h() as f32 * 0.05,
            "large"
        );

        let mut some_string = "Results:\n\n".to_string();

        some_string.push_str("Song played : ");
        some_string.push_str(&some_data.song);

        some_string.push_str("\n");
        
        some_string.push_str("Got a total score of : ");
        some_string.push_str(&(some_data.total_score as i32).to_string());

        some_string.push_str("\n");

        some_string.push_str("# of inputs : ");
        some_string.push_str(&(some_data.inputs).to_string());

        some_string.push_str("\n");

        some_string.push_str("# of correct inputs : ");
        some_string.push_str(&(some_data.correct_inputs).to_string());

        render_text_w_box(
            &some_string,
            screen().w() as f32 * 0.15,
            screen().h() as f32 * 0.4,
            screen().w() as f32 * 0.7,
            screen().h() as f32 * 0.25,
            "medium"
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