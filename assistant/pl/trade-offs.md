# Trade-offs

Użyj tego, gdy są dwie lub więcej zmierzonych wersji i liczby się nie zgadzają. Jedna prosta droga, bez drugiej wersji, nie potrzebuje kampanii.

## Użyj ACVC, gdy

- Sesja testuje więcej niż jedną hipotezę, a następna musi wiedzieć, która wciąż konkuruje.
- Przesłanka się zmienia, a motyw porzucenia liczy się tak samo jak liczba.
- Ostatnia próba sama z siebie nie jest najlepsza.
- Kilka miar ciągnie w przeciwne strony i obie hipotezy mają zostać widoczne.
- Inny agent, po godzinach, zrobi recall zamiast czytać czat od nowa.

## Nie używaj, gdy

- Praca jest jedną liniową ścieżką i Git już opowiada historię, która się liczy.
- Nie ma zadeklarowanej miary ani przesłanki. Bez tego ACVC staje się drugim Gitem bez granicy.
- Pytanie brzmi tylko „co jest teraz w working tree”. To `git status` albo `acvc status`, nie nowa kampania.
- Koszt zapisania przesłanki, przyczyny i pomiaru jest wyższy niż koszt zapomnienia próby.

## Co zyskujesz

Pamięć hipotezy, granicę Pareto, recall przesłanki, motyw oddzielony od wyniku i skład Gita tylko zwycięskiego wariantu.

## Co płacisz

Magazyn `.acvc` obok Gita. Dyscyplinę, żeby nie otwierać go ręcznie. Miarę zadeklarowaną przed próbą. Skan ma własną listę, więc duży plik wchodzi do hasha, jeśli nie ma go w `.acvcignore` ani na stałej liście pomijania. Zdalna strona kopiuje obiekty. Nie robi merge, który zrobiłby Git.

## Agenci w Cursor, Claude Code i OpenCode

Zasób, który mają wołać, to `acvc` w `PATH` albo serwer `acvc mcp`, jeśli sesja już go zarejestrowała. Oba mówią JSON. Żaden nie otwiera SQLite. Ten folder mówi umowę. Kod programu pozostaje implementacją.
