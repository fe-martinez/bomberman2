use super::tile::Tile;

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

    pub fn obtener_tile_mut(&mut self, x_pos: usize, y_pos: usize) -> Option<&mut Tile> {
        if x_pos >= self.side_size && y_pos >= self.side_size {
            return None;
        }
        return Some(&mut self.tiles[y_pos][x_pos]);
    }

    pub fn destruir_tile(&mut self, x_pos: usize, y_pos: usize) {
        self.tiles[y_pos][x_pos] = Tile::Vacio;
    }

    pub fn atacar_enemigo(&mut self, bomba_x: usize, bomba_y: usize, x_pos: usize, y_pos: usize, dmg: u32) {
        if let Some(tile) = self.obtener_tile_mut(x_pos, y_pos) {
            if let Tile::Enemigo(enemigo) = tile {
                if !enemigo.ya_impactado(bomba_x, bomba_y) {
                    if enemigo.vida <= dmg {
                        self.destruir_tile(x_pos, y_pos);
                    } else {
                        enemigo.recibir_impacto(bomba_x, bomba_y);
                        enemigo.descontar_vida(dmg);
                        
                    }
                }
            }
        }
    }
}