use std::collections::VecDeque;

use turbo::*;

use crate::turbecs;

use turbecs::{entity::Entity, component_system};
use component_system::component::{Component, ComponentData};

// Standard Components

// User defined components
use crate::assets;

use assets::components::{misc_components, minigame_components};

use minigame_components::comp_minigame_handler::MinigameHandlerComponent;
use minigame_components::comp_crochet_handler::CrochetHandlerComponent;

pub fn new_temp () -> (Entity, VecDeque<Component>) {

    let mut ent = Entity::new_base("temp".to_string());
    let mut ent_queue = VecDeque::new();

    return (ent, ent_queue);

}

pub fn new_minigame_time () -> (Entity, VecDeque<Component>) {

    let mut ent = Entity::new_base("minigame handler".to_string());

    ent.set_layer(10);

    let mut ent_queue = VecDeque::new();

    ent_queue.push_back(
        Component::new(
            ComponentData::MinigameHandler(
                MinigameHandlerComponent::new()
            )
        )
    );

    return (ent, ent_queue);

}

pub fn new_crochet_minigame () -> (Entity, VecDeque<Component>) {

    let mut ent = Entity::new_base("crochet minigame handler".to_string());

    ent.set_layer(10);

    let mut ent_queue = VecDeque::new();

    ent_queue.push_back(
        Component::new(
            ComponentData::CrochetHandler(
                CrochetHandlerComponent::new()
            )
        )
    );

    return (ent, ent_queue);

}