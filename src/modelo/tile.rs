use super::{bomba::Bomba, desvio::Desvio, enemigo::Enemigo, obstaculo::Obstaculo};

/// Representa un tile del mapa.
/// Agrupa todos los tipos posibles que pueden aparecer en el juego.
#[derive(Clone, Debug, PartialEq)]
pub enum Tile {
    Enemigo(Enemigo),
    BombaNormal(Bomba),
    BombaEspecial(Bomba),
    Desvio(Desvio),
    Piedra(Obstaculo),
    Pared(Obstaculo),
    Vacio,
}
