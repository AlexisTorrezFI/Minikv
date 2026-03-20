use std::fs::OpenOptions;
use std::io::Write;
//use std::collections::HashMap;

pub fn comando_set(clave: String, valor: String) {
    println!("Asignando {} a {}",clave,valor);
    let archivo_result = OpenOptions::new()
        .create(true)
        .append(true)
        .open(".minikv.log");
    match archivo_result {
        Ok(mut file) => {
            let clave_log = serializar(&clave);
            let valor_log = serializar(&valor);
            let linea = format!("set {} {}\n", clave_log, valor_log);
            match file.write_all(linea.as_bytes()) {
                Ok(()) => println!("OK"),
                Err(e) => println!("Error al escribir en el archivo .minikv.log: {}", e),
            }
        }
        Err(_e) => {
            println!("Archivo .minikv.log no se ha podido crear.");
        }           
    }
}

pub fn comando_unset(clave: String) {
    let archivo_result = OpenOptions::new()
        .create(true)
        .append(true)
        .open(".minikv.log");
    match archivo_result {
        Ok(mut file) => {
            let clave_log = serializar(&clave);
            let linea = format!("set {}\n", clave_log);
            match file.write_all(linea.as_bytes()) {
                Ok(()) => println!("OK"),
                Err(e) => println!("Error al escribir en el archivo .minikv.log: {}", e),
            }
        }
        Err(_e) => {
            println!("Archivo .minikv.log no se ha podido crear.");
        }           
    }
}
pb fn comando_get(clave: String) {
    let diccionario = crear_diccionario_clave_valor();

    match diccionario.get(&clave) {
        Some(valor) => println!("{}", valor),
        None => println!("NOT FOUND"),
    }
}
fn crear_diccionario_clave_valor() -> HashMap<String, String> {
    let mut diccionario = HashMap::new();

    if let Ok(contenido) = fs::read_to_string(".minikv.log") {
        for linea in contenido.lines() {
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
                _ => {}
            }
        }
    }
    diccionario
}

fn separar_argumentos(texto: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut actual = String::new();
    let mut en_comillas = false;
    let mut escape = false;
    for c in texto.chars() {
        if escape {
            actual.push(c);
            escape = false;
            continue;
        }
        if c == '\\' && en_comillas {
            escape = true;
            continue;
        }
        if c == '"' {
            en_comillas = !en_comillas;
            continue;
        }
        if c == ' ' && !en_comillas {
            if !actual.is_empty() {
                args.push(actual.clone());
                actual.clear();
            }
            continue;
        }
        actual.push(c);
    }
    if !actual.is_empty() {
        args.push(actual);
    }
    args
}




fn serializar(texto: &str) -> String {
    let mut resultado = String::new();
    for c in texto.chars() {
        if c == '"' {
            resultado.push('\\');       
        }
        resultado.push(c);
    }
    if resultado.contains(' ') {
        format!("\"{}\"", resultado)
    }else {
        resultado
    }
}

#[cfg(test)]
mod tests {
    use super::*;

   #[test]
    fn test_01_comando_set_crea_archivo() {
        use std::fs;
        let _ = fs::remove_file(".minikv.log");
        comando_set("clave1".to_string(), "valor1".to_string());
        assert!(fs::metadata(".minikv.log").is_ok());
    }
    #[test]
    fn test_02_comando_set_escribe_linea_en_log() {
        use std::fs;

        let _ = fs::remove_file(".minikv.log");

        comando_set("clave1".to_string(), "valor1".to_string());

        let contenido = fs::read_to_string(".minikv.log")
            .expect("no se pudo leer .minikv.log");

        assert_eq!(contenido, "set clave1 valor1\n");
    }
    #[test]
    fn test_03_comando_unset_crea_archivo() {
        use std::fs;
        let _ = fs::remove_file(".minikv.log");
        comando_unset("clave1".to_string());
        assert!(fs::metadata(".minikv.log").is_ok());
    }
    #[test]
    fn test_04_comando_unset_escribe_linea_en_log() {
        use std::fs;
        let _ = fs::remove_file(".minikv.log");
        comando_unset("clave1".to_string());
        let contenido = fs::read_to_string(".minikv.log")
            .expect("no se pudo leer .minikv.log");

        assert_eq!(contenido, "set clave1\n");
    }

    #[test]
    fn test_05_serializar_valor_simple_con_comillas() {
        let resultado = serializar("valor1");
        assert_eq!(resultado, "valor1");
    }
    #[test]
    fn test_06_serializar_valor_con_espacio() {
        let resultado = serializar("valor 2");
        assert_eq!(resultado, "\"valor 2\"");
    }
    #[test]
    fn test_07_serializar_valor_con_comillas(){
        let resultado = serializar("valor\"3");
        assert_eq!(resultado, "valor\\\"3");    
    }
    #[test]
    fn test_08_serializar_valor_con_comillas_y_espacio(){
        let resultado = serializar("valor \"4");
        assert_eq!(resultado, "\"valor \\\"4\"");   
    }    

    #[test]
    fn test_09_separar_argumentos_simple() {
        let resultado = separar_argumentos("set clave1 valor1");
        assert_eq!(resultado, vec!["set", "clave1", "valor1"]);
    }
    #[test]
    fn test_10_separar_argumentos_para_unset() {
        let resultado = separar_argumentos("set clave1");
        assert_eq!(resultado, vec!["set", "clave1"]);
    }
    #[test]
    fn test_11_separar_con_espacios() {
        let resultado = separar_argumentos(r#"set "clave 1" "valor 1""#);
        assert_eq!(resultado, vec!["set", "clave 1", "valor 1"]);
    }
    #[test]
    fn test_12_separar_con_comillas() {
        let resultado = separar_argumentos(r#"set "clave \"1" valor1"#);
        assert_eq!(resultado, vec!["set", "clave \"1", "valor1"]);
    }
    #[test]
    fn test_13_separar_con_comillas_y_espacios() {
        let resultado = separar_argumentos(r#"set "clave \"1" "valor 1""#);
        assert_eq!(resultado, vec!["set", "clave \"1", "valor 1"]);
    }
    
}
