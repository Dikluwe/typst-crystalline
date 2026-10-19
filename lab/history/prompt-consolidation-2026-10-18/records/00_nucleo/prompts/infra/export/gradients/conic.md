# L0 — infra/export/gradients/conic
Hash do Código: 4e21cba4

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/pdf-conic-mesh.toml sha256:90087538b1ce25b44a6d22122c3981b22ec40d33c5166ebfb8db3b8e67900954
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/gradients/conic.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/gradients/conic.rs:17` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## PDF conic Coons mesh

Own conic RGB sampling, circular-arc control points, patch-count helpers and
binary Type 6 Coons mesh emitters for RGB, CMYK and spot. Preserve this mesh
strategy without claiming upstream tessellation identity.

The production mesh follows the geometry and sampling refinement below. Historical
patch-count helpers remain test-only evidence of the former policy; fixed counts,
8-bit coordinates and a circle inscribed in the shape are superseded. Preserve
Type 6 Coons patches, not the defective geometry or stop-to-patch association.
The builder owns resource dictionaries and physical placement; this owner emits
payload plus matching coordinate Decode bounds.

## Verification

Check geometric continuity, angular direction, process/CMYK/spot component widths
and valid Type 6 decoding. Binary encoding validity is a format contract; patch
counts and equality to an upstream algorithm are not language parity.

## Cálculos numéricos dos exportadores — 2026-10-13

### Medição anterior à decisão

`03_infra/src/export/gradients/conic.rs` nas linhas [58, 60, 61, 169, 170, 278, 279] usa métodos apontados pelo Clippy. `clippy.toml` exige alternativas sem dependência da matemática da plataforma; libm =0.2.11 já está pinado no core/Cargo.lock. Os fontes da biblioteca, consumer, owner, manifest/lock, spans, HEAD, working tree não commitado, UTC, status e diffstat estão em `lab/results/construction/diagnostics/gradientes-numericos-ci-2026-10-13/before.json.gz`.

`before-renders.json` registra 24 fontes renderizadas em PDF/SVG por Tekt e pelo vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8, com hashes dos binários e comandos. `before-comparison.json` mede o canvas inteiro sem máscaras ajustadas pelo erro. Existem divergências anteriores; o corpus não é prova de paridade integral. A primeira tentativa de fixture usava hsv fora do namespace color, foi rejeitada pelo parser antes de qualquer mudança produtiva e está preservada em initial-fixture-error.log; cases.json contém as entradas válidas.

### Decisão e aceitação

Usar libm 0.2.11 em precisão f32: tanf no offset de arco e sincosf para os pares (seno, cosseno). Preservar diferença angular, ordem de multiplicação, centro/raio, progressão, amostragem e quantização dos meshes RGB/CMYK; spot continua consumindo a geometria existente. Nenhuma nova estratégia de tesselação ou transformação de bbox.

Dependência direta interna de L3 fixada em =0.2.11 no manifest, reutilizando a versão já no lock. Não atualizar versões de pacotes nem adicionar I/O. Mudança interna em fluxo contínuo ADR-0127, sem contrato público, default, fase ou estratégia novos. L0 precede código e resselo é limitado aos owners envolvidos.

Validar as fórmulas com a suíte existente, exportação dos mesmos 24 documentos antes/depois e leitura raster/estrutural de PDF/SVG. Comparar a preservação local separadamente das divergências contra o pin vanilla; nenhuma diferença nova de render é aceita sem análise. Não alargar tolerâncias/limites, mudar fixtures ou supressões para esconder uma falha. Compilar todos os targets, executar a suíte infra, verificar build, Clippy, Rustdoc, formato e linhagem. Contagens e identidade de arquivos não certificam a paridade da linguagem.

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

Usar Type 6 com coordenadas de 16 bits; emitir conjuntamente dados e limites
Decode. A geometria recebe domínio privado com tamanho físico da referência e
limites normalizados do canvas. Converter centro Y-down para Y-up. Para t em
[0,1], direção física theta=pi-angle-2*pi*t; normalizar angle antes da trigonometria.
Escalar direções por largura/altura para conservar o ângulo físico. O raio cobre
a maior distância física do centro aos cantos do domínio, com margem declarada
de 0.1% para quantização; Decode expande o raio em 1% para incluir controles.
Subdividir cada arco em no máximo 1/128 de volta; usar f64/libm para coordenadas.
A fórmula cúbica de arco continua r*(4/3)*tan(delta/4). Costura final reutiliza a
direção inicial exatamente, mas conserva a cor final. Centro/pontos não são
clampados à caixa unitária; apenas valores codificados ficam dentro de Decode.

Construir intervalos a partir de effective_offsets, preservando paradas explícitas,
auto-spacing, extensão constante anterior/posterior e limites laterais de offsets
repetidos. Não interpolar através de descontinuidade nem fechar a cor final na
primeira. Cada intervalo usa Conic::sample com duas paradas reparametrizadas,
conservando o interpolador L1 e o espaço declarado; não duplicar conversões de
espaço. Além do limite angular, subdividir se cores em 1/4,1/2,3/4 desviam mais
de 1/255 da interpolação linear de componentes emitidos, até profundidade 8.
A profundidade é limite de trabalho, não prova universal de erro. RGB/CMYK/spot
usam os mesmos intervalos/geom; CMYK conserva quatro componentes nativos e spot
o tint de um componente. Colorantes/Separation e tratamento geral de alpha não
mudam. Vazio produz stream vazio; uma parada produz cor constante. Domínios
singulares ou valores não finitos não são certificados por esta correção.

Helpers antigos de contagem/amostragem e os três wrappers de payload unitário
podem ficar sob cfg(test), delegando ao emissor produtivo quando aplicável.
Nenhum emissor antigo continua como caminho produtivo paralelo.

Correção interna de fórmula/paridade ADR-0127, sem contrato público externo,
modo/flag padrão ou mudança de fase. L0 precede testes RED e código. Não mudar
expectativas funcionais para absorver erro; assertions mecânicas obsoletas são
substituídas pela verificação independente descrita. Comparar corpus completo,
validar qpdf, raster, todas as regressões e linhagem/reverso. A referência
binária cônica só pode ser regenerada após demonstrar a mudança intencional
pelo harness e pelo protocolo vigente do corpus.

### Centro representável — refinamento após rasterização

render-controls/results.json mede reconstruções qpdf independentes: roundtrip
preserva pixels, AntiAlias não muda os casos, e deslocar Decode para representar
o centro exatamente reduz a diferença do caso angle de 107 para 15 pixels acima
de 16 níveis. A fonte e o estado anterior estão em center-quantization-before.json.gz.
Com 65536 códigos, limites simétricos colocam o centro entre dois códigos, e
round desloca o centro físico. Isso é erro de quantização evitável, não escolha
intencional de centro. Fazer o centro corresponder ao código 32768: limite
inferior center-R e superior center+R*(32767/32768), com R=raio*1.01. A expansão
continua incluindo controles. Exigir RED→GREEN do centro decodificado e recoletar
raster completo. Diferenças de rasterização na costura e contornos permanecem
registradas; não copiar a linha branca observada no topo do vanilla.

### Exaustividade de componentes

A primeira validação completa registra em first-validation/followup-architecture.log
um V16 novo em conic.rs:223: o fallback `_ => 3` mascara futuros espaços de cor.
Enumerar nominalmente Oklab, Oklch, Srgb, Luma, LinearRgb, Hsl e Hsv no braço
RGB; preservar Spot=1 e Cmyk=4. É explicitação do contrato já vigente, sem mudar
amostragem, bytes ou superfície pública. Revalidar todos os alvos e o corpus.


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

Migrar fixtures de FrameItem::Shape para FramePaint/FrameStroke, preservando valores e asserts semânticos existentes. Tiling deve construir célula preparada explícita; fixtures de avaliação continuam declarativas. Não adicionar conversão produtiva total nem enfraquecer asserções.

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
