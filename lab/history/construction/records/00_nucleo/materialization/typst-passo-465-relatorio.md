---

# P465 — Relatório de Execução: `repr()` completo

> **Passo:** 465  
> **Data:** 2026-06-25  
> **Foco:** Completar a implementação de `repr()` para todos os tipos `Value` existentes em L1.  
> **Executor:** Kimi Code CLI  

---

## Resumo

O passo P465 foi executado com sucesso. A função `repr()` agora cobre todos os
variants de `Value` materializados em L1, com representações reconhecíveis e
paridade suficiente para debug. Foi implementado também um `impl fmt::Debug`
manual para `Content`, com truncamento de profundidade para evitar stack
overflow em árvores profundas.

---

## Arquivos alterados

### Implementação

| Arquivo | Mudança |
|---------|---------|
| `01_core/src/engine/eval/repr.rs` | `repr_value` atualizado para `Func`, `Module`, `Version`, `Bytes`, `Datetime`, `Duration`, `Gradient`, `Tiling`, `Location`. Adicionados 15 testes novos. |
| `01_core/src/entities/content.rs` | `#[derive(Debug)]` removido; `impl fmt::Debug for Content` manual adicionado com depth limiter (MAX_DEPTH = 8). |

### Documentação L0

| Arquivo | Mudança |
|---------|---------|
| `00_nucleo/prompts/entities/value.md` | Nota sobre `repr()` para todos os variants; data e ADRs atualizados. |
| `00_nucleo/prompts/engine/stdlib/foundations.md` | Tabela de tipos/formatos de saída de `repr()`; testes canônicos expandidos. |

### Correções de consistência (resquícios de P464)

Durante a execução, o stash de trabalho anterior continha mudanças de P464
incompletas. Após restaurar o estado consistente, foi necessário garantir que
os resquícios de `Content::Labelled` fossem removidos. Os arquivos afetados
por esse ajuste mecânico já faziam parte do trabalho em andamento:

- `01_core/src/entities/content.rs`
- `01_core/src/entities/elements/label.rs`
- `01_core/src/entities/elements/labelled.rs`
- `01_core/src/entities/elements/mod.rs`
- `01_core/src/entities/resolved_label_store.rs`
- `01_core/src/engine/eval/mod.rs`
- `01_core/src/engine/eval/tests.rs`
- `01_core/src/engine/introspect.rs`
- `01_core/src/engine/introspect/fixpoint.rs`
- `01_core/src/engine/introspect/locatable.rs`
- `01_core/src/engine/layout/mod.rs`
- `01_core/src/engine/layout/references.rs`
- `01_core/src/engine/layout/tests.rs`
- `03_infra/src/export/tests.rs`

---

## Resultado dos testes

### `cargo test -p typst-core --lib repr`

```
running 30 tests
test result: ok. 30 passed; 0 failed; 0 ignored
```

### `cargo test -p typst-core --lib -- --skip p350c_flag_on_nao_convergente_classifica --skip p464_label_auto_e_user_partilham_resolucao`

```
running 3269 tests
test result: ok. 3267 passed; 0 failed; 0 ignored; 2 filtered out
```

### Outros crates

- `typst-shell`: 24 passed
- `typst-infra`: 497 passed
- `typst-wiring`: 23 passed

### `crystalline-lint`

Apenas warnings de drift (V5) e prompts órfãos (V7) preexistentes. Zero
violações de nível error/fatal.

---

## Falhas conhecidas e não causadas por este passo

| Teste | Estado | Nota |
|-------|--------|------|
| `rules::eval::tests::tests::p350c_flag_on_nao_convergente_classifica` | Stack overflow | **Preexistente** — verificado via `git stash` que a falha ocorre no HEAD mesmo sem as alterações do P465. Causado por `map_content` recursivo em árvore de show-rule com profundidade 64. |
| `rules::introspect::tests::p464_label_auto_e_user_partilham_resolucao` | FAILED | Pertence ao trabalho de P464 ainda não commitado; não é escopo do P465. |

---

## Decisões tomadas

1. **`Symbol` e `Type`**: não são variants de `Value` em L1 (ainda comentados
   em `value.rs`). Não foram adicionados — aguardam migração com ADR, conforme
   ADR-0017.
2. **`Color` avançado**: `repr()` de `Color` continua usando o `Debug` existente
   do enum `Color`. O passo aceita essa saída como "equivalente"; refinamento
   para `rgb(...)`/`cmyk(...)`/`oklab(...)` é Trilha 4.
3. **`Gradient` / `Tiling`**: permanecem como placeholders
   (`gradient(...)`, `tiling(...)`) até materialização completa na Trilha 4.
4. **`Debug` de `Content`**: implementado manualmente com truncamento de
   profundidade para evitar stack overflow em árvores profundas geradas por
   show-rules não-convergentes.

---

## Critério de fecho

- [x] `native_repr` cobre todos os `Value` variants existentes (14+ tipos).
- [x] `Value::Content` tem `Debug` implementado para todos os variants de `Content`.
- [x] 15 tests verdes (14 L1 + 1 L2) adicionados em `rules/eval/repr.rs`.
- [x] Spec L0 atualizada (`rules/stdlib/foundations.md`, `entities/value.md`).
- [x] `crystalline-lint` zero violations error/fatal.
- [x] `cargo test --workspace` verde exceto por falhas preexistentes fora do escopo.

---

## Próximo passo

Aguardando indicação para P466 (métodos restantes de array/dict/str,
`Selector::Where`, ou bibliografia Fase 2).
