# Relatório P476 — Sonda Trilha 5 (rustybuzz) + Operadores de cor (lighten/darken/mix/negate)

**Data:** 2026-06-27
**Executor:** Claude Sonnet 4.6 (Claude Code)
**Passo:** P476 (Trilha 5 sonda + Trilha 4 extensão Color)
**Materialização:** Sonda + Implementação + testes + atualização de spec L0

---

## 1. Resumo

**Sub-item A — Sonda Trilha 5:**
`rustybuzz` está em `03_infra/Cargo.toml` (ADR-0019) mas **não é usado em nenhum arquivo
`.rs`** da infra. O shaping é um stub que posiciona texto como `EcoString` plana sem kern,
ligatures ou OpenType. Bidi/RTL não existe. Trilha 5 é um épico XL declarado.

**Sub-item B — Operadores de cor:**
`lighten`, `darken`, `mix`, `negate` implementados em `entities/color.rs` + expostos no
módulo `color` do scope via `rules/stdlib/color.rs`. **4/6 scope-outs de ADR-0083
§"Operadores cor" revogados.** Restam `saturate`/`desaturate` (futuro P477).

---

## 2. Sondas pré-implementação (ADR-0108)

### Sub-item A — Sonda Trilha 5

| Sonda | Resultado | file:line |
|-------|-----------|-----------|
| `rustybuzz` em `03_infra/Cargo.toml`? | Sim — `rustybuzz = { workspace = true }` | `03_infra/Cargo.toml:3` |
| `rustybuzz` usado activamente em `03_infra/src/`? | **Não** — zero ocorrências via `grep -rn "rustybuzz::"` | — |
| `FrameItem::Text` usa lista de glifos shaped? | **Não** — campo `text: EcoString` (string plana) | `layout_types.rs:193` |
| `x_advance` per-char no layouter? | Não em `FrameItem::Text`; só em `FrameItem::Glyph` (math) | `layout_types.rs:219` |
| Bidi crate no workspace? | **Não** — zero `bidi` em todos os `Cargo.toml` | — |
| `Dir::RTL` usado no layout de texto? | **Não** — só em `stack(dir:)` como parâmetro de empilhamento | `stdlib/layout.rs:883` |
| Shaping kern/ligatures activo? | **Não** — texto é `EcoString` sequencial sem posicionamento OpenType | `layout_types.rs:193` |

**Conclusão:** rustybuzz é dependência declarada mas inactiva; shaping é stub sequencial.
RTL/bidi ausente. Trilha 5 = épico XL separado (não passo único).

### Sub-item B — Sonda operadores de cor

| Sonda | Resultado | file:line |
|-------|-----------|-----------|
| `Color::lighten` existe? | **Não** — ausente antes de P476 | `entities/color.rs` (pré-P476) |
| `Color::darken` existe? | **Não** | idem |
| `Color::mix` existe? | **Não** | idem |
| `Color::negate` existe? | **Não** | idem |
| `to_oklch_components` em `gradient.rs`? | Sim — privada linha 313 | `entities/gradient.rs:313` |
| `color_to_oklab_with_alpha` em `gradient.rs`? | Sim — `pub` linha 199 | `entities/gradient.rs:199` |
| `Value::Color` existe? | Sim — P257 | `entities/value.rs:65` (via layout_types) |
| `native_color_lighten` existe? | **Não** | — |
| módulo `color` no scope? | **Não** | — |

**Conclusão:** Caso real. Implementação necessária em `color.rs` + novo `stdlib/color.rs`.

---

## 3. Sub-item A — Estado Trilha 5 documentado

**rustybuzz** em `03_infra/Cargo.toml:3`:
```toml
rustybuzz   = { workspace = true }  # ADR-0019 — shaping de texto
```
Uso actual: **zero**. O crate está declarado como future-proof per ADR-0019, mas nenhum
código de produção usa `rustybuzz::UnicodeBuffer`, `rustybuzz::GlyphBuffer` ou
`rustybuzz::shape()`.

**Shaping actual** (`layout_types.rs:193`):
```rust
FrameItem::Text {
    pos:   Point,
    text:  EcoString,   // string plana — sem per-glyph advances
    style: TextStyle,
}
```
Texto é posicionado como string sequencial. `x_advance` per-glifo só existe no arm de
glifos matemáticos (`FrameItem::Glyph`). **Kern pairs, ligatures, script-shaping: ausentes.**

**Implicação para Trilha 5:** materializar shaping real requer épico XL (~8–12h):
1. Pipeline `rustybuzz::UnicodeBuffer` → `rustybuzz::GlyphBuffer` per-run.
2. `FrameItem::Text` → `FrameItem::TextShaped { glyphs: Vec<ShapedGlyph> }` (ADR nova).
3. Exporter PDF: iterar glyphs com `x_advance` real.
4. RTL/bidi: adicionar crate `unicode-bidi`; re-order runs.

**Anotação de roteiro:** Trilha 5 = épico XL; sonda P476 confirma stub; épico dedicado em
passo separado quando priorizado.

---

## 4. Sub-item B — Implementação operadores de cor

### 4.1 Helpers privados em `entities/color.rs`

Helpers duplicados de `gradient.rs` (circular dep impede import — `gradient.rs` importa
`Color` de `color.rs`; import inverso criaria ciclo):

```rust
// P476 — srgb_to_linear_p476, linear_rgb_to_oklab_p476,
//          to_oklab_p476, to_oklch_p476
fn srgb_to_linear_p476(c: f32) -> f32 { ... }
fn linear_rgb_to_oklab_p476(r, g, b) -> (f32, f32, f32) { ... }
fn to_oklab_p476(c: Color) -> (f32, f32, f32, f32) { ... }
fn to_oklch_p476(c: Color) -> (f32, f32, f32, f32) { ... }
```

Localização: `entities/color.rs` — bloco P476 a partir de linha 337.

### 4.2 Métodos `impl Color` (P476)

```rust
impl Color {
    pub fn lighten(self, amount: f32) -> Self {
        let (l, c, h, alpha) = to_oklch_p476(self);
        Color::oklch((l + amount).clamp(0.0, 1.0), c, h, alpha)
    }

    pub fn darken(self, amount: f32) -> Self {
        let (l, c, h, alpha) = to_oklch_p476(self);
        Color::oklch((l - amount).clamp(0.0, 1.0), c, h, alpha)
    }

    pub fn mix(self, other: Self, weight: f32) -> Self {
        // Interpolação linear em Oklab com weight clamped.
        let (l0, a0, b0, alpha0) = to_oklab_p476(self);
        let (l1, a1, b1, alpha1) = to_oklab_p476(other);
        let t = weight.clamp(0.0, 1.0);
        Color::oklab(l0+(l1-l0)*t, a0+(a1-a0)*t, b0+(b1-b0)*t, alpha0+(alpha1-alpha0)*t)
    }

    pub fn negate(self) -> Self {
        let (r, g, b, a) = self.to_rgba_f32();
        Color::srgb_f32(1.0 - r, 1.0 - g, 1.0 - b, a)
    }
}
```

Localização: `entities/color.rs:411–436`.

### 4.3 Módulo `color` stdlib (novo ficheiro)

`01_core/src/rules/stdlib/color.rs` — criado em P476:

```rust
pub fn make_color_module() -> Value {
    // Value::Dict com 4 entradas: lighten, darken, mix, negate.
}
pub(crate) fn native_color_lighten(...) -> SourceResult<Value>;
pub(crate) fn native_color_darken(...)  -> SourceResult<Value>;
pub(crate) fn native_color_mix(...)     -> SourceResult<Value>;
pub(crate) fn native_color_negate(...)  -> SourceResult<Value>;
```

Extracção de `amount`: aceita `Value::Float`, `Value::Int` (/ 100), `Value::Relative`
(parte `rel`; `abs` must be zero).

`color.mix`: `weight:` named opcional (default `0.5`); named desconhecido → Err.

### 4.4 Registo no scope

```rust
// eval/mod.rs:1019 (após sym, calc)
scope.define("color", make_color_module());
```

---

## 5. Testes (15 novos)

### L1 — `entities/color::tests` (9 testes)

| Teste | Cobertura |
|-------|-----------|
| `p476_negate_vermelho_da_ciano` | `srgb(1,0,0).negate()` → `(0,1,1,1)` |
| `p476_negate_preserva_alpha` | alpha = 0.3 preservado após negate |
| `p476_lighten_zero_nao_altera_luminancia` | `lighten(0.0)` → l idêntico ao original |
| `p476_lighten_um_da_branco_oklch` | `lighten(1.0)` → l clamped a 1.0 |
| `p476_darken_zero_nao_altera_luminancia` | `darken(0.0)` → l idêntico ao original |
| `p476_darken_um_da_preto_oklch` | `darken(1.0)` → l clamped a 0.0 |
| `p476_mix_peso_zero_igual_a_self` | `mix(red, blue, 0.0)` → oklab idêntico a red |
| `p476_mix_peso_um_igual_a_other` | `mix(red, blue, 1.0)` → oklab idêntico a blue |
| `p476_mix_meio_esta_entre_red_e_blue` | `mix(red, blue, 0.5)` → l médio |

### L2 — `rules::stdlib::tests` (6 testes)

| Teste | Cobertura |
|-------|-----------|
| `p476_native_color_lighten_retorna_color` | `color.lighten(red, 0.2)` → `Ok(Color)` |
| `p476_native_color_darken_retorna_color` | `color.darken(blue, 0.2)` → `Ok(Color)` |
| `p476_native_color_negate_vermelho_da_ciano` | negate via native → ciano verificado |
| `p476_native_color_mix_dois_positional_sem_weight` | mix sem weight: → Ok (default 0.5) |
| `p476_native_color_mix_com_weight_named` | mix com `weight: 0.25` → Ok |
| `p476_color_module_no_scope_tem_4_entradas` | `make_color_module()` → Dict com 4 entradas |

---

## 6. Arquivos alterados

### Spec L0 (atualizada / criada)

- `00_nucleo/prompts/entities/color.md` — §"Operadores de cor P476" adicionada;
  ADR-0083 §"Operadores cor" actualizado (4/6 fechados; `saturate`/`desaturate` preservados).
  Hash do código: `246dc470`.
- `00_nucleo/prompts/rules/stdlib/color.md` — **NOVO** — módulo `color` stdlib;
  4 funcs nativas; extracção de ratio; critérios de verificação. Hash: `7da27735`.

### Código L1 (implementado)

- `01_core/src/entities/color.rs`:
  - 4 helpers privados P476 (linhas 337–383).
  - `impl Color { lighten, darken, mix, negate }` (linhas 385–436).
  - 9 testes P476 (linhas 641–726).
  - `@prompt-hash` → `123059bb`.
- `01_core/src/rules/stdlib/color.rs` — **NOVO** (~160 LoC):
  - `make_color_module()`, 4 nativas, `extract_color_arg`, `extract_ratio_arg`.
  - `@prompt-hash` → `021bd30d`.
- `01_core/src/rules/stdlib/mod.rs`:
  - `mod color;` adicionado (linha 52).
  - `pub use ... make_color_module;` adicionado (linha 119).
  - 6 testes P476 (linhas 11638–11700).
- `01_core/src/rules/eval/mod.rs`:
  - import `make_color_module` adicionado.
  - `scope.define("color", make_color_module())` (linha 1019).

---

## 7. Resultados dos testes

### Testes específicos P476 (15 novos)

```
entities::color::tests::p476_negate_vermelho_da_ciano             ok
entities::color::tests::p476_negate_preserva_alpha                ok
entities::color::tests::p476_lighten_zero_nao_altera_luminancia   ok
entities::color::tests::p476_lighten_um_da_branco_oklch           ok
entities::color::tests::p476_darken_zero_nao_altera_luminancia    ok
entities::color::tests::p476_darken_um_da_preto_oklch             ok
entities::color::tests::p476_mix_peso_zero_igual_a_self           ok
entities::color::tests::p476_mix_peso_um_igual_a_other            ok
entities::color::tests::p476_mix_meio_esta_entre_red_e_blue       ok
rules::stdlib::tests::p476_native_color_lighten_retorna_color     ok
rules::stdlib::tests::p476_native_color_darken_retorna_color      ok
rules::stdlib::tests::p476_native_color_negate_vermelho_da_ciano  ok
rules::stdlib::tests::p476_native_color_mix_dois_positional_sem_weight ok
rules::stdlib::tests::p476_native_color_mix_com_weight_named      ok
rules::stdlib::tests::p476_color_module_no_scope_tem_4_entradas   ok
```

### Suite filtrada (excluindo testes de stack overflow pré-existentes)

```
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 3390 filtered out
```

Stack overflow em `recursao_infinita_retorna_err_sem_crash` e `recursao_profunda_retorna_err`
é pré-existente (confirmado com `git stash` antes de P476).

### `crystalline-lint --fix-hashes .`

```
Fixed 2 files:
  ./01_core/src/entities/color.rs               → 123059bb
  ./01_core/src/rules/stdlib/color.rs           → 021bd30d

Re-running analysis... ✅ 0 drift warnings remaining
```

`crystalline-lint .` após fix: **0 erros V1–V14**. Apenas warnings V7 pré-existentes
(prompts órfãos não relacionados com P476).

---

## 8. Scope-out explícito

| Área | Scope-out |
|------|-----------|
| **Shaping rustybuzz real** | Trilha 5 épico XL; épico declarado; sem código neste passo |
| **RTL / bidi** | Dependente de shaping; ausente; Trilha 5 épico |
| **`saturate`/`desaturate`** | Oklch chroma; futuro P477 |
| **`color.mix` com N > 2 cores** | Cristalino aceita 2 positional + `weight:`; N-ário scope-out |
| **`color.mix` com `space:` arg** | Futuro P477 |
| **`lighten`/`darken` preservando espaço de cor da entrada** | Cristalino converte sempre via Oklch |
| **`Value::Relative` com abs ≠ 0** | `extract_ratio_arg` aceita só `abs.is_zero()` = percentagem pura |

---

## 9. Critério de fecho

- [x] Sonda A: `rustybuzz` — stub confirmado (`03_infra/Cargo.toml:3`; zero `rustybuzz::` em src).
- [x] Sonda A: RTL/bidi — ausente (`Dir::RTL` só em `stack(dir:)`).
- [x] Estado Trilha 5 documentado (épico XL declarado neste relatório).
- [x] Sonda B: `Color::lighten` ausente antes de P476 — confirmado.
- [x] `Color::lighten`, `darken`, `mix`, `negate` implementados (`entities/color.rs:385–436`).
- [x] `to_oklch_p476` e `to_oklab_p476` acessíveis em `color.rs` (helpers privados P476).
- [x] `native_color_lighten/darken/mix/negate` implementados (`stdlib/color.rs`).
- [x] Módulo `color` registado no scope com 4 entradas (`eval/mod.rs:1019`).
- [x] 15 testes verdes (9 L1 + 6 L2).
- [x] `entities/color.md` actualizado (4 métodos + ADR-0083 actualizado).
- [x] `rules/stdlib/color.md` criado (novo L0).
- [x] `crystalline-lint --fix-hashes`: `color.rs` → `123059bb`; `stdlib/color.rs` → `021bd30d`.
- [x] `crystalline-lint .`: 0 erros V1–V14.
- [x] **ADR-0083 §"Operadores cor" 4/6 revogados** (lighten/darken/mix/negate fechados; restam saturate/desaturate).

---

## 10. Próximo passo recomendado

| Trilha | Estado pós-P476 |
|--------|-----------------|
| **3** | COMPLETA |
| **4** | ~completa (restam `saturate`/`desaturate` como scope-out XS) |
| **5** | Épico XL declarado (stub confirmado P476) |
| **6** | 4/5 (LoF/LoT page numbers = scope-out longo prazo) |
| **7** | COMPLETA |
| **8** | COMPLETA |

Opções para P477:
- **`saturate`/`desaturate`** (Trilha 4 refino, XS, ~15 min) — fecha ADR-0083 §"Operadores cor" totalmente.
- **Constantes de cor nomeadas** (`Color::RED`, `Color::NAVY`, etc.) — ADR-0083 §"Constantes nomeadas", S.
- **Sonda Tiling** — `Value::Tiling` activo ou stub.
