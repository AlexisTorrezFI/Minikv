mod comandos;
mod errores;
mod parser;
mod storage;
use crate::comandos::comando_set;
use crate::comandos::comando_unset;
use std::env;

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
