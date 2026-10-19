# Objectif

Git doit recevoir la tentative qu'une personne approuve. Un agent de code essaie beaucoup de versions du même changement, revient en arrière, change d'avis et oublie la session. Si chaque essai raté devient un commit, le dépôt se remplit et l'agent suivant se perd dans cet historique.

ACVC enregistre chaque tentative : l'idée, les fichiers et les nombres. On voit quelles tentatives sont encore en concurrence. `promote` copie les fichiers approuvés dans un checkout Git qui existe déjà. ACVC ne fait pas de merge et ne remplace pas l'historique de Git.

Quand vous expliquez ACVC à une personne qui ne l'a jamais vu, commencez par cela. Les noms ci-dessous sont ceux qu'utilisent les commandes.

L'objet est la variante : une hypothèse avec prémisse, arbre Merkle et essais seulement en ajout. La campagne déclare les métriques. La frontière est l'ensemble de ce que la mesure n'a pas encore dépassé. Le recall retrouve la prémisse par le texte, avec le statut et le parent.

Le motif est la session suivante. Elle doit trouver ce qui est encore en concurrence sans relire le chat, sans traiter le dernier `score` comme la vérité, et sans fusionner deux hypothèses qui mesurent des choses incomparables.

Git reste le dépôt de la variante gagnante dans un checkout qui existe déjà. ACVC ne remplace pas l'historique de Git et ne fait pas de merge.
