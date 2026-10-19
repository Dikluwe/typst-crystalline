# Relatório P474 — Sonda de fecho Trilha 3 + Sonda Trilha 8 pad/corners/sides

**Data:** 2026-06-27
**Executor:** Claude Sonnet 4.6 (Claude Code)
**Passo:** P474 (Trilhas 3 + 8 — sonda dupla)
**Materialização:** Sonda confirmatória + atualização de specs L0

---

## 1. Resumo

**Sub-item A — Trilha 3: fecho** (XS, Caso A):
P473 marcou Trilha 3 como 2/3 completo e identificou como pendente o wiring E2E de `#show elem.where(field: value)`. A sonda confirma que o wiring **já estava completo desde P417**: `eval_show_rule` → `query_selector_to_show_selector` → `Selector::Where` em `apply_show_rules` → `selector_matches(Where)`. Nenhum código novo necessário. **Trilha 3: 3/3 fechado.**

**Sub-item B — Trilha 8: sonda pad/corners/sides** (XS, Caso A):
Sonda dos 3 itens pendentes de Trilha 8 para decidir se há trabalho real. Resultado: `pad()` com `Sides` (left/right/top/bottom/x/y/rest) está implementado desde P156L com 14 testes; `block`/`box` com `Corners<Length>` via `radius:` estão implementados desde P242. **Trilha 8: 6/8 completo.** Nenhum código novo necessário.

---

## 2. Sondas pré-implementação (ADR-0108)

### Sub-item A — Trilha 3

| Sonda | Resultado | file:line |
|-------|-----------|-----------|
| `eval_show_rule` trata `Value::Selector`? | Sim — P417 | `eval/rules.rs:1047` |
| `query_selector_to_show_selector` converte `Where`? | Sim — P417 | `eval/rules.rs:69–88` |
| `is_node_rule` reconhece `Selector::Where`? | Sim — P417 | `eval/rules.rs:250` |
| `apply_show_rules` aplica `Where` via `selector_matches`? | Sim — P417 | `eval/rules.rs:340` |
| `selector_matches(Where)` lê campo via `Content::get_field`? | Sim — P417 | `eval/rules.rs:218–224` |
| `eval_element_where` intercepta `heading.where(...)`? | Sim — P417 | `eval/closures.rs:242–253` |
| Testes E2E `p417_show_rule_where_*` passam? | Sim — 6 testes E2E | `eval/tests.rs:4495–4570` |
| Total testes P417 passam? | Sim — 18 testes | `eval/rules.rs:1409–` + `eval/tests.rs` |

**Conclusão A:** Wiring completo desde P417. Nenhuma alteração de código necessária. Trilha 3: 3/3 fechado.

### Sub-item B — Trilha 8 pad/corners/sides

| Sonda | Resultado | file:line |
|-------|-----------|-----------|
| `native_pad` implementado? | Sim — P156C/L | `stdlib/layout.rs:366` |
| `pad(left/right/top/bottom/x/y/rest)` suportado? | Sim — P156L | `stdlib/layout.rs:396–444` |
| Precedência específico > eixo > rest? | Sim — P156L | `stdlib/layout.rs:426–429` |
| Testes `native_pad_*` passam? | Sim — 14 testes | `stdlib/mod.rs:4658–` |
| `extract_corners_length_value` implementado? | Sim — P242 | `stdlib/layout.rs:465–521` |
| `block(radius:)` suportado com `Corners<Length>`? | Sim — P242 | `stdlib/layout.rs:763–767` |
| `box(radius:)` suportado com `Corners<Length>`? | Sim — P242 | `stdlib/layout.rs:1024–1028` |
| Testes block/box com padding passam? | Sim | `stdlib/mod.rs:5550, 5583, 5785, 6138–6345` |

**Conclusão B:** `pad`/`corners`/`sides` completos desde P156L/P242. Trilha 8: 6/8 completo.

---

## 3. Sub-item A — Pipeline E2E de `#show elem.where(field: value)`

O pipeline completo para `#show heading.where(level: 1): it => ...`:

```
AST MethodCall(heading.where, level: 1)
  → closures.rs:242  — intercepção `.where` em eval_func_call
  → bindings.rs:140  — eval_element_where() → Selector::Where { Kind(Heading), "level", Int(1) }
  → closures.rs:251  — Ok(Value::Selector(sel))

eval_show_rule (rules.rs:1047)
  → Value::Selector(sel) → query_selector_to_show_selector
  → rules.rs:69          — QuerySelector::Where { Kind(Heading), ... }
                           → show::Selector::Where { NodeKind(Heading), "level", Int(1) }

apply_show_rules (rules.rs:265)
  → has_node_rules = true (is_node_rule(Where { NodeKind }) → true)
  → travessia de nós → selector_matches(Work, Where)
  → rules.rs:218  — selector_matches(base=NodeKind) && get_field("level") == Int(1)
  → recipe aplicado ✓
```

### Invariantes confirmadas

| Condição | Comportamento |
|----------|--------------|
| `heading.where(level: 1)` sobre heading nível 1 | recipe aplicado |
| `heading.where(level: 1)` sobre heading nível 2 | recipe não aplicado |
| `heading.where(level: 1)` sobre paragraph | recipe não aplicado |
| `heading.where(level: 9)` (level clamped para 6) | recipe não aplicado |
| `figure.where(kind: "image")` | suportado (P417) |
| `.where()` em `strong`/`emph`/`raw` | erro explícito (não suportado) |

---

## 4. Sub-item B — Estado de `pad`/`corners`/`sides`

### `native_pad` com `Sides` (P156L)

```
pad(rest: 5pt)   → Sides { left: Some(5pt), right: Some(5pt), top: Some(5pt), bottom: Some(5pt) }
pad(x: 3pt)      → Sides { left: Some(3pt), right: Some(3pt), top: None, bottom: None }
pad(left: 2pt, x: 5pt) → Sides { left: Some(2pt), right: Some(5pt), ... }   — específico > eixo
```

Precedência completa: `específico > eixo (x/y) > rest`. Negativos rejeitados.

### `block`/`box` com `Corners<Length>` (P242)

`radius: Length` → `extract_corners_length_value` → `Corners::uniform(len)`.
`radius: (top-left: L1, top-right: L2, ...)` via dict → `Corners::new(tl, tr, br, bl)`.

### Itens Trilha 8 confirmados nesta sonda

| Item | Passo | Estado |
|------|-------|--------|
| `pad` com Sides | P156L | ✓ implementado + 14 testes |
| `block`/`box` com Corners | P242 | ✓ implementado |
| `Sides` como tipo de domínio | P156 | ✓ implementado |

**Trilha 8: 6/8** — itens restantes (2): ainda por sondar/implementar.

---

## 5. Arquivos alterados

### Testes (adicionados)

- `01_core/src/engine/eval/tests.rs` — 2 testes P474 confirmatórios:
  - `p474_show_where_wiring_completo_heading_level_2`
  - `p474_pad_rest_aplica_uniforme_via_extract_sides`

### Specs L0 (actualizadas)

- `00_nucleo/prompts/engine/show-regex.md` — estatuto actualizado: Trilha 3: 3/3 completo (fechado); P474 referenciado.
- `00_nucleo/prompts/entities/show.md` — entrada no histórico de revisões: P474 fecho Trilha 3 + hash corrigido (V5 pré-existente de P473: `entities/show.rs` → `40e2d8ab`).

---

## 6. Resultados dos testes

### Testes específicos P474 (2 novos)

```
rules::eval::tests::tests::p474_show_where_wiring_completo_heading_level_2   ok
rules::eval::tests::tests::p474_pad_rest_aplica_uniforme_via_extract_sides   ok
```

### Suite completa

```
test result: ok. 3380 passed; 0 failed; 4 filtered (stack-overflow pré-existentes)
```

### `cargo build --workspace`

```
Finished `dev` profile — 0 errors
```

### `crystalline-lint --fix-hashes .`

```
Fixed 1 file:
  ./01_core/src/entities/show.rs → 40e2d8ab
Re-running analysis... ✅ 0 drift warnings remaining
```

V5 pré-existente de P473 (`entities/show.rs` modificado por `previously_cited_keys`) resolvido neste passo.

---

## 7. Scope-out explícito

| Área | Scope-out |
|------|-----------|
| **Multi-field `.where()`** | P417 suporta apenas 1 campo por restrição explícita ("suporta apenas um campo em P417"). Multi-field requer revisão de ADR. |
| **`.where()` em list/enum/raw/strong/emph** | Apenas `heading` e `figure` suportados. Outros retornam erro ou `Ok(None)`. |
| **Nested `.where()` / comparadores** | `where(level: > 1)` não suportado; apenas igualdade semântica. |
| **Introspector::query arm de `Where`** | Continua stub (`vec![]`) — conforme P417. |
| **Trilha 8 itens 7/8 e 8/8** | Dois itens ainda por identificar e sondar. |
| **`pad` com `Value::Relative`** | `Rel<Length>` não ligado a `extract_sides_lengths`. Scope-out P156L. |

---

## 8. Critério de fecho

- [x] Sondas com `file:line` para todos os pontos (Sub-item A: 8 sondas; Sub-item B: 8 sondas).
- [x] Pipeline E2E de `#show heading.where(level: N)` documentado: 5 passos com file:line.
- [x] Invariantes de matching documentadas (6 condições).
- [x] Estado de `pad`/`corners`/`sides` documentado e confirmado.
- [x] 2 testes P474 verdes (heading.where level 2 + pad rest).
- [x] `show-regex.md` L0 actualizado: Trilha 3: 3/3 completo (fechado).
- [x] `entities/show.md` L0 actualizado + V5 drift resolvido (`show.rs` → `40e2d8ab`).
- [x] `cargo build --workspace` verde; `crystalline-lint --fix-hashes` zero drift.
- [x] **Trilha 3: 3/3 completo (fechado)** — `Selector::Where` P467 + `#show regex(...)` P393/P473 + `#show elem.where(...)` P417/P474.
- [x] **Trilha 8: 6/8 completo** — pad/corners/sides confirmados (P156L/P242).

---

## 9. Próximo passo recomendado

**Trilha 8** — itens 7/8 e 8/8: sondar o que resta. Candidatos prováveis:
- `Value::Relative` conectado a `extract_sides_lengths` (actualmente scope-out de P156L)
- `inset`/`outset` em `block`/`box` com `Sides` (actualmente aceita apenas `Length` uniforme)
- Multi-field `.where()` (revisão de ADR necessária antes de implementar)

**Trilha 6** — restante: `LoF/LoT com page numbers` (requer 2-pass convergente — DEBT longo prazo).

**Trilha 4** — `Value::Gradient` tipo real (M, ~35 min) — ainda não iniciado.

**Trilha 7** — `columns`/`colbreak` (L, ~60+ min) — ainda não iniciado.

Opções para P475:
- **Trilha 8** — sonda itens restantes 7/8 e 8/8 (identificar o que falta).
- **Trilha 4** — `Value::Gradient` tipo real.
- **Trilha 7** — `columns`/`colbreak`.
