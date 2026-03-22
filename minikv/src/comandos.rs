use crate::errores::ErrorMiniKv;
use crate::parser::serializar;
use crate::storage::append_linea_log;
use crate::storage::reconstruir_estado;
use crate::storage::sobrescribir_data;
use crate::storage::vaciar_log;

/// Ejecuta el comando `set`, registrando una clave y su valor en el archivo `.minikv.log`.
///
/// La clave y el valor se serializan antes de ser escritos, para asegurar
/// que el formato sea consistente y permita su posterior reconstrucción.
///
/// La operación se registra en el log en el formato:
/// `set clave valor`
///
/// # Parámetros
///
/// - `clave`: clave a almacenar.
/// - `valor`: valor asociado a la clave.
///
/// # Retorna
///
/// - `Ok(())` si la línea se escribe correctamente en el log.
/// - `Err(ErrorMiniKv)` si ocurre un error al abrir o escribir el archivo.
///
/// # Comportamiento
///
/// - Si la clave ya existe, su valor será sobrescrito en futuras reconstrucciones
///   del estado.
/// - Esta función no modifica el estado en memoria, solo registra la operación.
pub fn comando_set(clave: String, valor: String) -> Result<(), ErrorMiniKv> {
    let clave_log = serializar(&clave);
    let valor_log = serializar(&valor);
    let mut linea_log: String = String::new();
    linea_log.push_str("set ");
    linea_log.push_str(&clave_log);
    linea_log.push(' ');
    linea_log.push_str(&valor_log);
    linea_log.push('\n');
    let resultado = append_linea_log(&linea_log);
    match resultado {
        Ok(()) => Ok(()),
        Err(e) => Err(e),
    }
}

/// Ejecuta el comando `unset`, registrando la eliminación de una clave en el archivo `.minikv.log`.
///
/// La operación se registra en el log utilizando el formato:
/// `set clave`
///
/// Durante la reconstrucción del estado, esta operación indica que la clave
/// debe eliminarse del diccionario.
///
/// # Parámetros
///
/// - `clave`: clave a eliminar.
///
/// # Retorna
///
/// - `Ok(())` si la línea se escribe correctamente en el log.
/// - `Err(ErrorMiniKv)` si ocurre un error al abrir o escribir el archivo.
///
/// # Comportamiento
///
/// - Si la clave existe, será eliminada en la reconstrucción del estado.
/// - Si la clave no existe, la operación no produce errores.
/// - Esta función solo registra la operación.
pub fn comando_unset(clave: String) -> Result<(), ErrorMiniKv> {
    let clave_log = serializar(&clave);
    let mut linea_log: String = String::new();
    linea_log.push_str("set ");
    linea_log.push_str(&clave_log);
    linea_log.push('\n');
    let resultado = append_linea_log(&linea_log);
    match resultado {
        Ok(()) => Ok(()),
        Err(e) => Err(e),
    }
}

/// Ejecuta el comando `get`, obteniendo el valor asociado a una clave.
///
/// La función reconstruye el estado actual del MiniKV a partir de los archivos
/// `.minikv.data` y `.minikv.log`, y luego busca la clave en el diccionario resultante.
///
/// # Parámetros
///
/// - `clave`: clave cuyo valor se desea obtener.
///
/// # Retorna
///
/// - `Ok(Some(valor))` si la clave existe.
/// - `Ok(None)` si la clave no se encuentra.
/// - `Err(ErrorMiniKv)` si ocurre un error al reconstruir el estado
///   (por ejemplo, error de lectura o línea inválida).
///
/// # Comportamiento
///
/// - La búsqueda se realiza sobre el estado reconstruido en memoria.
/// - Se utiliza `remove`, por lo que la clave es eliminada del diccionario temporal.
///   Esto no afecta al almacenamiento persistente.
/// - No modifica los archivos `.data` ni `.log`.
pub fn comando_get(clave: String) -> Result<Option<String>, ErrorMiniKv> {
    let respuesta = reconstruir_estado();
    match respuesta {
        Ok(mut diccionario) => Ok(diccionario.remove(&clave)),
        Err(e) => Err(e),
    }
}

/// Ejecuta el comando `length`, devolviendo la cantidad de claves almacenadas.
///
/// La función reconstruye el estado actual del MiniKV a partir de los archivos
/// `.minikv.data` y `.minikv.log`, y luego calcula el tamaño del diccionario resultante.
///
/// # Retorna
///
/// - `Ok(cantidad)` con el número de pares clave-valor almacenados.
/// - `Err(ErrorMiniKv)` si ocurre un error al reconstruir el estado
///   (por ejemplo, error de lectura o línea inválida).
///
/// # Comportamiento
///
/// - El resultado puede ser `0` si no hay claves almacenadas.
/// - No modifica el estado en memoria persistente ni los archivos.
pub fn comando_length() -> Result<usize, ErrorMiniKv> {
    let respuesta = reconstruir_estado();
    match respuesta {
        Ok(diccionario) => Ok(diccionario.len()),
        Err(e) => Err(e),
    }
}

/// Ejecuta el comando `snapshot`, consolidando el estado actual del MiniKV.
///
/// La función reconstruye el estado completo a partir de los archivos
/// `.minikv.data` y `.minikv.log`, y luego:
///
/// 1. Genera un nuevo contenido con los pares clave-valor finales.
/// 2. Sobrescribe el archivo `.minikv.data` con ese contenido.
/// 3. Vacía el archivo `.minikv.log`.
///
/// De esta forma, se elimina el historial de operaciones y se conserva
/// únicamente el estado final del sistema.
///
/// # Retorna
///
/// - `Ok(())` si el snapshot se realiza correctamente.
/// - `Err(ErrorMiniKv)` si ocurre algún error al:
///   - reconstruir el estado,
///   - escribir el archivo `.minikv.data`,
///   - o vaciar el archivo `.minikv.log`.
///
/// # Comportamiento
///
/// - El archivo `.minikv.data` queda con el estado final actualizado.
/// - El archivo `.minikv.log` queda vacío.
/// - No se pierde información si ocurre un error antes de vaciar el log.
///
/// # Notas
///
/// - Las claves y valores se serializan antes de ser escritos en `.data`.
pub fn comando_snapshot() -> Result<(), ErrorMiniKv> {
    let respuesta = reconstruir_estado();
    match respuesta {
        Ok(diccionario) => {
            let mut contenido = String::new();
            for (clave, valor) in diccionario {
                let clave_serializada = serializar(&clave);
                let valor_serializado = serializar(&valor);
                let mut linea: String = String::new();
                linea.push_str(&clave_serializada);
                linea.push(' ');
                linea.push_str(&valor_serializado);
                linea.push('\n');
                contenido.push_str(&linea);
            }
            sobrescribir_data(&contenido)?;
            vaciar_log()?;
            Ok(())
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01_comando_set_crea_archivo() {
        use std::fs;
        let _ = fs::remove_file(".minikv.log");
        let _ = comando_set("clave1".to_string(), "valor1".to_string());
        assert!(fs::metadata(".minikv.log").is_ok());
    }
    #[test]
    fn test_02_comando_set_escribe_linea_en_log() {
        use std::fs;

        let _ = fs::remove_file(".minikv.log");

        let _ = comando_set("clave1".to_string(), "valor1".to_string());

        let contenido = fs::read_to_string(".minikv.log").expect("no se pudo leer .minikv.log");

        assert_eq!(contenido, "set \"clave1\" \"valor1\"\n");
    }
    #[test]
    fn test_03_comando_unset_crea_archivo() {
        use std::fs;
        let _ = fs::remove_file(".minikv.log");
        let _ = comando_unset("clave1".to_string());
        assert!(fs::metadata(".minikv.log").is_ok());
    }
    #[test]
    fn test_04_comando_unset_escribe_linea_en_log() {
        use std::fs;
        let _ = fs::remove_file(".minikv.log");
        let _ = comando_unset("clave1".to_string());
        let contenido = fs::read_to_string(".minikv.log").expect("no se pudo leer .minikv.log");

        assert_eq!(contenido, "set \"clave1\"\n");
    }
}
