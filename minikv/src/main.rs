mod comandos;
mod errores;
mod parser;
mod storage;
use crate::comandos::{Comando, TipoComando, crear_comando};
use std::{
    env::{self},
    str::FromStr,
};

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

    let Some(comando) = args.next() else {
        errores::imprimir_error(errores::ErrorMiniKv::UnknownCommand);
        return;
    };
    let tipo_comando = match TipoComando::from_str(&comando) {
        Ok(tipo) => tipo,
        Err(e) => {
            errores::imprimir_error(e);
            return;
        }
    };
    match crear_comando(tipo_comando, args.next(), args.next(), args.next()) {
        Ok(comando) => ejecutar_comando(comando, path_data, path_log),
        Err(e) => errores::imprimir_error(e),
    }
}

fn ejecutar_comando(comando: Comando, path_data: &str, path_log: &str) {
    match comando {
        Comando::Set(clave, valor) => {
            imprimir_resultado_simple(comandos::comando_set(clave, valor, path_log));
        }
        Comando::Unset(clave) => {
            imprimir_resultado_simple(comandos::comando_unset(clave, path_log));
        }
        Comando::Get(clave) => {
            imprimir_resultado_valor(comandos::comando_get(clave, path_data, path_log));
        }
        Comando::Length => {
            imprimir_resultado_numero(comandos::comando_length(path_data, path_log));
        }
        Comando::Snapshot => {
            imprimir_resultado_simple(comandos::comando_snapshot(path_data, path_log));
        }
    }
}

fn imprimir_resultado_simple(resultado: Result<(), errores::ErrorMiniKv>) {
    match resultado {
        Ok(()) => println!("OK"),
        Err(e) => errores::imprimir_error(e),
    }
}

fn imprimir_resultado_valor(resultado: Result<String, errores::ErrorMiniKv>) {
    match resultado {
        Ok(valor) => println!("{}", valor),
        Err(e) => errores::imprimir_error(e),
    }
}

fn imprimir_resultado_numero(resultado: Result<usize, errores::ErrorMiniKv>) {
    match resultado {
        Ok(cantidad) => println!("{}", cantidad),
        Err(e) => errores::imprimir_error(e),
    }
}
