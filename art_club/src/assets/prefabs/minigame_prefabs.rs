use std::collections::VecDeque;

use turbo::*;

use crate::turbecs;

use turbecs::{entity::Entity, component_system};
use component_system::component::{Component, ComponentData};

// Standard Components

// User defined components
use crate::assets;

use assets::components::{misc_components};

pub fn new_temp () -> (Entity, VecDeque<Component>) {

    let mut ent = Entity::new_base("temp".to_string());
    let mut ent_queue = VecDeque::new();

    return (ent, ent_queue);

}