use crate::modelo::tile::Tile;

use super::{
    bomba::Bomba, desvio::Desvio, direccion::Direccion, enemigo::Enemigo, obstaculo::Obstaculo,
};

/// Crea un enemigo a partir de un string.
/// El string debe tener el siguiente formato: F(numero). Ejemplo: F2.
/// Si el string no tiene el formato correcto, devuelve un error.
fn crear_enemigo(x_pos: usize, y_pos: usize, segundo_caracter: Option<char>) -> Result<Tile, String> {
    let vida = match segundo_caracter {
        Some(c) => c.to_digit(10),
        None => None,
    };
    match vida {
        Some(vida) => if vida > 0 {
            Ok(Tile::Enemigo(Enemigo::crear(x_pos, y_pos, vida)))
        } else {
            Err(format!("Un enemigo no puede tener vida negativa: F{}", vida))
        },
        None => Err("Los enemigos deben tener vida".to_string()),
    }
}

/// Crea una bomba a partir de un string.
/// El string debe tener el siguiente formato: B(numero). Ejemplo: B2.
/// Si el string no tiene el formato correcto, devuelve un error.
fn crear_bomba(x_pos: usize, y_pos: usize, especial: bool, segundo_caracter: Option<char>) -> Result<Tile, String> {
    let radio = match segundo_caracter {
        Some(c) => c.to_digit(10),
        None => None,
    };
    match radio {
        Some(radio) => 
            if radio > 0 {
                let bomba = Bomba::crear(x_pos, y_pos, radio, especial);
                if especial {
                    Ok(Tile::BombaEspecial(bomba))
                } else {
                    Ok(Tile::BombaNormal(bomba))
                }
            } else {
                Err(format!("Una bomba no puede tener radio negativo: B{}", radio))
        },
        None => Err("Una bomba debe tener radio, B".to_string()),
    }
}

/// Crea un desvio a partir de un string.
/// El string debe tener el siguiente formato: D(direccion). Ejemplo: DU.
/// Si el string no tiene el formato correcto, devuelve un error.
fn crear_desvio(x_pos: usize, y_pos: usize, segundo_caracter: Option<char>) -> Result<Tile, String> {
    match segundo_caracter {
        None => Err("Un desvio debe tener direccion, D".to_string()),
        Some(direccion) => {
            let direccion = match direccion {
                'U' => Direccion::Arriba,
                'D' => Direccion::Abajo,
                'L' => Direccion::Izquierda,
                'R' => Direccion::Derecha,
                _ => return Err(format!("Direccion invalida: D{}", direccion)),
            };
            Ok(Tile::Desvio(Desvio {
                x: x_pos,
                y: y_pos,
                direccion,
            }))
        }
    }
}

/// Crea una pieza a partir de un string.
/// El string debe tener el siguiente formato: <tipo><numero>. Ejemplo: F2, B2, S2, R, W, D2, _.
/// Si el string no tiene el formato correcto, devuelve un error.
pub fn crear_pieza(s: &str, x_pos: usize, y_pos: usize) -> Result<Tile, String> {
    let primer_caracter = s.chars().next();
    let segundo_caracter = s.chars().nth(1);
    match primer_caracter {
        Some('F') => crear_enemigo(x_pos, y_pos, segundo_caracter),
        Some('B') => crear_bomba(x_pos, y_pos, false, segundo_caracter),
        Some('S') => crear_bomba(x_pos, y_pos, true, segundo_caracter),
        Some('R') => Ok(Tile::Piedra(Obstaculo::crear(x_pos, y_pos, false))),
        Some('W') => Ok(Tile::Pared(Obstaculo::crear(x_pos, y_pos, true))),
        Some('D') => crear_desvio(x_pos, y_pos, segundo_caracter),
        Some('_') => Ok(Tile::Vacio),
        _ => Err(format!("No se pudo crear la pieza: >{}<", s)),
    }
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
        assert_eq!(tile, Err("Los enemigos deben tener vida".to_string()));

        let tile = super::crear_pieza("F2", 0, 0);
        assert_eq!(tile, Ok(Tile::Enemigo(Enemigo::crear(0, 0, 2))));

        let tile = super::crear_pieza("B", 0, 0);
        assert_eq!(tile, Err("Una bomba debe tener radio, B".to_string()));

        let tile = super::crear_pieza("B2", 0, 0);
        assert_eq!(tile, Ok(Tile::BombaNormal(Bomba::crear(0, 0, 2, false))));

        let tile = super::crear_pieza("S2", 0, 0);
        assert_eq!(tile, Ok(Tile::BombaEspecial(Bomba::crear(0, 0, 2, true))));

        let tile = super::crear_pieza("R", 0, 0);
        assert_eq!(tile, Ok(Tile::Piedra(Obstaculo::crear(0, 0, false))));

        let tile = super::crear_pieza("W", 0, 0);
        assert_eq!(tile, Ok(Tile::Pared(Obstaculo::crear(0, 0, true))));

        let tile = super::crear_pieza("D", 0, 0);
        assert_eq!(tile.is_err(), true);

        let tile = super::crear_pieza("DU", 0, 0);
        assert_eq!(
            tile,
            Ok(Tile::Desvio(Desvio {
                x: 0,
                y: 0,
                direccion: Direccion::Arriba
            }))
        );

        let tile = super::crear_pieza("_", 0, 0);
        assert_eq!(tile, Ok(Tile::Vacio));
    }
}
