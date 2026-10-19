---

# P474 — `Selector::Where` + `#show heading.where(level: N)`

> **Passo:** 474
> **Data:** 2026-06-26
> **Foco:** Materializar `Selector::Where` (predicado por campo) e o seu wiring em show-rules, fechando Trilha 3.
> **Trilha:** 3 — Selectors completos em show rules.
> **Tipo:** Materialização / Sonda-first (ADR-0108 obrigatório antes de qualquer código).
> **Tamanho:** M (~40 min).
> **ADR-0117 Cláusula 4:** `Selector::Where` ausente de `entities/show.rs` e de `entities/selector.rs` (verificado). ADR-0041 documenta "AST não expõe `.where`, adiado". Verificar AST antes de propor estrutura.

---

## Contexto

Trilha 3 tem 3 itens no roteiro:

1. Sonda `Selector::Where` — **executada em P467**, resultado: ausente.
2. `#show regex(...)` — **fechado em P393/P473**.
3. `#show elem.where(field: value): ...` — **este passo**.

A ADR-0041 registou "selectores com filtro `.where`: AST não expõe, adiado". O P467 confirmou ausência. Este passo é a materialização.

O vanilla expõe:

```typst
#show heading.where(level: 1): it => upper(it.body)
#show figure.where(kind: image): it => ...
```

A sintaxe `.where(field: value)` é um método de campo-access sobre uma função construtora (`heading`, `figure`) que retorna um `Selector` com predicado. Em cristalino, o AST tem `FieldAccess` e `FuncCall`; a questão central é se `heading.where(level: 1)` é parseado como `FieldAccess("where") + FuncCall(level: 1)` ou se requer tratamento especial.

---

## ADR-0108 — Medir antes de decidir (sondas obrigatórias)

| Pergunta | Verificar em | Status |
|----------|-------------|--------|
| AST tem `FieldAccess` aplicado a `Func`? | `entities/ast/expr.rs` ou equivalente | 🟡 |
| `"where"` é palavra reservada no parser? | `rules/lexer/` ou `rules/parse/` | 🟡 |
| `heading.where(level: 1)` é parseado como `FieldAccess("where")` sobre `heading`? | teste de parse ou grep em `tests/` | 🟡 |
| `eval_field_access` em `eval/mod.rs` existe? | `rules/eval/mod.rs` | 🟡 |
| `Value::Func` tem método `.where`? | `entities/func.rs` | 🟡 |
| `Selector::Where` existe em `entities/show.rs`? | Confirmado ausente (P467) | ❌ |
| `Selector::Where` existe em `entities/selector.rs`? | Confirmado ausente (P467) | ❌ |
| `ElementField` enum ou equivalente existe? | `entities/` | 🟡 |
| `Content::Heading` tem campo `level` acessível? | `entities/elements/heading.rs` | 🟡 |

**Todas as sondas 🟡 devem ser executadas com `grep`/`file:line` antes de escrever código. O resultado das sondas determina a arquitectura.**

---

## Arquitectura esperada (hipótese pré-sonda)

A hipótese, sujeita a confirmação via sonda:

### A — AST `heading.where(level: 1)` como `FieldAccess` + `FuncCall`

Se o parser produz `FuncCall { callee: FieldAccess { target: heading, field: "where" }, args: [level: 1] }`, então o eval de `heading.where(...)` pode ser tratado como método especial de `Func` que retorna `Value::Selector(Selector::Where { ... })`.

### B — Tratamento especial no eval de `FuncCall`

Se `"where"` é palavra reservada (keyword), o parser pode rejeitar `heading.where(...)` como sintaxe inválida. Nesse caso, a alternativa é registar uma função `where_(...)` no scope ou tratar `.where` como syntax sugar no nível do eval.

**A sonda resolve esta bifurcação. Proceder apenas após confirmar qual é o caso real.**

---

## Implementação (condicional aos resultados da sonda)

### Caminho 1 — `FieldAccess` funciona (caso mais provável)

Se `heading.where(level: 1)` é parseado normalmente:

#### 1.1 — `Value::Selector`

Adicionar ao enum `Value`:

```rust
/// **P474** — Selector como valor de runtime. Permite `heading.where(...)`.
Selector(ShowSelector),
```

Onde `ShowSelector` é o tipo de selector de show-rules (distinção de `entities/selector.rs` de query). `type_name()` → `"selector"`.

#### 1.2 — `Selector::Where` em `entities/show.rs`

```rust
/// **P474** — Predicado por campo. Materializa `heading.where(level: 1)`.
Where {
    kind:      ElementKind,                      // tipo do elemento (Heading, Figure, ...)
    field:     EcoString,                        // nome do campo ("level", "kind", ...)
    value:     Value,                            // valor esperado (Int(1), Str("image"), ...)
},
```

#### 1.3 — Método `.where` em `eval_field_access` ou `eval_func_call`

Quando o target de `FieldAccess` é `Value::Func(f)` com `f` sendo função nativa correspondente a um element kind (heading, figure, etc.) e o field é `"where"`:

```rust
// Em eval de FieldAccess ou FuncCall especial:
if field == "where" {
    // Retornar um "partial" que aguarda os args
    return Ok(Value::Func(Func::native_where_builder(kind)));
}
```

O `where_builder` é invocado com os named args e retorna `Value::Selector(Selector::Where { kind, field, value })`.

**Alternativa mais simples:** registar `heading_where` como função separada, mas isso não suporta a sintaxe `.where`.

#### 1.4 — Mapeamento `Func → ElementKind`

Precisa de saber que `native_heading` → `ElementKind::Heading`, `native_figure` → `ElementKind::Figure`, etc. Verificar se existe `Func::element_kind()` ou equivalente.

#### 1.5 — `eval_show_rule` com `Value::Selector`

Em `eval_show_rule`, quando o selector avalia para `Value::Selector(s)`, usar `s` directamente como `ShowRule.selector`.

#### 1.6 — `apply_show_rules` com `Selector::Where`

No arm de `Content::Heading { level, .. }`:

```rust
if let Selector::Where { kind: ElementKind::Heading, field, value } = &rule.selector {
    if field == "level" {
        if let (Value::Int(expected), Some(actual)) = (value, level) {
            if *expected as usize == *actual {
                // match — aplicar transformação
            }
        }
    }
}
```

Para `Content::Figure { kind: Option<String>, .. }`:

```rust
if let Selector::Where { kind: ElementKind::Figure, field, value } = &rule.selector {
    if field == "kind" {
        if let (Value::Str(expected), Some(actual)) = (value, figure_kind) {
            if expected == actual {
                // match
            }
        }
    }
}
```

### Caminho 2 — `"where"` é palavra reservada

Se o parser rejeita `heading.where(...)`:

- Registar `heading.where` como método especial via `MethodDispatch` se existir.
- Ou: registar `where_` como função nativa no scope do módulo `heading` (se existir).
- Ou: scope-out documentado com divergência de sintaxe declarada — usar `heading_where(level: 1)` em cristalino.

**A decisão entre caminhos é determinada pela sonda. Não prosseguir sem confirmação.**

---

## Campos suportados no subset P474

Subset minimal para os casos de uso mais comuns:

| Element | Campo | Tipo | Exemplo |
|---------|-------|------|---------|
| `heading` | `level` | `Int` | `heading.where(level: 1)` |
| `figure` | `kind` | `Str` | `figure.where(kind: "image")` |
| `figure` | `kind` | `Str` | `figure.where(kind: "table")` |

Outros campos e elementos são aceites pelo `Selector::Where` como dados mas retornam `false` no match (sem erro hard) — comportamento graded documentado.

---

## Tests

- **L1:** `Selector::Where { kind: Heading, field: "level", value: Int(1) }` é construído sem erro.
- **L1:** `ShowSelector::Where` != `ShowSelector::NodeKind(Heading)`.
- **L2:** `#show heading.where(level: 1): it => upper(it.body)` aplica apenas a headings de nível 1, não ao nível 2.
- **L2:** `#show heading.where(level: 2): it => [...]` aplica ao nível 2, não ao nível 1.
- **L2:** `#show figure.where(kind: "image"): it => [...]` aplica apenas a figuras de imagem.
- **L2:** Combinação `#show heading: ...` e `#show heading.where(level: 1): ...` — ambas aplicadas independentemente.
- **L2:** `heading.where(level: 1)` avaliado em isolamento retorna `Value::Selector(...)`.
- **L2:** Field desconhecido em `.where(unknown: ...)` — sem erro hard, selector não casa (comportamento graded).

---

## Spec L0

### Novos

- `entities/show-where.md` — `Selector::Where` struct, campos, semântica de match. OU extensão de `entities/show.md`.

### Actualizados

- `entities/show.md` — `Selector::Where` variant, `Value::Selector` se adicionado.
- `rules/eval/rules.md` (ou equivalente) — wiring `.where` em eval de `FieldAccess`/`FuncCall`.
- ADR-0041 — anotação de resolução da dívida "selectores com filtro `.where`".

---

## Scope-out explícito

- **`heading.where(level: 1, outlined: true)`** — múltiplos predicados AND. Subset de um campo neste passo.
- **`figure.where(kind: image)`** sem aspas (vanilla aceita `image` como valor sem aspas para `Kind`) — requer type dispatch adicional. Scope-out; apenas `Str` neste passo.
- **`selector.before(loc)` / `selector.after(loc)`** — combinadores posicionais. Scope-out.
- **`.where` em elementos não-listados** (ex: `raw.where(lang: "rust")`) — aceite no `Selector::Where` mas sem match garantido (campo não verificado no arm).
- **`query(heading.where(level: 1))`** — uso de `Selector::Where` em query de introspecção. Scope-out; apenas show-rules neste passo.
- **`#show: rest => ...`** — catch-all. Scope-out; requer mudança no parser.

---

## Critério de fecho

- [ ] Sondas executadas: parse de `heading.where(...)`, `eval_field_access`, `Func::element_kind`, `ElementField` — todos com `file:line` e conclusão sobre Caminho 1 vs 2.
- [ ] Decisão de arquitectura fixada pós-sonda.
- [ ] `Selector::Where { kind, field, value }` adicionado a `entities/show.rs`.
- [ ] Wiring: `heading.where(level: 1)` em eval retorna `Value::Selector(Selector::Where { ... })` (ou equivalente Caminho 2).
- [ ] `eval_show_rule` mapeia `Value::Selector(s)` → `ShowRule` com selector `s`.
- [ ] `apply_show_rules` verifica `Selector::Where` sobre `Content::Heading` e `Content::Figure`.
- [ ] 8+ testes verdes (1 L1 + 7 L2).
- [ ] Spec L0 actualizada (2–3 ficheiros).
- [ ] ADR-0041 anotada com resolução da dívida `.where`.
- [ ] Divergência de sintaxe declarada se Caminho 2 (escaping de keyword).
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations.
- [ ] **Trilha 3: COMPLETA** (sonda P467 + regex P473 + Where P474).

---

## Próximo passo

Com P474, Trilha 3 fecha. O panorama das trilhas fica:

| Trilha | Estado |
|--------|--------|
| 1 — Numeração | COMPLETA |
| 2 — Referências cruzadas | COMPLETA |
| 3 — Selectors show rules | **COMPLETA pós-P474** |
| 4 — Gradients | COMPLETA |
| 5 — Shaping/rustybuzz | Bloqueada (sonda pendente) |
| 6 — Bibliografia Fase 2 | 4/5 (LoF/LoT page numbers é scope-out longo prazo) |
| 7 — Layout multi-região | COMPLETA |
| 8 — Refinos stdlib | 5/8 (restam sonda pad/corners + decorações) |

Opções para P475:
- **Sonda Trilha 8 restantes** — verificar se `pad`/`corners`/`sides` têm trabalho real pendente após P242/P243/P247 (XS, ~10 min).
- **Sonda Trilha 5** — viabilidade de rustybuzz no `Cargo.toml` (XS, ~10 min).
- **Refino Trilha 6** — `LoF/LoT com page numbers` (ML; requer 2-pass; pode ser scope-out definitivo).
- **Espaços de cor Trilha 4** — `cmyk`, `oklab` user-facing (M; extensão gradients).

---

## Estado pós-P473 (para referência)

| Passo | Descrição | Estado | Trilha |
|-------|-----------|--------|--------|
| P467 | Sonda `Selector::Where` | ✅ FECHADO | 3 |
| P468 | Estilos numéricos `[1]`, `[2]` | ✅ FECHADO | 6 |
| P469 | `Value::Relative` (`Rel<Length>`) | ✅ FECHADO | 8 |
| P470 | Marcadores list/enum + i18n caption | ✅ FECHADO | 8 |
| P471 | `Symbol` + `highlight` + `sub`/`super` size | ✅ FECHADO | 8 |
| P472 | Back-refs + ibid. + LoF/LoT | ✅ FECHADO | 6 |
| P473 | `op. cit.` + `#show regex(...)` | ✅ FECHADO | 6+3 |
| **P474** | `Selector::Where` + `#show heading.where(level: N)` | 🔄 EM PREPARAÇÃO | 3 |

**Trilha 1: COMPLETA.**
**Trilha 2: COMPLETA.**
**Trilha 3: 2/3 completo** (pré-P474).
**Trilha 4: COMPLETA.**
**Trilha 6: 4/5 completo.**
**Trilha 7: COMPLETA.**
**Trilha 8: 5/8 completo.**
**Inventário de débitos: LIMPO.**
