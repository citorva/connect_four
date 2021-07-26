# Défis Not a Name n°8 : Puissance 4

Implémentation du jeu de puissance 4 en Rust.

Ce dernier implémente les éléments suivant :
 * [x] Gestion des parties via un système générique en terme d’interface
 * [x] Interface en ligne de commande simple permettant les interactions entre le joueur et le jeu
 * [x] Intelligence artificielle jouant ses coups au hasard
 * [ ] Interface graphique pour le jeu
 * [ ] Intelligence artificielle plus élaborée avec différents niveaux de difficulté

## Implémentation d’une interface graphique

Afin de fournir une interface utilisateur plus élaborée, vous pouvez utiliser le moteur de jeu comme suit:

 1. Création des interfaces entre le joueur et le jeu via le trait `connect_four::Interface`
 2. Création d’une instance de la partie via la création d’un objet `connect_four::Engine`
 3. Affichage de l’aire de jeu via les opérations sur les tableaux sur l’objet `connect_four::Area` en utilisant un
uplet sous la forme `(colonne, ligne)` avec la ligne partant du bas.
    
## Implémentation d’une nouvelle intelligence artificielle

Une intelligence artificielle doit implémenter le trait `connect_four::Interface` et être donné à la création d’un objet
`connect_four::Engine` ou via l’appel de `connect_four::Engine::set_player`