# L0 — math/layout/_comum
Hash do Código: 2cdd4321

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:96fe00a0c6071bb88ffc63806b3e42c23b78c398fa26d46866e0f099ab0d4e36
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/math-attach-slot-presence.toml sha256:81b492ca5d01377da0b54b6deb21b6cb24b20919009ea3ea21b7350959779715
- 00_nucleo/prompts/_nuclei/math/callback-realization.toml sha256:3cf8b79e22fcda6466dc38b7691a73ada4540956fbed14129c9e1ed5a6a56c43
- 00_nucleo/prompts/_nuclei/math/layout-observables.toml sha256:42a441e59c53fdc5bf619005c0e018eb63251533fb6dc6c16c13d000e592e40c
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee



**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/mod.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/mod.rs:24` locates `mod accent;`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`00_nucleo/diagnosticos/sanitization-remaining-compiler-history.md`.

## Core ownership

Own only `01_core/src/compiler/math/layout/mod.rs`: MathLayouter, MathBox,
equation entrypoints, static dispatch, recursive transformations, sequence/grid
composition and external-content bridging. Feature geometry belongs to its
child owner, not to entities. External content bridging belongs to `00_nucleo/prompts/compiler/math/layout/external_bridge.md`; sequence flattening and horizontal composition belong to `00_nucleo/prompts/compiler/math/layout/sequence_composition.md`. Math grid measurement, placement and baselines belong to `00_nucleo/prompts/compiler/math/layout/grid_composition.md`. Recursive glyph/default style transformation belongs to `00_nucleo/prompts/compiler/math/layout/style_transform.md`. Keep closed static dispatch and exhaustive
FrameItem handling; no registry, vtable or reverse entities-to-compiler import.

Child owners are `00_nucleo/prompts/compiler/math/layout/accent.md`,
`00_nucleo/prompts/compiler/math/layout/assembly.md`,
`00_nucleo/prompts/compiler/math/layout/attach.md`,
`00_nucleo/prompts/compiler/math/layout/callbacks.md`,
`00_nucleo/prompts/compiler/math/layout/cancel.md`,
`00_nucleo/prompts/compiler/math/layout/cases.md`,
`00_nucleo/prompts/compiler/math/layout/delimited.md`,
`00_nucleo/prompts/compiler/math/layout/frac.md`,
`00_nucleo/prompts/compiler/math/layout/large_operator.md`,
`00_nucleo/prompts/compiler/math/layout/matrix.md`,
`00_nucleo/prompts/compiler/math/layout/op.md`,
`00_nucleo/prompts/compiler/math/layout/root.md`,
`00_nucleo/prompts/compiler/math/layout/spacing.md`,
`00_nucleo/prompts/compiler/math/layout/stretch.md`,
`00_nucleo/prompts/compiler/math/layout/stretchy.md`,
`00_nucleo/prompts/compiler/math/layout/underline.md`,
`00_nucleo/prompts/compiler/math/layout/underover.md` and
`00_nucleo/prompts/compiler/math/layout/vec.md`.
Their test consumers have separate owners.

## Pure context and equation entrypoints

MathLayouter is generic over injected FontMetrics; obtain MathConstants from
`metrics.math_constants(style)`, using the active face rather than an
unconditional fallback. Preserve `new(metrics, block, style)` and
`new_with_context(metrics, block, style, equation, region_height,
lexical_styles, callback_pass)`. The contextual constructor supplies the
equation Location, effective region height, causal StyleChain and validated
callback-pass state. The context-free constructor remains available for
existing callers; it is not a substitute for productive contextual layout.

No Func/Engine/World execution occurs here. Each equation's local cancellation
occurrence starts at zero; reserve/increment it once at MathCancel entry,
before descending into its body, including Auto and explicit-angle elements.
Pass that occurrence to cancel; cross lines share it.
Transient math.mid target state is local to the nearest lr scope.

`layout_equation` returns baseline-relative FrameItems.
`layout_equation_measured` returns items and EquationExtent from the same
layout result: width, ascent, descent, line_count and first/last baseline.
Do not introduce a second measuring algorithm or serialize a provisional
callback result as successful compilation. Pipeline realization belongs to
its existing infrastructure owner.

## Baseline and composition

MathBox contains width, ascent, descent and items. Its local y=0 is its own
baseline; y increases downwards. Horizontal concatenation changes x only,
takes maximum ascent/descent and does not top-align siblings. Missing gaps
in hconcat_spaced behave as zero. Offset every supported FrameItem consistently,
including lines, shapes, groups and shaped text.

For axis centering, `shift = axis - (ascent-descent)/2`; add shift to ascent,
subtract it from descent and move items by -shift. Grid content is centered
before merging already-centered delimiters. Fractions, roots and fenced groups
use their child-specific baseline contracts, not blanket axis centering.

Grid delimiter target is `(ascent+descent)*1.1*upem/font_size`, or zero for
nonpositive font size. The 1.1 ratio and delimiter shortfall are separate.
Shared grid helpers measure cells, allocate column widths and position rows
with explicit column/row gaps and alignment-boundary flags. At an ampersand
boundary, class spacing is accounted for in the left cell rather than adding
a normal column gap. Alternating subcolumns align right then left; grids
without alignment points are centered unless a feature specifies alignment.
Row ascent/descent are separately maximized with the synthetic-parenthesis
floor. Accumulate row descent without duplicating a row's ascent.
Multiline equations derive leading from paragraph style rather than
unconditionally using MathConstants.math_leading; retain first/last baselines
for the existing equation integration owner.

Numerator descent by level is Display→Text ×1, Text→Script ×script,
Script→ScriptScript ×sscript/script, ScriptScript→ScriptScript ×1.
Numerators inherit cramped; denominators force cramped. These are discrete
levels, not unconditional multiplication by the script factor.

## Style and morphology preservation

Apply mathematical default glyph transformation once at equation entry.
Single-character eligible Latin/lowercase Greek leaves receive mathematical
italic codepoints; uppercase Greek stays upright. Preserve MathStyled wrappers
for their own handler. Resolve nested setters per independent glyph, size,
bold, italic and cramped axes: the innermost specified setter wins on the same
axis; different axes compose. Explicit false is not absence. Size wrappers
survive glyph transformations; competing size setters are not multiplied.
Bold on a default-italic variable yields bold italic; explicit upright disables
italic. MathOp text is not rewritten into a glyph variant.

Recursive transformations preserve public identity and all causal fields:
attachment slot states; root index and index_explicit; fraction line, optional
style and both deparenthesization bits; limits inline/inline_explicit; accent
mark, size, dotless and presence; cases children, optional delimiter sides,
reverse, gap and presence; matrix rows, alignment, augment, gaps, delimiters
and presence; lr/mid/stretch bodies and explicit fields; every public
under/over spreader identity and optional annotation. Transform children
without baking inherited properties into explicit fields.
Omitted, explicit none, zero, auto and present empty Content are not interchangeable.
Preserve markup containers and semantic styles while descending; do not
flatten a mathematical subtree through plain_text as a transformation.

Dispatch fractions by explicit style > validated math.frac.style > Vertical;
line=false always selects the barless vertical binomial stack. Invalid
validated cascade values fail closed rather than silently choosing a default.
Forward complete accent/cases/matrix/cancel carriers to their geometry owners.
Root index_explicit is geometrically inert. Limits wrappers lay out their body
transparently; attach owns the limit placement decision. Public spreader
lowering is a local layout view, never a rewrite of the public carrier.

## Sequences, text and spacing

Flatten MathSequence. Keep Content::Sequence atomic only when all children are
Text/MathText/MathIdent leaves; open technical sequences with HSpace or
composite math nodes. Weak HSpace collapses at run edges but remains between
material items. Absolute space resolves against current font size; fractional
space is not fabricated as an absolute advance.

A punctuation MathText with terminal whitespace and a subsequent material
sibling contributes its boundary once, through a transient layout/spacing
view; retain the original carrier and preserve terminal whitespace when no
material successor exists. Do not reintroduce normalized glue via Text fallback.

Direct Content::Text is a TextItem: preserve math=true and math_text_item=true
through item/style merges; do not apply glyph-fragment italics correction or
script substitution to it. These flags do not reset size, cramped, font,
fallback, language, direction or features. Other math leaves are not TextItems.
Subtract terminal text glue only for direct Text immediately followed by
MathDelimited. Atomic markup and other boundaries retain their full advance.
Spacing rules belong to the spacing owner; pass active text-space metrics and
suppress class gaps when math_script or Script/ScriptScript size is active.
For a direct single-leaf MathClassOverride, retain glyph-fragment italics
correction unless the glyph has an extensible variant/assembly; do not repeat
that correction for nested composite frames.

Large operators in Display use the applicable display-operator minimum height.
Keep the base glyph when it already reaches that height. Otherwise choose the
first sufficient vertical variant; when none suffices, use a vertical assembly
if the active font supplies one, or its largest variant if it does not. Use the
base glyph only when no construction is available. The target has no delimiter
shortfall. The selected glyph's real ink is centered on the axis in both
inline and display modes, without changing operator identity or horizontal
advance semantics.

## External content boundary

Route Equation/Boxed/Align/Pad/Block and recursive content requiring normal
layout through the existing pure sub-layout bridge. Typed text deltas
(size, fill, weight, font, tracking, leading, lang, bold, italic) also require
that bridge; custom-only style transport and plain markup remain on the
baseline-aligned text path. Preserve lexical/effective style as far as the
existing bridge represents it, without claiming unsupported contextual access.

Use the first declared text/glyph baseline where present; only baseline-less
frames use axis-centered extents. Translate embedded items into local
baseline space; do not invent a top-relative Group wrapper or discard shapes.
If external layout emits nothing but plain_text is nonempty, preserve the
existing textual fallback. Region constraints, introspection, clipping and
metric approximations of this bridge remain documented limitations in the
diagnostic, not new language defaults.

## Acceptance

Observe language morphology, style precedence, baseline/extents and
font-derived geometry with attributable metrics. Preserve complete carriers
through both transformations, including empty-present attachments and optional
fields. Do not accept fixture coordinates, serialized-byte equality,
implementation similarity or historical test totals as parity evidence.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`01_core/src/compiler/math/layout/mod.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

Transportar posição, avanço já consumido, estilo e origem do TextSeparator sem criar tinta, baseline ou novo avanço; manter ordem dos filhos. Na varredura vertical não estabelecer baseline nem tinta a partir do separador.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

## Identidade textual — materialização aprovada

### Medição anterior à decisão

As sondas bilaterais e o estado exato em
`00_nucleo/diagnosticos/identidade-hifen-discricionario-2026-09-28/initial.json`
e `before/` medem U+002D versus SHY com geometria coincidente em oito controles.
A inspeção do percurso FrameItem deste consumer está arquivada em approved-before/.
O dono confirmou o contrato em `00_nucleo/prompts/entities/layout_types.md`.

### Responsabilidade exclusiva

Tratar TextReplacement como envelope visual transparente nos percursos deste owner; descender nos filhos preservando text, ordem, coordenadas e o cálculo existente, sem novo algoritmo ou avanço.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.

## Estilo da base de acento

Na sonda `.typ/sec_12.typ` de 2026-09-30, `hat(mu)` conserva `μ` sem o
itálico matemático enquanto o mesmo símbolo fora do acento vira `𝜇` no
vanilla pinado e no restante da equação. Causa medida em
`01_core/src/compiler/math/layout/mod.rs`: o braço `MathAccent` de
`apply_math_default` exclui toda folha `MathText`, apesar de a própria
função já restringir o mapeamento a um único caractere elegível. A base do
acento segue a mesma aplicação de itálico matemático por defeito que as
demais folhas, preservando wrappers `MathStyled` explícitos e textos
multicaractere. Não alterar o sinal de acento, a geometria ou a fase.

## Migração do carrier bidi ratificado (2026-10-05)

Medição anterior à decisão: o inventário `00_nucleo/diagnosticos/bidi-contexto-2026-10-05/inventory.json` identifica as referências desta unidade e os owners 1:1. O contrato do manifesto 1ae7513e foi confirmado; nesta unidade ajustar constructors não certificados com None e patterns de leitura com `..`, preservando os observáveis de linguagem existentes. Transformações obedecem ao núcleo pinado de origem; sem ampliar fase/API.


## Face efetiva de TextShaped — materialização ratificada 2026-10-05

Medição anterior à decisão: `00_nucleo/diagnosticos/identidade-fonte-shaped-2026-10-05/{initial.json.gz,probe.txt,bilateral.json}` distingue GIDs da regular de seleção Oblique. O campo público `TextShaped.font: Option<Font>` do owner layout_types foi confirmado pelo humano (SHA f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409). O snapshot e inventário desta rodada estão em `00_nucleo/diagnosticos/identidade-fonte-shaped-implementacao-2026-10-05/`.

Este owner migra apenas suas referências: constructors legados usam None; transformações preservam a fonte; comparação/junção shaped discrimina face e eixos. Emissão do shaper fornece clone da fonte efetiva também após cache hit; não alterar semântica/estilo para induzir seleção. Collectors/subset/CMap e PDF usam binding da mesma face original com coordenadas do estilo. PNG/SVG preparam bindings locais e instanciam uma vez por face/pedido antes de render, mantendo índice de recursos original distinto dos bytes instanciados. None segue resolução legada. API pública e FontKey existentes permanecem. Não criar fontes artificiais, unsafe, globals ou fase nova. Funções privadas podem receber fonte/bindings explicitamente.

A associação é por bytes da face original e pedido de eixos; nomes e slots não bastam. A tabela local dura a exportação e conserva ordem dos recursos fornecidos, anexando faces faltantes. Para multifont, distribuir itens pelos bindings uma vez antes de coletores por fonte; não reutilizar GIDs/CMap globais em faces diferentes. Código deste owner continua 1:1 com seu L0; referências a outros owners não compartilham propriedade. Aceitação RED→GREEN e controles bilaterais incluindo miss/hit/VF, sem promessa de ganho de tempo.

Claims compartilhadas desta migração pertencem ao núcleo shaped-face-identity pinado acima; este owner materializa somente as referências da sua unidade.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: 00_nucleo/diagnosticos/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em materialization/table-origens-d08b3c2-2026-10-07/contract.md.

workspace-seventh/p966 refuta Source como fallback textual cru: dispatcher math inspeciona body, preservando identificadores, variante itálica math, scripts e caminho externo/baseline. Não alterar fase de realização matemática.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.

## Integração condicional dos módulos de teste — 2026-10-13

Medição antes da decisão: HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b,
working tree não commitado. A duplicação cfg(test) ocorre em `01_core/src/compiler/math/layout/p1425_tests.rs:5` / pai :1714; `01_core/src/compiler/math/layout/p1426_tests.rs:5` / pai :1717; `01_core/src/compiler/math/layout/p1427_tests.rs:5` / pai :1720; `01_core/src/compiler/math/layout/p1429_tests.rs:5` / pai :1723; `01_core/src/compiler/math/layout/p1430_tests.rs:5` / pai :1726; `01_core/src/compiler/math/layout/p1431_tests.rs:5` / pai :1729.
Fontes, L0, UTC, status e diffstat estão em
`00_nucleo/diagnosticos/cfg-test-ci-2026-10-13/before.json.gz` e
`refinement-before.json.gz`. O Clippy pinado reproduz os atributos duplicados.
A tentativa de retirar a condição interna mantém os testes Rust, mas o linter
arquitetural pinado não propaga a condição do pai e acusa V2 nos helpers.

Neste owner, retirar apenas o atributo externo cfg(test) das declarações
listadas acima. Manter os caminhos, nomes e visibilidades dos módulos e seus
atributos path. A condição interna `#![cfg(test)]` de cada arquivo listado
continua a excluir integralmente o módulo do build de produção, inclusive
seus imports e helpers. Os consumers de teste e seus L0 permanecem intactos.
Não acrescentar dependências, lógica, API, modo padrão ou fase. Validar a
mesma lista de testes, suíte core, build sem cfg(test), Clippy e arquitetura.

## Referencial de pintura — transporte aprovado T1b — 2026-10-14

### Medição anterior à decisão

O manifesto `d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`
foi aprovado pelo humano para T1–T3; recibo em
`00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
A evidência bilateral de Parent e controles Solid está no contrato aprovado;
ela não é substituída por inspeção estrutural. A rodada
`00_nucleo/diagnosticos/paint-root-2026-10-14/before/results.json.gz` conserva
fontes, estado sujo, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, UTC,
diffstat e hashes dos binários; cobre contêineres, Place e Rotate com Solid
e Gradient Parent. O inventário before-source/syntax-nodes registra neste
consumer as construções/desestruturações antes da edição: [536, 543].
layout_types.rs:531–539 não distingue raízes de grupos técnicos; helpers.rs:253–269
reconstrói o Group e precisará preservar o campo. Box agrega fundo/corpo;
Block mantém body plano exceto no clip. Preservação do referencial é requisito
de linguagem; formato dos envelopes é mecânica. A aprovação não fecha as
divergências gerais de geometria previamente medidas em Rotate/Box.

### Responsabilidade exclusiva

Migrar construções e padrões de Group para o campo paint_root aprovado. Grupos técnicos novos e fixtures legadas são false; reconstruções transportam o valor recebido, sem inferir raiz a partir de dimensão, clipping ou transformação. Preservar os demais campos e o comportamento de desenho/recursos. Exportadores ainda não usam este metadado para patterns; essa integração é T2 e não está certificada aqui. Nos ramos que desestruturam e recompõem Group, capturar paint_root e reinseri-lo sem mudança. Não redefinir true como false durante projeção, remoção de marcadores ou rebasing.

### Aceitação e incompletude

L0 primeiro, testes RED→GREEN, resselo restrito; conservar geometria e texto
nos controles fonte→PDF/SVG e revalidar suíte/build/doc/check/fmt/linhagem.
A migração deste metadado não substitui Shape.fill/stroke por FramePaint nem
implementa patterns PDF/SVG/raster; T1c/T2/T3 continuam explicitamente abertos
e já autorizados. Não alegar paridade de tiling ou produção a partir do build.


## FrameItem — exaustividade de decisões existentes, V16, 2026-10-16

### Medição anterior à decisão

F040: 01_core/src/compiler/math/layout/mod.rs:302 (p1441_overline_geometry). Os spans AST, patterns/guards/corpos e complements foram conferidos antes do código. `00_nucleo/diagnosticos/tekt-frame-items-2026-10-16/before.json.gz` conserva HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, fontes/L0/configuração, hashes, status e git diff HEAD --stat. `classification.json` registra os 70 casos e seus complements em 26 owners, usando a árvore do parser pinado do CI. Cláusulas pertinentes do L0 foram lidas junto da fonte; não há proposta de nova arquitetura ou interpretação de intenção upstream. Padrões e exaustividade Rust são mecânica; semântica/morfologia, geometria, efeitos e diagnósticos do produto devem permanecer.

### Prescrição exclusiva desta unidade

- **F040 — `p1441_overline_geometry`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `TextSeparator`, `Semantic`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

Manter callbacks, efeitos, metadados, clipping, fontes, positions, tinta e largura conforme os owners vigentes. Esta rodada não amplia travessias, não transforma wrappers em folhas nem usa text/ActualText como substituto da geometria. Helpers de teste mantêm fixtures, esperados, tolerâncias, contagens e falha Unknown; não aproveitar o saneamento para aceitar ausência de observação. Grupos nominais compartilham o mesmo corpo anterior: notas V19 de alternativas continuam visíveis, sem confundir cobertura de um braço com todas as alternativas.

### Gate e aceitação

Manutenção interna ADR-0127 em fluxo contínuo: nenhum enum/campo novo, método de trait/assinatura pública, default, dependência, compatibilidade ou fase nova. Não materializar propostas históricas pendentes por este adendo. L0 primeiro e resselo restrito em ambas as direções. Não adicionar supressões, alterar limiar ou reescrever match como if-let para esconder V16.

Antes do código, executar o controle novo de FrameVisitor e a suíte workspace nos mesmos esperados, e confirmar RED dos 70 avisos da família por leitura AST. Após código, zero V16 dessa família nos locais abrangidos e nenhum aviso novo; manter V16 externos como pendências. Nos matches do core produtivo alcançados por cargo check -p typst-core, usar cópia descartável com uma variante sentinela de FrameItem para medir ausência de E0004 nos locais antes e presença depois. A sonda cobre somente os locais efetivamente compilados: não extrapolar aos helpers cfg(test) ou L3 que a dependência impede de alcançar. Para todo o lote, conferir padrões/guards/corpos por replay, enum e assinaturas inalterados, build e testes reais. Não substituir corpos por stubs.

Revalidar suíte workspace completa, cargo build, Clippy workspace/all-targets com -D warnings e -D unfulfilled_lint_expectations, formato, Rustdoc -D warnings com quatro índices HTML, linhagem V1/V5/V15/V26 estrita/reversa, SARIF e diff --check. Fontes/L0/configuração ficam congelados durante comandos. Execuções pesadas sequenciais com MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1. Não apresentar saneamento estrutural como nova paridade global, ganho de memória ou benefício medido para IA.


## Exaustividade nominal e origem matemática — 2026-10-17

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado; UTC, fontes/L0/hashes/configuração, status e git diff HEAD --stat exato em `00_nucleo/diagnosticos/tekt-v16-math-layout-2026-10-17/before.json.gz` (snapshot inicial preservado em initial-ten-owners.json.gz). AST/spans em contexts.json/all-contexts.json/wildcard-inventory.json, parser pinado do CI. A suíte anterior passou no mesmo código de entrada, revalidado por hashes.

source-probe-before.json mede limits inline com largura 7.2 pt versus 12.912 pt ao acrescentar Source. source-family-before.json preserva a primeira matriz de 12 pares (11 diferenças). source-family-supplement.json e sua análise preservam 23 pares com métricas atribuíveis (19 diferenças e 4 controles iguais): limites, kerns, Op.text, acentos/dotless, suportes, alinhamento de matriz, espaçamento, sequências, stretch e classe. source-lines-before.json mede a mesma equação com 2 linhas versus 1 e última baseline 16.2 versus 0 sob Source, apesar dos mesmos extents. São observáveis de geometria, identidade extraível e metadados da equação, não igualdade de bytes/pointers/Rust. A transparência já é obrigatória no núcleo content-source-origin; a inspeção mostra os helpers classificando o wrapper em vez do body. Não se infere intenção histórica ou paridade global a partir desses números. O erro de contagem do analisador complementar (24 em vez de 23 pares) foi corrigido usando o stdout original válido, sem refazer a medição nem contar o erro como RED funcional.

### Prescrição exclusiva

Registrar apenas o módulo privado source_transparency_tests, com a condição interna #![cfg(test)] no novo consumer e owner exclusivo, conforme a composição existente. No código produtivo, normalizar transformed.source_body() em layout_equation_measured para preservar line_count/first/last baseline; normalizar node e next em should_tighten_text_frame e content em single_math_leaf_char. flatten_math_sequence_nodes classifica node/children pelo body Source, preserva a Sequence de markup atômica e abre só sequências técnicas já elegíveis; todo fragmento derivado mantém origem própria ou a primeira origem comprovada do container por source_span/with_source_span. Não clonar fragmentos já originados nem acrescentar trabalho de reoriginação no caminho sem span. Preservar a transformação de glyph uma vez, baselines, callbacks, contexto, árvore autoral, API e fases.

Trocar os fallbacks enumerados pelo complemento nominal atual dos braços **incondicionais**, sem alterar corpo/guard/ordem; variantes guardadas continuam no complemento para quando a condição falha. A projeção Option de attach é a única exceção estrutural: None retorna por ?, Some passa ao match nominal de Content normalizado, sem padrão profundo.

- **L17** — `01_core/src/compiler/math/layout/mod.rs:320`, `p1441_overline_geometry`, FrameItem: excluir somente `Line`; guardados `(nenhum)`. O coletor procura somente a espécie de item explicitada; as demais não são candidatos. Conservar os asserts de presença/quantidade e não confundir None com sucesso de layout.

- **L18** — `01_core/src/compiler/math/layout/mod.rs:629`, `needs_external_layout`, Content: excluir somente `Equation, Boxed, Align, Pad, Block, Sequence, Styled`; guardados `(nenhum)`. Mesmo corpo anterior para variantes fora da propriedade procurada; não ampliar a classificação além da transparência Source aqui prescrita.

- **L19** — `01_core/src/compiler/math/layout/mod.rs:936`, `layout_equation_measured`, Content: excluir somente `MathSequence`; guardados `(nenhum)`. Mesmo corpo anterior para variantes fora da propriedade procurada; não ampliar a classificação além da transparência Source aqui prescrita.

- **L20** — `01_core/src/compiler/math/layout/mod.rs:1284`, `single_math_leaf_char`, Content: excluir somente `Text, MathText, MathIdent`; guardados `(nenhum)`. Mesmo corpo anterior para variantes fora da propriedade procurada; não ampliar a classificação além da transparência Source aqui prescrita.

- **L21** — `01_core/src/compiler/math/layout/mod.rs:1431`, `x_of`, FrameItem: excluir somente `(nenhuma variante inteira)`; guardados `Text`. O coletor procura somente a espécie de item explicitada; as demais não são candidatos. Conservar os asserts de presença/quantidade e não confundir None com sucesso de layout. Variantes sob guard permanecem no complemento para conservar a condição recusada.

### Aceitação e limites

L0 antes de código. Correção interna/paridade ADR-0127 em fluxo contínuo: sem campo/assinatura pública, trait, dependência, flag/default novo, compatibilidade ou fase nova. Grupos V19 compartilham o corpo anterior explicitamente justificado por caso; as notas permanecem visíveis e não provam cobertura de execução por alternativa. Não adicionar allow/expect, elevar limiar, capturar @ ou esconder um match em if-let/matches!. Somente a transparência e as reestruturações rasas aqui nomeadas alteram o código funcional; não editar fórmulas/constantes ou relaxar expectativas anteriores. RED funcional das regressões Source e RED estrutural dos avisos/sondas antes do reparo. Após, regressões GREEN, zero V16 dos locais emitidos, revisão futura dos 36 locais (incluindo a nova projeção de Content sob Option) por sentinelas Content/Value/MathStyleKind/FrameItem em workspace descartável com cfg(test) compilado. Os scrutinees/funções atuais e a composição ganham só as alterações descritas; revalidar ast/replay, assinaturas e demais owners.

Gates: suíte workspace completa, build, fmt, Clippy workspace/all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings com quatro índices, V1/V5/V15/V26 estrito/reversa, SARIF diferencial completo e diff-check. V16 esperado 170→152; notas V19 novas somente dos grupos prescritos e do observador novo; V20 sem crescimento. As demais violações continuam pendentes. Fontes congeladas durante os gates; MemoryMax=12 GiB, swap zero, um job Cargo, comandos pesados sequenciais. Sem commit/push/release ou alteração do pin CI. Não alegar ganho de RAM, leitura de IA ou paridade integral por encerrar o lint.

O comentário de needs_external_layout que dizia fallback aberto deve descrever o complemento nominal atual; preservar sua expressão false e os demais comentários.


### Marcadores Source no grid — complemento medido em 2026-10-17

source-markers-before.json confirma: Source(Linebreak) transforma duas linhas
em uma (a/b na mesma baseline); Source(MathAlignPoint) conserva duas linhas,
mas desloca 1/2 em relação à coluna de 333/4. Preservar esse RED antes do reparo.
No owner pai, needs_grid_layout e partition_grid também classificam os marcadores
por node.source_body(), sem mudar separadores, linhas/células vazias ou ordem.
No braço de conteúdo de partition_grid, reter o clone original com sua origem;
usar complemento nominal plano se isso eliminar o binding normalizado anterior,
sem captura @ ou suppressão. A nova origem nunca fabrica Location.
O novo consumer de regressão acrescenta dois testes públicos display, para
Linebreak e MathAlignPoint Source, exigindo as mesmas baselines e posições.
Os 27 controles anteriores não mudam: o total passa a 29, com RED esperado
5 passando/24 falhando. Nenhum novo contrato, fase, default ou API pública.
