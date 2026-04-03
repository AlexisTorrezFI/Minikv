/// Separa una cadena de texto en argumentos, respetando comillas y caracteres escapados.
///
/// Esta función procesa una línea de texto y la divide en un vector de `String`,
/// utilizando espacios como separadores, excepto cuando los argumentos están
/// encerrados entre comillas (`"`).
///
/// También permite el uso de caracteres escapados mediante `\`, lo que permite
/// incluir comillas u otros caracteres especiales dentro de un argumento.
///
/// # Reglas de parseo
///
/// - Los argumentos se separan por espacios.
/// - Las comillas (`"`) agrupan texto en un solo argumento.
/// - El carácter `\` permite escapar el siguiente carácter.
/// - Las comillas no forman parte del resultado final.
/// - Los espacios dentro de comillas se preservan.
///
/// # Parámetros
///
/// - `texto`: cadena de entrada a parsear.
///
/// # Retorna
///
/// - Un `Vec<String>` con los argumentos separados.
pub fn separar_argumentos(texto: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut actual = String::new();
    let mut en_comillas = false;
    let mut escape = false;

    for c in texto.chars() {
        match (escape, c) {
            (true, _) => {
                actual.push(c);
                escape = false;
            }
            (false, '\\') => escape = true,
            (false, '"') => en_comillas = !en_comillas,
            (false, ' ') if !en_comillas => {
                if !actual.is_empty() {
                    args.push(actual);
                    actual = String::new();
                }
            }
            _ => actual.push(c),
        }
    }

    if !actual.is_empty() {
        args.push(actual);
    }

    args
}

/// Serializa una cadena de texto para su almacenamiento en archivos.
///
/// Esta función transforma un texto para que pueda ser escrito en los archivos
/// `.minikv.data` o `.minikv.log`, asegurando que pueda ser correctamente
/// interpretado posteriormente por el parser.
///
/// # Comportamiento
///
/// - Escapa las comillas (`"`) agregando un `\` antes de cada una.
/// - Si el texto contiene espacios, lo envuelve entre comillas (`"`).
/// - Si no contiene espacios, se deja sin comillas externas.
///
/// # Parámetros
///
/// - `texto`: cadena a serializar.
///
/// # Retorna
///
/// - Un `String` con el texto serializado.
pub fn serializar(texto: &str) -> String {
    let mut resultado = String::new();
    for c in texto.chars() {
        if c == '"' {
            resultado.push('\\');
        }
        resultado.push(c);
    }
    let mut final_str = String::new();
    final_str.push('"');
    final_str.push_str(&resultado);
    final_str.push('"');
    final_str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01_serializar_valor_simple_con_comillas() {
        let resultado = serializar("valor1");
        assert_eq!(resultado, "\"valor1\"");
    }
    #[test]
    fn test_02_serializar_valor_con_espacio() {
        let resultado = serializar("valor 2");
        assert_eq!(resultado, "\"valor 2\"");
    }
    #[test]
    fn test_03_serializar_valor_con_comillas() {
        let resultado = serializar("valor\"3");
        assert_eq!(resultado, "\"valor\\\"3\"");
    }
    #[test]
    fn test_04_serializar_valor_con_comillas_y_espacio() {
        let resultado = serializar("valor \"4");
        assert_eq!(resultado, "\"valor \\\"4\"");
    }

    #[test]
    fn test_05_separar_argumentos_simple() {
        let resultado = separar_argumentos("set clave1 valor1");
        assert_eq!(resultado, vec!["set", "clave1", "valor1"]);
    }
    #[test]
    fn test_06_separar_argumentos_para_unset() {
        let resultado = separar_argumentos("set clave1");
        assert_eq!(resultado, vec!["set", "clave1"]);
    }
    #[test]
    fn test_07_separar_argumentos_con_espacios() {
        let resultado = separar_argumentos(r#"set "clave 1" "valor 1""#);
        assert_eq!(resultado, vec!["set", "clave 1", "valor 1"]);
    }
    #[test]
    fn test_08_separar_argumentos_con_comillas() {
        let resultado = separar_argumentos(r#"set "clave \"1" valor1"#);
        assert_eq!(resultado, vec!["set", "clave \"1", "valor1"]);
    }
    #[test]
    fn test_09_separar_argumentos_con_comillas_y_espacios() {
        let resultado = separar_argumentos(r#"set "clave \"1" "valor 1""#);
        assert_eq!(resultado, vec!["set", "clave \"1", "valor 1"]);
    }
    #[test]
    fn test_10_separar_argumentos_con_comillas_y_espacios() {
        let resultado = separar_argumentos(r#"set clave\"1 valor\"1"#);
        assert_eq!(resultado, vec!["set", "clave\"1", "valor\"1"]);
    }
}
