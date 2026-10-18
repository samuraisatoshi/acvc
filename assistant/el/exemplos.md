# Παραδείγματα

## Άνοιγμα εκστρατείας

```text
acvc init --repo .
acvc campaign create anazitisi --metric tests_passed:higher --metric latency_ms:lower
acvc seed --repo . --campaign anazitisi --path ./ypothesi --premise "η cache χωράει στον δίσκο" --assumption "ο δίσκος είναι τοπικός" --intent "λιγότερες αναγνώσεις" --agent julio
```

Το JSON του seed φέρνει την παραλλαγή, το δέντρο, πόσα αρχεία μπήκαν και ποια ζύγιζαν περισσότερο.

## Η επόμενη υπόθεση

```text
acvc record --repo . --workspace ./ypothesi --parent PROTHEMA --premise "το ευρετήριο είναι δέντρο" --assumption "τα κλειδιά είναι ταξινομημένα" --intent "συντομότερη αναζήτηση" --agent julio --edge mutate
acvc score --repo . --variant PROTHEMA --metric tests_passed=16 --metric latency_ms=4
```

## Να δείτε τι ακόμη συναγωνίζεται

```text
acvc context --repo . --campaign anazitisi
acvc frontier --repo . --campaign anazitisi
acvc why --repo . PROTHEMA
acvc recall --repo . --campaign anazitisi --query "δέντρο"
acvc compare --repo . PROTHEMA_A PROTHEMA_B
```

Ένα `why` υπόθεσης που ξεπεράστηκε, χωρίς `reject`, ονομάζει ποιος κέρδισε, το μέτρο, και λέει ότι το κίνητρο δεν καταγράφηκε.

## Δύο μετρήσεις της ίδιας υπόθεσης

```text
acvc score --repo . --variant PROTHEMA --metric tests_passed=10 --metric latency_ms=10
acvc score --repo . --variant PROTHEMA --metric tests_passed=1 --metric latency_ms=1
acvc score --repo . --variant PROTHEMA --metric tests_passed=0 --metric latency_ms=20
```

Οι δύο πρώτες είναι ασύγκριτες και μένουν στο σύνορο. Η τρίτη είναι χειρότερη και στα δύο μέτρα, φεύγει από το σύνορο και μένει στο `why`.

## Σκόπιμο κλείσιμο

```text
acvc reject --repo . --variant PROTHEMA --reason "η ακύρωση της cache σπάει με ταυτόχρονες εγγραφές"
acvc spend --repo . --campaign anazitisi --tokens 15000 --note "συνεδρία της υπόθεσης" --agent julio
```

## Να μείνει έξω το άχρηστο

Στον σπαρμένο φάκελο, `.acvcignore`:

```text
# αναφορά
lab/reports
```

Δύο κλώνοι με το ίδιο αρχείο αφήνουν έξω τις ίδιες διαδρομές. Η διαγραφή του αρχείου αφήνει το `lab/reports` να επιστρέψει στην επόμενη σάρωση. Υποθέσεις ήδη καταγεγραμμένες δεν ξαναγράφονται.
