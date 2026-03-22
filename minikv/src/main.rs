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
    let mut args = env::args();
    args.next();
    let comando_option: Option<String> = args.next();
    let clave_option: Option<String> = args.next();
    let valor_option: Option<String> = args.next();

    match (comando_option, clave_option, valor_option) {
        (Some(comando), Some(clave), Some(valor)) if comando == "set" => {
            if let Err(e) = comando_set(clave, valor) {
                errores::imprimir_error(e);
            } else {
                println!("OK");
            }
        }
        (Some(comando), Some(clave), None) if comando == "set" => {
            if let Err(e) = comando_unset(clave) {
                errores::imprimir_error(e);
            } else {
                println!("OK");
            }
        }
        (Some(comando), Some(clave), None) if comando == "get" => {
            match comandos::comando_get(clave) {
                Ok(Some(valor)) => println!("{}", valor),
                Ok(None) => println!("NOT FOUND"),
                Err(e) => errores::imprimir_error(e),
            }
        }
        (Some(comando), None, None) if comando == "length" => match comandos::comando_length() {
            Ok(cantidad) => println!("{}", cantidad),
            Err(e) => errores::imprimir_error(e),
        },
        (Some(comando), None, None) if comando == "snapshot" => {
            if let Err(e) = comandos::comando_snapshot() {
                errores::imprimir_error(e);
            } else {
                println!("OK");
            }
        }
        _ => {
            println!("Comando no reconocido o argumentos incorrectos!");
        }
    }
}
