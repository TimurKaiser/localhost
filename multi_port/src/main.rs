use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let ports = [8080, 8081, 8082];

    // un seul thread pour écouter plusieurs ports
    let listeners: Vec<TcpListener> = ports
        .iter()
        .map(|&port| TcpListener::bind(("127.0.0.1", port)).expect("Failed to bind"))
        .collect();

    for port in &ports {
        println!("Server listening on port http://127.0.0.1:{}", port);
    }

    loop {
        for listener in &listeners {
            if let Ok((stream, _)) = listener.accept() {
                handle_client(stream);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    if let Ok(_) = stream.read(&mut buffer) {
        let request = String::from_utf8_lossy(&buffer);
        println!("Received a request: {}", request);

        // réponse minimaliste du serveur
        let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\nHello, world!";

        // convertit la réponse en tableau d'octrés car les sockets ne peuvent envoyer que des octets
        // write et flush, remplie et envoie le buffer (système tamponné)
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}






// VERSION 1

//use std::net::{TcpListener, TcpStream};
//use std::io::{Read, Write};
//use std::thread;
//
//
//fn main() -> std::io::Result<()> {
//    let ports = vec![8080, 9090];
//
//    // thtead::spawn permet de créer un nouveau thread pour chaque port, un thread est une unité d'exécution
//    // qui permet de lancer plusieurs tâches en parallèle
//    for port in ports {
//        thread::spawn(move || {
//            start_server(port).unwrap();
//        });
//    }
//
//    // Empêche le programme de se terminer immédiatement
//    loop {
//        thread::park();
//    }
//}
//
//fn start_server(port: u16) -> std::io::Result<()> {
//
//    let address = format!("127.0.0.1:{}", port);
//
//    // ? permet de renvoyer une erreur si il y en a une
//    let listener = TcpListener::bind(&address)?;
//    println!("Server listening on port {}\n http://{}", port, address);
//
//    for stream in listener.incoming() {
//        match stream {
//            Ok(stream) => {
//                thread::spawn(|| handle_client(stream));
//                println!("Connection established on port {}", port);
//            }
//            Err(e) => {
//                // eprintln! permet d'écrire sur la sortie d'erreura
//                eprintln!("Error: {}", e);
//            }
//        }
//    }
//    Ok(())
//}
//
//fn handle_client(mut stream: TcpStream) {
//    let mut buffer = [0; 1024];
//    match stream.read(&mut buffer) {
//        Ok(_) => {
//            let request = String::from_utf8_lossy(&buffer);
//            println!("Received a request: {}", request);
//
//
//            // réponse  minimaliste du serveur
//            let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\nHello, world!\n";
//
//            // convertit la réponse en tableau d'octrés car les sockets ne peuvent envoyer que des octets
//            // write et flush, remplie et envoie le buffer (système tamponné)
//            stream.write(response.as_bytes()).unwrap();
//            stream.flush().unwrap();
//        }
//        Err(e) => {
//            eprintln!("Error: {}", e);
//        }
//    }
//}
//