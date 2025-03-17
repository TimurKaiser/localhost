Roadmap Technique - Serveur HTTP en Rust

Phase 1 : Préparation et Conception
Objectifs :

    Définir l’architecture globale du serveur
    Étudier et comprendre l’utilisation de epoll pour la gestion des connexions
    Établir les structures de données pour représenter les requêtes et réponses HTTP
    Définir le format du fichier de configuration

Tâches :

    Étudier les appels système nécessaires (socket, bind, listen, accept, epoll, read, write, fork)
    Définir la structure des requêtes HTTP (méthodes, headers, corps)
    Définir la structure des réponses HTTP (status code, headers, corps)
    Spécifier un modèle de gestion des connexions basé sur epoll
    Définir le format et les options du fichier de configuration

Phase 2 : Développement du Serveur de Base
Objectifs :

    Implémenter un serveur minimaliste capable d’accepter et de gérer des connexions
    Mettre en place une gestion basique des requêtes HTTP

Tâches :

    Initialiser un socket serveur (socket, bind, listen)
    Implémenter une boucle d’événements basée sur epoll pour gérer les connexions
    Accepter et lire les requêtes (accept, read)
    Implémenter un parseur minimaliste pour analyser les requêtes HTTP
    Envoyer une réponse HTTP statique avec les headers nécessaires (write)

Phase 3 : Gestion Complète des Requêtes HTTP
Objectifs :

    Ajouter la gestion des méthodes POST et DELETE
    Implémenter la gestion des en-têtes et du corps de requête

Tâches :

    Améliorer le parseur HTTP pour prendre en charge les méthodes POST et DELETE
    Lire et gérer le corps des requêtes en fonction de Content-Length et Transfer-Encoding: chunked
    Implémenter un système de gestion des cookies et des sessions
    Vérifier la conformité aux spécifications HTTP/1.1
    Gérer les réponses en fonction des codes de statut (200, 400, 403, 404, 500)

Phase 4 : Système de Fichier et Gestion des Routes
Objectifs :

    Servir des fichiers statiques en fonction des routes définies
    Permettre la configuration des routes via un fichier de configuration

Tâches :

    Lire et analyser le fichier de configuration
    Implémenter un routeur qui mappe les URL aux fichiers correspondants
    Gérer les fichiers par défaut pour les répertoires (ex : index.html)
    Gérer les permissions et sécuriser l’accès aux fichiers sensibles
    Implémenter la gestion des erreurs avec des pages personnalisées

Phase 5 : Gestion des Entrées/Sorties Non-Bloquantes
Objectifs :

    Garantir que toutes les opérations d’E/S passent par epoll
    Assurer une lecture et une écriture non bloquantes

Tâches :

    Remplacer les lectures et écritures bloquantes par des appels non-bloquants
    Vérifier que epoll est appelé une seule fois par communication client/serveur
    Implémenter un buffer pour gérer les requêtes partielles et les grandes réponses
    Tester la scalabilité en simulant plusieurs connexions simultanées

Phase 6 : Implémentation du CGI
Objectifs :

    Permettre l’exécution de scripts dynamiques en fonction des extensions de fichiers
    Assurer une bonne isolation entre le serveur et les processus CGI

Tâches :

    Ajouter un gestionnaire pour détecter les fichiers exécutables (ex : .php, .py)
    Implémenter le fork et l’exécution du script avec les bonnes variables d’environnement (PATH_INFO, QUERY_STRING)
    Lire la sortie du script et la retransmettre au client via HTTP
    Vérifier la gestion des erreurs et des permissions des scripts exécutés
    Tester l’exécution de scripts dans différentes conditions

Phase 7 : Gestion des Uploads de Fichiers
Objectifs :

    Permettre l’upload de fichiers via POST
    Respecter les limites définies dans le fichier de configuration

Tâches :

    Lire les requêtes multipart (Content-Type: multipart/form-data)
    Extraire et stocker les fichiers envoyés
    Appliquer des restrictions de taille et de format
    Vérifier les permissions d’accès aux fichiers uploadés
    Tester l’envoi de fichiers de différentes tailles et formats

Phase 8 : Tests et Optimisation
Objectifs :

    Valider la stabilité et la performance du serveur
    Détecter et corriger d’éventuelles fuites mémoire

Tâches :

    Écrire des tests unitaires et fonctionnels pour chaque fonctionnalité
    Exécuter des tests de charge (siege -b [IP]:[PORT])
    Vérifier l’utilisation mémoire et détecter d’éventuelles fuites
    Comparer les performances avec NGINX et analyser les résultats
    Réaliser une documentation technique sur l’implémentation

Phase 9 : Bonus (Optionnel)
Objectifs :

    Ajouter des fonctionnalités supplémentaires
    Expérimenter une autre implémentation

Tâches :

    Implémenter un second CGI (ex : support de .php et .py)
    Ajouter une gestion avancée des logs serveur
    Réécrire le serveur en C ou C++ pour comparaison
    Ajouter des métriques et des statistiques sur l’utilisation du serveur

Cette roadmap définit une approche méthodique et détaillée pour développer un serveur HTTP en Rust, en respectant toutes les contraintes et en assurant une implémentation robuste.

