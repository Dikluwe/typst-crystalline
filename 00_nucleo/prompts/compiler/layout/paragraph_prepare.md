# L0 — paragraph_prepare
Hash do Código: 81ae3966

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/paragraph-costs.toml sha256:70b72ac9efb2bf81aa4ef496c4e1c867264f696d454a9916e7e2e5f385b92020
- 00_nucleo/prompts/_nuclei/paragraph-selection.toml sha256:e9038b453bbb1971c9a75a76eced99c35db742f57bebbb09b03c4922b3862800


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/paragraph_prepare.rs`
**Architecture:** ADR-0107/0108/0109/0127/0129/0130.

### Medição anterior à decisão

O lote bilateral `00_nucleo/diagnosticos/correcao-seletores-paragrafo-2026-09-28/before-pdf/resultados.json` mede 32 entradas, com estado exato em initial.json e before/, contra vanilla 9f2b6e8715237cb086899a42873660fe744622e8. Em 140pt/margem 8pt, simple escolhe delta epsilon zeta eta, optimized escolhe delta epsilon zeta e junta eta à linha seguinte; Tekt ignora a estratégia. Em 240pt justificado, optimized admite eta na primeira linha reduzindo os espaços. Fonte: eval/rules.rs:3070-3100 e stdlib/structural/par.rs:131-150 validam e descartam linebreaks; text.rs:41 e word_layout.rs:29 escolhem incrementalmente; vanilla inline/mod.rs:194-196 resolve auto pela justificação, linebreak.rs:raw_ratio/raw_cost aplica redução até 1/3 e custos cúbicos/quadráticos; model/par.rs:Limits::SPACING_DEFAULT fixa 2/3..1.5. Estilos adjacentes, literal leading/trailing e conteúdo opaco estão medidos separadamente. Observação não prova intenção histórica; inferência refutável: os scope-outs L0 explícitos de preparação/optimized legitimaram o adiamento, pois ainda vedam a implementação global apesar do transporte aceito.

### Decisão e escopo

Correção interna de paridade ADR-0127: sem novos contratos públicos, flags ou fase. Primeira preparação global certificada: parágrafo latino composto por Text, Space, SpaceFromNewline, Empty, Sequence, Styled tipográfico e Strong/Emph, com hifenização desligada, direção LTR, sem matemática/smallcaps; tracking uniforme segue a extensão certificada de 2026-09-29, largura finita e constante, fora de colunas/células/floats. O conjunto completo deve ser certificado antes de emitir; uma unidade recusada preserva o caminho anterior. Hifenização, Unicode UAX14 completo, links/objetos/linebreaks, largura variável, limites customizados e seleção incremental restante continuam explicitamente incompletos; este lote não declara estado da arte nem otimização completa da língua.

Preparar palavras com fragmentos TextStyle efetivos e métricas word injetadas, gaps com largura space/origem/estilo; espaços literais iniciais conservam avanço, markup inicial não. Não inserir espaço em adjacência: fragmentos contíguos pertencem à mesma palavra; fragmentos de mesmo estilo podem unir antes da medição. Retirar gaps finais do custo sem perder carriers na emissão. Styled só tipografia certificada: rejeitar par.*, marcadores e custom desconhecido, math/tracking aninhado/direção RTL e hyphenate resolvido true; Strong/Emph conservam proveniência. Restauração da chain e estilo inclusive em erro; nenhuma emissão/cursor/location.

## Aceitação

RED→GREEN: estratégias distinguíveis sob a mesma entrada; auto e overrides lexicais; caixas bilaterais com tolerância 0.0005pt e linhas físicas agrupadas por baseline, não contagem XML. Controles de texto íntegro, estilos fortes/emph, literal leading/trailing, adjacência, overlong, restauração e fallback. Build e lint sem violations.

Preparar também uma fatia parts emprestada, com o mesmo collect e finalização do body; nenhuma alocação/clone de Content é necessária. Segmentos totalmente vazios não fabricam Prepared material.

## Hifenização global — 2026-09-28

### Medição anterior à decisão

O lote before-pdf de `00_nucleo/diagnosticos/correcao-hifenizacao-global-2026-09-28/` mede 67 entradas contra vanilla 9f2b6e8715237cb086899a42873660fe744622e8; estado inicial, binários, fontes e comandos estão em initial.json, before/ e proveniencia.json. paragraph_prepare.rs:80-94 recusa a política true, paragraph_select.rs:66 considera só finais de palavras, paragraph_plan.rs:18 escolhe estratégias já transportadas, break_selection.rs:41-66 hifeniza apenas por overflow local. Vanilla inline/linebreak.rs:170-209 usa os mesmos candidatos no greedy, :626-684 aplica 135*(1+0.15*(max(5-left,0)+max(5-right,0))) por hífen, mais 135 em duas linhas consecutivas; inline/shaping.rs:583-635 desenha o glyph de '-' mas conserva SHY no texto copiado. Isto mede comportamento e contratos, não intenção histórica.

### Decisão

Correção interna ADR-0127 em fluxo contínuo: sem campo/trait/assinatura pública, modo novo, fase ou quebra de compatibilidade. A restrição histórica de hifenização desligada é supersedida nesta rodada para palavras latinas de estilo/política uniformes, inclusive pontuação terminal simples. Mistura dentro da palavra, apostrofes internas, outras quebras UAX14, largura variável e objetos mantêm fallback. Texto copiado '-' versus SHY continua limitação de export independente; a geometria usa o glyph de '-' separado, como fonte medida. Aceitação mede palavras/posições/estilos e política lexical, normaliza somente o hífen de quebra para a comparação geométrica e declara a divergência textual separadamente.

Fragment captura boolean lexical, usado também para não unir fragmentos de política distinta. Word conserva candidatos/ranges privados produzidos pelo owner paragraph_hyphen. Remover recusa global de true; finalizar larguras finitas e certificar palavras com política ativa através do novo owner. Não alterar estilo/chain em erro nem emitir parcialmente.

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

Guardar overhang resolvido em Fragment privado, default true da linguagem; incorporar na fusão de fragmentos e permitir text.overhang na whitelist tipográfica. Não fundir políticas distintas dentro da palavra nem alterar a chain após preparação.

## Custos privados — adaptação dos controles existentes

Medição anterior: os dois testes de margem chamam paragraph_hyphen::select com
custos default implícitos. O novo contrato privado do seletor exige LineCosts
explícito, conforme text_costs.md e as 66 sondas before-pdf da rodada de custos.
Adaptar somente esses dois calls em cfg(test) para LineCosts::default(); conservar
inputs e expectativas. O refinamento subsequente desta rodada autoriza a
whitelist dos quatro canais neutros de costs, conforme medição e decisão abaixo.

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

Reconhecer os canais de custos pelo helper privado text_costs::is_channel,
reutilizando FIELDS do owner em vez de duplicar nomes na whitelist.

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

Fragment privado ganha replacement: Option<String>, None para texto normal; preservar no clone, não unir fragmentos com identidade diferente. Dados visuais e métricas permanecem.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.

## Tracking uniforme na preparação global — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/paragrafo-tracking-global-2026-09-29/focused-before/results.json` mede oito combinações de tracking 0, ±0,2 e 1 pt com Simple/Optimized, fonte e binários pinados em `initial.json`. Todas conservam o texto, mas o Tekt distribui 10+2 ou 9+3 palavras nas páginas, enquanto o vanilla escolhe 7+5 ou 6+6. O controle sem a propriedade tracking (`no-tracking-baseline.json`) coincide: 7+5 em Simple e 6+6 em Optimized. `paragraph_prepare.rs:78-95` recusa qualquer `Some(tracking)`, inclusive zero; o planejador então não chama os seletores nem a proteção regional. A hipótese é refutável se o mesmo documento com a preparação ampliada conservar distribuição divergente. Não se infere intenção histórica.

### Contrato interno

Admitir tracking resolvido finito em conteúdo textual LTR de estilo uniforme, sem Styled/Strong/Emph internos, hifenização ativa com tracking resolvido não nulo, matemática, smallcaps, objetos ou largura variável. Continuar a recusar escopos mistos e toda sequência cuja largura efetiva de palavra/gap seja negativa ou não finita. A métrica injetada permanece autoridade para tracking interno da palavra; o preparo não soma novamente esses intervalos. Expor helper privado da extensão uniforme para seleção/emissão. A ausência de propriedade e tracking explícito zero produzem a mesma largura, mantendo a distinção lexical fora deste subconjunto. Nenhuma mutação de Content, cursor ou região na preparação.

## Fronteira comprovada de tracking explícito zero com hifenização — 2026-09-30

### Medição anterior à decisão

`00_nucleo/diagnosticos/paragrafo-tracking-hifenizacao-2026-09-30/after-third/results.json` mede quatro controles com `tracking: 0pt` (100/140 pt, Simple/Optimized): texto, páginas, sequência e caixas coincidem com o vanilla, com desvio máximo inferior a 0,001 pt. Em tracking não nulo, há divergências de 0,2/1 pt e novas quebras; `after-second/results.json` mostra que somar o intervalo em todos os cortes corrige alguns e erra outros. A fonte pinada em `inline/shaping.rs:546-571` preserva glyphs de cortes seguros ou remodela o segmento; o contrato `FontMetrics` atual não expõe esse sinal. Inferência refutável por futuro adapter de shaping com informação de cortes seguros, não por hardcode de prefixos.

### Decisão final deste passo

Supersede a admissão ampla de hifenização com tracking escrita acima: quando houver candidato de hifenização, admitir tracking explícito somente se o valor resolvido for exatamente zero e o restante do subconjunto uniforme for certificado. Tracking não nulo conserva fallback integral, sem seleção/emissão parcial; nenhum valor de tracking é alterado. Não ampliar trait pública, fase ou modo padrão sem gate ADR-0127 próprio. O contrato de tracking uniforme sem hifenização permanece válido.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: 00_nucleo/diagnosticos/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em materialization/table-origens-d08b3c2-2026-10-07/contract.md.

Source não altera aceitação/preparo de parágrafo simples/optimized: coletar body transparente, mantendo chain, tokens, spans bidi/hifenização e custos. Refutadores de prefixo/render C2 exigem não recusar preparação só pela origem.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.

## Fluxo numérico contextual contíguo — 2026-10-08

### Medição anterior à decisão

Os fixtures `context100.typ`, `context300.typ` e `plain300.typ` em `snapshot-scopes-prioridade7-2026-10-08/fixtures/` emitem algarismos sem separadores. A comparação pinada em `atual-vanilla-memoria5-2026-10-08.md` mostra uma linha overlong vanilla e várias linhas Tekt. Sonda desta rodada: uma string única fica em uma linha nos dois motores; `for` com `[#i]` diverge. A inspeção do conteúdo realizado em `paragraph_plan.rs` encontra `sequence([context(ContextBlockElem), text(...)])` por iteração contextual; `context_block.rs:15-36` confirma que ContextBlock não emite material em layout paginado ordinário e só produz marcador quando `capture_html_context_markers` está ativo. Isto mede comportamento e fonte, não intenção histórica. O estado exato da sonda está no diagnóstico `fluxo-numerico-contextual-2026-10-08/`.

### Decisão

Correção interna de paridade, em fluxo contínuo pelo ADR-0127. Na preparação global, tratar `ContextBlock` como neutro somente quando a captura de marcadores estiver desligada. Quando ligada, recusar o preparo inteiro e conservar o caminho existente. Não executar a closure, perder marcador, criar espaço ou fronteira de palavra, nem alterar seleção ou fase. Aceitação: os três fixtures mantêm a sequência textual e a mesma linha overlong do vanilla; captura HTML continua no caminho próprio.


## TS1 — tamanho completo de texto (ratificado pelo humano, 2026-10-17)

### Medição anterior à decisão

`00_nucleo/diagnosticos/text-argumentos-2026-10-17/before.json.gz`, `before-measurement.json.gz`,
`morph-before-measurement.json.gz` e `audit.json` conservam HEAD
06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, UTC, status/diffstat,
fontes/L0/binários/argv e o pin9f2b6e8715237cb086899a42873660fe744622e8.
Em named-em, A mede0pt contra24pt; em affine,3pt contra27pt; nested sobre18pt
mede0pt contra36pt. As seis permutações Size/Color/body são aceitas pelo pin
mas rejeitadas pelo Tekt. Set posicional18pt é ignorado (12 contra18pt).
Fonte pinada text/mod.rs:290,303,924–930,1136–1142 e args.rs:178–247
estabelece busca tipada, preferência nomeada, body restante e composição.
style_chain.rs:513–517 e constructor.rs:123 descartam em; collapse em
style_chain.rs:394–395,419–422 aplica top-wins onde tamanho precisa fold.
É tamanho/escopo observável da linguagem; a representação Rust é escolha
interna. Não inferir intenção histórica. Refutar por restauração, Raw,
heading, math, capture/show, dupes/spans, zero/negativos ou métricas divergentes.
A sonda de igualdade refutou a hipótese de comparar os valores declarados:
no pin, text(size:2em)[A] == text(size:24pt)[A] é true. StyledElem::eq no pin
content/mod.rs:779–782 compara child. Esse defeito de igualdade requer sua
própria correção; o tipo completo não é prova de fechamento desse eixo.

### Ratificação

O humano aprovou TS1 e os dois contratos públicos da proposta de manifesto
771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba.
Implementação e testes S1–S16 sob 12 GiB/swap zero; sem release/merge/push.
Resselo somente após implementação e grafo válido. RG1 continua independente.

### Responsabilidade exclusiva adicional

Somente fixtures/adapters de testes: substituir Style::Size(Pt) por Size(Length absoluto), e expectativas de StyleDelta.size por Option<Length>. Conservar os observáveis linguísticos e os contratos funcionais existentes; não trocar geometria correta por snapshots da representação nova.
