use super::{enemigo::Enemigo, bomba::Bomba, desvio::Desvio, obstaculo::Obstaculo};

#[derive(Clone, Debug)]
pub enum Tile {
    Enemigo(Enemigo),
    BombaNormal(Bomba),
    BombaEspecial(Bomba),
    Desvio(Desvio),
    Piedra(Obstaculo),
    Pared(Obstaculo),
    Vacio,
}