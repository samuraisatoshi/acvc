# ACVC-assistent

Deze map is het contract voor een agent die ACVC gaat gebruiken, of ACVC aan een andere agent uitlegt, in een sessie van Cursor, Claude Code, OpenCode of iets gelijkwaardigs.

Het commando `acvc assistant` geeft dit contract terug in de taal van de computer. Een agent met de ASB-MCP indexeert het één keer: stuur de array `documents` naar `rag.index`. Zoeken daarna gaat met `rag.search`.

Lees in deze volgorde, en stop wanneer de vraag al een antwoord heeft:

1. `objetivo.md` — waarom ACVC bestaat.
2. `recursos.md` — wat het uitvoerbare bestand bewaart en welke commando's er zijn.
3. `como-usar.md` — de volgorde van een sessie.
4. `boas-praticas.md` — wat altijd moet.
5. `exemplos.md` — juist gebruik.
6. `mau-uso.md` — gebruik dat het contract breekt.
7. `trade-offs.md` — wanneer wel en wanneer niet.

Regels van deze map:

- De bron van waarheid is het uitvoerbare bestand `acvc`. Open `.acvc/graph.sqlite`, de log en de pack niet.
- Het machineformaat is JSON. Mensentaal is `--format text`.
- Het geïnstalleerde binaire bestand is `acvc` op het `PATH`. De pagina, de TUI, VS Code en Obsidian roepen dat bestand aan. Implementeer de graaf niet opnieuw.
- Verzin geen tokens, metriek, afwijzing of identificatie. Als het commando faalt, toon de fout.
