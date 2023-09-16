use crate::modelo::{bomba::Bomba, coordenada::Coordenada, desvio, mapa::Mapa, tile::Tile};

/// Devuelve un tile si es que se cumplen las condiciones correctas:
///   - La tile existe.
///   - Si la tile no es una pared.
///   - Si la tile es una roca y especial fue seteado en false.
fn chequear_tile(x_pos: usize, y_pos: usize, mapa: &Mapa, especial: bool) -> Option<&Tile> {
    if let Some(tile) = mapa.obtener_tile(x_pos, y_pos as usize) {
        if matches!(tile, Tile::Piedra(_)) && !especial {
            return None;
        } else if matches!(tile, Tile::Pared(_)) {
            return None;
        }
        return Some(tile);
    }
    return None;
}

/// Busca las posiciones en el mapa de la forma (y, x_pos) donde x_pos es fija e y va desde y_pos hasta y_pos - alcance.
/// Corta la busqueda cuando chequear_tile devuelve None.
fn buscar_arriba(
    mapa: &Mapa,
    x_pos: usize,
    y_pos: usize,
    alcance: usize,
    especial: bool,
) -> Vec<Coordenada> {
    let mut tiles_arriba: Vec<Coordenada> = Vec::new();

    for y in (y_pos as i32 - alcance as i32..y_pos as i32).rev() {
        if y < 0 {
            break;
        }

        match chequear_tile(x_pos, y as usize, &mapa, especial) {
            None => break,
            Some(tile) => match tile {
                Tile::Desvio(_) => {
                    let faltante = alcance as usize - (y_pos as i32 - y) as usize;
                    tiles_arriba.append(&mut desviar(mapa, x_pos, y as usize, faltante, especial));
                    break;
                }
                _ => tiles_arriba.push(Coordenada {
                    x: x_pos,
                    y: y as usize,
                }),
            },
        }
    }
    tiles_arriba
}

/// Busca las posiciones en el mapa de la forma (y, x_pos) donde x_pos es fija e y va desde y_pos hasta y_pos + alcance.
/// Corta la busqueda cuando chequear_tile devuelve None.
fn buscar_abajo(
    mapa: &Mapa,
    x_pos: usize,
    y_pos: usize,
    alcance: usize,
    especial: bool,
) -> Vec<Coordenada> {
    let mut tiles_abajo: Vec<Coordenada> = Vec::new();

    for y in y_pos..(y_pos + alcance + 1 as usize) {
        if y >= mapa.side_size as usize {
            break;
        }

        match chequear_tile(x_pos, y as usize, mapa, especial) {
            None => break,
            Some(tile) => match tile {
                Tile::Desvio(_) => {
                    let faltante = alcance as usize - (y - y_pos) as usize;
                    tiles_abajo.append(&mut desviar(mapa, x_pos, y, faltante, especial));
                    break;
                }
                _ => tiles_abajo.push(Coordenada {
                    x: x_pos,
                    y: y as usize,
                }),
            },
        }
    }

    tiles_abajo
}

/// Busca las posiciones en el mapa de la forma (y_pos, x) donde y_pos es fija y x va desde x_pos hasta x_pos - alcance.
/// Corta la busqueda cuando chequear_tile devuelve None.
fn buscar_izquierda(
    mapa: &Mapa,
    x_pos: usize,
    y_pos: usize,
    alcance: usize,
    especial: bool,
) -> Vec<Coordenada> {
    let mut tiles_izquierda: Vec<Coordenada> = Vec::new();

    for x in (x_pos as i32 - alcance as i32..x_pos as i32).rev() {
        if x < 0 {
            break;
        }

        match chequear_tile(x as usize, y_pos, mapa, especial) {
            None => break,
            Some(tile) => match tile {
                Tile::Desvio(_) => {
                    let faltante = alcance as usize - (x_pos as i32 - x) as usize;
                    tiles_izquierda
                        .append(&mut desviar(mapa, x as usize, y_pos, faltante, especial));
                    break;
                }
                _ => tiles_izquierda.push(Coordenada {
                    x: x as usize,
                    y: y_pos,
                }),
            },
        }
    }

    tiles_izquierda
}

/// Busca las posiciones en el mapa de la forma (y_pos, x) donde y_pos es fija e x va desde x_pos hasta x_pos + alcance.
/// Corta la busqueda cuando chequear_tile devuelve None.
fn buscar_derecha(
    mapa: &Mapa,
    x_pos: usize,
    y_pos: usize,
    alcance: usize,
    especial: bool,
) -> Vec<Coordenada> {
    let mut tiles_derecha: Vec<Coordenada> = Vec::new();

    for x in x_pos..(x_pos + alcance + 1 as usize) {
        if x >= (mapa.side_size as usize) {
            break;
        }

        match chequear_tile(x as usize, y_pos, mapa, especial) {
            None => break,
            Some(tile) => match tile {
                Tile::Desvio(_) => {
                    let faltante: usize = alcance as usize - (x - x_pos) as usize;
                    tiles_derecha.append(&mut desviar(mapa, x, y_pos, faltante, especial));
                    break;
                }
                _ => tiles_derecha.push(Coordenada {
                    x: x as usize,
                    y: y_pos,
                }),
            },
        }
    }

    tiles_derecha
}

/// Si en la posicion (x_pos, y_pos) hay un desvio, busca tiles en la direccion del desvio llamando a la funcion buscar correspondiente.
fn desviar(
    mapa: &Mapa,
    x_pos: usize,
    y_pos: usize,
    alcance: usize,
    especial: bool,
) -> Vec<Coordenada> {
    let mut tiles_desvio: Vec<Coordenada> = Vec::new();
    println!("VINE ACA, x: {}, y: {}, alcance: {}", x_pos, y_pos, alcance);
    if let Some(tile) = mapa.obtener_tile(x_pos, y_pos) {
        match tile {
            Tile::Desvio(desvio) => match desvio.direccion {
                desvio::Direccion::Arriba => {
                    tiles_desvio.append(&mut buscar_arriba(
                        mapa,
                        x_pos,
                        y_pos - 1,
                        alcance,
                        especial,
                    ));
                    println!("ACA TAMBIEN");
                }
                desvio::Direccion::Abajo => {
                    tiles_desvio.append(&mut buscar_abajo(
                        mapa,
                        x_pos,
                        y_pos + 1,
                        alcance,
                        especial,
                    ));
                }
                desvio::Direccion::Izquierda => {
                    tiles_desvio.append(&mut buscar_izquierda(
                        mapa,
                        x_pos - 1,
                        y_pos,
                        alcance,
                        especial,
                    ));
                }
                desvio::Direccion::Derecha => {
                    tiles_desvio.append(&mut buscar_derecha(
                        mapa,
                        x_pos + 1,
                        y_pos,
                        alcance,
                        especial,
                    ));
                }
            },
            _ => (),
        }
    }
    tiles_desvio
}

/// Busca tiles en todas las direcciones y las devuelve en un vector.
pub fn buscar_tiles(mapa: &Mapa, x_pos: usize, y_pos: usize, bomba: Bomba) -> Vec<Coordenada> {
    let mut tiles_encontradas = Vec::new();

    tiles_encontradas.append(&mut buscar_arriba(
        &mapa,
        x_pos,
        y_pos,
        bomba.radio as usize,
        bomba.especial,
    ));
    tiles_encontradas.append(&mut buscar_abajo(
        &mapa,
        x_pos,
        y_pos,
        bomba.radio as usize,
        bomba.especial,
    ));
    tiles_encontradas.append(&mut buscar_izquierda(
        &mapa,
        x_pos,
        y_pos,
        bomba.radio as usize,
        bomba.especial,
    ));
    tiles_encontradas.append(&mut buscar_derecha(
        &mapa,
        x_pos,
        y_pos,
        bomba.radio as usize,
        bomba.especial,
    ));

    for tile in &tiles_encontradas {
        println!("Tile: {:?}", tile);
    }

    tiles_encontradas
}

/// Juega un turno en la posicion (x_pos, y_pos) del mapa.
/// Si hay una bomba en esa posicion, destruye la bomba y busca tiles adyacentes, detonando otras bombas que se puedan encontrar en su alcance.
/// Si dentro del alcance de la bomba hay un enemigo, le descuenta vida.
pub fn jugar_turno(mapa: &mut Mapa, x_pos: usize, y_pos: usize) -> Result<(), &str> {
    if let Some(tile) = mapa.obtener_tile(x_pos, y_pos) {
        match tile {
            Tile::BombaNormal(bomba) | Tile::BombaEspecial(bomba) => {
                let tiles_adyacentes: Vec<Coordenada> =
                    buscar_tiles(&mapa, x_pos, y_pos, bomba.clone());
                mapa.destruir_tile(x_pos, y_pos);
                for tile in tiles_adyacentes {
                    match mapa.obtener_tile(tile.x, tile.y) {
                        Some(Tile::Enemigo(_)) => {
                            mapa.atacar_enemigo(x_pos, y_pos, tile.x, tile.y, 1);
                        }
                        Some(
                            Tile::BombaNormal(bomba_encontrada)
                            | Tile::BombaEspecial(bomba_encontrada),
                        ) => {
                            let _ = jugar_turno(mapa, bomba_encontrada.x, bomba_encontrada.y);
                        }
                        _ => continue,
                    }
                }
                return Ok(());
            }
            _ => return Err("No hay bomba en esa posicion"),
        }
    }
    return Err("No hay bomba en esa posicion");
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::modelo::{coordenada::Coordenada, mapa::Mapa, tile::Tile};

    #[test]
    fn test_chequear_tile() {
        let mapa = Mapa {
            tiles: vec![
                vec![Tile::Vacio, Tile::Vacio],
                vec![Tile::Vacio, Tile::Vacio],
            ],
            side_size: 2,
        };
        assert_eq!(chequear_tile(0, 0, &mapa, false), Some(&Tile::Vacio));
        assert_eq!(chequear_tile(1, 0, &mapa, false), Some(&Tile::Vacio));
        assert_eq!(chequear_tile(0, 1, &mapa, false), Some(&Tile::Vacio));
        assert_eq!(chequear_tile(1, 1, &mapa, false), Some(&Tile::Vacio));
        assert_eq!(chequear_tile(2, 2, &mapa, false), None);
    }

    #[test]
    fn test_buscar_arriba() {
        let mapa = Mapa {
            tiles: vec![
                vec![Tile::Vacio, Tile::Vacio],
                vec![Tile::Vacio, Tile::Vacio],
            ],
            side_size: 2,
        };
        let tiles = buscar_arriba(&mapa, 0, 1, 1, false);
        assert_eq!(tiles.len(), 1);
        assert_eq!(tiles[0], Coordenada { x: 0, y: 0 });
    }
}
