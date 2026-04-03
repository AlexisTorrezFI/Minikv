use crate::comandos::TipoComando;
use crate::errores::ErrorMiniKv;
use crate::parser::separar_argumentos;
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::BufReader;
use std::io::ErrorKind;
use std::io::Write;
use std::str::FromStr;

/// Agrega una línea al archivo de la ruta pasada por parámetro.
///
/// Crea el archivo si no existe y escribe la línea al final.
/// # Parámetros
/// - `linea`: texto a agregar al archivo, se asume que termina con un salto de línea (`\n`).
/// - `path_log`: ruta al archivo de log donde se agregará la línea.
/// # Errores
/// - Devuelve `InvalidLogFile` si no se puede abrir o crear el archivo.
/// - Devuelve `InvalidLogFile` si falla la escritura.
pub fn append_linea_log(linea: &str, path_log: &str) -> Result<(), ErrorMiniKv> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path_log)
        .map_err(|_| ErrorMiniKv::InvalidLogFile)?;
    file.write_all(linea.as_bytes())
        .map_err(|_| ErrorMiniKv::InvalidLogFile)?;
    Ok(())
}

/// Sobrescribe el archivo pasado por parámetro con el contenido proporcionado.
///
/// Si el archivo no existe, se crea. Si ya existe, su contenido previo
/// se elimina antes de escribir el nuevo.
///
/// # Parámetros
///
/// - `contenido`: texto que se escribirá en el archivo.
/// - `path`: ruta al archivo que se desea sobrescribir.
///
/// # Retorna
///
/// - `Ok(())` si el archivo se abre y se escribe correctamente.
/// - `Err(ErrorMiniKv::InvalidDataFile)` si no se puede abrir o crear el archivo.
/// - `Err(ErrorMiniKv::InvalidDataFile)` si ocurre un error al escribir.
pub fn sobrescribir_data(contenido: &str, path_data: &str) -> Result<(), ErrorMiniKv> {
    let mut archivo = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path_data)
        .map_err(|_| ErrorMiniKv::InvalidDataFile)?;
    archivo
        .write_all(contenido.as_bytes())
        .map_err(|_| ErrorMiniKv::InvalidDataFile)?;
    Ok(())
}

/// Vacía el contenido del archivo `.minikv.log`.
///
/// Si el archivo no existe, se crea vacío. Si existe, su contenido se elimina.
///
/// # Parámetros
/// - `path_log`: ruta al archivo de log que se desea vaciar.
///
/// # Errores
/// - Devuelve `Err(ErrorMiniKv::InvalidLogFile)` si no se puede abrir o crear el archivo.  
///
/// # Retorna
///
/// - `Ok(())` si el archivo se abre correctamente y queda vacío.
/// - `Err(ErrorMiniKv::InvalidLogFile)` si no se puede abrir o crear el archivo.
pub fn vaciar_log(path_log: &str) -> Result<(), ErrorMiniKv> {
    OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path_log)
        .map_err(|_| ErrorMiniKv::InvalidLogFile)?;
    Ok(())
}

/// Reconstruye el estado del MiniKV leyendo los archivos de las rutas path_data y path_log.
///
/// Primero carga los pares clave-valor desde .data, y luego aplica
/// las operaciones del log para obtener el estado final.
///
/// # Parámetros
/// - `path_data`: ruta al archivo de datos que contiene los pares clave-valor.
/// - `path_log`: ruta al archivo de log que contiene las operaciones a aplicar.
///
/// # Errores
/// Devuelve un `ErrorMiniKv` en los siguientes casos:
/// - `InvalidDataFile` si falla la lectura del archivo de datos.
/// - `InvalidLogFile` si falla la lectura del archivo de log.
pub fn reconstruir_estado(
    path_data: &str,
    path_log: &str,
) -> Result<HashMap<String, String>, ErrorMiniKv> {
    let mut diccionario = HashMap::new();
    cargar_data_en_memoria(&mut diccionario, path_data)?;
    cargar_log_en_memoria(&mut diccionario, path_log)?;
    Ok(diccionario)
}

fn abrir_archivo(path: &str) -> Result<Option<File>, ErrorMiniKv> {
    match File::open(path) {
        Ok(file) => Ok(Some(file)),
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                Ok(None)
            } else {
                Err(ErrorMiniKv::InvalidDataFile)
            }
        }
    }
}
/// Carga en memoria el contenido del archivo con el path proporcionado.
///
/// Esta función intenta leer el archivo y, por cada línea,
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
/// - `path_data`: ruta al archivo de datos que se desea cargar en memoria.
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
fn cargar_data_en_memoria(
    diccionario: &mut HashMap<String, String>,
    path_data: &str,
) -> Result<(), ErrorMiniKv> {
    let file = match abrir_archivo(path_data) {
        Ok(Some(f)) => f,
        Ok(None) => return Ok(()),
        Err(e) => return Err(e),
    };
    let reader = BufReader::new(file);
    for linea_resultado in reader.lines() {
        let Ok(linea) = linea_resultado else {
            return Err(ErrorMiniKv::InvalidDataFile);
        };
        if let Err(_e) = validar_linea_data(&linea) {
            return Err(ErrorMiniKv::InvalidDataFile);
        }
        let partes = separar_argumentos(&linea);

        match partes.as_slice() {
            [clave, valor] => {
                diccionario.insert(clave.to_string(), valor.to_string());
            }
            _ => return Err(ErrorMiniKv::InvalidDataFile),
        }
    }
    Ok(())
}

/// Carga en memoria las operaciones registradas en el archivo proporcionado.
///
/// Esta función lee el archivo y aplica cada operación sobre el
/// diccionario, reconstruyendo el estado final de este.
///
/// Cada línea del archivo representa una operación:
/// - `set clave valor`: inserta o actualiza la clave con el valor.
/// - `set clave`: elimina la clave del diccionario (unset).
///
/// El log se procesa en orden, por lo que las operaciones posteriores
/// sobrescriben o eliminan resultados anteriores.
///
/// Si el archivo con la ruta `path_log` no existe, no se considera un error y
/// simplemente no se realizan cambios sobre el diccionario.
///
/// # Parámetros
///
/// - `diccionario`: referencia mutable al `HashMap` donde se aplicarán
///   las operaciones del log.
/// - `path_log`: ruta al archivo de log que se desea cargar en memoria.
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
/// Devuelve un `ErrorMiniKv` en el siguiente caso:
///
/// - `InvalidLogFile`: si alguna línea no cumple con el formato esperado.
///
/// # Retorna
///
/// - `Ok(())` si el log se procesa correctamente.
/// - `Err(ErrorMiniKv)` si ocurre un error de lectura o si alguna línea es inválida.
fn cargar_log_en_memoria(
    diccionario: &mut HashMap<String, String>,
    path_log: &str,
) -> Result<(), ErrorMiniKv> {
    let file = match abrir_archivo(path_log) {
        Ok(Some(f)) => f,
        Ok(None) => return Ok(()),
        Err(e) => return Err(e),
    };
    let reader = BufReader::new(file);

    for linea_resultado in reader.lines() {
        let Ok(linea) = linea_resultado else {
            return Err(ErrorMiniKv::InvalidLogFile);
        };
        if let Err(_e) = validar_linea_log(&linea) {
            return Err(ErrorMiniKv::InvalidLogFile);
        }
        let partes = separar_argumentos(&linea);
        match partes.as_slice() {
            [_, clave, valor] => {
                diccionario.insert(clave.to_string(), valor.to_string());
            }
            [_, clave] => {
                diccionario.remove(clave);
            }
            _ => {
                return Err(ErrorMiniKv::InvalidLogFile);
            }
        }
    }
    Ok(())
}

/// Valida el formato de una línea de un archivo de tipo log.
///
/// Una línea válida debe cumplir con uno de los siguientes formatos:
/// - `set clave valor`
/// - `set clave`
///
/// En ambos casos, el primer elemento debe ser el comando `set`.
///
/// # Parámetros
///
/// - `linea`: línea del archivo log a validar.
///
/// # Retorna
///
/// - `Ok(())` si la línea tiene un formato válido.
/// - `Err(ErrorMiniKv::InvalidLogFile)` si la línea no cumple con el formato esperado.
fn validar_linea_log(linea: &str) -> Result<(), ErrorMiniKv> {
    let partes = separar_argumentos(linea);

    match partes.as_slice() {
        [comando, _] | [comando, _, _] => match TipoComando::from_str(comando) {
            Ok(TipoComando::Set) => Ok(()),
            _ => Err(ErrorMiniKv::InvalidLogFile),
        },
        _ => Err(ErrorMiniKv::InvalidLogFile),
    }
}

/// Valida el formato de una línea del archivo de tipo data.
///
/// Una línea válida debe contener exactamente dos elementos:
/// - `clave valor`
///
/// # Parámetros
///
/// - `linea`: línea del archivo data a validar .
///
/// # Retorna
///
/// - `Ok(())` si la línea tiene un formato válido.
/// - `Err(ErrorMiniKv::InvalidDataFile)` si la línea no cumple con el formato esperado.
fn validar_linea_data(linea: &str) -> Result<(), ErrorMiniKv> {
    let partes = separar_argumentos(linea);
    if partes.len() != 2 {
        return Err(ErrorMiniKv::InvalidDataFile);
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fs;
    pub fn borrar_archivos(paths: &[&str]) {
        for path in paths {
            let _ = fs::remove_file(path);
        }
    }

    #[test]
    fn test_01_append_linea_log_crea_archivo() {
        use std::fs;

        let path_log = ".test01.minikv.log";
        borrar_archivos(&[path_log]);

        append_linea_log("set clave valor\n", path_log).unwrap();

        assert!(fs::metadata(path_log).is_ok());

        borrar_archivos(&[path_log]);
    }
    #[test]
    fn test_02_append_linea_log_escribe_linea() {
        let path_log = ".test02.minikv.log";
        borrar_archivos(&[path_log]);

        append_linea_log("set \"clave\" \"valor\"\n", path_log).unwrap();

        let file = File::open(path_log).unwrap();
        let mut reader = BufReader::new(file);

        let mut linea = String::new();
        reader.read_line(&mut linea).unwrap();

        assert_eq!(linea, "set \"clave\" \"valor\"\n");

        borrar_archivos(&[path_log]);
    }
    #[test]
    fn test_03_append_linea_log_agrega_lineas() {
        let path_log = ".test03.minikv.log";
        borrar_archivos(&[path_log]);

        append_linea_log("set \"clave1\" \"valor1\"\n", path_log).unwrap();
        append_linea_log("set \"clave2\" \"valor2\"\n", path_log).unwrap();

        let file = File::open(path_log).unwrap();
        let mut reader = BufReader::new(file);

        let mut l1 = String::new();
        let mut l2 = String::new();

        reader.read_line(&mut l1).unwrap();
        reader.read_line(&mut l2).unwrap();

        assert_eq!(l1, "set \"clave1\" \"valor1\"\n");
        assert_eq!(l2, "set \"clave2\" \"valor2\"\n");
        borrar_archivos(&[path_log]);
    }
    #[test]
    fn test_04_append_linea_log_multiple_llamadas() {
        use std::fs::File;
        use std::io::{BufRead, BufReader};
        let path_log = ".test04.minikv.log";
        borrar_archivos(&[path_log]);

        for i in 0..5 {
            append_linea_log(&format!("set clave{} valor{}\n", i, i), path_log).unwrap();
        }
        let file = File::open(path_log).unwrap();
        let reader = BufReader::new(file);
        let mut encontro_clave0 = false;
        let mut encontro_clave4 = false;
        for linea_resultado in reader.lines() {
            let linea = linea_resultado.unwrap();

            if linea.contains("clave0") {
                encontro_clave0 = true;
            }
            if linea.contains("clave4") {
                encontro_clave4 = true;
            }
        }
        assert!(encontro_clave0);
        assert!(encontro_clave4);

        borrar_archivos(&[path_log]);
    }
    #[test]
    fn test_05_sobrescribir_data_crea_y_escribe() {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let path_data = ".test05.minikv.data";
        borrar_archivos(&[path_data]);

        sobrescribir_data("clave valor\n", path_data).unwrap();

        let file = File::open(path_data).unwrap();
        let mut reader = BufReader::new(file);

        let mut linea = String::new();
        reader.read_line(&mut linea).unwrap();

        assert_eq!(linea, "clave valor\n");

        borrar_archivos(&[path_data]);
    }
    #[test]
    fn test_06_sobrescribir_data_reemplaza_contenido() {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let path_data = ".test06.minikv.data";
        borrar_archivos(&[path_data]);

        sobrescribir_data("clave1 valor1\n", path_data).unwrap();
        sobrescribir_data("clave2 valor2\n", path_data).unwrap();

        let file = File::open(path_data).unwrap();
        let mut reader = BufReader::new(file);

        let mut linea = String::new();
        reader.read_line(&mut linea).unwrap();

        assert_eq!(linea, "clave2 valor2\n");

        borrar_archivos(&[path_data]);
    }
    #[test]
    fn test_07_sobrescribir_data_sobrescribe_contenido_vacio() {
        use std::fs;

        let path_data = ".test07.minikv.data";
        borrar_archivos(&[path_data]);

        sobrescribir_data("", path_data).unwrap();

        let metadata = fs::metadata(path_data).unwrap();
        assert_eq!(metadata.len(), 0);

        borrar_archivos(&[path_data]);
    }
    #[test]
    fn test_08_vaciar_log_elimina_contenido() {
        use std::fs;

        let path_log = ".test08.minikv.log";
        borrar_archivos(&[path_log]);

        fs::write(path_log, "set clave valor\n").unwrap();

        vaciar_log(path_log).unwrap();

        let metadata = fs::metadata(path_log).unwrap();
        assert_eq!(metadata.len(), 0);

        borrar_archivos(&[path_log]);
    }
    #[test]
    fn test_09_reconstruir_estado_solo_data() {
        use std::fs;

        let path_data = ".test09.minikv.data";
        let path_log = ".test09.minikv.log";
        let _ = fs::remove_file(path_data);
        let _ = fs::remove_file(path_log);

        fs::write(path_data, "clave1 valor1\nclave2 valor2\n").unwrap();

        let dic = reconstruir_estado(path_data, path_log).unwrap();

        assert_eq!(dic.get("clave1"), Some(&"valor1".to_string()));
        assert_eq!(dic.get("clave2"), Some(&"valor2".to_string()));

        let _ = fs::remove_file(path_data);
        let _ = fs::remove_file(path_log);
    }
    #[test]
    fn test_10_reconstruir_estado_log_sobrescribe_data() {
        use std::fs;

        let path_data = ".test10.minikv.data";
        let path_log = ".test10.minikv.log";
        let _ = fs::remove_file(path_data);
        let _ = fs::remove_file(path_log);

        fs::write(path_data, "clave1 valor1\n").unwrap();
        fs::write(path_log, "set clave1 valor2\n").unwrap();

        let dic = reconstruir_estado(path_data, path_log).unwrap();

        assert_eq!(dic.get("clave1"), Some(&"valor2".to_string()));

        let _ = fs::remove_file(path_data);
        let _ = fs::remove_file(path_log);
    }
    #[test]
    fn test_11_reconstruir_estado_unset() {
        use std::fs;

        let path_data = ".test11.minikv.data";
        let path_log = ".test11.minikv.log";
        let _ = fs::remove_file(path_data);
        let _ = fs::remove_file(path_log);

        fs::write(path_data, "clave1 valor1\n").unwrap();
        fs::write(path_log, "set clave1\n").unwrap();

        let dic = reconstruir_estado(path_data, path_log).unwrap();

        assert_eq!(dic.get("clave1"), None);

        let _ = fs::remove_file(path_data);
        let _ = fs::remove_file(path_log);
    }
    #[test]
    fn test_12_reconstruir_estado_solo_log() {
        use std::fs;

        let path_data = ".test12.minikv.data";
        let path_log = ".test12.minikv.log";
        let _ = fs::remove_file(path_data);
        let _ = fs::remove_file(path_log);

        fs::write(path_log, "set clave1 valor1\nset clave2 valor2\n").unwrap();

        let dic = reconstruir_estado(path_data, path_log).unwrap();

        assert_eq!(dic.get("clave1"), Some(&"valor1".to_string()));
        assert_eq!(dic.get("clave2"), Some(&"valor2".to_string()));
        let _ = fs::remove_file(path_data);
        let _ = fs::remove_file(path_log);
    }
    #[test]
    fn test_13_reconstruir_estado_sin_archivos() {
        use std::fs;

        let path_data = ".test13.minikv.data";
        let path_log = ".test13.minikv.log";
        let _ = fs::remove_file(path_data);
        let _ = fs::remove_file(path_log);

        let dic = reconstruir_estado(path_data, path_log).unwrap();

        assert_eq!(dic.len(), 0);
        let _ = fs::remove_file(path_data);
        let _ = fs::remove_file(path_log);
    }
    #[test]
    fn test_14_reconstruir_estado_linea_invalida() {
        use std::fs;

        let path_data = ".test14.minikv.data";
        let path_log = ".test14.minikv.log";
        let _ = fs::remove_file(path_data);
        let _ = fs::remove_file(path_log);

        fs::write(path_data, "esta es una linea invalida\n").unwrap();

        let resultado = reconstruir_estado(path_data, path_log);

        assert!(resultado.is_err());

        let _ = fs::remove_file(path_data);
        let _ = fs::remove_file(path_log);
    }
    #[test]
    fn test_15_cargar_data_en_memoria_correcta() {
        use std::collections::HashMap;
        use std::fs;

        let path_data = ".test15.minikv.data";
        let _ = fs::remove_file(path_data);

        fs::write(path_data, "clave1 valor1\nclave2 valor2\n").unwrap();

        let mut dic = HashMap::new();

        cargar_data_en_memoria(&mut dic, path_data).unwrap();

        assert_eq!(dic.get("clave1"), Some(&"valor1".to_string()));
        assert_eq!(dic.get("clave2"), Some(&"valor2".to_string()));
        let _ = fs::remove_file(path_data);
    }
    #[test]
    fn test_16_cargar_data_en_memoria_linea_invalida() {
        use std::collections::HashMap;
        use std::fs;

        let path_data = ".test16.minikv.data";
        let _ = fs::remove_file(path_data);

        fs::write(path_data, "clave1 valor1 extra\n").unwrap(); // inválida

        let mut dic = HashMap::new();

        let resultado = cargar_data_en_memoria(&mut dic, path_data);

        assert!(resultado.is_err());

        let _ = fs::remove_file(path_data);
    }
    #[test]
    fn test_17_cargar_log_en_memoria_la_clave_2_fue_actualizada_y_la_clave_1_fue_eliminada() {
        use std::collections::HashMap;
        use std::fs;

        let path_log = ".test17.minikv.log";
        let _ = fs::remove_file(path_log);
        fs::write(
            path_log,
            "set clave1 valor1\nset clave2 valor2\nset clave1 valor3\nset clave2\n",
        )
        .unwrap();

        let mut dic = HashMap::new();

        cargar_log_en_memoria(&mut dic, path_log).unwrap();

        // clave1 fue actualizada
        assert_eq!(dic.get("clave1"), Some(&"valor3".to_string()));

        // clave2 fue eliminada
        assert_eq!(dic.get("clave2"), None);

        let _ = fs::remove_file(path_log);
    }
    #[test]
    fn test_18_cargar_log_en_memoria_linea_invalida() {
        use std::collections::HashMap;
        use std::fs;

        let path_log = ".test18.minikv.log";
        let _ = fs::remove_file(path_log);

        fs::write(path_log, "comando invalido\n").unwrap();

        let mut dic = HashMap::new();

        let resultado = cargar_log_en_memoria(&mut dic, path_log);

        assert!(resultado.is_err());

        let _ = fs::remove_file(path_log);
    }
    #[test]
    fn test_19_validar_linea_log_valida_con_valor() {
        let resultado = validar_linea_log("set clave valor");

        assert!(resultado.is_ok());
    }
    #[test]
    fn test_20_validar_linea_log_valida_unset() {
        let resultado = validar_linea_log("set clave");

        assert!(resultado.is_ok());
    }
    #[test]
    fn test_21_validar_linea_log_comando_invalido() {
        let resultado = validar_linea_log("get clave valor");

        assert!(resultado.is_err());
    }
    #[test]
    fn test_22_validar_linea_log_argumentos_invalidos() {
        let resultado = validar_linea_log("set");

        assert!(resultado.is_err());
    }
    #[test]
    fn test_23_validar_linea_data_valida() {
        let resultado = validar_linea_data("clave valor");

        assert!(resultado.is_ok());
    }
    #[test]
    fn test_24_validar_linea_data_muchos_argumentos() {
        let resultado = validar_linea_data("clave valor extra");

        assert!(resultado.is_err());
    }
    #[test]
    fn test_25_validar_linea_data_pocos_argumentos() {
        let resultado = validar_linea_data("clave");

        assert!(resultado.is_err());
    }
}
