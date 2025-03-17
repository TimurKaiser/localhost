Cahier des charges

1. Objectif

Développer un serveur HTTP minimaliste et performant en Rust, capable de servir des pages statiques et d’exécuter des scripts CGI, sans bibliothèques externes comme tokio ou nix.
2. Contraintes techniques

    Langage : Rust
    Bibliothèques autorisées : libc pour les appels système
    Programmation bas niveau : usage restreint de unsafe
    Mono-thread, mono-processus

3. Fonctionnalités

Serveur HTTP

    Gestion des requêtes et réponses en HTTP/1.1
    Prise en charge des méthodes GET, POST, DELETE
    Gestion des cookies et sessions
    Timeout des requêtes longues
    Upload de fichiers
    Gestion des erreurs HTTP avec pages personnalisées

Performances et stabilité

    Aucune fuite mémoire ni crash toléré
    I/O non bloquantes via epoll
    Multi-port et multi-instance possible
    99.5% de disponibilité sous stress-test

Fichier de configuration

    Définition des hôtes et ports
    Configuration des routes et redirections
    CGI mappé aux extensions de fichiers
    Taille limite d’upload configurable
    Activation/désactivation du listing de répertoires

CGI

    Prise en charge d’un langage de script (ex: PHP, Python)
    Exécution via fork
    Gestion correcte des chemins relatifs

4. Tests et validation

    Tests unitaires et fonctionnels :
        Redirections
        Gestion des erreurs
        Pages statiques et dynamiques
        Configuration incorrecte
    Stress-test avec siege -b
    Analyse des fuites mémoire

5. Bonus

    Ajout d’un second CGI
    Réécriture en C ou C++