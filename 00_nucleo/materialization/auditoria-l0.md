# Auditoria L0 — Mapa de cobertura de prompts

Data: 2026-04-12 (criação) | Última actualização: 2026-04-12 (restauro Prioridade 1 parcial)
Método: cruzamento entre `find 00_nucleo/prompts -name "*.md"` e `find 01_core 03_infra -name "*.rs"`

---

## Como ler este documento

**Situação A — Prompt existe:** o ficheiro de código tem semente L0. O hash
pode estar desatualizado (V5), mas a intenção está documentada.

**Situação B — Sem prompt:** código foi escrito sem nucleação. Precisa de
restauro — a IA redige o prompt a partir do código existente, o humano guarda,
depois `crystalline-lint --fix-hashes .`.

**Situação C — Prompt existe mas sem critérios de verificação:** o prompt
existe mas não tem secção `Dado/Quando/Então`. Os testes podem ser sombra da
implementação.

---

## L1 — 01_core/src/entities/

| Ficheiro `.rs` | Prompt L0 | Situação |
|----------------|-----------|----------|
| `args.rs` | `prompts/entities/func.md` (partilhado) | A |
| `ast/code.rs` | desconhecido | verificar |
| `ast/expr.rs` | desconhecido | verificar |
| `ast/markup.rs` | desconhecido | verificar |
| `ast/math.rs` | desconhecido | verificar |
| `content.rs` | `prompts/entities/content.md` | A |
| `counter_state.rs` | `prompts/entities/counter_state.md` | **A — restaurado 2026-04-12** |
| `file_id.rs` | `prompts/core.md` (referência de Passo 1) | verificar se há prompt dedicado |
| `font_book.rs` | `prompts/world-types.md` (parcial) | C — sem critérios dedicados |
| `func.rs` | `prompts/entities/func.md` | A |
| `glyph_variants.rs` | não encontrado | **B — sem prompt** |
| `label.rs` | não encontrado | **B — sem prompt** |
| `layout_types.rs` | `prompts/entities/layout_types.md` | A |
| `math_class.rs` | não encontrado | **B — sem prompt** |
| `math_constants.rs` | não encontrado | **B — sem prompt** |
| `module.rs` | `prompts/entities/module.md` | A |
| `operators.rs` | `prompts/entities/operators.md` | **A — criado 2026-04-12** |
| `package_spec.rs` | não encontrado | **B — sem prompt** |
| `scope.rs` | `prompts/entities/scope.md` | A |
| `source.rs` | `prompts/entities/source.md` | A |
| `source_result.rs` | não encontrado | **B — sem prompt** |
| `span.rs` | `prompts/core.md` (referência de Passo 1) | verificar se há prompt dedicado |
| `style_chain.rs` | não encontrado | **B — sem prompt** |
| `syntax_kind.rs` | `prompts/core.md` (referência de Passo 1) | verificar se há prompt dedicado |
| `syntax_mode.rs` | não encontrado | **B — sem prompt** |
| `syntax_node.rs` | não encontrado | **B — sem prompt** |
| `syntax_set.rs` | não encontrado | **B — sem prompt** |
| `syntax_text.rs` | não encontrado | **B — sem prompt** |
| `value.rs` | `prompts/entities/value.md` | **A — restaurado 2026-04-12** (expandido para Passos 13–25) |
| `world_types.rs` | `prompts/world-types.md` | A |

---

## L1 — 01_core/src/contracts/

| Ficheiro `.rs` | Prompt L0 | Situação |
|----------------|-----------|----------|
| `world.rs` | `prompts/world-types.md` (parcial) | C — sem critérios dedicados para o trait |

---

## L1 — 01_core/src/rules/

| Ficheiro `.rs` | Prompt L0 | Situação |
|----------------|-----------|----------|
| `eval.rs` | `prompts/rules/eval.md` | A |
| `layout.rs` | `prompts/rules/layout.md` | A |
| `lexer/scanner.rs` | `prompts/rules/scanner.md` | A |
| `math/layout.rs` | `prompts/rules/math.md` (parcial?) | verificar |
| `math/symbols.rs` | `prompts/rules/math.md` (parcial?) | verificar |
| `parse.rs` | `prompts/rules/parse.md` | A |
| `scopes.rs` | `prompts/rules/scopes.md` | A |

---

## L1 — 01_core/src/

| Ficheiro `.rs` | Prompt L0 | Situação |
|----------------|-----------|----------|
| `utils.rs` | `prompts/rules/utils.md` | A |

---

## L3 — 03_infra/src/

| Ficheiro `.rs` | Prompt L0 | Situação |
|----------------|-----------|----------|
| `export.rs` | não encontrado | **B — sem prompt** |
| `font_metrics.rs` | não encontrado | **B — sem prompt** |
| `fonts.rs` | `prompts/infra/fonts.md` (referenciado, a criar) | **B — prompt planeado mas não criado** |
| `integration_tests.rs` | ficheiro de testes — isento de V1 | — |
| `layout.rs` | `prompts/infra/layout.md` | A |
| `world.rs` | `prompts/infra/system-world.md` (referenciado) | verificar se existe ficheiro |

---

## Resumo por prioridade de restauro

### Prioridade 1 — Sem prompt e com lógica de domínio (L1)

Estes são os mais críticos: código em L1 sem semente documenta
intenção que só existe na cabeça do autor ou enterrada nos passos de
materialização.

1. ~~`syntax_node.rs`~~ — **✅ restaurado 2026-04-12** (prompt expandido para interface completa)
2. `syntax_text.rs` — provavelmente ligado a `SyntaxNode`; hash sincronizado via fix-hashes
3. ~~`value.rs`~~ — **✅ restaurado 2026-04-12** (expandido Passos 13–25)
4. `style_chain.rs` — prompt `style_chain.md` existe e está atualizado (Passo 30) ✓
5. `source_result.rs` — prompt `source-result.md` existe ✓
6. `scope.rs` / `scopes.rs` — verificar se `prompts/rules/scopes.md` cobre ambos
7. ~~`counter_state.rs`~~ — **✅ restaurado 2026-04-12** (HashMap genérico, contadores planos)
8. ~~`operators.rs`~~ — **✅ criado 2026-04-12**
9. `package_spec.rs` — especificação de pacote
10. `syntax_set.rs`, `syntax_mode.rs` — tipos auxiliares de parse
11. `glyph_variants.rs`, `math_class.rs`, `math_constants.rs`, `label.rs` — math e labels

### Prioridade 2 — Sem prompt em L3

12. `fonts.rs` — o prompt estava planeado mas não foi criado antes do código
13. `export.rs` — exportadores
14. `font_metrics.rs` — métricas de fonte

### Prioridade 3 — Prompts de Passo 1 que precisam de ficheiros dedicados

15. `file_id.rs` — referenciado em `core.md` mas sem prompt próprio
16. `span.rs` — idem
17. `syntax_kind.rs` — idem

---

## Procedimento de restauro (por módulo)

Para cada módulo na Prioridade 1 ou 2:

```
1. Ler o ficheiro .rs existente
2. Ler os passos de materialização que o criaram (se identificáveis)
3. IA redige prompt L0 usando template-prompts.md como estrutura
4. Humano revê e guarda em 00_nucleo/prompts/<camada>/<módulo>.md
5. crystalline-lint --fix-hashes .
6. crystalline-lint . → confirmar que V5 desaparece para esse ficheiro
```

---

## Comandos de verificação após restauro completo

```bash
crystalline-lint .
# Expectativa: apenas V5 (hashes) — zero V1, V2, V4

crystalline-lint --fix-hashes .
crystalline-lint .
# Expectativa: zero violations
```

---

## Histórico de restauro

| Data | Módulo | Acção | Resultado |
|------|--------|-------|----------|
| 2026-04-12 | `counter_state` | Atualização — prompt desatualizado (omitia HashMap genérico, planos, CounterAction) | A |
| 2026-04-12 | `operators` | Criação — prompt ausente (Situação B) | A |
| 2026-04-12 | `syntax-node` | Expansão — prompt incompleto (omitia LinkedNode, pub(crate), NumberingResult) | A |
| 2026-04-12 | `value` | Expansão — prompt desatualizado (apenas 5 variantes, código tem 16) | A |

Próximos: `syntax_text`, `syntax_set`, `syntax_mode`, `package_spec`, `scope`/`scopes`.
```
