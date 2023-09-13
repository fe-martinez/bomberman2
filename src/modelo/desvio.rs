
#[derive(Debug, Clone, Copy)]
pub struct Desvio {
    pub x: usize,
    pub y: usize,
    pub direccion: Direccion,
}


#[derive(Debug, Clone, Copy)]
pub enum Direccion {
    Arriba,
    Abajo,
    Izquierda,
    Derecha,
}

impl Desvio {
    pub fn char_direccion(&self) -> char {
        match self.direccion {
            Direccion::Arriba => 'U',
            Direccion::Abajo => 'D',
            Direccion::Izquierda => 'L',
            Direccion::Derecha => 'R',
        }
    }
}