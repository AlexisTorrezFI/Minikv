mod comandos;
use std::env;
use crate::comandos::comando_set;
use crate::comandos::comando_unset;


fn main() {
    let mut args = env::args();
    args.next();
    let comando_option: Option<String> = args.next();
    let clave_option: Option<String> = args.next();
    let valor_option: Option<String> = args.next();

    match (comando_option,clave_option,valor_option) {
        (Some(comando),Some(clave),Some(valor)) if comando == "set" => {
            comando_set(clave, valor);
        },
        (Some(comando),Some(clave),None) if comando == "set" => {
            comando_unset(clave);
        },
        //get
        (Some(comando),Some(clave),None) if comando == "get" => {
            println!("Obteniendo valor de {}",clave);
            //comando_get(clave);
        },
        //length
        (Some(comando),None,None) if comando == "lenght" => {
            println!("Obteniendo cantidad de claves asignadas");
        },
        //snapshot
        (Some(comando),None,None) if comando == "snapshot" => {
            println!("Haciendo snapshot de las claves asignadas");
        },
        //nothing
        _ => {
            println!("Comando no reconocido o argumentos incorrectos!");
        }
    }
}
