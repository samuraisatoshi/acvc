# Recursos

Una persona que nunca vio ACVC necesita una frase antes de estos nombres. ACVC guarda los intentos que no deben volverse commits. Cada intento es la idea, los archivos y los números. Los intentos que nadie superó en todos los números todavía compiten. `promote --dest` materializa los archivos aprobados en una carpeta; `--git` también crea un commit en un worktree de Git existente. Los nombres de abajo son los que usan los comandos y el JSON.

## Qué es una hipótesis

El identificador es el hash BLAKE3 de los bytes canónicos: árbol, padres, arista, premisa, causas, revisión, intención y agente. El nombre de la campaña, la fecha, las mediciones y el gasto de tokens no entran en el hash. La misma hipótesis es el mismo nodo.

Aristas:

- `seed` — el único origen.
- `mutate` — los archivos cambiaron y la creencia sigue. Es la arista predeterminada de `record`.
- `revert` — el paso vuelve a una hipótesis anterior.
- `revise_premise` — cambió la propia creencia.
- `synthesize` — dos o más padres.

Estado, en los valores JSON:

- `on_frontier` — medida, y ninguna otra hipótesis la supera.
- `superseded` — medida, y otra de la frontera es mejor.
- `rejected` — alguien grabó el motivo.
- `unmeasured` — aún no hay ensayo completo para el esquema de la campaña.

Salir de la frontera no graba rechazo. Si no hay motivo, la lectura dice quién ganó, la métrica y que el motivo no quedó registrado. En es esa frase es `El motivo no quedó registrado.`

El texto que ve la persona sigue el idioma del computador (`LC_ALL`, después `LC_MESSAGES`, después `LANG`). Los cuatro valores JSON de arriba no cambian. El agente lee el campo `status`, no la frase traducida.

## Dónde quedan los archivos

El store es `.acvc/` en el directorio de `--repo`: log solo de añadido, pack y SQLite. En la carpeta sembrada quedan `.acvc-workspace` y `.acvc-index`.

El recorrido no lee `.gitignore`. Salta `.git`, `.acvc`, `target`, `node_modules`, `.build` y `DerivedData`. Un `.acvcignore` en la raíz de la carpeta sembrada lista prefijos relativos; ese archivo no entra en el hash. Un symlink en el recorrido es un error. El hardlink está prohibido.

## Comandos

`--repo` apunta el store. `--format` es `json` (predeterminado) o `text`.

Lectura, antes de actuar:

- `context` — mapa de la campaña: problema, causas, agentes, interacciones, tokens, edad y ramificaciones.
- `guide` — comandos más el mapa del repositorio abierto.
- `frontier` — lo que todavía compite.
- `why` — una hipótesis: causa, medición, quién la superó, padres e hijos.
- `recall` — búsqueda por la premisa. Devuelve estado y padre. El acento entra en la búsqueda.
- `compare` — dos hipótesis medidas: quién se queda y qué archivos cambiaron.
- `file-diff` — el texto de un camino entre dos hipótesis.
- `status` — la carpeta de trabajo contra la hipótesis materializada.
- `backtrack` — vuelve a un padre.

Escritura:

- `init` — crea el store.
- `campaign create NOMBRE --metric nombre:higher|lower` — el esquema. Puede haber más de una métrica.
- `seed` — la primera hipótesis de la campaña. El segundo origen se rechaza. El camino siguiente es `record`.
- `record` — la carpeta cambió. Sin `--parent`, el padre es la hipótesis del workspace, y el workspace pasa a apuntar a la hipótesis nueva. Arista predeterminada: `mutate`.
- `score --metric nombre=número` — añade un ensayo en la misma variante. El identificador no cambia. No borra los anteriores.
- `reject --reason "..."` — acto explícito. El motivo queda en el grafo.
- `spend --tokens N` — libro aparte. No entra en el hash, no es métrica y no traba `record`. Cero tokens se rechaza.
- `materialize` — copia la hipótesis a una carpeta.
- `promote` — `--dest` materializa la variante ganadora en una carpeta; `--git` requiere un worktree de Git y crea un commit. Ningún modo hace merge ni elimina del destino rutas ausentes de la variante.
- `pack` — compacta blobs fríos. No cambia la lectura de status ni de record.

Visores, todos llaman al ejecutable:

- `web` — página en `127.0.0.1:9420`.
- `tui` — terminal.
- `mcp` — servidor stdio. No abre el SQLite. Cada herramienta reejecuta el binario.
- `obsidian` — instala el visor en el vault. El comando en Obsidian es Open ACVC.

Remoto: `serve`, `push`, `fetch`. El remoto no hace checkout ni merge. No reenvía un objeto que el otro lado ya tiene.

## Frontera

Entran todas las mediciones completas que nadie supera. Dos mediciones incomparables de la misma hipótesis se quedan las dos. Una medición estrictamente peor sale de la frontera y permanece en el historial. No se mezcla la métrica de un ensayo con la de otro. La hipótesis rechazada queda fuera. Dos hipótesis incomparables siguen las dos. No se funde la campaña para forzar un ganador.
