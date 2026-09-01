# P1292 — recibo pré-gate L0 e ownership

**Papel:** Autor de contrato + Auditor de ownership, sequência 2
**Estado:** PARAGEM ADR-0127 — contrato redigido; nenhum RED/código/ataque/veredito autorizado
**HEAD observado:** `0eb39f8ecb48930515f2cadb6a378450855b5a72`
**Instante da selagem documental:** `2026-08-31T22:35:03-03:00`
**Working tree:** não commitado; os inputs frescos P1292 permanecem untracked e
os L0s abaixo estão modificados. `git diff HEAD --stat` foi capturado antes
deste recibo; arquivos untracked não aparecem nesse stat.

## Entradas congeladas

| Artefato | SHA-256 |
|---|---|
| `00_nucleo/diagnosticos/p1292-manifest.json` | `673ec4b7c4308020e45e8cbe228ad1e25c6f6c8607df86d27cd08846774179e6` |
| `00_nucleo/diagnosticos/p1292-baseline-status.txt` | `fbddc7594893df1dfae68451893bce3136792b3cd2470bb981d6f57d1ccb0eb2` |
| `00_nucleo/diagnosticos/p1292-vanilla-measurement-receipt.md` | `dd8009cac21d3ccadd061ef9ea4ce1a83a8f9eda75264a9c90f54b7dad2bb929` |
| vanilla `/usr/local/bin/typst` | `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` |

## Gate V15/V26 antes da redação

Comando, sobre o HEAD e as três entradas untracked acima:

```text
crystalline-lint . --checks v15,v26 --fail-on warning
```

Saída e código:

```text
✓ No violations found
exit 0
```

## Matriz Prompt L0 ↔ consumer

| Prompt proprietário | Consumer produtivo único | Estado no gate |
|---|---|---|
| `entities/elements/math_cancel.md` | `01_core/src/entities/elements/math_cancel.rs` | existente; 1:1 |
| `compiler/math/layout/cancel.md` | `01_core/src/compiler/math/layout/cancel.rs` | existente; 1:1 |
| `entities/elements/math_underline.md` | `01_core/src/entities/elements/math_underline.rs` | planejado; ausente; sem `Hash do Código` |
| `compiler/math/layout/underline.md` | `01_core/src/compiler/math/layout/underline.rs` | planejado; ausente; sem `Hash do Código` |
| `entities/elements/math_vec.md` | `01_core/src/entities/elements/math_vec.rs` | planejado; ausente; sem `Hash do Código` |
| `compiler/math/layout/vec.md` | `01_core/src/compiler/math/layout/vec.rs` | planejado; ausente; sem `Hash do Código` |
| `compiler/stdlib/structural/math.md` | `01_core/src/compiler/stdlib/structural/math.rs` | existente; 1:1 |
| `compiler/eval/math.md` | `01_core/src/compiler/eval/math.rs` | existente; 1:1 |
| `compiler/math/layout/_comum.md` | `01_core/src/compiler/math/layout/mod.rs` | existente; 1:1 |
| `entities/content.md` | `01_core/src/entities/content.rs` | existente; 1:1 |
| `entities/elements/_comum.md` | `01_core/src/entities/elements/mod.rs` | existente; 1:1 |
| `compiler/eval/repr.md` | `01_core/src/compiler/eval/repr.rs` | existente; 1:1 |
| `compiler/stdlib/layout.md` | `01_core/src/compiler/stdlib/layout.rs` | existente; 1:1 |
| `compiler/eval.md` | `01_core/src/compiler/eval/mod.rs` | existente; 1:1 |
| `compiler/layout.md` | `01_core/src/compiler/layout/mod.rs` | existente; 1:1 |
| `entities/elements/flush.md` | `01_core/src/entities/elements/flush.rs` | planejado; ausente; sem `Hash do Código` |
| `compiler/layout/flush.md` | `01_core/src/compiler/layout/flush.rs` | planejado; ausente; sem `Hash do Código` |

Nenhum owner de `place` foi alterado para absorver `flush`. A nativa
`native_flush` permanece no owner existente de stdlib/layout e o namespace
permanece no owner existente do scope base. Nenhum Núcleo Tekt novo foi criado:
não surgiu claim compartilhada que justificasse TOML 1:N; pins existentes
permaneceram inalterados.

## Contrato fixado por lote

- **A — cancel:** o payload/transcript P1291 é preservado; P1292 apenas expõe
  o mesmo constructor em `math.cancel` e conserva presença de named para
  morfologia. Nenhuma callback é executada em stdlib/layout.
- **B — underline:** identidade math própria body-only e free function de
  layout na camada de render, usando constantes MATH; nenhum alias textual.
- **C — vec:** identidade própria, filhos variádicos, defaults/named/casts,
  sintaxe e chamada canônicas, forma B e `gap` contra região efetiva. Em
  `height:auto`, a parcela relativa colapsa a zero e a absoluta sobrevive.
- **D — place.flush:** sentinela zero-field, namespace preservado por `.with`,
  owner de entidade/layout próprios e drenagem somente do prefixo de floats no
  ponto exato; sem item/cursor próprio e sem antecipar floats posteriores.

## Delta morfológico de `repr`

| Construção | Baseline cristalino | Contrato P1292 |
|---|---|---|
| `math.cancel([x])` | binding ausente em `math`; repr interno `cancel([x])` | `cancel(body: [x])` |
| cancel com named | campos descartados pelo repr atual | campos presentes em ordem `length,inverted,cross,angle,stroke,background` |
| `math.underline([x])` | binding/identidade ausentes | `underline(body: [x])` |
| `math.vec()` | binding ausente; syntax sugar degradado a matrix | `vec(children: ())` |
| vec explícito | identidade/presença perdidas | `delim`, `align`, `gap`, depois `children` |
| `place.flush()` | field access ausente | `flush()` |

Presença é observável mesmo para default explícito. Probe suplementar
reproduzido em `2026-08-31T22:35:13-03:00`, no mesmo binário pinado:

```text
repr(math.cancel([x], inverted: false))
→ "cancel(body: [x], inverted: false)"
repr(math.cancel([x], angle: auto))
→ "cancel(body: [x], angle: auto)"
repr(math.vec([a], align: center))
→ "vec(align: center, children: ([a],))"
repr(math.vec([a], gap: 0.2em))
→ "vec(gap: 0% + 0.2em, children: ([a],))"
```

Isso exige metadado tipado de presença nos payloads de cancel/vec; comparar
somente valor com default não satisfaz a morfologia. Os bits não participam da
geometria nem criam outro runtime.

## Hashes dos L0s redigidos

| L0 | SHA-256 |
|---|---|
| `entities/elements/math_cancel.md` | `cc164630f81c98c08951bae556cf9d12dbe094649f0e5ecb645bb4cbb825e3cc` |
| `compiler/math/layout/cancel.md` | `84b2c85216b568155318284d6a4579a8c13f232ad32dd9472e2207a0ef394ffd` |
| `entities/elements/math_underline.md` | `27b6cc393f0ccb97358971ff7c20ae335f01e1f360635bc46ae8b3993f180ca7` |
| `compiler/math/layout/underline.md` | `41d15e707a058f1d94eb9a37a1fa5f70cc216c77f8a5c609af3dc6c2d12a0582` |
| `entities/elements/math_vec.md` | `72f2ba668f201d0fb0edf7d2d6bd8f5f85aab156b883a8e644b427f7cd2d1afa` |
| `compiler/math/layout/vec.md` | `772331ee68ec1069b57e80dd6674ef60d62fcd8db83d7d3fae15a18b7b82b764` |
| `compiler/stdlib/structural/math.md` | `087e24d091d9ab8a191374d5a1ac8c47f8e01a7e2283602f73cc0028cb53e236` |
| `compiler/eval/math.md` | `f40e487b4462da0b90bb31236f5c4ce3279b7b6cfa01f38bc8f6b4f80d23f8e8` |
| `compiler/math/layout/_comum.md` | `903788030729e81faa7cf08622a673454f29301cfb99b8fa7c2adf317db31746` |
| `entities/content.md` | `48212970feaf1e46db64e31eaeba606e0dcbc283bba2c86a28dd679c3f51e7e9` |
| `entities/elements/_comum.md` | `07982785a86c500f204e8ed15dafa801105067f2c98ca90e6c6f33765583a0c3` |
| `compiler/eval/repr.md` | `4d5247c2eee78b0e0dbfb3958059f0aa63ebaacee8d31537f8140b25b87b3eaf` |
| `compiler/stdlib/layout.md` | `6ff688ec12444582ec9ec87bb7b432019ff568882fbb1c66170eb47367e62dc4` |
| `compiler/eval.md` | `36aad0a2e4588aec7a9029745f67a0a9a7dd0fa8bae4c2bf5576b09e6ef13b2b` |
| `compiler/layout.md` | `a0b310323c44777a0a906ea09f620221d1e0ec0d85d6cb920436ca275bae0da4` |
| `entities/elements/flush.md` | `7e1290bd948e4270bc380bf13fbf209819a679cfa87ad7d58280814e83cb9319` |
| `compiler/layout/flush.md` | `5cdce0d36980c63422b540928414ee634d462b9386f3fba0b5bbf4d2c5f05bb8` |

## Gate V15/V26 depois da redação

Comandos:

```text
crystalline-lint . --checks v15,v26 --fail-on warning
git diff --check
```

Saídas e códigos:

```text
✓ No violations found
exit 0

git diff --check: sem saída
exit 0
```

Como `git diff --check` não inclui arquivos untracked, os seis L0s sem consumer
foram verificados explicitamente com:

```text
rg -n '[ \t]+$' \
  00_nucleo/prompts/entities/elements/math_underline.md \
  00_nucleo/prompts/compiler/math/layout/underline.md \
  00_nucleo/prompts/entities/elements/math_vec.md \
  00_nucleo/prompts/compiler/math/layout/vec.md \
  00_nucleo/prompts/entities/elements/flush.md \
  00_nucleo/prompts/compiler/layout/flush.md
```

Saída vazia, exit `1` (sem correspondências), portanto nenhum whitespace
terminal nesses seis arquivos.

Não foi executado `--fix-hashes`. V5 não foi reparado: os consumers
existentes devem ser ressellados somente na sequência autorizada após o gate;
os seis consumers novos continuam ausentes e seus prompts continuam sem
`Hash do Código`.

## PARAGEM EXPLÍCITA

**PARAR AQUI.** Este recibo encerra exclusivamente a autoria de contrato e a
auditoria de ownership da sequência 2. Nenhum teste RED, código produtivo,
ataque, mutação, veredito ou resselo foi criado/executado. A próxima sequência
depende de confirmação humana explícita do gate P1292; a confirmação de P1291
não é transferida.
