# Relatório P166 — Expor `Introspector` ao caller (M4)

Executado em 2026-04-30. Passo único de M4 do refactor Introspection.

## Resumo

- Decisão **M4b** (entry point novo) tomada com base em inventário factual em `.A`.
- `pub fn introspect_with_introspector(content) -> (CounterStateLegacy, TagIntrospector)` adicionada em `01_core/src/rules/introspect.rs`.
- `pub fn introspect(content) -> CounterStateLegacy` passou a **wrapper** sobre `introspect_with_introspector` — descarta o `TagIntrospector`. Walk único subjacente (sem duplicação).
- API pública preservada: os ~38 call-sites identificados em `.A` continuam a compilar e funcionar sem alteração.
- L0 `00_nucleo/prompts/rules/introspect.md` actualizado — duas funções públicas documentadas.

## Inventário .A em formato literal

### Call-sites de `introspect()` — total ~38

| Localização | Quantidade |
|-------------|-----------:|
| `01_core/src/rules/introspect.rs` (tests) | 17 |
| `01_core/src/rules/layout/tests.rs` | 11 |
| `03_infra/src/integration_tests.rs` | 8 |
| `03_infra/src/layout.rs` (production) | 1 |
| `03_infra/src/pipeline.rs` (production) | 1 |
| **Total** | **38** |

Externos a 01_core (em 03_infra): 10 call-sites — API pública atravessa a fronteira inter-crate.

### Construtores de `CounterStateLegacy::new()`

| Localização | Tipo | Quantidade |
|-------------|------|-----------:|
| `01_core/src/entities/counter_state_legacy.rs` (tests do tipo) | tests | 17 |
| `01_core/src/rules/layout/mod.rs:144` (campo `Layouter.counter`) | production | 1 |
| **Total** | | **18** |

### Uso de retorno

Padrão dominante em todos os call-sites: `let state = introspect(&content); … layout(&content, state)` ou `… use state.format_hierarchical(...)`. Read-only puro. Zero call-sites mutam o retorno.

### Padrão cristalino para output múltiplo

Tuples já são usados em testes (helper `introspect_with_tags(content) -> (CounterStateLegacy, Vec<Tag>)` em P162.G). Adoptado.

### Estado pré-P166

`pub fn introspect()` em P165 construía `_introspector = from_tags(&tags)` e descartava-o (linha 53 antes de P166).

## Aplicação das regras de decisão

| Critério | Resultado | Implicação |
|----------|-----------|------------|
| Call-sites > 10 | ✓ (38) | M4a com custo alto |
| API externa pública (10 usos em 03_infra) | ✓ | M4a quebra inter-crate |
| Construtor de `CounterStateLegacy` em production | 1 (Layouter::new) | M4c precisaria modificar Layouter |
| Construtores em tests | 17+ | M4c precisaria `introspector: None` em todos |
| Mut consumer | 0 | M4a tuple seria viável |

**M4b** escolhida: minimal disruption, walk único subjacente, gradual migration path para M5.

## Verificações .D

| # | Critério | Estado |
|---|----------|--------|
| 1 | `cargo check --workspace` passa | ✅ |
| 2 | `cargo test --workspace` — todos passam, contagem aumenta | ✅ **1 593** (Δ +3 vs P165 = 1 590) |
| 3 | `crystalline-lint`: zero violations | ✅ "✓ No violations found" |
| 4 | M4b implementado: `introspect_with_introspector` existe; legacy preservada | ✅ |
| 5 | L0 `rules/introspect.md` reflecte novo estado | ✅ Hash `aecff834` → `b988ff3d` |
| 6 | (n/a — apenas M4c) | — |
| 7 | Walk não modificado (lógica de emissão preservada) | ✅ Body de `walk` inalterado |
| 8 | Snapshot tests ADR-0033 verdes | ✅ |
| 9 | Linter passa final | ✅ |

Δ tests: +3:
- `introspect_with_introspector_devolve_par` — entry point novo retorna par.
- `introspect_legacy_continua_a_funcionar` — backward compat.
- `introspect_e_introspect_with_introspector_produzem_mesmo_state` — wrapper consistente com entry point novo.

## Hashes finais

L0 modificado:

| L0 | Hash anterior | Hash actual | `@prompt-hash` em L1 |
|----|---------------|-------------|----------------------|
| `rules/introspect.md` | `aecff834` (P165) | `b988ff3d` | `4d084de6` |

Nenhum L0 novo. Nenhum L1 novo (apenas modificação de `rules/introspect.rs`).

## Decisão de optimização local (per `.B.M4b` step 4)

`introspect()` foi tornada **wrapper** sobre `introspect_with_introspector` (em vez de ter walks independentes). Razão: walk único é mais barato; os 28 call-sites internos a 01_core não pagam mais por chamar `introspect()` legacy. Os 10 call-sites externos a 03_infra podem migrar gradualmente para `introspect_with_introspector` em M5 quando precisarem do introspector.

## Pendências para M5

- **Migração de consumers**: 38 call-sites continuam a chamar `introspect()`. M5 escolhe primeiro consumer real e migra para `introspect_with_introspector`. Candidatos prováveis (ordem de impacto):
  - `01_core/src/rules/layout/` — Layouter pode começar a consumir `Introspector::query_by_label` em vez de `state.resolved_labels` para refs.
  - `03_infra/src/pipeline.rs` — pipeline central; migração aqui torna o introspector visível downstream.
- **Scope decision**: M5 pode optar por migrar apenas 1 consumer (e validar que tudo funciona) ou múltiplos em paralelo. Decisão depende da complexidade da migração.

## Pendências cumulativas (M1+M2+M3+M4)

- 3 divergências em `m1-lacunas-captura.md` (figure.kind, auto-labels, body frozen).
- Reshape opcional `CounterUpdate::Step` → `Step(usize)`.
- Refino opcional `hash_content`.
- `Position` ainda não materializado (M5/M9).
- `comemo::track` deferido (M7+).
- `CounterKey` enum vanilla deferido (M9).
- `MetadataStore` adiado (M9).
- **Novas em M4**: nenhuma. Decisão M4b foi minimal sem introduzir novas pendências.

## Estado pós-passo

**M4 concluído** (passo único P166). Mecanismo de exposição existe via `introspect_with_introspector`. API pública preservada — call-sites legacy continuam a funcionar. Walk único subjacente serve ambos os entry points sem duplicação.

**M5 desbloqueado** — pode começar migração do primeiro consumer real para `Introspector` (provavelmente layout ou pipeline central).

API pública preservada. Sem ADR nova. Sem reservas.
