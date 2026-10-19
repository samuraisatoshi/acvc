# Exemples

Chaque bloc ci-dessous est une question avec plus d'une tentative. Les nombres sont des mesures. Git ne reçoit que la tentative qu'une personne approuve.

## Ouvrir une campagne

```text
acvc init --repo .
acvc campaign create recherche --metric tests_passed:higher --metric latency_ms:lower
acvc seed --repo . --campaign recherche --path ./hypothese --premise "le cache tient sur le disque" --assumption "le disque est local" --intent "moins de lectures" --agent researcher
```

Le JSON du seed apporte la variante, l'arbre, combien de fichiers sont entrés et lesquels pesaient le plus.

## L'hypothèse suivante

```text
acvc record --repo . --workspace ./hypothese --parent PREFIXE --premise "l'index est un arbre" --assumption "les clés sont triées" --intent "une recherche plus courte" --agent researcher --edge mutate
acvc score --repo . --variant PREFIXE --metric tests_passed=16 --metric latency_ms=4
```

## Voir ce qui est encore en concurrence

```text
acvc context --repo . --campaign recherche
acvc frontier --repo . --campaign recherche
acvc why --repo . PREFIXE
acvc recall --repo . --campaign recherche --query "arbre"
acvc compare --repo . PREFIXE_A PREFIXE_B
```

Un `why` d'une hypothèse dépassée, sans `reject`, nomme qui a gagné, la métrique, et dit que le motif n'a pas été enregistré.

## Deux mesures de la même hypothèse

```text
acvc score --repo . --variant PREFIXE --metric tests_passed=10 --metric latency_ms=10
acvc score --repo . --variant PREFIXE --metric tests_passed=1 --metric latency_ms=1
acvc score --repo . --variant PREFIXE --metric tests_passed=0 --metric latency_ms=20
```

Les deux premières sont incomparables et restent sur la frontière. La troisième est pire sur les deux métriques, quitte la frontière et reste dans le `why`.

## Clore exprès

```text
acvc reject --repo . --variant PREFIXE --reason "l'invalidation du cache casse sous des écritures concurrentes"
acvc spend --repo . --campaign recherche --tokens 15000 --note "session de l'hypothèse" --agent researcher
```

## Retirer le bruit de l'arbre

Dans le dossier semé, `.acvcignore` :

```text
# rapport
lab/reports
```

Deux clones avec le même fichier laissent les mêmes chemins dehors. Effacer le fichier laisse `lab/reports` revenir au parcours suivant. Les hypothèses déjà enregistrées ne sont pas réécrites.
