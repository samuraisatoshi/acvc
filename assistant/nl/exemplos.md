# Voorbeelden

## Een campagne openen

```text
acvc init --repo .
acvc campaign create zoek --metric tests_passed:higher --metric latency_ms:lower
acvc seed --repo . --campaign zoek --path ./hypothese --premise "de cache past op schijf" --assumption "de schijf is lokaal" --intent "minder leesacties" --agent julio
```

De JSON van seed brengt de variant, de boom, hoeveel bestanden binnenkwamen en welke zwaarder wogen.

## De volgende hypothese

```text
acvc record --repo . --workspace ./hypothese --parent PREFIX --premise "de index is een boom" --assumption "de sleutels zijn gesorteerd" --intent "een kortere zoekactie" --agent julio --edge mutate
acvc score --repo . --variant PREFIX --metric tests_passed=16 --metric latency_ms=4
```

## Zien wat nog meedingt

```text
acvc context --repo . --campaign zoek
acvc frontier --repo . --campaign zoek
acvc why --repo . PREFIX
acvc recall --repo . --campaign zoek --query "boom"
acvc compare --repo . PREFIX_A PREFIX_B
```

Een `why` van een voorbijgestreefde hypothese, zonder `reject`, noemt wie won, de maat, en zegt dat het motief niet is vastgelegd.

## Twee metingen van dezelfde hypothese

```text
acvc score --repo . --variant PREFIX --metric tests_passed=10 --metric latency_ms=10
acvc score --repo . --variant PREFIX --metric tests_passed=1 --metric latency_ms=1
acvc score --repo . --variant PREFIX --metric tests_passed=0 --metric latency_ms=20
```

De eerste twee zijn onvergelijkbaar en blijven op de grens. De derde is slechter op beide maten, verlaat de grens en blijft in `why`.

## Bewust afsluiten

```text
acvc reject --repo . --variant PREFIX --reason "cache-invalidatie breekt bij gelijktijdig schrijven"
acvc spend --repo . --campaign zoek --tokens 15000 --note "sessie van de hypothese" --agent julio
```

## Rommel uit de boom houden

In de gezaaide map, `.acvcignore`:

```text
# rapport
lab/reports
```

Twee klonen met hetzelfde bestand laten dezelfde paden weg. Het bestand wissen laat `lab/reports` terugkomen bij de volgende scan. Al vastgelegde hypothesen worden niet herschreven.
