# L0 — compiler/layout/grid/tracks
Hash do Código: cc4a515a

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0


## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/layout/grid/tracks.rs`. Comportamento original em `01_core/src/compiler/layout/grid.rs:212-451` no HEAD `51a027f99`, árvore limpa, 2026-10-01 00:44:23 UTC. A classificação é extração interna, não nova prova bilateral de paridade ou desempenho.

## Contrato

Own Grid/Table column and row track resolution. Keep Fixed/Auto/Fraction order, constrained cell measurement, insets, page break before fractional row allocation, auto-height finite fallback, and current cursor/style effects. Return widths, starts and row heights without changing placement or emission. Preservar contrato público, fase do pipeline, defaults, compatibilidade e pureza L1.

## Aceitação

Suíte L1, `cargo build` e `crystalline-lint .` passam; a extração não implica ganho de desempenho.


## Produto medido consumível — 2026-10-05

Evidência em tracks.rs:175 e cell_body.rs:155 e diagnóstico `00_nucleo/diagnosticos/reuso-sem-transformacao-2026-10-05.md` precede a decisão. Devolver também a pool privada do owner measurement_cache e transferir para ela o produto completo descartado após usar a altura Auto. O helper decide elegibilidade; Fixed/Fraction e cálculos geométricos continuam vigentes. Delegar invariantes de cache ao owner canônico `00_nucleo/prompts/compiler/layout/grid/measurement_cache.md`. Mudança privada de mecânica, na mesma fase e fluxo contínuo ADR-0127.

## Sequência em coluna Auto e frame literal — 2026-10-06

### Medição anterior à decisão

`00_nucleo/diagnosticos/block-tabela-fluxo-2026-10-06/before.json` conserva
estado/HEAD/stat/status/fontes L0 e código/binários. O lote predecessor em
espaco-terminal-2026-10-06/verification.json.gz mede seis tabelas: ABC/123 e
árabe ficam em duas linhas no Tekt, uma no vanilla pinado 9f2b6e871.
measurement.rs:104-113 toma max das larguras de filhos e soma alturas de
Sequence, mesmo quando Styled/Text/Space formam um fluxo inline. O vanilla
inline/grid mede o frame completo (grid/layouter.rs:967-1100). Block RTL com
LTR literal terminal desloca ABC em 3.49658pt por espaço: block.rs:315 usa
line_content_right que exclui todo TextSeparator. page_geometry.rs:143-154
já distingue extensão auto de espaço literal da extensão de tinta da linha.
tracks.rs:108-113 ancora toda coluna em margin.left, inclusive uma coluna
Auto RTL; no oracle a coluna única termina na margem direita. Observáveis
de geometria/texto, não obrigação de copiar algoritmo ou bytes. Hipótese
refutável por quebra/âncora ainda divergente ou regressão de espaço fraco,
coluna fixa, estilos/linhas/parágrafos após medição de frame/âncora corretas.

### Classificação

Correção interna de paridade ADR-0127, sem API pública/fase/flag/dependência
ou unsafe novo. Não inferir intenção histórica de P80. Núcleo de extensão
intrínseca distingue dimensões do container de tinta/alinhamento de linha.

### Responsabilidade exclusiva

Para uma coluna em região finita, alinhar a âncora do grid ao End RTL da
chain quando não há override horizontal, usando available_width menos soma
das larguras resolvidas, sem alterar larguras nem reverter ordem lógica.
Override Left/Right/Center/Start/End resolve contra direção externa. Colunas
múltiplas conservam a política anterior nesta rodada: seu espelhamento e spans
precisam de medição própria. Origins/insets/linhas/bordas consomem col_starts
existentes. Larguras Auto continuam delegando à medição restrita, agora apta
para Sequence inline. Fixed/Fraction, cache de células e fases conservados.


## Origem e override horizontal da região — 2026-10-06

Medição: os controles exploratórios de Align no diagnóstico
`00_nucleo/diagnosticos/bordas-cantos-2026-10-06.md` divergem na âncora Left
RTL de uma coluna. `tracks.rs:104-116` restringe o offset a uma coluna e usa
margem da página como origem, mesmo em sub-região.

Responsabilidade exclusiva: resolver um override horizontal explícito para
a largura total das colunas em qualquer contagem, usando região finita e
a origem corrente line_start_x. Sem override, preservar o caminho de uma
coluna e a política anterior de várias colunas; não inverter ordem lógica.
O fluxo lexical que instala/restaura o override pertence ao owner canônico
`00_nucleo/prompts/compiler/layout/placement.md`. Não duplicar a tradução no
emissor de células. Fixed/Auto/Fraction, paginação e cache permanecem.
Correção interna de posição ADR-0127, sem mudança pública.

## D08B3-A — tabela resolvida na inserção; proposta pendente ADR-0127

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
00_nucleo/diagnosticos/table-cell-realizacao-d08b3-2026-10-07/
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

Na entrada preparada medir o visual Complete em coluna/linha correta, usando colspan/rowspan e coords do plano, nunca chunk de células de origem como grade nem raw body substituído. Conservar tipos de tracks, metrics/styles e fórmula/insets/cache; corrigir só adaptadores necessários. Grid legada conserva ordem anterior. Não resolver recipes aqui.

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

Medição anterior à decisão: 00_nucleo/diagnosticos/table-cell-realizacao-d08b3-2026-10-07/{after-second,refuters-second}.json.gz, pin/binários/HEAD/estado/fontes/UTC/argv/resultados completos.
after-second/show-bare mede/renderiza CELL e a em linhas diferentes. measure_content_constrained não possui ramo TableCell e não deve receber esse wrapper. Table preparada mede r.visual e usa propriedades de render do default sobrevivente via helper cell_body; substituição total tem inset zero. Coordenadas/spans do plano permanecem geométricos; Cell lexical conhecida não é mutada.
A aceitação é bilateral, efeitos uma vez e geometria; não normalizar whitespace nem alterar contratos ratificados.

Medição de fonte adicional: compiler/layout/measurement.rs:374 retorna zero para
TableCell/Table sem ramo próprio. Para a visual preparada, remover apenas
snapshot Cell do objeto de medição, conservando Styles. Conteúdo com Table
aninhada requer medida física por subframe já realizada (sem Func); ASCII
simples permanece na medida estática e no cache comprovado. Não inferir width
pelo texto do getter nem abandonar controles de contagem de subframes.

### Refutação e decisão interna antes de código — 2026-10-07

Medições em 00_nucleo/diagnosticos/table-cell-realizacao-d08b3-2026-10-07/{after-fourth,refuters-fourth}.json.gz, nested-reuse-red.log e effect-order-red.json: fontes/binários/HEAD/estado/UTC/resultados.
after-fourth/aliases e show-cell-no-default-escape permanecem com quebra de texto. measurement.rs:21–31 não certifica ShowRecipe dentro de Sequence como inline. Projeção para medir deve remover snapshots de Cell e wrappers ShowRecipe recursivamente, conservando Styled e fronteira de Table completa. Não percorrer nem retirar cells/coords de malha aninhada; ali o renderer mede plano físico por subframe. Projeção total é pura, não executa receita. Guardas originais de cache e medidas equivalentes continuam obrigatórias.
Supersede apenas a projeção/guarda aqui medida, sem contrato público ou fase adicional. Refutação: diferença nos testes/recoletas, efeitos repetidos, fonte ocultada visível ou regressão de cache/geometry.

### Medição e correção suplementar interna — 2026-10-07

Proveniência: diagnosticos/table-cell-realizacao-d08b3-2026-10-07/geometry-fifth.json e local-bool-red.log, binário 59da863f, fonte/HEAD/estado/UTC/argv/glifos/testes.
geometry-fifth.json registra quebra SECOND/d e alias LOCAL/a. break_selection.rs:24 compara coordenadas absolutas por > estrito; largura medida em origem zero e soma/transporte para origem da página podem diferir no último bit. Coluna Auto de plano preparado acrescenta limite de erro de representação EPSILON vezes escala geométrica (largura disponível + origem + largura natural), somente para largura positiva finita. Não ajustar texto, não introduzir tolerância editorial nem alterar Grid legada; testar glifos/linhas e cache. Subframe terminando com Table tem last_sub_frame_width zerado (sub_frame.rs:326), portanto medida usa bounds reais de FrameItems (metrics::line_content_right), conservando os corpos completos. Bool raw é conteúdo físico, usar esse mesmo fallback em vez da medida estática sem ramo Raw. Projeção permanece sem Func, plano já completo e sem extrair células de Table aninhada.
Não certificar antes da recoleta RED→GREEN e suites; nenhum campo/método público, flag ou mudança de fase suplementar.

### Refutador de largura auto contra largura explícita — 2026-10-07

width-refuter.json (binário ed82be14, snapshot tested-seventh, fontes/argv/UTC)
mede OUTER LOCAL/a com Auto, mas uma linha com79.167pt explícitos. Logo não é
um problema da receita ou dos glifos; a medida Auto subestima o fluxo composto.
outer-alias-visual.json observa sequência OUTER,space,LOCAL,space,a. O helper
measurement.rs:75–92 cria instância isolada para sequências; essa equivalência
não está provada para o fluxo transportado. Para visual de Cell preparada com
mais de um item inline material, medir por subframe na instância causal de
layout, com corpo completo, bounds reais e restauração existente. Text simples
mantém medida estática/cache; Metadata/Empty não tornam fluxo composto. Sem
segunda receita nem margem editorial. Validar fonte/glifos e controles cache.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: 00_nucleo/diagnosticos/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em materialization/table-origens-d08b3c2-2026-10-07/contract.md.

additional-debug3 mede FUNC e styled em linhas separadas: Source não torna produto composto atômico. Classificação composed_visual atravessa body e zeros metadata; medir composição inteira em subframe existente antes de decidir auto-width, preservando geometria e propriedades externas.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.


## C2-R1 — correção interna de paridade

Medição anterior à decisão: 00_nucleo/diagnosticos/performance-c2-2026-10-07/table-local-regression.json, três samples/r*-table-local/samples.json com fontes/argv/UTC/estado/hash do release ac889581 e vanilla pin9f2b6e871. Tamanho cresce10%, segunda coluna x60.880→38.999 com sobreposição. RED semântico em 00_nucleo/diagnosticos/table-local-c2r1-2026-10-07/red.log: tamanho10pt e separação Auto falham, tamanho local explícito11pt já passa.

Fonte upstream typst-eval/src/code.rs:32–43 aplica somente styles da declaração set à cauda. Tekt expression_dispatch.rs:132 colapsa raiz/defaults e não só declarações locais; style_chain.rs:304–315 contém size11/boldfalse/italicfalse na raiz. tracks.rs:328 reconhece somente Cell com resolved, deixando a Cell standalone Complete na projeção; measurement.rs não mede TableCell. É semântica/estilo/posição observável, não obrigação de reproduzir árvore ou algoritmo upstream. A causa conjunta é inferência refutável por fonte/tamanho/posição ainda divergentes após isolar estilos locais e medir o visual completo.

Classificação ADR-0127: reparo interno de paridade, sem API pública/trait/entidade/flag/fase nova. Fluxo contínuo autorizado por pedido de corrigir prioridade1; L0 primeiro, RED→GREEN, resselo restrito, builds/suítes/lint e recoleta física/temporal.

Responsabilidade: measurement_visual de TableCell preparada, posicionada ou standalone com stage conhecido, projeta e mede e.visual(), preservando Styled/Source em torno do texto. Raw Cell sem stage conserva fallback anterior; Table aninhada permanece atômica e não tem células extraídas. Nenhuma recipe/callback é executada na medição. Mesma projeção para auto-width e rows; inset/align do plano externo, tinta, placement e cache não mudam. Validar duas colunas sem overlap, estilo de texto10/11, local aliases, medidas físicas PDF e corpus C/C2.


## Tipos explícitos e leitura local — type_complexity (2026-10-15)

### Medição anterior à decisão

01_core/src/compiler/layout/grid/tracks.rs:225 (C013).
O snapshot before.json.gz em 00_nucleo/diagnosticos/type-complexity-2026-10-15
conserva HEAD, working tree não commitado, UTC, diffstat, fontes/owners e hashes.
O gate red-clippy.log falha com este lint promovido a erro antes da edição.
A inspeção distingue tuplas posicionais, repetição do alias FontKey e contratos
estáticos já explícitos. É manutenção de representação Rust; não medição de
benefício da IA, bug funcional ou paridade nova.

### Responsabilidade e aceitação

Nomear o registro privado RowCell com cell_index, content emprestado, column, colspan e rowspan. A coleção continua Vec<Vec<RowCell>>, com Clone/Copy dos mesmos valores/referências. Construir e desestruturar por nomes; preservar particionamento, ordem, spans, limites, alturas e captura do produto medido. Não copiar Content nem alterar cache/placement.

Correção interna em fluxo contínuo ADR-0127: nenhuma entidade, assinatura pública
resolvida, default ou fase é alterada. Uma abreviação de tipo só é permitida com
expansão comprovadamente idêntica; incompatibilidade ou tipo público novo exige
outro gate. As exceções são nominais apenas para type_complexity; não autorizar
supressão em módulo/impl/crate nem elevar limiar. Esta permissão pontual supera
proibições genéricas anteriores somente para os três itens justificados neste lote;
continuam proibidas supressões dos demais casos destinados à correção.
Usar os controles funcionais existentes sem mudar seus esperados. O gate selecionado
deve passar RED→GREEN, incluindo unfulfilled_lint_expectations como erro; revalidar
build, Clippy all-targets, suíte workspace, formato, Rustdoc e linhagem com resselo
restrito. Não criar testes que apenas espelhem aliases ou nomes de campos.
