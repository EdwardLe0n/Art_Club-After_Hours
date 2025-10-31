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
use player_components::comp_player_renderer::PlayerRendererComponent;
use player_components::comp_player_ghost::PlayerGhostComponent;

pub fn new_local_player () -> (Entity, VecDeque<Component>) {

    let mut ent = Entity::new_base("Player".to_string());
    let mut ent_queue = VecDeque::new();
    
    ent.set_layer(1);

    ent_queue.push_back(
        Component::new(
            ComponentData::PlayerRenderer(
                PlayerRendererComponent::new()
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

pub fn new_online_player (some_id : String) -> (Entity, VecDeque<Component>) {

    let mut ent = Entity::new_base("Player".to_string());
    let mut ent_queue = VecDeque::new();
    
    ent.set_layer(1);

    ent_queue.push_back(
        Component::new(
            ComponentData::PlayerRenderer(
                PlayerRendererComponent::new()
            )
        )
    );

    ent_queue.push_back(
        Component::new(
            ComponentData::PlayerGhost(
                PlayerGhostComponent::new(some_id)
            )
        )
    );

    return (ent, ent_queue);

}