# Asistente ACVC

Esta carpeta es el contrato para un agente que va a usar ACVC, o explicar ACVC a otro agente, en una sesión de Cursor, Claude Code, OpenCode o equivalente.

Cuando explique ACVC a una persona que nunca lo vio, empiece por `objetivo.md`: los intentos fallidos quedan fuera de Git, y solo el intento aprobado se copia a un checkout que ya existe.

El comando `acvc assistant` devuelve este contrato en el idioma del computador. Quien tiene el MCP de ASB lo indexa una vez: envía el array `documents` a `rag.index`. La búsqueda siguiente usa `rag.search`.

Lea en este orden, y pare cuando la pregunta ya tenga respuesta:

1. `objetivo.md` — por qué existe ACVC.
2. `recursos.md` — qué guarda el ejecutable y qué comandos hay.
3. `como-usar.md` — el orden de una sesión.
4. `boas-praticas.md` — qué hacer siempre.
5. `exemplos.md` — usos correctos.
6. `mau-uso.md` — usos que rompen el contrato.
7. `trade-offs.md` — cuándo usarlo y cuándo no.

Reglas de esta carpeta:

- La fuente de verdad es el ejecutable `acvc`. No abra `.acvc/graph.sqlite`, el log ni el pack.
- El formato de máquina es JSON. El texto humano es `--format text`.
- El binario instalado es `acvc` en el `PATH`. La página, la TUI, VS Code y Obsidian llaman a ese ejecutable. No reimplemente el grafo.
- No invente tokens, métrica, rechazo ni identificador. Si el comando falla, muestre el error.
