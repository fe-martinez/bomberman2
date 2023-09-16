use super::tile::Tile;

#[derive(Clone, Debug, PartialEq)]
pub struct Mapa {
    pub tiles: Vec<Vec<Tile>>,
    pub side_size: usize,
}

impl Mapa {
    /// Devuelve la referencia al tile en la posicion (x_pos, y_pos) si existe, caso contrario None.
    pub fn obtener_tile(&self, x_pos: usize, y_pos: usize) -> Option<&Tile> {
        if x_pos >= self.side_size && y_pos >= self.side_size {
            return None;
        }
        return Some(&self.tiles[y_pos][x_pos]);
    }

    /// Devuelve la referencia mutable al tile en la posicion (x_pos, y_pos) si existe, caso contrario None.
    pub fn obtener_tile_mut(&mut self, x_pos: usize, y_pos: usize) -> Option<&mut Tile> {
        if x_pos >= self.side_size && y_pos >= self.side_size {
            return None;
        }
        return Some(&mut self.tiles[y_pos][x_pos]);
    }

    /// Destruye el tile en la posicion (x_pos, y_pos), poniendo un Tile Vacio en su lugar.
    pub fn destruir_tile(&mut self, x_pos: usize, y_pos: usize) {
        self.tiles[y_pos][x_pos] = Tile::Vacio;
    }

    /// Recibe las coordenadas de una bomba y la posicion que se debe atacar, si hay un enemigo en esa posicion, le descuenta vida.
    /// Si la vida del enemigo es menor o igual a 0, destruye el tile.
    pub fn atacar_enemigo(
        &mut self,
        bomba_x: usize,
        bomba_y: usize,
        x_pos: usize,
        y_pos: usize,
        dmg: u32,
    ) {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mapa_obtener_tile() {
        let mapa = Mapa {
            tiles: vec![
                vec![Tile::Vacio, Tile::Vacio],
                vec![Tile::Vacio, Tile::Vacio],
            ],
            side_size: 2,
        };
        assert_eq!(mapa.obtener_tile(0, 0), Some(&Tile::Vacio));
        assert_eq!(mapa.obtener_tile(1, 0), Some(&Tile::Vacio));
        assert_eq!(mapa.obtener_tile(0, 1), Some(&Tile::Vacio));
        assert_eq!(mapa.obtener_tile(1, 1), Some(&Tile::Vacio));
        assert_eq!(mapa.obtener_tile(2, 2), None);
    }

    #[test]
    fn test_mapa_obtener_tile_mut() {
        let mut mapa = Mapa {
            tiles: vec![
                vec![Tile::Vacio, Tile::Vacio],
                vec![Tile::Vacio, Tile::Vacio],
            ],
            side_size: 2,
        };
        assert_eq!(mapa.obtener_tile_mut(0, 0), Some(&mut Tile::Vacio));
        assert_eq!(mapa.obtener_tile_mut(1, 0), Some(&mut Tile::Vacio));
        assert_eq!(mapa.obtener_tile_mut(0, 1), Some(&mut Tile::Vacio));
        assert_eq!(mapa.obtener_tile_mut(1, 1), Some(&mut Tile::Vacio));
        assert_eq!(mapa.obtener_tile_mut(2, 2), None);
    }

    #[test]
    fn test_mapa_destruir_tile() {
        let mut mapa = Mapa {
            tiles: vec![
                vec![Tile::Vacio, Tile::Vacio],
                vec![Tile::Vacio, Tile::Vacio],
            ],
            side_size: 2,
        };
        mapa.destruir_tile(0, 0);
        assert_eq!(mapa.obtener_tile(0, 0), Some(&Tile::Vacio));
        mapa.destruir_tile(1, 0);
        assert_eq!(mapa.obtener_tile(1, 0), Some(&Tile::Vacio));
        mapa.destruir_tile(0, 1);
        assert_eq!(mapa.obtener_tile(0, 1), Some(&Tile::Vacio));
    }
}
