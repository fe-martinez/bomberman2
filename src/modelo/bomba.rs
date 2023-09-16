#[derive(Clone, Debug, PartialEq)]
pub struct Bomba {
    pub x: usize,
    pub y: usize,
    pub radio: u32,
    pub especial: bool,
} 

impl Bomba {
    pub fn crear(x: usize, y: usize, radio: u32, especial: bool) -> Bomba {
        Bomba {x: x, y: y, radio: radio, especial: especial}
    }
}

