use crate::modelo::constantes::{ENEMIGO, BOMBA_NORMAL, BOMBA_ESPECIAL, PIEDRA, PARED, DESVIO, VACIO};
use crate::modelo::fabrica::crear_pieza;
use crate::modelo::mapa::Mapa;
use crate::modelo::tile::Tile;
use crate::turno;
use std::io::{self, Write};
use std::path::PathBuf;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// Lee un archivo y devuelve un mapa listo para jugar.
/// Si el archivo no existe o tiene el formato incorrecto, devuelve un error
/// Los tiles deben tener todos el formato correcto.
fn inicializar_mapa(path: &str) -> Result<Mapa, String> {
    match transformar_a_mapa(path) {
        Ok(mapa) => Ok(mapa),
        Err(why) => Err(format!("No se pudo transformar el archivo a mapa: {}", why)),
    }
}

/// Abre un archivo en la ruta especificada por linea de comando y lo devuelve.
/// Si el directorio no existe, no abre el archivo y devuelve un error.
/// Si el archivo no existe, lo crea.
/// Si el archivo existe, lo sobreescribe.
pub fn inicializar_output_dir(argumentos: &[String]) -> Result<File, String> {
    match open_path(&argumentos[2], &argumentos[1]) {
        Err(why) => Err(format!("No se pudo abrir el archivo: {}", why)),
        Ok(output_file) => Ok(output_file),
    }
}

/// Parsea los valores dados por linea de comando, devuelve una tupla si se pudo parsear.
fn inicializar_posicion(argumentos: &[String]) -> Result<(usize, usize), String> {
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


/// Juega un turno del juego a partir del archivo dado como input.
/// Devuelve el mapa resultante del turno.
/// Si el archivo no existe o tiene el formato incorrecto, devuelve un error.
pub fn jugar(argumentos: &[String]) -> Result<Mapa, String> {
    let mut mapa = inicializar_mapa(&argumentos[1])?;
    let (x_pos, y_pos) = inicializar_posicion(argumentos)?;
    turno::jugar_turno(&mut mapa, x_pos, y_pos)?;
    Ok(mapa)
}

/// Lee el archivo de texto en la ruta especificada y devuelve un iterador de lineas.
/// Si no se pudo abrir el archivo, devuelve un error.
fn read_file(path: &str) -> Result<io::Lines<BufReader<File>>, &str> {
    match File::open(path) {
        Ok(file) => Ok(BufReader::new(file).lines()),
        Err(_) => Err("No se pudo abrir el archivo"),
    }
}

/// Transforma una linea de texto en un vector de tiles.
/// Para poder ser transformada, toda la linea tiene que venir con el formato correcto:
/// <tipo<numero> <tipo><numero> <tipo><numero>
/// Si no se pudo transformar la linea, devuelve un error.
fn transformar_linea(s: String, y_pos: usize) -> Result<Vec<Tile>, String> {
    let caracteres: Vec<&str> = s.trim().split(' ').filter(|x| !x.is_empty()).collect();
    println!("Caracteres: {:?}", caracteres);
    let mut tiles: Vec<Tile> = Vec::new();

    for (x_pos, caracter) in caracteres.into_iter().enumerate() {
        let tile = crear_pieza(caracter, x_pos, y_pos);
        match tile {
            Ok(tile) => tiles.push(tile),
            Err(why) => return Err(why),
        }
    }
    Ok(tiles)
}

/// Transforma un archivo de texto en un mapa.
/// Si no se pudo transformar el archivo, devuelve un error.
pub fn transformar_a_mapa(path: &str) -> Result<Mapa, String> {
    let lineas = read_file(path)?;
    let mut mapa = Mapa::crear();
    let mut alto: usize = 0;

    for(y_pos, linea) in lineas.into_iter().enumerate() {
        match linea {
            Err(_) => return Err("No se pudo leer la linea".to_string()),
            Ok(linea) => {
                let tiles_temp = transformar_linea(linea, y_pos)?;
                if mapa.side_size == 0 {
                    mapa.side_size = tiles_temp.len();
                }

                if tiles_temp.len() != mapa.side_size {
                    return Err("El mapa no es cuadrado".to_string());
                }

                mapa.tiles.push(tiles_temp);
            }
        }
        alto = y_pos;
    }

    if mapa.side_size != alto + 1 {
        return Err("El mapa no es cuadrado".to_string());
    }

    Ok(mapa)
}

/// Abre un archivo en la ruta especificada.
/// Si el directorio no existe, no abre el archivo y devuelve un error.
/// Si el archivo no existe, lo crea.
fn open_path(carpeta: &str, filename: &str) -> io::Result<File> {
    let mut directorio = PathBuf::new();
    let carpeta_path = PathBuf::from(carpeta);

    if carpeta_path.is_absolute() {
        directorio.push(carpeta_path);
    } else {
        directorio.push(std::env::current_dir()?);
        directorio.push(carpeta);
    }

    // No se hace mencion en el enunciado de que hacer en caso de que no exista el directorio
    // Rust permite crear un directorio facilmente pero seria algo 'exploitable' el poder crear infinitos directorios desde la linea de comandos
    if directorio.exists() {
        directorio.push(filename);
        File::create(directorio)
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "No existe el directorio. Si el directorio es absoluto se debe pasar con / al principio, si es relativo se debe pasar sin /."))
    }
}

/// Imprime el mapa en un archivo de texto.
/// Los caracteres usados para representar cada tile estan puestos aca para separarlos del modelo.
pub fn print_mapa_to_file(mapa: &Mapa, file: &mut File) {
    let mut string: String = String::new();
    for v in mapa.tiles.iter() {
        for t in v.iter() {
            match t {
                Tile::Enemigo(enemigo) => string.push_str(format!("{}{}", ENEMIGO, enemigo.vida).as_str()),
                Tile::BombaNormal(bomba) => string.push_str(format!("{}{}", BOMBA_NORMAL, bomba.radio).as_str()),
                Tile::BombaEspecial(bomba) => string.push_str(format!("{}{}", BOMBA_ESPECIAL, bomba.radio).as_str()),
                Tile::Piedra(_) => string.push(PIEDRA),
                Tile::Pared(_) => string.push(PARED),
                Tile::Desvio(desvio) => {
                    string.push_str(format!("{}{}", DESVIO, desvio.char_direccion()).as_str())
                }
                Tile::Vacio => string.push(VACIO),
            }
            string.push(' ');
        }
        string.push('\n');
    }

    let _ = file.write(string.as_bytes());
}

/// Imprime un error en un archivo de texto.
pub fn print_err_to_file(err: String, mut file: File) -> std::io::Result<()> {
    let error_completo = format!("ERROR[{}]", err);
    file.write_all(error_completo.as_bytes())?;
    Ok(())
}

/// Imprime el mapa en la consola.
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

#[cfg(test)]
mod test {
    use crate::modelo::{bomba::Bomba, enemigo::Enemigo, obstaculo::Obstaculo};

    use super::*;

    #[test]
    fn test_transformar_linea() {
        let linea = "F1 _ _ B3 _ R W".to_string();
        let tiles = transformar_linea(linea, 0);
        assert_eq!(tiles.is_ok(), true);
        let tiles = tiles.unwrap();
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
        let mapa = transformar_a_mapa("mapas/mapa_test_transformar.txt");
        assert_eq!(mapa.is_ok(), true);
        let mapa = mapa.unwrap();
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
        let mut file = open_path("mapas", "mapa_test_guardar.txt").unwrap();
        let _ = print_mapa_to_file(&mapa, &mut file);
        let mapa2 = transformar_a_mapa("mapas/mapa_test_guardar.txt").unwrap();
        assert_eq!(mapa, mapa2);
    }

    #[test]
    fn test_abre_directorio_existente() {
        let file = open_path("mapas", "mapa_test_guardar.txt");
        assert_eq!(file.is_ok(), true);
    }

    #[test]
    fn test_no_abre_directorio_no_existente() {
        let file = open_path("no_mapas", "mapa1.txt");
        assert_eq!(file.is_err(), true);
    }

    #[test]
    fn test_mapa_no_cuadrado() {
        let mapa = transformar_a_mapa("mapas/mapa_test_no_cuadrado.txt");
        assert_eq!(mapa.is_err(), true);
    }

}
