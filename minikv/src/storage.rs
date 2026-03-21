use crate::errores::ErrorMiniKv;
use crate::parser::separar_argumentos;
use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::io::Write;

/// Agrega una línea al archivo .minikv.log.
///
/// Crea el archivo si no existe y escribe la línea al final.
///
/// # Errores
/// - Devuelve `NoSePudoAbrirArchivo` si no se puede abrir o crear el archivo.
/// - Devuelve `NoSePudoEscribirArchivo` si falla la escritura.
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

/// Sobrescribe el archivo `.minikv.data` con el contenido proporcionado.
///
/// Si el archivo no existe, se crea. Si ya existe, su contenido previo
/// se elimina antes de escribir el nuevo.
///
/// # Parámetros
///
/// - `contenido`: texto que se escribirá en el archivo `.minikv.data`.
///
/// # Retorna
///
/// - `Ok(())` si el archivo se abre y se escribe correctamente.
/// - `Err(ErrorMiniKv::NoSePudoAbrirArchivo)` si no se puede abrir o crear el archivo.
/// - `Err(ErrorMiniKv::NoSePudoEscribirArchivo)` si ocurre un error al escribir.
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

/// Vacía el contenido del archivo `.minikv.log`.
///
/// Si el archivo no existe, se crea vacío. Si existe, su contenido se elimina.
///
/// # Retorna
///
/// - `Ok(())` si el archivo se abre correctamente y queda vacío.
/// - `Err(ErrorMiniKv::NoSePudoAbrirArchivo)` si no se puede abrir o crear el archivo.
pub fn vaciar_log() -> Result<(), ErrorMiniKv> {
    OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(".minikv.log")
        .map_err(|_| ErrorMiniKv::NoSePudoAbrirArchivo)?;
    Ok(())
}

/// Reconstruye el estado del MiniKV leyendo los archivos .data y .log.
///
/// Primero carga los pares clave-valor desde .data, y luego aplica
/// las operaciones del log para obtener el estado final.
///
/// # Errores
/// Devuelve un `ErrorMiniKv` en los siguientes casos:
/// - `NoSePudoLeerArchivo` si falla la lectura de los archivos.
/// - `LineaInvalida` si alguna línea no tiene el formato esperado.
pub fn reconstruir_estado() -> Result<HashMap<String, String>, ErrorMiniKv> {
    let mut diccionario = HashMap::new();
    cargar_data_en_memoria(&mut diccionario)?;
    cargar_log_en_memoria(&mut diccionario)?;
    Ok(diccionario)
}

/// Carga en memoria el contenido del archivo `.minikv.data`.
///
/// Esta función intenta leer el archivo `.minikv.data` y, por cada línea,
/// parsea los pares clave-valor para insertarlos en el diccionario proporcionado.
///
/// Si el archivo no existe, no se considera un error y simplemente no se realiza
/// ninguna operación sobre el diccionario.
///
/// Cada línea del archivo debe representar un par clave-valor válido. Para ello:
/// - Se valida el formato de la línea mediante la funcion `validar_linea_data`.
/// - Luego se separan los argumentos con la funcion `separar_argumentos`.
/// - Si la línea es válida y contiene exactamente dos elementos, se inserta
///   la clave y el valor en el `HashMap`.
///
/// # Parámetros
///
/// - `diccionario`: referencia mutable a un `HashMap` donde se cargarán los pares clave-valor.
///
/// # Errores
///
/// Devuelve un `ErrorMiniKv` en los siguientes casos:
///
/// - `NoSePudoLeerArchivo`: si el archivo existe pero no se puede leer.
/// - `LineaInvalida`: si alguna línea del archivo no cumple con el formato esperado.
///
/// # Comportamiento
///
/// - Si una clave aparece más de una vez, el valor más reciente sobrescribe al anterior.
/// - Si el archivo no existe, la función retorna `Ok(())` sin modificar el diccionario.
///
/// # Retorna
///
/// - `Ok(())` si el .data se ha procesado correctamente.
/// - `Err(ErrorMiniKv)` si ocurre un error de lectura o si alguna línea es inválida.
fn cargar_data_en_memoria(diccionario: &mut HashMap<String, String>) -> Result<(), ErrorMiniKv> {
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

/// Carga en memoria las operaciones registradas en el archivo `.minikv.log`.
///
/// Esta función lee el archivo de log y aplica cada operación sobre el
/// diccionario, reconstruyendo el estado final de este.
///
/// Cada línea del log representa una operación:
/// - `set clave valor`: inserta o actualiza la clave con el valor.
/// - `set clave`: elimina la clave del diccionario (unset).
///
/// El log se procesa en orden, por lo que las operaciones posteriores
/// sobrescriben o eliminan resultados anteriores.
///
/// Si el archivo `.minikv.log` no existe, no se considera un error y
/// simplemente no se realizan cambios sobre el diccionario.
///
/// # Parámetros
///
/// - `diccionario`: referencia mutable al `HashMap` donde se aplicarán
///   las operaciones del log.
///
/// # Errores
///
/// Devuelve un `ErrorMiniKv` en los siguientes casos:
///
/// - `NoSePudoLeerArchivo`: si el archivo existe pero no se puede leer.
/// - `LineaInvalida`: si alguna línea no cumple con el formato esperado.
///
/// # Formato esperado del log
///
/// Cada línea debe ser una de las siguientes:
///
/// - `set clave valor`
/// - `set clave`
///
/// Cualquier otro formato se considera inválido.
///
/// # Comportamiento
///
/// - Si una clave es insertada varias veces, el último valor sobrevive.
/// - Si una clave es eliminada (`set clave`), se remueve del diccionario.
/// - Si se intenta eliminar una clave inexistente, no ocurre ningún error.
///
/// # Errores
///
/// Devuelve un `ErrorMiniKv` en el siguientes caso:
///
/// - `LineaInvalida`: si alguna línea no cumple con el formato esperado.
///
/// # Retorna
///
/// - `Ok(())` si el log se procesa correctamente.
/// - `Err(ErrorMiniKv)` si ocurre un error de lectura o si alguna línea es inválida.
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
