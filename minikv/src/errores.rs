pub enum ErrorMiniKv {
    NoSePudoAbrirArchivo,
    NoSePudoLeerArchivo,
    NoSePudoEscribirArchivo,
    LineaInvalida,
}

pub fn imprimir_error(e: ErrorMiniKv) {
    match e {
        ErrorMiniKv::NoSePudoAbrirArchivo => {
            println!("Error: no se pudo abrir el archivo");
        }
        ErrorMiniKv::NoSePudoLeerArchivo => {
            println!("Error: no se pudo leer el archivo");
        }
        ErrorMiniKv::NoSePudoEscribirArchivo => {
            println!("Error: no se pudo escribir el archivo");
        }
        ErrorMiniKv::LineaInvalida => {
            println!("Error: línea inválida en el archivo");
        }
    }
}