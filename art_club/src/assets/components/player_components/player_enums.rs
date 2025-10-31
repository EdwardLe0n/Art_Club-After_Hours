use turbo::*;

#[turbo::serialize]
#[derive(PartialEq)]
pub enum PlayerDirection {
    Up,
    Down,
    Left,
    Right
}

#[turbo::serialize]
#[derive(PartialEq)]
pub enum PlayerState {
    Idle,
    Walking
}

#[turbo::serialize]
#[derive(PartialEq)]
pub enum PlayerSprite {
    Test,
}