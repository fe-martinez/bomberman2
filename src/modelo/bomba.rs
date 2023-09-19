use super::coordenada::Coordenada;

/// Bomba que puede ser normal o especial.
/// Si es especial, puede sortear piedras.
/// Explota en un patron de cruz con cada lado siendo de largo radio.
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
    /// Si radio=0, la bomba no va a causar daño a ninguna casilla.
    /// Si radio>0, la bomba va a causar daño a las casillas que se encuentren a distancia radio en linea recta.
    pub fn crear(x: usize, y: usize, radio: u32, especial: bool) -> Bomba {
        Bomba {
            x,
            y,
            radio,
            especial,
        }
    }

    pub fn coordenadas(&self) -> Coordenada {
        Coordenada {
            x: self.x,
            y: self.y,
        }
    }
}
