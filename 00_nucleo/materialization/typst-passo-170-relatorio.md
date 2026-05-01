# Relatório P170 — Hierarquia em CounterRegistry (M9 sub-passo 2)

Executado em 2026-04-30. Segunda feature M9 do refactor Introspection.

## Resumo

- **Feature escolhida em `.A`: hierarquia em `CounterRegistry`** (refactor isolado, resolve lacuna #5).
- Adicionados métodos a `CounterRegistry`: `apply_hierarchical(key, level)` (paridade com `CounterStateLegacy::step_hierarchical`) e `format(key) -> Option<String>` (paridade com `format_hierarchical`).
- Adicionado método trait `Introspector::formatted_counter(key) -> Option<String>` que delega para `counters.format`.
- `from_tags` arm Heading actualizado: usa `apply_hierarchical(_, depth)` em vez de `apply(_, Step)` flat — paridade com walk arm `Content::Heading` em introspect.rs:279 que faz `state.step_hierarchical("heading", level)`.
- **Lacuna #5 resolvida** em `m1-lacunas-captura.md` — `CounterRegistry` deixa de ser flat para Headings.
- **Sem Content variant novo**, sem ElementPayload variant novo — refactor localizado.

## Verificações .C

| # | Critério | Estado |
|---|----------|--------|
| 1 | `cargo check --workspace` passa | ✅ |
| 2 | `cargo test --workspace` — todos passam, contagem aumenta | ✅ **1 619 tests** (Δ +7 vs P169 = 1 612) |
| 3 | `crystalline-lint`: zero violations | ✅ "✓ No violations found" |
| 4 | Hierarquia em CounterRegistry materializada | ✅ |
| 5a | `apply_hierarchical(key, level)` existe + paridade exacta | ✅ |
| 5b | `format(key)` existe + retorna "1.2.3" para hierarquia | ✅ |
| 5c | `Introspector::formatted_counter` existe no trait + impl | ✅ |
| 5d | `from_tags` Heading usa `apply_hierarchical(_, depth)` | ✅ |
| 6 | Snapshot tests ADR-0033 verdes | ✅ |
| 7 | Linter passa final | ✅ |

Δ tests: +7 todas relativas a `CounterRegistry` hierarquia (4 apply_hierarchical + 3 format).

E2E test `introspector_consistencia_heading` actualizado para verificar paridade explícita: walk com [1,2,2,3] → `state.format_hierarchical("heading") == Some("1.2.1") == intr.formatted_counter("heading")`.

## Hashes finais

L0s actualizados:

| L0 | Hash anterior (P169) | Hash actual | `@prompt-hash` em L1 |
|----|----------------------|-------------|----------------------|
| `entities/counter_registry.md` | `bc222255` (P165) | `0cecccd2` | `b72b6747` |
| `entities/introspector.md` | `6a5a2a2e` (P169) | `ee0371c4` | `3192b0fe` |
| `rules/introspect/from_tags.md` | `c4e24523` (P169) | `d3c24085` | `9254a648` |

Sem L0 novo nem L1 novo — só extensão de tipos existentes.

## Decisões registadas em `.A`

### Feature escolhida: hierarquia em `CounterRegistry`

Avaliação caso a caso:

| Critério | state | CounterKey hier. | query | counter.at |
|----------|-------|------------------|-------|------------|
| Destrava lacuna | #4 (parcial) | **#5 (clara)** | #7 | parcial |
| Auto-contida | parcial | **✓ (refactor isolado)** | depende Selector | ✗ depende hier. |
| Pré-reqs satisfeitos | ✓ | **✓** | ✗ | ✗ |
| Magnitude | M-L | **M** | M | S-M (bloqueada) |
| Cascade risk | 7-14 arms novos (Content variants) | **0 arms novos** | tipo Selector novo | — |
| Bloqueia outras features | não | **destrava counter.at** | não | — |

**`CounterKey hierarquia`** ganha por:
1. **Cascade zero** — sem Content variants nem ElementPayload variants; refactor localizado a 3 ficheiros (CounterRegistry, Introspector trait+impl, from_tags arm).
2. **Lacuna clara fechada** (#5).
3. **Pré-requisitos satisfeitos** sem necessidade de criar tipos novos (Selector, Func-eval-context, etc.).
4. **Desbloqueia `counter.at(label)` / `counter.final()`** para passos futuros.

### Decisão local: `CounterKey` enum vs String key

Adoptada **String key** (sem enum literal). Vanilla `CounterKey::{Page, Selector(Selector), Str(Str)}` é cosmético quando só `Str` variant é usado em prática. `Page` e `Selector` ficam para passos futuros se algum consumer exigir (típico: rich `counter("name")` API user-facing).

### Decisão local: `counter_update` em `ElementPayload::Heading`

Mantido como `CounterUpdate` field, mas **ignorado** em from_tags arm Heading (depth é fonte autoritativa). Field documentado como "reservado/redundante para Heading; usado activamente para Figure". Alternativa (remover field) requereria mais cascada e menos preserva-padrão; mantém-se para coerência com Figure.

### Lacuna #5 — resolvida

`m1-lacunas-captura.md` actualizado com nota "✅ Resolvida em P170". Mecanismo:
- `CounterRegistry::apply_hierarchical(key, level)` paridade exacta com legacy.
- `CounterRegistry::format(key)` paridade exacta com legacy.
- `Introspector::formatted_counter(key)` exposição via trait.
- Verified by E2E test que compara `state.format_hierarchical("heading")` vs `intr.formatted_counter("heading")` para sequência [1,2,2,3] → `"1.2.1"`.

## Tests novos

- `apply_hierarchical_passa_de_vazio_para_um` — empty + level 1.
- `apply_hierarchical_sequencia_typica` — [1,2,2,3] → [1,2,1].
- `apply_hierarchical_subir_nivel_reseta_inferior` — [1,2] + 1 → [2].
- `apply_hierarchical_level_zero_clamped_para_um` — clamp.
- `format_devolve_string_pontuada` — "1.1.1".
- `format_inexistente_devolve_none`.
- `format_de_counter_flat_funciona_tambem` — flat counter format.

E2E `introspector_consistencia_heading` actualizado (assertion adicional via `formatted_counter`).

## Pendências cumulativas (M1+M2+M3+M4+M5+M9 P169-P170)

Lacunas em `m1-lacunas-captura.md`:

| # | Lacuna | Estado |
|---|--------|--------|
| 1 | `figure.kind` None vs "image" | Parcial (P168 figure-ref filter) |
| 2 | Auto-labels só em state | Adiar |
| 3 | Body frozen em state vs hash em tags | Manter (intencional) |
| 4 | `is_numbering_active` / `numbering_active` | Adiar — candidato `state(key, init)` em passo futuro |
| **5** | **`format_hierarchical` hierárquico** | **✅ Resolvida em P170** |
| 6 | `bib_entries` / `bib_numbers` | Adiar — M9+ feature dedicada |
| 7 | `has_outline` | Adiar — pode resolver via `query(Outline)` quando `query()` for materializada |

Outras pendências:
- Reshape opcional `CounterUpdate::Step` → `Step(usize)` (não relevante para hierarquia em P170).
- Refino opcional `hash_content`.
- `Position` ainda não materializado.
- `comemo::track` deferido (M7+).
- `MetadataStore` resolvido em P169.

## Estado de M9

**2/11 features materializadas**:
1. P169: `metadata(value)` — feature nova com Content variant + sub-store + stdlib.
2. P170: `CounterKey` hierarquia — refactor de tipo existente, lacuna #5 fechada.

Próximas candidatas:
- `state(key, init)` — destrava lacuna #4. Reutiliza padrão metadata mas mais complexo (StateRegistry + StateUpdate).
- `query()` user-facing — depende de tipo `Selector`.
- `here()` — depende de Locator::current() + EvalContext.
- `counter.at(label)` / `counter.final()` — agora **desbloqueada** por hierarquia P170; pode usar `LabelRegistry::lookup` + `CounterRegistry::value_at(key, location)` (precisa ainda de location-aware counter — futuro refino).

## Estado pós-passo

- **P170 concluído**.
- **M9 2/11 features**.
- **Lacuna #5 fechada**.
- **P171 desbloqueado** — terceira feature M9.

API pública preservada. Output observable inalterado. Sem ADR nova.
