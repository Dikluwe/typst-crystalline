# L0 — place
Hash do Código: 8e47a9dc

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/block-place-region.toml sha256:77c7d31bf3f7276cc9b4d1b2951634724a6d0fb4352a0b45c76e6f3775b512dc
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:96fe00a0c6071bb88ffc63806b3e42c23b78c398fa26d46866e0f099ab0d4e36
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5
- 00_nucleo/prompts/_nuclei/layout/link-frame-scopes.toml sha256:4e487a99ea1f1e82a67dfaced0958f98392fb26b1a7d292355230d6ffa7f2d0b
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8
- 00_nucleo/prompts/_nuclei/place-coordinate-origin.toml sha256:2baf472bb540640d571ae618aa218e5f2b166839a2070f8a4a907ec65d7ded20
- 00_nucleo/prompts/_nuclei/place-flow-boundary.toml sha256:0e7c32f877924fea773be54f9719b43bb72b4c3bb065f6ff44a842c56683ff78
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/place.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/place.rs:22`, `01_core/src/compiler/layout/place.rs:141`, `01_core/src/compiler/layout/place.rs:154`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own the Place adapter plus float fitting, reservation and final emission. Merge missing alignment axes from the cell context. Non-float placement delegates geometry to `00_nucleo/prompts/compiler/layout/placement.md`; without explicit vertical alignment it follows the current ordinary flow origin, including pending-line semantic top edge, rather than always the physical page top. Apply that correction locally to this occurrence's items, decorations and vertical fixups; retain out-of-flow cursor behavior.

For a float, lay out/measure the body in the active isolated subframe, resolve clearance against effective font size and queue the existing DeferredFloat with dimensions, decoration and orphan alignment metadata. The language's default clearance arrives from its constructor as relative 1.5em; the adapter's absent-field fallback remains zero. The current DeferredFloat does not store dx/dy/scope, which are not applied by this float path: do not claim their support or broaden the private struct here.

Admit only an ordered fitting prefix using effective usable region and current material flow. Include adjacent clearance when flow or an earlier admitted float exists; retain the existing defensive admission of the first float on an empty region. Once realized, top/bottom reservation includes future-flow clearance. Anchor top floats at the top and accumulate downward, bottom floats at the physical bottom and accumulate upward; auto height retains its current top-placement fallback. Body, orphan paths and decoration share the final reference frame. Preserve existing top-reserve translations and bottom effective-region reduction, without applying clearance twice.

Ask `00_nucleo/prompts/compiler/layout/cursor.md` for region progress and `00_nucleo/prompts/compiler/layout/flow_suffix.md`/`00_nucleo/prompts/compiler/layout.md` for full suffix recomposition; do not emit in-flow at the cursor, inspect a future Content or duplicate replayed occurrences. Direct private HTML markers are non-material for fitting but remain geometrically transported. Oversize/unmeasured float combinations are not full parity guarantees.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`01_core/src/compiler/layout/place.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

Transportar posição, avanço já consumido, estilo e origem do TextSeparator sem criar tinta, baseline ou novo avanço; manter ordem dos filhos. Excluir separador da origem vertical medida.

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

Tratar TextReplacement como envelope visual transparente nos percursos deste owner; descender nos filhos preservando text, ordem, coordenadas e o cálculo existente, sem novo algoritmo ou avanço.

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

Frames de floats conservam limites lexicais capturados no subframe; sua emissão posterior não herda o link do fluxo que provocou a drenagem. Fechar/reabrir o segmento comitado ambiente ao inserir um float, sem alterar a ordem/fit/geometry e fixups existentes.

Aceitação: RED→GREEN de quebras antes/dentro do corpo, preservação de
prefixo/sufixo, regiões, destinos/bounds, corpo rico/aninhamento/colunas,
conteúdo diferido, contexto/rollback e ausência de limites privados na saída.
Revalidar os controles sem quebra e a suíte, build e linhagem.

### Neutralidade na origem do float

A sonda unitária fragment_link_deferred_float_retains_its_lexical_target
falhou em extended.log: FLOAT muda de y17.7 para y18.8 enquanto BODY fica
igual. extended-candidate.json.gz preserva fontes/L0. place.rs:268-294
desce em Semantic e usa a Shape zero do marcador como origem física 0,
substituindo a origem textual real. Ignorar marcadores privados diretos
em item_origin antes da classificação dos itens; preservar os demais
wrappers/objetos e a fórmula de translação. Não mudar o esperado geométrico.

## Origem geométrica de Place em região local — 2026-10-14

### Medição anterior à decisão

place-origem-local-2026-10-14/{before,extra-before}/results.json.gz registra HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree, diffstat/UTC/fontes/binários/argv e pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8. Box-place publica shape em y37.75pt contra28pt no canvas480×400 (bbox raster aproximado); no SVG fonte, box sem margem do corpus tiling tem y14.834 local contra5. O deslocamento cresce com fonte. placement.rs:463 veta origem geométrica dentro de qualquer subframe e :533 conserva o ascender local; place.rs:96–131 soma text top-edge mesmo para Top explícito. Root-text shape diverge29.5/22.25pt; Box-text32/22.25; Grid e Align são controles positivos. Fonte pinada flow/distribute.rs:703–715 separa align_y explícito de offset de fluxo. Isso é geometria de linguagem; não obrigação de buffers Rust. A hipótese é refutada se texto, VAlign, scopes, fixups ou checkpoint perderem coordenadas após origem única.

### Responsabilidade exclusiva

O adapter segue linha pendente somente quando effective_alignment.v é None. Na raiz, emprestar margin.top=cursor_y para a chamada; no subframe, transladar a ocorrência por cursor_y - sub_frame_origin_y. Não somar text top-edge: o body geométrico já instala o seu. Restaurar margin, não avançar cursor e aplicar mesma delta a itens/decoração/fixups. Floats não mudam.

### Aceitação e etapas

Esta seção supersede a restrição histórica de body geométrico apenas na raiz de Place. L0 primeiro e RED→GREEN em fluxo interno ADR-0127: sem campo público/trait/assinatura/fase/default novos. Comparar antes/depois SVG/PDF e preservar corpos textuais, mistura, páginas automáticas, clipping, posições observáveis e floats; build/workspace/rustdoc/fmt/linhagem/inventário.
A origem do body e a âncora após fluxo comitado são defeitos distintos. Esta etapa A não fecha âncora depois de Shape/Parbreak/Linebreak nem inset de Block, já refutados pelos controles. A etapa B de place-origem-local completa o fim causal do fluxo e a região de Block; não declarar Place universal pronto enquanto B estiver aberta. O transporte público de Tiling permanece pendente de aprovação em seu pacote, sem alteração por esta correção.

## Place após fluxo comitado — etapa B1 — 2026-10-14

### Medição antes da decisão

`lab/results/construction/diagnostics/place-fluxo-2026-10-14/before/results.json.gz` registra HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, diffstat/UTC/fontes/binários/argv e pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8. Nos 24 pares root/Box após 12 prefixos, o marcador diverge em todos; Linebreak/Parbreak precisam y22.25pt, Shape y27pt, Block/Grid/VSpace20pt precisam y35pt, mas Tekt usa15pt. Em Rotate há também sobreposição visual, defeito independente. line_commit.rs:721–733 prepara próxima baseline; shape.rs:196–206 agrega below/cap; block.rs:465 conserva fronteira e margem separadas; place.rs:91–99 só reconhece linha ainda pendente. Fonte pinada flow/distribute.rs:703–715 usa offset físico de flow para Place sem eixo explícito. Isso é geometria de linguagem, não exigência de estrutura Rust. A hipótese é refutada por margem fraca, sobreposição, página nova, rollback ou vazamento entre subframes.

### Responsabilidade exclusiva

Possuir métodos privados para registrar um fim com cursor corrente e consultar somente certificado cuja testemunha de cursor ainda coincide. O adapter não flutuante usa linha pendente primeiro, depois fim certificado, somente quando VAlign ausente; aplica a mesma delta a itens/decoração/fixups. Não faz scan de tinta nem flush antecipado. Floats não mudam.

### Aceitação e extensão pendente

Correção interna ADR-0127: L0 primeiro, RED→GREEN e resselo; sem campo público, flag/default ou fase nova. Este adendo autoriza somente o estado privado necessário a esse contrato, supersedendo proibições de novo estado limitadas a extrações anteriores. Guardar/restaurar em LIFO é obrigatório. Revalidar suíte completa, HTML/contexto, PDF/SVG, build/doc/fmt/check, linhagem e inventário de avisos.
B1 completa a âncora após produtores certificados (linha, forma, Block, Align, Grid, Raw, VSpace), sem substituir o objetivo de corrigir todo o sistema. B2 completa região de Block/inset; B3 resolve assentamento tardio de linha pendente com inline Box/Transform, e audita demais produtores (Heading/List/Stack/Equation/Columns) antes de certificar Place universal. Testemunha inválida conserva a rota legada e é pendência, não crédito de paridade. O gate público de tiling continua pendente.

## Block: região local e recorte — B2a — 2026-10-14

### Medição anterior à decisão

`lab/results/construction/diagnostics/place-fluxo-2026-10-14/block-regions-before/results.json.gz` preserva HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, árvore suja, diffstat, UTC, fontes e binários. Em 24 pares contra vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, só 2 bboxes coincidem: inset3/top Tekt(20,15), vanilla(20,18); bottom/right Tekt(71,37), vanilla(62,44); clip apaga o marcador Tekt. block.rs:90 ignora inset.right; :220 instala largura como body, :275 usa avanço zero de Place como altura de clip, :290 agrupa filhos absolutos sem rebase. placement.rs:505 usa página/subframe sem região Block. Fonte pinada typst-layout/src/flow/block.rs:66-89 e :223-234 força tamanho interno, cresce inset e só então recorta; :278-280 desconta inset da região. Geometria é observável de linguagem; buffers Rust são mecânica. A hipótese de região local é refutada por aninhamento, texto, auto, clipping ou vazamento aos irmãos.

### Responsabilidade exclusiva

Passar flow_y opcional para o helper privado de Place. Remover empréstimo de margin.top e pós-translação de subframe; o helper aplica a origem na fórmula comum, preservando cursor e floats.

### Aceitação e incompletude explícita

Correção interna/de paridade ADR-0127, sem campo público, modo ou fase. Este adendo substitui somente as fórmulas incompatíveis de região/width/clip anteriores; não legitima outros reparos sem medição. L0 primeiro, RED→GREEN, validação bilateral, suíte e resselo restrito. B2a cobre região local e clipping na região sobrevivente. B2b completa fragmentação de Block, dimensões/outset e integração dos metadados/decoradores que controles adicionais refutarem; B3 continua responsável por baseline tardia de inline Box/Transform e produtores ainda não certificados. Não declarar objetivo completo com essas frentes abertas.

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
consumer as construções/desestruturações antes da edição: [408, 415].
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
