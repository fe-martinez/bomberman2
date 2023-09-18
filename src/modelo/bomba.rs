use std::collections::HashSet;

use super::coordenada::Coordenada;

#[derive(Clone, Debug, PartialEq)]
pub struct Bomba {
    pub x: usize,
    pub y: usize,
    pub radio: u32,
    pub especial: bool,
    pub enemigos_impactados: HashSet<Coordenada>,
}

impl Bomba {
    /// Crea una bomba.
    /// Si especial=true, la bomba va a ser capaz de sortear piedras, caso contrario no.
    pub fn crear(x: usize, y: usize, radio: u32, especial: bool) -> Bomba {
        Bomba {
            x,
            y,
            radio,
            especial,
            enemigos_impactados: HashSet::new(),
        }
    }

    pub fn coordenadas(&self) -> Coordenada {
        Coordenada {
            x: self.x,
            y: self.y,
        }
    }
}
