use std::fs;
use std::io::Write;
use std::net::TcpStream;

pub fn handle_favicon(mut stream: TcpStream) {
    let contents = fs::read("assets/favicon.ico").unwrap_or_default();

    let response = if contents.is_empty() {
        "HTTP/1.1 204 No Content\r\n\r\n".to_string()
    } else {
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: image/x-icon\r\nContent-Length: {}\r\n\r\n",
            contents.len()
        )
    };

    stream.write_all(response.as_bytes()).unwrap();
    if !contents.is_empty() {
        stream.write_all(&contents).unwrap();
    }
    stream.flush().unwrap();
}
