---

# P467 — Relatório de Execução: Sonda `Selector::Where`

> **Passo:** 467  
> **Data:** 2026-06-25  
> **Foco:** Verificar estado real de `Selector::Where` no código e fechar a trilha 3.  
> **Executor:** Kimi Code CLI  

---

## Resumo

A sonda A.0 confirmou que `Selector::Where` **já está implementado e funcional**
em show rules (materializado em P417/P423). O passo foi, portanto, do **Caso A**
(XS): apenas atualização de inventário e specs L0 — nenhuma alteração de código
foi necessária.

---

## Evidência da sonda

```bash
grep -n "Where" 01_core/src/entities/selector.rs
# 56:    Where { base: Box<Selector>, field: EcoString, value: Box<Value> },
# 216-277: tests de Where

grep -rn "Selector::Where" 01_core/src/ | head
# entities/selector.rs, rules/eval/rules.rs, rules/eval/repr.rs, rules/eval/bindings.rs, etc.
```

### Estado encontrado

| Componente | Estado |
|------------|--------|
| `entities/selector.rs` | `Selector::Where { base, field, value }` existe com `Debug`/`Clone`/`PartialEq`/`Hash`. |
| `rules/eval/bindings.rs` | `eval_element_where()` constrói `Selector::Where` a partir de `heading.where(level: 1)`. |
| `rules/eval/rules.rs` | `selector_matches` implementa matching de `Where` via `Content::get_field` + igualdade semântica (`values_eq_semantic`). |
| `rules/eval/rules.rs` | `is_node_rule` reconhece `Where` como node-like. |
| `entities/content.rs` | `Content::get_field` expõe campos de `Heading` (`level`, `body`), `Figure` (`body`), etc. |
| `entities/introspector.rs` | Query arm de `Where` é stub `vec![]` (scope-out documentado em P417). |
| Tests | 18 tests de P417 passam, incluindo show rules E2E. |

---

## Decisão

Caso A aplicável: `Selector::Where` já existe e funciona. Nenhuma alteração de
código necessária.

---

## Arquivos alterados

| Arquivo | Mudança |
|---------|---------|
| `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md` | Linha 111: `#show <selector>.where(...): ...` de `ausente` → `implementado` (Passos 417, 467). |
| `00_nucleo/prompts/entities/selector.md` | Entrada no histórico de revisões confirmando estado funcional em P467. |
| `00_nucleo/prompts/entities/show.md` | Entrada no histórico de revisões confirmando estado funcional em P467. |
| `00_nucleo/prompts/engine/show-regex.md` | Nota de scope-out atualizada: `.where(field:)` é materializado em P417/P467. |

---

## Resultado dos testes

### `cargo test -p typst-core --lib p417`

```
running 18 tests
test result: ok. 18 passed; 0 failed; 0 ignored
```

### `crystalline-lint`

Apenas warnings de drift (V5) e prompts órfãos (V7) preexistentes. Zero
violações de nível error/fatal.

---

## Scope-outs mantidos

- Query arm de `Selector::Where` em `Introspector::query` permanece stub
  (`vec![]`) conforme P417 — consumer real fica para passo dedicado de
  introspecção.
- Nested `where`, operadores de comparação, `and`/`or` dentro de `where` e
  campos computados permanecem scope-out (documentados em P417/P467).

---

## Nota sobre trabalho em progresso

O working directory continha o P466 (métodos de array/dict/str) em execução,
com testes `p466_dict_pairs`, `p466_dict_remove`, `p466_dict_update` ainda
falhando. Esse trabalho **não foi tocado** neste passo e foi temporariamente
afastado para permitir o commit limpo do P467.

---

## Critério de fecho

- [x] Sonda A.0 executada com evidência.
- [x] Estado real de `Selector::Where` documentado: implementado e funcional.
- [x] Inventário de cobertura atualizado.
- [x] Specs L0 atualizadas.
- [x] `cargo test` verde para `Selector::Where` (18 tests P417).
- [x] `crystalline-lint` zero violations error/fatal.
- [x] **Trilha 3: 1/3 completo** (`Selector::Where` fechado).
