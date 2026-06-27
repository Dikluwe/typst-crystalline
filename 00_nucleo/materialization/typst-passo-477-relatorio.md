# Relatório P477 — `saturate`/`desaturate` + constantes de cor nomeadas

**Data:** 2026-06-27
**Executor:** Claude Sonnet 4.6 (Claude Code)
**Passo:** P477 (Trilha 4 — refino final Color)
**Materialização:** Implementação + testes + atualização de spec L0

---

## 1. Resumo

**Sub-item A — `saturate`/`desaturate`:**
`Color::saturate` e `Color::desaturate` implementados em `entities/color.rs` via componente
`c` (chroma) de Oklch. Helper `to_oklch_p476` reutilizado de P476 sem imports adicionais.
`native_color_saturate` e `native_color_desaturate` expostos no módulo `color` stdlib.
**ADR-0083 §"Operadores cor" TOTALMENTE FECHADO** (6/6: lighten/darken/mix/negate P476 +
saturate/desaturate P477).

**Sub-item B — constantes de cor nomeadas:**
`parse_color` alargada de 5 para 18 cores (13 novas + 2 aliases `gray`/`grey`,
`aqua`/`cyan`). Cobertura CSS basic colors. ADR-0083 §"Constantes nomeadas" parcialmente
revogado.

---

## 2. Sondas pré-implementação (ADR-0108)

### Sub-item A — Sonda operadores saturate/desaturate

| Sonda | Resultado | file:line |
|-------|-----------|-----------|
| `Color::saturate` existe? | **Não** — ausente antes de P477 | `entities/color.rs` (pré-P477) |
| `Color::desaturate` existe? | **Não** | idem |
| `to_oklch_p476` acessível? | **Sim** — helper privado P476 | `entities/color.rs:337+` |
| `native_color_saturate` existe? | **Não** | — |
| `make_color_module()` extensível? | **Sim** — P476, 4 entradas | `rules/stdlib/color.rs:33` |
| `extract_color_arg` disponível? | **Sim** — P476 | `rules/stdlib/color.rs:44` |
| `extract_ratio_arg` disponível? | **Sim** — P476 | `rules/stdlib/color.rs:54` |

**Conclusão:** Caso real. Implementação necessária; helpers P476 reutilizáveis sem
duplicação.

### Sub-item B — Sonda `parse_color`

| Sonda | Resultado | file:line |
|-------|-----------|-----------|
| `parse_color` localização? | `rules/stdlib/shapes.rs` | `shapes.rs:30` |
| Cores actuais antes de P477? | 5 (`red`, `green`, `blue`, `black`, `white`) | `shapes.rs:34–38` |
| `"green"` é `(0,128,0)` ou `(0,255,0)`? | `(0, 128, 0)` — CSS green, não lime | `shapes.rs:35` |
| `"lime"` conflitua com `"green"`? | **Não** — valores diferentes (`(0,255,0)` vs `(0,128,0)`) | idem |
| `parse_color` pub? | `pub(super)` | `shapes.rs:30` |

**Conclusão:** `parse_color` localizada; 5 cores activas; 13 novas sem conflito.

---

## 3. Sub-item A — Implementação `saturate`/`desaturate`

### 3.1 Métodos `impl Color` (P477)

```rust
// entities/color.rs:423–435
pub fn saturate(self, amount: f32) -> Self {
    let (l, c, h, alpha) = to_oklch_p476(self);
    Color::oklch(l, (c + amount).max(0.0), h, alpha)
}

pub fn desaturate(self, amount: f32) -> Self {
    let (l, c, h, alpha) = to_oklch_p476(self);
    Color::oklch(l, (c - amount).max(0.0), h, alpha)
}
```

- `to_oklch_p476` reutilizado (privado P476, mesmo ficheiro — zero import adicional).
- Chroma clampada ao mínimo `0.0`; sem máximo (cores fora de gamut permitidas; exporter
  clamp na conversão).
- `l` e `h` preservados em ambas as operações.

### 3.2 Funções nativas stdlib

`01_core/src/rules/stdlib/color.rs` estendido com 2 funções (linhas 164–207):

```rust
pub(crate) fn native_color_saturate(...)   -> SourceResult<Value>;
pub(crate) fn native_color_desaturate(...) -> SourceResult<Value>;
```

`make_color_module()` actualizado de 4 para 6 entradas:

```rust
dict.insert("saturate",   Value::Func(Func::native("color.saturate",   native_color_saturate)));
dict.insert("desaturate", Value::Func(Func::native("color.desaturate", native_color_desaturate)));
```

Helpers `extract_color_arg` e `extract_ratio_arg` reutilizados sem modificação.

---

## 4. Sub-item B — Constantes de cor nomeadas

`parse_color` em `shapes.rs:30` alargada com 13 novas entradas (linha 39+):

| Nome | RGB (u8) | Notas |
|------|----------|-------|
| `yellow` | 255, 255, 0 | — |
| `cyan` | 0, 255, 255 | — |
| `magenta` | 255, 0, 255 | — |
| `orange` | 255, 165, 0 | — |
| `purple` | 128, 0, 128 | — |
| `gray` / `grey` | 128, 128, 128 | alias duplo BrE/AmE |
| `silver` | 192, 192, 192 | — |
| `maroon` | 128, 0, 0 | — |
| `navy` | 0, 0, 128 | — |
| `olive` | 128, 128, 0 | — |
| `teal` | 0, 128, 128 | — |
| `lime` | 0, 255, 0 | distinto de `green` = `(0,128,0)` |
| `aqua` | 0, 255, 255 | alias de `cyan` |

`"gray" | "grey"` e `"aqua"` são aliases explícitos no `match`; sem nova lógica.

---

## 5. Testes (15 novos)

### L1 — `entities::color::tests` (6 testes, linhas 789–840)

| Teste | Cobertura |
|-------|-----------|
| `p477_saturate_zero_nao_altera_chroma` | `saturate(0.0)` → chroma inalterada |
| `p477_saturate_aumenta_chroma` | `saturate(0.1)` → chroma cresce |
| `p477_desaturate_zero_nao_altera_chroma` | `desaturate(0.0)` → chroma inalterada |
| `p477_desaturate_grande_clampado_a_zero` | `desaturate(1.0)` → c = 0.0 (cinzento) |
| `p477_saturate_preserva_l_e_h` | l e h inalterados após saturate |
| `p477_desaturate_preserva_l_e_h` | l e h inalterados após desaturate |

### L2 — `rules::stdlib::tests` (9 testes, linhas 11719–11796)

| Teste | Cobertura |
|-------|-----------|
| `p477_native_color_saturate_retorna_color` | `color.saturate(red, 0.2)` → `Ok(Color)` |
| `p477_native_color_desaturate_retorna_color` | `color.desaturate(blue, 0.5)` → `Ok(Color)` |
| `p477_color_module_tem_6_entradas` | `make_color_module()` → Dict com 6 entradas |
| `p477_parse_color_yellow` | `parse_color("yellow")` → `Some(rgb(255,255,0))` |
| `p477_parse_color_gray_grey_aliases` | `gray` == `grey` → mesmo `Color` |
| `p477_parse_color_aqua_cyan_aliases` | `aqua` == `cyan` → mesmo `Color` |
| `p477_parse_color_navy_maroon_teal` | 3 novas cores → `Some(...)` cada |
| `p477_parse_color_original_5_sem_regressao` | `red`/`green`/`blue`/`black`/`white` inalterados |
| `p477_parse_color_desconhecida_retorna_none` | `parse_color("unknown")` → `None` |

---

## 6. Arquivos alterados

### Spec L0 (actualizada)

- `00_nucleo/prompts/entities/color.md` — §"P477 — `saturate` e `desaturate`" adicionada;
  ADR-0083 §"Operadores cor" marcado TOTALMENTE FECHADO (6/6); §"Constantes nomeadas"
  actualizado (18 cores). Hash do código: `2701d418`.
- `00_nucleo/prompts/rules/stdlib/color.md` — §"P477 — saturate e desaturate" adicionada;
  `make_color_module()` actualizado para 6 entradas. Hash do código: `bcea91be`.
- `00_nucleo/prompts/rules/stdlib/shapes.md` — `parse_color` documentada com 18 cores
  (5 originais + 13 novas + 2 aliases). Hash do código: `149dbe59`.

### Código L1 (implementado)

- `01_core/src/entities/color.rs`:
  - `impl Color { saturate, desaturate }` (linhas 423–435).
  - 6 testes P477 (linhas 789–840).
  - `@prompt-hash` → `bf7c5345`.
- `01_core/src/rules/stdlib/color.rs`:
  - `native_color_saturate` e `native_color_desaturate` (linhas 164–207).
  - `make_color_module()` actualizado: 4 → 6 entradas.
  - `@prompt-hash` → `1c9dbcc1`.
- `01_core/src/rules/stdlib/shapes.rs`:
  - `parse_color` alargada: 5 → 18 cores (linhas 39–52).
  - `@prompt-hash` → `55d33081`.
- `01_core/src/rules/stdlib/mod.rs`:
  - 9 testes P477 (linhas 11719–11796).
  - Nota: `parse_color` já importada em linha 200; sem import duplicado (erro E0252
    evitado).

---

## 7. Resultados dos testes

### Testes específicos P477 (15 novos)

```
entities::color::tests::p477_saturate_zero_nao_altera_chroma      ok
entities::color::tests::p477_saturate_aumenta_chroma              ok
entities::color::tests::p477_desaturate_zero_nao_altera_chroma    ok
entities::color::tests::p477_desaturate_grande_clampado_a_zero    ok
entities::color::tests::p477_saturate_preserva_l_e_h              ok
entities::color::tests::p477_desaturate_preserva_l_e_h            ok
rules::stdlib::tests::p477_native_color_saturate_retorna_color    ok
rules::stdlib::tests::p477_native_color_desaturate_retorna_color  ok
rules::stdlib::tests::p477_color_module_tem_6_entradas            ok
rules::stdlib::tests::p477_parse_color_yellow                     ok
rules::stdlib::tests::p477_parse_color_gray_grey_aliases          ok
rules::stdlib::tests::p477_parse_color_aqua_cyan_aliases          ok
rules::stdlib::tests::p477_parse_color_navy_maroon_teal           ok
rules::stdlib::tests::p477_parse_color_original_5_sem_regressao   ok
rules::stdlib::tests::p477_parse_color_desconhecida_retorna_none  ok
```

### Suite filtrada

```
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 3405 filtered out
```

Stack overflow em `recursao_infinita_retorna_err_sem_crash` e
`recursao_profunda_retorna_err` é pré-existente (confirmado com `git stash` em P476).

### `crystalline-lint --fix-hashes .`

```
Fixed 3 files:
  ./01_core/src/entities/color.rs               → bf7c5345
  ./01_core/src/rules/stdlib/color.rs           → 1c9dbcc1
  ./01_core/src/rules/stdlib/shapes.rs          → 55d33081

Re-running analysis... ✅ 0 drift warnings remaining
```

`crystalline-lint .` após fix: **0 erros V1–V14**. Apenas warnings V7 pré-existentes
(prompts órfãos não relacionados com P477).

---

## 8. Scope-out explícito

| Área | Scope-out |
|------|-----------|
| **`saturate`/`desaturate` com `space:` arg** | `color.saturate(red, 20%, space: "hsl")` — sem espaço explícito |
| **Chroma máxima definida** | Sem clamp superior; cores fora de gamut tratadas no exporter |
| **`color.space()` runtime** | ADR-0083 §"ColorSpace runtime" preservado scope-out |
| **Constantes Rust estáticas** (`Color::RED`, etc.) | Vanilla expõe assim; cristalino usa `parse_color` string (ADR-0083) |
| **Constantes além de CSS basic** | `DARK_GRAY`, `LIGHT_BLUE`, etc. do vanilla — cobertura CSS basic suficiente P477 |
| **Cores hex em `parse_color`** (`#rrggbb`) | Requer lexer dedicado; scope-out declarado |

---

## 9. Critério de fecho

- [x] Sonda A: `Color::saturate` ausente antes de P477 — confirmado.
- [x] Sonda A: `to_oklch_p476` acessível em `entities/color.rs` — confirmado.
- [x] Sonda B: `parse_color` localizada em `shapes.rs:30`; 5 cores activas — confirmado.
- [x] Sonda B: `"green"` = `(0,128,0)` ≠ `"lime"` = `(0,255,0)` — sem conflito.
- [x] `Color::saturate(self, amount: f32) -> Self` implementado (`color.rs:423`).
- [x] `Color::desaturate(self, amount: f32) -> Self` implementado (`color.rs:429`).
- [x] `native_color_saturate` e `native_color_desaturate` implementados (`stdlib/color.rs:164,187`).
- [x] `make_color_module()` inclui `saturate` e `desaturate` (6 entradas).
- [x] `parse_color` alargado com 13 novas cores + aliases (`shapes.rs:39–52`).
- [x] 6 testes L1 verdes (`entities/color::tests`).
- [x] 9 testes L2 verdes (`rules::stdlib::tests`).
- [x] `entities/color.md` actualizado (ADR-0083 §"Operadores cor" TOTALMENTE FECHADO).
- [x] `rules/stdlib/color.md` actualizado (6 entradas em `make_color_module`).
- [x] `rules/stdlib/shapes.md` actualizado (`parse_color` 18 cores).
- [x] `crystalline-lint --fix-hashes`: 3 ficheiros → hashes `bf7c5345`, `1c9dbcc1`, `55d33081`.
- [x] `crystalline-lint .`: 0 erros V1–V14.
- [x] **ADR-0083 §"Operadores cor" TOTALMENTE FECHADO** (6/6: lighten/darken/mix/negate P476 + saturate/desaturate P477).
- [x] **ADR-0083 §"Constantes nomeadas" parcialmente revogado** (CSS basic colors cobertas).

---

## 10. Estado ADR-0083 pós-P477

| ADR-0083 scope-out | Estado pós-P477 |
|--------------------|-----------------|
| PDF native CMYK | FECHADO (P270.2) |
| Operadores cor | **TOTALMENTE FECHADO** (P476+P477, 6/6) |
| ColorSpace runtime | Preservado scope-out permanente |
| Constantes nomeadas | Parcialmente fechado (CSS basic P477) |

---

## 11. Próximo passo recomendado

| Trilha | Estado pós-P477 |
|--------|-----------------|
| **3** | COMPLETA |
| **4** | **~completa** (ColorSpace runtime = scope-out permanente) |
| **5** | Épico XL declarado (stub confirmado P476) |
| **6** | 4/5 (LoF/LoT page numbers = scope-out longo prazo) |
| **7** | COMPLETA |
| **8** | COMPLETA |

Opções para P478:
- **Footnotes Fase 2** — `Content::Footnote` body não renderizado no rodapé (P295
  materializou só o marker `[N]`; body diferido). S–M.
- **Tiling sonda** — verificar se `Value::Tiling` está activo ou stub. XS.
- **Sonda estado geral** — varrer DEBT.md e roteiro para itens pendentes fora das
  trilhas listadas. XS.
