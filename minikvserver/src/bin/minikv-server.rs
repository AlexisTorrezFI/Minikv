use std::collections::HashMap;
use std::io::Write;
use std::io::{BufRead, BufReader,ErrorKind};
use std::time::Duration;
use std::net::TcpStream;
use std::str::FromStr;
use std::{net::TcpListener, sync::mpsc::Receiver};
use std::sync::mpsc::{self, Sender};
use std::thread;
use minikvserver::comunicacion::respuesta::Respuesta;
use minikvserver::minikv::comandos::{Comando, TipoComando, comando_get, comando_length, comando_set, comando_snapshot, comando_unset, crear_comando};
use minikvserver::minikv::errores::{ErrorCliente, ErrorComunicacion, ErrorServidorDatos};
use minikvserver::minikv::parser::separar_argumentos;
use minikvserver::{comunicacion::peticion::Peticion, minikv::{errores::{self, ErrorServidorConexion}, storage::reconstruir_estado}};
const TIMEOUT_SEGUNDOS: u64 = 200;
fn main() {

    let mut args = std::env::args();

    let _bin = args.next();
    let direccion = match args.next() {
        Some(dir) => dir,
        None => {
            eprintln!(
                "{}",
                errores::obtener_mensaje_de_error_servidor_conexion(
                    ErrorServidorConexion::InvalidArgs
                )
            );
            return;
        }
    };

    if args.next().is_some() {
        eprintln!(
            "{}",
            errores::obtener_mensaje_de_error_servidor_conexion(
                ErrorServidorConexion::InvalidArgs
            )
        );
        return;
    }

    let Ok(tcp_listener) = TcpListener::bind(&direccion) else {
        eprintln!(
            "{}",
            direccion
        );
        eprintln!(
            "{}",
            errores::obtener_mensaje_de_error_servidor_conexion(
                ErrorServidorConexion::ServerSocketBinding
            )
        );
        return;
    };
    let diccionario;
    match reconstruir_estado(".minikv.data", ".minikv.log"){
        Ok(dicc) => diccionario = dicc,
        Err(e) => {
            eprintln!("{}",errores::obtener_mensaje_de_error_servidor_datos(e));
            return;
        }
    }
    let (tx_jefe, rx_jefe) = mpsc::channel::<Peticion>();
    let (tx_error_fatal, rx_error_fatal) = mpsc::channel::<ErrorServidorDatos>();
    crear_jefe(rx_jefe,diccionario,tx_error_fatal);
    thread::spawn(move || {
        escuchar_conexiones(tcp_listener,tx_jefe);
    });

    let error = rx_error_fatal.recv();
    match error {
    Ok(e) => {
        eprintln!("{}", errores::obtener_mensaje_de_error_servidor_datos(e));
    }
    Err(_) => {
        eprintln!("Error recibiendo error fatal");
    }
}
    
}

fn escuchar_conexiones(tcp_listener:TcpListener,tx_jefe:Sender<Peticion>){
    for conexion in tcp_listener.incoming() {
        match conexion {
            Ok(stream) => {
                let tx_jefe_clonado = tx_jefe.clone();
                thread::spawn(move || {
                    atender_cliente(stream, tx_jefe_clonado);
                });
            }
            Err(_) => {
                eprintln!(
                    "{}",
                    errores::obtener_mensaje_de_error_comunicacion(
                        ErrorComunicacion::ConnectionClosed
                    )
                );
            }
        }
    }
}



fn atender_cliente(stream: TcpStream, tx_jefe: Sender<Peticion>) {
    if stream
        .set_read_timeout(Some(Duration::from_secs(TIMEOUT_SEGUNDOS)))
        .is_err()
    {
        eprintln!(
            "{}",
            errores::obtener_mensaje_de_error_comunicacion(
                ErrorComunicacion::ConnectionClosed,
            )
        );
        return;
    }

    if stream
        .set_write_timeout(Some(Duration::from_secs(TIMEOUT_SEGUNDOS)))
        .is_err()
    {
        eprintln!(
            "{}",
            errores::obtener_mensaje_de_error_comunicacion(
                ErrorComunicacion::ConnectionClosed,
            )
        );
        return;
    }

    let reader_stream = match stream.try_clone() {
        Ok(s) => s,
        Err(_) => {
            eprintln!(
                "{}",
                errores::obtener_mensaje_de_error_comunicacion(
                    ErrorComunicacion::ConnectionClosed,
                )
            );
            return;
        }
    };

    let mut reader = BufReader::new(reader_stream);
    let mut writer = stream;

    loop {
        let mut linea = String::new();

        let bytes_leidos = match reader.read_line(&mut linea) {
            Ok(n) => n,
            Err(e) => {
                let error_comunicacion = match e.kind() {
                    ErrorKind::TimedOut | ErrorKind::WouldBlock => {
                        ErrorComunicacion::Timeout
                    }
                    _ => ErrorComunicacion::ConnectionClosed,
                };

                eprintln!(
                    "{}",
                    errores::obtener_mensaje_de_error_comunicacion(error_comunicacion)
                );
                return;
            }
        };

        if bytes_leidos == 0 {
            eprintln!(
                "{}",
                errores::obtener_mensaje_de_error_comunicacion(
                    ErrorComunicacion::ConnectionClosed,
                )
            );
            return;
        }

        let linea = linea.trim();

        if linea.is_empty() {
            continue;
        }

        let argumentos = separar_argumentos(linea);
        let mut iterador_argumentos = argumentos.into_iter();

        let Some(comando_str) = iterador_argumentos.next() else {
            if let Err(e) = writer.write_all(
                format!(
                    "{}\n",
                    errores::obtener_mensaje_de_error_cliente(ErrorCliente::UnknownCommand)
                )
                .as_bytes(),
            ) {
                let error_comunicacion = match e.kind() {
                    ErrorKind::TimedOut | ErrorKind::WouldBlock => {
                        ErrorComunicacion::Timeout
                    }
                    _ => ErrorComunicacion::ConnectionClosed,
                };

                eprintln!(
                    "{}",
                    errores::obtener_mensaje_de_error_comunicacion(error_comunicacion)
                );
                return;
            }
            continue;
        };

        let tipo_comando = match TipoComando::from_str(&comando_str) {
            Ok(tipo) => tipo,
            Err(e) => {
                if let Err(error_escritura) = writer.write_all(
                    format!(
                        "{}\n",
                        errores::obtener_mensaje_de_error_cliente(e)
                    )
                    .as_bytes(),
                ) {
                    let error_comunicacion = match error_escritura.kind() {
                        ErrorKind::TimedOut | ErrorKind::WouldBlock => {
                            ErrorComunicacion::Timeout
                        }
                        _ => ErrorComunicacion::ConnectionClosed,
                    };

                    eprintln!(
                        "{}",
                        errores::obtener_mensaje_de_error_comunicacion(error_comunicacion)
                    );
                    return;
                }
                continue;
            }
        };

        let comando = match crear_comando(
            tipo_comando,
            iterador_argumentos.next(),
            iterador_argumentos.next(),
            iterador_argumentos.next(),
        ) {
            Ok(comando_valido) => comando_valido,
            Err(e) => {
                if let Err(error_escritura) = writer.write_all(
                    format!(
                        "{}\n",
                        errores::obtener_mensaje_de_error_cliente(e)
                    )
                    .as_bytes(),
                ) {
                    let error_comunicacion = match error_escritura.kind() {
                        ErrorKind::TimedOut | ErrorKind::WouldBlock => {
                            ErrorComunicacion::Timeout
                        }
                        _ => ErrorComunicacion::ConnectionClosed,
                    };

                    eprintln!(
                        "{}",
                        errores::obtener_mensaje_de_error_comunicacion(error_comunicacion)
                    );
                    return;
                }
                continue;
            }
        };

        let (tx_respuesta, rx_respuesta) = mpsc::channel::<Respuesta>();

        let peticion = Peticion {
            comando,
            tx_respuesta,
        };

        if tx_jefe.send(peticion).is_err() {
            return;
        }

        let respuesta = match rx_respuesta.recv() {
            Ok(respuesta) => respuesta,
            Err(_) => {
                return;
            }
        };

        let mensaje_a_enviar = match respuesta {
            Respuesta::Ok => "OK\n".to_string(),
            Respuesta::Valor(valor) => format!("{valor}\n"),
            Respuesta::Cantidad(cantidad) => format!("{cantidad}\n"),
            Respuesta::ErrorCliente(error) => {
                format!(
                    "{}\n",
                    errores::obtener_mensaje_de_error_cliente(error)
                )
            }
            Respuesta::ErrorServidor(error) => {
                format!(
                    "{}\n",
                    errores::obtener_mensaje_de_error_servidor_datos(error)
                )
            }
        };

        if let Err(e) = writer.write_all(mensaje_a_enviar.as_bytes()) {
            let error_comunicacion = match e.kind() {
                ErrorKind::TimedOut | ErrorKind::WouldBlock => {
                    ErrorComunicacion::Timeout
                }
                _ => ErrorComunicacion::ConnectionClosed,
            };

            eprintln!(
                "{}",
                errores::obtener_mensaje_de_error_comunicacion(error_comunicacion)
            );
            return;
        }
    }
}

fn crear_jefe(
    receptor_de_peticiones: Receiver<Peticion>,
    mut diccionario: HashMap<String, String>,
    emisor_de_error_fatal: Sender<ErrorServidorDatos>,
 ) {
    thread::spawn(move || {
        while let Ok(peticion) = receptor_de_peticiones.recv() {
            let respuesta = match peticion.comando {
                Comando::Set(clave, valor) => {
                    match comando_set(clave, valor, ".minikv.log", &mut diccionario) {
                        Ok(()) => Respuesta::Ok,
                        Err(e) => {
                            let _ = emisor_de_error_fatal.send(e);
                            break;
                        }
                    }
                }

                Comando::Unset(clave) => {
                    match comando_unset(clave, ".minikv.log", &mut diccionario) {
                        Ok(()) => Respuesta::Ok,
                        Err(e) => {
                            let _ = emisor_de_error_fatal.send(e);
                            break;
                        }
                    }
                }

                Comando::Get(clave) => {
                    match comando_get(clave, & mut diccionario) {
                        Ok(valor) => Respuesta::Valor(valor),
                        Err(e) => Respuesta::ErrorCliente(e),
                    }
                }

                Comando::Length => {
                    Respuesta::Cantidad(comando_length(& mut diccionario))
                }

                Comando::Snapshot => {
                    match comando_snapshot(".minikv.data", ".minikv.log") {
                        Ok(()) => Respuesta::Ok,
                        Err(e) => {
                            let _ = emisor_de_error_fatal.send(e);
                            break;
                        }
                    }
                }
            };
            if peticion.tx_respuesta.send(respuesta).is_err() {
                eprintln!("No se pudo enviar la respuesta al thread cliente");
            }
        }
    });
}