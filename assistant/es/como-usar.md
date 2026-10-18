# Cómo usar

1. `acvc context --campaign NOMBRE` antes de proponer, medir o descartar.
2. Si la campaña no existe: `acvc init --repo .` y `acvc campaign create NOMBRE --metric metrica:higher`.
3. La primera hipótesis es `seed` de la carpeta que contiene los archivos de la hipótesis, con `--premise`, `--assumption`, `--intent` y `--agent`.
4. Cambió un archivo: `record` a partir del padre. No haga otro `seed`.
5. Hay un número real de la métrica declarada: `score`. Un ensayo incompleto no entra en la frontera.
6. `frontier` muestra lo que todavía compite. `why` explica una hipótesis. `recall --query` la encuentra por el texto de la premisa.
7. Dos medidas concretas: `compare`.
8. La hipótesis murió por un motivo conocido: `reject --reason`. Sin motivo, no rechace. La lectura ya dice que el motivo no quedó registrado.
9. La sesión gastó tokens y ese número se conoce: `spend`. No estime.
10. La variante que ganó entra en Git con `promote` en un checkout que ya existe.

Prefijo de identificador: de 8 a 64 caracteres hexadecimales, sin ambigüedad.

Para otro agente en la misma máquina, el canal es el ejecutable o `acvc mcp`. La página y la TUI son para la persona. No las sustituya por una lectura directa de la base.
