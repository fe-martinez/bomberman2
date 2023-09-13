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

impl Tile {
    fn coordenada(&self) -> (usize, usize) {
        match self {
            Tile::Enemigo(enemigo) => (enemigo.x, enemigo.y),
            Tile::BombaNormal(bomba) => (bomba.x, bomba.y),
            Tile::BombaEspecial(bomba) => (bomba.x, bomba.y),
            Tile::Desvio(desvio) => (desvio.x, desvio.y),
            Tile::Piedra(piedra) => (piedra.x, piedra.y),
            Tile::Pared(pared) => (pared.x, pared.y),
            Tile::Vacio => (0, 0),
        }
    }
}