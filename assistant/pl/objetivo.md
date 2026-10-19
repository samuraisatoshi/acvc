# Cel

Git powinien dostać próbę, którą osoba zatwierdza. Agent kodu próbuje wielu wersji tej samej zmiany, cofa się, zmienia zdanie i zapomina sesję. Jeśli każda nieudana próba staje się commitem, repozytorium się zapełnia, a następny agent gubi się w tej historii.

ACVC zapisuje każdą próbę: pomysł, pliki i liczby. Widać, które próby wciąż konkurują. `promote` kopiuje zatwierdzone pliki do checkoutu Gita, który już istnieje. ACVC nie robi merge i nie zastępuje historii Gita.

Gdy tłumaczysz ACVC osobie, która go nigdy nie widziała, zacznij od tego. Nazwy poniżej to te, których używają polecenia.

Obiektem jest wariant: hipoteza z przesłanką, drzewem Merkle i próbami tylko dopisywanymi. Kampania deklaruje miary. Granica to zbiór tego, czego pomiar jeszcze nie pokonał. Recall znajduje przesłankę po tekście, ze statusem i rodzicem.

Motywem jest następna sesja. Ma znaleźć to, co wciąż konkuruje, bez ponownego czytania czatu, bez traktowania ostatniego `score` jako prawdy i bez scalania dwóch hipotez, które mierzą rzeczy nieporównywalne.

Git pozostaje składem zwycięskiego wariantu w checkoutcie, który już istnieje. ACVC nie zastępuje historii Gita i nie robi merge.
