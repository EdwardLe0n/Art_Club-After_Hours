#[turbo::serialize]
#[derive(PartialEq)]
pub enum InputRenderPair {
    
    Letter(String),
    Image(String)

}