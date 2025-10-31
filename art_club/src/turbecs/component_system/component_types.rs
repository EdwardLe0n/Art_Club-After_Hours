#[turbo::serialize]
#[derive(PartialEq)]
pub enum ComponentTypes {
    Camera,
    Rectangle,
    Sprite,
    Text,
    TextBox,
    Button,
    Particle,

    // User made structs

    Resizer,

    PlayerRenderer,
    PlayerController,
    PlayerGhost,

    // Other

    Other

}