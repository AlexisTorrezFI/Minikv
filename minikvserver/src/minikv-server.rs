use std::net::TcpListener;

fn main() {
    let Ok(tcp_listener) = TcpListener::bind("127.0.0.1:8080") else {
        eprintln!("Fallo en la coneccion del servidor");
        return;
    };
    println!("Server listening on 127.0.0.1:8080");
}
