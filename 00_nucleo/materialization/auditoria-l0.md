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
| `ast/code.rs` | `prompts/entities/ast/code.md` | **✅ restaurado 2026-04-12** |
| `ast/expr.rs` | `prompts/entities/ast/expr.md` | **✅ restaurado 2026-04-12** |
| `ast/markup.rs` | `prompts/entities/ast/markup.md` | **✅ restaurado 2026-04-12** |
| `ast/math.rs` | `prompts/entities/ast/math.md` | **✅ restaurado 2026-04-12** |
| `content.rs` | `prompts/entities/content.md` | A |
| `counter_state.rs` | `prompts/entities/counter_state.md` | **✅ restaurado 2026-04-12** |
| `file_id.rs` | `prompts/entities/file-id.md` | **✅ restaurado 2026-04-12** (dedicado) |
| `font_book.rs` | `prompts/world-types.md` (parcial) | C — sem critérios dedicados |
| `func.rs` | `prompts/entities/func.md` | A |
| `glyph_variants.rs` | `prompts/entities/math-topology.md` | **✅ restaurado 2026-04-12** |
| `label.rs` | `prompts/entities/label.md` | **✅ restaurado 2026-04-12** |
| `layout_types.rs` | `prompts/entities/layout_types.md` | A |
| `math_class.rs` | `prompts/entities/math-topology.md` | **✅ restaurado 2026-04-12** |
| `math_constants.rs` | `prompts/entities/math-topology.md` | **✅ restaurado 2026-04-12** |
| `module.rs` | `prompts/entities/module.md` | A |
| `operators.rs` | `prompts/entities/operators.md` | **✅ criado 2026-04-12** |
| `package_spec.rs` | `prompts/entities/package_spec.md` | **✅ restaurado 2026-04-12** |
| `scope.rs` | `prompts/entities/scope.md` | A |
| `source.rs` | `prompts/entities/source.md` | A |
| `source_result.rs` | `prompts/entities/source-result.md` | A |
| `span.rs` | `prompts/entities/span.md` | **✅ restaurado 2026-04-12** (dedicado) |
| `style_chain.rs` | `prompts/entities/style_chain.md` | A (Passo 30) |
| `syntax_kind.rs` | `prompts/entities/syntax-kind.md` | **✅ restaurado 2026-04-12** (dedicado) |
| `syntax_mode.rs` | `prompts/entities/syntax-mode.md` | **✅ restaurado 2026-04-12** |
| `syntax_node.rs` | `prompts/entities/syntax-node.md` | **✅ restaurado 2026-04-12** |
| `syntax_set.rs` | `prompts/entities/syntax-set.md` | **✅ restaurado 2026-04-12** |
| `syntax_text.rs` | `prompts/entities/syntax-text.md` | **✅ restaurado 2026-04-12** |
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
| `export.rs` | `prompts/infra/export.md` | **✅ restaurado 2026-04-12** (CIDFont, ToUnicode, Glyph) |
| `font_metrics.rs` | `prompts/infra/font_metrics.md` | **✅ restaurado 2026-04-12** (math_constants, math_kern, glyph_variants) |
| `fonts.rs` | `prompts/infra/fonts.md` | **✅ restaurado 2026-04-12** (font_info_from_bytes, build_font_book, .ttc) |
| `integration_tests.rs` | ficheiro de testes — isento de V1 | — |
| `layout.rs` | `prompts/infra/layout.md` | A |
| `world.rs` | `prompts/infra/system-world.md` | A |

---

## Resumo por prioridade de restauro

### Prioridade 1 — Sem prompt e com lógica de domínio (L1) [CONCLUÍDO ✅]

Todos os módulos L1 agora possuem sementes documentadas e critérios de verificação.

1. ~~`syntax_node.rs`~~ — **✅ restaurado 2026-04-12**
2. ~~`syntax_text.rs`~~ — **✅ restaurado 2026-04-12**
3. ~~`value.rs`~~ — **✅ restaurado 2026-04-12**
4. ~~`style_chain.rs`~~ — **✅ restaurado 2026-04-12**
5. ~~`source_result.rs`~~ — **✅ restaurado 2026-04-12**
6. ~~`scope.rs` / `scopes.rs`~~ — **✅ restaurado 2026-04-12**
7. ~~`counter_state.rs`~~ — **✅ restaurado 2026-04-12**
8. ~~`operators.rs`~~ — **✅ restaurado 2026-04-12**
9. ~~`package_spec.rs`~~ — **✅ restaurado 2026-04-12**
10. ~~`syntax_set.rs`, `syntax_mode.rs`~~ — **✅ restaurado 2026-04-12**
11. ~~`glyph_variants.rs`, `math_class.rs`, `math_constants.rs`, `label.rs`~~ — **✅ restaurado 2026-04-12**
12. ~~`ast/*.rs`~~ — **✅ restaurado 2026-04-12** (prompts dedicados por módulo)

### Prioridade 2 — Sem prompt em L3 [CONCLUÍDO ✅]

1. ~~`export.rs`~~ — **✅ restaurado 2026-04-12** (CIDFont, Identity-H, ToUnicode, FrameItem::Glyph)
2. ~~`font_metrics.rs`~~ — **✅ restaurado 2026-04-12** (math_constants, math_kern, glyph_variants, reverse_map)
3. ~~`fonts.rs`~~ — **✅ restaurado 2026-04-12** (font_info_from_bytes, build_font_book, suporte .ttc)

### Prioridade 3 — Prompts de Passo 1 que precisam de ficheiros dedicados [CONCLUÍDO ✅]

1. ~~`file_id.rs`~~ — **✅ restaurado 2026-04-12**
2. ~~`span.rs`~~ — **✅ restaurado 2026-04-12**
3. ~~`syntax_kind.rs`~~ — **✅ restaurado 2026-04-12**

---

## Procedimento de restauro (por módulo)

Para cada módulo na Prioridade 1 ou 2:

```text
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
```text

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
