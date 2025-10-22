// 
use std::collections::VecDeque;
use crate::{turbecs, assets};

// necessary turbecs imports
use turbecs::entity::Entity;
use turbecs::component_system::{component, components};
use component::{Component, ComponentData};

// various component imports
use components::{comp_spr::SpriteComponent};

// Non-core components
use assets::components::{player_components};

use player_components::comp_player_controller::PlayerControllerComponent;

pub fn new_local_player () -> (Entity, VecDeque<Component>) {

    let mut ent = Entity::new_base("Player".to_string());
    let mut ent_queue = VecDeque::new();
    
    ent.set_layer(1);

    let mut some_sprite = SpriteComponent::new("smile".to_string());
    
    let sprite_size = 32;

    some_sprite.transform.set_width(sprite_size);
    some_sprite.transform.set_height(sprite_size);

    some_sprite.transform.nudge_x(-sprite_size/2);
    some_sprite.transform.nudge_y(sprite_size/2);

    ent_queue.push_back(
        Component::new(
            ComponentData::Sprite(
                some_sprite
            )
        )
    );

    ent_queue.push_back(
        Component::new(
            ComponentData::PlayerController(
                PlayerControllerComponent::new()
            )
        )
    );

    return (ent, ent_queue);

}