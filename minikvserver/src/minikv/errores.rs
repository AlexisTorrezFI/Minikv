/// Enum que representa los posibles errores del sistema MiniKV.
///
/// Este tipo modela los errores permitidos por la especificación del TP.
/// Cada variante corresponde a un código de error válido que debe ser
/// mostrado al usuario en el formato `ERROR: [TIPO]`.
#[derive(Debug, PartialEq, Eq)]
pub enum ErrorCliente {
    /// La clave solicitada no existe.
    NotFound,

    /// Se proporcionaron más argumentos de los esperados.
    ExtraArgument,

    /// Faltan argumentos requeridos para el comando.
    MissingArgument,

    /// El comando ingresado no es reconocido.
    UnknownCommand,
}
#[derive(Debug, PartialEq, Eq)]
pub enum ErrorServidorDatos {
    /// El archivo `.minikv.data` es inválido o no se pudo procesar.
    InvalidDataFile,

    /// El archivo `.minikv.log` es inválido o no se pudo procesar.
    InvalidLogFile,

}
#[derive(Debug, PartialEq, Eq)]
pub enum ErrorServidorConexion {
    // No se reciben los argumentos esperados en la ejecución del server.
    InvalidArgs, 

    //El servidor no puede bindear un socket en la dirección especificada.
    ServerSocketBinding,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorComunicacion {
    /// El server tarda demasiado en contestar, lo cual puede indicar que está caído.
    Timeout,

    /// La conexión se cierra de forma repentina.
    ConnectionClosed,

    // El cliente no puede bindear un socket en la dirección especificada del server.
    ClientSocketBinding, 
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
pub fn obtener_mensaje_de_error_cliente(e: ErrorCliente) -> &'static str {
    match e {
        ErrorCliente::NotFound => "ERROR: NOT FOUND",
        ErrorCliente::ExtraArgument => "ERROR: EXTRA ARGUMENT",
        ErrorCliente::MissingArgument => "ERROR: MISSING ARGUMENT",
        ErrorCliente::UnknownCommand => "ERROR: UNKNOWN COMMAND",
    }
}
pub fn obtener_mensaje_de_error_comunicacion(e: ErrorComunicacion) -> &'static str {
    match e {
        ErrorComunicacion::Timeout => "ERROR: TIMEOUT",
        ErrorComunicacion::ConnectionClosed => "ERROR: CONNECTION CLOSED",
        ErrorComunicacion::ClientSocketBinding => "ERROR: CLIENT SOCKET BINDING",
    }
}

pub fn obtener_mensaje_de_error_servidor_datos(e: ErrorServidorDatos) -> &'static str {
    match e {
        ErrorServidorDatos::InvalidDataFile => "ERROR: INVALID DATA FILE",
        ErrorServidorDatos::InvalidLogFile => "ERROR: INVALID LOG FILE",
    }
}


pub fn obtener_mensaje_de_error_servidor_conexion(e: ErrorServidorConexion) -> &'static str {
    match e {
        ErrorServidorConexion::InvalidArgs => "ERROR: INVALID ARGS",
        ErrorServidorConexion::ServerSocketBinding => "ERROR: SERVER SOCKET BINDING",
    }
}