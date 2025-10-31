use turbo::*;

#[turbo::serialize]
#[derive(PartialEq)]
pub struct InputBasket {

    pub up : bool,
    pub down : bool,
    pub left : bool,
    pub right : bool,

}

impl InputBasket {
    pub fn new() -> Self {

        return Self {
            up : false,
            down : false,
            left : false,
            right : false
        };

    }
}

impl InputBasket {

    // pub fn update

}