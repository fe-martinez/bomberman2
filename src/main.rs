use bomberman::bomberman_game;
use bomberman::bomberman_game::{jugar, print_mapa_to_file};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 {
        println!("Uso: cargo run -- maze.txt /path/to/output_dir/ x y");
        return;
    }
    let mut output_file = match bomberman_game::inicializar_output_dir(&args) {
        Err(why) => {
            println!("No se pudo abrir el directorio de salida: {why}");
            return;
        }
        Ok(output_dir) => output_dir,
    };

    match jugar(&args) {
        Err(why) => {
            println!("Error al ejecutar, dirijase al archivo de output");
            let _ =
                bomberman_game::print_err_to_file(format!("No se pudo jugar: {why}"), output_file);
        }
        Ok(mapa) => {
            println!("Juego terminado satisfactoriamente, dirijase al archivo de output");
            print_mapa_to_file(&mapa, &mut output_file);
        }
    };
}
