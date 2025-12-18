use std::collections::VecDeque;

use turbo::*;

use crate::turbecs;

use turbecs::{entity::Entity, component_system};
use component_system::component::{Component, ComponentData};

// Standard Components
use component_system::components::{comp_text::TextComponent, comp_text_box::TextBoxComponent, comp_butn::ButtonComponent};
use component_system::components::buttons::button_types::ButtonTypes;

use component_system::components::{comp_rect::RectangleComponent, comp_spr::SpriteComponent};

use turbecs::helpers::{bound_data};

// User defined components
use crate::assets;

use assets::components::{misc_components};

use misc_components::{comp_resizer::ResizerComponent, comp_textbox_resizer::TextBoxResizerComponent};
use misc_components::{comp_text_box_filler::TextBoxFillerComponent};
use misc_components::{comp_fade::FadeComponent};

use misc_components::{comp_logo_fade::{LogoFadeComponent, LogoFade}};

pub fn new_title () -> (Entity, VecDeque<Component>) {

    let mut ent = Entity::new_base("Title".to_string());
    let mut ent_queue = VecDeque::new();
    
    ent.set_layer(10);

    let mut text_box = TextBoxComponent::new("Art Club : After Hours".to_string());

    text_box.font = "large".to_string();
    text_box.color = 0xff0000ff;

    text_box.transform.set_width(TextComponent::get_text_offset(&text_box.text, &text_box.font).get_x() * 4);
    text_box.transform.set_height(TextComponent::get_text_offset(&text_box.text, &text_box.font).get_y() as i32 * -2);
    // text_box.transform.set_scale(1.2);

    ent.transform.nudge_y(text_box.transform.get_height() / 2);

    text_box.transform.position.set_horizontal_pref(bound_data::Horizontal::Center);

    ent_queue.push_front(Component::new(ComponentData::TextBox(text_box)));

    return (ent, ent_queue);

}

pub fn new_to_misc() -> (Entity, VecDeque<Component>) {

    let mut ent = Entity::new_base("Misc".to_string());
    let mut ent_queue = VecDeque::new();

    ent.transform.position.nudge_y( -40);

    ent.transform.position.set_vertical_pref(bound_data::Vertical::Bottom);

    let mut button = ButtonComponent::new();

    button.color = 0x000000ff;

    button.border.set_size(2);
    button.border.set_radius(2);
    button.border.set_color(0xaaaaaaff);

    button.transform.set_width(100);
    button.transform.set_height(40);

    button.button_type = ButtonTypes::Misc;

    ent_queue.push_back(Component::new(ComponentData::Button(button)));

    let mut text_box = TextBoxComponent::new("Misc".to_string());

    text_box.transform.set_height(100);
    text_box.transform.set_width(100);

    text_box.font = "large".to_string();
    text_box.color = 0xffffffff;

    ent_queue.push_back(Component::new(ComponentData::TextBox(text_box)));

    ent_queue.push_back(Component::new(ComponentData::TextBoxResizer(TextBoxResizerComponent::new_with_buffers(2, 2))));

    return (ent, ent_queue);

}

pub fn new_to_lobby() -> (Entity, VecDeque<Component>) {

    let mut ent = Entity::new_base("Play".to_string());
    let mut ent_queue = VecDeque::new();

    ent.transform.position.nudge_y( -40);

    ent.transform.position.set_vertical_pref(bound_data::Vertical::Bottom);

    let mut button = ButtonComponent::new();

    button.color = 0x000000ff;

    button.border.set_size(2);
    button.border.set_radius(2);
    button.border.set_color(0xaaaaaaff);

    button.transform.set_width(100);
    button.transform.set_height(40);

    button.button_type = ButtonTypes::ToLobby;

    ent_queue.push_back(Component::new(ComponentData::Button(button)));

    let mut text_box = TextBoxComponent::new("Play".to_string());

    text_box.transform.set_height(100);
    text_box.transform.set_width(100);

    text_box.font = "large".to_string();
    text_box.color = 0xffffffff;

    ent_queue.push_back(Component::new(ComponentData::TextBox(text_box)));

    ent_queue.push_back(Component::new(ComponentData::TextBoxResizer(TextBoxResizerComponent::new_with_buffers(2, 2))));

    return (ent, ent_queue);

}

pub fn new_to_title() -> (Entity, VecDeque<Component>) {

    let mut ent = Entity::new_base("Back".to_string());
    let mut ent_queue = VecDeque::new();

    ent.transform.nudge_y( (screen().h() / 2 * 17 / 20) as i32 );
    ent.transform.nudge_x((screen().h() / 2 * 3 / 4) as i32 * -1);

    ent.transform.position.set_vertical_pref(bound_data::Vertical::Bottom);

    let mut button = ButtonComponent::new();

    button.color = 0x000000ff;

    button.border.set_size(2);
    button.border.set_radius(2);
    button.border.set_color(0xaaaaaaff);

    button.transform.set_width(100);
    button.transform.set_height(40);

    button.button_type = ButtonTypes::Title;

    ent_queue.push_back(Component::new(ComponentData::Button(button)));

    let mut text_box = TextBoxComponent::new("Back".to_string());

    text_box.transform.set_height(100);
    text_box.transform.set_width(100);

    text_box.font = "large".to_string();
    text_box.color = 0xffffffff;

    ent_queue.push_back(Component::new(ComponentData::TextBox(text_box)));

    ent_queue.push_back(Component::new(ComponentData::TextBoxResizer(TextBoxResizerComponent::new_with_buffers(2, 2))));

    return (ent, ent_queue);

}

pub fn new_turbecs_card() -> (Entity, VecDeque<Component>) {

    let mut ent = Entity::new_base("Turbecs".to_string());
    let mut ent_queue = VecDeque::new();

    ent.set_layer(1);

    ent.transform.position.is_bounded = true;
    ent.transform.position.bound_data.set_ui_status(true);

    let mut logo = SpriteComponent::new("misc/TurbECS_Logo_mini".to_owned());

    logo.transform.set_width(102);
    logo.transform.set_height(102);

    logo.transform.nudge_x(-51);
    logo.transform.nudge_y(51 + 25);

    ent_queue.push_back(
        Component::new(
            ComponentData::Sprite(
                logo
            )
        )
    );

    let mut text = TextBoxComponent::new("Made with TurbECS".to_owned());

    text.transform.nudge_y(25);

    text.font = "large".to_owned();
    text.color = 0xffffffff;

    ent_queue.push_back(
        Component::new(
            ComponentData::TextBox(
                text
            )
        )
    );

    ent_queue.push_back(
        Component::new(
            ComponentData::LogoFade(
                LogoFadeComponent::new(
                    LogoFade::TurbECS
                )
            )
        )
    );

    return (ent, ent_queue);

}

pub fn new_background() -> (Entity, VecDeque<Component>) {
    let mut ent = Entity::new_base("Turbecs".to_string());
    let mut ent_queue = VecDeque::new();

    ent.transform.position.is_bounded = true;
    ent.transform.position.bound_data.set_ui_status(true);

    let mut rect = RectangleComponent::new_rect(
        screen().w() as i32 * 2,
        screen().h() as i32 * 2,
        0x000000ff
    );

    rect.transform.nudge_x(-1 * screen().w() as i32);
    rect.transform.nudge_y(screen().h() as i32);

    ent_queue.push_back(
        Component::new(
            ComponentData::Rectangle(
                rect
            )
        )
    );

    return (ent, ent_queue);

}

pub fn new_input_notice() -> (Entity, VecDeque<Component>) {
    let mut ent = Entity::new_base("notice".to_string());
    let mut ent_queue = VecDeque::new();

    ent.set_layer(1);

    ent.transform.position.is_bounded = true;
    ent.transform.position.bound_data.set_ui_status(true);

    ent.transform.position.bound_data.set_vertical_pref(bound_data::Vertical::Top);

    let mut text = TextBoxComponent::new("Choose an input type".to_owned());

    text.transform.nudge_y((screen().h() / 4) as i32 * -1);

    text.color = 0xffffffff;
    text.font = "large".to_owned();

    ent_queue.push_back(
        Component::new(
            ComponentData::TextBox(
                text
            )
        )
    );

    ent_queue.push_back(
        Component::new(
            ComponentData::TextBoxResizer(
                TextBoxResizerComponent::new()
            )
        )
    );

    return (ent, ent_queue);

}

pub fn new_mouse_notice() -> (Entity, VecDeque<Component>) {
    let mut ent = Entity::new_base("notice".to_string());
    let mut ent_queue = VecDeque::new();

    ent.set_layer(1);

    ent.transform.position.is_bounded = true;
    ent.transform.position.bound_data.set_ui_status(true);

    let button_width = 50;

    ent.transform.nudge_x(-(screen().w() as i32) / 4);    

    let mut some_button = ButtonComponent::new();

    some_button.transform.set_width(button_width);
    some_button.transform.set_height(button_width);

    some_button.color = 0x000000ff;
    some_button.border.set_size(4);
    some_button.border.set_color(0xffffffff);
    some_button.border.set_radius(2);

    some_button.button_type = ButtonTypes::MouseOn;

    ent_queue.push_back(
        Component::new(
            ComponentData::Button(
                some_button
            )
        )
    );

    return (ent, ent_queue);

}

pub fn new_mobile_notice() -> (Entity, VecDeque<Component>) {
    let mut ent = Entity::new_base("notice".to_string());
    let mut ent_queue = VecDeque::new();

    ent.set_layer(1);

    ent.transform.position.is_bounded = true;
    ent.transform.position.bound_data.set_ui_status(true);

    let button_width = 50;

    ent.transform.nudge_x((screen().w() as i32) / 4);    

    let mut some_button = ButtonComponent::new();

    some_button.transform.set_width(button_width);
    some_button.transform.set_height(button_width);

    some_button.color = 0x000000ff;
    some_button.border.set_size(4);
    some_button.border.set_color(0xffffffff);
    some_button.border.set_radius(2);

    some_button.button_type = ButtonTypes::MobileOn;

    ent_queue.push_back(
        Component::new(
            ComponentData::Button(
                some_button
            )
        )
    );

    return (ent, ent_queue);

}

pub fn new_from_input_to_title() -> (Entity, VecDeque<Component>) {
    let mut ent = Entity::new_base("notice".to_string());
    let mut ent_queue = VecDeque::new();

    ent.set_layer(1);

    ent.transform.position.is_bounded = true;
    ent.transform.position.bound_data.set_ui_status(true);

    ent.transform.position.bound_data.set_vertical_pref(bound_data::Vertical::Bottom);

    ent.transform.nudge_y((screen().h() / 4) as i32);

    let mut some_button = ButtonComponent::new();

    some_button.color = 0x000000ff;
    some_button.border.set_size(2);
    some_button.border.set_color(0xffffffff);
    some_button.border.set_radius(2);

    some_button.button_type = ButtonTypes::Title;

    ent_queue.push_back(
        Component::new(
            ComponentData::Button(
                some_button
            )
        )
    );

    let mut text = TextBoxComponent::new("To Title".to_owned());

    text.color = 0xffffffff;
    text.font = "large".to_owned();

    ent_queue.push_back(
        Component::new(
            ComponentData::TextBox(
                text
            )
        )
    );

    ent_queue.push_back(
        Component::new(
            ComponentData::TextBoxResizer(
                TextBoxResizerComponent::new()
            )
        )
    );

    return (ent, ent_queue);
}