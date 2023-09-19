use archivos::print_mapa_to_file;

pub mod archivos;
mod modelo;
mod turno;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 {
        println!("Uso: cargo run -- maze.txt /path/to/output_dir/ x y");
        return;
    }

    let output_dir = match archivos::inicializar_output_dir(&args) {
        Err(why) => {
            println!("No se pudo crear el directorio de salida: {why}");
            return;
        }
        Ok(output_dir) => output_dir,
    };

    println!("Directorio de salida: {:?}", output_dir);

    let mut mapa = match archivos::inicializar_mapa(&args) {
        Err(why) => {
            let _ = archivos::print_err_to_file(
                format!("No se pudo inicializar el mapa: {why}"),
                output_dir,
            );
            return;
        }
        Ok(mapa) => mapa,
    };

    let (x_pos, y_pos) = match archivos::inicializar_posicion(&args) {
        Err(why) => {
            let _ = archivos::print_err_to_file(
                format!("No se pudo inicializar la posicion: {why}"),
                output_dir,
            );
            return;
        }
        Ok((x_pos, y_pos)) => (x_pos, y_pos),
    };

    if let Err(why) = turno::jugar_turno(&mut mapa, x_pos, y_pos) {
        let _ =
            archivos::print_err_to_file(format!("No se pudo jugar el turno: {why}"), output_dir);
        return;
    }
    
    match print_mapa_to_file(&mapa, &output_dir) {
        Err(why) => {
            let _  = archivos::print_err_to_file(format!("No se pudo imprimir el mapa: {why}"), output_dir);
            return;
        }
        Ok(_) => (),
    }
}
