Pour tester notre code utiliser `curl`
```
curl -v http://127.0.0.1:8080
```
Réponse
```
Server listening on port 8080
Received a request: GET / HTTP/1.1
Host: 127.0.0.1:8080
User-Agent: curl/8.9.1
Accept: */*


Connection established!
```

On peut également se connecter depuis le navigateur et la réponse est intéressante

```
http://127.0.0.1:8080/
```
Réponse
'''
Server listening on port 8080
Received a request: GET / HTTP/1.1
Host: 127.0.0.1:8080
User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:136.0) Gecko/20100101 Firefox/136.0
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
Accept-Language: fr,fr-FR;q=0.8,en-US;q=0.5,en;q=0.3
Accept-Encoding: gzip, deflate, br, zstd
Connection: keep-alive
Upgrade-Insecure-Requests: 1
Sec-Fetch-Dest: document
Sec-Fetch-Mode: navigate
Sec-Fetch-Site: none
Sec-Fetch-User: ?1
Priority: u=0, i


Connection established!
```