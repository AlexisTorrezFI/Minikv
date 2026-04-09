use crate::minikv::errores::ErrorCliente;
use crate::minikv::errores::ErrorServidorDatos;

pub enum Respuesta {
    Ok,
    Valor(String),
    Cantidad(usize),
    ErrorCliente(ErrorCliente),
    ErrorServidor(ErrorServidorDatos),
}