# Trade-offs

## Use ACVC cuando

- La sesión prueba más de una hipótesis y la siguiente necesita saber cuál todavía compite.
- La premisa cambia, y el motivo del abandono importa tanto como el número.
- El último ensayo no es, por sí, el mejor.
- Varias métricas tiran hacia lados opuestos y las dos hipótesis deben seguir visibles.
- Otro agente, horas después, va a hacer recall en vez de releer el chat.

## No lo use cuando

- El trabajo es un único camino lineal y Git ya cuenta la historia que importa.
- No hay métrica declarada ni premisa. Sin eso ACVC se vuelve un segundo Git sin frontera.
- La pregunta es solo “qué hay en el working tree ahora”. Eso es `git status` o `acvc status`, no una campaña nueva.
- El costo de escribir premisa, causa y medición es mayor que el costo de olvidar el intento.

## Lo que se gana

Memoria de hipótesis, frontera de Pareto, recall de la premisa, motivo separado del marcador, y un depósito Git solo de la variante que ganó.

## Lo que se paga

Un store `.acvc` al lado de Git. Disciplina de no abrir ese store a mano. Métrica declarada antes del ensayo. El recorrido tiene lista propia, así que un archivo grande entra en el hash si no está en `.acvcignore` ni en la lista fija. El remoto copia objetos, no hace el merge que haría Git.

## Agentes en Cursor, Claude Code y OpenCode

El recurso que esos agentes deben llamar es `acvc` en el `PATH`, o el servidor `acvc mcp` si la sesión ya lo registró. Los dos hablan JSON. Ninguno abre el SQLite. Esta carpeta dice el contrato. El código del ejecutable sigue siendo la implementación.
