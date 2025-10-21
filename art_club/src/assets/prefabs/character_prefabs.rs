// 
use std::collections::VecDeque;
use crate::{turbecs, assets};

// necessary turbecs imports
use turbecs::entity::Entity;
use turbecs::component_system::{component, components};
use component::{Component, ComponentData};

// various component imports
use components::{comp_spr::SpriteComponent};

pub fn new_local_player () -> (Entity, VecDeque<Component>) {

    let mut ent = Entity::new_base("Player".to_string());
    let mut ent_queue = VecDeque::new();
    
    ent.set_layer(1);

    let mut some_sprite = SpriteComponent::new("smile".to_string());
    
    some_sprite.transform.set_width(32);
    some_sprite.transform.set_height(32);

    ent_queue.push_back(
        Component::new(
            ComponentData::Sprite(
                some_sprite
            )
        )
    );

    return (ent, ent_queue);

}