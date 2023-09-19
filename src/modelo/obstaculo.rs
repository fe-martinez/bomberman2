/// Representa un obstaculo en el mapa.
/// Si pasable=true, el obstaculo va a poder ser sorteado por bombas especiales, caso contrario no.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Obstaculo {
    pub x: usize,
    pub y: usize,
    pub pasable: bool,
}

impl Obstaculo {
    /// Crea un obstaculo.
    /// Si pasable=true, el obstaculo va a poder ser sorteado por bombas especiales, caso contrario no.
    pub fn crear(x: usize, y: usize, pasable: bool) -> Obstaculo {
        Obstaculo { x, y, pasable }
    }
}
