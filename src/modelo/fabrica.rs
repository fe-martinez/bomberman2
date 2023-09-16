use crate::modelo::tile::Tile;

use super::{enemigo::Enemigo, bomba::Bomba, obstaculo::Obstaculo, desvio::{Desvio, Direccion}};

pub fn crear_pieza(s: &str, x_pos: usize, y_pos: usize) -> Tile {
    let primer_caracter = s.chars().nth(0);
    let segundo_caracter = s.chars().nth(1);

    let tile = match primer_caracter {
        Some('F') => match segundo_caracter {
            None => Tile::Enemigo(Enemigo::crear(x_pos, y_pos, 1)),
            Some(numero) => Tile::Enemigo(Enemigo::crear(x_pos, y_pos, numero.to_digit(10).unwrap())),
        },
        Some('B') => match segundo_caracter {
            None => Tile::Vacio,
            Some(numero) => Tile::BombaNormal(Bomba::crear(x_pos, y_pos, numero.to_digit(10).unwrap(), false)),
        },
        Some('S') => match segundo_caracter {
            None => Tile::Vacio,
            Some(numero) => Tile::BombaEspecial(Bomba::crear(x_pos, y_pos, numero.to_digit(10).unwrap(), true)),
        },
        Some('R') => Tile::Piedra(Obstaculo::crear(x_pos, y_pos, false)),
        Some('W') => Tile::Pared(Obstaculo::crear(x_pos, y_pos, true)),
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

#[cfg(test)]
mod test {
    use crate::modelo::{enemigo::Enemigo, bomba::Bomba, obstaculo::Obstaculo};
    use crate::modelo::tile::Tile;
    use crate::modelo::desvio::{Desvio, Direccion};

    #[test]
    fn test_crear_pieza() {
        let tile = super::crear_pieza("F", 0, 0);
        assert_eq!(tile, Tile::Enemigo(Enemigo::crear(0, 0, 1)));

        let tile = super::crear_pieza("F2", 0, 0);
        assert_eq!(tile, Tile::Enemigo(Enemigo::crear(0, 0, 2)));

        let tile = super::crear_pieza("B", 0, 0);
        assert_eq!(tile, Tile::Vacio);

        let tile = super::crear_pieza("B2", 0, 0);
        assert_eq!(tile, Tile::BombaNormal(Bomba::crear(0, 0, 2, false)));

        let tile = super::crear_pieza("S2", 0, 0);
        assert_eq!(tile, Tile::BombaEspecial(Bomba::crear(0, 0, 2, true)));

        let tile = super::crear_pieza("R", 0, 0);
        assert_eq!(tile, Tile::Piedra(Obstaculo::crear(0, 0, false)));

        let tile = super::crear_pieza("W", 0, 0);
        assert_eq!(tile, Tile::Pared(Obstaculo::crear(0, 0, true)));

        let tile = super::crear_pieza("D", 0, 0);
        assert_eq!(tile, Tile::Vacio);

        let tile = super::crear_pieza("D2", 0, 0);
        assert_eq!(tile, Tile::Desvio(Desvio {x: 0, y: 0, direccion: Direccion::Arriba}));

        let tile = super::crear_pieza("_", 0, 0);
        assert_eq!(tile, Tile::Vacio);
    }
}