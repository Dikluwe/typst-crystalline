# L0 — builder/resources
Hash do Código: 0f11047f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/pdf-conic-mesh.toml sha256:90087538b1ce25b44a6d22122c3981b22ec40d33c5166ebfb8db3b8e67900954
- 00_nucleo/prompts/_nuclei/pdf-conic-placement.toml sha256:9010358e51cfa3267960432244b972b179d447f572c902101eba650f6d6a67cb
- 00_nucleo/prompts/_nuclei/pdf-solid-opacity.toml sha256:8b7fcfaf39e60540d16868e662d48fb466fab58860385fcdae499f8c2edfe799
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee


**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/builder/resources.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/builder/resources.rs:46` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Resource aggregation

Own page ColorSpace entries, spot-colorant collection and Separation dictionaries,
gradient-object emission and image-XObject aggregation. Consume image and gradient
payloads from their specialized owners; do not redefine decoding, ICC profile
selection, gradient sampling or conic mathematics here.

Verbose resources include /c0 referencing the shared sRGB ICC profile, even when
there is no RGB JPEG. Compact retains its existing resource policy. An RGB JPEG
requires the shared profile; grayscale and CMYK keep their own image paths.
Collect selected-page spot colorants in encounter order across background, body,
foreground and nested content, including text, line, shape fill/stroke and gradient
stops. Identity is the complete colorant, not only its name. Emit deterministic
resource names by delegating complete-colorant naming to
`00_nucleo/prompts/infra/export/stream/shape.md`, and correctly escaped PDF names,
including None and All. Separation
alternate spaces and Type2 tint functions preserve the declared fallback: RGB uses
the shared ICC when available, and CMYK starts at zero ink. Do not replace /c0.

Emit supplied gradient and image objects at their allocated IDs with resolvable
references. Preserve spot-gradient Separation resources and the existing axial
Extend and stop-function routing without taking ownership of the gradient model.
Merge prepared bitmap /ImN entries into page /XObject dictionaries alongside normal
images; preserve both when the page uses both. Excluded pages must not introduce
page-local spot inks into an included page's resource dictionary.

## Verification

Check RGB JPEG, grayscale/CMYK, verbose/compact, repeated and same-name distinct
spot colorants, gradient roles and mixed bitmap/image resource closure.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`03_infra/src/export/builder/resources.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

Recolher estilo de fonte do separador para recursos PDF, inclusive grupos e links.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

## Posicionamento cônico — 2026-10-14

### Medição anterior à decisão

`03_infra/src/export/gradients/mod.rs:224-264` perde a posição própria de
Conic RGB/CMYK e deduplica por Arc/bbox; `builder/resources.rs:564-574`
emite matriz identidade nesses ramos. `stream/draw.rs:522-555,939-965`
compõe grupos, mas Pattern é relativo ao espaço inicial do stream.
`stream/page.rs:45-49` acrescenta bleed antes dos itens. O corpus constante
red versus conic(red,red), em before/probe.json.gz, reproduz a perda de
cobertura na árvore atual. Fonte, L0, HEAD, UTC, status e diffstat estão em
`00_nucleo/diagnosticos/conic-posicionamento-2026-10-14/placement-before.json.gz`.
Os dois lados do teste são Tekt: o defeito independe de imitar o vanilla.

### Responsabilidade e aceitação

Emitir em todos os espaços de cor de Conic a matriz já preparada pelo coletor. Conservar IDs, Type 6, Decode, payload e cores; não recalcular amostragem ou inferir posição da primeira página. Ler seis componentes afins, preservando a forma decimal anterior dos padrões diagonais spot para não alterar fixtures de formato sem necessidade.

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

## Geometria e amostragem cônica — 2026-10-14

### Medição anterior à decisão

`conic.rs:142-143,241-242` fixa centro/raio; `:271-272` associa paradas CMYK
por índice circular, ignorando offsets e a amostra final. `resources.rs:508-552`
fixa coordenadas de 8 bits e Decode unitário. As sondas antes/depois de
posicionamento e refuters no diagnóstico conic-posicionamento-2026-10-14 provam
cantos brancos, centro ignorado e offsets CMYK sem efeito, apesar de sample()
responder corretamente. before.json.gz em conic-geometria-amostragem-2026-10-14
conserva fontes/L0, HEAD, working tree, diffstat, UTC e os binários. O corpus
bilateral mede centro e cores nos eixos/diagonal, quadrado/retângulo e rotação.
A fonte pinada typst-pdf/src/paint.rs:274-299 e os pixels concordam: o início
é à esquerda e o sentido é horário; o código Tekt anterior não corresponde.
O vanilla observado nesses controles não é um fallback uniforme.
PDF Reference 1.3, tabela 4.31 (página PDF 250), permite coordenadas 16 bits e
Decode independente por eixo; é contrato de formato, não igualdade upstream.

### Responsabilidade e aceitação

Consumir o emissor único de malha e seu domínio preparado. Escrever BitsPerCoordinate
16 e montar Decode com quatro limites geométricos e os pares de componentes
corretos (RGB três, CMYK quatro, spot um). Conservar Type 6, IDs, cores nativas,
Separation, dummy Function e matriz física já validada. Não assumir Decode
unitário nem reamostrar cores neste owner.

Correção interna de fórmula/paridade ADR-0127, sem contrato público externo,
modo/flag padrão ou mudança de fase. L0 precede testes RED e código. Não mudar
expectativas funcionais para absorver erro; assertions mecânicas obsoletas são
substituídas pela verificação independente descrita. Comparar corpus completo,
validar qpdf, raster, todas as regressões e linhagem/reverso. A referência
binária cônica só pode ser regenerada após demonstrar a mudança intencional
pelo harness e pelo protocolo vigente do corpus.


## Transporte preparado em Shape — T1c/T2 aprovado — 2026-10-14

### Medição anterior à decisão

`00_nucleo/diagnosticos/tiling-nativo-2026-10-14/before.json.gz` registra
HEAD, working tree, diffstat/UTC e fontes/owners antes desta migração.
layout_types.rs:510–511 ainda transporta Paint/Stroke; shape.rs:151–191
materializa só fill em grupos; tiling.rs:85 limita/clona a grade. O preparador
prepare_cell já retorna TilingFrame uma vez por ocorrência. As sondas
`tiling-preparacao-2026-10-14/{before,after}-renders/results.json.gz`
comprovam que dense-open e stroke-open ainda divergem nos três targets.
O contrato foi aprovado no manifesto
`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`;
recibo em `00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
A preparação/contagem de objetos é mecânica de transporte; render, contornos,
texto, geometria e diagnósticos são obrigações observáveis de linguagem.

### Responsabilidade exclusiva desta unidade

Coletar colorantes spot nos paints preparados e células aninhadas, preservando PageFill declarativo. Emissão de gradientes segue contrato vigente.

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

Detecção de JPEG RGB percorre também células preparadas e camadas background/foreground para reservar ICC no modo compacto. O walker semântico continua usado pelo collector spot, que desce paints explicitamente, sem duplicação.

## Composição de período e alpha sólido — 2026-10-14

### Medição anterior à decisão

`00_nucleo/diagnosticos/tiling-overlap-2026-10-14/before.json.gz` preserva fontes, owners, HEAD, UTC e working tree. `tiling-nativo-2026-10-14/final-renders` e `remaining-analysis.md` registram cell8×8/pitch4×4 alpha50%: PDF vermelho opaco; SVG só uma contribuição; PNG quatro contribuições (255,15,15). `pdf_tiling.rs:184` emite BBox=size, `svg/paints.rs:148` usa overflow visible; `stream/shape.rs:155` e `stream/text.rs:275` descartam alpha. São defeitos observáveis de composição; o vanilla também corta overlap e não serve de requisito para esse defeito. Hipótese refutável: compor explicitamente um período preserva controles sem overlap e quatro contribuições coincidem com fonte-over analítica.

### Responsabilidade desta unidade

Coletar pares de opacidade das folhas via helper único no stream e percurso visual de recursos que desce células. Incluir canvas Solid e emitir ExtGState /ca /CA diretos, deduplicados em ordem determinística. Retornar os recursos no mesmo agregado privado usado pelos três builders e Forms de célula; não mudar API externa nem ICC/spot.

### Aceitação

Correção interna ADR-0127 dentro do T1–T3 já aprovado, sem nova API/fase/flag/default. L0 primeiro, RED→GREEN, composição analítica (incluindo ordem com cores distintas), controles simples/nested/rotated/text/image e extração/seleção de páginas. Build, suíte workspace, formato, Rustdoc e linhagem. Alpha variável de gradientes e qualidade PNG seguem incompletos; não certificar pacote de produção com esses itens abertos.
