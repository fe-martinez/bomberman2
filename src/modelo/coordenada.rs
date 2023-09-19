#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]

/// Representa una coordenada en el mapa.
/// x siendo la posicion horizontal e y la vertical.
/// El modelo no utiliza coordenadas negativas. El (0, 0) se encuentra en la esquina superior izquierda.
pub struct Coordenada {
    pub x: usize,
    pub y: usize,
}
