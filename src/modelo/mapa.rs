use super::{tile::Tile, enemigo::Enemigo};

#[derive(Clone)]
pub struct Mapa {
    pub tiles: Vec<Vec<Tile>>,
    pub side_size: usize,
}

impl Mapa {
    pub fn obtener_tile(&self, x_pos: usize, y_pos: usize) -> Option<&Tile> {
        if x_pos >= self.side_size && y_pos >= self.side_size {
            return None;
        }
        return Some(&self.tiles[y_pos][x_pos]);
    }

    pub fn destruir_tile(&mut self, x_pos: usize, y_pos: usize) {
        self.tiles[y_pos][x_pos] = Tile::Vacio;
    }

    pub fn atacar_enemigo(&mut self, x_pos: usize, y_pos: usize, dmg: u32) {
        match self.tiles[y_pos][x_pos] {
            Tile::Enemigo(enemigo) => {
                self.tiles[y_pos][x_pos] = Tile::Enemigo(Enemigo {
                    x: x_pos,
                    y: y_pos,
                    vida: enemigo.vida - dmg,
                });
            }
            _ => (),
        }
    }
}