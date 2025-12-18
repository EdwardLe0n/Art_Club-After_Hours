// Core directories

use turbo::*;

use crate::{GameState, turbecs::{self, managers::scene_manager::Scenes}};

use turbecs::entity::Entity;
use turbecs::component_system;

use component_system::{component::ComponentData, component_types::ComponentTypes};

const ALIVE_TIME : f32 = 2.0;
const PEAK_TIME : f32 = 3.0;

#[turbo::serialize]
#[derive(PartialEq)]
pub enum LogoFade {

    TurbECS,
    None

}

#[turbo::serialize]
#[derive(PartialEq)]
pub struct LogoFadeComponent {
    pub fade_option : LogoFade,
    pub time : f32,
    pub da_tween : Tween<u32>,
    pub peak_time : bool,
}

impl LogoFadeComponent {
    pub fn new(some_option : LogoFade) -> Self {
        return Self{
            fade_option : some_option,
            time : 0.0, 
            da_tween : Tween::new(0x000000ff)
                .set(0x00000000)
                .ease(Easing::EaseInOutQuad)
                .duration(280),
            peak_time : true
        };
    }
}

impl LogoFadeComponent {

    pub fn update(&mut self, ent : &mut Entity, state : &mut GameState) {

        self.time += state.time_manager.delta;

        if self.peak_time {

            if self.time >= PEAK_TIME {

                self.time = 0.0;
                self.peak_time = false;

                log!("left peak time");

            }

            return;

        }

        // All adjustment stuff

        // setting duration of the tween to the time between

        self.da_tween.elapsed = (((self.time / ALIVE_TIME) * 255.0) as usize);

        // sanity
        // log!("time {} vs elapsed {}", self.time, self.da_tween.elapsed);

        let t_box_locat = ent.find_component_in_state(ComponentTypes::TextBox, state);

        if t_box_locat.0 {

            if let ComponentData::TextBox(some_t_box_comp) = &mut state.component_manager.components[t_box_locat.1].component_data {

                some_t_box_comp.color &= 0xffffff00;
                
                some_t_box_comp.color += self.da_tween.get() as u32;

            }

        }

        let sprite_locat = ent.find_component_in_state(ComponentTypes::Sprite, state);

        if sprite_locat.0 {

            if let ComponentData::Sprite(sprite_comp) = &mut state.component_manager.components[sprite_locat.1].component_data {

                sprite_comp.color &= 0xffffff00;
                sprite_comp.color += self.da_tween.get() as u32;

            }

        }

        if self.time >= ALIVE_TIME {
            log!("will now explode");
            state.entity_manager.lifetime_data.new_destroy.push_back(ent.locat);

            match &self.fade_option {
                
                LogoFade::TurbECS => {
                    state.scene_manager.load_scene(Scenes::InputSelection);
                }
                _default => {}

            }

        }

    }

}