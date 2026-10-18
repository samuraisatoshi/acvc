# Mal uso

- Abrir `.acvc/graph.sqlite`, `objects.log` u `objects.pack` para leer estado, frontera o premisa. El contrato es el ejecutable.
- Tratar Git como el grafo de hipótesis. El commit no guarda el ensayo perdedor ni el motivo que no se escribió.
- Un segundo `seed` en la misma campaña. La campaña ya tiene origen. El camino es `record` a partir del padre.
- Rechazar porque otra hipótesis dominó. Eso graba un motivo que nadie dio.
- Fundir dos hipótesis incomparables, o religar seeds sueltos, para que la frontera tenga un único ganador.
- Usar `spend` como presupuesto, métrica de la frontera o traba de `record`. Inventar un número de tokens.
- Creer que el último `score` borra el anterior, o mezclar `tests_passed` de un ensayo con `latency_ms` de otro.
- Leer `.gitignore` como política del recorrido. La política explícita es `.acvcignore`.
- Hardlink entre árboles. Symlink dentro de la carpeta sembrada.
- `promote` como merge, o `push`/`fetch` esperando checkout del otro lado.
- Mostrar hash, arista o “left/right” a la persona. La frase es la premisa y quién se queda.
- Pegar todas las causas en una sola necesidad de la campaña cuando hay más de una causa en el grafo.
- Reescribir una hipótesis ya grabada para “corregir” la premisa. Una premisa nueva es `revise_premise` o `record`, otro nodo.
- Levantar `acvc web` o `acvc tui` desde la sesión del agente cuando la persona ya tiene el visor abierto. Dígale el comando para su terminal.
