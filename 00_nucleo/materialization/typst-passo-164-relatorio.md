# Relatório P164 — `is_locatable` como função pública (M2)

Executado em 2026-04-30. Passo único de M2 do refactor Introspection.

## Resumo

- `is_locatable(content: &Content) -> bool` extraído como função pura em `01_core/src/rules/introspect/locatable.rs`.
- Match **exaustivo** sobre os 56 variants de `Content` — sem `_ => false` fall-through. 3 variants em `true` (Heading, Figure, Cite); 53 variants em `false`.
- Invariante `is_locatable(c) == extract_payload(c).is_some()` verificado por test exhaustivo sobre representantes de cada bucket.
- Walk em `rules/introspect.rs` **não modificado** — apenas adicionada a declaração `pub mod locatable;` em paralelo a `pub mod extract_payload;`. Body da função walk inalterado.
- API pública preservada. Output observable inalterado.

## Verificações .C

| # | Critério | Estado |
|---|----------|--------|
| 1 | `cargo check --workspace` passa | ✅ |
| 2 | `cargo test --workspace` — todos passam, contagem aumenta vs P163 | ✅ **1 304 + 215 + 24 + 21 = 1 564** (Δ +9 vs P163 = 1 555) |
| 3 | `crystalline-lint`: zero violations | ✅ "✓ No violations found" |
| 4 | L0 `locatable.md` existe com cabeçalho correcto | ✅ Hash `02397820` |
| 5 | L1 `locatable.rs` existe com `pub fn is_locatable` | ✅ |
| 6 | `is_locatable` re-exportado | ✅ via `pub mod locatable;` em `rules/introspect.rs` (mesma forma do `extract_payload`) |
| 7 | Walk em `rules/introspect.rs` não modificado (body) | ✅ apenas declaração `pub mod locatable;` adicionada; lógica do walk inalterada |
| 8 | Snapshot tests ADR-0033 verdes | ✅ |
| 9 | Linter passa em verificação final | ✅ |

Δ tests: +9 (3 cobertura locatable + 5 cobertura não-locatable + 1 invariante exaustivo).

## Hashes finais

L0 novo:

| L0 | Hash do código L0 | @prompt-hash em L1 |
|----|-------------------|--------------------|
| `00_nucleo/prompts/rules/introspect/locatable.md` | `02397820` | `e512f448` |

## Decisões registadas em .A

### Lista exaustiva de variants

`Content` tem **56 variants** (confirmado por `awk` sobre `entities/content.rs`).

**Locatable (3)**: `Heading`, `Figure`, `Cite`.

**Não-locatable (53)**: Empty, Text, Space, Sequence, Raw, ListItem, EnumItem, Link, Equation, MathSequence, MathIdent, MathText, MathFrac, MathAttach, MathRoot, MathDelimited, MathAlignPoint, Linebreak, MathMatrix, MathCases, Labelled, Ref, SetHeadingNumbering, CounterDisplay, CounterUpdate, Outline, SetFigureNumbering, Image, Shape, Transform, Grid, SetPage, Align, Place, Styled, Divider, Terms, TermItem, Quote, Pad, Hide, HSpace, VSpace, Pagebreak, Stack, Boxed, Block, TableCell, Bibliography, TableHeader, TableFooter, Table, Repeat.

Sem variants novos entre M1 e M2 — gate trivial não disparou.

### Forma do match em `is_locatable`

**Match exaustivo** com `=> true` para os 3 locatable e or-pattern `Empty | Text(_, _) | Space | … | Repeat { .. } => false` agrupando os 53 não-locatable. Sem `_ => false` fall-through. Razão arquitectural: compilador força revisão quando variant novo é adicionado a `Content` — ambos `extract_payload` e `is_locatable` deverão ser editados em coordenação.

`extract_payload`, em contraste, mantém `_ => None` (P162.D). Discrepância intencional: `extract_payload` produz `Option<ElementPayload>` cuja construção é trabalhosa (chama `hash_content`), justificando o catch-all conveniente. `is_locatable` é classificação binária pura — adoptar match exaustivo aqui é gratis e ganha-se a propriedade de exhaustivity-checking. Ambos os ficheiros referenciam-se mutuamente nos comentários para que a coordenação seja explícita.

### Mecanismo de re-export

P164.B step 3 mencionou "update `01_core/src/rules/introspect/mod.rs`". Esse ficheiro **não existe** — a estrutura usada desde P162.D é `rules/introspect.rs` (módulo pai como ficheiro) + `rules/introspect/<sub>.rs` (submódulos como ficheiros irmãos). Adicionei `pub mod locatable;` a `rules/introspect.rs` em paralelo à declaração existente `pub mod extract_payload;`. Forma equivalente; sem necessidade de criar `mod.rs` redundante.

## Estado pós-passo

**M2 concluído** (passo único P164).

`is_locatable` está disponível como utilitária para consumers futuros:
- M3 `Introspector::from_tags` — pode consultar para classificar tags sem reconstruir payload.
- M9+ features novas — query/filtro genérico por "qualquer locatable" sem reusar `extract_payload`.

**M3 desbloqueado** — pode começar `Introspector::from_tags`, primeiro consumidor real do `Vec<Tag>` que P162 emite e P163 verificou estar bem capturado. M3 traz a infraestrutura de query (`query_first`, `query_all`, `query_by_label`, etc.) sobre os tags emitidos.

**Pendências passadas para M3+**:
- Continuação dos 3 itens herdados de M1 (ver relatório P163):
  1. 3 divergências em `m1-lacunas-captura.md`.
  2. Reshape opcional `CounterUpdate::Step` → `Step(usize)`.
  3. Refino opcional de `hash_content`.
- Sem novos itens de M2 — passo limpo, função pura adicionada sem interferir com walk.

API pública `pub fn introspect(content: &Content) -> CounterStateLegacy` preservada. Sem ADR nova. Sem reservas.
