La première version de notre code executer avec des threads lancer en parrallele, probleme; l'audit refuse le multi threads

```
for port in ports {
    thread::spawn(move || {
        start_server(port).unwrap();
    });
}
```

Ceci execute dans un seule thread

```
// un seul thread pour écouter plusieurs ports
let listeners: Vec<TcpListener> = ports
    .iter()
    .map(|&port| TcpListener::bind(("127.0.0.1", port)).expect("Failed to bind"))
    .collect();
```
