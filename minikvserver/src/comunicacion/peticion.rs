

use std::sync::mpsc::Sender;

use crate::{comunicacion::respuesta::Respuesta, minikv::comandos::Comando};

pub struct Peticion {
    pub comando: Comando,
    pub tx_respuesta: Sender<Respuesta>,
}