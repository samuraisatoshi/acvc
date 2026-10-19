# Złe użycie

To sposoby, w które zapis przestaje znaczyć to, co mówi. Nieudane wersje zostają tutaj. Git dostaje tylko wersję, którą osoba zatwierdza.

- Otwieranie `.acvc/graph.sqlite`, `objects.log` albo `objects.pack`, żeby czytać status, granicę albo przesłankę. Umową jest program.
- Traktowanie Gita jako grafu hipotez. Commit nie trzyma przegranej próby ani motywu, którego nie zapisano.
- Drugi `seed` w tej samej kampanii. Kampania ma już pochodzenie. Drogą jest `record` od rodzica.
- Odrzucenie, bo inna hipoteza zdominowała. To zapisuje motyw, którego nikt nie podał.
- Scalanie dwóch nieporównywalnych hipotez albo ponowne wiązanie luźnych seedów, żeby granica miała jednego zwycięzcę.
- Użycie `spend` jako budżetu, miary granicy albo blokady `record`. Wymyślanie liczby tokenów.
- Wiara, że ostatni `score` kasuje poprzedni, albo mieszanie `tests_passed` jednej próby z `latency_ms` innej.
- Czytanie `.gitignore` jako polityki skanu. Jawną polityką jest `.acvcignore`.
- Hardlink między drzewami. Symlink w zasianym folderze.
- `promote` jako merge albo `push`/`fetch` z oczekiwaniem checkoutu po drugiej stronie.
- Pokazywanie człowiekowi hasha, krawędzi albo „left/right”. Zdaniem jest przesłanka i to, co zostaje.
- Sklejanie wszystkich przyczyn w jedną potrzebę kampanii, gdy graf ma więcej niż jedną przyczynę.
- Przepisywanie już zapisanej hipotezy, żeby „poprawić” przesłankę. Nowa przesłanka to `revise_premise` albo `record`, inny węzeł.
- Uruchamianie `acvc web` albo `acvc tui` z sesji agenta, gdy człowiek ma już otwarty podgląd. Podaj polecenie do jego terminala.
