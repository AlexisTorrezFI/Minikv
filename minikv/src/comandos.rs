use crate::parser::serializar;
use crate::errores::ErrorMiniKv;
use crate::storage::append_linea_log;
use crate::storage::reconstruir_estado;
use crate::storage::sobrescribir_data;
use crate::storage::vaciar_log;


pub fn comando_set(clave: String, valor: String) -> Result<(), ErrorMiniKv> {
    let clave_log = serializar(&clave);
    let valor_log = serializar(&valor);
    let linea_log = format!("set {} {}\n", clave_log, valor_log);
    let resultado = append_linea_log(&linea_log);
    match resultado {
        Ok(()) => Ok(()),
        Err(e) => {
            Err(e)
        }
    }
}
pub fn comando_unset(clave: String)  -> Result<(),ErrorMiniKv> {
    let clave_log = serializar(&clave);
    let linea_log = format!("set {}\n", clave_log);
    let resultado = append_linea_log(&linea_log);
    match resultado {
        Ok(()) => Ok(()),
        Err(e) => {
            Err(e)
        }
    }
 
}
pub fn comando_get(clave: String) -> Result<Option<String>, ErrorMiniKv> {
    let respuesta = reconstruir_estado();
    match respuesta {
        Ok(diccionario) => {
            Ok(diccionario.get(&clave).cloned())
        }
        Err(e) => Err(e),
    }
}
pub fn comando_length() -> Result<usize, ErrorMiniKv> {
    let respuesta = reconstruir_estado();
    match respuesta {
        Ok(diccionario) => {
            Ok(diccionario.len())
        }
        Err(e) => Err(e),
    }
}
pub fn comando_snapshot() -> Result<(), ErrorMiniKv> {
    let respuesta = reconstruir_estado();
    match respuesta {
        Ok(diccionario) => {
            let mut contenido = String::new();
            for (clave, valor) in diccionario {
                let clave_serializada = serializar(&clave);
                let valor_serializado = serializar(&valor);
                contenido.push_str(&format!("{} {}\n", clave_serializada, valor_serializado));
            }
            if let Err(e) = sobrescribir_data(&contenido) {
                return Err(e);
            }
            if let Err(e) = vaciar_log() {
                return Err(e);
            }
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

        let _ =comando_set("clave1".to_string(), "valor1".to_string());

        let contenido = fs::read_to_string(".minikv.log")
            .expect("no se pudo leer .minikv.log");

        assert_eq!(contenido, "set clave1 valor1\n");
    }
    #[test]
    fn test_03_comando_unset_crea_archivo() {
        use std::fs;
        let _ = fs::remove_file(".minikv.log");
        let _ =comando_unset("clave1".to_string());
        assert!(fs::metadata(".minikv.log").is_ok());
    }
    #[test]
    fn test_04_comando_unset_escribe_linea_en_log() {
        use std::fs;
        let _ = fs::remove_file(".minikv.log");
        let _ = comando_unset("clave1".to_string());
        let contenido = fs::read_to_string(".minikv.log")
            .expect("no se pudo leer .minikv.log");

        assert_eq!(contenido, "set clave1\n");
    }
}
