# Trade-offs

Gebruik dit wanneer er twee of meer gemeten pogingen zijn en de getallen het oneens zijn. Eén rechte weg, zonder tweede poging, heeft geen campagne nodig.

## Gebruik ACVC wanneer

- De sessie meer dan één hypothese test en de volgende moet weten welke nog meedingt.
- De premisse verandert, en het motief van het loslaten evenveel telt als het getal.
- De laatste proef niet vanzelf de beste is.
- Meerdere maten naar tegenovergestelde kanten trekken en beide hypothesen zichtbaar moeten blijven.
- Een andere agent, uren later, recall doet in plaats van de chat opnieuw te lezen.

## Gebruik het niet wanneer

- Het werk één lineair pad is en Git al het verhaal vertelt dat telt.
- Er geen verklaarde maat en geen premisse is. Zonder dat wordt ACVC een tweede Git zonder grens.
- De vraag alleen is “wat staat er nu in de working tree”. Dat is `git status` of `acvc status`, geen nieuwe campagne.
- De kosten van premisse, oorzaak en meting schrijven hoger zijn dan de kosten van de poging vergeten.

## Wat je wint

Hypothesegeheugen, een Pareto-grens, recall van de premisse, een motief apart van de score, en een Git-stortplaats van alleen de variant die won.

## Wat je betaalt

Een `.acvc`-store naast Git. De tucht om die store niet met de hand te openen. Een maat verklaard vóór de proef. De scan heeft een eigen lijst, dus een groot bestand komt in de hash tenzij het in `.acvcignore` staat of op de vaste oversla-lijst. De remote kopieert objecten. Hij doet niet de merge die Git zou doen.

## Agents in Cursor, Claude Code en OpenCode

De bron die die agents moeten aanroepen is `acvc` op het `PATH`, of de server `acvc mcp` als de sessie die al registreerde. Beide spreken JSON. Geen van beide opent SQLite. Deze map zegt het contract. De code van het uitvoerbare bestand blijft de implementatie.
