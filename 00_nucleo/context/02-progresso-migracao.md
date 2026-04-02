# Progresso da Migração — Passos Concluídos e Próximos

**Data**: 2026-04-01
**Estimativa total**: ~120 passos | **Concluídos**: ~27

---

## Passos concluídos (0–27)

### Fase 0 — Fundação (Passos 0–3)

| Passo | Descrição | Resultado |
|-------|-----------|-----------|
| 0 | Estrutura base, `lab/typst-original/`, workspace cristalino | Directórios criados, `Cargo.toml` workspace, `crystalline.toml` |
| 1 | `FileId`, `SyntaxKind`, `Span` migrados para L1 | Tipos de domínio fundamentais; `FileIdInterner` movido para L3 |
| 2 | `SyntaxText` (Opção C), `SyntaxNode`, `SyntaxSet` | IR de sintaxe completa; `SyntaxText(Arc<str>)` sem `ecow` inicial |
| 3 | `PackageSpec` (DTO pattern), `World` + `TrackedWorld` (B3) | Contratos L1; blanket impl sem supertrait (limitação comemo); 69 testes |

### Fase 1 — Pipeline básico (Passos 4–10)

| Passo | Descrição | Resultado |
|-------|-----------|-----------|
| 4–6 | `parse()`, `Source` real, pipeline eval/layout inicial | Parser funcional; Passo 4 original dividido em 3 por granularidade |
| 7 | `SystemWorld` em L3 | Implementação concreta de `World` |
| 8 | `Scope` e `Module` stub | `indexmap` autorizada (ADR-0023); `Scope` com `FxBuildHasher` |
| 9 | Datetime real com crate `time` (ADR-0021) | `world.today()` funcional |
| 10 | `FontBook` real em L3 (ADR-0022) | `FontInfo`, `FontVariant`, métodos de pesquisa |

### Fase 2 — Layout e Content (Passos 11–23)

| Passo | Descrição | Resultado |
|-------|-----------|-----------|
| 11 | `Scope` em L1 com `Binding` | Infraestrutura de nomes |
| 12–18 | Value real, eval expressions, stdlib functions | `Value` enum com tipos básicos; `rgb()`, `luma()`, funções `calc` |
| 19 | Layout types (`Pt`, `Frame`, `PagedDocument`) + Layouter | Layouter genérico sobre `FontMetrics` trait |
| 20 | `FontBookMetrics` com métricas proporcionais | `ttf-parser` em L3; métricas reais por injecção |
| 21 | Baseline correcta, métricas escaláveis | `advance()`, `vertical_metrics()`, descender defensivo |
| 22 | `Content::Strong/Emph/Heading` + `TextStyle` | Rich text; `TextStyle` plano (DEBT-1 para StyleChain) |
| 23 | `Content::Raw/ListItem/EnumItem/Link` | Último passo incremental antes do pagamento de dívida; 351+ testes |

### Fase 3 — Pagamento de dívida (Passos 24–27)

| Passo | Descrição | Resultado |
|-------|-----------|-----------|
| 24 | DEBT-5: Unicode PDF — CIDFont/ToUnicode | Embedding TrueType; bullets Unicode no PDF |
| 25 | Tipos tipográficos (`Length`, `Ratio`, `Angle`, `Color`, `Auto`) em `Value` | `rgb()`/`luma()` stdlib |
| 26 | Correcções arquitecturais | `Length` como vanilla struct; `Content::Sequence` → `Arc<[Content]>` com `PartialEq` manual |
| 27 | DEBT-4: funções de conversão e módulo `calc` | `str()`, `int()`, `float()`, `Expr::FieldAccess`; `calc` como `Value::Dict` |

**Contagem de testes ao fim do Passo 27**: ~high 300s (L1 + L3)
**Violations do linter**: zero mantidas ao longo de toda a migração

---

## Dívida técnica — inventário activo

O inventário foi produzido após auditoria externa (Gemini) no Passo 22.

| ID | Descrição | Passos estimados | Estado |
|----|-----------|-----------------|--------|
| **DEBT-5** | Unicode PDF (CIDFont/ToUnicode) | 24 | ✅ Resolvido |
| **DEBT-4** | `Value` real com conversões e `calc` | 25–27 | ✅ Resolvido |
| **DEBT-3** | Hardcoded safety rails | 28–29 | ⏳ Próximo |
| **DEBT-1** | StyleChain (lista ligada de deltas de estilo) | 30 | ⏳ Requer `Arc<[Content]>` (concluído no Passo 26) |
| **DEBT-2** | Lazy/eager closure capture | 31+ | ⏳ Pendente |
| **DEBT-6** | `eval_for_test` coverage blind spot | 32+ | ⏳ Pendente |

---

## Próximos passos imediatos (28–32)

| Passo | Descrição | Pré-condição |
|-------|-----------|-------------|
| 28–29 | **DEBT-3** — remover safety rails hardcoded | DEBT-4 e DEBT-5 resolvidos ✓ |
| 30 | **DEBT-1** — StyleChain | ADR-0026 (`Arc<[Content]>`) concluído ✓ |
| 31 | **DEBT-2** — lazy/eager closure capture | StyleChain implementado |
| 32 | **DEBT-6** — cobertura de `eval_for_test` | Closures resolvidas |

---

## Horizonte de longo prazo (Passos 33–120)

Baseado no roadmap `typst-roadmap-paridade.md`:

| Fase | Passos | Descrição |
|------|--------|-----------|
| Layout engine real | 36–55 | Caixas, parágrafos, flow layout, grids, tabelas, imagens |
| Matemática | 56–75 | Motor de equações (`$...$`), símbolos matemáticos |
| Introspection e estado | 76–95 | Contadores, referências cruzadas, `locate()`, `query()` |
| Graphics/export/stdlib | 96–120 | SVG, PNG, HTML, biblioteca padrão completa |

**Marco importante**: Passo ~50 — subconjunto viável de produção (estilos, grids, tabelas, imagens).

**Decisão pendente**: Passo 36 — milestone de decisão sobre se a fase de matemática é prioritária ou se o projecto salta directamente para graphics.

---

## Velocidade e padrão observado

- Passos 0–27 cobriram a fundação completa: tipos de domínio, parser, evaluator, layouter, PDF export com Unicode
- Cada passo segue o ciclo: diagnóstico → ADR (se necessário) → implementação → verificação → relatório
- Granularidade máxima: um ADR por decisão, passos divididos quando o scope é largo demais
- Claude Code executa; Diego supervisiona e reporta; Claude (chat) gera ADRs e prompts de materialização
