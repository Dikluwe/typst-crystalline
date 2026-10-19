# L0 — text_baseline
Hash do Código: 1992d265

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:7b8a61eaafc8a07df12218f8f0560c33fae5c36f621b822b7350ccefc9774711
- 00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml sha256:3a97c056eff81b344f46d83567274225e3519ae141fce4817e342d573a92fb41
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/text_baseline.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidência anterior à decisão

A inspeção de `01_core/src/compiler/layout/text_measure.rs:85`, `word_layout.rs:89` e `space.rs:162` encontrou respectivamente fronteira textual, admissão de baseline e teste de integração de espaços iniciais em owners de outras responsabilidades. O estado exato, fontes e prompts anteriores estão em `lab/results/construction/diagnostics/separacao-tracking-2026-09-29/initial.json` e `before/`. A correção já medida encontra-se em `lab/results/construction/diagnostics/correcao-tracking-2026-09-29.md`; esta separação conserva seus resultados e limites. Posições, texto e estilos são observáveis da língua; localização das funções é mecânica. A inspeção não demonstra intenção histórica nem nova paridade bilateral.

## Contrato vigente

Possuir admit(layouter), privado ao pai. Antes da admissão, registrar se a linha não está vazia e não contém material segundo html_frame::items_have_material. Chamar ensure_initial_baseline uma única vez. Somente se havia carriers sem material, alinhar y de cada TextSeparator ao cursor_y admitido mais baseline_offset resolvido com o tamanho do próprio estilo. Conservar x, advance, estilo, origem, ordem e demais itens.

Word/chunk chamam admit antes de emitir e novamente após wrap. O produtor Space não cria demanda vertical por espaços isolados. prepared conserva a admissão existente sem introduzir este alinhamento ou tracking. Não possuir as fórmulas de admissão regional, fechamento de linha ou medição de texto; delegá-las aos adapters existentes. O teste de espaços iniciais acompanha este owner e continua exercitando Space e Word reais.

## Aceitação e limites

Preservar os testes causais existentes e o texto, posições, estilos, páginas e diagnósticos dos controles anteriores à separação. Ownership 1:1, pins válidos, build e lint sem erros. Movimentação interna ADR-0109/0127: funções estáticas descendentes, sem estado novo. As limitações de tracking em estilos mistos, RTL, seleção global e contagem de clusters permanecem as registradas no diagnóstico da correção.

## Fecho de linha literal — medição e contrato (2026-09-29)

`lab/results/construction/diagnostics/correcao-auto-proximas-2026-09-29/after/results.json` mostra dois parágrafos de espaços a 12pt em 30.206pt Tekt versus 31.536pt vanilla; a sonda por tamanhos 10–18pt em `size-probe.log` mostra crescimento de 1.914pt/pt no Tekt contra 2.628pt/pt vanilla. `text_baseline.rs:14-16` admite pelo `layouter.style`, que volta ao tamanho exterior ao sair de `text.rs`, mesmo quando cada TextSeparator Literal guarda o estilo efetivo. Criar helper privado que admite a baseline de uma linha somente Literal usando o estilo do carrier de maior tamanho, restaurando o estilo exterior depois; reutilizar `admit` e não criar estado/global. O conteúdo misto continua no caminho de Word/Chunk.

## Recuo francês — aplicação física (2026-10-18)

### Medição anterior à decisão

lab/results/construction/diagnostics/bibliography-hanging-2026-10-18/before-sources.json.gz
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

Depois da admissão vertical, aplicar a origem horizontal pendente uma vez; alinhar separadores literais anteriores pelo mesmo delta, sem criar demanda vertical por eles. Nova região reabre aplicação sem reiniciar a linha lógica.

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
admit continua responsável apenas por alinhar carriers literais e pedir baseline. Usar a fachada privada ensure_text_baseline, pois já se trata de admissão textual com estilo efetivo. Não medir aqui, não alterar o ponto de admissão/aplicação do recuo e não modificar o caminho Literal-only.

Aceitação: RED→GREEN das quatro regressões adicionais e dos controles existentes, matriz30+20 regional PDF, corpus132+94+21, suíte ampla/Clippy/Rustdoc/linhagem e12GiB/High8GiB/swap0/jobs1. Registrar as tentativas sem ocultar falhas.
