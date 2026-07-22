# Relatório — typst-passo-836: parâmetro `variations:` de `#text` (achado #21 de P831)

**Data**: 2026-07-22
**Commit HEAD no arranque**: `7bdc25477` (working tree limpa — `git status --porcelain` vazio)
**Medições "depois"**: feitas com working tree **não commitada**; ficheiros alterados listados em §6.

---

## 1. Achado e medição inicial (antes)

`#text(variations: (wght: 250))` — cristalino rejeitava com
`error: text() argumento nomeado desconhecido: 'variations'` (exit 1);
vanilla 0.15.0 aceita (campo `#[fold] #[ghost]`,
`lab/typst-original/crates/typst-library/src/text/mod.rs:846-850`).

### Sonda (Passo 1) — comandos exactos e saídas literais

Binários: vanilla `lab/typst-original/target/release/typst compile <f> <out.pdf>`;
cristalino `./target/release/typst <f> -o <out.pdf>` (binário pré-passo).
Fixtures novas em `temp/p836/` (+ reuso de `temp/p831/var1..5.typ`).

| Fixture | Conteúdo | Vanilla | Cristalino (antes) |
|---|---|---|---|
| `v1_float.typ` | `wght: 250` / `wght: 800` (Ubuntu VF) | exit 0 | exit 1 `text() argumento nomeado desconhecido: 'variations'` |
| `v2_int.typ` | `wght: 250` (int) | exit 0 | exit 1 (idem) |
| `v3_axes.typ` | `ital: 1`, `opsz: 12`, `wdth: 75` | exit 0 | exit 1 (idem) |
| `e1_tag5.typ` | `(wgght: 1)` | exit 1 — erro + 2 hints (abaixo) | exit 1 (mensagem errada) |
| `e2_tagempty.typ` | `("": 1)` | exit 1 — `found 0 characters` | exit 1 (mensagem errada) |
| `e3_strval.typ` | `(wght: "bold")` | exit 1 — `expected float, found string` | exit 1 (mensagem errada) |
| `e4_nonascii.typ` | `("wg€t": 1)` | exit 1 — ASCII + cluster | exit 1 (mensagem errada) |
| `e5_range.typ` | `(wght: 99999)` | **exit 0** (sem validação de faixa) | exit 1 (mensagem errada) |
| `e6_space.typ` | `("w g": 1)` | exit 1 — `spaces may only appear as padding following a tag` | exit 1 (mensagem errada) |
| `e7_nondict.typ` | `variations: 5` | exit 1 — `expected dictionary, found integer` (sem hint) | exit 1 (mensagem errada) |
| `e8_short_dup.typ` | tag 1 char + chave duplicada | exit 1 — `duplicate key: wght` (do eval de dict) | exit 1 (mensagem errada) |
| `s1_nonvar.typ` | `wght: 300` em Libertinus Serif (não-variável) | exit 0 (no-op silencioso) | exit 1 |
| `s2_setrule.typ` | `#set text(variations: (wght: 250))` | **exit 0** (settable — campo `#[ghost]`) | exit 1 `unexpected argument: variations` |

Saídas literais do vanilla (erros):

```text
error: tag must be one to four characters in length
  = hint: found 5 characters
  = hint: occurred in tag at index 0 (`"wgght"`)

error: expected float, found string
  = hint: occurred in tag at index 0 (`"wght"`)

error: tag may contain only printable ASCII characters
  = hint: found invalid cluster `"€"`
  = hint: occurred in tag at index 0 (`"wg€t"`)

error: spaces may only appear as padding following a tag
  = hint: occurred in tag at index 0 (`"w g"`)

error: expected dictionary, found integer        (sem hint)
```

Fontes vanilla lidas: cast `FontVariations`
(`text/font/variations.rs:217-236`, `tag_hint_helper` em :233-235),
cast `Tag` (`text/font/tag.rs:85-117`, ordem: ASCII → comprimento →
espaços), `AxisValue` (`variations.rs:77-81`, só `f64` — int coage),
fusão `automatic.chain(custom).normalized()`
(`text/font/mod.rs:113-120`, custom vence por tag), `#[fold]`
(`variations.rs:210-214`).

## 2. Código identificado

- Rejeição: `01_core/src/engine/stdlib/text.rs:92-100` (qualquer named
  arg ≠ `fill` era erro hard) e `01_core/src/engine/eval/rules.rs`
  (`VANILLA_TEXT_SET_PROPS` sem `variations`).
- Cadeia de eixos existente: `03_infra/src/font_variant.rs`
  (`text_style_to_font_variant`, `axis_variations_for_font_variant`),
  consumida por `shaper.rs` (2 call sites), `font_metrics.rs` (3 call
  sites), `pipeline.rs` (gate do instancer + chave de fontes) e
  `export/builder.rs` (instanciação VF). Os eixos derivavam **só** de
  `FontVariant` (weight/style) — faltava ligar o novo parâmetro em
  `TextStyle` a esta cadeia, incluindo a **chave** de dedup de fontes
  do export (dois runs com o mesmo `FontVariant` mas `variations:`
  distintas precisam de instâncias embutidas distintas).

## 3. Implementação (diff resumido)

**L1 (`01_core`)**
- `01_core/src/entities/font_variations.rs` (**novo**): tipo
  `FontVariations(Vec<([u8;4], f32)>)` normalizado (ordenado por tag,
  dedup later-wins); `from_value` com validação verbatim do vanilla
  (mensagens + hints, ordem ASCII→comprimento→espaços, nomes de tipo
  vanilla `integer`/`string`/`boolean`); `from_validated_dict`;
  `fold` (interno vence por tag). Tag `[u8;4]` com padding de espaço
  (paridade `Tag::from_bytes_lossy`) — sem `ttf_parser` em L1.
  12 testes unitários.
- `entities/mod.rs`: registo do módulo.
- `entities/layout_types.rs`: `TextStyle.variations:
  Option<FontVariations>`.
- `entities/style_chain.rs`: resolver `StyleChain::variations()` —
  fold por tag sobre todos os níveis do canal custom
  `"text.variations"` (paridade `#[fold]`); `From<&StyleChain>` propaga
  para `TextStyle`. 3 testes.
- `engine/layout/text.rs`: merge `variations:
  layouter.style.variations.clone().or(ns_variations)` com
  `ns_variations = layouter.chain.variations()` (fold, não `custom()`
  top-wins — o campo é `#[fold]` no vanilla).
- `engine/stdlib/text.rs` (`native_text`): named arg `variations:`
  validado por `FontVariations::from_value`; o dict viaja no canal
  custom `"text.variations"` dos `Styles` do body (converge com a set
  rule no resolver — paridade ghost+fold).
- `engine/eval/rules.rs`: `"variations"` em `VANILLA_TEXT_SET_PROPS`
  (é settable no vanilla — medido) + braço no dispatch com a mesma
  validação e `push_custom("text.variations", val)`.

**L3 (`03_infra`)**
- `font_variant.rs`: `axis_variations_for_text_style(style)` e
  `merge_explicit_variations(base, custom)` — explícitos vencem por
  tag (paridade `automatic.chain(custom)`). 4 testes.
- `shaper.rs`, `font_metrics.rs`: call sites com `TextStyle` passam a
  usar `axis_variations_for_text_style` (o `set_variations` existente
  aplica os eixos; eixo ausente na fonte é no-op silencioso, paridade).
- `pipeline.rs`: chave de fontes alargada para
  `(FontList, FontVariant, FontVariations)`; gates do instancer
  (multi-font e single-font→multi-font) usam os eixos fundidos.
- `export/mod.rs`, `export/stream.rs`, `export/builder.rs`: assinaturas
  com a chave tripla; `font_index_for_style` compara também as
  variações; a instanciação (`instantiate_variable_font`) recebe os
  eixos fundidos — a fonte embutida é instanciada nas coordenadas
  pedidas (contornos, não só avanços).
- `export/tests.rs`, `pipeline.rs` (testes): tuplos actualizados para
  a chave tripla (adaptação de interface, sem mudança de lógica).

**L0 (regra de linhagem)**
- Novo: `00_nucleo/prompts/entities/font_variations.md`.
- Secções P836 anexadas a: `entities/mod.md`,
  `entities/layout_types.md`, `entities/style_chain.md`,
  `engine/stdlib/text.md`, `engine/eval.md`, `infra/font_variant.md`,
  `infra/shaper.md`, `infra/font_metrics.md`, `infra/pipeline.md`,
  `infra/export/mod.md`, `infra/export/builder.md`,
  `infra/export/stream.md`.
- `crystalline-lint --fix-hashes .` executado (actualizou headers,
  incluindo 6 ficheiros `engine/eval/*.rs` que partilham `eval.md`).

## 4. Medição depois (binário release reconstruído)

`cargo build --release` → `./target/release/typst` novo. Todas as 12
fixtures + as 5 de P831:

```text
v1_float / v2_int / v3_axes / e5_range / s1_nonvar / s2_setrule: exit 0
e1_tag5:      error: tag must be one to four characters in length
                hint: found 5 characters
                hint: occurred in tag at index 0 (`"wgght"`)        exit 1
e2_tagempty:  idem com `found 0 characters` / (`""`)                exit 1
e3_strval:    error: expected float, found string
                hint: occurred in tag at index 0 (`"wght"`)         exit 1
e4_nonascii:  error: tag may contain only printable ASCII characters
                hint: found invalid cluster `"€"`
                hint: occurred in tag at index 0 (`"wg€t"`)         exit 1
e6_space:     error: spaces may only appear as padding following a tag
                hint: occurred in tag at index 0 (`"w g"`)          exit 1
e7_nondict:   error: expected dictionary, found integer  (sem hint) exit 1
p831/var2,3,4: idem aos e1/e3/e4; var1/var5: exit 0
```

Mensagens e hints **verbatim** do vanilla (texto, ordem dos hints,
backticks/aspas). O formato de apresentação difere (cristalino:
`ficheiro:linha:col: error:` + linhas `hint:`; vanilla: bloco com
snippet) — é o formato global do cristalino, fora do scope do achado.

### Verificação geométrica (efeito real no glifo)

`pdftotext -bbox` em `v1_float` (Ubuntu VF, `wght: 250` vs `wght: 800`):

| Palavra | Vanilla | Cristalino |
|---|---|---|
| `Weight` @250 | xMax−xMin = **63.88** | **63.88** |
| `Test` @250 | 36.98 | 36.98 |
| `Weight` @800 | **68.30** | **68.30** |
| `Test` @800 | 41.04 | 41.04 |

`s2_setrule` (`#set text(variations: (wght: 250))`): larguras
**idênticas** ao vanilla (`Set` 28.80, `Rule` 39.36, `Test` 36.98).

`pdffonts v1_float.cris.pdf`: **2 subfontes embutidas**
(`CrystallineFont1/2`, CID TrueType, subset) — uma por conjunto de
variações (o vanilla embute 3; a 3.ª não foi investigada porque a
geometria medida é idêntica — ver §7).

Render a 150 dpi (`pdftoppm`, medido com PIL na venv `lab/.venv`):
linha `wght: 250` com 216 px de tinta vs linha `wght: 800` com
234 px — os **contornos** alargam (instanciação real), não só os
avanços; razão 234/216 ≈ 68.30/63.88 ✓.

## 5. Testes (disciplina test-first)

Testes escritos **antes** da implementação: os 10 testes de eval
falharam todos com `text() argumento nomeado desconhecido:
'variations'` / `unexpected argument: variations` (saída capturada),
confirmando o achado; depois da implementação passam.

| Suíte | Antes | Depois | Delta |
|---|---|---|---|
| `cargo test -p typst-core` | 4527 passed, 0 failed | **4552 passed, 0 failed** | +25 (12 entidade + 3 chain + 10 eval) |
| `cargo test -p typst-infra` | 678 passed, 0 failed | **682 passed, 0 failed** | +4 (fusão de eixos) |

`cargo build --workspace`: limpo. `crystalline-lint .`: **exit 0**,
zero V5 (6 warnings V7 de prompts órfãos pré-existentes, não
relacionados).

## 6. Proveniência da working tree (medições "depois")

`git status --porcelain` na medição final — modificados:
12 prompts L0 (`00_nucleo/prompts/...`), `01_core/src/entities/
{mod,layout_types,style_chain}.rs`, `01_core/src/engine/{stdlib/text,
layout/text,eval/rules,eval/tests}.rs`, `01_core/src/engine/eval/
{bibliography,control_flow,flow,markup,math,modules}.rs` (só headers
`@prompt-hash` de `eval.md`, via `--fix-hashes`), `03_infra/src/
{shaper,font_metrics,font_variant,pipeline,fallback_fonts}.rs`,
`03_infra/src/export/{mod,builder,stream,tests}.rs`.
Novos (untracked): `01_core/src/entities/font_variations.rs`,
`00_nucleo/prompts/entities/font_variations.md`, fixtures
`temp/p836/`, este relatório.

## 7. Decisões do executor, nuances e limitações

1. **Correcção manual de hashes multi-prompt (decisão minha, não do
   dono).** O `--fix-hashes` comporta-se mal com ficheiros com 2
   `@prompt`: em `fallback_fonts.rs` escreveu o hash de
   `font_metrics.md` na entrada de `shaper.md`. Corrigi manualmente
   para os valores SHA256[0..8] correctos (algoritmo verificado na
   fonte do linter, `tekt-linter/03_infra/prompt_reader.rs:25-50`:
   conteúdo sem linhas `Hash do Código:`). Também actualizei as
   entradas `pipeline.md` em `pipeline.rs` (→ `efc912bb`) e `eval.md`
   em `rules.rs` (→ `1d46e2c1`), que **já estavam stale antes do
   passo** (headers `59f2cf75`/`2f2e3e80` vs hashes em HEAD
   `67ad93bc`/`3532b6fa`) — o V5 aparentemente só verifica o último
   `@prompt` de cada ficheiro, razão pela deriva pré-existente nunca
   ter sido apanhada. Fica a nota para eventual correcção do linter.
2. **Grapheme clusters**: o vanilla reporta o cluster grapheme no hint
   `found invalid cluster`; o cristalino itera `chars()`
   (unicode-segmentation não está na whitelist L1). Idêntico para
   chars simples (caso medido: `€`); pode divergir no hint para
   sequências combinantes (ex. `e`+U+0301). Limitação registada.
3. **Span dos erros**: cristalino aponta para a chamada `text(...)` /
   argumento da set rule; vanilla aponta para a expressão do dict.
   Mensagem e hints são verbatim; a precisão do span fica como nuance.
4. **Sem validação de faixa** (`wght: 99999` compila): paridade
   medida — o clamp é feito pelo `rustybuzz`/fontTools, como no vanilla.
5. **3.ª subfonte do vanilla** em `v1_float`: vanilla embute 3 subsets
   (dois textos com 2 instâncias); cristalino embute exactamente as 2
   usadas. Geometria medida idêntica; a origem da 3.ª não foi
   investigada (fora do critério do achado).
6. **Line breaking e posição vertical**: o vanilla quebra a linha de
   `v1_float` noutro ponto e os `yMin/yMax` diferem — diferenças
   pré-existentes do motor de layout, fora do scope. O observável do
   achado (larguras por palavra por eixo) bate exactamente.
7. **`wght` explícito sobrepõe o derivado de `weight:`** (paridade
   `automatic.chain(custom)`), incluindo `wght: 400` a anular bold —
   coberto por teste unitário (`p836_explicita_sobrepoe_tag_derivada`).
8. **Chaves duplicadas no dict** (`(wght: 300, wght: 500)`): o eval de
   dict do cristalino já rejeita com `duplicate key: wght`, mensagem
   igual à do vanilla — sem trabalho adicional necessário.

## 8. Estado final

- `cargo test -p typst-core`: 4552 passed, 0 failed.
- `cargo test -p typst-infra`: 682 passed, 0 failed.
- `crystalline-lint .`: exit 0, zero violations.
- Sem commits (working tree por commitar pelo dono).
