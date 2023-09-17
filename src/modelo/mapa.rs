use super::{coordenada::Coordenada, direccion, tile::Tile};

#[derive(Clone, Debug, PartialEq)]
pub struct Mapa {
    pub tiles: Vec<Vec<Tile>>,
    pub side_size: usize,
}

trait EstrategiaBusqueda {
    fn buscar(&self, x_pos: usize, y_pos: usize, alcance: usize, especial: bool)
        -> Vec<Coordenada>;
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
            if x < 0 || x >= self.side_size as i32 || y < 0 || y >= self.side_size as i32 {
                break;
            }
            match self.chequear_tile(x as usize, y as usize, especial) {
                None => break,
                Some(tile) => match tile {
                    Tile::Desvio(_) => {
                        let faltante = alcance - tiles.len();
                        tiles.append(&mut self.desviar(x as usize, y as usize, faltante, especial));
                        break;
                    }
                    _ => tiles.push(Coordenada {
                        x: x as usize,
                        y: y as usize,
                    }),
                },
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
        if x_pos >= self.side_size && y_pos >= self.side_size {
            return None;
        }
        Some(&self.tiles[y_pos][x_pos])
    }

    /// Devuelve la referencia mutable al tile en la posicion (x_pos, y_pos) si existe, caso contrario None.
    pub fn obtener_tile_mut(&mut self, x_pos: usize, y_pos: usize) -> Option<&mut Tile> {
        if x_pos >= self.side_size && y_pos >= self.side_size {
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
    pub fn atacar_enemigo(&mut self, coor_bomba: Coordenada, coor_enemigo: Coordenada, dmg: u32) {
        if let Some(Tile::BombaNormal(bomba) | Tile::BombaEspecial(bomba)) =
            self.obtener_tile_mut(coor_bomba.x, coor_bomba.y)
        {
            if !bomba.registar_impacto(coor_enemigo.x, coor_enemigo.y) {
                return;
            }
        }

        if let Some(Tile::Enemigo(enemigo)) = self.obtener_tile_mut(coor_enemigo.x, coor_enemigo.y)
        {
            if enemigo.vida <= dmg {
                self.destruir_tile(coor_enemigo.x, coor_enemigo.y);
            } else {
                enemigo.descontar_vida(dmg);
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
