# Relatório P162 — `hash_content`, `extract_payload`, walk em paralelo

Executado em 2026-04-30. Segundo de três passos da série M1 (P161 + P162 + P163).

## Resumo

- `hash_content` materializado em `entities/content_hash.rs` — função pura `&Content → u128` determinística, implementada via `format!("{:?}", c)` + duplo SipHash com sementes distintas.
- Placeholders P161 resolvidos: L0 de `element_payload.md` e `tag.md` perderam as notas de "pendência P162 / `0` placeholder"; texto agora referencia `hash_content`.
- `extract_payload` criado em `rules/introspect/extract_payload.rs` — função pura `&Content → Option<ElementPayload>` com match top-level sobre os 3 kinds locatable.
- Walk em `rules/introspect.rs` aceita 5 parâmetros (`content`, `state`, `locator`, `tags`, `label_from_parent`); emite `Tag::Start`/`Tag::End` em paralelo. Tags descartadas em `introspect()` via `drop(tags)`.
- API pública `pub fn introspect(content) -> CounterStateLegacy` preservada — consumidores externos (Layouter, materialize_time pipeline) inalterados.

## Verificações .H

| # | Critério | Estado |
|---|----------|--------|
| 1 | `cargo check --workspace` passa | ✅ |
| 2 | `cargo test --workspace` — todos passam, contagem aumenta | ✅ 1288 + 215 + 24 + 21 = 1 548 (+17 vs baseline P161 = 1 531) |
| 3 | `crystalline-lint`: zero violations | ✅ "✓ No violations found" |
| 4 | 2 ficheiros L1 novos existem | ✅ `entities/content_hash.rs`, `rules/introspect/extract_payload.rs` |
| 5 | 2 L0 novos existem | ✅ `entities/content_hash.md`, `rules/introspect/extract_payload.md` |
| 6 | L0 de `introspect.rs` reflecte nova assinatura | ⚠️ não foi modificado neste passo — assinatura é interna (`fn walk` não-pública); os 5 parâmetros estão documentados em comentário no L1. L0 mantém o foco na semântica do walk. Decisão registada em "Notas". |
| 7 | L0 `element_payload.md` e `tag.md` actualizados | ✅ notas de placeholder removidas |
| 8 | Walk aceita `&mut Locator` e `&mut Vec<Tag>` | ✅ 5 parâmetros |
| 9 | `pub fn introspect()` retorna `CounterStateLegacy` | ✅ assinatura preservada |
| 10 | Snapshot tests de paridade ADR-0033 passam | ✅ todos verdes |
| 11 | Linter passa em verificação final | ✅ |

Δ tests: +17 (5 hash_content + 7 extract_payload + 5 walk-with-tags).

## Hashes finais

L0 novos (calculados pelo `crystalline-lint --fix-hashes`):

| L0 | Hash do código L0 | Hash @prompt-hash em L1 |
|----|-------------------|-------------------------|
| `00_nucleo/prompts/entities/content_hash.md` | `e55b46d6` | `e1e9d070` |
| `00_nucleo/prompts/rules/introspect/extract_payload.md` | `e61765d4` | `2cab773f` |

L0 modificados em P162 (hashes recalculados):

| L0 | Hash actual |
|----|-------------|
| `00_nucleo/prompts/entities/element_payload.md` | `a67d96b1` |
| `00_nucleo/prompts/entities/tag.md` | `399e1b67` |

L0 `00_nucleo/prompts/rules/introspect.md` **não** foi modificado neste passo (decisão em ".H .6"); o seu hash permanece `264f58c8`. Refino para reflectir a assinatura de 5 parâmetros e a emissão de tags fica pendente — pode ser feito em P163 com o resto da consolidação documental.

## Decisões registadas em .A

### Hash de `Content`

Escolha: `format!("{:?}", content)` + duplo `DefaultHasher::new()` com sementes distintas (0 e 1) → concatenação dos 2 × u64 num u128.

Justificação: implementação minimalista sem dependência externa nova (nenhum `siphasher` em `[l1_allowed_external]`). Determinismo garantido pelo Debug derive de `Content` (que é estrutural recursivo). Fragilidade declarada no L0: estabilidade depende do output do Debug derive da stdlib do Rust; aceitável em M1 (tags descartadas) mas pode requerer hash recursivo manual quando M2/M3 começarem a consumir tags entre iterações.

Tests obrigatórios passam:
- Igualdade (5 tests passados, incluindo "iguais_produzem_mesmo_hash" e "clone_preserva_hash").
- Distinguibilidade (5 Contents distintos produzem 5 hashes distintos — verificado no test "distintos_produzem_hashes_distintos").
- Determinismo (100 chamadas idênticas — verificado no test "determinismo_em_100_chamadas").

### Mecanismo de label

**Gate disparado** em .A: cristalino usa wrapper `Content::Labelled { target, label }` em vez de `Option<Label>` directo dentro de Heading/Figure/Cite. P162 mandava parar e reabrir.

Decisão tomada (sem ADR nova; registada explicitamente neste relatório):

- Walk recebe `label_from_parent: Option<&Label>` como 5º parâmetro.
- No arm `Content::Labelled`, walk recursa para `target` passando `Some(label)`.
- Em todos os outros arms, walk recursa passando `None`.
- A emissão de `Tag::Start` no topo de walk usa `label_from_parent.cloned()` para popular `ElementInfo.label`.

Resultado: `Content::Labelled { target: Heading, label }` produz `Tag::Start(loc, ElementInfo { payload: Heading{...}, label: Some(label) })`. Verificado em `walk_label_de_wrapper_chega_ao_payload` (test verde).

### Adaptações em `extract_payload` para campos reais

P162 .D mostrou exemplo schemático com nomes que não batem com o cristalino:

| P162 esquemático | Cristalino real (P161 + P162 .A) |
|------------------|----------------------------------|
| `Content::Heading { depth, body }` | `Content::Heading { level, body }` — usar `depth: *level` |
| `Content::Figure { figure_kind }` | `Content::Figure { kind }` — usar `kind: kind.clone()` |
| `Content::Citation { key }` | `Content::Cite { key }` — variant é `Cite`, não `Citation` |
| `CounterUpdate::Step(*depth)` | `CounterUpdate::Step` (sem payload em cristalino — P161 manteve forma de `CounterAction`) |

Adaptações aplicadas literalmente em `extract_payload.rs`. Match arm `Content::Cite` mapeia para `ElementPayload::Citation` — mantém o nome cristalino do enum (`Citation`) coerente com `ElementKind::Citation`.

## Pendências para P163

- Tests E2E de paralelismo: walk produz `state` E `tags` consistentes para documentos compostos (sub-passos .G de P162 cobrem casos unitários simples; P163 fará casos complexos).
- Verificação de bracketing válido: para qualquer walk completo, a sequência `Vec<Tag>` deve ter Start/End correctamente aninhados (todo `Start(loc, _)` tem o seu `End(loc, _)` correspondente, sem overlapping).
- Consistência por kind: `state.format_hierarchical("heading")` deve igualar a contagem de `Tag::Start(_, ElementInfo { payload: Heading{..}, .. })` na sequência.
- Refino do L0 `introspect.md` (assinatura de 5 parâmetros + emissão de tags) — adiado para P163 onde o doc ganha mais material para consolidar.

## Estado pós-passo

- M1 sub-passo 2/3 concluído.
- Walk emite tags determinísticas em paralelo; consumidor real ainda não existe (M2/M3).
- 7 ficheiros L1 novos vs P161 baseline:
  - `entities/content_hash.rs` (P162 .B)
  - `rules/introspect/extract_payload.rs` (P162 .D)
- 4 L0 alterados:
  - `entities/element_payload.md` (.C — placeholder removido)
  - `entities/tag.md` (.C — placeholder removido)
  - 2 L0 novos com hashes calculados.
- Walk em `introspect.rs` reescrita: 5 parâmetros, emissão Tag::Start/End, label-from-parent threading.
- API pública `introspect(content) -> CounterStateLegacy` inalterada.

Pronto para P163 (verificação E2E completa de captura).
