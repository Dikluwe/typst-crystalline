# P1299 — rebaseline quadrilateral e seleção causal do P1300

## Veredito

`P1299_PASS_P1300_COHORT_SELECTED`

Coorte única selecionada para o P1300: **`color-global-constructors`**, com os
paths `hsl`, `hsv` e `linear_rgb`. P1299 não altera L0 nem produto e não alega
paridade funcional geral.

## Proveniência

- baseline: `1f082370e59939de7b57992e137a9f74bfb6758f` (`Tekt`), congelado em
  `2026-09-03T18:08:58,106215151-03:00`;
- árvore antes do build: somente
  `?? 00_nucleo/materialization/typst-passo-1299.md`; `git diff HEAD --stat`
  vazio, SHA-256 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`;
- vanilla ratificado: revisão `a51e02804`, `/usr/local/bin/typst`, SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
- cristalino fresco: `/tmp/p1299.k5gKsy/target/release/typst`, SHA-256
  `104bd664494b34b301c91022e3eb4efa8a8eca3278e90f17990dbebdbd08dc1a`,
  `typst 0.15.1 (1f082370)`;
- o build release terminou com exit `0` em `162.04 s`; stdout vazio, SHA-256
  `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`;
  stderr integral com 40.511 bytes, SHA-256
  `17cb87434009cf35323f6b00149bb73bceb9eecec7115cd4cdb312c6a1c5ae58`.

Os hashes dos inputs e as linhas de comando integrais estão no manifesto; cada
execução da matriz preserva argv, exit code, stdout, stderr e duração em
nanosegundos.

## Reenumeração integral

| Perfil | MATCH | EXTRA_BINDING | MISSING_MEMBER | UNVERIFIED_METADATA | UNKNOWN | blocked_by_ancestor |
|---|---:|---:|---:|---:|---:|---:|
| default | 1552 | 45 | 56 | 401 | 0 | 6 |
| html | 1553 | 45 | 100 | 471 | 0 | 6 |

`MISSING_BINDING`, `WRONG_KIND` e erros de proveniência são zero nos dois
inventários. Os seis `blocked_by_ancestor` não foram descartados: entraram no
catálogo P1299 e a execução bilateral os resolveu como `VANILLA_ONLY`, portanto
como `MISSING_LANGUAGE_MEMBER`. Não restou `UNKNOWN`, `invalid_provenance`,
`blocked_by_ancestor` sem observação bilateral nem `EXECUTION_UNKNOWN`.

## Matriz quadrilateral

O catálogo mecanicamente derivado contém 626 probes e 34 pares
extra/rota-canônica. Isso produz 2.504 observações classificadas:

| Perfil | MATCH_VALUE | MATCH_DIAGNOSTIC | CRYSTALLINE_ONLY | VANILLA_ONLY | DIFFERENT_VALUE | DIFFERENT_DIAGNOSTIC | EXECUTION_UNKNOWN |
|---|---:|---:|---:|---:|---:|---:|---:|
| default | 390 | 115 | 45 | 62 | 11 | 3 | 0 |
| html | 461 | 0 | 45 | 106 | 11 | 3 | 0 |
| a11y | 393 | 115 | 45 | 62 | 11 | 0 | 0 |
| html+a11y | 464 | 0 | 45 | 106 | 11 | 0 | 0 |
| **total** | **1708** | **230** | **180** | **336** | **44** | **6** | **0** |

O canário `html` é `MATCH_DIAGNOSTIC` sem a feature e `MATCH_VALUE` nos
perfis com `html`: `EXPECTED_FEATURE_DISABLED`. Os três canários `pdf.*` são
`DIFFERENT_DIAGNOSTIC` quando `a11y-extras` está desligada e `MATCH_VALUE`
quando ligada; a diferença é o span público, não a semântica do gate.

## Ledger semântico

O ledger contém 166 paths, todos com owner, hash declarado, SHA-256 do Prompt
L0, inferência e condição de refutação:

| Classe | Paths |
|---|---:|
| `MISSING_LANGUAGE_MEMBER` | 106 |
| `INTENTIONAL_PRODUCT_EXTENSION` | 42 |
| `WRONG_PUBLIC_KIND_OR_IDENTITY` | 11 |
| `L0_CONTRADICTION` | 3 |
| `DIAGNOSTIC_SPAN_DIVERGENCE` | 3 |
| `EXPECTED_FEATURE_DISABLED` | 1 |
| `UNRESOLVED` | 0 |

Comentários históricos de compatibilidade não foram contados como paridade.
Quando o L0 assume explicitamente a extensão cristalina, ela foi mantida como
`INTENTIONAL_PRODUCT_EXTENSION`; removê-la exigiria reabrir o contrato.

## Medição anterior à seleção

O L0 `00_nucleo/prompts/compiler/stdlib/color.md:22-23` afirma que os oito
constructors do tipo são “as mesmas funções nativas registadas globalmente”. O
cristalino registra `linear_rgb`, `hsl` e `hsv` em
`01_core/src/compiler/eval/mod.rs:1650-1654`. A fonte vanilla ratificada registra
globalmente somente `luma`, `oklab`, `oklch`, `rgb` e `cmyk` em
`lab/typst-original/crates/typst-library/src/lib.rs:397-401`.

A matriz confirma nos quatro perfis:

- `hsl`, `hsv`, `linear_rgb`: `CRYSTALLINE_ONLY`;
- `color.hsl`, `color.hsv`, `color.linear-rgb`: `MATCH_VALUE`, com tipo e repr
  idênticos bilateralmente.

Inferência: os três nomes bare são uma contradição entre L0, produto e alvo
ratificado, enquanto as rotas canônicas já funcionam. Refutação: o vanilla
ratificado aceitar qualquer nome bare, ou qualquer rota `color.*` deixar de ser
`MATCH_VALUE` no mesmo baseline.

## Seleção determinística

Aplicando a prioridade do passo, `L0_CONTRADICTION` com rota canônica funcional
vence aliases, spans, membros ausentes e extensões intencionais. Dentro dessa
classe há uma única coorte: três paths, um owner de registro, um Prompt L0 e
preservação integral das três rotas canônicas. Portanto não há empate.

**Incluídos:** `hsl`, `hsv`, `linear_rgb`.

**Excluídos — extensões intencionais (42):** `accent`, `asset`, `bb`, `bold`,
`cal`, `calc.deg`, `calc.log10`, `calc.rad`, `cancel`, `counter_at`,
`counter_display`, `counter_final`, `counter_step`, `display`, `frak`,
`grid_cell`, `grid_footer`, `grid_header`, `inline`, `italic`, `lof`, `lot`,
`math.registered`, `mono`, `op`, `replace`, `sans`, `scr`, `script`, `serif`,
`sscript`, `state_at`, `state_display`, `state_final`, `state_update`,
`state_update_with`, `sym.registered`, `table_cell`, `table_footer`,
`table_header`, `underover`, `upright`.

**Excluídos — spans diagnósticos (3):** `pdf.data-cell`, `pdf.header-cell`,
`pdf.table-summary`.

**Excluídos — membros ausentes (106):** `angle.deg`, `angle.rad`, `color.spot`,
`color.spot.tint`, `enum.item`, `figure.caption`, `float.from-bytes`,
`float.inf`, `float.nan`, `float.signum`, `float.to-bytes`, `footnote.entry`,
`function.where`, `function.with`, `html.area`, `html.audio`, `html.base`,
`html.blockquote`, `html.canvas`, `html.caption`, `html.colgroup`, `html.data`,
`html.del`, `html.details`, `html.dialog`, `html.embed`, `html.fieldset`,
`html.form`, `html.frame`, `html.hr`, `html.img`, `html.input`, `html.ins`,
`html.label`, `html.link`, `html.map`, `html.meta`, `html.meter`, `html.object`,
`html.optgroup`, `html.option`, `html.output`, `html.progress`, `html.q`,
`html.script`, `html.slot`, `html.source`, `html.style`, `html.table`,
`html.tbody`, `html.td`, `html.textarea`, `html.tfoot`, `html.th`, `html.thead`,
`html.time`, `html.tr`, `html.track`, `json.encode`, `list.item`, `math.abs`,
`math.accent`, `math.bold`, `math.cal`, `math.cases`, `math.display`,
`math.frac`, `math.italic`, `math.limits`, `math.lr`, `math.mat`, `math.mid`,
`math.norm`, `math.overbrace`, `math.overbracket`, `math.overline`,
`math.overparen`, `math.overshell`, `math.primes`, `math.root`, `math.round`,
`math.sans`, `math.scr`, `math.sscript`, `math.stretch`, `math.text`,
`math.underbrace`, `math.underbracket`, `math.underparen`, `math.undershell`,
`math.upright`, `outline.entry`, `outline.entry.body`,
`outline.entry.indented`, `outline.entry.inner`, `outline.entry.page`,
`outline.entry.prefix`, `par.line`, `polygon.regular`, `raw.line`,
`selector.after`, `selector.before`, `terms.item`, `toml.encode`, `version.at`,
`yaml.encode`.

**Excluídos — identidade/repr (11):** `color.map`, `color.map.cividis`,
`color.map.coolwarm`, `color.map.crest`, `color.map.flare`,
`color.map.icefire`, `color.map.mako`, `color.map.rainbow`, `color.map.rocket`,
`color.map.turbo`, `color.map.vlag`.

**Excluído — gate esperado (1):** `html`.

Esses 163 paths excluídos permanecem visíveis no ledger e não são perdoados;
somente não são misturados à coorte causal do P1300.

## Gate obrigatório do P1300

Como ocultar/remover bindings públicos muda contrato e compatibilidade, o P1300
deve, nesta ordem:

1. medir novamente `hsl`, `hsv`, `linear_rgb` e as três rotas `color.*`;
2. atualizar primeiro cada Prompt L0 proprietário afetado;
3. apresentar o diff L0 e **PARAR para confirmação humana** (ADR-0127);
4. somente após a confirmação escrever testes RED e código.

P1299 não antecipa esse gate, não edita L0/produto, não faz staging e não faz
commit.

## Verificação

Os gates finais e seus recibos são pinados no certificado P1299. O veredito PASS
é condicionado a todos eles terminarem com exit `0` e ao diff permanecer restrito
aos artefatos diagnósticos permitidos e ao próprio passo.

> A superfície pública foi reenumerada no baseline P1298, as divergências observadas
> foram classificadas por perfil, owner e natureza de linguagem, e um lote causal foi
> selecionado para um passo posterior; não se alega paridade funcional geral.
