# P1307 R2 — observabilidade contextual e controles completos

Estado: **estratégia de medição corrigida, sem autorização de implementação**.
A matriz documental R2 termina com zero `Unknown`: os valores contextuais são
observáveis bilateralmente nos quatro perfis pela CLI existente. Isto remove
o bloqueio de observabilidade R3 do predecessor, mas não resolve os bloqueios
de ownership, representação ou argumentos, nem constitui selo de encoders.
Regime: executado sem atestação de isolamento técnico. Autor: `/root/p1307_oracle`.

## Proveniência anterior às conclusões

HEAD `b303f1f15b610e09872b567027e0d806387fde8c`, mais o working tree P1306
certificado. O `diff_stat` capturado nas execuções registra exatamente:

```text
 .../prompts/compiler/eval/bindings/field_access.md |  94 +++++++++-
 00_nucleo/prompts/compiler/eval/tests.md           |  76 +++++++-
 01_core/src/compiler/eval/bindings/field_access.rs |   4 +-
 01_core/src/compiler/eval/tests.rs                 | 198 ++++++++++++++++++++-
 4 files changed, 360 insertions(+), 12 deletions(-)
```

| Entrada ou artefato | SHA-256 |
|---|---|
| `p1307-r2-preflight.json` | `f09eb44503af474a02f8f4823756e1672bb7d3266610c8805f28c9448536c257` |
| `p1307-r2-baseline.json` | `0c0e92fa4c84bb23a1e2adefc95e93ad75ffaf0f12f3edce58ab8e9393b2f611` |
| `p1307-r2-observe.py` | `ebf05054b737357c9ee80bbef764fe4c1cac37f8209af4af3d59b88108dcce62` |
| `p1307-r2-measurement.json` | `847faabad41df603a82f7fc5c5d0435180cdec66c33ae9b3a1fd55d7ee320fa2` |
| Vanilla `/usr/local/bin/typst`, upstream ratificado `a51e02804` | `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` |
| Baseline `/dev/shm/p1307-baseline-target.hqbv0ooo/release/typst` | `945e488c53246b33fd11c56ff670c697aeaffa5a92d5c15cc8e09a76cf7cf1f3` |

A matriz final começou em `2026-09-07T16:44:21.062280+00:00` e encerrou
em `2026-09-07T16:44:45.243137+00:00`: 48 casos × quatro perfis × dois
binários × três ordens = 1.152 execuções. `matrix.orders` preserva normal,
repetição e inversão; zero diferença em exit/stdout/stderr/observável e zero
`Unknown`. As 190 execuções focais anteriores permanecem em `attempts` e
`extra_focal`, com hipóteses, tempos, argv, source numerizada, stdin e bytes
de stdout. Concorrência limitada a quatro processos; não se alega execução serial.

## Medição e revisões do adaptador

| Revisão | Execuções | Unknown | O que foi medido |
|---|---:|---:|---|
| 0 | 24 | 6 | Query aceita metadata direto no perfil default, mas o baseline não aceita `--features` no subcomando. `array(bytes(...))` é constructor ausente no baseline, com diagnóstico de linguagem observado. |
| 1 | 40 | 24 | Compile com stdin `-` funciona no vanilla; no baseline significa o caminho literal `./-`. A projeção por `bytes.len/at` funciona no vanilla. Quatro Unknown vanilla adicionais eram classificação equivocada do marker impresso como trecho da source, preservados. |
| 2 | 40 | 0 | Ficheiros `.typ` temporários explícitos e `compile --features` existente tornam o transporte bilateral. O parser aceita somente a linha real `error: panicked with: P1307R2:...:END`. |

Não houve terceira revisão. Depois do sucesso focal, 43 novos casos receberam
sondagem bilateral default (86 execuções), antes da matriz integral. Nenhuma
expectativa foi recalculada usando candidato: não existe candidato P1307.

`02_shell/src/cli.rs:313-324,411-420` confirma que QueryArgs/QueryIntent não
possuem features; `04_wiring/src/main.rs:512-557` e
`03_infra/src/query_helpers.rs:459-481` mostram o caminho de query distinto.
Os perfis necessários já existem em CompileArgs (`02_shell/src/cli.rs:144-147`).
A solução documental usa esse caminho, sem adicionar ou alterar a CLI.

## Rota contextual validada

O documento contém um heading rotulado e um bloco `context`. O bloco espera
que `query(<probe>)` devolva conteúdo real, então avalia a expressão sobre
`found.first()` ou `found.first().location()`. `type(...)` confirma `content`
e `location` dos dois lados. A espera evita confundir a primeira passagem sem
resultados com falha de construção.

A string pública é convertida em Bytes e transportada por seus octetos:

```typst
let raw = bytes(public-string)
panic("P1307R2:" + range(raw.len()).map(i => str(raw.at(i))).join(",") + ":END")
```

O diagnóstico intencional é apenas transporte. Seu conteúdo decimal é
reconstituído sem normalização: o recibo guarda a Str completa, a lista UTF-8
e SHA-256, além de stderr integral. A sentinela `α`, newline e aspas mediu
exatamente os octetos `[206,177,10,34,120,34]` em ambos os lados e perfis.
Não se usa `Debug`, truncamento de array em repr, round-trip ou ordenação de
mapas para comparar a Str do encoder.

As fixtures são criadas por `apply_patch` em diretórios temporários exclusivos;
conteúdo, nome absoluto e hash permanecem no recibo. Os mesmos ficheiros são
reutilizados intactos nas três ordens. O caminho do ficheiro aparece diferente
na apresentação do diagnóstico de cada binário (relativo no vanilla, absoluto
no baseline); nenhuma dessas saídas foi normalizada ou declarada igual.

Os controles `html.elem` e `pdf.table-summary` confirmam que as flags têm
efeito real, não apenas que a CLI aceita o texto:

| Perfil | `html.elem` | `pdf.table-summary` |
|---|---|---|
| default | diagnóstico de feature desativada | diagnóstico de feature desativada |
| html | função `elem` | diagnóstico de feature desativada |
| a11y | diagnóstico de feature desativada | função `table-summary` |
| html+a11y | função `elem` | função `table-summary` |

Location mede `location(..)` no vanilla e `location(...)` no baseline em todos
os perfis. A inferência anterior do formatter agora tem testemunha bilateral.
LocatedContent mede repr longa de heading no vanilla e
`heading(level: 1)[[Probe]]` no baseline. Essa diferença de repr não autoriza
encodar Content como fallback: a serialização dedicada é uma obrigação distinta.

Os seis casos `context-{json,toml,yaml}-{Location,LocatedContent}` congelam os
resultados vanilla completos. No baseline, todos observam o diagnóstico exato
de membro ausente. Não são Unknown nem implementação RED. O JSON do conteúdo
localizado inclui `func`, `level`, `depth`, `offset`, `numbering`, `supplement`,
`outlined`, `bookmarked`, `hanging-indent`, `body` e `label`. O resultado inteiro,
inclusive defaults, ordem e espaços, está em `matrix.orders.normal`.

## CBOR: payload integral agora observável

Cada caso `cbor-payload-*` usa somente linguagem pública:

```typst
{ let raw = cbor.encode(value); range(raw.len()).map(i => raw.at(i)) }
```

Os sete controles são dictionary, Bytes, Symbol, Content, Bytes vazio, none
e float negativo zero. O transporte CLI JSON entrega todos os octetos, não
apenas `bytes(N)`. Exemplo Bytes `(0,255,10)` → `[67,0,255,10]` bilateralmente.

As dívidas CBOR preexistentes continuam identificadas: `sym.alpha` produz
`[98,206,177]` no vanilla e
`[108,115,121,109,98,111,108,40,34,206,177,34,41]` no baseline. Content `[hi]`
é mapa no vanilla e string `[100,91,104,105,93]` no baseline. O futuro controle
de preservação deve usar o payload baseline individualizado; isto não autoriza
corrigir CBOR silenciosamente nem inventa obrigação de igualdade física de
outros resultados internos.

## `.with`: ocorrências e origens que não podem ser descartadas

Os casos extras incluem parent com positional/named inválido, named inválido
pré-ligado no encoder, `pretty:false` pré-ligado, override válido, valor inválido
na primeira/última ocorrência, identidade e spread de Args de duas origens.
São casos observados nos quatro perfis e nas três ordens.

`json.encode.with(pretty:"bad")(...,pretty:true)` e o paralelo TOML rejeitam
a ocorrência original inválida; o cast não se limita ao named final.
Na source congelada de `with-json-invalid-first-occurrence`, o diagnóstico
é `expected boolean, found string`, range UTF-8 half-open `25..30` (`"bad"`).
No caso `with-json-invalid-last-occurrence`, aponta `58..63`.
`with-yaml-encoder-bound-pretty` aponta `17..30` (`pretty: false`) na ligação.

Os pares `with-spread-arguments-f/g` diferem somente na chamada final `f/g`:
a escolha interna determina dois Args com os mesmos valores, mas origens
distintas. O vanilla aponta `46..53` para `a` e `78..85` para `b`, ambos
`nope: 1`. Com captura por closure `((..xs)=>xs)`, os ranges são `51..58`
e `88..95`. Traces e stderr integrais permanecem no recibo. A medida sustenta
que preservar somente valores fundidos e o span da chamada `.with` perde
informação pública exigida; o desenho de carrier pertence à auditoria separada.

**Retificação documental R2, sem reescrever o predecessor:** a afirmação de
`p1307-contract.md` de que `.with()` retorna repr `encode` está incorreta.
Os três casos `with-{json,toml,yaml}-encoder-with-identity` medem
`["function","(..) => .."]` no vanilla. A função original `json.encode`
continua com repr curto `encode`; a função parcialmente aplicada é outra
observação. As obrigações futuras devem usar a medição R2.

## Limites e reprodução

Esta auditoria fecha a lacuna de transporte contextual, completa os controles
CBOR selecionados e acrescenta as rotas `.with` explicitamente listadas. Não
prova paridade de implementação, completude geral de Content, nem mutation
score. Os três encoders continuam ausentes no baseline. A origem de argumentos
e o fallback público divergente ainda exigem o desenho e a autorização do
escopo sucessor; nenhum L0, Rust, header, Cargo ou evidência anterior foi editado.

Os comandos executados foram `python3 -B p1307-r2-observe.py initial`,
`contextual`, `file-contextual`, `focal-extra` e `full`, a partir da raiz do
repositório, com acesso ao binário pinado no tmpfs do host. O script bloqueia
reescrita da mesma fase já registrada. Para reprodução de leitura, pode-se
importá-lo, carregar `matrix.cases` do JSON e chamar `matrix(cases, profiles)`;
as fixtures e hashes estão registrados e os processos não alteram o produto.
Uma nova cadeia que precise recriar artefatos permanentes deve escolher nomes
sucessores; não sobrescrever os recibos congelados desta R2.
