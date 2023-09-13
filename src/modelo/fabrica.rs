use crate::modelo::tile::Tile;

use super::{enemigo::Enemigo, bomba::Bomba, obstaculo::Obstaculo, desvio::{Desvio, Direccion}};

pub fn crear_pieza(s: &str, x_pos: usize, y_pos: usize) -> Tile {
    let primer_caracter = s.chars().nth(0);
    let segundo_caracter = s.chars().nth(1);

    let tile = match primer_caracter {
        Some('F') => match segundo_caracter {
            None => Tile::Enemigo(Enemigo {x: x_pos, y: y_pos, vida: 1}),
            Some(numero) => Tile::Enemigo(Enemigo {x: x_pos, y: y_pos, vida: numero.to_digit(10).unwrap()}),
        },
        Some('B') => match segundo_caracter {
            None => Tile::Vacio,
            Some(numero) => Tile::BombaNormal(Bomba {x: x_pos, y: y_pos, radio: numero.to_digit(10).unwrap(), especial: false, enemigos_afectados: Vec::new() }),
        },
        Some('S') => match segundo_caracter {
            None => Tile::Vacio,
            Some(numero) => Tile::BombaEspecial(Bomba {x: x_pos, y: y_pos, radio: numero.to_digit(10).unwrap(), especial: true, enemigos_afectados: Vec::new()}),
        },
        Some('R') => Tile::Piedra(Obstaculo {x: x_pos, y: y_pos, pasable: true }),
        Some('W') => Tile::Pared(Obstaculo {x: x_pos, y: y_pos, pasable: false }),
        Some('D') => match segundo_caracter {
            None => Tile::Vacio,
            Some(direccion) => {
                let direccion = match direccion {
                    'U' => Direccion::Arriba,
                    'D' => Direccion::Abajo,
                    'L' => Direccion::Izquierda,
                    'R' => Direccion::Derecha,
                    _ => Direccion::Arriba,
                };
                Tile::Desvio(Desvio {x: x_pos, y: y_pos, direccion: direccion})
            },
        },
        Some('_') => Tile::Vacio,
        _ => Tile::Vacio,
    };

    return tile;
}