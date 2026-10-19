# Relatório P417 — `Selector::Where`

**Data**: 2026-06-23  
**Passo**: P417 (M) — `Selector::Where`: filtragem de show rules por campos de elemento  
**Executor**: Kimi Code CLI

---

## 1. Sonda do substrato (Fase A.0)

Executados os 8 grep obrigatórios; todos passaram:

| # | Verificação | Resultado |
|---|-------------|-----------|
| 1 | `enum Selector` existe | ✅ `01_core/src/entities/selector.rs:26` |
| 2 | variant de seleção por tipo | ✅ `Kind(ElementKind)` (paridade semântica com `Elem`) |
| 3 | `struct ShowRule` existe | ✅ `01_core/src/entities/show.rs:75` |
| 4 | show rule matching existe | ✅ `rules/eval/rules.rs` / `rules/eval/tests.rs` |
| 5 | field access (P412) | ✅ `Expr::FieldAccess` em AST + eval |
| 6 | element fields acessíveis | ✅ `Element::get_field` / `Content::get_field` |
| 7 | Set/Show no AST | ✅ `SetRule` / `ShowRule` em `entities/ast/` |
| 8 | query infrastructure | ✅ `native_query` + `Introspector::query` |

Nenhuma reclassificação necessária.

---

## 2. Decisões arquiteturais (Fase A.1)

- **Opção β** (ADR-0109): `Selector::Where` como estrutura de dados estática, sem vtable/closure.
- **Dois enums `Selector`** mantidos:
  - `entities/selector.rs` — predicado para `Introspector::query`.
  - `entities/show.rs` — predicado para `ShowRule`.
- **Recursão `Selector ↔ Value`** quebrada com `Box<Value>` no campo `value` de `Where`.
- **Igualdade semântica** (ADR-0107 / ADR-0025): `Value::Int(1)` casa `Value::Float(1.0)` no matching.
- **Atomização forma B**: lógica de matching em free function (`selector_matches`) na camada de render; enum `Selector` permanece fechado.

---

## 3. Implementação (Fase B)

### Entities
- `01_core/src/entities/selector.rs`: +variant `Where { base, field, value: Box<Value> }`; +4 unit tests.
- `01_core/src/entities/show.rs`: +variant `Where { base, field, value: Box<Value> }`.
- `01_core/src/entities/value.rs`: +variant `Value::Selector(Selector)`; `type_name()` → `"selector"`; `From<Selector>`.

### Parser / Eval
- `01_core/src/engine/eval/closures.rs`: intercepta `heading.where(level: 1)` antes do `FuncCall` genérico.
- `01_core/src/engine/eval/bindings.rs`: `eval_element_where()` constrói `Selector::Where` a partir de função nativa de elemento + named arg.
- `01_core/src/engine/eval/rules.rs`:
  - `query_selector_to_show_selector()` converte `Value::Selector` para `show::Selector`.
  - `is_node_rule()` recursivo inclui `Where` com base node-like na travessia de show rules.
  - `selector_matches()` ganha arm `Where` usando `Content::get_field()` + `values_eq_semantic()`.

### Query
- `01_core/src/entities/introspector.rs`: arm `Selector::Where` retorna `vec![]` (stub documentado; scope-out real para query neste passo).
- `01_core/src/engine/stdlib/foundations.rs`: `parse_selector_arg()` aceita `Value::Selector`.

### L0
- `00_nucleo/prompts/entities/selector.md`: atualizado com P417; hash `ce7ef79d`.
- `00_nucleo/prompts/entities/show.md`: criado registro de P417; hash `334ecddd`.
- Hashes atualizados nos cabeçalhos `@prompt-hash` de `selector.rs` e `show.rs`.

---

## 4. Validação (Fase C)

```bash
cargo test -p typst-core --lib -- --skip p350c_flag_on_nao_convergente_classifica
# → 3050 passed; 0 failed; 1 filtered out (teste preexistente com stack overflow)

crystalline-lint .
# → 0 drift nos prompts tocados
# → warnings preexistentes de prompts órfãos (fora do escopo P417)
```

### Tests novos (18)
- 4 `entities::selector::tests::p417_selector_where_*` — struct, igualdade, hash, nested.
- 3 `rules::eval::rules::tests::p417_extract_field_*` — `Content::get_field`.
- 6 `rules::eval::rules::tests::p417_matches_where_*` — matching positivo/negativo/coerção.
- 5 E2E `rules::eval::tests::tests::p417_*` — parse → eval → show rule aplicada/não aplicada.

### Critérios de fecho
- [x] 16+ tests verdes (18)
- [x] Lint zero drift nos L0 tocados
- [x] `heading.where(level: 1)` funciona em `#show`
- [x] Matching semântico (`1 == 1.0`)
- [x] Nenhum vtable/`dyn`/`Box<dyn>` introduzido
- [x] `match` exaustivo preservado
- [x] Lógica atomizada em free functions
- [x] L0 hashado e propagado

---

## 5. Scope-out explícito

- Múltiplos campos em um único `.where(...)` (`heading.where(level: 1, numbering: "1.")`).
- Campos aninhados (`.where(parent.child: 1)`).
- `Selector::Where` em `#query` — arm stub `vec![]`; consumer real fica para passo dedicado.
- `Selector::Where` com base `Or`/`And` ou `DynKind` em show rules.
- Coerção de tipo ampla além de `Int ↔ Float`.

---

## 6. Notas

- Teste `p350c_flag_on_nao_convergente_classifica` falha com stack overflow de forma preexistente (confirmado via `git stash` antes das alterações); excluído da execução completa.
- Nenhum warning novo introduzido pelo P417.
