# L0 — math/layout/accent

Hash do Código: 4f862227

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/accent.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/accent.rs:29` locates `pub(super) fn layout_accent(`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`00_nucleo/diagnosticos/sanitization-remaining-compiler-history.md`.

## Complete accent carrier

Consume MathAccentElem without mutating public Content. Resolve size and
dotless independently by explicit > math.accent cascade > default:
size=100%, dotless=true. Preserve absence and explicit false/zero.
Unicode combining class Below and ⏟/⎵/⏝/⏡ select a lower accent.
Above accents use a cramped base and contextual dotless i/j when enabled;
below accents use a non-cramped base and no dotless rewriting.

Resolve horizontal size against the measured base width, with absolute
length against causal font size. Stretch uses shortfall 0.5em through
`00_nucleo/prompts/compiler/math/layout/stretchy.md`, not the vertical
delimiter policy. Preserve the base's identity and source presence metadata.

Above placement is baseline-relative:
`accent_y = -base.ascent + min(base.ascent, accent_base_height)`;
`ascent = max(base.ascent, -accent_y + accent.ascent)`.
Keep base descent. Below placement puts the accent beneath the base and
includes its lower ink extent. Attachment, not arbitrary fixture centering,
determines x: `base_attach - accent_attach`, with the existing metric
fallbacks for composite bases. The accent must not erase nested accent extent.

Acceptance distinguishes orientation, size 100%/150%, dotless on/off for i/j,
attachment and nested baseline geometry, using attributable font metrics.
The current oversized-accent width heuristic and incomplete attachment
attribution are audit limitations, not new normative exceptions.

## Identidade extraível dos acentos

Na sonda `.typ/sec_10.typ` e `.typ/sec_12.typ` do diagnóstico
`typs-teste-2026-09-30`, `pdftotext -layout` separa/inverte a marca combinante
e sua base enquanto a expressão da linguagem conserva a relação base+marca.
Quando a base realizada é uma folha textual de um caractere e o acento é uma
marca combinante, envolver os itens visuais posicionados em um
`TextReplacement` com a sequência base seguida da marca, por ocorrência.
Para base composta, conservar seus itens e não inventar uma serialização
textual. O envelope não muda tamanho, deslocamento, fonte nem desenho.

A sonda pós-correção mostrou `dot(dot(x))` extraído como marca+base+marca
quando o envelope interno permanece aninhado. Para acentos aninhados de uma
base textual atômica, compor a sequência de todas as marcas na ordem interna
para externa e abrir somente um envelope externo; os envelopes internos são
transparentes apenas para esse caminho, mantendo todos os filhos visuais.

## Atomização interna do owner

A resolução de `size`/`dotless`, a orientação da marca, o posicionamento
geométrico e a composição da identidade textual devem ser operações privadas
distintas dentro do consumer `accent.rs`. A identidade extraível é aplicada
depois de posicionar os itens e não altera métricas. Essa separação não
introduz consumer, API ou comportamento novo.
