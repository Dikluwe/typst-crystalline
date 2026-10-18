# L0 — paragraph_plan
Hash do Código: 8806c514

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml sha256:3a97c056eff81b344f46d83567274225e3519ae141fce4817e342d573a92fb41
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8
- 00_nucleo/prompts/_nuclei/paragraph-costs.toml sha256:70b72ac9efb2bf81aa4ef496c4e1c867264f696d454a9916e7e2e5f385b92020
- 00_nucleo/prompts/_nuclei/paragraph-selection.toml sha256:a28cd4f2dd22d2bdb2cb30085596c2a758af7770fa21eb28c38dc35e904928eb


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/paragraph_plan.rs`
**Architecture:** ADR-0107/0108/0109/0127/0129/0130.

### Medição anterior à decisão

O lote bilateral `00_nucleo/diagnosticos/correcao-seletores-paragrafo-2026-09-28/before-pdf/resultados.json` mede 32 entradas, com estado exato em initial.json e before/, contra vanilla 9f2b6e8715237cb086899a42873660fe744622e8. Em 140pt/margem 8pt, simple escolhe delta epsilon zeta eta, optimized escolhe delta epsilon zeta e junta eta à linha seguinte; Tekt ignora a estratégia. Em 240pt justificado, optimized admite eta na primeira linha reduzindo os espaços. Fonte: eval/rules.rs:3070-3100 e stdlib/structural/par.rs:131-150 validam e descartam linebreaks; text.rs:41 e word_layout.rs:29 escolhem incrementalmente; vanilla inline/mod.rs:194-196 resolve auto pela justificação, linebreak.rs:raw_ratio/raw_cost aplica redução até 1/3 e custos cúbicos/quadráticos; model/par.rs:Limits::SPACING_DEFAULT fixa 2/3..1.5. Estilos adjacentes, literal leading/trailing e conteúdo opaco estão medidos separadamente. Observação não prova intenção histórica; inferência refutável: os scope-outs L0 explícitos de preparação/optimized legitimaram o adiamento, pois ainda vedam a implementação global apesar do transporte aceito.

### Decisão e escopo

Correção interna de paridade ADR-0127: sem novos contratos públicos, flags ou fase. Primeira preparação global certificada: parágrafo latino composto por Text, Space, SpaceFromNewline, Empty, Sequence, Styled tipográfico e Strong/Emph, com hifenização desligada, direção LTR, sem matemática/smallcaps; tracking uniforme segue a extensão certificada de 2026-09-29, largura finita e constante, fora de colunas/células/floats. O conjunto completo deve ser certificado antes de emitir; uma unidade recusada preserva o caminho anterior. Hifenização, Unicode UAX14 completo, links/objetos/linebreaks, largura variável, limites customizados e seleção incremental restante continuam explicitamente incompletos; este lote não declara estado da arte nem otimização completa da língua.

Coordenação privada: resolve estratégia na chain. Só prepara um fluxo que começa sem linha material, com largura finita positiva e constante (não colunas/células/floats/smallcaps). Não muda estado se preparação falha. Delega preparo, seleção e emissão aos owners descendentes. Expor helper de estratégia e try_layout/try_parts privados. O aviso de suporte parcial já existente em set par(justify:true) permanece factual; esta unidade não cria warnings especulativos nem promete suporte ao conjunto recusado. Texto isolado pode tentar; Sequence tenta o conjunto inteiro antes de percorrer siblings e suprime tentativas dos fragmentos ao recusar; Par tenta seu body completo ainda dentro do escopo. O guard privado é restaurado e participa de rollback.

## Aceitação

RED→GREEN: estratégias distinguíveis sob a mesma entrada; auto e overrides lexicais; caixas bilaterais com tolerância 0.0005pt e linhas físicas agrupadas por baseline, não contagem XML. Controles de texto íntegro, estilos fortes/emph, literal leading/trailing, adjacência, overlong, restauração e fallback. Build e lint sem violations.

### Vários parágrafos no mesmo fluxo

Medição anterior à decisão: multi-before/resultados.json conserva a fonte, o binário e proveniência da sonda two-paragraphs; ambos produzem 8 linhas físicas; o Tekt mantém escolhas greedy no primeiro segmento, enquanto o segundo já coincide. Os 10 agrupamentos XML do Tekt não são linhas físicas (contra 8 agrupamentos do vanilla). Ao receber parts, separar somente nos Parbreak explícitos de nível imediato. Certificar todos os segmentos pela mesma preparação, antes de emitir qualquer um; segmentos feitos apenas de Empty/Space/SpaceFromNewline são neutros. Planejar e emitir cada segmento não vazio, invocando parbreak entre eles. Conteúdo opaco em qualquer segmento recusa o caminho global inteiro. Estratégia, font-size e largura ficam constantes sob o escopo exterior; sem tratamento de widow/orphan nesta rodada.

## Hifenização global — 2026-09-28

### Medição anterior à decisão

O lote before-pdf de `00_nucleo/diagnosticos/correcao-hifenizacao-global-2026-09-28/` mede 67 entradas contra vanilla 9f2b6e8715237cb086899a42873660fe744622e8; estado inicial, binários, fontes e comandos estão em initial.json, before/ e proveniencia.json. paragraph_prepare.rs:80-94 recusa a política true, paragraph_select.rs:66 considera só finais de palavras, paragraph_plan.rs:18 escolhe estratégias já transportadas, break_selection.rs:41-66 hifeniza apenas por overflow local. Vanilla inline/linebreak.rs:170-209 usa os mesmos candidatos no greedy, :626-684 aplica 135*(1+0.15*(max(5-left,0)+max(5-right,0))) por hífen, mais 135 em duas linhas consecutivas; inline/shaping.rs:583-635 desenha o glyph de '-' mas conserva SHY no texto copiado. Isto mede comportamento e contratos, não intenção histórica.

### Decisão

Correção interna ADR-0127 em fluxo contínuo: sem campo/trait/assinatura pública, modo novo, fase ou quebra de compatibilidade. A restrição histórica de hifenização desligada é supersedida nesta rodada para palavras latinas de estilo/política uniformes, inclusive pontuação terminal simples. Mistura dentro da palavra, apostrofes internas, outras quebras UAX14, largura variável e objetos mantêm fallback. Texto copiado '-' versus SHY continua limitação de export independente; a geometria usa o glyph de '-' separado, como fonte medida. Aceitação mede palavras/posições/estilos e política lexical, normaliza somente o hífen de quebra para a comparação geométrica e declara a divergência textual separadamente.

Se qualquer Word tem candidatos, selecionar linhas pelo owner paragraph_hyphen e emitir pelo owner paragraph_emit. Sem candidatos, manter o caminho certificado anterior. Todos os segmentos entre Parbreak continuam certificados antes da emissão; não emitir o primeiro se outro falha. Na ausência de linebreaks, auto resolve simple para par.justify false, permitindo hifenização simples também na origem de linha. Contextos excluídos permanecem.

## Custos textuais personalizados — 2026-09-28

### Medição anterior à decisão

`correcao-custos-paragrafo-2026-09-28/before-pdf/` registra 66 entradas no
estado `initial.json` e `before/`, contra pin 9f2b6e8715237cb086899a42873660fe744622e8.
`text/mod.rs:1507-1564` valida Dictionary, Ratio por campo e composição parcial;
`inline/mod.rs:240` lê costs do escopo compartilhado, não de cada fragmento;
`inline/linebreak.rs:29-30,961-962` aplica defaults 135/100 vezes o ratio.
O Tekt descarta costs em eval/rules e aceita sem transportar no constructor;
paragraph_hyphen e paragraph_select usam defaults fixos. Os erros e spans
estão no stderr vanilla, inclusive negativos aceitos. Não inferir intenção.

### Decisão vigente

Correção interna/de paridade ADR-0127 em fluxo contínuo, sem novo contrato
público, flag ou fase. Módulo privado compiler/text_costs possui owner único
para cast, transporte por componente e resolução dos custos compartilhados.
Hifenização/runt globais passam a consultar a configuração. Widow/orphan são validados e transportados; sua influência é limitada ao subconjunto regional certificado descrito abaixo.
Custos de fragmentos parciais não substituem o custo compartilhado; os quatro
canais validados são neutros para glyphs na whitelist Styled. Um wrapper integral
é fatorado pela realização existente antes de formar Par. Largura variável
permanece limitada. Não deduzir custo pelo glyph. Preservar todos os defaults.

Resolver LineCosts privado pela chain compartilhada na entrada do planejamento; passar aos dois seletores certificados. Ranges/emissão não mudam. Todos os segmentos continuam certificados antes de emitir.

### Refinamento de custo compartilhado medido antes de implementar

after-first-resultados.json/proveniencia.json registra 66 entradas com o primeiro
binário de custos. Só restore e inline divergem entre os inputs válidos. A fonte
`inline/mod.rs:240` consulta shared.get(TextElem::costs); `eval/rules.rs` conserva
fatoração dos Styled que cobrem todo o material do fluxo antes de formar Par.
`paragraph_prepare.rs:45-78` recusa os novos canais mesmo quando eles não alteram
nenhum atributo de glyph: o fallback incremental produz escolhas divergentes.
A exclusão absoluta desses canais foi refutada; isto supersede o scope-out
anterior, sem promover fragmentos ao escopo exterior nem mudar fase de pipeline.

Permitir exclusivamente text.costs.hyphenation/runt/widow/orphan na whitelist
Styled da preparação. O custo lido pelo planejamento é o escopo compartilhado
exterior: push temporário da preparação é restaurado e não altera tal leitura.
Inline parcial não escolhe custos do parágrafo; Styles integral continua a ser
fatorado pelo mecanismo existente de realização e entra na chain antes de Par.
Não criar espaço ou fronteira de palavra por diferenças de costs. Manter
limitações físicas widow/orphan e contextos fora do subconjunto certificado.

## Proteção regional de viúvas/órfãs — 2026-09-28

### Medição anterior à decisão

`00_nucleo/diagnosticos/correcao-viuvas-orfas-2026-09-28/before-pdf/` registra 78 entradas e `before-extra/` registra 24 controles, com estado completo em initial.json e before/, contra vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8. page-65-100%-0%-False deixa três linhas e uma no Tekt, mas duas e duas no vanilla; custos zero coincidem. Tiny/large-positive têm a mesma proteção vanilla. `lab/typst-original/crates/typst-layout/src/flow/collect.rs:196-229` mede habilitação estritamente positiva, primeiros dois/últimos dois e proteção integral de três linhas; `flow/distribute.rs:343-364` só avança por necessidade agrupada quando cabe na próxima região. paragraph_emit.rs emite linha por linha sem essa necessidade. A semântica observável é a distribuição de linhas/palavras em regiões, não os frames/Child do vanilla. Não inferir intenção do scope-out antigo.

### Decisão vigente

Correção interna/de paridade em fluxo contínuo ADR-0127: nenhum campo/trait/assinatura pública, flag ou fase nova. Supersede a exclusão anterior de widow/orphan exclusivamente para parágrafos preparados com métricas verticais uniformes, sem baseline offset, geometria paginada constante, sem colunas/células/subframes/notas/floats ou transação regional ativa. Os demais contextos conservam fallback e limites explícitos. Não promete algoritmo completo nem desempenho.

O módulo privado paragraph_pagination possui exclusivamente certificação vertical, agrupamento e admissão regional. Recebe ranges já escolhidos, não refaz seleção, não emite glyphs nem duplica a lógica de new_page. A certificação inteira precede emissão; todos os fragmentos devem ter size/arestas/leading iguais ao estilo exterior (negrito/itálico admissíveis se as métricas coincidirem), e finite nonnegative top/descent/leading. Bounds auto não precisam de proteção. Respeitar a baseline inicial e o colapso existente de block-chain antes de comparar necessidade real com page_bottom_limit. Novo espaço disponível é calculado com margens resolvidas para a página seguinte. Avançar só se há material na região atual e o grupo cabe na próxima. Grupos muito altos conservam progresso ordinário e conteúdo íntegro; nunca repetir new_page em laço.

Helpers privados de text_costs resolvem widow/orphan independentemente dos pesos hyphenation/runt, reutilizando FIELDS, com default Ratio 1 e teste estrito > 0. O default de estratégia do planejamento passa a Simple para parágrafo comum sem linebreaks, permitindo a preparação necessária à proteção padrão; a escolha automática por justify e todos os contextos recusados permanecem. Emitir sem alterar justificação, carriers, última linha aberta, fronteiras e restauração lexical. Preservar notas e colunas por não aplicar proteção incompleta nesses caminhos.

Aceitação: RED→GREEN com mesmas palavras distribuídas 2+2 versus 3+1, proteção integral de três, orphan desligado independentemente, grupo excessivo, restauração e default sem flag. Sondas bilaterais com caixas/palavras/páginas, tipos/spans anteriores preservados e controles de fallback. Não usar grupos XML como número físico de linhas. Resselo restrito aos owners, build e crystalline-lint sem erros.

### Refinamento medido de admissão padrão

core-first.log mede três falhas de invisibilidade (Metadata/State/Func) em tokens adjacentes de uma única linha: a preparação padrão introduz diferença no observador plain_text entre o texto de controle e seu percurso incremental. Os controles existentes verificam a invariância do conteúdo invisível, não justificam promover todos os parágrafos de uma linha. A proteção regional só necessita planejamento quando há pelo menos duas linhas escolhidas. Para a estratégia implícita sem justify/hyphenate true, certificar/selecionar mas conservar o percurso incremental quando há só uma linha em todos os segmentos. Flags explícitas e casos já admitidos continuam iguais. Isto reduz o alargamento ao funcionamento solicitado e não afrouxa as expectativas desses três testes.

## Refinamento de escopo integral e fronteira explícita

### Medição anterior à decisão

first-round/acceptance.json registra 14 falhas nas 397 entradas: negative/native/empty-fold/restore/plain-disabled e regressões lexicais passam a planejar antes do wrapper integral de Styles, lendo custos exteriores e usando font/leading exteriores. Fonte: paragraph_plan.rs admite default Simple antes da descida; paragraph_prepare percorre Styled restaurando a chain; sequence.rs:947-956 reabilita somente o último sibling substantivo. terminal-break conserva 3+1 em vez de 2+2 porque o último sibling é Pagebreak e o texto anterior permanece suprimido. Isto refuta a admissão antecipada; não é autorização para promover um fragmento parcial a escopo compartilhado.

### Decisão

Quando body, ou segmento completo entre Parbreaks, tem exatamente um sibling substantivo Styled (outros são Empty/Space/SpaceFromNewline), recusar a tentativa exterior e deixar style_scope entrar normalmente. Sem alterar chain/Style por antecipação, sem deduzir configuração por glyph e sem emitir metade de um conjunto recusado. Após a entrada, tentar novamente sob o escopo correto pelo dispatch existente. Preservar overrides inline parciais.

No owner Sequence, também reabilitar a tentativa quando o sibling seguinte substantivo é Pagebreak e a linha atual não tem material. Esta é uma fronteira explícita do fluxo: o sibling corrente pode fornecer o parágrafo completo antes da quebra; prefixos já materiais continuam suprimidos. Não segmentar por conteúdo opaco genérico, não alterar preflight de blocos/notas/colunas nem o despacho de pagebreak. Testar restauração/ratios desativados e repetir corpus lexical, com os mesmos inputs e expectativas.

### Reentrada antes de Parbreak medida antes de corrigir

second-round/acceptance.json registra quatro falhas: os dois controles additional-scopes/positive-first (adicionados depois da implementação, sem medição Tekt prévia) e as regressões costs/restore e overhang/restore. Positive-middle já coincide. O primeiro wrapper integral antes de Parbreak continua suprimido porque não é terminal e o guard novo só reconhece Pagebreak; o fluxo posterior entra no escopo normalmente. Parbreak é uma fronteira explícita de parágrafo com a mesma evidência de fechamento necessária. Estender a reabilitação do guard ao próximo sibling substantivo Parbreak, além de Pagebreak, sempre com current_line sem material. Não abrir tentativa antes de sibling inline ou conteúdo opaco genérico. A certificação do parágrafo continua inteira; a iteração apenas restaura possibilidade de tentativa em cada fronteira material de parágrafo. Manter inputs e expectativas dos quatro controles e repetir regressões após rebuild.

## Admissão de tracking uniforme — 2026-09-29

A evidência bilateral em `paragrafo-tracking-global-2026-09-29/focused-before/results.json` e o controle sem tracking em `no-tracking-baseline.json` isolam a recusa da preparação. O planejador conserva a seleção estática Simple/Optimized e só recebe o novo subconjunto depois de certificação integral pelo owner paragraph_prepare. Escopos mistos, hifenização com tracking e demais recusas mantêm o fallback incremental sem emissão parcial. A proteção regional existente continua a operar somente após certificação vertical independente; não há novo modo padrão ou contrato público.

## Tracking explícito zero com hifenização — 2026-09-30

### Medição anterior à decisão

`00_nucleo/diagnosticos/paragrafo-tracking-hifenizacao-2026-09-30/before/results.json` mede divergência em tracking explícito zero + hifenização, enquanto o controle sem propriedade coincide. `after-third/results.json` mede quatro combinações de Simple/Optimized e 100/140 pt com texto, páginas, palavras e caixas dentro de 0,001 pt após admitir o preparo global. Valores não nulos continuam a divergir, e `inline/shaping.rs:546-571` preserva ou remodela glyphs em cortes seguros que FontMetrics não expõe. Não se infere intenção.

### Decisão

Supersede a exclusão conjunta anterior apenas para tracking resolvido exatamente zero sob a certificação uniforme existente. O planejador continua a usar o seletor estático de hifenização sem novo modo ou fase; tracking não nulo e misto conservam fallback integral, sem emissão parcial.

## Transporte lógico textual — contrato ratificado em 2026-10-05

Medição anterior à decisão: `00_nucleo/diagnosticos/bidi-contexto-2026-10-05/probe.json.gz` refuta resolver 456 789 isoladamente; a medição de fonte e os locais afetados estão no inventário e initial.json.gz daquele diagnóstico. A API imutável e os três campos opcionais foram ratificados pelo humano (manifesto 1ae7513e). Correção interna de paridade sem mudar fase.

Esta unidade participa do transporte por Cursor privado, input Arc e spans exatos. Preparar antes de emitir o parágrafo textual inteiro, base lexical já certificada; conservar offsets por ordem de visita, não matching de strings. Sequence/Par criam escopo, nested Sequence herda; checkpoint conserva cursor e span corrente; subframe isola e restaura. Text fornece subspans para palavras e espaços literais; Space consome seu segmento mesmo quando fraco; Linebreak consome fronteira. Word emite span somente se texto exato não foi transformado.

Escopo inicial: incremental simple, fora de células/colunas/subframes, sem tracking/hifenização/justificação/smallcaps, Text sem newline/tab/segmentação linguística. Região recusada tem None. Preparação global anterior permanece para parágrafos que não precisam do contexto bidi; para o subconjunto certificado com base RTL ou caracteres RTL/override oposto, encaminhar pelo incremental sem mudar os breaks permitidos. Não interferir em optimized explicitamente escolhido. O owner privado bidi_origin prepara/valida; esta unidade só coordena o seu papel. Testes de origem por ocorrências, UTF8, restauração e parbreak; validação bilateral de wraps.

## Palavra contígua overlong — 2026-10-08

### Medição anterior à decisão

`plain300.typ` produz uma palavra composta por centenas de fragmentos `Text` adjacentes: `paragraph_prepare.rs:130-157` junta-os sem gap, mas `paragraph_plan.rs` devolve a seleção de linha única ao fallback por `regional_only && lines.len() < 2`. O fallback `word_layout.rs:27-47` decide quebras por fragmento, enquanto o coletor vanilla `inline/collect.rs:275-298` agrega texto adjacente antes de selecionar. A sonda bilateral com string única conserva uma linha em ambos os motores; com `for` e `[#i]`, somente o Tekt quebra. Proveniência e resultados em `00_nucleo/diagnosticos/fluxo-numerico-contextual-2026-10-08/`. Esta observação não demonstra intenção histórica.

### Decisão

Correção interna de paridade pelo ADR-0127. Se o preparo global certificado contém exatamente uma palavra sem gap inicial ou interno, cuja largura excede a largura disponível e cuja seleção resulta em uma linha, emitir essa linha pelo caminho preparado também sob a guarda `regional_only`. Um separador terminal não cria ponto de quebra dentro da palavra e conserva a emissão já definida. Conservar o fallback de linha única para os demais casos. Não introduzir quebra em fronteira entre fragmentos, alterar a regra de hifenização ou afetar escopos recusados. Aceitação RED→GREEN: fluxo `for` contíguo overlong coincide em linha e texto com o vanilla, com controles de palavra curta e espaços.

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

Conservar certificação inteira e recusas. Preparar a forma de escopo depois da entrada lexical válida; passar Shape inicial/continuação a seletores sem hifenização ou com candidatos. Facades uniformes continuam usadas quando a forma é uniforme. Não trocar estratégia por hanging nem emitir conjunto parcialmente recusado.

### Aceitação

RED→GREEN antes da implementação; controles fonte→layout e bilaterais dos sinais,
fronteiras, variantes e restauração pertinentes. Não deduzir cobertura de todas
as alternativas por executar um arm agrupado: classificações nominais fechadas
do contexto inline/fluxo são intencionais e exigem testes por classe/variante.
Resselo apenas após V15/V26 válidos; testes, build, Clippy/Rustdoc/fmt, reversa
e diferencial Tekt do pin1279707, sob12GiB/High8GiB/swap0/jobs1. Sem publicação.
