# Prompt L0 — Meta: P426 Análise de Estado do Ciclo P329–P424
Hash do Código: *n/a* (prompt de processo; não gera código direto)

**Camada**: Meta / análise de estado
**Ficheiro alvo**: `00_nucleo/materialization/typst-passo-426.md`
**Criado em**: 2026-06-23
**ADRs relevantes**: ADR-0107, ADR-0108, ADR-0109, ADR-0062

---

## Propósito

P426 é um passo de **análise pura** (tipo A). Não produz código, não altera tests e não gasta LOC. O objetivo é medir o estado do projeto Cristalino após P424 e produzir um snapshot documental para o dono decidir o próximo ciclo.

---

## Sonda obrigatória (A.0)

Antes de redigir qualquer conclusão, executar os 12 comandos do materialization P426 e registar os valores observados:

1. Linhas de código por camada (`wc -l` em `*.rs`).
2. Variants de `Content` (total + arms consumidos em `layout_content`).
3. Variants de `Value` (total + arms consumidos em `repr.rs`).
4. Tests (`cargo test -p typst-core --lib` e `cargo test -p typst-infra --lib`).
5. Lint (`crystalline-lint . | grep -E "error|drift"`).
6. Warnings de compilador (`cargo check --workspace | grep -c "warning:"`).
7. ADRs por status (arquivos IMPLEMENTADO/PROPOSTO/EM VIGOR).
8. Arms do `match content` em `layout/mod.rs`.
9. Dependências externas do workspace e de `typst-core`.
10. Commits desde o início do ciclo.
11. Ocorrências de "scope-out" em `00_nucleo/`.
12. TODO/FIXME/HACK/XXX/DEBT em `01_core/src/` e `03_infra/src/`.

---

## Inventário de Domínios (template)

| Domínio | Implementado | Parcial | Ausente | Scope-out | Nota |
|---------|-------------|---------|---------|-----------|------|
| Layout | ~97% | — | — | — | P422/P424 fecharam link; P425 atomizou arms |
| Model | ~66% | ~7 | ~2 | — | Bibliography/cite completo (P418–P420) |
| Text | ~44% | — | ~4 | ~1 | `text.lang` rustybuzz scope-out |
| Math | ~92% | — | — | — | Estável |
| Foundations | ~75% | ~4 | ~1 | — | `repr()` completo (P421) |
| Introspection | ~88% | ~1 | — | — | Selector And/Or (P423) |
| PDF Export | ~60% | ~20 | ~20 | ~5 | P424 fechou link; faltam shapes, images, fonts |
| Shell/CLI | ~30% | — | ~50 | — | Fora do escopo do ciclo atual |
| Wiring/Plugins | ~10% | — | ~80 | — | Fora do escopo do ciclo atual |

> Os valores percentuais são estimativas qualitativas baseadas no inventário de variants e na cobertura de tests. Não devem ser apresentados como medições precisas.

---

## Gaps Mecânicos Identificados (template)

| # | Gap | Tamanho | Impacto | Status |
|---|-----|---------|---------|--------|
| 1 | PDF writer: shapes (rect, circle, ellipse, polygon) | M | Médio | Ausente |
| 2 | PDF writer: images (embed PNG/JPEG) | M | Médio | Ausente |
| 3 | PDF writer: fonts (embed subset) | L | Alto | Parcial |
| 4 | PDF writer: gradients e patterns | M | Médio | Ausente |
| 5 | Layout: `text.lang` shaping (rustybuzz) | XL | Alto | Scope-out |
| 6 | Layout: `text.hyphenate` | L | Médio | Ausente |
| 7 | Layout: floats avançados (wrap around) | L | Médio | Scope-out |
| 8 | Layout: multi-column com footnotes | M | Médio | Scope-out |
| 9 | Introspection: `query` com `Selector::Where` real | M | Médio | Stub (P417) |
| 10 | Shell: CLI argument parsing | M | Alto | Ausente |
| 11 | Shell: watch mode | L | Alto | Ausente |

---

## Débitos Técnicos Acumulados (template)

| # | Débito | Origem | Risco | Nota |
|---|--------|--------|-------|------|
| 1 | `p350c_flag_on_nao_convergente_classifica` stack overflow | P350c | Alto | Teste preexistente falhando; mascarado com `--skip` |
| 2 | `FrameItem::Link` bbox aproximada para `Group` | P424 | Baixo | Usa `inner_width`/`inner_height` sem transformar |
| 3 | `Selector::Where` em query retorna `vec![]` | P417 | Médio | Stub documentado; não afeta show rules |
| 4 | `text.lang` rustybuzz não implementado | Antigo | Alto | Maior gap tipográfico |
| 5 | File loader genérico não existe | P419 | Médio | Cada feature faz I/O próprio |

---

## Recomendações para Próximo Ciclo (template)

**Opção A — PDF + Layout (recomendada)**
- P427: PDF writer shapes — M
- P428: PDF writer images — M
- P429: PDF writer fonts (subset embed) — L
- P430: `text.lang` rustybuzz — XL (decisão do dono)

**Opção B — Shell/CLI**
- CLI argument parsing, watch mode, REPL.

**Opção C — Qualidade**
- Resolver stack overflow P350c, `Selector::Where` real, file loader genérico.

**Opção D — Pesquisa/Arquitetura**
- Benchmark, decisão de crates wiring/plugins, documentação de usuário.

---

## Critério de fecho

- [ ] Sonda A.0 executada e valores registados.
- [ ] Materialization `typst-passo-426.md` atualizado com métricas reais.
- [ ] L0 `meta/p426-analise-estado.md` criado.
- [ ] Nenhum arquivo de código alterado.
- [ ] Commit `P426: análise de estado — snapshot do ciclo P329–P424`.

---

## Histórico de Revisões

| Data | Motivo | Ficheiros afetados |
|------|--------|-------------------|
| 2026-06-23 | Criação do L0 de análise de estado | `meta/p426-analise-estado.md` |
