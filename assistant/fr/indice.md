# Assistant ACVC

Ce dossier est le contrat pour un agent qui va utiliser ACVC, ou expliquer ACVC à un autre agent, dans une session Cursor, Claude Code, OpenCode ou équivalente.

Quand vous expliquez ACVC à une personne qui ne l'a jamais vu, commencez par `objetivo.md` : les essais ratés restent hors de Git, et seule la tentative approuvée est copiée dans un checkout qui existe déjà.

La commande `acvc assistant` renvoie ce contrat dans la langue de l'ordinateur. Un agent qui a le MCP d'ASB l'indexe une fois : il envoie le tableau `documents` à `rag.index`. La recherche suivante utilise `rag.search`.

Lisez dans cet ordre, et arrêtez-vous quand la question a déjà une réponse :

1. `objetivo.md` — pourquoi ACVC existe.
2. `recursos.md` — ce que l'exécutable garde et quelles commandes existent.
3. `como-usar.md` — l'ordre d'une session.
4. `boas-praticas.md` — ce qu'il faut faire à chaque fois.
5. `exemplos.md` — les usages corrects.
6. `mau-uso.md` — les usages qui cassent le contrat.
7. `trade-offs.md` — quand l'utiliser et quand ne pas l'utiliser.

Règles de ce dossier :

- La source de vérité est l'exécutable `acvc`. N'ouvrez pas `.acvc/graph.sqlite`, le journal ni le pack.
- Le format machine est JSON. Le texte humain est `--format text`.
- Le binaire installé est `acvc` sur le `PATH`. La page, la TUI, VS Code et Obsidian appellent cet exécutable. Ne réimplémentez pas le graphe.
- N'inventez pas de jetons, de métrique, de rejet ni d'identifiant. Si la commande échoue, montrez l'erreur.
