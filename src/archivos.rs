use crate::modelo::fabrica::crear_pieza;
use crate::modelo::{mapa::Mapa, tile::Tile};
use std::io::{self, Write};
use std::path::PathBuf;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn inicializar_mapa(argumentos: &Vec<String>) -> Result<Mapa, String> {
    match transformar_a_mapa(&argumentos[1]) {
        Ok(mapa) => Ok(mapa),
        Err(why) => Err(format!("No se pudo transformar el archivo a mapa: {}", why)),
    }
}

pub fn inicializar_output_dir(argumentos: &Vec<String>) -> Result<PathBuf, String> {
    match open_path(&argumentos[2], &argumentos[1]) {
        Err(why) => Err(format!("No se pudo abrir el archivo: {}", why)),
        Ok(output_dir) => {
            Ok(output_dir)
        }
    }
}

pub fn inicializar_posicion(argumentos: &Vec<String>) -> Result<(usize, usize), String> {
    let x_pos = match argumentos[3].parse::<usize>() {
        Err(why) => {
            return Err(format!("No se pudo parsear el argumento x: {}", why));
        }
        Ok(x_pos) => x_pos,
    };
    let y_pos = match argumentos[4].parse::<usize>() {
        Err(why) => {
            return Err(format!("No se pudo parsear el argumento y: {}", why));
        }
        Ok(y_pos) => y_pos,
    };
    Ok((x_pos, y_pos))
}

/// Lee el archivo de texto en la ruta especificada y devuelve un iterador de lineas.
/// Si no se pudo abrir el archivo, devuelve un error.
pub fn read_file(path: &str) -> Result<io::Lines<BufReader<File>>, &str> {
    match File::open(path) {
        Ok(file) => Ok(BufReader::new(file).lines()),
        Err(_) => Err("No se pudo abrir el archivo"),
    }
}

/// Transforma un archivo de texto en un mapa.
/// Si no se pudo transformar el archivo, devuelve un error.
pub fn transformar_a_mapa(path: &str) -> Result<Mapa, &str> {
    let lineas = read_file(path)?;
    let mut mapa = Mapa {
        tiles: Vec::new(),
        side_size: 0,
    };
    let mut y_pos: usize = 0;

    for linea in lineas {
        let _ = match linea {
            Err(_) => Err("No se pudo leer la linea"),
            Ok(linea) => {
                {
                    let tiles_temp = transformar_linea(linea, y_pos);
                    if mapa.side_size == 0 {
                        mapa.side_size = tiles_temp.len();
                    }
                    y_pos += 1;
                    if tiles_temp.len() != mapa.side_size {
                        return Err("El mapa no es cuadrado");
                    }
                    mapa.tiles.push(tiles_temp);
                };
                Ok(())
            }
        };
    }
    Ok(mapa)
}

pub fn open_path(carpeta: &str, filename: &str) -> io::Result<PathBuf> {
    let path = PathBuf::from(carpeta);
    if let Ok(path_relativo) = path.strip_prefix("/") {
        let mut directorio = std::env::current_dir()?;
        print!("Directorio: {:?}", directorio);
        directorio.push(path_relativo);
        directorio.push(filename);
        if !directorio.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No existe el directorio",
            ))?;
        } else {
            return Ok(directorio);
        }
    } else {
        let mut directorio = std::env::current_dir()?;
        print!("Directorio: {:?}", directorio);
        directorio.push(path);
        directorio.push(filename);
        if !directorio.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No existe el directorio",
            ))?;
        } else {
            return Ok(directorio);
        }
    }
}

/// Imprime el mapa en un archivo de texto.
pub fn print_mapa_to_file(mapa: &Mapa, path: &PathBuf) -> std::io::Result<()> {
    let mut string: String = String::new();
    for v in mapa.tiles.iter() {
        for t in v.iter() {
            match t {
                Tile::Enemigo(enemigo) => string.push_str(format!("F{}", enemigo.vida).as_str()),
                Tile::BombaNormal(bomba) => string.push_str(format!("B{}", bomba.radio).as_str()),
                Tile::BombaEspecial(bomba) => string.push_str(format!("S{}", bomba.radio).as_str()),
                Tile::Piedra(_) => string.push('R'),
                Tile::Pared(_) => string.push('W'),
                Tile::Desvio(desvio) => {
                    string.push_str(format!("D{}", desvio.char_direccion()).as_str())
                }
                Tile::Vacio => string.push('_'),
            }
            string.push(' ');
        }
        string.push('\n');
    }

    let mut file = File::create(path)?;
    file.write(string.as_bytes())?;
    Ok(())
}

pub fn print_err_to_file(err: String, path: PathBuf) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write(err.as_bytes())?;
    Ok(())
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
        println!()
    }
}

/// Transforma una linea de texto en un vector de tiles.
fn transformar_linea(s: String, y_pos: usize) -> Vec<Tile> {
    let caracteres: Vec<&str> = s.split(' ').filter(|x| !x.is_empty()).collect();
    println!("Caracteres: {:?}", caracteres);
    let mut tiles: Vec<Tile> = Vec::new();

    for (x_pos, caracter) in caracteres.into_iter().enumerate() {
        tiles.push(crear_pieza(caracter, x_pos, y_pos));
    }
    tiles
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
        let mapa = transformar_a_mapa("mapas/mapa_test_crear.txt").unwrap();
        assert_eq!(mapa.side_size, 7);
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
        let mapa = transformar_a_mapa("mapas/mapa_test_crear.txt").unwrap();
        let path = PathBuf::from("mapas/mapa_test_crear.txt");
        let _ = print_mapa_to_file(&mapa, &path);
        let mapa2 = transformar_a_mapa("mapas/mapa_test_crear.txt").unwrap();
        assert_eq!(mapa, mapa2);
    }

    #[test]
    fn test_abre_directorio_existente() {
        let path = open_path("mapas", "mapa_test_crear.txt").unwrap();
        assert_eq!(path.exists(), true);
    }

    #[test]
    fn test_no_abre_directorio_no_existente() {
        let path = open_path("no_mapas", "mapa1.txt");
        assert_eq!(path.is_err(), true);
    }
}
