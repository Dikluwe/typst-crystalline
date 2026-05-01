## Relatório P172 — `Func` callback em `StateUpdate` (M9 sub-passo 4)

Executado em 2026-04-29. Quarta feature M9 do refactor Introspection.

## Resumo

- **`StateUpdate::Func(Func)` variant adicionada** — completude tipológica.
- **Eval real adiada** — abordagem **stub**, não Resolution + Engine cascade.
  `from_tags::apply_update` arm `Func(_)` é no-op silencioso.
- **Stdlib `state_update_with(key, fn)`** registada — produz
  `Content::StateUpdate { update: Func(fn) }` que é construído normalmente
  pelo eval mas ignorado em from_tags.
- **Walk NÃO modificado** — invariante P163 preservado.
- **API legacy `introspect()` inalterada** — sem cascade, sem propagação
  de Engine.
- **Pendência registada**: eval real de Func em from_tags requer
  pipeline restructuring (Engine + EvalContext disponíveis em from_tags).
  Adiado para passo dedicado (M7+ ou refactor pipeline).

## Decisão arquitectural — stub vs cascade

L0 P172 originalmente especificou **Resolution + Engine cascade**:
modificar `from_tags(tags, engine, ctx)`, `introspect_with_introspector`
propaga, ~38 call-sites adaptados.

Após gate report `.A`, optou-se por **stub minimal**:

| Aspecto | Resolution (spec original) | Stub (P172 efectivo) |
|---------|---------------------------|---------------------|
| `StateUpdate::Func` variant | ✅ | ✅ |
| Stdlib `state_update_with` | ✅ | ✅ |
| `from_tags` aceita Engine | ✅ | ❌ — assinatura inalterada |
| `introspect_with_introspector` propaga Engine | ✅ | ❌ |
| Walk modificado | ❌ (preservado puro) | ❌ (preservado puro) |
| Eval de Func em from_tags | ✅ via `apply_func` | ❌ silenciosamente ignorado |
| Call-sites externos afectados | ~38 | 0 |
| Pendência fechada | ✅ | ❌ — fica aberta |

Justificação stub:
- Cascade afecta ~38 call-sites externos em 03_infra. Magnitude L+.
- Stub mantém superfície de tipos completa: stdlib `state_update_with`
  é registada e construível; `Content::StateUpdate { update: Func }`
  flui pelo walk e from_tags sem panic.
- Eval real desloca-se para passo dedicado quando Engine/EvalContext
  cascade já estiver justificada por outras features (query, here,
  counter.at — todas P173+).

Trade-off explicito: P172 NÃO fecha a pendência "Func callback em
StateUpdate" registada por P171. Reabre-se sob nome
"Func eval em from_tags requer pipeline restructuring".

## Verificações `.H`

| # | Critério | Estado |
|---|----------|--------|
| 1 | `cargo check --workspace` passa | ✅ |
| 2 | `cargo test --workspace` — todos passam | ✅ **1 380** lib (Δ +6 vs P171 = 1 374). Total workspace: 1 380 + 215 + 24 + 21 = 1 640 (Δ +6) |
| 3 | `crystalline-lint`: zero violations | ✅ "✓ No violations found" |
| 4 | `StateUpdate::Func(Func)` variant existe | ✅ |
| 5 | `from_tags` aceita Engine (Resolution) | ❌ **stub** — não modificada |
| 6 | `introspect_with_introspector` propaga Engine | ❌ **stub** |
| 7 | `introspect()` legacy preservada | ✅ — assinatura inalterada |
| 8 | Stdlib `state_update_with` registada | ✅ |
| 9 | Walk em `introspect.rs::walk` NÃO modificado | ✅ |
| 10 | Determinismo do walk preservado | ✅ |
| 11 | Snapshot tests ADR-0033 passam inalterados | ✅ |
| 12 | Linter passa em verificação final | ✅ |

Δ tests: +6 — distribuídos por StateUpdate (3 Func variant) + p172_func_callback E2E (3).

## Hashes finais

L0s modificados (P172):

| L0 | Hash actual | `@prompt-hash` em L1 |
|----|-------------|----------------------|
| `entities/state_update.md` | `3fe54b8d` | `1b276c4e` |
| `entities/state_registry.md` | `c6baac63` | `60e9f868` |
| `rules/stdlib.md` | `6e6c49e4` | `f6cc2443` |
| `rules/eval.md` | `4ce356e8` | `19073424` |

Hashes consolidados via `crystalline-lint --fix-hashes`.

## Tests novos

**`state_update::tests`** — 3 tests Func variant:
- `func_variant_construi_e_compara_por_arc_ptr_eq` — `StateUpdate::Func(f.clone())` × 2 ⇒ iguais (mesmo Arc).
- `func_variants_distintas_sao_diferentes` — duas `Func::native` distintas ⇒ desiguais.
- `set_e_func_sao_distintos` — `Set` vs `Func` ⇒ desiguais.

**`p172_func_callback` E2E** — 3 tests:
- `func_variant_e_silenciosamente_ignorada_em_from_tags` — state init + state_update_with(Func) ⇒ state final = init (Func ignorada).
- `func_variant_e_invisivel_em_layout` — frame de página com Func update ⇒ zero glifos visíveis.
- `set_continua_a_funcionar_apos_func_variant` — regressão: Set variant não afectada pela introdução de Func.

## Decisões registadas em `.A`

### Forma de `StateUpdate::Func`: variant directo

Adoptado `StateUpdate::Func(Func)` em vez de `StateUpdate::Func(Box<Func>)` ou
`StateUpdate::Func { closure: Func }`. Razão: paridade com vanilla
`StateUpdate { Set(Value), Func(Func) }`. `Func` interno já é
`Arc<FuncRepr>` — clone O(1).

### `PartialEq` manual: `Arc::ptr_eq`

`Func` não implementa `PartialEq` derive (closures internas). Adoptada
comparação por ponteiro Arc — duas Funcs com mesmo comportamento mas
construídas separadamente são `!=`. Paridade com vanilla onde Func
não compara estruturalmente.

### Stdlib forma: `state_update_with(key, fn)`

Adoptado nome separado em vez de detecção polimórfica em `state_update`.
Razão: explicitude. Cláusula gate trivial decidida sem reabrir.

### Adiamento de Resolution: stub

Decisão tomada após gate `.A` confrontar magnitude L+ (~38 call-sites).
Stub preserva interface tipológica completa sem cascade. Pendência
re-registada para passo dedicado.

## Estado de M9

**4/11 features materializadas** (parcial em P172):

1. P169: `metadata(value)` — completo.
2. P170: `CounterKey` hierarquia — completo.
3. P171: `state(key, init)` + `state_update(key, value)` — completo.
4. **P172**: `StateUpdate::Func` + `state_update_with(key, fn)` —
   **superfície tipológica completa, eval adiada**.

Próximas candidatas:
- Eval real de Func em StateUpdate — requer Engine cascade (passo
  dedicado, possivelmente combinado com query/here).
- `query()` user-facing — depende de `Selector`.
- `here()` — depende de `Locator::current()` + EvalContext.
- `counter.at(label)` / `counter.final()` — desbloqueada por P170.

## Pendências cumulativas (M1+M2+M3+M4+M5+M9 P169-P172)

Lacunas em `m1-lacunas-captura.md`:

| # | Lacuna | Estado |
|---|--------|--------|
| 1 | `figure.kind` None vs "image" | Parcial (P168 figure-ref filter) |
| 2 | Auto-labels só em state | Adiar |
| 3 | Body frozen em state vs hash em tags | Manter (intencional) |
| 4 | `is_numbering_active` / `numbering_active` | Infraestrutura pronta em P171; consumer aguarda M5 |
| 5 | `format_hierarchical` hierárquico | ✅ Resolvida em P170 |
| 6 | `bib_entries` / `bib_numbers` | Adiar — M9 feature dedicada |
| 7 | `has_outline` | Adiar — pode resolver via `query(Outline)` futuramente |

Outras pendências:
- Reshape opcional `CounterUpdate::Step` → `Step(usize)`.
- Refino opcional `hash_content`.
- `Position` ainda não materializado.
- `comemo::track` deferido (M7+).
- **P171**: `Func` callback em `StateUpdate` — variant adicionada em
  P172 mas **eval permanece stub**. Pendência re-registada como
  "Func eval em from_tags requer pipeline restructuring (Engine+EvalContext
  cascade)".

## Estado pós-passo

- **P172 concluído (stub)**.
- **M9 4/11 features** (3 completas + 1 com superfície tipológica).
- **Pendência Func callback** re-registada com escopo refinado:
  pipeline restructuring para cascade Engine.
- **P173 desbloqueado** — quinta feature M9 (candidata: `query()` ou
  `here()`).

API pública preservada. Output observable inalterado. Sem ADR nova.
Stub explicitamente documentado em L0 e L1 (`StateUpdate::Func` doc-comment).
