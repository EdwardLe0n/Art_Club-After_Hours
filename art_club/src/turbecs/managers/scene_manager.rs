use turbo::*;

use std::collections::VecDeque;

use crate::{turbecs};
use turbecs::entity::Entity;
use turbecs::component_system::component::Component;

use crate::assets;

use assets::prefabs::{general_prefabs, character_prefabs};

#[turbo::serialize]
#[derive(Copy, PartialEq)]
pub struct SceneManager {
    pub active_scene : Scenes,
    pub is_loaded : bool
}

#[turbo::serialize]
#[derive(Copy, PartialEq)]
pub enum Scenes {
    DevCards,
    InputSelection,
    Title,
    Lobby,
    Misc,
}

impl SceneManager {

    pub fn new() -> Self {

        return Self{active_scene : Scenes::DevCards, is_loaded : false};

    }

    pub fn load_scene(&mut self, _some_scene : Scenes) {

        self.active_scene = _some_scene;
        self.is_loaded = false;

    }
}

pub fn make_scene (some_scene : Scenes) ->  VecDeque<(Entity, VecDeque<Component>)>{

    // Will always reset the camera location when make scene is called

    camera::set_xy(0, 0);

    // Then the scene data will be pulled

    match some_scene {
        Scenes::DevCards => {return make_dev_cards_scene();},
        Scenes::InputSelection => {return make_input_scene();},
        Scenes::Title => {return make_title_scene()},
        Scenes::Lobby => {return make_lobby_scene()},
        Scenes::Misc => {return make_misc_scene()},
        _default => {
            return VecDeque::new();
        }
    }

}

pub fn make_dev_cards_scene() -> VecDeque<(Entity, VecDeque<Component>)> {

    let mut ent_vec = VecDeque::new();

    ent_vec.push_back(general_prefabs::new_background());

    ent_vec.push_back(general_prefabs::new_turbecs_card());

    return ent_vec;

}

pub fn make_input_scene() -> VecDeque<(Entity, VecDeque<Component>)> {

    let mut ent_vec = VecDeque::new();

    ent_vec.push_back(general_prefabs::new_background());

    ent_vec.push_back(general_prefabs::new_input_notice());
    ent_vec.push_back(general_prefabs::new_mouse_notice());
    ent_vec.push_back(general_prefabs::new_mobile_notice());

    ent_vec.push_back(general_prefabs::new_from_input_to_title());

    return ent_vec;

}

pub fn make_title_scene() -> VecDeque<(Entity, VecDeque<Component>)> {

    let mut ent_vec = VecDeque::new();

    ent_vec.push_back(general_prefabs::new_title());

    ent_vec.push_back(general_prefabs::new_to_lobby());

    return ent_vec;

}


pub fn make_lobby_scene() -> VecDeque<(Entity, VecDeque<Component>)> {

    let mut ent_vec = VecDeque::new();

    ent_vec.push_back(general_prefabs::new_to_title());

    ent_vec.push_back(character_prefabs::new_local_player());

    return ent_vec;

}

pub fn make_misc_scene() -> VecDeque<(Entity, VecDeque<Component>)> {

    let mut ent_vec = VecDeque::new();

    ent_vec.push_back(general_prefabs::new_to_title());

    ent_vec.push_back(character_prefabs::new_local_player());

    return ent_vec;

}