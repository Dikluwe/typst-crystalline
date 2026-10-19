# L0 — compiler/layout/grid/cell_body
Hash do Código: 78a92043

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/layout/cell-bidi-scope.toml sha256:dd8c7661f2246385e2aed1737e8494d778b9f563b7ac709fc3c750a23fe50596
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee


## Medição anterior à decisão

No HEAD ea7ae25ab (árvore limpa, 2026-09-30), a implementação vigente está em `01_core/src/compiler/layout/grid/cell_emission.rs:95-583`. A medição identifica o limite do bloco a extrair; não mede desempenho nem altera a paridade de linguagem.

## Owner e contrato

Camada L1. Consumer produtivo único: `01_core/src/compiler/layout/grid/cell_body.rs`. Dispor uma célula posicionada, com inset, alinhamento, subframe, decoração e caudas de overflow. Preservar save/restore do estado, ordem dos itens e índices de alinhamento adiados. Esta extração mantém contrato público, defaults, fase do pipeline, compatibilidade e pureza L1.

## Aceitação

O despacho permanece estático. Testes L1, build e crystalline-lint devem passar. Não inferir ganho de desempenho da separação.


## Produto medido consumível — 2026-10-05

Evidência em tracks.rs:175 e cell_body.rs:155 e diagnóstico `lab/results/construction/diagnostics/reuso-sem-transformacao-2026-10-05.md` precede a decisão. Solicitar o produto medido ao helper measurement_cache antes de instalar região da célula, depois de calcular body_x/body_w e alinhamento efetivo. Sem hit executar o subframe vigente; com hit conservar exatamente os pipelines posteriores de decoração, rebase, clipping, overflow, preenchimentos e caudas. Regras de validade e métricas laterais pertencem ao helper. Delegar invariantes de cache ao owner canônico `00_nucleo/prompts/core/compiler/layout/grid/measurement_cache.md`. Mudança privada de mecânica, na mesma fase e fluxo contínuo ADR-0127.



## Célula textual certificada — refutação 2026-10-06

### Medição anterior à decisão

focused-candidate-verification.json.gz e candidate-source.json.gz em
`lab/results/construction/diagnostics/block-tabela-fluxo-2026-10-06/` preservam candidato
com largura/âncora corretas mas ordem ABC/123/árabe incorreta em RTL.
bidi_origin.rs:111-117 recusa todas as células/subframes; o input integral
não chega à reordenação L3. cell_body.rs:223-235 classifica Text e TextShaped
como texto para rebase de baseline, mas não TextSeparator, introduzindo
um delta de top-edge nos separadores da mesma linha. São duas causas
internas refutáveis por ordem/baseline ainda divergentes após certificação
limitada e recomposição consistente. Correção de paridade interna, sem
contrato público, fase, flags de produto ou unsafe novo.

### Responsabilidade exclusiva

Instalar/restaurar capacidade privada somente para col_starts.len()==1 e body_w finito, no trecho de layout da célula. Classificar TextSeparator junto de Text/TextShaped ao rebasear sua baseline; não acrescentar avanço, altura ou tinta. Produtos cacheados ASCII continuam com certificado de equivalência vigente; não recertificar frames por string/posição.

## D08B3-A — tabela resolvida na inserção; proposta pendente ADR-0127

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/
{before,additional,order,aliases}.json.gz registra pin 9f2b6e8715237cb086899a42873660fe744622e8,
UTC, diffstat/status, fontes/binários, argumentos e resultados. Oracle binário
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; Tekt
7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2.
Vanilla lab/typst-original/crates/typst-library/src/layout/grid/resolve.rs:204–278 resolve campos, :1500–1530 rejeita
conflitos na ordem, :1665–1703 cria vazios, :1945–1988 deriva breakable.
Tekt grid_placement.rs:91–112 prioriza explícitas e :262–270 só conhece GridCell;
realize/mod.rs:160–200 visita receitas bottom-up; table.rs:127–152 despeja
wrappers sem atributos. order mede zero efeitos de células quando a tabela é
substituída e zero efeitos de heading quando a célula o oculta; aliases mede
LOCAL a, OUTER LOCAL a e NEW a. É linguagem, sem copiar mecânica/bytes/igualdade
Rust. Hipótese refutada por campos desconhecidos, efeito oculto ou ordem errada
após a mudança; cache/paginação não são inferidos como equivalentes.

### Responsabilidade exclusiva proposta

Prepared TableCell consome resolved.visual/props, incluindo fill/stroke None conhecidos que não herdam novamente do container. Alinhamento/inset conhecidos, bounds/overflow/decoração/caudas e conservação de estado permanecem. Fallback Grid/raw mantém or/fold histórico. Usar cache somente sob prova de equivalência atualizada pelo seu owner; não executar recipes.

### Gate, aceitação e divisão explícita

Proposta não vigente até confirmação humana do manifesto: há cinco campos públicos,
dois carriers públicos e uma fachada pública nova, além da fase lógica antes de Cell
show. Após ratificação, RED→GREEN, suites/build/lint/preservação restrita e coletas
D08A/B2, D05/D06/D07/query obrigatórios. Aceitação em linguagem/efeitos/geométricas
reais, nunca igualdade de bytes ou normalização de whitespace para fechar render.
Este primeiro pacote D08B3-A é incompleto no universo Cell: D08B3-B completa defaults
de Cell isolada (raw-cell-no-placement/outside-fields) e decisão separada de geração
fresca show-cell-reconstruct. Defaults/fields genéricos de Table (table-root-equality),
styles funções/paint/strokes/insets ricos, linhas/gutters ricos, spans multipágina e
conformidade PDF/UA continuam fora da certificação. Não mudar casts legados extras
do D08B2, sintaxe, flags ou política não-localizável global neste pacote.

### Refinamento interno medido — 2026-10-07

Medição anterior à decisão: lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/{after-second,refuters-second}.json.gz, pin/binários/HEAD/estado/fontes/UTC/argv/resultados completos.
after-second/show-body: vanilla ab, Tekt a b. Inset/alignment do default show da Cell só são desenhados se sua receita emitir o snapshot da Cell. A medição ab refere espaçamento; não prova perda de fill/stroke, que a precisão C abaixo reconhece como tinta independente do plano da Table. Um helper puro neste owner reconhece somente snapshot preparado sobrevivente (wrappers semânticos transparentes), para ser compartilhado por tracks; sem reflexão pelo texto/PartialEq e sem callbacks.
A aceitação é bilateral, efeitos uma vez e geometria; não normalizar whitespace nem alterar contratos ratificados.

## D08B3-C — tinta do plano da Table separada do default show da Cell

Medição anterior à decisão: table-cell-styles-d08b3c-2026-10-07/physical-before.json.gz conserva PDF/SVG, fontes, HEAD/working tree, UTC/argv/hashes completos no pin 9f2b6e8715237cb086899a42873660fe744622e8. fill-with-body-recipe e fill-with-hidden-recipe conservam retângulo red no vanilla, ausente Tekt. stroke-with-body-recipe conserva red 2pt vanilla contra black 1pt Tekt; default-with-body-recipe conserva black 1pt nos dois. Fonte vanilla layout/grid/resolve.rs:218–237 e :270–282 retorna Cell.fill/stroke antes de seu body passar por show; typst-layout grid/layouter.rs desenha esses campos independentemente do default show. Tekt cell_body.rs:61–63 condiciona fill à sobrevivência do snapshot, borders.rs:43–48 lê stroke lexical. O refinamento A mediu espaçamento/ab, não tinta: sua generalização de sobrevivência para fill/stroke é refutada e supersedida por esta cláusula.

Decisão contínua interna ADR-0127: TableCell posicionada usa fill/stroke/breakable do plano externo TableCellResolved, independentemente de recipe it/body/substituição; None conhecido vence qualquer fallback de container. Apenas inset/alignment derivados do default show continuam dependentes do snapshot sobrevivente reconhecido por prepared_default, com geometria/medidas sob owners existentes. Borders consome diretamente stroke externo conhecido; Cell body emite o fill externo antes do produto visual. Não executa conteúdo ocultado nem muda fonte lexical, API pública, fases, render default novo ou política Grid/crua. Mantém bounds/coalescência, clipping/caudas, ordem de efeitos e estado causal. RED→GREEN de tinta com produto substituído, células conhecidas None e regressão visual SVG real; builds/suítes/lint/preservação necessários.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md.

Medição additional-debug3 refuta Source como barreira ao default show sobrevivente: reconhecer snapshot/metadata zero por body transparente, sem perder inset/align nem tocar tinta externa do plano.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.

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
consumer as construções/desestruturações antes da edição: [360].
layout_types.rs:531–539 não distingue raízes de grupos técnicos; helpers.rs:253–269
reconstrói o Group e precisará preservar o campo. Box agrega fundo/corpo;
Block mantém body plano exceto no clip. Preservação do referencial é requisito
de linguagem; formato dos envelopes é mecânica. A aprovação não fecha as
divergências gerais de geometria previamente medidas em Rotate/Box.

### Responsabilidade exclusiva

Migrar construções e padrões de Group para o campo paint_root aprovado. Grupos técnicos novos e fixtures legadas são false; reconstruções transportam o valor recebido, sem inferir raiz a partir de dimensão, clipping ou transformação. Preservar os demais campos e o comportamento de desenho/recursos. Exportadores ainda não usam este metadado para patterns; essa integração é T2 e não está certificada aqui.

### Aceitação e incompletude

L0 primeiro, testes RED→GREEN, resselo restrito; conservar geometria e texto
nos controles fonte→PDF/SVG e revalidar suíte/build/doc/check/fmt/linhagem.
A migração deste metadado não substitui Shape.fill/stroke por FramePaint nem
implementa patterns PDF/SVG/raster; T1c/T2/T3 continuam explicitamente abertos
e já autorizados. Não alegar paridade de tiling ou produção a partir do build.


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

Converter somente o fill Color já resolvido do backing para FramePaint::Solid. Conservar o plano da célula, body, layout, clipping, caudas e semântica.

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
