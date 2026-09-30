# L0 — paragraph_hyphen
Hash do Código: d8391ef7

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/paragraph-costs.toml sha256:70b72ac9efb2bf81aa4ef496c4e1c867264f696d454a9916e7e2e5f385b92020
- 00_nucleo/prompts/_nuclei/paragraph-selection.toml sha256:363fc473f68d7a9faea59460d462eae77c4eb7c6df1360b91a30f6b6f0dd92ef

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/paragraph_hyphen.rs`
**Architecture:** ADR-0107/0108/0109/0127/0129/0130.

## Hifenização global — 2026-09-28

### Medição anterior à decisão

O lote before-pdf de `00_nucleo/diagnosticos/correcao-hifenizacao-global-2026-09-28/` mede 67 entradas contra vanilla 9f2b6e8715237cb086899a42873660fe744622e8; estado inicial, binários, fontes e comandos estão em initial.json, before/ e proveniencia.json. paragraph_prepare.rs:80-94 recusa a política true, paragraph_select.rs:66 considera só finais de palavras, paragraph_plan.rs:18 escolhe estratégias já transportadas, break_selection.rs:41-66 hifeniza apenas por overflow local. Vanilla inline/linebreak.rs:170-209 usa os mesmos candidatos no greedy, :626-684 aplica 135*(1+0.15*(max(5-left,0)+max(5-right,0))) por hífen, mais 135 em duas linhas consecutivas; inline/shaping.rs:583-635 desenha o glyph de '-' mas conserva SHY no texto copiado. Isto mede comportamento e contratos, não intenção histórica.

### Decisão

Correção interna ADR-0127 em fluxo contínuo: sem campo/trait/assinatura pública, modo novo, fase ou quebra de compatibilidade. A restrição histórica de hifenização desligada é supersedida nesta rodada para palavras latinas de estilo/política uniformes, inclusive pontuação terminal simples. Mistura dentro da palavra, apostrofes internas, outras quebras UAX14, largura variável e objetos mantêm fallback. Texto copiado '-' versus SHY continua limitação de export independente; a geometria usa o glyph de '-' separado, como fonte medida. Aceitação mede palavras/posições/estilos e política lexical, normaliza somente o hífen de quebra para a comparação geométrica e declara a divergência textual separadamente.

Preparar dados privados de hifenização para Word uniforme: consultar helper hyphenation com idioma efetivo (default English); offsets em bytes derivados de chars, left/right em chars da palavra alfabética; trailing .,!? não altera comprimentos da palavra alfabética. Medir cada range entre offsets injetando FontMetrics; guardar larguras e largura separada do glyph '-'. Palavra sem pontos/padrões mantém candidato normal. Guardar apenas dados puros, sem FrameItem/Layouter ou I/O neste owner.

O grafo ordenado tem origem, cortes dentro das palavras e finais normais de palavras, último mandatory. Medir candidatos por prefixos das palavras/gaps, substituindo larguras do começo/fim cortados pelos ranges pré-medidos, sem aditividade suposta entre syllables. Greedy segue o percurso vanilla medido: ao overflow confirma último fitting, reconstrói desde seu fim e confirma também qualquer candidato indivisível ainda overfull; mandatory fecha. Portanto pode hifenizar palavra que começa na origem da linha.

Optimized usa DP com predecessor por breakpoint, custo-base compartilhado, penalidades 135 e edge-distance, mais 135 se predecessor e final atuais hifenizam; runt só quando mandatory e predecessor é breakpoint imediatamente anterior. Desempate aceita predecessor posterior; poda ativa somente com larguras não negativas e ratio abaixo do mínimo, preservando tentativa atual. Retrace cria apenas as linhas escolhidas. Não copiar bounds aproximados nem alegar ganho temporal sem medição.

Linhas escolhidas conservam fragmentos originais completos quando não cortados. Corte só ocorre em Word de fragmento uniforme. Construir texto substring + fragmento '-' separado, cada qual com largura/estilo exatos; resto conserva UTF-8 e estilos. Gaps iniciais de linha normal são aparados; leading original pertence apenas à primeira linha; gap após final normal é carrier trailing, nunca espaço artificial entre prefixo e resto. Sem candidatos negativos ou offsets fora do texto.

Aceitação RED→GREEN: preparação true é aceita, Unicode português íntegro, início overlong hifeniza, false e auto lexical, simple/optimized distintos quando custo exige, penalidade consecutiva aplicada, sem emissão parcial quando misturam políticas; regressões do seletor sem hifens e corpus bilateral no pin. Build e lint sem erros.

Idioma default é medido em lab/typst-original/crates/typst-library/src/text/mod.rs:473-475 (English). Testar métricas injetadas não aditivas, offsets Unicode, recomposição, política desativada/mista, pontuação terminal e penalidades consecutivas. Esses testes não usam igualdade de bytes PDF como gate.

### Medição e correção da margem óptica

after-pdf inicial mede quebras coincidentes, mas linhas justificadas com hífen ficam 2.1252pt aquém do vanilla; en-140-optimized-True põe ex- em xMin115.26/xMax132 contra117.385197/134.1252. Fonte: lab/typst-original/crates/typst-layout/src/inline/line.rs:518-526 acrescenta overhang somente ao remaining da emissão; :708-709 fixa 0.55 para glyph '-' ou SHY; text/mod.rs:391-393 default true. Não alterar custo/seleção. Prepared guarda right_overhang privado, default zero; linhas materializadas terminadas em hífen recebem dash_width*0.55; emissor usa width+overhang para ajustar os espaços. Overhang textual configurável e de outras pontuações permanece escopo pendente, com controle separado.

## Remoção de hardcode funcional — 2026-09-28

### Medição anterior à decisão

`correcao-hardcode-paragrafo-2026-09-28/before-pdf/` mede 20 entradas contra
9f2b6e8715237cb086899a42873660fe744622e8 no estado `initial.json` e `before/`.
`eval/rules.rs:3483-3525` e `stdlib/text/constructor.rs:153-190` não capturam
`overhang`; auto/int/string são aceitos indevidamente. A fonte vanilla
`text/mod.rs:391-393` declara bool default true; `inline/line.rs:518-525`
condiciona a compensação ao estilo final. `paragraph_hyphen.rs:221-224`
sempre multiplica a largura por0.55. A suspeita inicial sobre `paragraph_select.rs:78-82` foi refutada antes de
implementar: vanilla `inline/linebreak.rs:551-559` usa attempt.justify, que
`inline/line.rs:138-139` desativa para mandatory. Conservar a fórmula atual
e testar a última linha natural. Isto mede comportamento, não intenção histórica.

### Decisão vigente

Correção interna/de paridade em fluxo contínuo ADR-0127, sem campo ou assinatura
pública. Não confundir constantes normativas de um algoritmo com configuração
ignorada. O fator0.55, custo-base135, distância5 e incremento0.15 vêm da fonte
pinada; identificar essas constantes pelo significado e referência, sem alegar
que renomear números implementa configuração. Custos customizados continuam
escopo pendente. Conservar identidade textual literal do hífen nesta rodada:
ela requer carrier/contrato separado, não heurística sobre glyph ou posição.

Usar overhang do fragmento final para ativar a margem óptica; false produz zero. Guardar contagens Unicode em usize sem saturação arbitrária255; saturating_sub no limite normativo5 já delimita o efeito do custo. Nomear constantes normativas. Conservar todas as escolhas/penalidades padrão.

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

Receber LineCosts privado; usar custo efetivo hyphenation na borda e em linhas consecutivas e passar custos ao line_cost. Retirar default135 deste owner; manter distância/incremento/margem normativos. Offsets/ranges/estratégia/emissão permanecem.

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

## Identidade textual — materialização aprovada

### Medição anterior à decisão

As sondas bilaterais e o estado exato em
`00_nucleo/diagnosticos/identidade-hifen-discricionario-2026-09-28/initial.json`
e `before/` medem U+002D versus SHY com geometria coincidente em oito controles.
A inspeção do percurso FrameItem deste consumer está arquivada em approved-before/.
O dono confirmou o contrato em `00_nucleo/prompts/entities/layout_types.md`.

### Responsabilidade exclusiva

Na materialização de corte escolhido, fragmento visual hífen recebe replacement Some(U+00AD); ranges de texto comum recebem None. Não alterar seleção, custos, largura ou overhang. A limitação textual antiga é supersedida apenas para quebras automáticas escolhidas.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.
