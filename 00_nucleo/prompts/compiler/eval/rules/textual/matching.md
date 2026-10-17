# L0 — compiler/eval/rules/textual/matching
Hash do Código: ae2c9427

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/rules/textual/matching.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

At the working tree captured in `00_nucleo/diagnosticos/atomizacao-textual-algoritmos-2026-09-30-before.json.gz`, `01_core/src/compiler/eval/rules/textual.rs:96-333` contains realized group traversal, rule precedence, matching, transformation and projection of unmatched original ranges. The `p1494` filter passed 17 tests on that state. This is a structural baseline, not a timing or new vanilla measurement.

## Decision and behavior

Own the complete realized textual group algorithm, from grouping by causal snapshot and style boundary through match selection and replacement, delegating preservation of unmatched
content to `00_nucleo/prompts/compiler/eval/rules/textual/matching/projection.md`. Search literal/regex over the largest eligible realized group of the same lexical snapshot and style chain, including separated Text/space and dynamic text nodes. Each recipe receives only its matched slice; preserve leftmost order, unmatched prefix/suffix, precedence, guard/revocation and depth limit. A replacement is not fed back into the same revoked rule. Empty regex matches remain rejected and literal '.' stays distinct from regex '.'.

While a ContentBlock is only a value, preserve raw Space/SpaceFromNewline. Its local textual show operates provisionally before normalization: replace only the matched slice, preserving unmatched separators for later insertion. Root realization normalizes the inserted value and applies its eligible snapshot. The combined local CJK show/normalization case and sequential-identical-rule override remain explicit unresolved parity obligations, not presumed successes.

The coordinator passes its existing inputs after optional normalization and early return when show is disabled. This module may access the parent snapshot and call the existing selector matcher/call dispatcher, but must not duplicate space normalization, alter the engine restoration order or introduce dynamic dispatch. The descendant projection owner preserves unmatched nodes and flattened
replacement sequences; its unmatched-slice test lives with that algorithm. No phase, public signature or default changes.

## Projection refinement evidence

At the working tree captured in
`00_nucleo/diagnosticos/atomizacao-textual-projecao-2026-09-30-before.json.gz`,
this consumer contains `project_original_range` and `append_flat_sequence` at
lines 189-223, followed by the matching transform. The slice projection test
passed in the `p1494` filter after the first movement. Their only input is
content and range/sequence structure, so projection can be a pure descendant
without access to engine state.

## Acceptance

The existing literal/regex slicing, lexical snapshot and guard tests and full core suite remain green. `cargo build` and `crystalline-lint .` have no new blocking violations. No speedup is claimed without measurement. Header ownership is 1:1.


## Realização com avaliação única

Grupos textuais podem projetar Realized/Located e sequências exclusivamente textuais produzidas por elementos. Elementos estruturais continuam fronteiras. O snapshot causal do grupo conserva origens consumidas por splice.


## Realização com avaliação única

Um produto textual de elemento participa do match de grupo; espaços sozinhos continuam sem receita.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: 00_nucleo/diagnosticos/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em materialization/table-origens-d08b3c2-2026-10-07/contract.md.

Agrupamento textual Source delega somente body para regex/texto; preservar origem na reconstituição de produto e não criar fronteira extra.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.


## Exaustividade nominal e espaços transparentes — 2026-10-17

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Fontes/L0/configuração, hashes, UTC, status e git diff HEAD --stat em `00_nucleo/diagnosticos/tekt-v16-whitespace-2026-10-17/before.json.gz`. AST/spans em contexts.json; 11 V16 emitidos e um forwarding de identidade latente em classified_content. measurement-before.log mede cinco diferenças entre sete pares Source. measurement-family-before.log e source-family-before-analysis.json medem 28 pares, 20 diferentes: os cinco casos de quebra à esquerda/direita e grupo atravessando Empty falham igualmente sob Source, Located, Realized.visual e Styled. Parbreak e Text continuam fronteiras/apoios iguais. É medição do algoritmo real dentro do crate em workspace descartável, não uma cópia adaptada de helpers privados nem prova global de paridade. A primeira sonda externa falhou por E0624, foi preservada e não conta como RED funcional.

### Prescrição exclusiva

- **W01** — `01_core/src/compiler/eval/rules/textual/matching.rs:243`, `textual_piece`: Só texto/space/empty e os carriers já projetados entram no grupo. Os demais continuam fronteiras; não atravessar Styled/ShowRecipe ou alterar snapshots/receitas. Excluir do complemento somente `Realized, Source, Located, Sequence, Empty, Text, Space, SpaceFromNewline`; guardados preservados `(nenhum)`.

### Aceitação

L0 antes de código; correção interna de invariantes existentes (ADR-0127 contínuo), sem campo/assinatura/trait/API, default, fase, dependência, flag ou compatibilidade nova. Complementos excluem somente braços Content incondicionais; conservar variantes guardadas no fallback e manter o guard had_newline. Grupos V19 têm a mesma decisão por caso, permanecem visíveis e não comprovam executar cada alternativa. Não introduzir allow/expect, @ ou um if-let que esconda a decisão. HTML mantém os braços Boundary absorventes existentes e fecha nominalmente as três células do complemento; não muda preformatted/raw/title/ruby, tabelas de tags ou SmartQuote.

Acrescentar regressões nos três módulos proprietários: ambos consumidores de normalização, Source único/aninhado e os quatro carriers em quebras/Empty, controles Parbreak/Text/MathIdent, preservação de span/estilo e Realized.visual separado de original; projeções matching positivas/negativas e matrizes 3×3 inline/top-level com Boundary/EmptyInline/Supportive. Não relaxar testes existentes. RED real antes da implementação, GREEN após. Sondas de variante futura Content/SpaceSupport com cfg(test) compilado: no core usar enum real; no infra desbloquear somente E0004 de dependências/unidades fora dos alvos, em cópia descartável, com braços panic sintéticos registrados e nunca executados. Nunca reparar o local selecionado na própria sonda. Isso distingue dependência que impede compilar o alvo de exaustividade efetiva do alvo. Registrar os 12 matches finais (13 braços), incluindo o helper novo; dois braços de space_with_origin compartilham um match e não contam como dois erros do compilador.

Gates: build e suíte workspace completa, fmt, Clippy workspace/all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings com quatro índices, V1/V5/V15/V26 estrito, reversa, replay e diferencial SARIF completo. Esperado V16 152→141; V19 somente dos grupos prescritos; V20 sem crescimento. Recursos: MemoryMax=12 GiB, swap zero, um job Cargo e comandos pesados sequenciais. Sem commit/push/merge/release ou alteração de pin CI. O objetivo geral e math-stretch-composite permanecem abertos.
