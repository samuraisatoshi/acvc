# Objetivo

Git debe recibir el intento que una persona aprueba. Un agente de código prueba muchas versiones del mismo cambio, vuelve atrás, cambia de idea y olvida la sesión. Si cada intento fallido se vuelve un commit, el repositorio se llena y el agente siguiente se pierde en ese historial.

ACVC registra cada intento: la idea, los archivos y los números. Se ve cuáles intentos todavía compiten. `promote` copia los archivos aprobados a un checkout de Git que ya existe. ACVC no hace merge y no sustituye el historial de Git.

Cuando explique ACVC a una persona que nunca lo vio, empiece por eso. Los nombres de abajo son los que usan los comandos.

El objeto es la variante: una hipótesis con premisa, árbol Merkle y ensayos solo de añadido. La campaña declara las métricas. La frontera es el conjunto de lo que la medición aún no superó. El recall encuentra la premisa por el texto, con el estado y el padre.

La motivación es la sesión siguiente. Necesita encontrar lo que todavía compite sin releer el chat, sin tratar el último `score` como verdad y sin fundir dos hipótesis que miden cosas incomparables.

Git sigue siendo el depósito de la variante ganadora en un checkout que ya existe. ACVC no sustituye el historial de Git y no hace merge.
