# L0 — paragraph_emit
Hash do Código: 5fba1d13

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml sha256:3a97c056eff81b344f46d83567274225e3519ae141fce4817e342d573a92fb41
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8
- 00_nucleo/prompts/_nuclei/paragraph-costs.toml sha256:70b72ac9efb2bf81aa4ef496c4e1c867264f696d454a9916e7e2e5f385b92020
- 00_nucleo/prompts/_nuclei/paragraph-selection.toml sha256:a28cd4f2dd22d2bdb2cb30085596c2a758af7770fa21eb28c38dc35e904928eb


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/paragraph_emit.rs`
**Architecture:** ADR-0107/0108/0109/0127/0129/0130.

### Medição anterior à decisão

O lote bilateral `lab/results/construction/diagnostics/correcao-seletores-paragrafo-2026-09-28/before-pdf/resultados.json` mede 32 entradas, com estado exato em initial.json e before/, contra vanilla 9f2b6e8715237cb086899a42873660fe744622e8. Em 140pt/margem 8pt, simple escolhe delta epsilon zeta eta, optimized escolhe delta epsilon zeta e junta eta à linha seguinte; Tekt ignora a estratégia. Em 240pt justificado, optimized admite eta na primeira linha reduzindo os espaços. Fonte: eval/rules.rs:3070-3100 e stdlib/structural/par.rs:131-150 validam e descartam linebreaks; text.rs:41 e word_layout.rs:29 escolhem incrementalmente; vanilla inline/mod.rs:194-196 resolve auto pela justificação, linebreak.rs:raw_ratio/raw_cost aplica redução até 1/3 e custos cúbicos/quadráticos; model/par.rs:Limits::SPACING_DEFAULT fixa 2/3..1.5. Estilos adjacentes, literal leading/trailing e conteúdo opaco estão medidos separadamente. Observação não prova intenção histórica; inferência refutável: os scope-outs L0 explícitos de preparação/optimized legitimaram o adiamento, pois ainda vedam a implementação global apesar do transporte aceito.

### Decisão e escopo

Correção interna de paridade ADR-0127: sem novos contratos públicos, flags ou fase. Primeira preparação global certificada: parágrafo latino composto por Text, Space, SpaceFromNewline, Empty, Sequence, Styled tipográfico e Strong/Emph, com hifenização desligada, direção LTR, sem matemática/smallcaps; tracking uniforme segue a extensão certificada de 2026-09-29, largura finita e constante, fora de colunas/células/floats. O conjunto completo deve ser certificado antes de emitir; uma unidade recusada preserva o caminho anterior. Hifenização, Unicode UAX14 completo, links/objetos/linebreaks, largura variável, limites customizados e seleção incremental restante continuam explicitamente incompletos; este lote não declara estado da arte nem otimização completa da língua.

Materializar ranges escolhidos sem chamar o seletor incremental novamente. Emitir palavras/fragmentos com estilo efetivo via helper privado de word_layout, com admissão da baseline. Espaços preservam origem e advance ajustado uma vez; redução proporcional às larguras até 1/3 quando linha overfull, expansão nas linhas continuadas justificadas usa stretch até 1/2 proporcional e restante igual por oportunidade. Não justificar a última linha natural; preservar carriers de separação no fim das linhas/final sem incluir seu avanço no custo. Fechar linhas continuadas com flush_line porque ajustes já foram aplicados; conservar última linha aberta para fechamento causal do owner externo. Restaurar estilo externo, document_language tem primeiro estilo real. Sem pipeline eval, objetos, localização ou mutação de Content.

## Aceitação

RED→GREEN: estratégias distinguíveis sob a mesma entrada; auto e overrides lexicais; caixas bilaterais com tolerância 0.0005pt e linhas físicas agrupadas por baseline, não contagem XML. Controles de texto íntegro, estilos fortes/emph, literal leading/trailing, adjacência, overlong, restauração e fallback. Build e lint sem violations.

## Hifenização global — 2026-09-28

### Medição anterior à decisão

O lote before-pdf de `lab/results/construction/diagnostics/correcao-hifenizacao-global-2026-09-28/` mede 67 entradas contra vanilla 9f2b6e8715237cb086899a42873660fe744622e8; estado inicial, binários, fontes e comandos estão em initial.json, before/ e proveniencia.json. paragraph_prepare.rs:80-94 recusa a política true, paragraph_select.rs:66 considera só finais de palavras, paragraph_plan.rs:18 escolhe estratégias já transportadas, break_selection.rs:41-66 hifeniza apenas por overflow local. Vanilla inline/linebreak.rs:170-209 usa os mesmos candidatos no greedy, :626-684 aplica 135*(1+0.15*(max(5-left,0)+max(5-right,0))) por hífen, mais 135 em duas linhas consecutivas; inline/shaping.rs:583-635 desenha o glyph de '-' mas conserva SHY no texto copiado. Isto mede comportamento e contratos, não intenção histórica.

### Decisão

Correção interna ADR-0127 em fluxo contínuo: sem campo/trait/assinatura pública, modo novo, fase ou quebra de compatibilidade. A restrição histórica de hifenização desligada é supersedida nesta rodada para palavras latinas de estilo/política uniformes, inclusive pontuação terminal simples. Mistura dentro da palavra, apostrofes internas, outras quebras UAX14, largura variável e objetos mantêm fallback. Texto copiado '-' versus SHY continua limitação de export independente; a geometria usa o glyph de '-' separado, como fonte medida. Aceitação mede palavras/posições/estilos e política lexical, normaliza somente o hífen de quebra para a comparação geométrica e declara a divergência textual separadamente.

Extrair emissão de uma linha preparada para uso compartilhado. Entrada das linhas partidas tem ranges medidos e hífen como fragmento separado sem gerar Space/TextSeparator no interior da palavra. Manter ajuste/continuação/última linha/restauração. Coordenador pode emitir Vec<Prepared> escolhido pelo novo owner, sem novas quebras locais.

### Medição e correção da margem óptica

after-pdf inicial mede quebras coincidentes, mas linhas justificadas com hífen ficam 2.1252pt aquém do vanilla; en-140-optimized-True põe ex- em xMin115.26/xMax132 contra117.385197/134.1252. Fonte: lab/typst-original/crates/typst-layout/src/inline/line.rs:518-526 acrescenta overhang somente ao remaining da emissão; :708-709 fixa 0.55 para glyph '-' ou SHY; text/mod.rs:391-393 default true. Não alterar custo/seleção. Prepared guarda right_overhang privado, default zero; linhas materializadas terminadas em hífen recebem dash_width*0.55; emissor usa width+overhang para ajustar os espaços. Overhang textual configurável e de outras pontuações permanece escopo pendente, com controle separado.

## Proteção regional de viúvas/órfãs — 2026-09-28

### Medição anterior à decisão

`lab/results/construction/diagnostics/correcao-viuvas-orfas-2026-09-28/before-pdf/` registra 78 entradas e `before-extra/` registra 24 controles, com estado completo em initial.json e before/, contra vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8. page-65-100%-0%-False deixa três linhas e uma no Tekt, mas duas e duas no vanilla; custos zero coincidem. Tiny/large-positive têm a mesma proteção vanilla. `lab/typst-original/crates/typst-layout/src/flow/collect.rs:196-229` mede habilitação estritamente positiva, primeiros dois/últimos dois e proteção integral de três linhas; `flow/distribute.rs:343-364` só avança por necessidade agrupada quando cabe na próxima região. paragraph_emit.rs emite linha por linha sem essa necessidade. A semântica observável é a distribuição de linhas/palavras em regiões, não os frames/Child do vanilla. Não inferir intenção do scope-out antigo.

### Decisão vigente

Correção interna/de paridade em fluxo contínuo ADR-0127: nenhum campo/trait/assinatura pública, flag ou fase nova. Supersede a exclusão anterior de widow/orphan exclusivamente para parágrafos preparados com métricas verticais uniformes, sem baseline offset, geometria paginada constante, sem colunas/células/subframes/notas/floats ou transação regional ativa. Os demais contextos conservam fallback e limites explícitos. Não promete algoritmo completo nem desempenho.

O módulo privado paragraph_pagination possui exclusivamente certificação vertical, agrupamento e admissão regional. Recebe ranges já escolhidos, não refaz seleção, não emite glyphs nem duplica a lógica de new_page. A certificação inteira precede emissão; todos os fragmentos devem ter size/arestas/leading iguais ao estilo exterior (negrito/itálico admissíveis se as métricas coincidirem), e finite nonnegative top/descent/leading. Bounds auto não precisam de proteção. Respeitar a baseline inicial e o colapso existente de block-chain antes de comparar necessidade real com page_bottom_limit. Novo espaço disponível é calculado com margens resolvidas para a página seguinte. Avançar só se há material na região atual e o grupo cabe na próxima. Grupos muito altos conservam progresso ordinário e conteúdo íntegro; nunca repetir new_page em laço.

Helpers privados de text_costs resolvem widow/orphan independentemente dos pesos hyphenation/runt, reutilizando FIELDS, com default Ratio 1 e teste estrito > 0. O default de estratégia do planejamento passa a Simple para parágrafo comum sem linebreaks, permitindo a preparação necessária à proteção padrão; a escolha automática por justify e todos os contextos recusados permanecem. Emitir sem alterar justificação, carriers, última linha aberta, fronteiras e restauração lexical. Preservar notas e colunas por não aplicar proteção incompleta nesses caminhos.

Aceitação: RED→GREEN com mesmas palavras distribuídas 2+2 versus 3+1, proteção integral de três, orphan desligado independentemente, grupo excessivo, restauração e default sem flag. Sondas bilaterais com caixas/palavras/páginas, tipos/spans anteriores preservados e controles de fallback. Não usar grupos XML como número físico de linhas. Resselo restrito aos owners, build e crystalline-lint sem erros.

## Identidade textual — materialização aprovada

### Medição anterior à decisão

As sondas bilaterais e o estado exato em
`lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/initial.json`
e `before/` medem U+002D versus SHY com geometria coincidente em oito controles.
A inspeção do percurso FrameItem deste consumer está arquivada em approved-before/.
O dono confirmou o contrato em `00_nucleo/prompts/entities/layout_types.md`.

### Responsabilidade exclusiva

Encaminhar replacement do Fragment ao adapter word_layout::prepared, sem escolher quebras ou modificar geometria/estilo.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.

## Emissão de tracking uniforme certificado — 2026-09-29

Medição anterior à decisão: `paragrafo-tracking-global-2026-09-29/focused-before/results.json` mostra páginas e caixas divergentes com tracking explícito; `no-tracking-baseline.json` coincide sem essa propriedade. `paragraph_emit.rs:5-75` emite gaps e palavras sem aplicar intervalos entre unidades; o percurso incremental os aplica em `space.rs:22-32` e `word_layout.rs:29-58`. A seleção ampliada só pode ser aceita se a emissão usar os mesmos intervalos fixos, sem duplicá-los no PDF.

Antes de cada gap materializado com antecessor na mesma linha, e antes da palavra após gaps incluídos na linha, avançar exatamente um tracking assinado. O primeiro gap literal inicial não tem antecessor; gap anterior a uma palavra que abre nova linha pertence à linha precedente e não acrescenta intervalo antes da nova palavra. Ajustes de justificação alteram apenas a largura base dos espaços, nunca os intervalos de tracking. Conservar origem, texto, glyphs e carriers; medir e emitir a mesma extensão. Não mudar o helper público, fase do pipeline nem comportamento de conteúdo recusado.

## Migração do carrier bidi ratificado (2026-10-05)

Medição anterior à decisão: o inventário `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json` identifica as referências desta unidade e os owners 1:1. O contrato do manifesto 1ae7513e foi confirmado; nesta unidade ajustar constructors não certificados com None e patterns de leitura com `..`, preservando os observáveis de linguagem existentes. Transformações obedecem ao núcleo pinado de origem; sem ampliar fase/API.

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

Preparar contexto da linha antes de admitir/emiti-la; ajuste usa a largura efetiva inicial/continuada da mesma forma, mantendo carrier, última linha natural e admissão regional. Continuar sem selecionar de novo ou executar eval. Estado de linha do novo owner recebe commit pelo LineCommit.

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
Encaminhar ao certificador privado os ranges já selecionados no caminho emit e a fatia já preparada de linhas no caminho emit_prepared_lines. Reutilizar o Profile emprestado na admissão de cada linha. Não duplicar geometria/group_size, não refazer seleção, alterar estilos, carriers, justificação ou executar conteúdo para prever futuras linhas.

Aceitação: RED→GREEN das quatro regressões adicionais e dos controles existentes, matriz30+20 regional PDF, corpus132+94+21, suíte ampla/Clippy/Rustdoc/linhagem e12GiB/High8GiB/swap0/jobs1. Registrar as tentativas sem ocultar falhas.
