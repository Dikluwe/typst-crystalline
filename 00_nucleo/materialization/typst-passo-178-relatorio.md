## Relatório P178 — `ElementKind::Outline` cascade (lacuna #7 fecha)

Executado em 2026-04-29. Refino arquitectural — não é feature stdlib, mas completa lacuna #7 parcialmente resolvida em P175.

## Resumo

- **`ElementKind::Outline`** — variant 7 adicionada.
- **`ElementPayload::Outline`** — unit variant (Opção α). Refino futuro pode capturar `depth`/`title_hash`.
- **`is_locatable(Content::Outline) == true`** — movido de or-pattern não-locatable para arm dedicado `=> true`.
- **`extract_payload(Content::Outline) -> Some(ElementPayload::Outline)`** — arm novo antes do `_ => None` fall-through.
- **`from_tags` arm** popula `kind_index[Outline]` com Location.
- **`ElementKind::from_name("outline") -> Some(Outline)`** — extensão do helper P175.
- **Stdlib `query("outline")`** (P175) agora retorna count correcto. Equivalente a `has_outline := query("outline") > 0`.
- **Walk inalterado** — usa `extract_payload` automaticamente. `Content::Outline` variant **NÃO modificado**.

## Verificações `.I`

| # | Critério | Estado |
|---|----------|--------|
| 1 | `cargo check --workspace` passa | ✅ |
| 2 | `cargo test --workspace` — todos passam | ✅ **1 438** lib (Δ +9 vs P177 = 1 429). Total workspace: 1 438 + 215 + 24 + 21 = **1 698** (Δ +9) |
| 3 | `crystalline-lint`: zero violations | ✅ |
| 4 | `ElementKind::Outline` existe | ✅ |
| 5 | `ElementPayload::Outline` existe | ✅ unit variant |
| 6 | `is_locatable(&Content::Outline) == true` | ✅ |
| 7 | `extract_payload(&Content::Outline) -> Some` | ✅ |
| 8 | `from_tags` arm para Outline popula `kind_index` | ✅ |
| 9 | `ElementKind::from_name("outline")` retorna `Some(Outline)` | ✅ |
| 10 | Walk em `introspect.rs::walk` NÃO modificado | ✅ |
| 11 | `Content::Outline` variant NÃO modificado | ✅ |
| 12 | Lacuna #7 actualizada em `m1-lacunas-captura.md` | ✅ |
| 13 | Snapshot tests ADR-0033 verdes | ✅ |
| 14 | Linter passa final | ✅ |

Δ tests: +9 — distribuídos por:
- `element_kind::tests`: +3 (outline_existe_e_distinto, outline_as_str, from_name_outline).
- `element_payload::tests`: +1 (outline_e_unit_e_distinto_de_outras).
- `extract_payload::tests`: +1 (outline_produz_some_payload).
- `stdlib::tests`: +1 (stdlib_query_outline_funciona_pos_p178).
- `fixpoint::tests`: +3 (P178 outline_locatable_e_indexavel, query_outline_doc_sem_outline, query_outline_doc_com_outline).
- `fixpoint::tests`: -1 (`p175_lacuna_7_outline_kind_ausente` removido — codificava invariante incorrecto pós-P178; substituído por `p178_lacuna_7_outline_kind_resolvida`).

Net: +9 = +3 + 1 + 1 + 1 + 3 + 1 (replaced).

## Hashes finais

L0s modificados (P178):

| L0 | Hash actual | `@prompt-hash` em L1 |
|----|-------------|----------------------|
| `entities/element_kind.md` | `c9b77b3b` | `a807a96a` |
| `entities/element_payload.md` | `73291466` | `f0a2159c` |

L0s não-modificados afectados (mudanças em L1 sem mudar L0):
- `rules/introspect/locatable.md` (cascade — Outline movido).
- `rules/introspect/extract_payload.md` (arm novo).
- `rules/introspect/from_tags.md` (arm novo).
- `rules/stdlib.md` (query agora aceita "outline").

Diagnóstico actualizado:
- `00_nucleo/diagnosticos/m1-lacunas-captura.md` — lacuna #7 marcada como ✅ Resolvida em P178.

## Tests novos

**`element_kind::tests`** — 3:
- `outline_existe_e_distinto`.
- `outline_as_str` — "outline".
- `from_name_outline` — round-trip.

**`element_payload::tests`** — 1:
- `outline_e_unit_e_distinto_de_outras`.

**`extract_payload::tests`** — 1:
- `outline_produz_some_payload`.

**`stdlib::tests`** — 1:
- `stdlib_query_outline_funciona_pos_p178` — vazio (0) e populado (1).

**`fixpoint::tests`** — 3 (P178 E2E):
- `p178_outline_locatable_e_indexavel` — 1 Outline → kind_index[Outline] = 1.
- `p178_query_outline_doc_sem_outline` — heading-only doc → 0.
- `p178_query_outline_doc_com_outline` — sequência com Outline → 1.

**Removido**:
- `p175_lacuna_7_outline_kind_ausente` (substituído por `p178_lacuna_7_outline_kind_resolvida`).

## Decisões registadas em `.A`

### Forma de `ElementPayload::Outline`: **Opção α** (unit)

`Content::Outline` é unit variant — sem campos a capturar. Payload unit espelha esta forma. Suficiente para `query("outline").len() > 0` (predicado `has_outline`).

Refino futuro: se consumer real precisar de `depth` ou `title`, capturar `Content::Outline` campos quando este for promovido em vanilla cristalino.

### Sem cascade ~9 sítios

Diferença face a P169 (Metadata): P178 NÃO adiciona variant novo a `Content` — `Content::Outline` já existia. Modifica apenas:
1. `is_locatable` arm (or-pattern → arm dedicado).
2. `extract_payload` arm (antes do fall-through).
3. `ElementKind` (variant nova).
4. `ElementPayload` (variant nova).
5. `from_tags` arm.
6. `ElementKind::from_name` (string parser).

Outros arms exhaustive de `Content` (em layout, plain_text, materialize_time, etc.) já tinham decisão tomada para `Content::Outline` — não são afectados.

### Test stub P175 substituído

`p175_lacuna_7_outline_kind_ausente` codificava invariante "Outline NÃO em ElementKind" (P175 documento estado da época). P178 inverte essa invariante — test substituído por `p178_lacuna_7_outline_kind_resolvida` que afirma o oposto.

## Estado de M9

**8/11 features materializadas** (Outline conta como feature/refino arquitectural):
1. P169: `metadata(value)` — completa.
2. P170: `CounterKey` hierarquia — completa.
3. P171: `state(key, init)` + `state_update(key, value)` — completa.
4. P172+P173: `state_update_with(key, fn)` — completa.
5. P175: `query(selector)` minimal — completa.
6. P176: `counter.final(key)` minimal — completa.
7. P177: `counter.at(label)` minimal — completa.
8. **P178: Outline cascade — lacuna #7 fechada**.

Próximas candidatas:
- `here()` — precisa `Locator::current()` + `EvalContext.current_location`. Magnitude M.
- `locate(callback)` — depende de `Position`. Magnitude desconhecida.
- Bib state (lacuna #6) — magnitude desconhecida.
- `query` upgrade (Locations/Content) — refino de P175. Magnitude S.

## Estado de M7

Sem mudança. Mecanismo + 3 clientes (P175/P176/P177). P178 é refino arquitectural, não cliente fixpoint.

## Pendências cumulativas

Lacunas em `m1-lacunas-captura.md`:

| # | Lacuna | Estado |
|---|--------|--------|
| 1 | `figure.kind` None vs "image" | Parcial (P168) |
| 2 | Auto-labels só em state | Adiar |
| 3 | Body frozen em state vs hash em tags | Manter (intencional) |
| 4 | `is_numbering_active` / `numbering_active` | Infraestrutura pronta P171; consumer aguarda M5 |
| 5 | `format_hierarchical` hierárquico | ✅ Resolvida em P170 |
| 6 | `bib_entries` / `bib_numbers` | Adiar — M9 feature dedicada |
| 7 | `has_outline` | ✅ **Resolvida em P178** (`query("outline")`) |

Outras pendências (sem alteração):
- Reshape opcional `CounterUpdate::Step` → `Step(usize)`.
- Refino opcional `hash_content`.
- `Position` ainda não materializado.
- Erros de Func eval em from_tags propagam silenciosamente.
- `run_fixpoint` sem optimização early-exit.
- `Value::Location` ausente.
- Selector apenas `Kind` variant.
- CounterRegistry tem dual API (com/sem history).
- **Nova em P178**: `ElementPayload::Outline` é unit. Refino futuro pode capturar `depth`/`title_hash` para queries mais ricas.

## Estado pós-passo

- **P178 concluído**.
- **M9 8/11 features**.
- **Lacuna #7 fechada**.
- **P179 desbloqueado** — feature seguinte M9 (candidata: `here()` ou `locate()`).

API pública preservada. Output observable inalterado em snapshot tests. `Content::Outline` variant intocada — apenas funções que matcham sobre ele foram modificadas. Walk puro. Sem ADR nova. Sem reservas.
