# Jak używać

1. `acvc context --campaign NAZWA` zanim zaproponujesz, zmierzysz albo odrzucisz.
2. Jeśli kampanii nie ma: `acvc init --repo .` i `acvc campaign create NAZWA --metric miara:higher`.
3. Pierwsza hipoteza to `seed` folderu z plikami hipotezy, z `--premise`, `--assumption`, `--intent` i `--agent`.
4. Plik się zmienił: `record` od rodzica. Nie rób drugiego `seed`.
5. Jest prawdziwa liczba zadeklarowanej miary: `score`. Niepełna próba nie wchodzi na granicę.
6. `frontier` pokazuje, co wciąż konkuruje. `why` wyjaśnia jedną hipotezę. `recall --query` znajduje ją po tekście przesłanki.
7. Dwa konkretne pomiary: `compare`.
8. Hipoteza umarła ze znanego motywu: `reject --reason`. Bez motywu nie odrzucaj. Odczyt już mówi, że motyw nie został zapisany.
9. Sesja zużyła tokeny i ta liczba jest znana: `spend`. Nie szacuj.
10. Wariant, który wygrał, wchodzi do Gita przez `promote` w checkoucie, który już istnieje.

Przedrostek identyfikatora: od 8 do 64 znaków szesnastkowych, bez niejednoznaczności.

Dla innego agenta na tej samej maszynie kanałem jest program albo `acvc mcp`. Strona i TUI są dla człowieka. Nie zastępuj ich bezpośrednim odczytem bazy.
