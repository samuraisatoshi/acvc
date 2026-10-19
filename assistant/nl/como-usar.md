# Gebruik

De stappen hieronder gebruiken de namen van de commando's. Een poging is een hypothese. Een meting is een proef. De pogingen die niemand op elk getal heeft verslagen, zijn de grens.

1. `acvc context --campaign NAAM` voordat je voorstelt, meet of weggooit.
2. Als de campagne niet bestaat: `acvc init --repo .` en `acvc campaign create NAAM --metric maat:higher`.
3. De eerste hypothese is `seed` van de map met de bestanden van de hypothese, met `--premise`, `--assumption`, `--intent` en `--agent`.
4. Een bestand veranderde: `record`. Zonder `--parent` is de ouder de hypothese waar de workspace naar wijst. `record` wijst de workspace daarna naar de nieuwe hypothese, dus de volgende `record` ketent daarop. Twee kinderen van de seed vragen `--parent` met het seed-id, of `materialize` van de seed vóór elke `record`. Doe geen tweede `seed`.
5. Er is een echt getal voor de verklaarde maat: `score`. Een onvolledige proef komt niet op de grens.
6. `frontier` toont wat nog meedingt. `why` legt één hypothese uit. `recall --query` vindt haar via de premissetekst.
7. Twee concrete metingen: `compare`.
8. De hypothese stierf om een bekend motief: `reject --reason`. Zonder motief wijs je niet af. De lezing zegt al dat het motief niet is vastgelegd.
9. De sessie verbruikte tokens en dat getal is bekend: `spend`. Schat niet.
10. De variant die won gaat Git in met `promote` in een checkout die al bestaat.

Identificatieprefix: 8 tot 64 hexadecimale tekens, zonder dubbelzinnigheid.

Voor een andere agent op dezelfde machine is het kanaal het uitvoerbare bestand of `acvc mcp`. De pagina en de TUI zijn voor de persoon. Vervang ze niet door een directe lezing van de database.
