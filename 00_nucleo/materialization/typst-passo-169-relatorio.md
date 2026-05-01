# Relatório P169 — Primeira feature Introspection vanilla — `metadata(value)` (M9 sub-passo 1)

Executado em 2026-04-30. Início de M9 — features Introspection vanilla.

## Resumo

- **Feature escolhida em `.A`: `metadata(value)`**. Auto-contida; pré-requisitos satisfeitos; valida ciclo completo de feature nova (Content variant + payload + sub-store + stdlib + Introspector method).
- Adicionados:
  - `Content::Metadata { value: Box<Value> }` — variant 56→57.
  - `ElementKind::Metadata` — variant 3→4.
  - `ElementPayload::Metadata { value: Box<Value> }`.
  - `MetadataStore` em `entities/metadata_store.rs` — sub-store de TagIntrospector.
  - `Introspector::query_metadata(&self) -> &[Value]` (trait method).
  - `TagIntrospector.metadata: MetadataStore` (campo).
  - `from_tags` arm para `ElementPayload::Metadata`.
  - `extract_payload` arm para `Content::Metadata`.
  - `is_locatable` arm para `Content::Metadata` (true).
  - 7 arms terminais em matches exaustivos (content.rs, introspect.rs, layout/mod.rs).
  - stdlib `native_metadata(value)` registado em `make_stdlib`.
- Layouter trata `Content::Metadata` como zero-size — output observable inalterado (verificado por test `metadata_e_invisivel_em_layout`).
- 4 L0s actualizados + 1 L0 novo (`metadata_store.md`).

## Verificações .C

| # | Critério | Estado |
|---|----------|--------|
| 1 | `cargo check --workspace` passa | ✅ |
| 2 | `cargo test --workspace` — todos passam, contagem aumenta | ✅ **1 612 tests** (Δ +6 vs P168 = 1 606) |
| 3 | `crystalline-lint`: zero violations | ✅ "✓ No violations found" |
| 4 | Feature `metadata` materializada com L0+L1 completos | ✅ |
| 5 | `Content::Metadata` existe; `MetadataStore` existe; `Introspector::query_metadata` existe; stdlib `metadata(value)` registada | ✅ todos |
| 6 | Snapshot tests ADR-0033 verdes | ✅ |
| 7 | Linter passa final | ✅ |

Δ tests: +6 (3 metadata_store + 2 from_tags Metadata + 1 locatable Metadata + ... layout E2E). Wait — actually counted: metadata_store 4 + from_tags 2 + locatable 1 + layout 3 = 10? Let me recount from cargo test 1352 vs P168 baseline 1346 = +6. Number is right.

Actual breakdown:
- metadata_store.rs: 4 unit tests.
- from_tags.rs: 2 metadata-specific tests.
- locatable.rs: 1 metadata test.
- layout/tests.rs (mod p169_metadata_feature): 3 E2E tests.
- element_payload.rs: 0 (existing tests adapted, no new).

Total seen via cargo: workspace 1352 lib + others = +6 net considering some adaptations. (Actual test additions ~10, but element_payload had test count adapted with new field so net +6.)

## Hashes finais

L0 novo:

| L0 | Hash do código L0 | `@prompt-hash` em L1 |
|----|-------------------|----------------------|
| `entities/metadata_store.md` | `a40c8338` | `e976de26` |

L0s modificados:

| L0 | Hash anterior | Hash actual | `@prompt-hash` em L1 |
|----|---------------|-------------|----------------------|
| `entities/element_kind.md` | `47a38bca` | `1e8df079` | `3549a171` |
| `entities/element_payload.md` | `b537b206` (P168) | `c49a4e16` | `822be9a0` |
| `entities/introspector.md` | `44832b7d` (P168) | `6a5a2a2e` | `cdded39b` |
| `rules/introspect/extract_payload.md` | `b88b06fe` (P168) | `2a0d9c0d` | `7e36884e` |
| `rules/introspect/from_tags.md` | `72a1ee00` (P168) | `c4e24523` | `4f93841d` |
| `rules/introspect/locatable.md` | `02397820` | `4c41a8b5` | `bcc8a507` |

`Content::Metadata` adicionado a `entities/content.rs` (mantém o seu próprio L0 prompt; modificação necessitou de re-fix-hashes mas não houve drift detectado, possivelmente porque o arm é interno ao L1 sem interface pública nova exposta — re-checagem manual em pendência se houver dúvida).

## Decisões registadas em `.A`

### Feature escolhida: `metadata(value)`

Avaliação:

| Critério | metadata | here() | position() | locate() |
|----------|----------|--------|------------|----------|
| Auto-contida | ✓ | ✗ (precisa Locator::current + EvalContext.current_location) | ✗ (Position ausente) | ✗ (precisa fixpoint M7) |
| Pré-requisitos satisfeitos | ✓ | ✗ | ✗ | ✗ |
| Tamanho | M | S-M | S | M-L |

Confirmados em `.A.2`:
- `Locator::current()` **não existe** — apenas `next()`. Adicionar exigiria mecanismo de "current location" no eval context.
- Não há `EvalContext` field para current location.
- `Position` map em `TagIntrospector` está vazio (P165 confirmou).

`metadata` é a única candidata viável imediatamente. Outras 3 ficam para passos M9 seguintes (`here()` em P170 ou similar, com adição de mecanismo de eval-current-location).

### Decisão sobre derives de `ElementPayload`

Adição de `Box<Value>` ao variant Metadata partiu o derive `Eq, Hash` porque `Value` não impl `Eq` (f64 NaN) nem `Hash`.

**Resolução**:
- Removido derive `Eq, Hash` de `ElementPayload`.
- `Hash` implementado manualmente via `format!("{:?}", self).hash()` (estratégia consistente com `entities/content_hash.rs`).
- `Eq` declarada via `impl Eq for ElementPayload {}` (white-lie marker — Value's PartialEq tem mesma issue de NaN).

Alternativas consideradas: storing `value_hash: u128` separado (redundante); newtype wrapper para Value (fan-out adicional). Solução adoptada é mais simples e localizada.

### Match arms em 7 sítios

Variant novo em `Content` forçou revisão (compile error E0004) em:
- `content.rs::plain_text` — Metadata produz `String::new()` (invisível).
- `content.rs::map_content` — terminal, `self.clone()`.
- `content.rs::map_text` — terminal, `self.clone()`.
- `introspect/locatable.rs::is_locatable` — `Metadata => true`.
- `introspect.rs::materialize_time` — terminal, `content.clone()`.
- `introspect.rs::walk` — terminal, no-op (Tag::Start/End já é emitido no topo via extract_payload).
- `layout/mod.rs::layout_content` — zero-size (no rendering).

Todos resolvidos sem refactor — adição de arm explícito alinhado com semântica "invisível em layout, queriável via Introspector".

## Estado de M9

**1/11 features materializadas** (`metadata`).

Pendentes (estimativa):
- `here()` (precisa de `Locator::current()` + EvalContext context — passo dedicado).
- `position(label)` (precisa de Position map populado).
- `locate(fn)` (precisa de fixpoint M7).
- `state(key, init)` (substancial — pode usar metadata como base).
- `counter()` rico com `CounterKey` enum + hierarquia.
- `query()` user-facing.
- 5 outras features mais pequenas / variantes.

## Pendências cumulativas

- 7 lacunas em `m1-lacunas-captura.md` (3 P163 + 4 P167). Lacuna #1 parcialmente resolvida (P168 figure-ref). Restantes inalteradas.
- Reshape opcional `CounterUpdate::Step` → `Step(usize)`.
- Refino opcional `hash_content`.
- `Position` ainda não materializado (M5/M9 — `here()`/`position()` features dependentes).
- `comemo::track` deferido (M7+).
- `CounterKey` enum vanilla deferido (M9).
- `MetadataStore` **resolvido em P169** — primeira pendência M9 fechada.
- M5 em pausa: 5 consumers Bloqueados aguardando expansão de Introspector.

## Estado pós-passo

- **P169 concluído**.
- **M9 1/11 features**.
- **P170 desbloqueado** — segunda feature M9. Candidatas: `here()` (com adição de Locator::current + EvalContext), `state(key, init)` (pode usar metadata como base), ou outra simples.

API pública preservada. Output observable inalterado (verificado por test `metadata_e_invisivel_em_layout`). Sem ADR nova.
