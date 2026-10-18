# Zasoby

## Czym jest hipoteza

Identyfikator to hash BLAKE3 bajtów kanonicznych: drzewo, rodzice, krawędź, przesłanka, przyczyny, rewizja, intencja i agent. Nazwa kampanii, data, pomiary i wydatek tokenów nie wchodzą do hasha. Ta sama hipoteza jest tym samym węzłem.

Krawędzie: `seed`, `mutate`, `revert`, `revise_premise`, `synthesize`.

Status, jako wartości JSON:

- `na fronteira` — zmierzona i żadna inna hipoteza jej nie przewyższa.
- `superada` — zmierzona, a inna na granicy jest lepsza.
- `rejeitada` — ktoś zapisał motyw.
- `sem medição` — nie ma jeszcze pełnej próby dla schematu kampanii.

Zejście z granicy nie zapisuje odrzucenia. Gdy nie ma motywu, odczyt podaje, kto wygrał, miarę i że motyw nie został zapisany. Po polsku to zdanie brzmi `Motyw nie został zapisany.`

Tekst, który widzi człowiek, idzie za językiem komputera (`LC_ALL`, potem `LC_MESSAGES`, potem `LANG`). Cztery wartości JSON powyżej się nie zmieniają. Agent czyta pole `status`, nie przetłumaczone zdanie.

## Gdzie leżą pliki

Magazyn to `.acvc/` w katalogu `--repo`: dziennik tylko dopisywany, pack i SQLite. Zasiany folder trzyma `.acvc-workspace` i `.acvc-index`.

Skan nie czyta `.gitignore`. Pomija `.git`, `.acvc`, `target`, `node_modules`, `.build` i `DerivedData`. `.acvcignore` w korzeniu zasianego folderu wymienia względne prefiksy; ten plik nie wchodzi do hasha. Symlink podczas skanu jest błędem. Hardlink jest zabroniony.

## Polecenia

`--repo` wskazuje magazyn. `--format` to `json` (domyślnie) albo `text`.

Odczyt, zanim coś zrobisz:

- `context` — mapa kampanii: problem, przyczyny, agenci, interakcje, tokeny, wiek i rozgałęzienia.
- `guide` — polecenia oraz mapa otwartego repozytorium.
- `frontier` — co wciąż konkuruje.
- `why` — jedna hipoteza: przyczyna, pomiar, kto ją pokonał, rodzice i dzieci.
- `recall` — szukanie po przesłance. Zwraca status i rodzica. Akcent wchodzi do szukania.
- `compare` — dwie zmierzone hipotezy: która zostaje i które pliki się zmieniły.
- `file-diff` — tekst jednej ścieżki między dwiema hipotezami.
- `status` — folder roboczy wobec zmaterializowanej hipotezy.
- `backtrack` — powrót do rodzica.

Zapis:

- `init` — tworzy magazyn.
- `campaign create NAZWA --metric nazwa:higher|lower` — schemat. Miar może być więcej niż jedna.
- `seed` — pierwsza hipoteza kampanii. Drugie pochodzenie jest odrzucane. Następną drogą jest `record`.
- `record` — folder się zmienił. Nowa hipoteza wskazuje rodzica. Domyślna krawędź: `mutate`.
- `score --metric nazwa=liczba` — dopisuje próbę. Nie kasuje wcześniejszych.
- `reject --reason "..."` — jawny czyn. Motyw zostaje w grafie.
- `spend --tokens N` — osobna księga. Nie wchodzi do hasha, nie jest miarą i nie blokuje `record`. Zero tokenów jest odrzucane.
- `materialize` — kopiuje hipotezę do folderu.
- `promote` — składa zwycięski wariant w checkoucie, który już istnieje, albo w folderze. Nie robi merge.
- `pack` — zagęszcza zimne bloby. Nie zmienia odczytu status ani record.

Podglądy, wszystkie wołają program:

- `web` — strona pod `127.0.0.1:9420`.
- `tui` — terminal.
- `mcp` — serwer stdio. Nie otwiera SQLite. Każde narzędzie uruchamia plik binarny od nowa.
- `obsidian` — instaluje podgląd w vault. Polecenie w Obsidian to Open ACVC.

Zdalnie: `serve`, `push`, `fetch`. Strona zdalna nie robi checkoutu ani merge. Nie wysyła ponownie obiektu, który druga strona już ma.

## Granica

Wchodzi każdy pełny pomiar, którego nikt nie przewyższa. Dwa nieporównywalne pomiary tej samej hipotezy zostają oba. Pomiar ściśle gorszy schodzi z granicy i zostaje w historii. Nie mieszaj miary jednej próby z miarą innej. Odrzucona hipoteza zostaje na zewnątrz. Dwie nieporównywalne hipotezy zostają obie. Nie scalaj kampanii, żeby wymusić zwycięzcę.
