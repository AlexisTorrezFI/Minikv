use crate::parser::separar_argumentos;
use crate::errores::ErrorMiniKv;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::fs;
use std::io::ErrorKind;

pub fn append_linea_log(linea: &str) -> Result<(), ErrorMiniKv> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(".minikv.log")
        .map_err(|_| ErrorMiniKv::NoSePudoAbrirArchivo)?;

    file.write_all(linea.as_bytes())
        .map_err(|_| ErrorMiniKv::NoSePudoEscribirArchivo)?;

    Ok(())
}
pub fn sobrescribir_data(contenido: &str) -> Result<(), ErrorMiniKv> {
    let mut archivo = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(".minikv.data")
        .map_err(|_| ErrorMiniKv::NoSePudoAbrirArchivo)?;

    archivo
        .write_all(contenido.as_bytes())
        .map_err(|_| ErrorMiniKv::NoSePudoEscribirArchivo)?;

    Ok(())
}
pub fn vaciar_log() -> Result<(), ErrorMiniKv> {
    OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(".minikv.log")
        .map_err(|_| ErrorMiniKv::NoSePudoAbrirArchivo)?;

    Ok(())
}


pub fn reconstruir_estado() -> Result<HashMap<String, String>, ErrorMiniKv> {
    let mut diccionario = HashMap::new();

    if let Err(e) = cargar_data_en_memoria(&mut diccionario) {
        return Err(e);
    }
    if let Err(e) = cargar_log_en_memoria(&mut diccionario) {
        return Err(e);
    }

    Ok(diccionario)
}

fn cargar_data_en_memoria( diccionario: &mut HashMap<String, String>) -> Result<(), ErrorMiniKv> {
    let contenido = match fs::read_to_string(".minikv.data") {
        Ok(contenido) => contenido,
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                return Ok(());
            } else {
                return Err(ErrorMiniKv::NoSePudoLeerArchivo);
            }
        }
    };
    for linea in contenido.lines() {
            if let Err(_e) = validar_linea_data(linea) {
                return Err(ErrorMiniKv::LineaInvalida);
            }
            let partes = separar_argumentos(linea);

            if partes.len() == 2 {
                let clave = &partes[0];
                let valor = &partes[1];

                diccionario.insert(clave.to_string(), valor.to_string());
            }
        }
    Ok(())
}

fn cargar_log_en_memoria(diccionario: &mut HashMap<String, String>) -> Result<(), ErrorMiniKv> {
    let contenido = match fs::read_to_string(".minikv.log") {
        Ok(contenido) => contenido,
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                return Ok(());
            } else {
                return Err(ErrorMiniKv::NoSePudoLeerArchivo);
            }
        }
    };
    for linea in contenido.lines() {
        if let Err(_e) = validar_linea_log(linea) {
            return Err(ErrorMiniKv::LineaInvalida);
        }
        let partes = separar_argumentos(linea);

        match partes.len() {
            3 => {
                let clave = &partes[1];
                let valor = &partes[2];

                diccionario.insert(clave.to_string(), valor.to_string());
            }
            2 => {
                let clave = &partes[1];
                diccionario.remove(clave);
            }
            _ => {
                return Err(ErrorMiniKv::LineaInvalida);
            }
        }
    }
    Ok(())
}

fn validar_linea_log(linea: &str) -> Result<(), ErrorMiniKv> {
    let partes = separar_argumentos(linea);
    match partes.len() {
        3 => {
            if partes[0] != "set" {
                return Err(ErrorMiniKv::LineaInvalida);
            }
        }
        2 => {
            if partes[0] != "set" {
                return Err(ErrorMiniKv::LineaInvalida);
            }
        }
        _ => {
            return Err(ErrorMiniKv::LineaInvalida);
        }
    }
    Ok(())
}

fn validar_linea_data(linea: &str) -> Result<(), ErrorMiniKv> {
    let partes = separar_argumentos(linea);
    if partes.len() != 2 {
        return Err(ErrorMiniKv::LineaInvalida);
    }
    Ok(())
}