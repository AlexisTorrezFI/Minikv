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

/// Devuelve un string con el mensaje de error en el formato requerido por el TP.
///
/// Traduce cada variante de `ErrorMiniKv` al formato:
/// `ERROR: [TIPO]`
///
/// # Parámetros
///
/// - `e`: error a obtener el mensaje.
///
/// # Comportamiento
///
/// - Devuelve el mensaje de error correspondiente.
/// - No imprime nada por consola.
pub fn obtener_mensaje(e: ErrorMiniKv) -> &'static str {
    match e {
        ErrorMiniKv::NotFound => "ERROR: NOT FOUND",
        ErrorMiniKv::ExtraArgument => "ERROR: EXTRA ARGUMENT",
        ErrorMiniKv::InvalidDataFile => "ERROR: INVALID DATA FILE",
        ErrorMiniKv::InvalidLogFile => "ERROR: INVALID LOG FILE",
        ErrorMiniKv::MissingArgument => "ERROR: MISSING ARGUMENT",
        ErrorMiniKv::UnknownCommand => "ERROR: UNKNOWN COMMAND",
    }
}
