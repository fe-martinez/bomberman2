use crate::modelo::fabrica::crear_pieza;
use crate::modelo::{mapa::Mapa, tile::Tile};
use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
};

/// Lee el archivo de texto en la ruta especificada y devuelve un iterador de lineas.
pub fn read_map(path: &str) -> std::io::Lines<BufReader<File>> {
    let file = match File::open(path) {
        Err(why) => panic!("No se pudo abrir el archivo: {why}"),
        Ok(file) => file,
    };

    let reader: BufReader<File> = BufReader::new(file);
    reader.lines()
}

/// Imprime el mapa en un archivo de texto.
pub fn print_mapa_to_file(mapa: &Mapa, path: &str) -> std::io::Result<()> {
    let mut string: String = String::new();
    for v in mapa.tiles.iter() {
        for t in v.iter() {
            match t {
                Tile::Enemigo(enemigo) => string.push_str(format!("F{}", enemigo.vida).as_str()),
                Tile::BombaNormal(bomba) => string.push_str(format!("B{}", bomba.radio).as_str()),
                Tile::BombaEspecial(bomba) => string.push_str(format!("S{}", bomba.radio).as_str()),
                Tile::Piedra(_) => string.push_str("R"),
                Tile::Pared(_) => string.push_str("W"),
                Tile::Desvio(desvio) => {
                    string.push_str(format!("D{}", desvio.char_direccion()).as_str())
                }
                Tile::Vacio => string.push_str("_"),
            }
            string.push_str(" ");
        }
        string.push_str("\n");
    }
    fs::write(path, string)?;
    return Ok(());
}

pub fn print_mapa_debug(mapa: &Mapa) {
    for v in mapa.tiles.iter() {
        for t in v.iter() {
            match t {
                Tile::Enemigo(enemigo) => print!("F{}", enemigo.vida),
                Tile::BombaNormal(bomba) => print!("B{}", bomba.radio),
                Tile::BombaEspecial(bomba) => print!("S{}", bomba.radio),
                Tile::Piedra(_) => print!("R"),
                Tile::Pared(_) => print!("W"),
                Tile::Desvio(desvio) => print!("D{}", desvio.char_direccion()),
                Tile::Vacio => print!("_"),
            }
            print!(" ");
        }
        print!("\n");
    }
}

/// Transforma una linea de texto en un vector de tiles.
fn transformar_linea(s: String, y_pos: usize) -> Vec<Tile> {
    let caracteres: Vec<&str> = s.split(' ').collect();
    let mut x_pos = 0;

    let mut tiles: Vec<Tile> = Vec::new();

    for caracter in caracteres {
        tiles.push(crear_pieza(caracter, x_pos, y_pos));
        x_pos += 1;
    }
    tiles
}

/// Transforma un archivo de texto en un mapa.
pub fn transformar_a_mapa(path: &str) -> Option<Mapa> {
    let lineas = read_map(path);
    let mut mapa = Mapa {
        tiles: Vec::new(),
        side_size: 0,
    };
    let mut y_pos: usize = 0;

    for linea in lineas {
        match linea {
            Err(why) => {
                println!("No se pudo leer la linea: {why}");
                return None;
            }
            Ok(linea) => {
                let tiles_temp = transformar_linea(linea, y_pos);
                if mapa.side_size == 0 {
                    mapa.side_size = tiles_temp.len();
                }
                mapa.tiles.push(tiles_temp);
                y_pos += 1;
            }
        };
    }

    println!(
        "Se transformo el archivo a mapa, side size: {}",
        mapa.side_size
    );
    return Some(mapa);
}

#[cfg(test)]
mod test {
    use crate::modelo::{bomba::Bomba, enemigo::Enemigo, obstaculo::Obstaculo};

    use super::*;

    #[test]
    fn test_transformar_linea() {
        let linea = "F1 _ _ B3 _ R W".to_string();
        let tiles = transformar_linea(linea, 0);
        assert_eq!(tiles.len(), 7);
        assert_eq!(tiles[0], Tile::Enemigo(Enemigo::crear(0, 0, 1)));
        assert_eq!(tiles[1], Tile::Vacio);
        assert_eq!(tiles[2], Tile::Vacio);
        assert_eq!(tiles[3], Tile::BombaNormal(Bomba::crear(3, 0, 3, false)));
        assert_eq!(tiles[4], Tile::Vacio);
        assert_eq!(tiles[5], Tile::Piedra(Obstaculo::crear(5, 0, false)));
        assert_eq!(tiles[6], Tile::Pared(Obstaculo::crear(6, 0, true)));
    }

    #[test]
    fn test_transformar_a_mapa() {
        let mapa = transformar_a_mapa("mapas/mapa1.txt").unwrap();
        assert_eq!(mapa.side_size, 7);
        assert_eq!(mapa.tiles.len(), 1);
        assert_eq!(mapa.tiles[0][0], Tile::Enemigo(Enemigo::crear(0, 0, 1)));
        assert_eq!(mapa.tiles[0][1], Tile::Vacio);
        assert_eq!(mapa.tiles[0][2], Tile::Vacio);
        assert_eq!(
            mapa.tiles[0][3],
            Tile::BombaNormal(Bomba::crear(3, 0, 3, false))
        );
        assert_eq!(mapa.tiles[0][4], Tile::Vacio);
        assert_eq!(
            mapa.tiles[0][5],
            Tile::Piedra(Obstaculo::crear(5, 0, false))
        );
        assert_eq!(mapa.tiles[0][6], Tile::Pared(Obstaculo::crear(6, 0, true)));
    }

    #[test]
    fn test_print_mapa_to_file() {
        let mapa = transformar_a_mapa("mapas/mapa1.txt").unwrap();
        print_mapa_to_file(&mapa, "mapas/mapa1_test.txt").unwrap();
        let mapa_test = transformar_a_mapa("mapas/mapa1_test.txt").unwrap();
        assert_eq!(mapa, mapa_test);
    }
}
