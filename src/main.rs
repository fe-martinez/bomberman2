use archivos::{print_mapa_debug, print_mapa_to_file};

pub mod archivos;
mod modelo;
mod turno;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 {
        println!("Uso: cargo run -- maze.txt /path/to/output_dir/ x y");
        return;
    }

    let mut mapa = match archivos::transformar_a_mapa(&args[1]) {
        Err(why) => {
            println!("No se pudo transformar el archivo a mapa: {why}");
            return;
        }
        Ok(mapa) => mapa,
    };

    let printprint = match archivos::open_path(&args[2], &args[1]) {
        Err(why) => {
            println!("No se pudo abrir el archivo: {why}");
            return;
        }
        Ok(printprint) => printprint,
    };

    println!("{}", printprint.display());

    let x_pos = match args[3].parse::<usize>() {
        Err(why) => {
            println!("No se pudo parsear el argumento x: {why}");
            return;
        }
        Ok(x_pos) => x_pos,
    };
    let y_pos = match args[4].parse::<usize>() {
        Err(why) => {
            println!("No se pudo parsear el argumento y: {why}");
            return;
        }
        Ok(y_pos) => y_pos,
    };

    print_mapa_debug(&mapa);
    if let Err(why) = turno::jugar_turno(&mut mapa, x_pos, y_pos) {
        println!("No se pudo jugar el turno: {why}");
        return;
    }
    print_mapa_debug(&mapa);

    let resultado = print_mapa_to_file(&mapa, printprint);

    println!("{:?}", resultado);

}
