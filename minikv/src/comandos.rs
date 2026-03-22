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
pub fn comando_set(clave: String, valor: String,path_log: &str) -> Result<(), ErrorMiniKv> {
    let clave_log = serializar(&clave);
    let valor_log = serializar(&valor);
    let mut linea_log: String = String::new();
    linea_log.push_str("set ");
    linea_log.push_str(&clave_log);
    linea_log.push(' ');
    linea_log.push_str(&valor_log);
    linea_log.push('\n');
    let resultado = append_linea_log(&linea_log,path_log);
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
pub fn comando_unset(clave: String,path_log: &str) -> Result<(), ErrorMiniKv> {
    let clave_log = serializar(&clave);
    let mut linea_log: String = String::new();
    linea_log.push_str("set ");
    linea_log.push_str(&clave_log);
    linea_log.push('\n');
    let resultado = append_linea_log(&linea_log,path_log);
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
pub fn comando_get(clave: String, path_log: &str,path_data: &str) -> Result<Option<String>, ErrorMiniKv> {
    let respuesta = reconstruir_estado(path_data, path_log);
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
pub fn comando_length(path_data: &str, path_log: &str) -> Result<usize, ErrorMiniKv> {
    let respuesta = reconstruir_estado(path_data, path_log);
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
pub fn comando_snapshot(path_data: &str, path_log: &str) -> Result<(), ErrorMiniKv> {
    let respuesta = reconstruir_estado(path_data, path_log);
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
            sobrescribir_data(&contenido,path_data)?;
            vaciar_log(path_log)?;
            Ok(())
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01_integracion_set_get() {
        use std::fs;
        let path_log = ".test.integracion01.minikv.log";
        let path_data = ".test.integracion01.minikv.data";
        let _ = fs::remove_file(path_data);
        let _ = fs::remove_file(path_log);

        comando_set("clave1".to_string(), "valor1".to_string(), path_log).unwrap();

        let resultado = comando_get("clave1".to_string(), path_log, path_data).unwrap();

        assert_eq!(resultado, Some("valor1".to_string()));

        let _ = fs::remove_file(path_data);
        let _ = fs::remove_file(path_log);
    }

    #[test]
    fn test_02_integracion_set_sobrescritura() {
        use std::fs;

        let path_log = ".test.integracion02.minikv.log";
        let path_data = ".test.integracion02.minikv.data";
        let _ = fs::remove_file(path_data);
        let _ = fs::remove_file(path_log);

        comando_set("clave1".to_string(), "valor1".to_string(), path_log).unwrap();
        comando_set("clave1".to_string(), "valor2".to_string(), path_log).unwrap();

        let resultado = comando_get("clave1".to_string(), path_log, path_data).unwrap();

        assert_eq!(resultado, Some("valor2".to_string()));

        let _ = fs::remove_file(path_data);
        let _ = fs::remove_file(path_log);
    }

    #[test]
    fn test_03_integracion_unset() {
        use std::fs;


        let path_log = ".test.integracion03.minikv.log";
        let path_data = ".test.integracion03.minikv.data";
        let _ = fs::remove_file(path_data);
        let _ = fs::remove_file(path_log);

        comando_set("clave1".to_string(), "valor1".to_string(), path_log).unwrap();
        comando_unset("clave1".to_string(), path_log).unwrap();

        let resultado = comando_get("clave1".to_string(), path_log, path_data).unwrap();

        assert_eq!(resultado, None);

        let _ = fs::remove_file(path_data);
        let _ = fs::remove_file(path_log);
    }

    #[test]
    fn test_04_integracion_snapshot() {
        use std::fs;

        let path_log = ".test.integracion04.minikv.log";
        let path_data = ".test.integracion04.minikv.data";
        let _ = fs::remove_file(path_data);
        let _ = fs::remove_file(path_log);

        comando_set("clave1".to_string(), "valor1".to_string(), path_log).unwrap();
        comando_set("clave2".to_string(), "valor2".to_string(), path_log).unwrap();

        comando_snapshot(path_data, path_log).unwrap();

        // después del snapshot, el log debería estar vacío
        let log = fs::read_to_string(path_log).unwrap();
        assert_eq!(log, "");

        // y los datos deben estar en .data
        let data = fs::read_to_string(path_data).unwrap();
        assert!(data.contains("clave1"));
        assert!(data.contains("clave2"));

        let _ = fs::remove_file(path_data);
        let _ = fs::remove_file(path_log);
    }
    #[test]
    fn test_05_integracion_length() {
        use std::fs;

        let path_log = ".test.integracion05.minikv.log";
        let path_data = ".test.integracion05.minikv.data";

        let _ = fs::remove_file(path_data);
        let _ = fs::remove_file(path_log);

        comando_set("clave1".to_string(), "valor1".to_string(), path_log).unwrap();
        comando_set("clave2".to_string(), "valor2".to_string(), path_log).unwrap();

        let resultado = comando_length(path_data, path_log).unwrap();

        assert_eq!(resultado, 2);

        let _ = fs::remove_file(path_data);
        let _ = fs::remove_file(path_log);
    }
}
