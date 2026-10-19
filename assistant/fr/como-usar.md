# Comment l'utiliser

Les étapes ci-dessous utilisent les noms des commandes. Une tentative est une hypothèse. Une mesure est un essai. Les tentatives que personne n'a battues sur tous les nombres sont la frontière.

1. `acvc context --campaign NOM` avant de proposer, mesurer ou écarter.
2. Si la campagne n'existe pas : `acvc init --repo .` et `acvc campaign create NOM --metric metrique:higher`.
3. La première hypothèse est `seed` du dossier qui contient les fichiers de l'hypothèse, avec `--premise`, `--assumption`, `--intent` et `--agent`.
4. Un fichier a changé : `record` à partir du parent. Ne faites pas un autre `seed`.
5. Il y a un nombre réel pour la métrique déclarée : `score`. Un essai incomplet n'entre pas sur la frontière.
6. `frontier` montre ce qui est encore en concurrence. `why` explique une hypothèse. `recall --query` la trouve par le texte de la prémisse.
7. Deux mesures concrètes : `compare`.
8. L'hypothèse est morte pour un motif connu : `reject --reason`. Sans motif, ne rejetez pas. La lecture dit déjà que le motif n'a pas été enregistré.
9. La session a dépensé des jetons et ce nombre est connu : `spend`. N'estimez pas.
10. La variante qui a gagné entre dans Git avec `promote` dans un checkout qui existe déjà.

Préfixe d'identifiant : 8 à 64 caractères hexadécimaux, sans ambiguïté.

Pour un autre agent sur la même machine, le canal est l'exécutable ou `acvc mcp`. La page et la TUI sont pour la personne. Ne les remplacez pas par une lecture directe de la base.
