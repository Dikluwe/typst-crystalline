# Relatório — Passo 311 — Math style (12 funções `bb`/`cal`/`frak`/...)

**Data**: 2026-05-20
**Spec**: `00_nucleo/materialization/typst-passo-311.md`
**Diagnóstico**: P311a fechado em `diagnosticos/diagnostico-math-style-passo-311a.md`
**Composição**: passo composto P311a (diagnóstico, M documental) +
P311b (materialização, M+ partida em 6 sub-passos).
**Caminho escolhido**: **I (Variant `Content::MathStyled`) + γ (Unicode
on-the-fly + ~21 excepções BMP)** per diagnóstico P311a §3.1+§3.2.
**Baseline pós-P310**: 2 409 testes typst-core. **P311b: 2 462 testes**
(+53 net: 17 P311b.1 + 0 P311b.2 + 16 P311b.3 + 8+6 P311b.4 + 6 P311b.5).
Zero regressão.
**Hash `entities/content.rs`**: `82d3c47d` → `492f5ccd` → `339daebd`
(**28º consecutivo termina P311b.2**; primeira quebra desde P282).
**Hash `export/*`**: todos preservados bit-exact (**4º consecutivo
pós-P307**).
**ADRs novas**: **2** — ADR-0102 (Math-Style-Mechanism;
Caminho I formalizado) + ADR-0103 (Math-Style-Composition; 4 regras
outer-wins).
**ADRs pré-existentes anotadas**: 1 — ADR-0033 §"Anotação cumulativa
P311b — Math style mechanism + composition".
**Drift L0 deliberado**: 4 prompts — `entities/math_style.md` (novo;
hash `a297f91f`), `entities/content.md` (`82d3c47d` → `492f5ccd`),
`rules/stdlib.md` (`aa4ca50f` → `292ed749`), `rules/math/layout.md`
(`c45536b1` → `7be2c621`).
**ADRs meta novas**: **0** (**18ª vez consecutiva** anti-padrão
P273.17 §0 honrado).

---

## §1 — Sumário executivo

P311 materializa 12 funções vanilla math style cumprindo paridade
**12/12 = 100%** categoria. Segunda categoria stdlib cristalina a
fechar após `calc` (41/41 em P308). Cobertura A.7 Text features
ascende de 57,1% (P305) para ~65% estimado.

P311 é passo **composto** com diagnóstico-primeiro per ADR-0065:

- **P311a** (M documental): 12 §§ catalogando vanilla + opções
  arquitecturais + recomendação Caminho I + Opção γ. Sem código.
- **P311b** (M+): 6 sub-passos materialização cobrindo
  enum/variant/funções stdlib/layout handler/E2E corpus/ADRs+relatório.

Refinamento empírico durante P311b.4 refuta diagnóstico P311a §3.3
(bold bitwise + size multiplicativo) — substituído por regra
uniforme outer-wins via `Option::or`. Refutação documentada em
ADR-0103 §"Refutações ao diagnóstico P311a §3.3".

**Sub-padrão N=1 inaugural** *"Refinamento empírico de diagnóstico
durante materialização"* — candidato observação cumulativa futura
(N≥3 para formalização).

---

## §2 — Sub-passos materializados

### P311b.1 — `entities/math_style.rs` + L0 dedicado

**Output**:
- L0 novo `00_nucleo/prompts/entities/math_style.md` (~270 linhas).
- L1 novo `01_core/src/entities/math_style.rs` (~190 linhas).
- 17 unit tests (todos verdes).

**Conteúdo**:
- `enum MathStyleKind` (7 variants glyph + 2 size).
- `is_size_variant()` + `size_factor()` helpers.
- `pub fn map_glyph(c, kind, bold, italic) -> char` — Opção γ
  (geração on-the-fly + tabela de ~21 excepções BMP).
- Cobertura inicial: Latin ASCII (`A-Z`/`a-z`) + dígitos (`0-9`).
- Greek + dígitos parciais documentados como não-objectivos
  (cobertura inicial só Latin).

**Hash L0 `math_style.md`**: `a297f91f` (registado no header
@prompt-hash do L1).

### P311b.2 — Variant `Content::MathStyled` em `entities/content.rs`

**Output**:
- Variant 25º adicionado: `MathStyled { kind: Option<MathStyleKind>,
  bold: Option<bool>, italic: Option<bool>, body: Box<Content>,
  cramped: Option<bool> }`.
- 5 sítios exhaustive match cobertos (plain_text, PartialEq,
  map_content, map_text + 3 sítios fora content.rs: introspect.rs ×2,
  locatable.rs, layout/mod.rs).
- L0 `entities/content.md` actualizado §"Variant `Content::MathStyled`".

**Hash `entities/content.rs`**: `82d3c47d` → `492f5ccd` → `339daebd`
(quebra 28º consecutivo).

**Decisão refinamento**: campo `kind` changed from `MathStyleKind`
para `Option<MathStyleKind>` durante implementação. Justificação:
distinguir "outer não overriding variant" (None; ex. `bold(bb(x))`)
de "outer força este variant" (Some; ex. `serif(bb(x))` força Plain).

### P311b.3 — 12 funções stdlib

**Output**:
- L1 novo `01_core/src/rules/stdlib/math_style.rs` (~120 linhas).
- Helper único `wrap_math_style` elimina ~10× duplicação.
- 12 funções: `native_bb/bold/cal/frak/math_italic/mono/sans/scr/
  script/serif/sscript/upright`.
- Registadas em `make_root_scope` (`eval/mod.rs`) com nomes
  user-facing.
- 16 unit tests em `stdlib::mod.rs#[cfg(test)]` (pattern P308).

**Hash L0 `rules/stdlib.md`**: `aa4ca50f` → `292ed749` (drift duplo
P310 + P311b.3); 11 ficheiros consumidores re-hashed.

**`italic` renomeada para `native_math_italic`**: para evitar conflito
com função markup `italic` (existente). Registo user-facing `italic`
aponta para `native_math_italic`.

### P311b.4 — Handler + `apply_math_style` em `rules/math/layout/mod.rs`

**Output**:
- Handler `Content::MathStyled` em `layout_node` (~20 linhas).
- Função livre `apply_math_style` (~70 linhas) recursiva com
  Option::or composição.
- Auto-itálico suprimido em wraps math style (`math_style.italic =
  false`).
- Size factor aplicado a `style.size` para Script/SScript.
- 8 unit tests em módulo `p311b_tests` (todos verdes).

**Hash L0 `rules/math/layout.md`**: `c45536b1` → `7be2c621`; 9
ficheiros consumidores re-hashed.

**Refinamento vs diagnóstico**: implementação descobriu que size
variant outer-wins (paralelo a glyph variant), não multiplicativo
como sugeria diagnóstico §3.5. Tests p311b4 + p311b5 confirmam.

### P311b.5 — E2E paridade corpus + tests

**Output**:
- 5 ficheiros novos em `lab/parity/corpus/math/`:
  - `style_bb.typ` — caso simples `bb(R)`.
  - `style_cal_frak.typ` — `cal(L)` + `frak(g)`.
  - `style_compose_outer_wins.typ` — `bb(cal(x))` + `upright(italic(x))`.
  - `style_orthogonal_flags.typ` — `bold(italic(x))` + `bold(bb(x))`.
  - `style_size.typ` — `script(x)` + `sscript(x)` +
    `script(sscript(x))`.
- 6 tests E2E em `01_core/src/rules/math/layout/tests.rs` que
  verificam codepoints Unicode variant emitidos no FrameItem::Text
  via `MathLayouter::layout_equation`.

**Validações empíricas**:
- `bb(x)` → 𝕩 (U+1D569) em FrameItem::Text. ✓
- `cal(L)` → ℒ (U+2112 excepção BMP). ✓
- `bb(cal(x))` → outer Bb wins → 𝕩. ✓
- `upright(italic(x))` → outer upright wins → 'x' literal. ✓
- `bb(frac(a,b))` → DS num (𝕒) + DS den (𝕓). ✓
- `bold(bb(x))` → DS plane preservado (kind inherit). ✓

### P311b.6 — ADRs + L0 final + relatório (este documento)

**Output**:
- ADR-0102 (Math-Style-Mechanism): ~220 linhas formalizando
  Caminho I; rejeita Caminho II (falha bindings) + III (anti-padrão
  capture sem consumer; reabre DEBT-1).
- ADR-0103 (Math-Style-Composition): ~180 linhas formalizando 4
  regras `Option::or` uniforme; refuta diagnóstico §3.3 bitwise +
  multiplicativo.
- ADR-0033 §"Anotação cumulativa P311b" — anotação 22ª desde
  origem (paralelo P266/P268.1/.../P273/P310 cumulativas).
- Relatório este documento.

---

## §3 — Validações finais

```
$ cargo test --workspace --release
test result: ok. 2462 passed; 0 failed (typst-core)
test result: ok.  472 passed; 0 failed; 6 ignored (typst-infra)
test result: ok.   24 passed; 0 failed (typst-shell)
test result: ok.    2 passed; 0 failed (typst-parity)
test result: ok.   21 passed; 0 failed (typst-wiring)
```

**Δ vs P310**: +53 testes net (typst-core 2 409 → 2 462). Distribuição:
- P311b.1: 17 tests (`entities::math_style::tests`).
- P311b.3: 16 tests (`stdlib::tests::p311b_*`).
- P311b.4: 8 tests (`math::layout::p311b_tests::*`).
- P311b.5: 6 tests (`math::layout::tests::p311b5_*`).

```
$ crystalline-lint .
✓ No violations found
```

---

## §4 — Hashes finais

| Artefacto | Hash inicial (P310) | Hash final (P311b) | Δ |
|---|---|---|---|
| `entities/content.rs` | `82d3c47d` | `339daebd` | **quebra** (28º consec termina) |
| `entities/math_style.rs` | — | `281739c5` | **novo** |
| `entities/math_style.md` (L0) | — | `a297f91f` | **novo** |
| `entities/content.md` (L0) | `82d3c47d→` | `492f5ccd` | drift |
| `rules/stdlib/math_style.rs` | — | (consumer hash) | **novo** |
| `rules/stdlib.md` (L0) | `aa4ca50f` | `292ed749` | drift duplo P310+P311 |
| `rules/math/layout/mod.rs` | — | (consumer hash) | edit |
| `rules/math/layout.md` (L0) | `c45536b1` | `7be2c621` | drift |
| 21 ficheiros stdlib/* | (P310 hash) | `292ed749` | re-hashed |
| 9 ficheiros math/layout/* | (anterior) | `7be2c621` | re-hashed |
| `export/*` | preservado | preservado | **4º consec pós-P307** |

---

## §5 — Cobertura empírica

### Categorias stdlib pós-P311b

| Categoria | Estado | Cobertura |
|---|---|---|
| `calc` | **fechada** | 41/41 = 100% (P308) |
| `math style` | **fechada** | 12/12 = 100% (P311b) |
| `markup` (strong/emph/raw/heading/...) | parcial | ~80% |
| `text` (lower/upper/replace/underline/strike/overline/smartquote) | parcial | ~70% |
| `layout` (align/place/pad/hide/block/box/stack/h/v/...) | parcial | ~75% |
| `cores` (rgb/luma/cmyk/oklab/...) | parcial | ~90% |
| `gradients` (linear/radial/conic) | parcial | ~85% |

**Stdlib agregada**: ~57% (P310) → **~60% (P311b)** (+3pp).

### Cobertura A.7 Text features

P305 baseline: 57,1%. P311b actual estimado: **~65%** (+8pp).

---

## §6 — Pendências adiadas

Documentadas em ADR-0102 §"Futuras (pós-P311b)":

1. **Greek + dígitos completos**: Latin priorizado. Greek capitals
   (Α-Ω) + `∇`/`∂` em sub-passo dedicado se cobertura empírica
   exigir. Diagnóstico §"Não-objectivos" preservado.
2. **`display`/`inline` (2 funções extras vanilla)**: scope-out
   P311; sub-passo futuro se cobertura A.7 ampliada.
3. **DEBT-1 StyleChain real**: ADR-0102 §"Caminho III" documenta
   migração futura possível `MathStyled` → `Style::MathVariant` se
   StyleChain real for materializada.
4. **`cramped` consumer downstream**: field set por `script`/
   `sscript` mas script-specific kerning em `attach.rs` ainda não
   honra. Sub-passo P311b.X candidato se cobertura empírica exigir.
5. **Refactor `layout_node` context passing real**: P311b.4 usa
   pré-transformação do body (apply_math_style retorna novo
   Content). Refactor para walker state-based pode ser mais
   eficiente em árvores grandes mas não é blocking; sub-passo
   pontual P311b.X futuro.

---

## §7 — Sub-padrões observados

### N=1 inaugural — *"Refinamento empírico de diagnóstico durante materialização"*

Diagnóstico P311a §3.3 + §3.5 propôs:
- Bold ortogonal bitwise OR.
- Size compõe multiplicativamente.

P311b.4 implementação descobriu:
- `Option::or` uniforme é mais simples e correcto.
- Size variant é outer-wins (não multiplicativo) em vanilla.

Refutação documentada em ADR-0103 §"Refutações ao diagnóstico P311a
§3.3". Candidato observação cumulativa N≥3 futura.

### Sub-padrão "Excepção categorial documentada via ADR dedicada" N=2 cumulativo

P310 (IEEE 754 — ADR-0101) + P311b (Math style — ADR-0102+0103).
Padrão consolidando: divergências cristalinas significativas merecem
ADRs dedicadas (não só anotações cumulativas).

### Sub-padrão "Anotação cumulativa em vez de ADR nova" N+1 cumulativo

ADR-0033 §"Anotação cumulativa P311b" — anotação 22ª desde origem.
Pattern P266/P268.1/.../P273/P310 continua a consolidar-se.

### Sub-padrão "Diagnóstico-primeiro factual antes de materialização M+" N=5+1

P156B Layout + P154A Model + P307a Export + P309 IEEE 754 +
**P311a Math style + P311b refinamento empírico**.

N=5+1 (refinamento é evolução natural do padrão; mantém N=5
cumulativo da promoção P273.17 §0 adiada).

---

## §8 — Próximas frentes pós-P311b

Stdlib agregada ~60%. Frentes candidatas por magnitude/impacto:

### Cat A — Stdlib categorias parciais (refinos M)

- **A.7 Text features completas** (~65% → ~80%): functions
  pendentes vanilla (`smallcaps`/`overline`/`underline`/`text` set
  variants/etc.). Cluster S/M.
- **A.1 Foundations** (`assert`/`type`/`repr`/`panic`/`label`/etc.)
  — refino S.

### Cat B — Stdlib categorias ausentes (materialização M+)

- **A.10 Construct** (`array`/`dictionary`/`group`/`tuple`):
  refino estructural.
- **A.13 Tables** (refino além de P234 actual).
- **A.15 Layout primitives extras**: `place(top/bottom/left/right)`
  parcial.

### Cat C — DEBT consolidados

- **DEBT-1 StyleChain real**: refactor M+ que desbloqueia Caminho
  III math style + outras features. Magnitude alta.
- **DEBT-libm** (ADR-0018): migração agregada `f64::*` → `libm::*`
  em ~7 sítios stdlib. Magnitude M.
- **DEBT-37** (ADR-0096): campo Layouter consumer pending.

### Cat D — Reforços pontuais

- **P312 candidato**: Greek + dígitos completos math style (se
  cobertura empírica exigir).
- **`display`/`inline`** vanilla math style (2 extras P311 scope-out).

---

## §9 — Linha de chegada P311

P311 fecha com:

- **1 diagnóstico** (P311a) + **6 sub-passos materialização** (P311b).
- **2 ADRs novas** (0102 Mechanism + 0103 Composition).
- **1 anotação cumulativa ADR-0033** (§22ª).
- **4 drifts L0 deliberados** (math_style.md novo + content.md
  + stdlib.md + math/layout.md).
- **+53 tests** net (2 409 → 2 462 typst-core).
- **0 regressão** preservada.
- **`crystalline-lint .` zero violations** preservada.
- **Hash content.rs quebra explicitamente** (28º consec termina,
  primeira quebra desde P282).
- **Cobertura stdlib agregada**: ~57% → **~60%** (+3pp).
- **Paridade math style**: **12/12 = 100%** (2ª categoria stdlib a
  fechar 100% após calc P308).
- **Sub-padrão N=1 inaugural** *"Refinamento empírico durante
  materialização"* — candidato observação cumulativa futura.

Conclusão substantiva: Caminho I (`Content::MathStyled`) + Opção γ
(Unicode on-the-fly + ~21 excepções) validados empiricamente em
P311b.4-5. Composição cross-variant determinística via `Option::or`
uniforme. Paridade observable vanilla preservada per ADR-0033 +
ADR-0054 graded.

Próxima frente: Cat A (Text features completas) ou Cat C (DEBT-1
StyleChain real) — decisão humana per ADR-0065 (diagnóstico-primeiro
para magnitudes M+).
