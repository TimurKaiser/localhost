use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};


// mod favicon_handler;

// doc associé https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.bind
fn main() -> std::io::Result<()> {
    // ? permet de renvoyer une erreur si il y en a une
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server listening on port 8080\n http://127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
                println!("Connection established!");
            }
            Err(e) => {
                // eprintln! permet d'écrire sur la sortie d'erreur
                eprintln!("Error: {}", e);
            }
        }
    }   
    Ok(())
}


fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(_) => {
            let request = String::from_utf8_lossy(&buffer);
            println!("Received a request: {}", request);

            // Ceci ne marche pas 
            // if request.starts_with("GET /favicon.ico") {
            //     println!("Request is for a favicon");
            //     favicon_handler::handle_favicon(stream);
            //     return;
            // }   

            // réponse  minimaliste du serveur
            let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\nHello, world!\n    ";

            // convertit la réponse en tableau d'octrés car les sockets ne peuvent envoyer que des octets
            // write et flush, remplie et envoie le buffer (système tamponné)
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}