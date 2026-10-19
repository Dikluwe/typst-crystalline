# L0 — compiler/layout/grid/borders
Hash do Código: b4ea069b

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/layout/grid/borders.rs`. Extração interna medida no HEAD `ce27af795` e árvore limpa em 2026-09-30 23:12:44 UTC. A fonte é o consumer pai desta unidade; a extração não é medição bilateral de paridade nem prova de desempenho.

A lógica extraída estava em `01_core/src/compiler/layout/grid.rs:1262-1499` no HEAD indicado. A classificação é extração interna de comportamento, não nova decisão de paridade.

## Contrato

Own cell bounds and effective strokes, horizontal and vertical segment emission, open vertical segment flushing, and row-border coalescing. Preserve stroke priority, geometry, item ordering and pagination boundaries. Do not perform placement or cell layout. Não alterar contrato público, fase do pipeline, comportamento por defeito ou compatibilidade.

## Aceitação

Os testes L1, `cargo build` e `crystalline-lint .` passam. Não inferir ganho de desempenho da divisão estrutural.


## Extensão geométrica dos extremos — 2026-10-06

Medição antes da decisão: no vanilla pinado 9f2b6e871,
`lab/typst-original/crates/typst-layout/src/grid/layouter.rs:568-578` e
`:779-791` deslocam o início por menos meia espessura e acrescentam uma
espessura ao comprimento dos segmentos verticais/horizontais. No consumer
`borders.rs:74-114`, os extremos terminavam na fronteira geométrica.
A rasterização do diagnóstico `lab/results/construction/diagnostics/block-tabela-fluxo-2026-10-06.md`
mostrou falta de tinta nos cantos das seis tabelas. A origem histórica não
foi demonstrada; não inferir intenção.

Decisão: corrigir a forma visível das bordas (paridade morfológica), sem copiar
a mecânica de ordenação do vanilla nem alterar tracks, células ou texto.
Cada segmento positivo emitido por este owner começa meia espessura antes
da fronteira e termina meia espessura depois, no próprio eixo. As dimensões
do Shape refletem o comprimento estendido; eixo perpendicular, stroke, fill
e ordem permanecem. Aplicar somente na emissão final, após coalescência, sem
acumular extensão no estado de segmentos abertos. Segmento vazio não emite.
Correção interna de paridade ADR-0127, sem API/default/fase/unsafe novo.

Aceitação: regressão RED→GREEN dos extremos horizontais e verticais para
espessuras distintas, vazio preservado; tinta dos controles bilaterais sem
lacuna nos cantos e sem regressões de texto/posição; build e lint válidos.

## D08B3-C — consumo de stroke resolvido e fold de show-set

Medição anterior à decisão: table-cell-styles-d08b3c-2026-10-07/show-fold-before.json.gz compara duas declarações show-set Cell/Table: vanilla combina red e 2pt, Tekt perde red. geometry-final.json.gz / cell-stroke-two-sets e cell-stroke-constructor registra PDFs/argv/binaries/HEAD/working tree/UTC, cuja conversão pdftocairo SVG contém stroke preto 1pt Tekt contra red 2pt vanilla. Tekt borders.rs:43–48 lê e.stroke lexical; apply.rs:244,465 colapsa deltas cell.stroke com top-wins. Vanilla model/table.rs:765 #[fold] e layout/grid/resolve.rs:225–237 resolve componentes; typst-layout grid/layouter.rs:568–578 desenha o stroke já resolvido. É cor/espessura visível e campos de linguagem, sem exigir igualdade de bytes SVG/PDF ou número de comandos de desenho.

Decisão contínua ADR-0127: helper privado puro TableCellProperties::chain_styles combina somente Value::Stroke declarativo em cell.stroke, mantendo None explícito, demais chaves/topologia/ordem/guards. Apply usa esse helper na acumulação de show-set em ambas as rotas existentes; realizer usa ao aplicar deltas locais. Não armazenar Cell/plano em Styles. Borders, para TableCell preparada, consome o stroke do plano externo TableCellResolved, inclusive None conhecido, independentemente do produto da receita; não faz fallback lexical/container. Grid e Table crua conservam prioridade histórica. Não mudar geometria de extremos/coalescência, assinaturas, fases, modos nem contratos públicos. RED→GREEN antes de código, cores/espessuras reais via SVG em unidades físicas, controles de default/None/substituição e corpus/suítes/lint/preservação obrigatórios. Alinhamento de linha fixa e resíduos métricos anteriores não são inferidos como resolvidos por este reparo.

## D08B3-C — tinta do plano da Table separada do default show da Cell

Medição anterior à decisão: table-cell-styles-d08b3c-2026-10-07/physical-before.json.gz conserva PDF/SVG, fontes, HEAD/working tree, UTC/argv/hashes completos no pin 9f2b6e8715237cb086899a42873660fe744622e8. fill-with-body-recipe e fill-with-hidden-recipe conservam retângulo red no vanilla, ausente Tekt. stroke-with-body-recipe conserva red 2pt vanilla contra black 1pt Tekt; default-with-body-recipe conserva black 1pt nos dois. Fonte vanilla layout/grid/resolve.rs:218–237 e :270–282 retorna Cell.fill/stroke antes de seu body passar por show; typst-layout grid/layouter.rs desenha esses campos independentemente do default show. Tekt cell_body.rs:61–63 condiciona fill à sobrevivência do snapshot, borders.rs:43–48 lê stroke lexical. O refinamento A mediu espaçamento/ab, não tinta: sua generalização de sobrevivência para fill/stroke é refutada e supersedida por esta cláusula.

Decisão contínua interna ADR-0127: TableCell posicionada usa fill/stroke/breakable do plano externo TableCellResolved, independentemente de recipe it/body/substituição; None conhecido vence qualquer fallback de container. Apenas inset/alignment derivados do default show continuam dependentes do snapshot sobrevivente reconhecido por prepared_default, com geometria/medidas sob owners existentes. Borders consome diretamente stroke externo conhecido; Cell body emite o fill externo antes do produto visual. Não executa conteúdo ocultado nem muda fonte lexical, API pública, fases, render default novo ou política Grid/crua. Mantém bounds/coalescência, clipping/caudas, ordem de efeitos e estado causal. RED→GREEN de tinta com produto substituído, células conhecidas None e regressão visual SVG real; builds/suítes/lint/preservação necessários.


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

Manter Stroke declarativo na escolha e coalescência de bordas. Somente após fechar um segmento positivo, preparar seu stroke pelo helper de tiling com o Layouter e emitir FrameStroke. Alterar assinaturas privadas de emit/flush para receber Layouter em vez de apenas buffer; conservar todas as coordenadas, ordem e decisões de prioridade. Não executar layout do body da célula de tabela; a preparação delegada é apenas do paint. Fixtures conferem geometria/campos, sem exigir PartialEq em FrameStroke.

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
