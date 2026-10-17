# Item 6 — experimento de avaliação única do receptor

## Proveniência

Execução em 2026-10-09 11:03:55–11:04:30 UTC, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. A bateria guardou o estado exato no início em [status](arrays-dicts-item6-2026-10-09/dispatch-experiment-git-status-short.txt) e [diff stat](arrays-dicts-item6-2026-10-09/dispatch-experiment-git-diff-head-stat.txt), além dos hashes das entradas, fontes e binários em [medições](arrays-dicts-item6-2026-10-09/dispatch-experiment-measurements.json). O Tekt anterior tem SHA-256 `4c2b720e7f242ec9eb6b725ae5d91854c601680f5deebd9014c4e47219fc61b0`; o candidato release tem `076633d63083aaeed0f51443960350412bb6672457d3841629681d8439a8273f`. A receita é [measure-dispatch-experiment.py](arrays-dicts-item6-2026-10-09/measure-dispatch-experiment.py): 9 fixtures, 3 blocos × 4 pares em ordem alternada, processo fresco, CPU fixada, fontes e ambiente constantes. `peak_rss_kib` vem de `/usr/bin/time -f %M`; mede pico do processo, não heap vivo. A referência vanilla permanece o binário pinado no diagnóstico principal do item 6.

## Teste RED→GREEN

O receptor `({ n += 1; (1, 2, 3) }).len()` dentro de um bloco local retorna `(3, 2)` no Tekt anterior e `(3, 1)` no vanilla pinado: o Tekt avaliava o receptor duas vezes. Após a mudança, o Tekt retorna `(3, 1)`. O teste unitário `collection_len_avalia_receptor_uma_vez` passou. Os L0 de `call`, `instance_methods`, `collection_methods` e `tests` foram editados antes da implementação e seus hashes bidirecionais foram selados. O despacho de instância passa o `Value` já avaliado ao de coleção quando não intercepta o método, sem reavaliar o AST ou clonar o receptor na passagem.

## Clones e RSS

O GDB contou entradas em `<Value as Clone>::clone` no binário debug SHA-256 `f4cb66c15ad09272a111b71f214415d3921418f7fdccaf82833576e456c927db`. Logs: [array](arrays-dicts-item6-2026-10-09/dispatch-experiment-array-clones.log) e [dicionário](arrays-dicts-item6-2026-10-09/dispatch-experiment-dict-clones.log); estado de código e diff stat foram capturados antes da série em [status](arrays-dicts-item6-2026-10-09/dispatch-experiment-clone-status.txt) e [diff stat](arrays-dicts-item6-2026-10-09/dispatch-experiment-clone-diff-stat.txt). A linha anterior usa o binário debug SHA-256 `1428a872950ceb2b338929160e93528022ba48adead5a4a1c1293c4b63cc61e5`, documentado em `clone-results.json`.

| Documento | Clones antes | Clones depois | Redução | RSS mediano antes/depois | Delta RSS pareado mediano |
|---|---:|---:|---:|---:|---:|
| array 20.000 | 41.350 | 21.349 | 20.001 | 33.732 / 32.022 KiB | −1.610 KiB |
| array 40.000 | — | — | — | 39.234 / 35.374 KiB | −3.820 KiB |
| dict 2.000 | 19.360 | 17.359 | 2.001 | 29.334 / 29.454 KiB | +20 KiB |
| plain | 1.336 | não recontado | — | 28.748 / 28.910 KiB | +156 KiB |

No array de 20 mil, o documento sem `.len()` tinha 1.348 clones antes; depois da correção, `.len()` acrescenta 20.001 clones, contra 40.002 antes. No dict de 2 mil, o documento sem `.len()` mantém 15.358 clones; `.len()` acrescenta 2.001, contra 4.002 antes. Isto confirma a eliminação de uma das duas cópias profundas. A cópia restante nasce da leitura do binding em `expression_dispatch.rs:79-81`; este experimento não decide mudar a representação de `Value`.

Para array 20 mil, os 12 deltas de RSS pareados ficaram entre −2.016 e −1.280 KiB; para array 40 mil, entre −4.188 e −3.344 KiB. Os menores casos e os dicionários permaneceram dentro da variação de RSS desta bateria; não há prova de redução nesses casos. As medianas de tempo constam no JSON e não foram usadas como decisão.

## Equivalência e validação

Antes e depois produziram o mesmo texto extraído nos 108 pares. A [qualificação contra vanilla](arrays-dicts-item6-2026-10-09/dispatch-experiment-qualification.json), com seu próprio [status](arrays-dicts-item6-2026-10-09/dispatch-experiment-qualification-git-status-short.txt) e [diff stat](arrays-dicts-item6-2026-10-09/dispatch-experiment-qualification-git-diff-head-stat.txt), confirmou nos nove casos mesmas palavras, páginas e raster a 36 dpi; desvio máximo de caixa de palavra de 0,00002 pt. `cargo test -p typst-core collection_len_avalia_receptor_uma_vez --lib`, `cargo build`, build release, `git diff --check` e `crystalline-lint .` passaram. O linter saiu com código 0 e mantém warnings preexistentes fora dos pares L0/L1 deste experimento.
