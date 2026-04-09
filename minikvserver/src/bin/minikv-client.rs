use std::io::{self, BufRead, BufReader, ErrorKind, Write};
use std::net::TcpStream;
use std::time::Duration;
use std::sync::Arc;

use minikvserver::minikv::errores::{self, ErrorComunicacion, ErrorServidorConexion};

const TIMEOUT_SEGUNDOS: u64 = 200;

fn main() {
    let direccion = match obtener_direccion() {
        Ok(direccion) => direccion,
        Err(error) => {
            println!(
                "{}",
                errores::obtener_mensaje_de_error_servidor_conexion(error)
            );
            return;
        }
    };

    let (reader_stream, writer) = match conectar_y_preparar(&direccion) {
    Ok(conexion) => conexion,
    Err(error) => {
        println!(
            "{}",
            errores::obtener_mensaje_de_error_comunicacion(error)
        );
        return;
    }
    };

    ejecutar_cliente(&reader_stream, &writer);
}

fn obtener_direccion() -> Result<String, ErrorServidorConexion> {
    let mut args = std::env::args();

    let _ = args.next();
    let Some(direccion) = args.next() else {
        return Err(ErrorServidorConexion::InvalidArgs);
    };

    if args.next().is_some() {
        return Err(ErrorServidorConexion::InvalidArgs);
    }

    Ok(direccion)
}
fn conectar_y_preparar(
    direccion: &str,
) -> Result<(Arc<TcpStream>, Arc<TcpStream>), ErrorComunicacion> {
    let stream = TcpStream::connect(direccion)
        .map_err(|_| ErrorComunicacion::ClientSocketBinding)?;

    configurar_timeouts(&stream)?;

    let stream = Arc::new(stream);

    Ok((Arc::clone(&stream), stream))
}

fn enviar_operacion(writer: &Arc<TcpStream>, linea: &str) -> io::Result<()> {
    let mut stream = &**writer;
    stream.write_all(format!("{linea}\n").as_bytes())
}

fn configurar_timeouts(stream: &TcpStream) -> Result<(), ErrorComunicacion> {
    stream
        .set_read_timeout(Some(Duration::from_secs(TIMEOUT_SEGUNDOS)))
        .map_err(|_| ErrorComunicacion::ConnectionClosed)?;

    stream
        .set_write_timeout(Some(Duration::from_secs(TIMEOUT_SEGUNDOS)))
        .map_err(|_| ErrorComunicacion::ConnectionClosed)?;

    Ok(())
}

fn ejecutar_cliente(reader_stream: &Arc<TcpStream>, writer: &Arc<TcpStream>) {
    let stdin = io::stdin();

    for linea in stdin.lock().lines() {
        let Ok(linea) = linea else {
            return;
        };

        if let Err(e) = enviar_operacion(writer, &linea) {
            imprimir_error_comunicacion(&e);
            return;
        }

        if let Err(e) = recibir_respuesta(reader_stream) {
            imprimir_error_comunicacion(&e);
            return;
        }
    }
}


fn recibir_respuesta(reader_stream: &Arc<TcpStream>) -> io::Result<()> {
    let mut stream_ref = &**reader_stream;
    let mut reader = BufReader::new(&mut stream_ref);

    let mut respuesta = String::new();
    let bytes_leidos = reader.read_line(&mut respuesta)?;

    if bytes_leidos == 0 {
        return Err(io::Error::new(
            ErrorKind::ConnectionAborted,
            "conexion cerrada",
        ));
    }

    print!("{respuesta}");
    Ok(())
}

fn imprimir_error_comunicacion(error: &io::Error) {
    let error_comunicacion = match error.kind() {
        ErrorKind::TimedOut | ErrorKind::WouldBlock => ErrorComunicacion::Timeout,
        _ => ErrorComunicacion::ConnectionClosed,
    };

    println!(
        "{}",
        errores::obtener_mensaje_de_error_comunicacion(error_comunicacion)
    );
}