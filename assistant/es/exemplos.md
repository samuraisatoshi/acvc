# Ejemplos

## Abrir una campaña

```text
acvc init --repo .
acvc campaign create busca --metric tests_passed:higher --metric latency_ms:lower
acvc seed --repo . --campaign busca --path ./hipotesis --premise "la caché cabe en disco" --assumption "el disco es local" --intent "menos lecturas" --agent researcher
```

El JSON del seed trae la variante, el árbol, cuántos archivos entraron y cuáles pesaron más.

## La hipótesis siguiente

```text
acvc record --repo . --workspace ./hipotesis --parent PREFIJO --premise "el índice es un árbol" --assumption "las claves están ordenadas" --intent "búsqueda más corta" --agent researcher --edge mutate
acvc score --repo . --variant PREFIJO --metric tests_passed=16 --metric latency_ms=4
```

## Ver lo que todavía compite

```text
acvc context --repo . --campaign busca
acvc frontier --repo . --campaign busca
acvc why --repo . PREFIJO
acvc recall --repo . --campaign busca --query "árbol"
acvc compare --repo . PREFIJO_A PREFIJO_B
```

Un `why` de una hipótesis superada, sin `reject`, nombra quién ganó, la métrica y dice que el motivo no quedó registrado.

## Dos mediciones de la misma hipótesis

```text
acvc score --repo . --variant PREFIJO --metric tests_passed=10 --metric latency_ms=10
acvc score --repo . --variant PREFIJO --metric tests_passed=1 --metric latency_ms=1
acvc score --repo . --variant PREFIJO --metric tests_passed=0 --metric latency_ms=20
```

Las dos primeras son incomparables y se quedan en la frontera. La tercera es peor en las dos métricas, sale de la frontera y continúa en el `why`.

## Cerrar a propósito

```text
acvc reject --repo . --variant PREFIJO --reason "la invalidación de la caché se rompe con escritura concurrente"
acvc spend --repo . --campaign busca --tokens 15000 --note "sesión de la hipótesis" --agent researcher
```

## Sacar basura del árbol

En la carpeta sembrada, `.acvcignore`:

```text
# informe
lab/reports
```

Dos clones con el mismo archivo dejan fuera los mismos caminos. Borrar el archivo hace que `lab/reports` vuelva en el recorrido siguiente. Las hipótesis ya grabadas no se reescriben.
