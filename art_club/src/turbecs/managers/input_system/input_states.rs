use turbo::*;

#[turbo::serialize]
#[derive(PartialEq)]

pub enum InputStates {
    JustPressed,
    Pressed,
    JustReleased,
    Released,
}