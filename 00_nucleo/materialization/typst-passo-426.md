# P426 — Análise de Estado: snapshot do projeto Cristalino para migração de ciclo

**Título**: Análise de estado — diagnóstico completo do Tekt/Cristalino post-P424 para próximo ciclo
**Tipo**: Análise (A) — não materialização; diagnóstico e planejamento
**Bloqueadores**: Nenhum; este passo é puramente documental
**Referências**: ADR-0107, ADR-0108, ADR-0109, ADR-0062, P329–P424

---

## FASE A.0 — Sonda do substrato: inventário completo

Métricas medidas em `2026-06-23` no commit `a2b3b3536` (branch `Tekt`).

| # | Métrica | Comando | Resultado |
|---|---------|---------|-----------|
| 1 | LOC `typst-core` | `find 01_core/src -name "*.rs" \| xargs wc -l` | **100.192** |
| 1 | LOC `typst-infra` | `find 03_infra/src -name "*.rs" \| xargs wc -l` | **15.865** |
| 1 | LOC `typst-shell` | `find 02_shell/src -name "*.rs" \| xargs wc -l` | **541** |
| 1 | LOC `typst-wiring` | `find 04_wiring/src -name "*.rs" \| xargs wc -l` | **214** |
| 2 | Variants `Content` (total) | `awk` no `enum Content` | **76** |
| 2 | Arms `Content::` em `layout_content` | `grep -rn 'Content::' 01_core/src/rules/layout/mod.rs \| grep -c '=>'` | **103** |
| 3 | Variants `Value` (total) | `awk` no `enum Value` | **28** |
| 3 | Arms `Value::` em `repr.rs` | `grep -rn 'Value::' 01_core/src/rules/eval/repr.rs \| grep -c '=>'` | **28** |
| 4 | Tests `typst-core` | `cargo test -p typst-core --lib -- --skip p350c_flag_on_nao_convergente_classifica` | **3.156 passed**; 1 skipped (stack overflow preexistente) |
| 4 | Tests `typst-infra` | `cargo test -p typst-infra --lib` | **482 passed**; 6 ignored |
| 5 | Lint errors/drift | `crystalline-lint . \| grep -E "error\|drift"` | **0** |
| 6 | Warnings workspace | `cargo check --workspace \| grep -c "warning:"` | **29** |
| 7 | ADR arquivos | `find 00_nucleo/adr -type f -name '*.md' \| wc -l` | **114** |
| 7 | ADRs IMPLEMENTADO | `grep -rl 'IMPLEMENTADO' 00_nucleo/adr/ \| wc -l` | **46** |
| 7 | ADRs PROPOSTO | `grep -rl 'PROPOSTO' 00_nucleo/adr/ \| wc -l` | **45** |
| 7 | ADRs EM VIGOR | `grep -rl 'EM VIGOR' 00_nucleo/adr/ \| wc -l` | **58** |
| 8 | Arms do `match content` | `grep -rn 'match content' 01_core/src/rules/layout/mod.rs -A 50 \| grep -c '=>'` | **13** |
| 9 | Deps workspace | `Cargo.toml [workspace.dependencies]` | 25+ entradas (ver `Cargo.toml`) |
| 9 | Deps `typst-core` | `01_core/Cargo.toml [dependencies]` | 18 crates diretas |
| 10 | Commits desde 2026-06-01 | `git log --oneline --since="2026-06-01" \| wc -l` | **188** |
| 11 | Scope-outs acumulados | `grep -rni 'scope-out\|scope_out\|Scope-out' 00_nucleo/ \| wc -l` | **3.055** |
| 12 | Débitos `typst-core` | `grep -rni 'TODO\|FIXME\|HACK\|XXX\|DEBT' 01_core/src/ \| wc -l` | **426** |
| 12 | Débitos `typst-infra` | `grep -rni 'TODO\|FIXME\|HACK\|XXX\|DEBT' 03_infra/src/ \| wc -l` | **69** |

**HEAD atual**: `a2b3b3536d8493d0e792c7dd3f213bfa8033da0f`  
**Branch**: `Tekt`

---

## FASE A.1 — L0

Criado `00_nucleo/prompts/meta/p426-analise-estado.md` com o protocolo de sonda, templates de inventário e critério de fecho.

---

## FASE B — Análise

### B.1 — Métricas do ciclo P329–P424

| Métrica | Valor | Nota |
|---------|-------|------|
| Passos executados (P329–P424) | ~96 | Contagem aproximada dos passos documentados |
| Commits desde início do ciclo | **188** | Desde 2026-06-01 |
| Linhas de código (typst-core) | **100.192** | Domínio principal |
| Linhas de código (typst-infra) | **15.865** | Exportador PDF e infraestrutura |
| Tests verdes (typst-core) | **3.156** | 1 teste preexistente skipped |
| Tests verdes (typst-infra) | **482** | 6 ignored |
| Warnings compilador | **29** | Workspace completo |
| ADRs IMPLEMENTADO | **46** | Arquivos |
| ADRs PROPOSTO | **45** | Arquivos |
| ADRs EM VIGOR | **58** | Arquivos |
| Scope-outs acumulados | **3.055** | Ocorrências em `00_nucleo/` |
| TODO/FIXME/DEBT no código | **426** (`core`) + **69** (`infra`) | Marcadores explícitos |

### B.2 — Inventário de Domínios (post-P424)

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

> Percentuais são estimativas qualitativas baseadas em variants implementados, cobertura de tests e scope-outs documentados.

### B.3 — Gaps Mecânicos Identificados

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
| 9 | Model: `bibliography` CSL-JSON de disco | S | Baixo | Fechado (P419) |
| 10 | Model: `cite` supplement CSL-native | S | Baixo | Scope-out |
| 11 | Introspection: `query` com `Selector::Where` real | M | Médio | Stub (P417) |
| 12 | Foundations: `datetime` formatting | S | Baixo | Parcial |
| 13 | Foundations: `calc` functions avançadas | M | Médio | Parcial |
| 14 | Shell: CLI argument parsing | M | Alto | Ausente |
| 15 | Shell: watch mode (recompile on change) | L | Alto | Ausente |

### B.4 — Débitos Técnicos Acumulados

| # | Débito | Origem | Risco | Nota |
|---|--------|--------|-------|------|
| 1 | `p350c_flag_on_nao_convergente_classifica` stack overflow | P350c | Alto | Teste preexistente falhando; pode mascarar regressões |
| 2 | `FrameItem::Link` bbox aproximada para Group | P424 | Baixo | Usa inner_width/inner_height sem transformar; aceitável para MVP |
| 3 | `Selector::Where` em query retorna `vec![]` | P417 | Médio | Stub documentado; não afeta show rules |
| 4 | `text.lang` rustybuzz não implementado | Antigo | Alto | Maior gap tipográfico; bloqueia documentos não-latinos |
| 5 | File loader genérico não existe | P419 | Médio | Cada feature faz I/O próprio; reutilização limitada |
| 6 | Cache de styles não persistente | P420 | Baixo | Recarrega CSL a cada eval; aceitável |

### B.5 — ADRs que Precisam de Revisão

| ADR | Status | Nota |
|-----|--------|------|
| ADR-0107 | EM VIGOR | Funcionando; nenhuma revisão necessária |
| ADR-0108 | EM VIGOR | Funcionando; nenhuma revisão necessária |
| ADR-0109 | EM VIGOR | Funcionando; nenhuma revisão necessária |
| ADR-0062 | IMPLEMENTADO | Hayagriva integrado; estável |
| ADR-0054 | PROPOSTO | Graded scope-out; pode ser promovido a EM VIGOR ou descartado |
| ADR-0026 | EM VIGOR | Enum sem vtable; satisfeito por P421/P423 |
| ADR-0033 | EM VIGOR | Paridade funcional; satisfeito |

### B.6 — Recomendações para Próximo Ciclo

**Opção A: Continuar Materialização (foco em PDF + Layout)**
- P427: PDF writer shapes (rect, circle, ellipse) — M
- P428: PDF writer images (PNG/JPEG embed) — M
- P429: PDF writer fonts (subset embed) — L
- P430: `text.lang` rustybuzz — XL (scope-out antigo; requer decisão de dono)

**Opção B: Expansão para Shell/CLI**
- P427: CLI argument parsing (clap ou próprio) — M
- P428: Watch mode (notify crate) — L
- P429: REPL interativo — XL

**Opção C: Refinamento e Qualidade**
- P427: Resolver `p350c_flag_on_nao_convergente_classifica` stack overflow — M
- P428: `Selector::Where` em query real — M
- P429: File loader genérico — L
- P430: Cache persistente de styles — S

**Opção D: Pesquisa e Arquitetura**
- P427: Análise de benchmark (tempo de compilação, memória) — A
- P428: Decisão de crates para wiring/plugins — A
- P429: Plano de documentação de usuário — A

**Recomendação do P426**: **Opção A** (PDF + Layout) — fecha o maior gap funcional (PDF export) antes de expandir para shell. `text.lang` rustybuzz é XL e requer decisão do dono; pode ser adiado.

---

## FASE C — Validação

```bash
# Nenhum código alterado — apenas documentação
crystalline-lint .
# → 0 errors; 0 drift
```

**Critério de fecho**:
- [x] Sonda A.0 executada com valores reais.
- [x] L0 `meta/p426-analise-estado.md` criado.
- [x] Materialization `typst-passo-426.md` atualizado com métricas medidas.
- [x] Nenhum arquivo de código alterado.
- [x] Commit registrando a análise.

---

## Relatório de Execução — P426

**Data**: 2026-06-23
**Executor**: assistente IA (Kimi Code CLI)
**Branch**: `Tekt`
**Commit**: `a2b3b3536d8493d0e792c7dd3f213bfa8033da0f`

**Sonda A.0**: todas as 12 medições executadas e registadas na tabela acima.

**L0**: `00_nucleo/prompts/meta/p426-analise-estado.md` criado.

**Análise**:
- O ciclo P329–P424 produziu **188 commits** e deixou o `typst-core` com **100.192 LOC** e **3.156 tests verdes**.
- O maior gap funcional é **PDF Export** (~60%): shapes, images e font embedding ainda ausentes.
- O maior gap tipográfico é **`text.lang` / rustybuzz**, mantido como scope-out.
- Não há drift de prompts (`crystalline-lint` → 0 errors).
- Apenas **29 warnings** de compilador no workspace.

**Decisão recomendada**: próximo ciclo deve focar em **PDF Export** (Opção A), começando por shapes e images, antes de avançar para shell/CLI ou qualidade interna.

**Notas epistêmicas**:
- **ADR-0108**: todas as recomendações são baseadas nas métricas medidas, não em intuição.
- **Honestidade**: o teste `p350c_flag_on_nao_convergente_classifica` continua falhando por stack overflow preexistente; isso é documentado como débito técnico.
- **Próximo passo**: depende da decisão do dono (Opção A, B, C ou D).

---

## FASE A.2 — Checklist de Migração para Nova Conversa

Para carregar este estado em uma nova sessão, o dono deve fornecer:

- [x] Commit atual: `a2b3b3536d8493d0e792c7dd3f213bfa8033da0f`
- [x] Branch: `Tekt`
- [x] Este arquivo: `typst-passo-426.md`
- [ ] ADRs em vigor: listar de `00_nucleo/adr/`
- [ ] Scope-outs acumulados: resumo de `00_nucleo/scope-outs.md` (se existir)
- [ ] Próximo passo desejado: A, B, C, ou D (ou número específico P427+)
