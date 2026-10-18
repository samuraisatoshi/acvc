# Примеры

## Открыть кампанию

```text
acvc init --repo .
acvc campaign create poisk --metric tests_passed:higher --metric latency_ms:lower
acvc seed --repo . --campaign poisk --path ./gipoteza --premise "кеш помещается на диск" --assumption "диск локальный" --intent "меньше чтений" --agent julio
```

JSON команды seed приносит вариант, дерево, сколько файлов вошло и какие весили больше.

## Следующая гипотеза

```text
acvc record --repo . --workspace ./gipoteza --parent PREFIKS --premise "индекс — это дерево" --assumption "ключи отсортированы" --intent "более короткий поиск" --agent julio --edge mutate
acvc score --repo . --variant PREFIKS --metric tests_passed=16 --metric latency_ms=4
```

## Увидеть, что ещё соревнуется

```text
acvc context --repo . --campaign poisk
acvc frontier --repo . --campaign poisk
acvc why --repo . PREFIKS
acvc recall --repo . --campaign poisk --query "дерево"
acvc compare --repo . PREFIKS_A PREFIKS_B
```

`why` превзойдённой гипотезы без `reject` называет, кто победил, меру и говорит, что мотив не записан.

## Два измерения одной гипотезы

```text
acvc score --repo . --variant PREFIKS --metric tests_passed=10 --metric latency_ms=10
acvc score --repo . --variant PREFIKS --metric tests_passed=1 --metric latency_ms=1
acvc score --repo . --variant PREFIKS --metric tests_passed=0 --metric latency_ms=20
```

Две первые несравнимы и остаются на границе. Третья хуже по обеим мерам, уходит с границы и остаётся в `why`.

## Закрыть намеренно

```text
acvc reject --repo . --variant PREFIKS --reason "инвалидация кеша ломается при параллельной записи"
acvc spend --repo . --campaign poisk --tokens 15000 --note "сессия гипотезы" --agent julio
```

## Держать мусор вне дерева

В посеянной папке, `.acvcignore`:

```text
# отчёт
lab/reports
```

Два клона с тем же файлом оставляют те же пути снаружи. Удаление файла возвращает `lab/reports` при следующем обходе. Уже записанные гипотезы не переписываются.
