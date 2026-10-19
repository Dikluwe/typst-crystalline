# L0 — stream/shape
Hash do Código: efd113ae

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/pdf-conic-placement.toml sha256:9010358e51cfa3267960432244b972b179d447f572c902101eba650f6d6a67cb
- 00_nucleo/prompts/_nuclei/pdf-group-local-space.toml sha256:492b4c76d690fad97626f180997765e9ae130f2eb0df0c003fd0332839010274
- 00_nucleo/prompts/_nuclei/pdf-solid-opacity.toml sha256:8b7fcfaf39e60540d16868e662d48fb466fab58860385fcdae499f8c2edfe799
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee


**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/stream/shape.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/stream/shape.rs:18` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Paint and geometry primitives

Own process/spot paint operators, paths, rounded rectangles, fill/stroke and line
caps together. Consume ShapeKind<Pt>/Corners<Pt> resolved in L1; clamp radii to the
available geometry without resolving Length or discarding relative context.
Preserve path segments, curves, ellipse and fill rules. Butt omits 0 J, Round emits
1 J and Square 2 J; a missing line color retains the empty prefix. Isolate state
with the existing graphics envelopes.

Process colors preserve rg/RG. Spot references use complete colorant identity,
deterministic resource names and the existing u8-quantized tint. Gradient paint
finds the prepared Pattern using its DedupKey/bounding box; spot gradients use the
shape box consistently with resource collection. Preserve linear/radial process-gradient keys,
tiling and missing-resource fallbacks. Do not create resources, choose subsetting
or compress streams.

## Verification

Check four resolved corners, paths/fill rules, cap variants, spot tint/resource
closure and gradient bounding-box lookup in top and local painting.

## Caminhos de clip em espaço local Y-down — 2026-10-13

### Medição anterior à decisão

`emit_shape_path_local` em `03_infra/src/export/stream/shape.rs:215-259`
emite Rect em y=-height, inverte y dos PathItems e ignora Ellipse/Line.
As duas chamadas pertencem a grupos cujo cm já estabelece Y-down.
O teste causal `nested_group_clip_has_transformed_bounds_and_does_not_leak`
obteve y de página 177..169 em vez de 169..161. O estado do primeiro
candidato, falha e fontes/L0 anteriores deste owner estão registrados em
`lab/results/construction/diagnostics/grupos-aninhados-2026-10-13/clip-before.json.gz`;
HEAD, status, diffstat e UTC base em before.json.gz do mesmo lote.
A diferença é uma segunda reflexão semântica indevida, não liberdade
de codificação PDF.

### Decisão e aceitação

Emitir os caminhos de recorte no sistema local Y-down: Rect ocupa
[0,width] × [0,height]; Path preserva y autoral de pontos e controles.
RoundedRect reutiliza o formatter numérico existente em origem (0,0),
permutando top/bottom dos quatro raios porque o formatter interpreta
o seu próprio eixo y como crescente para cima. A permutação não muda
a identidade dos cantos físicos. Ellipse ocupa a caixa local inteira
com quatro arcos cúbicos e o kappa já vigente; Line emite seu segmento
com os sinais dx/dy conservados. Match exaustivo, sem fallback vazio
para máscara geométrica conhecida. O caller continua dono de W n e
do envelope gráfico; o helper não muda CTM nem pinta.

Não mudar emit_rounded_rect_ops nem as formas pintadas fora deste helper.
Correção interna ADR-0127: sem API pública, fase, default ou estratégia
nova. Escrever antes do código testes RED de coordenadas locais de
Path, identidade dos cantos de RoundedRect e extremos de Ellipse;
reexecutar o teste de clip aninhado sem mudar seus esperados e a suíte
de infraestrutura. Rasterizar controles fonte→PDF, preservando evidência
de falhas e distinguindo os defeitos de gradiente ainda abertos.

As invariantes compartilhadas de coordenadas e escopo gráfico pertencem ao
Núcleo pdf-group-local-space pinado acima; este owner materializa somente
a responsabilidade descrita para seu consumer exclusivo.

## Posicionamento cônico — 2026-10-14

### Medição anterior à decisão

`03_infra/src/export/gradients/mod.rs:224-264` perde a posição própria de
Conic RGB/CMYK e deduplica por Arc/bbox; `builder/resources.rs:564-574`
emite matriz identidade nesses ramos. `stream/draw.rs:522-555,939-965`
compõe grupos, mas Pattern é relativo ao espaço inicial do stream.
`stream/page.rs:45-49` acrescenta bleed antes dos itens. O corpus constante
red versus conic(red,red), em before/probe.json.gz, reproduz a perda de
cobertura na árvore atual. Fonte, L0, HEAD, UTC, status e diffstat estão em
`lab/results/construction/diagnostics/conic-posicionamento-2026-10-14/placement-before.json.gz`.
Os dois lados do teste são Tekt: o defeito independe de imitar o vanilla.

### Responsabilidade e aceitação

Receber a ocorrência Shape do dispatcher no helper privado de fill/stroke e resolver o padrão pela chave do coletor. A caixa legada continua aplicável aos outros gradientes. Não criar recursos nem alterar paths, cores sólidas, tiling ou fallbacks.

Correção interna ADR-0127, sem entidade/assinatura pública externa, flag,
default ou fase nova. L0 precede testes RED e implementação; resselo restrito.
Testar posição e dimensão, mesmo Arc em ocorrências/páginas distintas,
fill/stroke, grupos afins aninhados, bleed, parent explícito/local/página,
recursos e pintura nos modos Compact/Verbose. Repetir o corpus raster sem
recortar ou realinhar o resultado.

Esta etapa permanece incompleta quanto à funcionalidade cônica integral:
a etapa seguinte, conic-geometria-amostragem, deve completar centro autoral,
cobertura dos cantos e amostragem CMYK/offsets. Não atribuir cobertura total
à correção de posicionamento nem legitimar o círculo inscrito como resultado
final. O owner conic conserva a malha nesta etapa para isolar a causa.

Refinamento da composição privada: os helpers de paint recebem o PageContext
já existente em vez de dois parâmetros separados para mapa/referências.
Isso transporta os mesmos borrows sem aumentar a lista de argumentos; nenhum
campo ou construtor de PageContext muda. O dispatcher passa seu contexto.


## Transporte preparado em Shape — T1c/T2 aprovado — 2026-10-14

### Medição anterior à decisão

`lab/results/construction/diagnostics/tiling-nativo-2026-10-14/before.json.gz` registra
HEAD, working tree, diffstat/UTC e fontes/owners antes desta migração.
layout_types.rs:510–511 ainda transporta Paint/Stroke; shape.rs:151–191
materializa só fill em grupos; tiling.rs:85 limita/clona a grade. O preparador
prepare_cell já retorna TilingFrame uma vez por ocorrência. As sondas
`tiling-preparacao-2026-10-14/{before,after}-renders/results.json.gz`
comprovam que dense-open e stroke-open ainda divergem nos três targets.
O contrato foi aprovado no manifesto
`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`;
recibo em `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
A preparação/contagem de objetos é mecânica de transporte; render, contornos,
texto, geometria e diagnósticos são obrigações observáveis de linguagem.

### Responsabilidade exclusiva desta unidade

Consumir FramePaint e emitir Pattern cs/scn ou CS/SCN para fill/stroke Tiling registrado, sem fallback sólido. Preservar Solid/Gradient, geometria e espessura. Uma célula preparada deve estar registrada; ausência é erro interno explícito, não cor representativa.

### Aceitação e continuidade

Aplicar o núcleo pinado. A ratificação supersede cláusulas antigas que
aguardavam aprovação ou exigiam Paint/Stroke declarativo dentro de Shape,
somente no âmbito do contrato aprovado. Preservar os demais comportamentos
vigentes desta unidade. L0 primeiro e testes RED→GREEN; migração explícita de
fixtures e consumidores, sem conversão total From<Paint>, sem PartialEq nos
carriers e sem fallback sólido para obter build verde. Não elevar o limite
antigo nem transferir layout para L3. Validar transporte fill/stroke, recursos,
clipping, Parent afim, alvos nativos, controles Solid/Gradient, semântica de
página e toda a suíte/build/doc/fmt/linhagem. T1c, T2 e T3 só estão concluídos
quando integrados e verificados; estado intermediário quebrado ou parcial
não é certificação de produção. T4 e publicação continuam fora deste pacote.

## Composição de período e alpha sólido — 2026-10-14

### Medição anterior à decisão

`lab/results/construction/diagnostics/tiling-overlap-2026-10-14/before.json.gz` preserva fontes, owners, HEAD, UTC e working tree. `tiling-nativo-2026-10-14/final-renders` e `remaining-analysis.md` registram cell8×8/pitch4×4 alpha50%: PDF vermelho opaco; SVG só uma contribuição; PNG quatro contribuições (255,15,15). `pdf_tiling.rs:184` emite BBox=size, `svg/paints.rs:148` usa overflow visible; `stream/shape.rs:155` e `stream/text.rs:275` descartam alpha. São defeitos observáveis de composição; o vanilla também corta overlap e não serve de requisito para esse defeito. Hipótese refutável: compor explicitamente um período preserva controles sem overlap e quatro contribuições coincidem com fonte-over analítica.

### Responsabilidade desta unidade

Possuir helpers privados para opacidade sólida de cada folha FrameItem, par (fill,stroke) e nome determinístico por bits f32, sem quantização u8. Shape considera só Solid em cada role; texto/glyph/separator usam alpha da cor para ambos (incluindo faux-bold), Line usa stroke; demais folhas/containers opacos. Gradient/Tiling não herdam alpha de uma cor representativa. Gerar operador ExtGState somente se algum alpha difere de1. Esses helpers não criam objetos nem recursos.

### Aceitação

Correção interna ADR-0127 dentro do T1–T3 já aprovado, sem nova API/fase/flag/default. L0 primeiro, RED→GREEN, composição analítica (incluindo ordem com cores distintas), controles simples/nested/rotated/text/image e extração/seleção de páginas. Build, suíte workspace, formato, Rustdoc e linhagem. Alpha variável de gradientes e qualidade PNG seguem incompletos; não certificar pacote de produção com esses itens abertos.
