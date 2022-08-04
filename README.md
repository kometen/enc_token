Dette program kan lave et JWT token som sendes til maskinporten
for at hente et token derfra.

Det token som leveres af maskinporten kan så benyttes af en
konsument.

Det læser integrasjons-id'en, scope samt URI'en til maskinporten
fra filen .env (dot env), samt den private nøkkel som er i
private.key. Disse to filer skal ikke tilføjes git, så de ved
en fejl lægges ud til offentlig skue.

Disse to filer ligger i roden, i samme mappe som Cargo.toml.

Programmet oversættes med `cargo build` og programmet ligger i
`./target/debug/enc_token`. Ønskes der mere fart på programmet
kan det oversættes med `cargo build --release`. Så ligger
det i `./target/release/enc_token` i stedet.

En .env-fil fil kan se slik ut.

```
INTEGRATION_ID = integrasjons-UUID
AUD = https://maskinporten-url/
SCOPE = et:scope.skriv
```

Og den private nøkkel noko ala. dette.

```
-----BEGIN PRIVATE KEY-----
foo
bar
baz
-----END PRIVATE KEY-----
```

Denne må naturligvis ikke komme på afveje.

Kan pipes til jq så outputtet er enklere at læse.
