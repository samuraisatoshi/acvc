# Ressources

## Ce qu'est une hypothèse

L'identifiant est le hash BLAKE3 des octets canoniques : arbre, parents, arête, prémisse, causes, révision, intention et agent. Le nom de la campagne, la date, les mesures et la dépense de jetons n'entrent pas dans le hash. La même hypothèse est le même nœud.

Arêtes : `seed`, `mutate`, `revert`, `revise_premise`, `synthesize`.

Statut, dans les valeurs JSON :

- `na fronteira` — mesurée, et aucune autre hypothèse ne la dépasse.
- `superada` — mesurée, et une autre de la frontière est meilleure.
- `rejeitada` — quelqu'un a enregistré le motif.
- `sem medição` — il n'y a pas encore d'essai complet pour le schéma de la campagne.

Quitter la frontière n'enregistre pas de rejet. S'il n'y a pas de motif, la lecture nomme qui a gagné, la métrique, et que le motif n'a pas été enregistré. En fr cette phrase est `Le motif n'a pas été enregistré.`

Le texte qu'une personne voit suit la langue de l'ordinateur (`LC_ALL`, puis `LC_MESSAGES`, puis `LANG`). Les quatre valeurs JSON ci-dessus ne changent pas. L'agent lit le champ `status`, pas la phrase traduite.

## Où sont les fichiers

Le store est `.acvc/` dans le répertoire de `--repo` : journal en ajout seulement, pack et SQLite. Le dossier semé garde `.acvc-workspace` et `.acvc-index`.

Le parcours ne lit pas `.gitignore`. Il saute `.git`, `.acvc`, `target`, `node_modules`, `.build` et `DerivedData`. Un `.acvcignore` à la racine du dossier semé liste des préfixes relatifs ; ce fichier n'entre pas dans le hash. Un symlink pendant le parcours est une erreur. Un hardlink est interdit.

## Commandes

`--repo` pointe le store. `--format` est `json` (par défaut) ou `text`.

Lecture, avant d'agir :

- `context` — carte de la campagne : problème, causes, agents, interactions, jetons, âge et ramifications.
- `guide` — les commandes plus la carte du dépôt ouvert.
- `frontier` — ce qui est encore en concurrence.
- `why` — une hypothèse : cause, mesure, qui l'a dépassée, parents et enfants.
- `recall` — recherche par la prémisse. Renvoie le statut et le parent. L'accent entre dans la recherche.
- `compare` — deux hypothèses mesurées : laquelle reste et quels fichiers ont changé.
- `file-diff` — le texte d'un chemin entre deux hypothèses.
- `status` — le dossier de travail contre l'hypothèse matérialisée.
- `backtrack` — retour à un parent.

Écriture :

- `init` — crée le store.
- `campaign create NOM --metric nom:higher|lower` — le schéma. Il peut y avoir plus d'une métrique.
- `seed` — la première hypothèse de la campagne. Une seconde origine est refusée. Le chemin suivant est `record`.
- `record` — le dossier a changé. La nouvelle hypothèse pointe vers le parent. Arête par défaut : `mutate`.
- `score --metric nom=nombre` — ajoute un essai. Il n'efface pas les précédents.
- `reject --reason "..."` — acte explicite. Le motif reste dans le graphe.
- `spend --tokens N` — livre à part. Il n'entre pas dans le hash, ce n'est pas une métrique et il ne verrouille pas `record`. Zéro jeton est refusé.
- `materialize` — copie l'hypothèse dans un dossier.
- `promote` — dépose la variante gagnante dans un checkout qui existe déjà, ou dans un dossier. Il ne fait pas de merge.
- `pack` — compacte les blobs froids. Il ne change pas la lecture de status ni de record.

Visualiseurs, tous appellent l'exécutable :

- `web` — page sur `127.0.0.1:9420`.
- `tui` — terminal.
- `mcp` — serveur stdio. Il n'ouvre pas SQLite. Chaque outil réexécute le binaire.
- `obsidian` — installe le visualiseur dans le vault. La commande dans Obsidian est Open ACVC.

Distant : `serve`, `push`, `fetch`. Le distant ne fait ni checkout ni merge. Il ne renvoie pas un objet que l'autre côté a déjà.

## Frontière

Entrent toutes les mesures complètes que personne ne dépasse. Deux mesures incomparables de la même hypothèse restent toutes les deux. Une mesure strictement pire quitte la frontière et reste dans l'historique. Ne mélangez pas une métrique d'un essai avec celle d'un autre. Une hypothèse rejetée reste dehors. Deux hypothèses incomparables restent toutes les deux. Ne fusionnez pas la campagne pour forcer un vainqueur.
