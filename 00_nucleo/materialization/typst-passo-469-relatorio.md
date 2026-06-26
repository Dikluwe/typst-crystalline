---

# P469 — Relatório de Execução: `Value::Relative` (`Rel<Length>`)

> **Passo:** 469  
> **Data:** 2026-06-25  
> **Foco:** Materializar `Rel<Length>` e integrá-lo em `Value` para expressões como `50%`, `100% - 1em`, `50% + 2cm`.  
> **Executor:** Kimi Code CLI  

---

## Resumo

O passo P469 foi executado com sucesso. Foi criada a entidade genérica
`Rel<T>` (instanciada para `Length`), adicionado o variant `Value::Relative`,
e implementada a aritmética, `repr`, cast `NeedsContext` e eval do literal
percentual. A resolução em contexto de layout foi deliberadamente deixada para
consumers da Trilha 7.

---

## Arquivos alterados

### Implementação

| Arquivo | Mudança |
|---------|---------|
| `01_core/src/entities/rel.rs` | Novo — `Rel<T>` genérico e instanciação para `Length` (`zero`, `from_percent`, `resolve`, operações aritméticas). |
| `01_core/src/entities/layout_types.rs` | `Sub`, `Mul<f64>`, `Div<f64>` para `Abs`; `Sub`, `Mul<f64>`, `Div<f64>` e construtores `cm`/`mm`/`inches` para `Length`. |
| `01_core/src/entities/value.rs` | Novo variant `Value::Relative(Rel<Length>)`, `type_name()` = `"relative length"`, `From<Rel<Length>>`. |
| `01_core/src/entities/mod.rs` | `pub mod rel;` adicionado. |
| `01_core/src/rules/eval/mod.rs` | `Unit::Percent` agora produz `Value::Relative`; módulo `cast` reexportado. |
| `01_core/src/rules/eval/operators.rs` | Braços aritméticos para `Value::Relative` (`+`, `-`, `*`, `/` com `Relative`/`Length`/`Int`/`Float`) e negação unária. |
| `01_core/src/rules/eval/repr.rs` | Representação de `Value::Relative` (`50%`, `50% + ...`). |
| `01_core/src/rules/eval/cast.rs` | Novo — `CastError` e `cast_length`, retornando `NeedsContext` para `Value::Relative`. |
| `01_core/src/rules/eval/tests.rs` | 6 testes E2E/binários para `Relative`. |

### Documentação L0

| Arquivo | Mudança |
|---------|---------|
| `00_nucleo/prompts/entities/rel.md` | Novo — especificação de `Rel<T>` e `Rel<Length>`. |
| `00_nucleo/prompts/entities/value.md` | Atualizado com `Value::Relative`, `type_name`, `From`, critérios e histórico. |
| `00_nucleo/prompts/rules/eval/ops.md` | Novo — operadores e eval percentual para `Relative`. |

---

## Resultado dos testes

### `cargo test -p typst-core rel`

```
running 31 tests
test result: ok. 31 passed; 0 failed; 0 ignored
```

(Inclui 5 testes unitários em `entities::rel`, 2 testes de `repr`, 3 testes de
cast, 6 testes E2E/binários do P469, e testes preexistentes que coincidem com
filtro `rel`.)

### `cargo test --workspace`

- `typst-core`: 496 passed; falhas preexistentes fora do escopo (stack overflow
  em testes de recursão e snapshot `p307b_09_cidfont`).
- `crystalline-lint` (`cargo test -p typst-wiring --test crystalline_lint`):

```
running 2 tests
test result: ok. 2 passed; 0 failed; 0 ignored
```

---

## Falhas conhecidas e não causadas por este passo

| Teste | Estado | Nota |
|-------|--------|------|
| `rules::eval::tests::tests::recursao_infinita_retorna_err_sem_crash` | Stack overflow | Preexistente no HEAD de P466; `map_content` recursivo excede stack em árvores profundas. |
| `rules::eval::tests::tests::recursao_profunda_retorna_err` | Stack overflow | Preexistente. |
| `rules::eval::tests::tests::p350c_flag_on_nao_convergente_classifica` | Stack overflow | Preexistente. |
| `p307b_snapshot_tests::p307b_snapshot::p307b_09_cidfont` | Snapshot mismatch | Preexistente em `typst-infra`; não relacionado a P469. |

---

## Decisões tomadas

1. **`Unit::Percent` → `Value::Relative`**: no subset actual, o literal `50%`
   materializa diretamente um comprimento relativo. `Value::Ratio` permanece
   como tipo L1, mas deixa de ser produzido por literais percentuais.
2. **`Length::cm`/`mm` usam fatores do parser** (`28.346` / `2.8346`) para
   garantir paridade estrutural entre `2cm` parseado e `Length::cm(2.0)`.
3. **Cast `Relative → Length`**: no eval puro retorna `CastError::NeedsContext`.
   Consumers de layout (Trilha 7) recebem `Rel<Length>` e resolvem com contexto.
4. **Scope-out respeitado**: `Fr`, resolução em layout, comparação `<`/`>` de
   `Relative`, e `Rel<Abs>` não foram implementados.

---

## Critério de fecho

- [x] `Rel<T>` implementado em `entities/rel.rs` (genérico, instanciado para `Length`).
- [x] `Value::Relative(Rel<Length>)` adicionado ao enum `Value`.
- [x] Eval de `%` como relativo (`Unit::Percent` → `Value::Relative`).
- [x] Operações aritméticas `+`, `-`, `*`, `/` com `Relative`.
- [x] `repr` de `Relative` implementado.
- [x] Cast de `Relative` para `Length` retorna `NeedsContext` sem contexto.
- [x] 8+ tests verdes para `Relative`.
- [x] Spec L0 atualizada (`entities/rel.md`, `entities/value.md`, `rules/eval/ops.md`).
- [x] `crystalline-lint` zero violações error/fatal.
- [x] `cargo test --workspace` verde exceto por falhas preexistentes fora do escopo.

---

## Próximo passo

Aguardando indicação para P470 (pad/corners/sides, parâmetros decorações,
`Symbol`, back-references, ou regex split).
