# Asystent ACVC

Ten folder jest umową dla agenta, który użyje ACVC albo wyjaśni ACVC innemu agentowi w sesji Cursor, Claude Code, OpenCode lub równoważnej.

Polecenie `acvc assistant` zwraca tę umowę w języku komputera. Agent z MCP ASB indeksuje ją raz: wysyła tablicę `documents` do `rag.index`. Późniejsze szukanie używa `rag.search`.

Czytaj w tej kolejności i zatrzymaj się, gdy pytanie ma już odpowiedź:

1. `objetivo.md` — dlaczego ACVC istnieje.
2. `recursos.md` — co przechowuje program i jakie są polecenia.
3. `como-usar.md` — kolejność sesji.
4. `boas-praticas.md` — co robić zawsze.
5. `exemplos.md` — poprawne użycia.
6. `mau-uso.md` — użycia, które łamią umowę.
7. `trade-offs.md` — kiedy używać i kiedy nie.

Zasady tego folderu:

- Źródłem prawdy jest program `acvc`. Nie otwieraj `.acvc/graph.sqlite`, dziennika ani packa.
- Format maszynowy to JSON. Tekst dla człowieka to `--format text`.
- Zainstalowany plik binarny to `acvc` w `PATH`. Strona, TUI, VS Code i Obsidian wołają ten program. Nie implementuj grafu od nowa.
- Nie wymyślaj tokenów, miary, odrzucenia ani identyfikatora. Gdy polecenie się nie uda, pokaż błąd.
