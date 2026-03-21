pub fn separar_argumentos(texto: &str) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();
    let mut actual: String = String::new();
    let mut en_comillas: bool = false;
    let mut escape: bool = false;
    for c in texto.chars() {
        if escape {
            actual.push(c);
            escape = false;
            continue;
        }
        if c == '\\' {
            escape = true;
            continue;
        }
        if c == '"' {
            en_comillas = !en_comillas;
            continue;
        }
        if c == ' ' && !en_comillas {
            if !actual.is_empty() {
                args.push(actual);
                actual = String::new();
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

pub fn serializar(texto: &str) -> String {
    let mut resultado = String::new();
    for c in texto.chars() {
        if c == '"' {
            resultado.push('\\');
        }
        resultado.push(c);
    }
    if resultado.contains(' ') {
        let mut final_str = String::new();
        final_str.push('"');
        final_str.push_str(&resultado);
        final_str.push('"');
        final_str
    } else {
        resultado
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_07_serializar_valor_con_comillas() {
        let resultado = serializar("valor\"3");
        assert_eq!(resultado, "valor\\\"3");
    }
    #[test]
    fn test_08_serializar_valor_con_comillas_y_espacio() {
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
    #[test]
    fn test_14_separar_con_comillas_y_espacios() {
        let resultado = separar_argumentos(r#"set clave\"1 valor\"1"#);
        assert_eq!(resultado, vec!["set", "clave\"1", "valor\"1"]);
    }
}
