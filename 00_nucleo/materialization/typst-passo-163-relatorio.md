# Relatório P163 — Verificação E2E de M1

Executado em 2026-04-30. Terceiro e último passo da série M1 (P161 + P162 + P163).

## Resumo

- L0 `00_nucleo/prompts/rules/introspect.md` refinado para reflectir alterações de P162 (assinatura de walk com 5 parâmetros, emissão de tags em paralelo, helper de teste). Pendência herdada de P162 verificação .H.6 resolvida.
- 7 tests E2E adicionados em `01_core/src/rules/introspect.rs`:
  - `.C.1` determinismo do walk;
  - `.C.2` bracketing válido em aninhamento complexo (heading ⊃ figure ⊃ heading);
  - `.C.2` (caso adicional) bracketing válido em sequência plana de 3 headings irmãos;
  - `.C.3` hash em End distingue conteúdo;
  - `.D.1` consistência heading (depth + format_hierarchical cruzados);
  - `.D.2` consistência figure (kinds preservados literal em tags vs colapsados em state);
  - `.D.3` consistência citation (3 keys, incluindo repetição).
- Diagnóstico `00_nucleo/diagnosticos/m1-lacunas-captura.md` criado com 3 divergências detectadas (todas adiadas para M2+, sem correcção em M1).
- M1 inteiro concluído. Walk emite tags determinísticas, com bracketing válido, consistentes (modulo as 3 divergências documentadas) com `CounterStateLegacy`.

## Verificações .F

| # | Critério | Estado |
|---|----------|--------|
| 1 | `cargo check --workspace` passa | ✅ |
| 2 | `cargo test --workspace` — todos passam, contagem aumenta vs P162 | ✅ **1 295 + 215 + 24 + 21 = 1 555** (Δ +7 vs P162 = 1 548) |
| 3 | `crystalline-lint`: zero violations | ✅ "✓ No violations found" |
| 4 | L0 `introspect.md` reflecte walk com 5 parâmetros + emissão de tags | ✅ Hash actualizado de `264f58c8` → `2e13b8b8` |
| 5 | Helper `introspect_with_tags` acessível aos tests E2E | ✅ existe em `#[cfg(test)] mod tests` desde P162.G; reusado nos 7 tests novos |
| 6 | Tests .C.1, .C.2, .C.3 passam | ✅ (4 tests: determinismo, bracketing aninhado, bracketing plano, hash distinguishability) |
| 7 | Tests .D.1, .D.2, .D.3 passam | ✅ (3 tests: heading levels, figure kinds, citation keys) |
| 8 | Lacunas detectadas → ficheiro `m1-lacunas-captura.md` existe | ✅ 3 divergências documentadas |
| 9 | Snapshot tests de paridade ADR-0033 verdes | ✅ |
| 10 | Linter passa em verificação final | ✅ |

Δ tests: +7 (4 .C + 3 .D).

## Hashes finais

L0 actualizado em P163:

| L0 | Hash anterior (P162) | Hash actual (P163) |
|----|----------------------|--------------------|
| `00_nucleo/prompts/rules/introspect.md` | `264f58c8` | `2e13b8b8` |

L1 correspondente actualizado pelo `crystalline-lint --fix-hashes`:

| L1 | `@prompt-hash` actual |
|----|----------------------|
| `01_core/src/rules/introspect.rs` | `5fce62c7` |

## Decisões registadas em .A

### API de `CounterStateLegacy` para verificação cruzada

Identificada API existente:
- `format_hierarchical(key) -> Option<String>` — retorna string formatada `"1.2.3"`. Usada em `.D.1` para verificar contador heading após sequência [1,2,2,3] = `"1.2.1"` (interpretação: walk arm Heading chama `step_hierarchical` que avança o último segmento ou empurra novo nível).
- `figure_numbers: HashMap<String, Vec<usize>>` — campo público, permite contar figures por kind. Usado em `.D.2`.
- `bib_numbers` existe mas não usado em P163 (citations não populam state.bib_numbers em M1 — só são acumuladas em `state.bib_entries` quando há `Content::Bibliography`; tests `.D.3` não incluem Bibliography).

Sem necessidade de adicionar getters novos. API actual suficiente.

### Helpers de Content criados

Helper local `make_content_complexo()` no test module — constrói Content com SetHeadingNumbering + heading(1) + figure + heading(2) + cite. Usado pelo test `.C.1` (determinismo).

Outros tests (`.C.2`, `.C.3`, `.D.*`) constroem Content inline com `Content::heading(level, body)` (já existente em `entities/content.rs`), `Content::cite(key, supplement, form)` (já existente), e `Content::Figure { ... }` literal (sem helper porque tem 4 campos).

Nenhum helper novo em `entities/content.rs` foi necessário. Tudo em `#[cfg(test)] mod tests`.

## Resultado das verificações de captura

3 divergências detectadas, registadas em `00_nucleo/diagnosticos/m1-lacunas-captura.md`:

1. **`figure.kind` literal em tags vs colapsado em state** (kind=None preservado em payload mas resolvido para "image" em state.figure_numbers). Adiar — reabrir se M2/M3 exigir paridade.
2. **Auto-labels só em state** (state.headings_for_toc tem auto-labels mas tags não). Adiar — reabrir em M3 quando Introspector for materializado.
3. **Body frozen em state vs hash em tags** (state.headings_for_toc guarda Content completo; Tag::End só guarda hash). Manter — divergência arquitectural intencional.

Nenhuma é bug. Todas são consequência da topologia "tags em paralelo a state legacy" introduzida em P162. Listadas como referência para passos M2+ que consumam tags.

## Estado pós-passo

**M1 concluído** (P161 + P162 + P163):

| Passo | Resumo | Tests Δ |
|-------|--------|---------|
| P161 | 7 tipos novos em `entities/` (Location, Locator, ElementKind, ElementPayload, ElementInfo, CounterUpdate, Tag) + rename CounterState→CounterStateLegacy | +30 |
| P162 | `hash_content` + `extract_payload`; walk passa a emitir Vec<Tag> em paralelo | +17 |
| P163 | L0 `introspect.md` refinado; 7 tests E2E de captura; lacunas documentadas | +7 |
| **Total M1** | **9 ficheiros L1 novos, 9 L0 novos/actualizados, walk reescrita, +54 tests** | |

**M2 desbloqueado**: pré-condição "M1 inteiro concluído" satisfeita. M2 pode começar — extrair `is_locatable` como função pública e iniciar consumo de `Vec<Tag>` (primeiro consumidor real das tags que P163 verificou estarem bem capturadas).

**Pendências passadas adiante para M2+**:
- 3 divergências em `m1-lacunas-captura.md` (revisitar quando consumer real for materializado).
- Reshape opcional de `CounterUpdate::Step` para `Step(usize)` (P161 manteve sem payload por compatibilidade; vanilla usa `NonZeroUsize`).
- Refino de `hash_content` para versão recursiva manual (P162 escolheu versão minimalista via `format!("{:?}", c)`; aceitável em M1, pode precisar de revisão se Debug stability se tornar preocupação).

API pública `pub fn introspect(content: &Content) -> CounterStateLegacy` preservada ao longo de toda a série M1. Consumidores externos (Layouter, materialize_time pipeline) não tocaram.

Output observable inalterado em todo o M1. Snapshot tests de paridade ADR-0033 verdes em P161, P162, P163.
