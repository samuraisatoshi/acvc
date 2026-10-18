# Objectif

Git garde ce qu'un humain a choisi d'enregistrer. Un agent de code teste des hypothèses en volume, revient en arrière, réécrit la prémisse et oublie la session. Le commit suivant ne dit pas quelle mesure est encore en concurrence, ni pourquoi l'autre a été abandonnée.

ACVC existe pour cette mémoire.

L'objet est la variante : une hypothèse avec prémisse, arbre Merkle et essais seulement en ajout. La campagne déclare les métriques. La frontière est l'ensemble de ce que la mesure n'a pas encore dépassé. Le recall retrouve la prémisse par le texte, avec le statut et le parent.

Le motif est la session suivante. Elle doit trouver ce qui est encore en concurrence sans relire le chat, sans traiter le dernier `score` comme la vérité, et sans fusionner deux hypothèses qui mesurent des choses incomparables.

Git reste le dépôt de la variante gagnante dans un checkout qui existe déjà. ACVC ne remplace pas l'historique de Git et ne fait pas de merge.
