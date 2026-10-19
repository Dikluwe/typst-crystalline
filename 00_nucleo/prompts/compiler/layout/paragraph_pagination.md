# L0 — paragraph_pagination
Hash do Código: 5845032f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/installed-line-ascent.toml sha256:42175932c75f37ed19205d1d82526969999e18fba8fed16f1b1dcff6e418bc0f
- 00_nucleo/prompts/_nuclei/paragraph-costs.toml sha256:70b72ac9efb2bf81aa4ef496c4e1c867264f696d454a9916e7e2e5f385b92020

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/paragraph_pagination.rs`

## Proteção regional de viúvas/órfãs — 2026-09-28

### Medição anterior à decisão

`lab/results/construction/diagnostics/correcao-viuvas-orfas-2026-09-28/before-pdf/` registra 78 entradas e `before-extra/` registra 24 controles, com estado completo em initial.json e before/, contra vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8. page-65-100%-0%-False deixa três linhas e uma no Tekt, mas duas e duas no vanilla; custos zero coincidem. Tiny/large-positive têm a mesma proteção vanilla. `lab/typst-original/crates/typst-layout/src/flow/collect.rs:196-229` mede habilitação estritamente positiva, primeiros dois/últimos dois e proteção integral de três linhas; `flow/distribute.rs:343-364` só avança por necessidade agrupada quando cabe na próxima região. paragraph_emit.rs emite linha por linha sem essa necessidade. A semântica observável é a distribuição de linhas/palavras em regiões, não os frames/Child do vanilla. Não inferir intenção do scope-out antigo.

### Decisão vigente

Correção interna/de paridade em fluxo contínuo ADR-0127: nenhum campo/trait/assinatura pública, flag ou fase nova. Supersede a exclusão anterior de widow/orphan exclusivamente para parágrafos preparados com métricas verticais uniformes, sem baseline offset, geometria paginada constante, sem colunas/células/subframes/notas/floats ou transação regional ativa. Os demais contextos conservam fallback e limites explícitos. Não promete algoritmo completo nem desempenho.

O módulo privado paragraph_pagination possui exclusivamente certificação vertical, agrupamento e admissão regional. Recebe ranges já escolhidos, não refaz seleção, não emite glyphs nem duplica a lógica de new_page. A certificação inteira precede emissão; todos os fragmentos devem ter size/arestas/leading iguais ao estilo exterior (negrito/itálico admissíveis se as métricas coincidirem), e finite nonnegative top/descent/leading. Bounds auto não precisam de proteção. Respeitar a baseline inicial e o colapso existente de block-chain antes de comparar necessidade real com page_bottom_limit. Novo espaço disponível é calculado com margens resolvidas para a página seguinte. Avançar só se há material na região atual e o grupo cabe na próxima. Grupos muito altos conservam progresso ordinário e conteúdo íntegro; nunca repetir new_page em laço.

Helpers privados de text_costs resolvem widow/orphan independentemente dos pesos hyphenation/runt, reutilizando FIELDS, com default Ratio 1 e teste estrito > 0. O default de estratégia do planejamento passa a Simple para parágrafo comum sem linebreaks, permitindo a preparação necessária à proteção padrão; a escolha automática por justify e todos os contextos recusados permanecem. Emitir sem alterar justificação, carriers, última linha aberta, fronteiras e restauração lexical. Preservar notas e colunas por não aplicar proteção incompleta nesses caminhos.

Aceitação: RED→GREEN com mesmas palavras distribuídas 2+2 versus 3+1, proteção integral de três, orphan desligado independentemente, grupo excessivo, restauração e default sem flag. Sondas bilaterais com caixas/palavras/páginas, tipos/spans anteriores preservados e controles de fallback. Não usar grupos XML como número físico de linhas. Resselo restrito aos owners, build e crystalline-lint sem erros.

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
Superseder somente a restrição de arestas uniformes para o escopo certificado descrito. Profile privado empresta Prepared+ranges selecionados ou a fatia de Prepared de linhas hifenizadas; não clonar conteúdo nem alocar vetor de perfis. Certificar o conjunto inteiro antes de emitir: bounds e exclusões anteriores, sizes/leading iguais, offsets zero, arestas válidas. Ler estilos dos fragmentos/gaps Literal exatamente emitidos pelo range; preservar leading uniforme e os custos/guards existentes. Calcular top máximo e descent máximo por linha e necessidade do grupo como descent atual + somas de leading+top+descent seguintes, em janela de no máximo três linhas. Admitir a baseline atual via ensure_line_baseline(top); comparar baseline reconciliada (Some(0) preservado) com o bottom, e próxima baseline=margin.top+top da primeira linha do grupo. Avançar só com material atual e grupo cabendo na próxima, sem laços. Não executar callbacks/objetos futuros, refazer breaks ou relaxar recusas de colunas/notas/float. Exigir teste que distribui palavras6+5 (2+2 linhas) vs9+2 (3+1) com faces diferentes; custos zero preservam3+1. Nada de dyn/vtable ou métrica por aparência.

Aceitação: RED→GREEN das quatro regressões adicionais e dos controles existentes, matriz30+20 regional PDF, corpus132+94+21, suíte ampla/Clippy/Rustdoc/linhagem e12GiB/High8GiB/swap0/jobs1. Registrar as tentativas sem ocultar falhas.

### Top certificado na entrada de bloco

Inspeção do candidato regional: cursor.rs ainda ignora KnownTop nos ramos
initial_baseline_pending/block_chain_active. Assim, o certificador pode pedir
top6 mas o ramo de bloco testar gap+top7 e avançar antes de comparar o
grupo real. A evidência é a ligação de ensure_line_baseline em
paragraph_pagination.rs com os dois cálculos top em cursor.rs; a origem do
top de linha continua o máximo dos frames pinados (inline/line.rs:559–568).
Exigir RED por layout de texto preparado depois de uma fronteira real com
material já emitido: baseline anterior7, below6.5, página20, primeiro topo
itálico6 deve ficar em19.5 na página atual; o topo exterior7 não o expulsa.
Custo widow/orphan zero isola a admissão, sem depender da proteção do grupo.

Refinar a cláusula anterior: preservar os protocolos inicial e de block-chain,
mas quando Admission é KnownTop instalar/testar o top certificado também
nesses dois ramos. Generic e Text mantêm a consulta da face já corrente.
Não trocar fonte/size/leading/margem, executar body para medir ou instalar
altura/tinta. Guardar Some(top) efetivamente instalado (zero incluso).
O owner pagination só fornece a medida; cálculo do protocolo continua em
cursor/region_progress. A correção é privada e na mesma fase de layout.
