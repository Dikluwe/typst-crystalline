# L0 — word_layout
Hash do Código: 896e2465

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/bibliography-prefix-admission.toml sha256:64d61513bab0293f918958404cf4fe0387dea84cc53f5442faad2b467fa4e077
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:7b8a61eaafc8a07df12218f8f0560c33fae5c36f621b822b7350ccefc9774711
- 00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml sha256:3a97c056eff81b344f46d83567274225e3519ae141fce4817e342d573a92fb41
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-tracking.toml sha256:3b0c535436048536ecd3133fcf8299e88c128a62c222661f36b7bcc6d98a598c
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8
- 00_nucleo/prompts/_nuclei/paragraph-break-policy.toml sha256:7e41c96527dd5936bebe6916f2a9ea93477c057495845c7e8b52ce3c5917855a
- 00_nucleo/prompts/_nuclei/paragraph-composition.toml sha256:6eec17984449d21b626379651b9b0a511a2dfe8e1a1f0a0af230cce52a47b827
- 00_nucleo/prompts/_nuclei/paragraph-selection.toml sha256:a28cd4f2dd22d2bdb2cb30085596c2a758af7770fa21eb28c38dc35e904928eb
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/word_layout.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidência anterior à decisão

A inspeção de `01_core/src/compiler/layout/text_measure.rs:85`, `word_layout.rs:89` e `space.rs:162` encontrou respectivamente fronteira textual, admissão de baseline e teste de integração de espaços iniciais em owners de outras responsabilidades. O estado exato, fontes e prompts anteriores estão em `00_nucleo/diagnosticos/separacao-tracking-2026-09-29/initial.json` e `before/`. A correção já medida encontra-se em `00_nucleo/diagnosticos/correcao-tracking-2026-09-29.md`; esta separação conserva seus resultados e limites. Posições, texto e estilos são observáveis da língua; localização das funções é mecânica. A inspeção não demonstra intenção histórica nem nova paridade bilateral.

## Contrato vigente

Possuir word, chunk, prepared e o produtor privado push_text com highlight. Word/chunk pedem admissão a text_baseline::admit; word preserva delegação ICU e ambos medem uma vez por text_measure::word. Resolver hyphenate lexicalmente: Bool explícito prevalece; auto/ausência usam par.justify Bool, default false. Não modificar chain ou persistir política no estilo.

Consultar State::boundary do owner text_tracking e projetar cursor_x + intervalo para break_selection. Keep aplica o intervalo uma vez antes de emitir. Wrap zera o intervalo, fecha com flush_continued_line, admite baseline e emite com a largura previamente medida. Hyphenate aplica intervalo, emite prefixo com substituição causal, avança sua largura, fecha continuação e recorre para o restante. Chunk não hifeniza e conserva seleção por largura. Não inserir espaço automático entre fragmentos.

push_text conserva estilo, baseline/offset, ordem da tinta e highlight: extent resolvido no tamanho atual, radius positivo determina rounded rect, demais casos rect, ascender e altura vêm das métricas injetadas. Substituição opcional envolve exclusivamente a folha Text em TextReplacement, deixando highlight fora. Texto ordinário/chunk usam None; palavra usa replacement do seletor.

prepared aplica temporariamente o TextStyle fornecido, chama ensure_initial_baseline e push_text com largura e substituição pré-medidas, avança e restaura estilo. Não seleciona quebras ou acrescenta tracking; a certificação global continua recusando tracking. As fórmulas de fronteira e alinhamento de carriers pertencem aos respectivos owners, não a este emissor.

## Aceitação e limites

Preservar os testes causais existentes e o texto, posições, estilos, páginas e diagnósticos dos controles anteriores à separação. Ownership 1:1, pins válidos, build e lint sem erros. Movimentação interna ADR-0109/0127: funções estáticas descendentes, sem estado novo. As limitações de tracking em estilos mistos, RTL, seleção global e contagem de clusters permanecem as registradas no diagnóstico da correção.

## Continuidade lexical do tracking — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/correcao-tracking-escopos-2026-09-29/before-pdf/results.json` e `summary.json` registram 30 fontes/90 PDFs, comandos, fontes e estado exato em initial.json e before-pdf/provenance.json, contra vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8. Escopos iguais repetidos acumulam cerca de 0.4pt e aninhados 0.8pt; wrap positivo muda a linha. `text_tracking.rs:50` compara apenas TextStyle resolvido. Vanilla `inline/collect.rs:287-295` conserva segmentos de cadeias distintas; `inline/shaping.rs:1269-1295` soma tracking apenas dentro de cada run. Isto mede fronteiras textuais observáveis, não exige copiar PartialEq ou representação da chain. Hide/Box também são fronteiras: hide.rs:21-24 e boxed.rs:157-208 substituem/restauram buffers sem preservar essa distinção.

### Contrato interno da correção

Usar boundary do State no cursor projetado de word/chunk. Registrar produtor após push_text, inclusive prepared, para associar texto realmente emitido ao escopo atual. Keep/wrap/hifenização, widths pré-medidas, SHY, highlight e estilo permanecem; fim/início de linha continua regido pelo buffer real.

Correção interna de paridade ADR-0127 em fluxo contínuo. Aceitação RED→GREEN, controles bilaterais de texto/posições/linhas e preservação dos controles de tracking uniforme; build e lint sem erros, resselo somente destes owners. Demais limites abertos mantidos.

## Fronteiras entre unidades em RTL explícito — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/correcao-tracking-rtl-2026-09-29/before-pdf/results.json`, `summary.json`, `provenance.json` e `initial.json` registram 66 entradas e 198 PDFs contra o vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, com fontes, comandos, binários, HEAD e working tree completos. Em `dir:rtl`, três termos latinos deixam de receber quatro intervalos: com ±0.2pt o primeiro termo difere 0.8pt; 0pt coincide. Texto árabe explícito difere 0.4pt nos dois intervalos entre palavras para ±0.2pt. `text_tracking.rs:65` recusa indiscriminadamente RTL; `word_layout.rs:34,72` e `space.rs:25` já projetam/adicionam a fronteira assinada; `cursor.rs:150-186` ancora a linha pelo limite direito real. No vanilla `inline/shaping.rs:1269-1295`, tracking aplica-se antes de cada próximo cluster distinto também em direção negativa. Isso mede posições e escolha de linha, não a intenção histórica.

Hebraico (`dir:rtl`) e espaço literal inicial apresentam diferenças mesmo em 0pt; não são prova de defeito da fronteira. Ligaduras, mistura de direção e paginação global ainda exigem suas próprias sondas.

### Contrato da correção

A fronteira calculada por State::boundary pode ser não nula em RTL explícito. Projetá-la no cursor usado por break_selection; Keep/Hyphenate aplicam uma vez, Wrap zera antes de emitir na linha seguinte. Conservar largura nominal, hifenização, SC, prepared (tracking recusado) e identidade textual. Não espelhar coordenadas ou deslocar manualmente glyphs: owner cursor ancora a linha pelo limite direito.

Correção interna de paridade ADR-0127 em fluxo contínuo: sem contrato público, default, quebra de compatibilidade ou fase nova. A restrição histórica a LTR fica substituída somente por esta fatia medida. Aceitação RED→GREEN, controles bilaterais em ambas as direções/valores, preservação LTR, build e lint sem erros, resselo restrito.

## Migração do carrier bidi ratificado (2026-10-05)

Medição anterior à decisão: o inventário `00_nucleo/diagnosticos/bidi-contexto-2026-10-05/inventory.json` identifica as referências desta unidade e os owners 1:1. O contrato do manifesto 1ae7513e foi confirmado; nesta unidade ajustar constructors não certificados com None e patterns de leitura com `..`, preservando os observáveis de linguagem existentes. Transformações obedecem ao núcleo pinado de origem; sem ampliar fase/API.

## Transporte lógico textual — contrato ratificado em 2026-10-05

Medição anterior à decisão: `00_nucleo/diagnosticos/bidi-contexto-2026-10-05/probe.json.gz` refuta resolver 456 789 isoladamente; a medição de fonte e os locais afetados estão no inventário e initial.json.gz daquele diagnóstico. A API imutável e os três campos opcionais foram ratificados pelo humano (manifesto 1ae7513e). Correção interna de paridade sem mudar fase.

Esta unidade participa do transporte por Cursor privado, input Arc e spans exatos. Preparar antes de emitir o parágrafo textual inteiro, base lexical já certificada; conservar offsets por ordem de visita, não matching de strings. Sequence/Par criam escopo, nested Sequence herda; checkpoint conserva cursor e span corrente; subframe isola e restaura. Text fornece subspans para palavras e espaços literais; Space consome seu segmento mesmo quando fraco; Linebreak consome fronteira. Word emite span somente se texto exato não foi transformado.

Escopo inicial: incremental simple, fora de células/colunas/subframes, sem tracking/hifenização/justificação/smallcaps, Text sem newline/tab/segmentação linguística. Região recusada tem None. Preparação global anterior permanece para parágrafos que não precisam do contexto bidi; para o subconjunto certificado com base RTL ou caracteres RTL/override oposto, encaminhar pelo incremental sem mudar os breaks permitidos. Não interferir em optimized explicitamente escolhido. O owner privado bidi_origin prepara/valida; esta unidade só coordena o seu papel. Testes de origem por ocorrências, UTF8, restauração e parbreak; validação bilateral de wraps.


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

O retângulo de highlight já Color-only emite FramePaint::Solid. Conservar todo o protocolo de texto, estilo, tracking e posição.

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

## Origem local da linha — correção medida

### Evidência anterior à decisão

`00_nucleo/diagnosticos/transformacoes-2026-10-14/margin-before/results.json.gz`
registra estado exato, comandos, binários e imagens bilaterais. plain-box e
first-overlong com margem de 70pt mantêm palavras fora da caixa no Tekt; no pin
as palavras seguintes quebram. As imagens foram inspecionadas. Em
`word_layout.rs:44,81` o limiar é page_config.margin.left, mas sub_frame.rs:196–217
usa origem local zero. break_selection.rs:25 permite overflow enquanto o cursor
não excede esse limiar. Isso explica o RED das regressões em
implementation/reflow-green-first.log, não é motivo para relaxar os testes.
A antiga cláusula de preservar a margem foi uma restrição de atomização; agora
está refutada para regiões locais. Geometria e quebras são observáveis da língua.

### Contrato substitutivo

O limiar de progresso da primeira palavra é a origem real da linha no mesmo
referencial do cursor e da borda direita. O predicado continua estrito:
cursor + largura > borda direita e cursor > origem. Assim uma primeira palavra
maior que a caixa progride sem linha vazia, mas não bloqueia quebras posteriores.
Nenhum novo algoritmo de hifenização, API pública ou fase do pipeline. Preservar
tracking e os controles de palavra longa, exact-fit, chunks e região ilimitada.

Word/chunk passam regions.current.line_start_x ao seletor puro, não a margem física.

## A2 — prefixo admitido e região inline de Block (2026-10-18)

### Medição anterior à decisão — admissão e continuação

pagination-block.json.gz e block-freeze.json.gz em
00_nucleo/diagnosticos/bibliography-structure-2026-10-18 preservam estado completo,
UTC/HEAD/fontes/binários/argv e caixas do candidato Block/Place c1c222ae54ab3b67487471326012a7f6c4ed86c1e6bd61b1830ec6a987c2ccea.
Em IEEE/200×90pt ele recupera duas páginas, mas a continuação começa em x8
contra29.924 no oracle bd5ebec285505889e8ff1d4f68804ab147a699f3 (binário
9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9).
region_progress.rs:75–76 reinstala a margem física ao transitar, descartando o
inset ativo. O teste corrigido para70pt também mostrou prefixo na primeira
página e corpo admitido na segunda: paragraph_pagination.rs:87–105 pode adiar
o corpo para respeitar widow/orphan depois que o Place já foi emitido.

A exigência inicial de FIRSTWORD sempre na primeira página da fixture FixedMetrics
não é o contrato correto: group_size em paragraph_pagination.rs:69 pode manter
três linhas juntas. Substituir esse critério pelo prefixo acompanhar a página
real da primeira linha; preservar o RED PDF90/duas páginas e a distinção entre
coluna estável e estratégia de quebra. Não forçar greedy nem desabilitar
widow/orphan para satisfazer um assert. Posições geométricas, conteúdo e agrupamento
são observáveis da língua; slots/representação são mecânica privada.

Correção interna ADR-0127 dentro do layout, sem assinatura/campo/trait público,
flag/default novo, compatibilidade ou execução eval/show. A primeira tinta do
prefixo continua antes da primeira tinta do corpo, após a admissão física do
parágrafo. Não é nova ordem de realização de show nem transferência de fase.
A hipótese é refutável por prefixo órfão, recuo perdido, emissão dupla, efeitos
novos, estratégia alterada ou regressão dos controles de Block/columns/subframe.

### Responsabilidade exclusiva

No produtor privado push_text, depois da admissão da baseline feita por word/chunk/prepared e antes de highlight e folha, pedir a emissão do prefixo pendente ao owner bibliography. Ausência é no-op; prefixo é tomado antes de emissão recursiva. Não mover a chamada para ensure_initial_baseline/paragraph_pagination, que também preparam geometria antes da decisão de página.

Aceitação: RED→GREEN de prefixo/corpo na mesma página e inset nas continuações, inclusive saída para OUTSIDE, simple/optimized e controles existentes. Medir PDF90/150/250, RTL, prefix-only/formatado, cores e regressões; build, suites/doctests/Clippy/Rustdoc/fmt/linhagem/reversa/diferencial Tekt. MemoryMax12GiB/High8GiB/swap0/jobs1; sem publicação. Hanging genérico/overrides e múltiplas bibliografias continuam frentes abertas.

## Recuo francês — aplicação física (2026-10-18)

### Medição anterior à decisão

00_nucleo/diagnosticos/bibliography-hanging-2026-10-18/before-sources.json.gz
preserva HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, árvore não commitada,
status/diffstat/UTC e2099 fontes/L0s. before.json.gz contém92 pares;
extra-before.json.gz contém26; boundary-before.json.gz contém10 e
repeat-before.json.gz contém4. Oracle bd5ebec285505889e8ff1d4f68804ab147a699f3,
binário9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9;
Tekt inicial2cc69a555e9738dc659c06ed6d933853ab981cbb73e93c388a5b43c55afad095.
Em160pt/margem8/font12, hanging10 mantém a primeira linha x8 mas move
continuações x8→18; negativo-5 exige x3. RTL mantém primeira borda direita152
mas continuações vão para142. O Tekt ignora o comprimento. MLA tem default
18pt (1.5em), set ambiente3em não o troca, show-set3em exige36pt; IEEE/Plain
não adquirem recuo de hanging por esse show-set. bibliography.where(style:"mla")
não casa na sonda vanilla: conservar a decisão do seletor, não forçar override.

Block/Box com body apenas inline ignoram hanging10; Par explícito dentro
deles e Block com Parbreak aplicam. Heading/Raw/List/Quote simples não o
aplicam. Declaração parcial, zero, repetida e escopo vazio fecham fronteiras:
FIRST/MIDDLE/LAST em5.168/27.464/49.76, contra todos5.168 no Tekt. A primeira
sonda lexical usou colchetes literais: foi refutada, corrigida para #[...] e
preservada como extra-before-initial; não é evidência de escopo Styled.
Valores extremos geram tinta recortada pelo extrator: não equiparar ausência
em pdftotext a apagamento de Content. Fonte parcial24pt não muda o em do
comprimento compartilhado12pt, como partial-font registra.

Fontes pinadas: typst-realize/src/lib.rs:1061–1085,1190–1219 distinguem
fragmento/Par e interrupção; typst-layout/src/flow/collect.rs:72–134 escolhe
fluxo, inline/mod.rs:174,216–221 e collect.rs:141–147 resolvem forma;
inline/line.rs:496–503 posiciona; rules.rs:510–523 faz inset/compensação CSL.
typst-library/src/model/bibliography.rs:112–127,367 determina o default e
precedência. Isso mede língua/geometria; não exige a estrutura Rust vanilla
nem atribui intenção histórica. Refutação por fronteira perdida, em de run,
recuo duplo, reset em página, fallback greedy imposto, cópia perdida, callbacks
reexecutados ou regressões dos controles impede fechamento.

### Classificação e limites

Correção interna/de paridade ADR-0127, fluxo contínuo: sem campo ou assinatura
pública, flag/default externo novo, fase ou ordem de realização diferente.
A nova forma inicial/continuação supersede somente a limitação correspondente
de largura constante; RTL latino certificado segue as condições do núcleo,
sem BidiCursor ativo/tracking e sem inventar BidiSpan. Os demais limites
anteriores continuam. Os valores/cópia fora da página requerem inspeção dos
frames ou SVG, não igualdade de extração recortada. Não antecipar R3-T1.

### Responsabilidade exclusiva deste owner

Word/chunk usam limites inicial/final efetivos do novo owner em should_wrap/decisão, preservando progresso em origem recuada, inclusive negativa/extrema. prepared recebe admissão horizontal pela mesma autoridade após baseline. Não alterar widths dos glyphs, carriers ou políticas.

### Aceitação

RED→GREEN antes da implementação; controles fonte→layout e bilaterais dos sinais,
fronteiras, variantes e restauração pertinentes. Não deduzir cobertura de todas
as alternativas por executar um arm agrupado: classificações nominais fechadas
do contexto inline/fluxo são intencionais e exigem testes por classe/variante.
Resselo apenas após V15/V26 válidos; testes, build, Clippy/Rustdoc/fmt, reversa
e diferencial Tekt do pin1279707, sob12GiB/High8GiB/swap0/jobs1. Sem publicação.

## Faces mistas, fronteiras e regiões — refinamento medido 2026-10-18

### Medição antes da decisão

Os registros first-after/first-bib-after/first-regions-after e
candidate1-sources em mixed-style-metrics-2026-10-18 preservam o candidato
com apenas line_commit alterado e os sete testes verdes. audit-candidate1
mede y SVG igual no título dos seis casos bibliográficos e na mistura
itálico→romano, mas forced ainda difere0.156. linebreak.rs:46–58 acrescenta
ExplicitLinebreakBoundary com Text vazio de estilo exterior; esse filho é
causal para bidi, não uma face material.

Os20 controles regionais PDF preservam os mesmos binários/pin e fontes do
registro before-sources. Na altura39.44pt, o Tekt põe ITALIC na página2;
o vanilla o conserva na página1. Em55.14pt forced usa2 páginas vs1. Em70pt
wrap distribui3+1 linhas vs2+2. O primeiro candidato corrige y das linhas
wrapped, mas não essas decisões regionais. A tentativa SVG sem template
de páginas foi preservada em regions-before: o oracle recusa saída múltipla
sem padrão; não é prova SVG de paridade. Para regiões, usar os PDFs válidos,
e o script regional futuro solicita somente PDF.

region_progress.rs:83–88 instala baseline com uma face mas não atualiza o
ascent assumido. :143–152 admite só pela previsão, antes do texto real.
paragraph_pagination.rs:43–59 recusa arestas diferentes, desligando a
proteção de custos positivos, embora a seleção de ranges já seja conhecida.
Fonte pinada flow/collect.rs:196–239 protege primeiros/últimos frames com
suas alturas reais; distribute.rs:343–364 avança o grupo apenas quando cabe
na próxima região. A geometria/palavras/páginas são linguagem; formato do
Profile e IDs SVG são mecânica. Refutar por salto precoce, recontagem,
callbacks novos, métrica por Group, alteração de largura/seleção/leading
ou regressão de colunas/notas/float/NoTypography.

### Classificação

Refinamento privado de paridade ADR0127 contínuo: sem contrato público,
flag/default do produto ou fase/ordem de callbacks nova. Preservar caminhos
recusados/escalares anteriores. A medição amplia apenas a certificação
regional de texto preparado com mesmo size/leading, baseline_offset zero,
arestas finitas não negativas e faces com tops/descents diferentes. Tamanhos
ou leading variáveis continuam fora dessa certificação, sem alegar paridade
completa nesses casos.

### Responsabilidade exclusiva deste owner
prepared pede ensure_text_baseline com a mesma face efetiva já instalada temporariamente, preservando restauração, origem, cópia e shaping. Os caminhos word/chunk continuam delegando à fachada text_baseline; não criar decisão de fitting no emissor.

Aceitação: RED→GREEN das quatro regressões adicionais e dos controles existentes, matriz30+20 regional PDF, corpus132+94+21, suíte ampla/Clippy/Rustdoc/linhagem e12GiB/High8GiB/swap0/jobs1. Registrar as tentativas sem ocultar falhas.
