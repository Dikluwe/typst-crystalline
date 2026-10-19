# Relatório — typst-passo-837: `top-edge`/`bottom-edge` inválido aceito em silêncio (#22) e `Length` descartado (#23)

**Origem**: achados #22 e #23 de P831 (lote 5). Prompt: `00_nucleo/materialization/typst-passo-837.md`.

## Proveniência

- **HEAD no arranque**: `2ae3ff53f7e52e73a39acc65fb2a369682b3e197`, árvore limpa (`git status --porcelain` vazio), 2026-07-22 15:57:54 -03.
- **Medições "antes"**: feitas no HEAD acima, árvore limpa, com os binários release já existentes (`lab/typst-original/target/release/typst` = vanilla 0.15.0, jun 29; `target/release/typst` = cristalino pré-P837, jul 22 15:56).
- **Medições "depois"**: working tree **não commitado** com as alterações deste passo (lista exacta em `git diff HEAD --stat`, reproduzida no fim), binário cristalino rebuildado com `cargo build --release` às ~16:20 -03.
- Geometria medida com `pdftotext -bbox` (mesmo método de P831). Fixtures em `temp/p837/`.
- Decisões de implementação abaixo são **do executor** (não do dono), assinaladas como tal.

## Baseline de testes (antes)

```
cargo test -p typst-core   → 4552 passed, 0 failed
cargo test -p typst-infra  →  682 passed, 0 failed
```

---

## Achado #22 — string fora do domínio enumerado aceite em silêncio

### Medição antes

Fixture `#set page(width: 200pt, height: 150pt, margin: 20pt)` + `#set text(top-edge: "middle")` + `Hello world`:

```
vanilla 0.15.0:  error: expected "ascender", "cap-height", "x-height", "baseline", "bounds", or length  (span 2:20, exit 1)
cristalino:      exit 0, silêncio total (caía no default)
```

Sonda alargada (saídas literais do vanilla, todas exit 1; cristalino exit 0 em todas):

| Caso | Vanilla |
|---|---|
| `bottom-edge: "middle"` | `error: expected "baseline", "descender", "bounds", or length` |
| `top-edge: "descender"` | erro (domínio top não inclui `"descender"`) |
| `bottom-edge: "ascender"` | erro (domínio bottom não inclui `"ascender"`) |
| `top-edge: 3` (int) | `... or length, found integer` + hint `a length needs a unit - did you mean 3pt?` |
| `top-edge: 3.5` (float) | `... or length, found float` (sem hint) |
| `top-edge: true` (bool) | `... or length, found boolean` |

### Código identificado

- Cristalino: `01_core/src/engine/eval/rules.rs`, arms `"top-edge"`/`"bottom-edge"` de `eval_set_text` — `if let Value::Str(s)` sem qualquer validação de domínio; outros tipos ignorados.
- Vanilla: `lab/typst-original/crates/typst-library/src/text/mod.rs:1169-1177` (cast `TopEdge`), `:1217-1225` (cast `BottomEdge`), domínios em `TopEdgeMetric`/`BottomEdgeMetric` (`:1180-1248`).

### Diff (substância)

`rules.rs`: novo helper `edge_cast_error(is_top, found, span)` com a mensagem **verbatim** do vanilla (string inválida sem sufixo `found`; outros tipos com `, found {type}` no vocabulário do vanilla — reusa o mapeamento `int→integer`/`str→string`/`bool→boolean` de `expected_length_error`, P816; hint `did you mean {i}pt?` só para `Int`, medido). Novas constantes `TOP_EDGE_METRICS = ["ascender", "cap-height", "x-height", "baseline", "bounds"]` e `BOTTOM_EDGE_METRICS = ["baseline", "descender", "bounds"]`. Os arms passam a `match`: string no domínio → push; `Value::Length` → push (#23); resto → `Err` com span na expressão do valor.

### Medição depois (binário rebuildado)

```
invalid-top:      ...:2:20: error: expected "ascender", "cap-height", "x-height", "baseline", "bounds", or length       exit=1
invalid-bottom:   ...:2:23: error: expected "baseline", "descender", "bounds", or length                                exit=1
invalid-type (3): ...:2:20: error: ... or length, found integer  +  hint: a length needs a unit - did you mean 3pt?     exit=1
descender-top:    ...:2:20: error: expected "ascender", ... or length                                                   exit=1
ascender-bottom:  ...:2:23: error: expected "baseline", "descender", "bounds", or length                                exit=1
t-float:          ... error: ... or length, found float                                                                 exit=1
t-bool:           ... error: ... or length, found boolean                                                               exit=1
```

Mensagens, spans (2:20/2:23) e exit codes batem com o vanilla. Nomes enumerados válidos (`ascender`, `cap-height`, `x-height`, `baseline`, `descender`, `bounds`) continuam a compilar com exit 0 e geometria inalterada (controlo: `valid-enum` 20.000/32.540 = vanilla; `default` 17.404/29.944 = vanilla).

### Nuances

- `"bounds"` é **válido** nos dois domínios do vanilla e passa a ser aceite (antes era silenciosamente default). A resolução real via bbox do glyph (vanilla `TextEdgeBounds::Glyph`, `text/font/mod.rs:261-265`) não existe no contrato `FontMetrics::text_edges` (não recebe glyphs) — cai no fallback defensivo de `edge_offset_pt`. Divergência medida que **permanece**: `top-edge/bottom-edge: "bounds"` → vanilla yMin=17.844, cristalino 17.404 (= default). Limitação registada no L0 (`eval.md` §P837) — implementação de `bounds` real é scope futuro, fora deste passo.
- O hint `did you mean {i}pt?` só existe para `Int` (medido: float e bool não têm hint) — replicado assim.

---

## Achado #23 — `Value::Length` descartado

### Medição antes

`#set text(top-edge: 18pt, bottom-edge: -4pt)`:

```
vanilla:     yMin=28.166000 yMax=40.706000   (P831: 10.12/32.92 noutra página; aqui margem 20pt)
cristalino:  yMin=17.404000 yMax=29.944000   (idêntico ao default — Length ignorado)
```

Sonda alargada (primeira palavra, `pdftotext -bbox`):

| Caso | Vanilla | Cristalino (antes) |
|---|---|---|
| `top-edge: 18pt, bottom-edge: -4pt` | 28.166 / 40.706 | 17.404 / 29.944 |
| `top-edge: 1.5em` (11pt → 16.5pt) | 26.666 / 39.206 | 17.404 / 29.944 |
| `size: 20pt, top-edge: 1em` | 22.120 / 44.920 | 15.280 / 38.080 |
| `bottom-edge: -4pt`, 2 linhas (yMin da 2ª) | 35.792 | 31.792 (= default) |
| `bottom-edge: "descender"`, 2 linhas (2ª) | 34.498 | 34.498 (já batia) |

### Semântica confirmada na fonte vanilla

`lab/typst-original/crates/typst-library/src/text/font/mod.rs:276-289` (`FontInstance::edges`): `TopEdge::Length(l) => l.at(font_size)` (offset absoluto **da baseline**, positivo para cima; `Length::at` resolve a componente `em` no font-size); `BottomEdge::Length(l) => -l.at(font_size)` com a convenção vanilla bottom-positivo-abaixo-da-baseline. Confirmado por medição: `bottom-edge: -4pt` desce a 2ª linha exactamente 4pt (35.792 − 31.792); `top-edge: 18pt` desce a baseline `18 − cap-height`. A implementação replica a **semântica** (resolve `abs + em·size` a partir da baseline), não o número de um caso — coberto pelos casos `pt`, `em` e `em` com outro font-size.

### Código identificado

- `01_core/src/engine/eval/rules.rs`: arms só tratavam `Value::Str`; `Value::Length` caía fora (descartado).
- `01_core/src/entities/style_chain.rs` / `layout_types.rs`: campos `top_edge`/`bottom_edge` eram `Option<EcoString>` — sem representação para Length.
- `03_infra/src/font_metrics.rs:141-190` (`edge_offset_pt`): só aceitava `Option<&str>`.

### Diff (substância)

- **`layout_types.rs`**: novo enum `TextEdge { Metric(EcoString), Length(Length) }` (espelho dos enums `TopEdge`/`BottomEdge` do vanilla); campos `TextStyle::top_edge`/`bottom_edge` passam a `Option<TextEdge>`.
- **`style_chain.rs`**: campos do `StyleDelta` e resolvers `top_edge()`/`bottom_edge()` passam a `Option<TextEdge>`; o canal custom resolve `Value::Str → Metric`, `Value::Length → Length`.
- **`rules.rs`**: braço `Value::Length` nos arms `top-edge`/`bottom-edge` (push no canal custom).
- **`engine/layout/text.rs`**: merge `ns_top_edge`/`ns_bottom_edge` constrói `TextEdge` a partir do canal custom.
- **`engine/layout/metrics.rs`** (`FixedMetrics`): braço `TextEdge::Length` → `Pt(l.resolve_pt(size))` em ambos os edges.
- **`03_infra/src/font_metrics.rs`** (`edge_offset_pt`): assinatura `Option<&TextEdge>`; `TextEdge::Length` → `Pt(l.resolve_pt(size))` directamente — na convenção cristalina (bottom negativo = abaixo da baseline) isto é exactamente a fórmula do vanilla (`top = l.at(size)`, `bottom = -l.at(size)` convertida de sinal). A derivação do sinal foi verificada contra o caminho de métricas existente (`"descender"` dá bottom negativo) e contra a medição de 2 linhas.

### Medição depois (binário rebuildado)

| Caso | Vanilla | Cristalino (depois) | Bate |
|---|---|---|---|
| `top-edge: 18pt, bottom-edge: -4pt` | 28.166 / 40.706 | 28.166 / 40.706 | ✓ |
| `top-edge: 1.5em` | 26.666 / 39.206 | 26.666 / 39.206 | ✓ |
| `size: 20pt, top-edge: 1em` | 22.120 / 44.920 | 22.120 / 44.920 | ✓ |
| `bottom-edge: -4pt`, 2ª linha | 35.792 | 35.792 | ✓ |
| `bottom-edge: "descender"`, 2ª linha | 34.498 | 34.498 | ✓ |
| `top-edge: "ascender", bottom-edge: "descender"` | 20.000 / 32.540 | 20.000 / 32.540 | ✓ (sem regressão) |
| default | 17.404 / 29.944 | 17.404 / 29.944 | ✓ (inalterado) |

### Nuances

- O braço Length é resolvido no font-size do estilo activo em cada chamada de `text_edges` — `em` segue `size`, paridade com `Length::at(font_size)`.
- `"bounds"` (válido) continua no fallback defensivo — ver nuance do #22.

---

## Testes (testes-primeiro)

Escritos antes da implementação; RED confirmado como erro de compilação (E0432, `TextEdge` inexistente) em `typst-core` e `typst-infra`; GREEN após implementação.

Novos testes (+6 core, +1 infra):

- `engine::eval::tests`: `p837_set_text_top_edge_string_invalida_erro`, `p837_set_text_bottom_edge_string_invalida_erro`, `p837_set_text_edges_dominios_cruzados_erro`, `p837_set_text_top_edge_int_erro_com_hint`, `p837_set_text_edges_validos_aceites`.
- `engine::layout::tests`: `p837_text_edges_length_explicito` (FixedMetrics: pt, pt negativo em bottom, em).
- `font_metrics::tests` (L3, face real NimbusSans): `p837_text_edges_length_explicito_face_real` (Length pt/em + controlo de métrica `"baseline"`).

## Contagens antes/depois

```
typst-core:   4552 passed, 0 failed  →  4558 passed, 0 failed   (+6 = os 6 testes novos)
typst-infra:   682 passed, 0 failed  →   683 passed, 0 failed   (+1 = o teste novo)
typst-wiring + typst-shell: 68 passed, 0 failed (sem alteração)
```

## Lint e L0

- L0s actualizados: `00_nucleo/prompts/engine/eval.md` (nova secção §P837), `00_nucleo/prompts/engine/layout.md` (§P762 ponto 3, contrato `text_edges`, `FixedMetrics`).
- `crystalline-lint --fix-hashes .` reescreveu 17 ficheiros (hashes de `eval.md`→`b75345e3`, `layout.md`→`3127a867`).
- **Bug multi-`@prompt` confirmado e corrigido manualmente**: `rules.rs` (2 prompts) ficou com o hash antigo de `eval.md` (`1d46e2c1`) sem o linter o assinalar; hash real recomputado a partir da fonte do linter (`03_infra/prompt_reader.rs:25-45` — SHA256[0..8] do ficheiro sem linhas `Hash do Código:`) = `b75345e3`, escrito manualmente. 2ª entrada (`p792...` = `a2844d48`) já estava correcta.
- `crystalline-lint .` → **exit 0**, zero V5/drift (warnings V7 pré-existentes de prompts órfãos, inalterados).

## Ficheiros tocados

Código: `01_core/src/engine/eval/rules.rs`, `01_core/src/engine/eval/tests.rs`, `01_core/src/engine/layout/metrics.rs`, `01_core/src/engine/layout/text.rs`, `01_core/src/engine/layout/tests.rs`, `01_core/src/entities/layout_types.rs`, `01_core/src/entities/style_chain.rs`, `03_infra/src/font_metrics.rs`.
L0: `00_nucleo/prompts/engine/eval.md`, `00_nucleo/prompts/engine/layout.md`.
Só header `@prompt-hash` (fix-hashes): `eval/{bibliography,control_flow,flow,markup,math,modules,tests}.rs`, `layout/{dynamic,grid,grid_placement,helpers,hyphenation,placement,slicing,sub_frame,tests}.rs` (metrics.rs e tests.rs também têm alterações de código).
Fixtures (não commitar): `temp/p837/`.

## Desvios e limitações a rever antes do commit

1. **`"bounds"` sem resolução real** (limitação, não regressão): aceite no eval (paridade de domínio) mas cai no default; divergência medida 17.404 vs 17.844 do vanilla. Exige estender `FontMetrics::text_edges` com contexto de glyphs — scope futuro, registado no L0 §P837.
2. **Decisão minha (executor)**: tipo único `TextEdge` para top e bottom (o vanilla tem dois enums) — a distinção de domínio vive no eval, não no tipo. Mais simples e suficiente; se o dono preferir dois tipos, é refactor local.
3. **Decisão minha (executor)**: hint `did you mean {i}pt?` replicado só para `Int` (medido: vanilla não emite hint para float/bool).
4. `top-edge: "descender"` era aceite em silêncio antes e usava a métrica; agora é erro — paridade vanilla (medido: vanilla erra). Se algum documento interno usava esta combinação inválida, passa a falhar (comportamento pretendido).
