use super::{coordenada::Coordenada, direccion, tile::Tile};

#[derive(Clone, Debug, PartialEq)]
pub struct Mapa {
    pub tiles: Vec<Vec<Tile>>,
    pub side_size: usize,
}

impl Mapa {
    /// Devuelve un tile si es que se cumplen las condiciones correctas:
    ///   - La tile existe.
    ///   - Si la tile no es una pared.
    ///   - Si la tile es una roca y especial fue seteado en false.
    fn chequear_tile(&self, x_pos: usize, y_pos: usize, especial: bool) -> Option<&Tile> {
        if let Some(tile) = self.obtener_tile(x_pos, y_pos) {
            if matches!(tile, Tile::Piedra(_)) && !especial | matches!(tile, Tile::Pared(_)) {
                return None;
            }
            return Some(tile);
        }
        None
    }

    fn esta_fuera_de_rango(&self, x: i32, y: i32) -> bool {
        x < 0 || x >= self.side_size as i32 || y < 0 || y >= self.side_size as i32
    }

    /// Devuelve un vector de coordenadas que representan las tiles que se encuentran en el alcance de la bomba.
    /// Si la bomba es especial, puede sortear piedras, caso contrario no.
    /// Si la bomba encuentra un desvio, se desvia en la direccion que indica el desvio.
    /// Busca desde la posicion de la bomba en direccion a la recta indicada por el vector (dx, dy).
    pub fn buscar_en_direccion(
        &self,
        x_pos: usize,
        y_pos: usize,
        alcance: usize,
        especial: bool,
        dx: i32,
        dy: i32,
    ) -> Vec<Coordenada> {
        let mut tiles: Vec<Coordenada> = Vec::new();
        let mut x = x_pos as i32;
        let mut y = y_pos as i32;
        for _ in 0..alcance {
            x += dx;
            y += dy;
            if self.esta_fuera_de_rango(x, y) {
                break;
            }
            match self.chequear_tile(x as usize, y as usize, especial) {
                None => break,
                Some(Tile::Desvio(_)) => {
                    let faltante = alcance - tiles.len();
                    tiles.append(&mut self.desviar(x as usize, y as usize, faltante, especial));
                    break;
                }
                Some(_) => tiles.push(Coordenada {
                    x: x as usize,
                    y: y as usize,
                }),
            }
        }
        tiles
    }

    fn desviar(
        &self,
        x_pos: usize,
        y_pos: usize,
        alcance: usize,
        especial: bool,
    ) -> Vec<Coordenada> {
        if let Some(Tile::Desvio(desvio)) = self.obtener_tile(x_pos, y_pos) {
            match desvio.direccion {
                direccion::Direccion::Arriba => {
                    return self.buscar_en_direccion(x_pos, y_pos - 1, alcance, especial, 0, -1);
                }
                direccion::Direccion::Abajo => {
                    return self.buscar_en_direccion(x_pos, y_pos + 1, alcance, especial, 0, 1);
                }
                direccion::Direccion::Izquierda => {
                    return self.buscar_en_direccion(x_pos - 1, y_pos, alcance, especial, -1, 0);
                }
                direccion::Direccion::Derecha => {
                    return self.buscar_en_direccion(x_pos + 1, y_pos, alcance, especial, 1, 0);
                }
            }
        }
        Vec::new()
    }

    /// Devuelve la referencia al tile en la posicion (x_pos, y_pos) si existe, caso contrario None.
    pub fn obtener_tile(&self, x_pos: usize, y_pos: usize) -> Option<&Tile> {
        if x_pos >= self.side_size || y_pos >= self.side_size {
            return None;
        }
        Some(&self.tiles[y_pos][x_pos])
    }

    /// Devuelve la referencia mutable al tile en la posicion (x_pos, y_pos) si existe, caso contrario None.
    fn obtener_tile_mut(&mut self, x_pos: usize, y_pos: usize) -> Option<&mut Tile> {
        if x_pos >= self.side_size || y_pos >= self.side_size {
            return None;
        }
        Some(&mut self.tiles[y_pos][x_pos])
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

    #[test]
    fn test_buscar_tiles() {
        let mapa = Mapa {
            tiles: vec![
                vec![Tile::Vacio, Tile::Vacio, Tile::Vacio],
                vec![Tile::Vacio, Tile::Vacio, Tile::Vacio],
                vec![Tile::Vacio, Tile::Vacio, Tile::Vacio],
            ],
            side_size: 3,
        };
        let tiles = mapa.buscar_en_direccion(1, 2, 2, false, 0, -1);
        assert_eq!(tiles.len(), 2);
        assert_eq!(tiles[0], Coordenada { x: 1, y: 1 });
        assert_eq!(tiles[1], Coordenada { x: 1, y: 0 });
    }
}
