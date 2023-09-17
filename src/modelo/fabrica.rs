use crate::modelo::tile::Tile;

use super::{
    bomba::Bomba,
    desvio::Desvio,
    enemigo::Enemigo,
    obstaculo::Obstaculo,
    direccion::Direccion,
};

/// Crea una pieza a partir de un string.
/// El string debe tener el siguiente formato: <tipo><numero>. Ejemplo: F2, B2, S2, R, W, D2, _.
pub fn crear_pieza(s: &str, x_pos: usize, y_pos: usize) -> Tile {
    let primer_caracter = s.chars().next();
    let numero = match s.chars().nth(1) {
        None => 1,
        Some(numero) => numero.to_digit(10).unwrap_or(1),
    };

    let tile = match primer_caracter {
        Some('F') => Tile::Enemigo(Enemigo::crear(x_pos, y_pos, numero)),
        Some('B') => Tile::BombaNormal(Bomba::crear(x_pos, y_pos, numero, false)),
        Some('S') => Tile::BombaEspecial(Bomba::crear(x_pos, y_pos, numero, true)),
        Some('R') => Tile::Piedra(Obstaculo::crear(x_pos, y_pos, false)),
        Some('W') => Tile::Pared(Obstaculo::crear(x_pos, y_pos, true)),
        Some('D') => match s.chars().nth(1) {
            None => Tile::Vacio,
            Some(direccion) => {
                let direccion = match direccion {
                    'U' => Direccion::Arriba,
                    'D' => Direccion::Abajo,
                    'L' => Direccion::Izquierda,
                    'R' => Direccion::Derecha,
                    _ => Direccion::Arriba,
                };
                Tile::Desvio(Desvio {
                    x: x_pos,
                    y: y_pos,
                    direccion,
                })
            }
        },
        Some('_') => Tile::Vacio,
        _ => Tile::Vacio,
    };

    tile
}

#[cfg(test)]
mod test {
    use crate::modelo::desvio::Desvio;
    use crate::modelo::direccion::Direccion;
    use crate::modelo::tile::Tile;
    use crate::modelo::{bomba::Bomba, enemigo::Enemigo, obstaculo::Obstaculo};

    #[test]
    fn test_crear_pieza() {
        let tile = super::crear_pieza("F", 0, 0);
        assert_eq!(tile, Tile::Enemigo(Enemigo::crear(0, 0, 1)));

        let tile = super::crear_pieza("F2", 0, 0);
        assert_eq!(tile, Tile::Enemigo(Enemigo::crear(0, 0, 2)));

        let tile = super::crear_pieza("B", 0, 0);
        assert_eq!(tile, Tile::BombaNormal(Bomba::crear(0, 0, 1, false)));

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
        assert_eq!(
            tile,
            Tile::Desvio(Desvio {
                x: 0,
                y: 0,
                direccion: Direccion::Arriba
            })
        );

        let tile = super::crear_pieza("_", 0, 0);
        assert_eq!(tile, Tile::Vacio);
    }
}
