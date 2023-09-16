#[derive(Clone, Debug, PartialEq)]
pub struct Bomba {
    pub x: usize,
    pub y: usize,
    pub radio: u32,
    pub especial: bool,
}

impl Bomba {
    /// Crea una bomba.
    /// Si especial=true, la bomba va a ser capaz de sortear piedras, caso contrario no.
    pub fn crear(x: usize, y: usize, radio: u32, especial: bool) -> Bomba {
        Bomba {
            x: x,
            y: y,
            radio: radio,
            especial: especial,
        }
    }
}
