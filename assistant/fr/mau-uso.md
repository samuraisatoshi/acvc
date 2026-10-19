# Mauvais usage

Voici les façons dont le registre cesse de dire ce qu'il dit. Les essais ratés restent ici. Git ne reçoit que la tentative qu'une personne approuve.

- Ouvrir `.acvc/graph.sqlite`, `objects.log` ou `objects.pack` pour lire le statut, la frontière ou la prémisse. Le contrat est l'exécutable.
- Traiter Git comme le graphe d'hypothèses. Le commit ne garde pas l'essai perdant ni le motif qui n'a pas été écrit.
- Un second `seed` dans la même campagne. La campagne a déjà une origine. Le chemin est `record` à partir du parent.
- Rejeter parce qu'une autre hypothèse a dominé. Cela enregistre un motif que personne n'a donné.
- Fusionner deux hypothèses incomparables, ou relier des seeds isolés, pour que la frontière ait un seul vainqueur.
- Utiliser `spend` comme budget, métrique de frontière ou verrou de `record`. Inventer un nombre de jetons.
- Croire que le dernier `score` efface le précédent, ou mélanger `tests_passed` d'un essai avec `latency_ms` d'un autre.
- Lire `.gitignore` comme la politique du parcours. La politique explicite est `.acvcignore`.
- Un hardlink entre arbres. Un symlink dans le dossier semé.
- `promote` comme un merge, ou `push`/`fetch` en attendant un checkout de l'autre côté.
- Montrer un hash, une arête ou « left/right » à la personne. La phrase est la prémisse et celle qui reste.
- Coller toutes les causes dans un seul besoin de campagne quand le graphe a plus d'une cause.
- Réécrire une hypothèse déjà enregistrée pour « corriger » la prémisse. Une prémisse nouvelle est `revise_premise` ou `record`, un autre nœud.
- Lancer `acvc web` ou `acvc tui` depuis la session de l'agent quand la personne a déjà le visualiseur ouvert. Donnez-lui la commande pour son terminal.
