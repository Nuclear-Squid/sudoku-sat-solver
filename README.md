Projet Sudoku Inf402
====================

Auteurs :

- Ivanova Antoria
- Chevrier-Pivot Yohan
- Cazenave Léo

# Présentation rapide du projet

La majorité du code se trouve dans un projet Rust nommé `sudoku_dimacs`.

Une fois compilé (un Makefile minimaliste est à disposition), deux fichiers
executables sont générés : `encode-sat` et `decode-sat`. Le premier permet
d’inscrire les clauses associées à un sudoku donné au format DIMACS dans un
fichier de sortie, et le second permet de parser la sortie du solveur-sat
`minisat` pour la rendre lisible par un humain.

Pour aller plus vite, il y a un script shell nommé `resolution_rapide_sudoku.sh`
qui prends en argument plusieurs instances de sudoku, chacunes dans des fichiers
distincts et les résouds à la suite.

Malheureusement, nous n’avons pas eu le temps d’implémenter un solveur-sat.

Un exemple de grille de sudoku et un template sont disponibles à la racine du
projet, respectivement sous les noms `exemple_sudoku.txt` et `grille_sudoku_vide.txt`.
Quelques instances de tests ont été founies dans le dossier `grilles_test`

# Précisions sur les différents fichiers executables

## encode-sat

`encode-sat` peux prendre un ou deux arguments dans la ligne de commande. Le
premier doit obligatoirement être le nom du fichier contenant le sudoku dont
on veut générer les clauses. Le second est le fichier dans lequel les clauses
sont inscrites. Si ce deuxième argument n’es pas précisé, les clauses seront
affichées dans la console. (Tout autre argument sera ignoré).

## decode-sat

`decode-sat` ne prends qu’un seul argument : le nom du fichier contenant la
sortie du solveur-sat (on a choisi `minisat`). Comme pour `encode-sat`, tout
autre argument sera ignoré.

Si l’ensemble de clauses est insatisfaisable, on en conclut que le sudoku est
insolluble, et un message sera affiché dans le terminal.

Dans le cas où le solveur-sat donne un modèle de l’ensemble de clauses, le
programme va vérifier que la grille donnée par le solveur-sat est valide.
Si plusieurs chiffres ont été inscrits à la même case, le programme va juste
afficher un message d’erreur (puisque le conflit empêche d’interpréter le
résultat). Si la grille n’es pas valide, elle sera affichée avec un
avertissement au dessus. Enfin, si la grille est valide, elle sera directement
affichée, sans précision en plus.
