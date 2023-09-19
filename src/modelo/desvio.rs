use super::direccion::Direccion;

/// Desvio que altera el recorrido de la explosion de una bomba.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Desvio {
    pub x: usize,
    pub y: usize,
    pub direccion: Direccion,
}

impl Desvio {
    /// Devuelve el char correspondiente para simbolizar la direccion del desvio.
    /// U: Arriba, D: Abajo, L: Izquierda, R: Derecha.
    pub fn char_direccion(&self) -> char {
        match self.direccion {
            Direccion::Arriba => 'U',
            Direccion::Abajo => 'D',
            Direccion::Izquierda => 'L',
            Direccion::Derecha => 'R',
        }
    }
}
