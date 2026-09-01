# Passo 1291 — relatório de execução parcial certificada

## Veredito

Execução parcial, conforme permitido pelo próprio passo. Cinco membros seguros
foram materializados e verificados: `math.bb`, `math.frak`, `math.inline`,
`math.scripts` e `math.serif`. Três membros permanecem abertos no gate humano
ADR-0127: `math.cancel`, `math.underline` e `math.vec`.

O regime Tekt A/B foi executado por capacidades e ordem: medição e ownership
somente leitura; L0 antes do código; Testador A com RED; Implementador B sem
escrita nos testes protegidos; veredito do coordenador. Como todos os papéis
partilharam filesystem, o resultado é executado **sem atestação técnica de
isolamento físico**.

## Proveniência

- HEAD: `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.
- Estado: working tree não commitado e já muito sujo antes de P1291; baseline
  exata em `p1291-baseline-status.txt`, SHA-256
  `46007bc74479c66497b0c1ccd94f1585ca3d0f0997b9ba58207f80c6eb4a794a`.
- Medição final: `2026-08-31T12:00:20-03:00`.
- SHA-256 de `git status --short` na medição final:
  `6fcce2fa4d42a4312248be2249ee3b16f8dfe5b544c5751990eadff9c1fb4f82`.
- SHA-256 de `git diff HEAD --stat` na medição final:
  `5c7a98512bd4d8e8fa5b5b6ffb8984e9549c35936caa1326d42c7ff4a54def28`.
- Vanilla: `/usr/local/bin/typst`, SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`,
  revisão ratificada `a51e02804`.

## Materialização segura

- Os quatro estilos reutilizam os function pointers donos de
  `compiler/stdlib/math_style.rs`; não há wrappers de namespace.
- `scripts` usa adapter ABI privado e o constructor canónico já existente de
  `MathLimitsOverrideElem`, forçando attachments laterais.
- A montagem continua fechada: nenhum scope global foi copiado para `math` e o
  espelho `sym → math` preservou precedência, kind e valores existentes.
- A medição vanilla refutou a expectativa histórica outer-wins. No mesmo eixo,
  o setter mais interno vence; eixos de glyph, tamanho, bold, italic e cramped
  continuam ortogonais.
- Mensagens públicas ficaram verbatim para ausência, excesso, tipo e named
  desconhecido, incluindo `integer`/`boolean`; String continua convertível nos
  estilos.

## Evidência RED→GREEN e gates

- Testes P1291: RED independente, depois `14 passed; 0 failed`.
- Regressão P311b: duas expectativas antigas outer-wins foram demonstradas RED,
  corrigidas pelo Testador A e fecharam `31 passed; 0 failed`.
- Regressões de tamanho/attachments: filtros P809, P812 e P992 verdes.
- Espelho do namespace: teste P1283 focal verde.
- Repetição bilateral: os cinco nomes curtos e as mensagens de erro coincidem;
  em ambos os binários, os SVGs confirmam
  `serif(bb(A B C)) == bb(A B C)` e
  `bb(serif(A B C)) == serif(A B C)`.
- `cargo build`: exit `0`.
- `cargo test --workspace`: não ficou globalmente verde; após os pacotes e os
  testes P1291 passarem, encontrou uma falha em
  `04_wiring/tests/cli.rs::p1168_html_typed_batch_repr_and_dom`. É alheia ao
  diff P1291 e pertence à superfície `repr` já modificada na árvore de entrada.
- `crystalline-lint .`: exit `0`; os três consumers P1291 estão resselados e
  ausentes do dry-run de drift. O lint global ainda lista V5 em consumers
  alheios já modificados na árvore partilhada; não foram reescritos por P1291.
- `git diff --check`: exit `0`.

## Resíduos explícitos

1. `cancel` requer transportar seis opções públicas e seus efeitos de layout;
   o `MathCancelElem` atual só possui body.
2. `underline` é um elemento matemático distinto no vanilla; aliasar a
   decoração textual é semanticamente incorreto.
3. `vec` requer identidade vetorial, delimitadores, alinhamento e gap; o
   `MathMatrixElem` atual não é contrato equivalente.
4. O `repr` de conteúdo produzido pelos cinco membros ainda difere e permanece
   dependência expressamente atribuída a P1290. P1291 não editou `repr.rs`.
5. As mensagens diagnósticas coincidem, mas spans continuam limitados pelo
   transporte atual de `Args`/diagnósticos; não se reivindica paridade de span.

Assim, este lote certifica cinco bindings funcionais, não o teto `+8 MATCH`.
O crédito final de inventário deve aguardar P1290 e a repetição dos probes; os
três membros no gate não recebem sucesso parcial.

## Adenda 2026-08-31 — redação dos contratos públicos após confirmação

Às `2026-08-31T12:25:40-03:00`, ainda no HEAD
`53d21c5a602f4045a769a0ab0c935baa5ecd3b88`, o humano autorizou a **redação**
dos L0 de `cancel`, `underline` e `vec`. Isso não constitui o selo ADR-0127 e
não autorizou testes ou código. SHA-256 de `git status --short` nesse ponto:
`28008b35f76d68389f201c3b5facd7382f4cf0f86dbe0ef898264c54a9e306ef`;
SHA-256 de `git diff HEAD --stat`:
`4c4f69a4b5bc748af2dbee2df43bd2a230e178081566cfbcbd96b97190bc2fa8`.

Hashes SHA-256 integrais dos rascunhos submetidos a selo:

| Prompt L0 | SHA-256 |
|---|---|
| `entities/elements/math_cancel.md` | `03d492ec7a83c9b004403180cb7456ee130d50dd903cae54386bdae9cbe7c1c3` |
| `compiler/math/layout/cancel.md` | `e9a32e5e4c38e2a5b86e6e9781a9207024d50d9ae3e6aa322ac4d0c0edeecd28` |
| `entities/content.md` | `bdcc9dc3c91d2893810769f368865a9dc0ad51a65e4681801f6102947aef4e52` |
| `entities/elements/_comum.md` | `35a47f1687612a3db221873efb376f0535375048d541bdf499af0c3853fa07c2` |
| `compiler/math/layout/_comum.md` | `25e10b845d7fff7e2ae55a45527cf780eac77b5022c45d9aa4f76fc0deb393d8` |
| `compiler/eval/math.md` | `55fe9ae296ee87fe8f6da89bd8ec01a2d9f86276547203e566f4af8dd0ce857e` |
| `compiler/stdlib/structural/math.md` | `b0a5cecefc8f833bd04136e4d495e69fe1f55a6d4c5462ce99cd0bcb7605d468` |
| `entities/elements/math_underline.md` | `fffed91c5f497fba957044907c47498b05594ffc9f4bc37643db3abd4d5f18af` |
| `entities/elements/math_vec.md` | `c66e12a31bfa5b4a43b1378bfff22bf890d6ade77348e06f3b260c8a043378fa` |
| `compiler/math/layout/underline.md` | `dbae17a3475e287a22a749e8a6283b2e0683b6087d9a5e3f01375c35e6bff097` |
| `compiler/math/layout/vec.md` | `87692f9193bc1c4c8ff5606a838228608fabb8889d2869802ed634269d87404f` |

Os prompts novos permanecem explicitamente sem consumer e sem `Hash do
Código` durante a pausa, portanto V15 não pode ser usado como sucesso antes da
materialização. A tentativa somente leitura
`crystalline-lint --checks v15,v26 --format text .` nem chegou a esse gate:
terminou com exit `1` por metadados canónicos malformados já presentes em
`compiler/eval/table.md` e `entities/compiler_features.md`, ambos alheios a
P1291 na árvore compartilhada. Nenhum hash de consumer foi reescrito.

Duas incompletudes de fase foram nomeadas, em vez de ocultadas:
`P1291.cancel-angle-runtime` (callback depois de medir o body) e
`P1291.vec-region-gap` (percentual contra altura da região). Ambas exigem L0
dos owners de pipeline e nova confirmação antes da respectiva implementação.

O regime continua executado **sem atestação técnica de isolamento físico**,
porque autores e verificadores partilham o mesmo filesystem.
