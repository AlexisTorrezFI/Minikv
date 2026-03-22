/// Enum que representa los posibles errores del sistema MiniKV.
///
/// Este tipo se utiliza para modelar fallos durante operaciones de
/// lectura, escritura y validación de datos en los archivos.
///
/// Cada variante representa un tipo específico de error que puede
/// ocurrir durante la ejecución del programa.
#[derive(Debug)]
pub enum ErrorMiniKv {
    /// No se pudo abrir o crear un archivo.
    NoSePudoAbrirArchivo,

    /// No se pudo leer el contenido de un archivo.
    NoSePudoLeerArchivo,

    /// Ocurrió un error al escribir en un archivo.
    NoSePudoEscribirArchivo,

    /// Una línea del archivo no cumple con el formato esperado.
    LineaInvalida,
}

/// Imprime un mensaje de error correspondiente a un `ErrorMiniKv`.
///
/// Esta función traduce cada variante del enum a un mensaje legible
/// para el usuario.
///
/// # Parámetros
///
/// - `e`: error a imprimir.
///
/// # Comportamiento
///
/// - Muestra un mensaje descriptivo por consola según el tipo de error.
/// - No devuelve ningún valor.
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
