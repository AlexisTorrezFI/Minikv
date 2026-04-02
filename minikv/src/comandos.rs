use std::str::FromStr;

use crate::errores::ErrorMiniKv;
use crate::parser::serializar;
use crate::storage::append_linea_log;
use crate::storage::reconstruir_estado;
use crate::storage::sobrescribir_data;
use crate::storage::vaciar_log;

/// Enum que representa los comandos válidos del sistema MiniKV.
#[derive(Debug, PartialEq, Eq)]
pub enum TipoComando {
    /// Asigna un valor a una clave.
    Set,
    /// Obtiene el valor asociado a una clave.
    Get,
    /// Devuelve la cantidad de claves almacenadas.
    Length,
    /// Genera un snapshot del estado actual y limpia el log.
    Snapshot,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Comando {
    Set(String, String),
    Unset(String),
    Get(String),
    Length,
    Snapshot,
}
impl FromStr for TipoComando {
    type Err = ErrorMiniKv;

    fn from_str(comando: &str) -> Result<Self, Self::Err> {
        match comando {
            "set" => Ok(TipoComando::Set),
            "get" => Ok(TipoComando::Get),
            "length" => Ok(TipoComando::Length),
            "snapshot" => Ok(TipoComando::Snapshot),
            _ => Err(ErrorMiniKv::UnknownCommand),
        }
    }
}

/// Construye un comando válido de MiniKV a partir de los argumentos parseados.
///
/// Esta función recibe los argumentos de línea de comandos ya separados
/// (comando, clave, valor y posible argumento extra) y determina qué operación
/// debe ejecutarse.
///
/// En caso de que la combinación de argumentos sea válida, devuelve el
/// `Comando` correspondiente. Si no, retorna un `ErrorMiniKv` indicando
/// el tipo de error según la especificación del sistema.
///
/// # Parámetros
///
/// - `comando`: nombre del comando (`set`, `get`, `length`, `snapshot`).
/// - `clave`: clave sobre la cual operar (si aplica).
/// - `valor`: valor asociado a la clave (solo para `set`).
/// - `extra`: argumento adicional no esperado.
///
/// # Retorno
///
/// - `Ok(Comando)` si los argumentos son válidos.
/// - `Err(ErrorMiniKv)` si hay un error en la cantidad o tipo de argumentos.
///
/// # Errores posibles
///
/// - `ExtraArgument`: si se reciben más argumentos de los esperados.
/// - `MissingArgument`: si faltan argumentos obligatorios.
/// - `UnknownCommand`: si el comando no es reconocido.
pub fn crear_comando(
    comando: TipoComando,
    clave: Option<String>,
    valor: Option<String>,
    extra: Option<String>,
) -> Result<Comando, ErrorMiniKv> {
    match (comando, clave, valor, extra) {
        (TipoComando::Set, Some(clave), Some(valor), None) => Ok(Comando::Set(clave, valor)),
        (TipoComando::Set, Some(clave), None, None) => Ok(Comando::Unset(clave)),
        (TipoComando::Get, Some(clave), None, None) => Ok(Comando::Get(clave)),
        (TipoComando::Length, None, None, None) => Ok(Comando::Length),
        (TipoComando::Snapshot, None, None, None) => Ok(Comando::Snapshot),
        (TipoComando::Set, _, _, Some(_))
        | (TipoComando::Get, _, Some(_), _)
        | (TipoComando::Length, Some(_), _, _)
        | (TipoComando::Snapshot, Some(_), _, _) => Err(ErrorMiniKv::ExtraArgument),
        (TipoComando::Set, None, _, _) | (TipoComando::Get, None, _, _) => {
            Err(ErrorMiniKv::MissingArgument)
        }
        _ => Err(ErrorMiniKv::UnknownCommand),
    }
}
/// Ejecuta el comando `set`, registrando una clave y su valor en el archivo con ruta `path_log`.
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
/// - `path_log`: ruta al archivo de log.
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
pub fn comando_set(clave: String, valor: String, path_log: &str) -> Result<(), ErrorMiniKv> {
    let clave_log = serializar(&clave);
    let valor_log = serializar(&valor);
    let mut linea_log: String = String::new();
    linea_log.push_str("set ");
    linea_log.push_str(&clave_log);
    linea_log.push(' ');
    linea_log.push_str(&valor_log);
    linea_log.push('\n');
    let resultado = append_linea_log(&linea_log, path_log);
    match resultado {
        Ok(()) => Ok(()),
        Err(e) => Err(e),
    }
}

/// Ejecuta el comando `unset`, registrando la eliminación de una clave en el archivo con ruta `path_log`.
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
/// - `path_log`: ruta al archivo de log.
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
pub fn comando_unset(clave: String, path_log: &str) -> Result<(), ErrorMiniKv> {
    let clave_log = serializar(&clave);
    let mut linea_log: String = String::new();
    linea_log.push_str("set ");
    linea_log.push_str(&clave_log);
    linea_log.push('\n');
    let resultado = append_linea_log(&linea_log, path_log);
    match resultado {
        Ok(()) => Ok(()),
        Err(e) => Err(e),
    }
}

/// Ejecuta el comando `get`, obteniendo el valor asociado a una clave.
///
/// La función reconstruye el estado actual del MiniKV a partir de los archivos
/// con rutas `<path_data>` y `<path_log>`, y luego busca la clave en el diccionario resultante.
///
/// # Parámetros
///
/// - `clave`: clave cuyo valor se desea obtener.
/// - `path_data`: ruta al archivo de datos.
/// - `path_log`: ruta al archivo de log.
///
/// # Retorna
///
/// - `Ok(valor)` si la clave existe.
/// - `Err(ErrorMiniKv)` si ocurre un error al reconstruir el estado
///   (por ejemplo, error de lectura o línea inválida) o si no encuentra la clave.
///
/// # Comportamiento
///
/// - La búsqueda se realiza sobre el estado reconstruido en memoria.
/// - No modifica los archivos con rutas `<path_data>` ni `<path_log>`.
pub fn comando_get(clave: String, path_data: &str, path_log: &str) -> Result<String, ErrorMiniKv> {
    let mut diccionario = reconstruir_estado(path_data, path_log)?;

    match diccionario.remove(&clave) {
        Some(valor) => Ok(valor.clone()),
        None => Err(ErrorMiniKv::NotFound),
    }
}

/// Ejecuta el comando `length`, devolviendo la cantidad de claves almacenadas.
///
/// La función reconstruye el estado actual del MiniKV a partir de los archivos
/// con rutas `<path_data>` y `<path_log>`, y luego calcula el tamaño del diccionario resultante.
///
/// # Parámetros
/// - `path_data`: ruta al archivo de datos.
/// - `path_log`: ruta al archivo de log.
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
/// La función reconstruye el estado completo a partir de los archivos cuyas rutas son
/// `<path_data>` y `<path_log>`, y luego:
///
/// 1. Genera un nuevo contenido con los pares clave-valor finales.
/// 2. Sobrescribe el archivo con path `<path_data>` con ese contenido.
/// 3. Vacía el archivo con path `<path_log>`.
///
/// De esta forma, se elimina el historial de operaciones y se conserva
/// únicamente el estado final del sistema.
///
/// # Parámetros
/// - `path_data`: ruta al archivo de datos.
/// - `path_log`: ruta al archivo de log.
///
/// # Retorna
///
/// - `Ok(())` si el snapshot se realiza correctamente.
/// - `Err(ErrorMiniKv)` si ocurre algún error al:
///   - reconstruir el estado,
///   - escribir el archivo con path `<path_data>`,
///   - o vaciar el archivo con path `<path_log>`.
///
/// # Comportamiento
///
/// - El archivo `<path_data>` queda con el estado final actualizado.
/// - El archivo `<path_log>` queda vacío.
/// - No se pierde información si ocurre un error antes de vaciar el log.
///
/// # Notas
///
/// - Las claves y valores se serializan antes de ser escritos en `<path_data>`.
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
            sobrescribir_data(&contenido, path_data)?;
            vaciar_log(path_log)?;
            Ok(())
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs::{self, File},
        io::BufReader,
    };

    pub fn borrar_archivos(paths: &[&str]) {
        for path in paths {
            let _ = fs::remove_file(path);
        }
    }

    #[test]
    fn test_01_integracion_set_get() {
        let path_log = ".test.integracion01.minikv.log";
        let path_data = ".test.integracion01.minikv.data";
        borrar_archivos(&[path_data, path_log]);

        comando_set("clave1".to_string(), "valor1".to_string(), path_log).unwrap();

        let resultado = comando_get("clave1".to_string(), path_data, path_log).unwrap();

        assert_eq!(resultado, "valor1".to_string());

        borrar_archivos(&[path_data, path_log]);
    }

    #[test]
    fn test_02_integracion_set_sobrescritura() {
        let path_log = ".test.integracion02.minikv.log";
        let path_data = ".test.integracion02.minikv.data";
        borrar_archivos(&[path_data, path_log]);

        comando_set("clave1".to_string(), "valor1".to_string(), path_log).unwrap();
        comando_set("clave1".to_string(), "valor2".to_string(), path_log).unwrap();

        let resultado = comando_get("clave1".to_string(), path_data, path_log).unwrap();

        assert_eq!(resultado, "valor2".to_string());

        borrar_archivos(&[path_data, path_log]);
    }

    #[test]
    fn test_03_integracion_unset() {
        let path_log = ".test.integracion03.minikv.log";
        let path_data = ".test.integracion03.minikv.data";
        borrar_archivos(&[path_data, path_log]);

        comando_set("clave1".to_string(), "valor1".to_string(), path_log).unwrap();
        comando_unset("clave1".to_string(), path_log).unwrap();

        let resultado: Result<String, ErrorMiniKv> =
            comando_get("clave1".to_string(), path_data, path_log);
        assert_eq!(resultado, Err(ErrorMiniKv::NotFound));

        borrar_archivos(&[path_data, path_log]);
    }

    #[test]
    fn test_04_integracion_snapshot() {
        use std::fs;
        use std::io::BufRead;
        let path_log = ".test.integracion04.minikv.log";
        let path_data = ".test.integracion04.minikv.data";
        borrar_archivos(&[path_data, path_log]);

        comando_set("clave1".to_string(), "valor1".to_string(), path_log).unwrap();

        comando_snapshot(path_data, path_log).unwrap();

        let metadata = fs::metadata(path_log).unwrap();
        assert_eq!(metadata.len(), 0);

        let file = File::open(path_data).unwrap();
        let reader = BufReader::new(file);

        let mut encontro_clave1 = false;

        for linea in reader.lines() {
            let linea = linea.unwrap();

            if linea.contains("clave1") {
                encontro_clave1 = true;
            }
        }
        assert!(encontro_clave1);
        borrar_archivos(&[path_data, path_log]);
    }
    #[test]
    fn test_05_integracion_length() {
        let path_log = ".test.integracion05.minikv.log";
        let path_data = ".test.integracion05.minikv.data";

        borrar_archivos(&[path_data, path_log]);

        comando_set("clave1".to_string(), "valor1".to_string(), path_log).unwrap();
        comando_set("clave2".to_string(), "valor2".to_string(), path_log).unwrap();

        let resultado = comando_length(path_data, path_log).unwrap();

        assert_eq!(resultado, 2);

        borrar_archivos(&[path_data, path_log]);
    }
}
