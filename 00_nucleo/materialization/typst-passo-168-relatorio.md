# Relatório P168 — Migrar `figure-ref` em `layout_ref` (M5 sub-passo 2)

Executado em 2026-04-30. Primeira migração real de consumer para `Introspector`.

## Resumo

- **Caminho C escolhido** em `.A`: campo `is_counted: bool` adicionado a `ElementPayload::Figure`. `extract_payload` calcula via `figure.numbering.is_some() && figure.caption.is_some()` — paridade com walk arm `Content::Labelled` em `introspect.rs:366`.
- `from_tags` popula novo sub-store `TagIntrospector.figure_label_numbers: HashMap<Label, usize>` com numeração 1-based para figuras `is_counted=true` que tenham label associada.
- Trait `Introspector` ganhou método `figure_number_for_label(&self, label: &Label) -> Option<usize>` — paridade observable com `state.figure_label_numbers.get(label).copied()` legacy.
- `Layouter` ganhou field `introspector: TagIntrospector` (default `empty()`); novo entry point `pub fn layout_with_introspector(content, state, introspector) -> PagedDocument`. `pub fn layout(content, state)` torna-se wrapper que chama com introspector vazio (callers legacy preservados).
- `references.rs::layout_ref` migrado: tenta `introspector.figure_number_for_label` PRIMEIRO; fallback a `state.figure_label_numbers` (caminho legacy preservado para callers de `layout()` legacy). Section-ref permanece em legacy (lacunas #4–#7).

## Verificações .F

| # | Critério | Estado |
|---|----------|--------|
| 1 | `cargo check --workspace` passa | ✅ |
| 2 | `cargo test --workspace` — todos passam, contagem aumenta | ✅ **1 597 tests** (Δ +9 vs P167 = 1 588 — 1 593 total no P166, baixou para 1 593 após P167 sem código tocado, e subiu para 1 597 com P168). Note: `1342 + 215 + 24 + 21 = 1602`, e somando os 4 e2e em layout/tests = 1602 menos 1593 = 9. |
| 3 | `crystalline-lint`: zero violations | ✅ "✓ No violations found" |
| 4 | Caminho C implementado em `.B` | ✅ `is_counted: bool` no payload + cálculo em `extract_payload` |
| 5 | `Layouter` tem acesso a `TagIntrospector` (field) | ✅ `pub(super) introspector: TagIntrospector` |
| 6 | `layout_ref` consulta `introspector.figure_number_for_label` | ✅ com fallback ao legacy |
| 7 | `CounterStateLegacy.figure_label_numbers` continua a ser populado | ✅ walk arm `Content::Labelled` inalterado; legacy callers via `layout()` ainda usam o fallback |
| 8 | L0s actualizados | ✅ 4 L0s actualizados (element_payload, extract_payload, from_tags, introspector) |
| 9 | Snapshot tests ADR-0033 verdes | ✅ |
| 10 | Linter passa em verificação final | ✅ |

Δ tests: +9 (1 figure_is_counted distingue + 4 from_tags figure_label_numbers + 4 layout migration tests).

## Hashes finais

L0s actualizados (calculados pelo `crystalline-lint --fix-hashes`):

| L0 | Hash anterior | Hash actual | `@prompt-hash` em L1 |
|----|---------------|-------------|----------------------|
| `entities/element_payload.md` | `a67d96b1` (P162.C) | `b537b206` | `554fe3c0` |
| `entities/introspector.md` | `27afd094` (P165) | `44832b7d` | `23d07186` |
| `rules/introspect/extract_payload.md` | `e61765d4` (P162) | `b88b06fe` | `9a9d35ab` |
| `rules/introspect/from_tags.md` | `ff2f0f2f` (P165) | `72a1ee00` | `48814d94` |

Outros L1s modificados em P168 (sem L0 separado dedicado):
- `01_core/src/rules/layout/mod.rs` — adiciona `Layouter.introspector` field + `layout_with_introspector` entry point.
- `01_core/src/rules/layout/references.rs` — `layout_ref` consulta introspector primeiro.
- `01_core/src/rules/layout/tests.rs` — 4 tests novos (`p168_figure_ref_migration`).

## Decisões registadas em .A

### Caminho A vs C: escolhido **C**

**Justificação**: `figure.numbering` e `figure.caption` são acessíveis directamente em `extract_payload` (recebe `&Content`); caminho C é viável e mais alinhado com o desenho payload type-safe. Adicionar `is_counted: bool` ao payload torna a propriedade explícita e indexação em `from_tags` torna-se trivial.

Compromisso aceite: 4 L0s + 4 L1s tocados (L0 element_payload, extract_payload, from_tags, introspector — todos minimamente). Caminho A teria sido marginalmente menor mas com lógica mais espalhada e payload menos descritivo.

### Mecanismo de passagem `introspector` ao `Layouter`

**Decisão**: field `introspector: TagIntrospector` no Layouter (default `empty()`); novo entry point `layout_with_introspector(content, state, introspector)`. Wrapper `layout(content, state)` chama com `TagIntrospector::empty()`.

Vantagens:
- ~22 call-sites legacy (`layout(&content, introspect(&content))`) **não mudam** — backward compat 100 %.
- Callers que querem migrar usam `layout_with_introspector(&content, state, intr)` com `(state, intr) = introspect_with_introspector(&content)`.
- `references.rs::layout_ref` faz substitution-with-fallback: introspector primeiro, legacy depois — invariante observable preservado nos dois caminhos (test `paridade_pre_post_migracao` confirma).

### Resolução de lacuna #1 (figure.kind None vs "image")

Caminho C resolve **indirectamente** via `is_counted`: filtro `numbering.is_some() && caption.is_some()` aplica-se igualmente a kind=None e kind=Some("image"); kind literal preserva-se em `payload.kind` mas índice `figure_label_numbers` é coerente entre legacy e introspector. Lacuna #1 deixa de afectar este consumer específico (figure-ref). Lacunas #2 (auto-labels) e #3 (body frozen) permanecem irrelevantes para figure-ref.

## Estado de M5

- **1 consumer migrado** em P168: `references.rs::layout_ref` figure-arm.
- **5 consumers permanecem Bloqueados** (per inventário P167):
  - `Layouter::layout` (mod.rs) — bloqueado por lacunas #4 (numbering_active), #5 (hierarquia counters), #6 (bib state), #7 (has_outline).
  - `layout_outline` (outline.rs) — bloqueado por lacuna #3 (body frozen).
  - `counter_helpers` (counters.rs) — bloqueado por padrão de mutação (Introspector é read-only).
  - `layout_equation` (equation.rs) — bloqueado: Equation não é payload kind.
  - `references.rs::layout_ref` section-arm — bloqueado por lacuna texto formatado (`resolved_labels`).
- **M5 fica em pausa** após P168. P169 inicia M9 (features Introspection vanilla) — todas as 11 features antes de M5 retomar para os outros consumers (decisão pós-P167 inscrita em P168 §"Após P168, M5 fica em pausa").

## Pendências cumulativas

- 7 lacunas em `m1-lacunas-captura.md`. **Lacuna #1** parcialmente resolvida para o caso figure-ref (caminho C); permanece relevante para futuros consumers que não apliquem o mesmo filtro.
- Lacunas #2 (auto-labels), #3 (body frozen) permanecem.
- Lacunas #4 (numbering_active), #5 (hierarquia counters), #6 (bib state), #7 (has_outline) permanecem — pré-condições para retomar M5.
- Reshape opcional `CounterUpdate::Step` → `Step(usize)`.
- Refino opcional `hash_content`.
- `Position` não materializado (M5/M9).
- `comemo::track` deferido (M7+).
- `CounterKey` enum vanilla deferido (M9).
- `MetadataStore` adiado (M9).
- **Nova em M5**: `figure_label_numbers` legacy continua a ser populado pelo walk paralelamente ao `Introspector` — redundância intencional até M6 eliminar legacy.

## Estado pós-passo

- **P168 concluído**.
- **M5 em pausa** — 1/6 consumers migrados; 5 bloqueados por lacunas #4–#7.
- **P169 desbloqueado** — começa M9 (features Introspection vanilla). Decisão sobre primeira feature em P169 `.A`.

API pública preservada (`layout(content, state)` continua a funcionar). Output observable inalterado. Sem ADR nova. Sem reservas.
