/// Enum que representa los posibles errores del sistema MiniKV.
///
/// Este tipo modela los errores permitidos por la especificación del TP.
/// Cada variante corresponde a un código de error válido que debe ser
/// mostrado al usuario en el formato `ERROR: [TIPO]`.
#[derive(Debug, PartialEq, Eq)]
pub enum ErrorMiniKv {
    /// La clave solicitada no existe.
    NotFound,

    /// Se proporcionaron más argumentos de los esperados.
    ExtraArgument,

    /// El archivo `.minikv.data` es inválido o no se pudo procesar.
    InvalidDataFile,

    /// El archivo `.minikv.log` es inválido o no se pudo procesar.
    InvalidLogFile,

    /// Faltan argumentos requeridos para el comando.
    MissingArgument,

    /// El comando ingresado no es reconocido.
    UnknownCommand,
}

/// Imprime un mensaje de error en el formato requerido por el TP.
///
/// Traduce cada variante de `ErrorMiniKv` al formato:
/// `ERROR: [TIPO]`
///
/// # Parámetros
///
/// - `e`: error a imprimir.
///
/// # Comportamiento
///
/// - Imprime el código de error correspondiente por consola.
/// - No devuelve ningún valor.
pub fn imprimir_error(e: ErrorMiniKv) {
    match e {
        ErrorMiniKv::NotFound => println!("ERROR: NOT FOUND"),
        ErrorMiniKv::ExtraArgument => println!("ERROR: EXTRA ARGUMENT"),
        ErrorMiniKv::InvalidDataFile => println!("ERROR: INVALID DATA FILE"),
        ErrorMiniKv::InvalidLogFile => println!("ERROR: INVALID LOG FILE"),
        ErrorMiniKv::MissingArgument => println!("ERROR: MISSING ARGUMENT"),
        ErrorMiniKv::UnknownCommand => println!("ERROR: UNKNOWN COMMAND"),
    }
}