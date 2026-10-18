# Trade-offs

## Utilisez ACVC quand

- La session teste plus d'une hypothèse et la suivante doit savoir laquelle est encore en concurrence.
- La prémisse change, et le motif de l'abandon compte autant que le nombre.
- Le dernier essai n'est pas, à lui seul, le meilleur.
- Plusieurs métriques tirent dans des sens opposés et les deux hypothèses doivent rester visibles.
- Un autre agent, des heures plus tard, fera un recall au lieu de relire le chat.

## Ne l'utilisez pas quand

- Le travail est un seul chemin linéaire et Git raconte déjà l'histoire qui compte.
- Il n'y a ni métrique déclarée ni prémisse. Sans cela, ACVC devient un second Git sans frontière.
- La question est seulement « ce qu'il y a dans le working tree maintenant ». C'est `git status` ou `acvc status`, pas une nouvelle campagne.
- Le coût d'écrire la prémisse, la cause et la mesure est plus haut que le coût d'oublier la tentative.

## Ce que l'on gagne

Mémoire d'hypothèse, frontière de Pareto, recall de la prémisse, motif séparé du score, et un dépôt Git seulement de la variante qui a gagné.

## Ce que l'on paie

Un store `.acvc` à côté de Git. La discipline de ne pas ouvrir ce store à la main. Une métrique déclarée avant l'essai. Le parcours a sa propre liste, donc un gros fichier entre dans le hash s'il n'est ni dans `.acvcignore` ni dans la liste fixe. Le distant copie les objets. Il ne fait pas le merge que Git ferait.

## Agents dans Cursor, Claude Code et OpenCode

La ressource que ces agents doivent appeler est `acvc` sur le `PATH`, ou le serveur `acvc mcp` si la session l'a déjà enregistré. Les deux parlent JSON. Aucun n'ouvre SQLite. Ce dossier énonce le contrat. Le code de l'exécutable reste l'implémentation.
