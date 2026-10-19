## Relatório P180 — Inventário de bib state (lacuna #6)

Executado em 2026-04-29. Passo documental — sem código de produção tocado.

## Resumo

- **Diagnóstico produzido**: `00_nucleo/diagnosticos/inventario-bib-state.md` com 9 secções (componentes, consumers, comparação vanilla, magnitude, recomendação, decisões pendentes, riscos, resumo numérico, estado lacuna).
- **Magnitude confirmada**: **S-M** — bib state cristalino é minimal (2 fields legacy, 1 consumer real, sem dependências externas como hayagriva/CSL).
- **Recomendação**: **Caminho A — implementação directa em P181** via padrão sub-store + locatable kind (replicação de P165/P169/P171/P178 estabelecidos).
- **Lacuna #6 actualizada** em `m1-lacunas-captura.md` com nota P180.

## Verificações `.F`

| # | Critério | Estado |
|---|----------|--------|
| 1 | `cargo check --workspace` passa (não tocámos código) | ✅ |
| 2 | `cargo test --workspace` passa sem mudança de contagem | ✅ **1 700** (inalterado vs P179) |
| 3 | `crystalline-lint`: zero violations (não modificámos L0/L1) | ✅ |
| 4 | Diagnóstico `inventario-bib-state.md` existe com 5 secções obrigatórias | ✅ — produzido com 9 secções |
| 5 | Nenhum L0 modificado | ✅ |

## Resumo numérico do inventário

- **Fields legacy em `CounterStateLegacy`**: 2 (`bib_entries: Vec<BibEntry>`, `bib_numbers: HashMap<String, u32>`).
- **Tipos relacionados**: 3 (`BibEntry` 16-fields, `Content::Bibliography`, `Content::Cite`).
- **Walk arms com mutação**: 1 (`Content::Bibliography` em `introspect.rs:567-573`).
- **Stdlib funcs**: 1 (`bibliography(...)` via `extract_bib_entries`).
- **Consumers production**: 1 (Layouter cite-arm em `layout/mod.rs:584-597` resolve forms `Normal/Prose/Author/Year`).
- **Sites de cópia state→Layouter**: 2 (`layout()` legacy + `layout_with_introspector`).
- **Test sites**: 3 (`counter_state_legacy::tests`).

## Decisão recomendada para P181

**Caminho A — Implementação directa**.

Sub-passos sugeridos (10 total):

1. `.A` — confirmar inventário P180 + decisões locais (forma de `BibStore`, walk arm modificação).
2. `.B` — `entities/bib_store.rs` — sub-store paralelo a `MetadataStore`.
3. `.C` — `ElementKind::Bibliography` + `ElementPayload::Bibliography { entries }`.
4. `.D` — `is_locatable(Content::Bibliography) == true` + `extract_payload` arm.
5. `.E` — `from_tags` arm popula `bib_store`.
6. `.F` — `Introspector` trait estendido com `bib_entry_for_key` + `bib_number_for_key`.
7. `.G` — walk arm modificado (Opção β: emite Tag, sem mutar state directamente).
8. `.H` — Layouter cite-arm migra para `Introspector` (similar a P168 figure-ref).
9. `.I` — tests E2E + lacuna #6 fechada.
10. `.J` — relatório P181.

**Estimativa**: +15 a +25 tests; magnitude comparável a P171 (state feature).

**Sem gates substanciais identificados** — todos os pré-requisitos arquitecturais estão satisfeitos. Padrão sub-store estabelecido por 4 sub-stores anteriores (LabelRegistry P165, MetadataStore P169, StateRegistry P171, history em CounterRegistry P177). `BibStore` replica directamente.

## Decisões a tomar antes de P181

Documentadas em `inventario-bib-state.md` §6:

1. Forma de `BibStore` storage: `Vec<BibEntry>` vs `IndexMap<key, BibEntry>` (sugestão `IndexMap`).
2. Multi-Bibliography concat semantics (replicar `extend` actual).
3. `bib_numbers` order preservation (manter `or_insert` actual).
4. Walk arm modificação: Opção α (dual-state) vs Opção β (puro). Sugestão **β**.
5. Layouter cite-arm migração (caminho similar a P168 figure-ref).
6. Lacuna #6 fechamento: P181 fecha em "infraestrutura pronta + consumer migrado"; M6 elimina fields legacy.

## Comparação com vanilla — síntese

| Aspecto | Vanilla | Cristalino |
|---------|---------|------------|
| Parser BibLaTeX | `hayagriva` (3rd party) | Ausente — input literal |
| CSL formatting | `citationberg` (100+ styles) | 4 forms hardcoded |
| Storage | `IndexMap<PicoStr, hayagriva::Entry>` | `Vec<BibEntry>` linear scan |
| Memoization | comemo via Introspector | Sem memoization |
| LOC `bibliography.rs` | ~1226 | ~150 (entity + walk arm) |

Cristalino divergiu deliberadamente para subset minimal **sem dependências externas**. Bib state cristalino é **Introspection-style**, não domínio próprio externo. Compatível com migração para `Introspector`.

## Estado de M9

Sem mudança estrutural. P180 é documental. **9/11 features materializadas**:
1-9. P169-P179 (sem alteração).

P181 → 10/11 (bib state) se executado.

## Pendências cumulativas

Sem alteração — P180 não toca código.

Lacuna #6 actualizada com nota: "Inventário P180; magnitude S-M; recomendação P181".

## Estado pós-passo

- **P180 concluído**.
- **Diagnóstico completo** disponível para consulta em `00_nucleo/diagnosticos/inventario-bib-state.md`.
- **P181 desbloqueado** — implementação directa de bib state como `BibStore` sub-store + `Bibliography` locatable kind.

API pública preservada (P180 não toca). Output observable inalterado (P180 não toca). Sem ADR nova. Walk puro (P180 não toca). Sem reservas.
