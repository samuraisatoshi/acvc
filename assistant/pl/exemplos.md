# Przykłady

## Otwarcie kampanii

```text
acvc init --repo .
acvc campaign create szukanie --metric tests_passed:higher --metric latency_ms:lower
acvc seed --repo . --campaign szukanie --path ./hipoteza --premise "cache mieści się na dysku" --assumption "dysk jest lokalny" --intent "mniej odczytów" --agent julio
```

JSON z seed podaje wariant, drzewo, ile plików weszło i które ważyły więcej.

## Następna hipoteza

```text
acvc record --repo . --workspace ./hipoteza --parent PREFIKS --premise "indeks jest drzewem" --assumption "klucze są posortowane" --intent "krótsze szukanie" --agent julio --edge mutate
acvc score --repo . --variant PREFIKS --metric tests_passed=16 --metric latency_ms=4
```

## Zobaczyć, co wciąż konkuruje

```text
acvc context --repo . --campaign szukanie
acvc frontier --repo . --campaign szukanie
acvc why --repo . PREFIKS
acvc recall --repo . --campaign szukanie --query "drzewo"
acvc compare --repo . PREFIKS_A PREFIKS_B
```

`why` pokonanej hipotezy, bez `reject`, podaje, kto wygrał, miarę i mówi, że motyw nie został zapisany.

## Dwa pomiary tej samej hipotezy

```text
acvc score --repo . --variant PREFIKS --metric tests_passed=10 --metric latency_ms=10
acvc score --repo . --variant PREFIKS --metric tests_passed=1 --metric latency_ms=1
acvc score --repo . --variant PREFIKS --metric tests_passed=0 --metric latency_ms=20
```

Dwa pierwsze są nieporównywalne i zostają na granicy. Trzeci jest gorszy w obu miarach, schodzi z granicy i zostaje w `why`.

## Zamknięcie z zamiarem

```text
acvc reject --repo . --variant PREFIKS --reason "unieważnianie cache pęka przy równoległym zapisie"
acvc spend --repo . --campaign szukanie --tokens 15000 --note "sesja hipotezy" --agent julio
```

## Śmieci poza drzewem

W zasianym folderze, `.acvcignore`:

```text
# raport
lab/reports
```

Dwa klony z tym samym plikiem pomijają te same ścieżki. Usunięcie pliku pozwala `lab/reports` wrócić przy następnym skanie. Już zapisane hipotezy nie są przepisywane.
