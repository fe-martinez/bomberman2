use super::{bomba::Bomba, desvio::Desvio, enemigo::Enemigo, obstaculo::Obstaculo};

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
