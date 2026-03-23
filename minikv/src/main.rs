mod comandos;
mod errores;
mod parser;
mod storage;
use crate::comandos::comando_set;
use crate::comandos::comando_unset;
use std::env;

/// Punto de entrada del programa MiniKV.
///
/// Esta función procesa los argumentos recibidos por línea de comandos e
/// identifica qué operación debe ejecutarse sobre el sistema.
///
/// Comandos soportados:
///
/// - `set <clave> <valor>`: registra una asignación en el log.
/// - `set <clave>`: registra la eliminación de una clave.
/// - `get <clave>`: busca el valor asociado a una clave.
/// - `length`: devuelve la cantidad de claves almacenadas.
/// - `snapshot`: consolida el estado actual en `.minikv.data` y vacía `.minikv.log`.
///
/// # Comportamiento
///
/// - Si la operación se ejecuta correctamente:
///   - `set`, `unset` y `snapshot` imprimen `OK`.
///   - `get` imprime el valor o `NOT FOUND`.
///   - `length` imprime la cantidad de claves.
/// - Si ocurre un error, se imprime el mensaje correspondiente mediante
///   `errores::imprimir_error`.
/// - Si los argumentos no coinciden con ningún comando válido, se informa
///   que el comando no fue reconocido.
fn main() {
    let path_log = ".minikv.log";
    let path_data = ".minikv.data";
    let mut args = env::args();
    args.next();
    let comando_option: Option<String> = args.next();
    let clave_option: Option<String> = args.next();
    let valor_option: Option<String> = args.next();
    let extra_argument_option: Option<String> = args.next();

    match (
        comando_option,
        clave_option,
        valor_option,
        extra_argument_option,
    ) {
        (Some(comando), Some(clave), Some(valor), None) if comando == "set" => {
            if let Err(e) = comando_set(clave, valor, path_log) {
                errores::imprimir_error(e);
            } else {
                println!("OK");
            }
        }
        (Some(comando), Some(clave), None, None) if comando == "set" => {
            if let Err(e) = comando_unset(clave, path_log) {
                errores::imprimir_error(e);
            } else {
                println!("OK");
            }
        }
        (Some(comando), Some(clave), None, None) if comando == "get" => {
            match comandos::comando_get(clave, path_data, path_log) {
                Ok(valor) => println!("{}", valor),
                Err(e) => errores::imprimir_error(e),
            }
        }
        (Some(comando), None, None, None) if comando == "length" => {
            match comandos::comando_length(path_data, path_log) {
                Ok(cantidad) => println!("{}", cantidad),
                Err(e) => errores::imprimir_error(e),
            }
        }
        (Some(comando), None, None, None) if comando == "snapshot" => {
            if let Err(e) = comandos::comando_snapshot(path_data, path_log) {
                errores::imprimir_error(e);
            } else {
                println!("OK");
            }
        }
        (Some(comando), _, _, Some(_)) if comando == "set" => {
            errores::imprimir_error(errores::ErrorMiniKv::ExtraArgument);
        }
        (Some(comando), _, Some(_), _) if comando == "get" => {
            errores::imprimir_error(errores::ErrorMiniKv::ExtraArgument);
        }
        (Some(comando), Some(_), _, _) if comando == "length" || comando == "snapshot" => {
            errores::imprimir_error(errores::ErrorMiniKv::ExtraArgument);
        }
        (Some(comando), None, _, _) if comando == "set" || comando == "get" => {
            errores::imprimir_error(errores::ErrorMiniKv::MissingArgument);
        }
        _ => {
            errores::imprimir_error(errores::ErrorMiniKv::UnknownCommand);
        }
    }
}
