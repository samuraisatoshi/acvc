# Cel

Git przechowuje to, co człowiek postanowił zapisać. Agent kodu testuje hipotezy hurtowo, cofa się, przepisuje przesłankę i zapomina sesję. Następny commit nie mówi, który pomiar wciąż konkuruje, ani dlaczego drugi został porzucony.

ACVC istnieje dla tej pamięci.

Obiektem jest wariant: hipoteza z przesłanką, drzewem Merkle i próbami tylko dopisywanymi. Kampania deklaruje miary. Granica to zbiór tego, czego pomiar jeszcze nie pokonał. Recall znajduje przesłankę po tekście, ze statusem i rodzicem.

Motywem jest następna sesja. Ma znaleźć to, co wciąż konkuruje, bez ponownego czytania czatu, bez traktowania ostatniego `score` jako prawdy i bez scalania dwóch hipotez, które mierzą rzeczy nieporównywalne.

Git pozostaje składem zwycięskiego wariantu w checkoutcie, który już istnieje. ACVC nie zastępuje historii Gita i nie robi merge.
