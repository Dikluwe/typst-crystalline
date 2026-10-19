# L0 — footnote_overflow_columns
Hash do Código: d45f180f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:96fe00a0c6071bb88ffc63806b3e42c23b78c398fa26d46866e0f099ab0d4e36
- 00_nucleo/prompts/_nuclei/layout/link-frame-scopes.toml sha256:4e487a99ea1f1e82a67dfaced0958f98392fb26b1a7d292355230d6ffa7f2d0b
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee


```text
```

## Ownership and evidence

Layer: L1. Sole productive consumer:
`01_core/src/compiler/layout/footnote_flush.rs` (lineage header at `:2`).
The flush body at `:40`, safe boundary at `:91`, greedy fitting at `:128`
and placement at `:188` implement the current behavior.
`01_core/src/compiler/layout/mod.rs:2187` forwards warning strings;
`01_core/src/entities/layout_types.rs:989` declares the existing document
warning carrier; `03_infra/src/pipeline/paged.rs:439` converts it to diagnostics.
Those files are evidence of integration, not additional consumers owned here.

Column placement/translation is specified in
`00_nucleo/prompts/compiler/columns.md`. Layouter state, document fields and
pipeline warning conversion retain their own L0s; this prompt does not authorize
new fields or changes to those modules.

## Flush and bounded overflow

Preserve flush_pending_footnote_bodies(bottom_y: Option<f64>).
It consumes pending numbered bodies, measures their marker plus body in a local
sub-frame of the current usable page/column width, and emits fitting bodies in
order. bottom_y optionally supplies the container's footnote-area bottom;
otherwise use the region height minus bottom margin.

Compute top_safe as the maximum of top margin, current cursor_y, the current
positioned-item Y values and the open line's item Y values, using the existing
FrameItem projection (including Semantic children).
available_h is max(area_bot - top_safe, 0).
This measured projection is not a general bounding-box/ink-extent proof.

Fit greedily by cumulative measured height. At the first non-fitting body,
defer it and subsequent bodies in order for the caller's next flush.
Force the first body only when its own measured height exceeds the full usable
page/column height, not merely the remaining current space.
This defensive escape prevents non-progress on a body that cannot fit anywhere.

Place accepted bodies top-down from
max(area_bot - accumulated_height, top_safe), compensating the sub-frame's
ascender in physical item translation. Supply the actual left origin to the
sub-frame and do not add it again as a full horizontal translation.
Retain decoration and orphan alignment-fixup rebasing, distinguishing logical
origins from ascender-adjusted physical baselines. Preserve causal numbering,
remainder order and pending-note coordination by the caller.

## Warning and purity contract

On forced emission, append the existing pure string, substituting the number:

```text
footnote body [N] exceeds available column/page height and may be truncated or overlap content
```

L1 writes no Sink, file, environment or clock. Existing Layouter warnings move
to PagedDocument.layout_warnings; the paged pipeline drains them into its
returned warnings vector as SourceDiagnostic::warning with detached span.
This consumer owns warning production, not the carrier declaration or conversion.

The user must be able to recover the note or be warned of placement failure;
mere presence of bytes in a PDF is not success. A fitting/deferred note must
not silently disappear. The forced oversized fallback explicitly permits
truncation/overlap with a warning: do not promise impossible no-overlap or
vanilla-style note splitting/continuation for that case.

## Acceptance and boundaries

Verify empty queues, notes that fit, deferral when only remaining space is too
small, inclusion of unflushed line/cursor state in top_safe, oversized-body
termination and exact warning production, optional bottom_y, and single-origin
translation with nested placement/alignment fixups.
Integration checks cover preserved warning forwarding and existing small-note
column placement. No new source edits are implied by these checks.

The old quarantined-vanilla observation did not establish that current vanilla
lacks page(columns:). Its incomplete provenance is historical, not an exception
to the active parity target or a new default. Relevant history and outstanding
evidence limits are in
`lab/results/construction/diagnostics/sanitization-introspection-columns-history.md`.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`01_core/src/compiler/layout/footnote_flush.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

Transportar posição, avanço já consumido, estilo e origem do TextSeparator sem criar tinta, baseline ou novo avanço; manter ordem dos filhos. Excluir separador do limite vertical seguro.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

## Identidade textual — materialização aprovada

### Medição anterior à decisão

As sondas bilaterais e o estado exato em
`lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/initial.json`
e `before/` medem U+002D versus SHY com geometria coincidente em oito controles.
A inspeção do percurso FrameItem deste consumer está arquivada em approved-before/.
O dono confirmou o contrato em `00_nucleo/prompts/entities/layout_types.md`.

### Responsabilidade exclusiva

Limite seguro consulta filhos visuais; a tradução preserva text e desloca filhos pelo delta do posicionamento, sem avanço novo.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.

## Migração do carrier bidi ratificado (2026-10-05)

Medição anterior à decisão: o inventário `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json` identifica as referências desta unidade e os owners 1:1. O contrato do manifesto 1ae7513e foi confirmado; nesta unidade ajustar constructors não certificados com None e patterns de leitura com `..`, preservando os observáveis de linguagem existentes. Transformações obedecem ao núcleo pinado de origem; sem ampliar fase/API.


## Face efetiva de TextShaped — materialização ratificada 2026-10-05

Medição anterior à decisão: `lab/results/construction/diagnostics/identidade-fonte-shaped-2026-10-05/{initial.json.gz,probe.txt,bilateral.json}` distingue GIDs da regular de seleção Oblique. O campo público `TextShaped.font: Option<Font>` do owner layout_types foi confirmado pelo humano (SHA f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409). O snapshot e inventário desta rodada estão em `lab/results/construction/diagnostics/identidade-fonte-shaped-implementacao-2026-10-05/`.

Este owner migra apenas suas referências: constructors legados usam None; transformações preservam a fonte; comparação/junção shaped discrimina face e eixos. Emissão do shaper fornece clone da fonte efetiva também após cache hit; não alterar semântica/estilo para induzir seleção. Collectors/subset/CMap e PDF usam binding da mesma face original com coordenadas do estilo. PNG/SVG preparam bindings locais e instanciam uma vez por face/pedido antes de render, mantendo índice de recursos original distinto dos bytes instanciados. None segue resolução legada. API pública e FontKey existentes permanecem. Não criar fontes artificiais, unsafe, globals ou fase nova. Funções privadas podem receber fonte/bindings explicitamente.

A associação é por bytes da face original e pedido de eixos; nomes e slots não bastam. A tabela local dura a exportação e conserva ordem dos recursos fornecidos, anexando faces faltantes. Para multifont, distribuir itens pelos bindings uma vez antes de coletores por fonte; não reutilizar GIDs/CMap globais em faces diferentes. Código deste owner continua 1:1 com seu L0; referências a outros owners não compartilham propriedade. Aceitação RED→GREEN e controles bilaterais incluindo miss/hit/VF, sem promessa de ganho de tempo.

Claims compartilhadas desta migração pertencem ao núcleo shaped-face-identity pinado acima; este owner materializa somente as referências da sua unidade.

## Captura de links através de fronteiras — 2026-10-14

### Medição anterior à decisão

lab/results/construction/diagnostics/links-paginacao-2026-10-14.md e os recibos
posicoes-compostas-2026-10-14/automatic-boundary-probes registram fonte,
HEAD/árvore suja/UTC, binários e pin vanilla. Página 160×40pt/margem 10pt:
texto comum atravessa a quebra; Link URL, Ref e Ref Page abortam com índice
antigo após esvaziamento dos buffers. references.rs:270 aparece na pilha;
link.rs:22-32 usa o mesmo mecanismo. outline.rs:28-50 também guarda índices
antes do corpo. Limitar o índice não prova preservação do prefixo, ordem,
fragmentos, destino ou geometria. Snapshot anterior deste reparo:
lab/results/construction/diagnostics/links-captura-2026-10-14/before.json.gz.
Os observáveis são conteúdo, destino, regiões e bounds; limites privados
são mecânica. Correção interna ADR-0127, sem API, default ou fase nova.

### Contrato deste owner

A emissão diferida de notas não deve herdar um link do fluxo que provocou o flush. Suspender escopos ambientes e fechar/reabrir seu segmento comitado em torno da emissão; links explícitos no corpo da nota permanecem. Não mudar a política de divisão/overflow de notas neste reparo.

Aceitação: RED→GREEN de quebras antes/dentro do corpo, preservação de
prefixo/sufixo, regiões, destinos/bounds, corpo rico/aninhamento/colunas,
conteúdo diferido, contexto/rollback e ausência de limites privados na saída.
Revalidar os controles sem quebra e a suíte, build e linhagem.

## Referencial de pintura — transporte aprovado T1b — 2026-10-14

### Medição anterior à decisão

O manifesto `d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`
foi aprovado pelo humano para T1–T3; recibo em
`lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
A evidência bilateral de Parent e controles Solid está no contrato aprovado;
ela não é substituída por inspeção estrutural. A rodada
`lab/results/construction/diagnostics/paint-root-2026-10-14/before/results.json.gz` conserva
fontes, estado sujo, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, UTC,
diffstat e hashes dos binários; cobre contêineres, Place e Rotate com Solid
e Gradient Parent. O inventário before-source/syntax-nodes registra neste
consumer as construções/desestruturações antes da edição: [293, 300].
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
