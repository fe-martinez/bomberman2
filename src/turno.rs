use crate::modelo::{mapa::Mapa, bomba::Bomba, tile::Tile, desvio, coordenada::Coordenada};

fn chequear_tile (x_pos: usize, y_pos: usize, mapa: &Mapa, especial: bool) -> Option<&Tile> {
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

fn buscar_arriba(mapa: &Mapa, x_pos: usize, y_pos: usize, alcance: usize, especial: bool) -> Vec<Coordenada> {
    let mut tiles_arriba: Vec<Coordenada> = Vec::new();
    
    for y in (y_pos as i32 - alcance as i32..y_pos as i32).rev() {
        if y < 0 {break};

        match chequear_tile(x_pos, y as usize, &mapa, especial){
            None => break,
            Some(tile) => match tile {
                Tile::Desvio(_) => {
                    let faltante = alcance as usize - (y_pos as i32 - y) as usize;
                    tiles_arriba.append(&mut desviar(mapa, x_pos, y as usize, faltante, especial));
                    break;
                }
                _ => tiles_arriba.push(Coordenada {x: x_pos, y: y as usize}),
            },
        }
    }
    tiles_arriba
}

fn buscar_abajo(mapa: &Mapa, x_pos: usize, y_pos: usize, alcance: usize, especial: bool) -> Vec<Coordenada> {
    let mut tiles_abajo: Vec<Coordenada> = Vec::new();
    
    for y in y_pos..(y_pos + alcance as usize + 1) {
        if y >= mapa.side_size as usize {break};

        match chequear_tile(x_pos, y as usize, mapa, especial){
            None => break,
            Some(tile) => match tile {
                Tile::Desvio(_) => {
                    let faltante = alcance as usize - (y - y_pos) as usize;
                    tiles_abajo.append(&mut desviar(mapa, x_pos, y, faltante, especial));
                    break;
                }
                _ => tiles_abajo.push(Coordenada {x: x_pos, y: y as usize}),
            },
        }
    }

    tiles_abajo
}

fn buscar_izquierda(mapa: &Mapa, x_pos: usize, y_pos: usize, alcance: usize, especial: bool) -> Vec<Coordenada> {
    let mut tiles_izquierda: Vec<Coordenada> = Vec::new();
    
    for x in (x_pos as i32 - alcance as i32..x_pos as i32).rev() {
        if x < 0 {break};

        match chequear_tile(x as usize, y_pos, mapa, especial){
            None => break,
            Some(tile) => match tile {
                Tile::Desvio(_) => {
                    let faltante = alcance as usize - (x_pos as i32 - x) as usize;
                    tiles_izquierda.append(&mut desviar(mapa, x as usize, y_pos, faltante, especial));
                    break;
                }
                _ => tiles_izquierda.push(Coordenada {x: x as usize, y: y_pos}),
            },
        }
    }

    tiles_izquierda
}

fn buscar_derecha(mapa: &Mapa, x_pos: usize, y_pos: usize, alcance: usize, especial: bool) -> Vec<Coordenada> {
    let mut tiles_derecha: Vec<Coordenada> = Vec::new();
    
    for x in x_pos..(x_pos + alcance as usize + 1) {
        if x >= mapa.side_size as usize {break};

        match chequear_tile(x as usize, y_pos, mapa, especial){
            None => break,
            Some(tile) => match tile {
                Tile::Desvio(_) => {
                    let faltante = alcance as usize - (x - x_pos) as usize;
                    tiles_derecha.append(&mut desviar(mapa, x, y_pos, faltante, especial));
                    break;
                }
                _ => tiles_derecha.push(Coordenada {x: x as usize, y: y_pos}),
            },
        }
    }

    tiles_derecha
}

fn desviar(mapa: &Mapa, x_pos: usize, y_pos: usize, alcance: usize, especial: bool) -> Vec<Coordenada>{
    let mut tiles_desvio: Vec<Coordenada> = Vec::new();
    println!("VINE ACA");
    if let Some(tile) = mapa.obtener_tile(x_pos, y_pos) {
        match tile {
            Tile::Desvio(desvio) => {
                match desvio.direccion {
                    desvio::Direccion::Arriba => {
                        tiles_desvio.append(&mut buscar_arriba(mapa, x_pos, y_pos, alcance, especial));
                        println!("ACA TAMBIEN");
                    }
                    desvio::Direccion::Abajo => {
                        tiles_desvio.append(&mut buscar_abajo(mapa, x_pos, y_pos, alcance, especial));
                    }
                    desvio::Direccion::Izquierda => {
                        tiles_desvio.append(&mut buscar_izquierda(mapa, x_pos, y_pos, alcance, especial));
                    }
                    desvio::Direccion::Derecha => {
                        tiles_desvio.append(&mut buscar_derecha(mapa, x_pos, y_pos, alcance, especial));
                    }
                }
            }
            _ => (),
        }
    }
    tiles_desvio
}


pub fn buscar_tiles(mapa: &Mapa, x_pos: usize, y_pos: usize, bomba: Bomba) -> Vec<Coordenada>{
    let mut tiles_encontradas = Vec::new();

    tiles_encontradas.append(&mut buscar_arriba(&mapa, x_pos, y_pos, bomba.radio as usize, bomba.especial));
    tiles_encontradas.append(&mut buscar_abajo(&mapa, x_pos, y_pos, bomba.radio as usize, bomba.especial));
    tiles_encontradas.append(&mut buscar_izquierda(&mapa, x_pos, y_pos, bomba.radio as usize, bomba.especial));
    tiles_encontradas.append(&mut buscar_derecha(&mapa, x_pos, y_pos, bomba.radio as usize, bomba.especial));
    
    for tile in &tiles_encontradas {
        println!("Tile: {:?}", tile);
        //println!("Tile: {:?}", mapa.obtener_tile(x_pos, y_pos));
    }

    tiles_encontradas
}

pub fn jugar_turno(mapa: &mut Mapa, x_pos: usize, y_pos: usize){
    if let Some(tile) = mapa.obtener_tile(x_pos, y_pos) {
        match tile {
            Tile::BombaNormal(bomba) | Tile::BombaEspecial(bomba) => {
                let tiles_adyacentes: Vec<Coordenada> = buscar_tiles(&mapa, x_pos, y_pos, bomba.clone());
                mapa.destruir_tile(x_pos, y_pos);
                for tile in tiles_adyacentes {
                    println!("Tile: {:?}", tile);
                    println!("Tile: {:?}", mapa.obtener_tile(tile.x, tile.y));
                    match mapa.obtener_tile(tile.x, tile.y) {
                        Some(Tile::Enemigo(enemigo)) => {
                            if enemigo.vida > 1 {
                                mapa.atacar_enemigo(tile.x, tile.y, 1);
                            } else {
                                mapa.destruir_tile(tile.x, tile.y);
                            }
                        }
                        Some(Tile::BombaNormal(bomba_encontrada) | Tile::BombaEspecial(bomba_encontrada)) => {
                            jugar_turno(mapa, bomba_encontrada.x, bomba_encontrada.y);
                        }
                        _ => continue,
                    }
                }
            }
            _ => (),
        }
    }
}