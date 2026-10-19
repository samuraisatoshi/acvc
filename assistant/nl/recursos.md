# Middelen

Iemand die ACVC nog nooit zag, heeft één zin nodig vóór deze namen. ACVC bewaart de pogingen die geen commit mogen worden. Elke poging is het idee, de bestanden en de getallen. De pogingen die niemand op elk getal heeft verslagen, dingen nog mee. `promote` kopieert de goedgekeurde bestanden naar een Git-checkout die al bestaat. De namen hieronder zijn die van de commando's en de JSON.

## Wat een hypothese is

De identificatie is de BLAKE3-hash van de canonieke bytes: boom, ouders, kant, premisse, oorzaken, herziening, bedoeling en agent. De campagnenaam, de datum, de metingen en het tokenverbruik gaan niet in de hash. Dezelfde hypothese is dezelfde knoop.

Kanten: `seed`, `mutate`, `revert`, `revise_premise`, `synthesize`.

Status, als JSON-waarden:

- `on_frontier` — gemeten, en geen andere hypothese overtreft haar.
- `superseded` — gemeten, en een andere op de grens is beter.
- `rejected` — iemand legde het motief vast.
- `unmeasured` — er is nog geen volledige proef voor het schema van de campagne.

De grens verlaten legt geen afwijzing vast. Als er geen motief is, noemt de lezing wie won, de maat, en dat het motief niet is vastgelegd. In nl is die zin `Het motief is niet vastgelegd.`

De tekst die een persoon ziet volgt de taal van de computer (`LC_ALL`, dan `LC_MESSAGES`, dan `LANG`). De vier JSON-waarden hierboven veranderen niet. Een agent leest het veld `status`, niet de vertaalde zin.

## Waar de bestanden staan

De store is `.acvc/` in de map van `--repo`: een log die alleen groeit, een pack en SQLite. De gezaaide map houdt `.acvc-workspace` en `.acvc-index`.

De scan leest `.gitignore` niet. Hij slaat `.git`, `.acvc`, `target`, `node_modules`, `.build` en `DerivedData` over. Een `.acvcignore` in de wortel van de gezaaide map noemt relatieve prefixen; dat bestand gaat niet in de hash. Een symlink tijdens de scan is een fout. Een hardlink is verboden.

## Commando's

`--repo` wijst de store aan. `--format` is `json` (standaard) of `text`.

Lezen, vóór het handelen:

- `context` — kaart van de campagne: probleem, oorzaken, agents, interacties, tokens, leeftijd en vertakkingen.
- `guide` — de commando's plus de kaart van de open repository.
- `frontier` — wat nog meedingt.
- `why` — één hypothese: oorzaak, meting, wie haar overtrof, ouders en kinderen.
- `recall` — zoeken op de premisse. Geeft status en ouder terug. Een accent hoort bij de zoekopdracht.
- `compare` — twee gemeten hypothesen: welke blijft en welke bestanden veranderden.
- `file-diff` — de tekst van één pad tussen twee hypothesen.
- `status` — de werkmap tegen de gematerialiseerde hypothese.
- `backtrack` — terug naar een ouder.

Schrijven:

- `init` — maakt de store.
- `campaign create NAAM --metric naam:higher|lower` — het schema. Er kan meer dan één maat zijn.
- `seed` — de eerste hypothese van de campagne. Een tweede oorsprong wordt geweigerd. Het volgende pad is `record`.
- `record` — de map veranderde. De nieuwe hypothese wijst naar de ouder. Standaardkant: `mutate`.
- `score --metric naam=getal` — voegt een proef toe. Hij wist de eerdere niet.
- `reject --reason "..."` — een expliciete daad. Het motief blijft in de graaf.
- `spend --tokens N` — een apart boek. Het gaat niet in de hash, het is geen maat en het vergrendelt `record` niet. Nul tokens wordt geweigerd.
- `materialize` — kopieert de hypothese naar een map.
- `promote` — legt de winnende variant in een checkout die al bestaat, of in een map. Hij doet geen merge.
- `pack` — verdicht koude blobs. Hij verandert de lezing van status en record niet.

Viewers, allemaal roepen ze het uitvoerbare bestand aan:

- `web` — pagina op `127.0.0.1:9420`.
- `tui` — terminal.
- `mcp` — stdio-server. Hij opent SQLite niet. Elk gereedschap voert het binaire bestand opnieuw uit.
- `obsidian` — installeert de viewer in de vault. Het commando in Obsidian is Open ACVC.

Remote: `serve`, `push`, `fetch`. De remote doet geen checkout en geen merge. Hij stuurt een object niet opnieuw als de andere kant het al heeft.

## Grens

Elke volledige meting die niemand overtreft komt erin. Twee onvergelijkbare metingen van dezelfde hypothese blijven allebei. Een strikt slechtere meting verlaat de grens en blijft in de geschiedenis. Meng geen maat van de ene proef met een maat van een andere. Een afgewezen hypothese blijft erbuiten. Twee onvergelijkbare hypothesen blijven allebei. Voeg de campagne niet samen om een winnaar te forceren.
