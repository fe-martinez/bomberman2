
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Obstaculo {
    pub x: usize,
    pub y: usize,
    pub pasable: bool,
}

impl Obstaculo {
    pub fn crear(x: usize, y: usize, pasable: bool) -> Obstaculo {
        Obstaculo {x: x, y: y, pasable: pasable}
    }
}