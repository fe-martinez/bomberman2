use crate::modelo::{bomba::Bomba, coordenada::Coordenada, mapa::Mapa, tile::Tile};

/// Busca tiles en todas las direcciones y las devuelve en un vector.
/// Busca en el siguiente orden de direcciones: arriba, abajo, derecha, izquierda.
pub fn buscar_tiles(mapa: &Mapa, x_pos: usize, y_pos: usize, bomba: Bomba) -> Vec<Coordenada> {
    let mut tiles_encontradas = Vec::new();
    let alcance = bomba.radio as usize;
    let especial = bomba.especial;

    let direcciones = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    for direccion in direcciones {
        tiles_encontradas.append(&mut mapa.buscar_en_direccion(
            x_pos,
            y_pos,
            alcance,
            especial,
            direccion.0,
            direccion.1,
        ));
    }
    tiles_encontradas
}

/// Juega un turno en la posicion (x_pos, y_pos) del mapa.
/// Si hay una bomba en esa posicion, destruye la bomba y busca tiles adyacentes, detonando otras bombas que se puedan encontrar en su alcance.
/// Si dentro del alcance de la bomba hay un enemigo, le descuenta vida.
pub fn jugar_turno(mapa: &mut Mapa, x_pos: usize, y_pos: usize) -> Result<(), &str> {
    match mapa.obtener_tile(x_pos, y_pos) {
        Some(Tile::BombaNormal(bomba)) | Some(Tile::BombaEspecial(bomba)) => {
            let tiles_adyacentes: Vec<Coordenada> = buscar_tiles(mapa, x_pos, y_pos, bomba.clone());
            mapa.destruir_tile(x_pos, y_pos);
            for tile in tiles_adyacentes {
                match mapa.obtener_tile(tile.x, tile.y) {
                    Some(Tile::Enemigo(_)) => {
                        mapa.atacar_enemigo(x_pos, y_pos, tile.x, tile.y, 1);
                    }
                    Some(Tile::BombaNormal(bomba_encontrada))
                    | Some(Tile::BombaEspecial(bomba_encontrada)) => {
                        let _ = jugar_turno(mapa, bomba_encontrada.x, bomba_encontrada.y);
                    }
                    _ => continue,
                }
            }
            Ok(())
        }
        _ => Err("No hay bomba en esa posicion"),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::modelo::{
        bomba::Bomba, coordenada::Coordenada, desvio::Desvio, direccion::Direccion,
        enemigo::Enemigo, mapa::Mapa, obstaculo::Obstaculo, tile::Tile,
    };

    #[test]
    fn test_fuera_de_rango() {
        let mut mapa = Mapa {
            side_size: 3,
            tiles: vec![
                vec![Tile::Vacio, Tile::Vacio, Tile::Vacio],
                vec![
                    Tile::Vacio,
                    Tile::BombaNormal(Bomba::crear(1, 1, 2, false)),
                    Tile::Vacio,
                ],
                vec![Tile::Vacio, Tile::Vacio, Tile::Vacio],
            ],
        };
        let resultado = jugar_turno(&mut mapa, 3, 3);
        assert_eq!(resultado, Err("No hay bomba en esa posicion"));
    }

    #[test]
    fn test_detonar() {
        let mut mapa = Mapa {
            side_size: 3,
            tiles: vec![
                vec![
                    Tile::Vacio,
                    Tile::Enemigo(Enemigo::crear(1, 0, 1)),
                    Tile::Vacio,
                ],
                vec![
                    Tile::Vacio,
                    Tile::BombaNormal(Bomba::crear(1, 1, 2, false)),
                    Tile::Vacio,
                ],
                vec![Tile::Vacio, Tile::Vacio, Tile::Vacio],
            ],
        };
        let _ = jugar_turno(&mut mapa, 1, 1);
        assert_eq!(mapa.obtener_tile(1, 1), Some(&Tile::Vacio));
        assert_eq!(mapa.obtener_tile(1, 0), Some(&Tile::Vacio));
        assert_eq!(mapa.obtener_tile(1, 2), Some(&Tile::Vacio));
    }

    #[test]
    fn test_detonar_radio_mayor_a_len() {
        let mut mapa = Mapa {
            side_size: 3,
            tiles: vec![
                vec![
                    Tile::Vacio,
                    Tile::Enemigo(Enemigo::crear(1, 0, 1)),
                    Tile::Vacio,
                ],
                vec![
                    Tile::Vacio,
                    Tile::BombaNormal(Bomba::crear(1, 1, 10, false)),
                    Tile::Vacio,
                ],
                vec![Tile::Vacio, Tile::Vacio, Tile::Vacio],
            ],
        };
        let _ = jugar_turno(&mut mapa, 1, 1);
        assert_eq!(mapa.obtener_tile(1, 1), Some(&Tile::Vacio));
        assert_eq!(mapa.obtener_tile(1, 0), Some(&Tile::Vacio));
        assert_eq!(mapa.obtener_tile(1, 2), Some(&Tile::Vacio));
    }

    #[test]
    fn test_detonar_radio_0() {
        let mut mapa = Mapa {
            side_size: 3,
            tiles: vec![
                vec![
                    Tile::Vacio,
                    Tile::Enemigo(Enemigo::crear(1, 0, 1)),
                    Tile::Vacio,
                ],
                vec![
                    Tile::Vacio,
                    Tile::BombaNormal(Bomba::crear(1, 1, 0, false)),
                    Tile::Vacio,
                ],
                vec![Tile::Vacio, Tile::Vacio, Tile::Vacio],
            ],
        };
        let _ = jugar_turno(&mut mapa, 1, 1);
        assert_ne!(mapa.obtener_tile(1, 0), Some(&Tile::Vacio));
        assert_eq!(mapa.obtener_tile(1, 1), Some(&Tile::Vacio));
        assert_eq!(mapa.obtener_tile(1, 2), Some(&Tile::Vacio));
    }

    #[test]
    fn test_buscar_tiles() {
        let mapa = Mapa {
            side_size: 3,
            tiles: vec![
                vec![Tile::Vacio, Tile::Vacio, Tile::Vacio],
                vec![
                    Tile::Vacio,
                    Tile::BombaNormal(Bomba::crear(1, 1, 2, false)),
                    Tile::Vacio,
                ],
                vec![Tile::Vacio, Tile::Vacio, Tile::Vacio],
            ],
        };
        let tiles_encontradas = buscar_tiles(&mapa, 1, 1, Bomba::crear(1, 1, 2, false));
        assert_eq!(
            tiles_encontradas,
            vec![
                Coordenada { x: 1, y: 2 },
                Coordenada { x: 1, y: 0 },
                Coordenada { x: 2, y: 1 },
                Coordenada { x: 0, y: 1 }
            ]
        );
    }

    #[test]
    fn test_buscar_mapa_vacio() {
        let mapa = Mapa {
            side_size: 0,
            tiles: vec![],
        };
        let tiles_encontradas = buscar_tiles(&mapa, 1, 1, Bomba::crear(1, 1, 2, false));
        assert_eq!(tiles_encontradas, vec![]);
    }

    #[test]
    fn test_buscar_mapa_lleno_piedras() {
        let mapa = Mapa {
            side_size: 3,
            tiles: vec![
                vec![
                    Tile::Piedra(Obstaculo {
                        x: 0,
                        y: 0,
                        pasable: true,
                    }),
                    Tile::Piedra(Obstaculo {
                        x: 1,
                        y: 0,
                        pasable: true,
                    }),
                    Tile::Piedra(Obstaculo {
                        x: 2,
                        y: 0,
                        pasable: true,
                    }),
                ],
                vec![
                    Tile::Piedra(Obstaculo {
                        x: 0,
                        y: 1,
                        pasable: true,
                    }),
                    Tile::BombaNormal(Bomba::crear(1, 1, 2, false)),
                    Tile::Piedra(Obstaculo {
                        x: 2,
                        y: 1,
                        pasable: true,
                    }),
                ],
                vec![
                    Tile::Piedra(Obstaculo {
                        x: 0,
                        y: 2,
                        pasable: true,
                    }),
                    Tile::Piedra(Obstaculo {
                        x: 1,
                        y: 2,
                        pasable: true,
                    }),
                    Tile::Piedra(Obstaculo {
                        x: 2,
                        y: 2,
                        pasable: true,
                    }),
                ],
            ],
        };
        let tiles_encontradas = buscar_tiles(&mapa, 1, 1, Bomba::crear(1, 1, 2, false));
        assert_eq!(tiles_encontradas, vec![]);
    }

    #[test]
    fn test_buscar_mapa_lleno_piedras_bomba_especial() {
        let mapa = Mapa {
            side_size: 3,
            tiles: vec![
                vec![
                    Tile::Piedra(Obstaculo {
                        x: 0,
                        y: 0,
                        pasable: true,
                    }),
                    Tile::Piedra(Obstaculo {
                        x: 1,
                        y: 0,
                        pasable: true,
                    }),
                    Tile::Piedra(Obstaculo {
                        x: 2,
                        y: 0,
                        pasable: true,
                    }),
                ],
                vec![
                    Tile::Piedra(Obstaculo {
                        x: 0,
                        y: 1,
                        pasable: true,
                    }),
                    Tile::BombaEspecial(Bomba::crear(1, 1, 2, true)),
                    Tile::Piedra(Obstaculo {
                        x: 2,
                        y: 1,
                        pasable: true,
                    }),
                ],
                vec![
                    Tile::Piedra(Obstaculo {
                        x: 0,
                        y: 2,
                        pasable: true,
                    }),
                    Tile::Piedra(Obstaculo {
                        x: 1,
                        y: 2,
                        pasable: true,
                    }),
                    Tile::Piedra(Obstaculo {
                        x: 2,
                        y: 2,
                        pasable: true,
                    }),
                ],
            ],
        };
        let tiles_encontradas = buscar_tiles(&mapa, 1, 1, Bomba::crear(1, 1, 2, true));
        assert_eq!(
            tiles_encontradas,
            vec![
                Coordenada { x: 1, y: 2 },
                Coordenada { x: 1, y: 0 },
                Coordenada { x: 2, y: 1 },
                Coordenada { x: 0, y: 1 }
            ]
        );
    }

    #[test]
    fn test_bomba_pasa_dos_veces_por_enemigo() {
        let mut mapa = Mapa {
            side_size: 5,
            tiles: vec![
                vec![
                    Tile::Vacio,
                    Tile::Desvio(Desvio {
                        x: 1,
                        y: 0,
                        direccion: Direccion::Abajo,
                    }),
                    Tile::Vacio,
                    Tile::Vacio,
                    Tile::Vacio,
                ],
                vec![
                    Tile::Vacio,
                    Tile::Enemigo(Enemigo::crear(1, 1, 2)),
                    Tile::Vacio,
                    Tile::Vacio,
                    Tile::Vacio,
                ],
                vec![
                    Tile::Vacio,
                    Tile::BombaNormal(Bomba::crear(1, 2, 4, false)),
                    Tile::Vacio,
                    Tile::Vacio,
                    Tile::Vacio,
                ],
                vec![
                    Tile::Vacio,
                    Tile::Vacio,
                    Tile::Vacio,
                    Tile::Vacio,
                    Tile::Vacio,
                ],
                vec![
                    Tile::Vacio,
                    Tile::Vacio,
                    Tile::Vacio,
                    Tile::Vacio,
                    Tile::Vacio,
                ],
            ],
        };

        let _ = jugar_turno(&mut mapa, 1, 2);
        let enemigo = mapa.obtener_tile(1, 1).unwrap();
        let _ = match enemigo {
            Tile::Enemigo(enemigo) => {
                assert_eq!(enemigo.vida, 1);
            }
            _ => assert_eq!(*enemigo, Tile::Enemigo(Enemigo::crear(1, 1, 2))),
        };
    }
}
