# L0 — Infrastructure end-to-end test harness
Hash do Código: d3c17558

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/math/bare-insertion.toml sha256:5cfb723d50e9518cdcee2df348927d02330b6019df96acef8394c1429cc950ff
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8
- 00_nucleo/prompts/_nuclei/paragraph-costs.toml sha256:70b72ac9efb2bf81aa4ef496c4e1c867264f696d454a9916e7e2e5f385b92020
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee


**Layer:** L3 (cfg(test) only)
**Exclusive consumer:** `03_infra/src/integration_tests.rs`

## Current evidence

- `03_infra/src/integration_tests.rs` — the test-only module and compile/eval/frame helpers.
- `03_infra/src/integration_tests.rs:5203` — inverted-bracket witness from `lab/corpus/legacy-sections/sec_22.typ`.
- `03_infra/src/integration_tests.rs` — bundle feature-gate and active-target cases.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Own only this end-to-end harness. Exercise SystemWorld, temporary filesystem, evaluation, introspection, layout and export across the real infrastructure boundaries. Helpers remain test-only/private and remove their temporary resources. Productive modules keep their individual owners; test setup cannot move I/O or concrete World access into L1.

Assert language values, morphology, diagnostics/warnings and observable exported structures. Byte/internal-structure identity is acceptance only when explicitly selected as the observable, such as the independently owned snapshot corpus.

Geometric helpers accumulate ancestor affine transforms: Group translation, matrix and inherited transform compose in exporter order; nested groups compose transitively. Project Text, TextShaped, Glyph, Image, Shape and Line endpoints once into global coordinates. Semantic is transparent and Link keeps its existing convention. Counting/classification may inspect local variants, but geometric assertions use global coordinates. Do not compensate an observer bug by changing expected geometry, fixtures or productive layout.

For the pinned no-text placement witnesses, check an empty page's requested PDF MediaBox, preserved cover-image clip and placed-text baseline against the independently observed vanilla geometry. Run the real World and production pipeline for font-demand controls. A no-font declaration with an isolated shape or image must not request the font book; a placed nontext body must meet the same condition without suppressing explicit font-family validation or text shaping. These assertions belong to this test harness; the productive owners remain responsible for their mechanisms. Record RED failures without weakening expectations to match current output.

Use public color values (`red`, `blue`, `black`) in shape/rotation fixtures, not strings that trigger cast errors. `lab/corpus/legacy-sections/sec_22.typ` contains normal and inverted bracket pairs; observe base character, glyph identity and advance and require equivalent vertical variants when positional role changes. That witness protects `00_nucleo/prompts/compiler/math/layout/delimited.md` without requiring a particular MathDelimited representation.

Bundle tests distinguish feature absence, out-of-target calls and the active target. Absent bundle feature must fail during eval with its exact gated diagnostic/hints, before layout/export. Active tests cover byte/UTF-8 assets, nested/normalized paths, duplicate asset/document paths, empty bundles, mixed PDF/SVG/PNG/HTML document entries, explicit/unknown formats, HTML without its feature and loose top-level content. Asset names/payloads are exact observables; document acceptance uses language/format structure. Do not accept an empty placeholder asset as data or leave outputs outside isolated temporary directories.

## Verification boundary

Retain existing integration cases and their owner boundaries; this prompt does not assert a current full-suite result or authorize productive changes to accommodate observer mechanics.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`03_infra/src/integration_tests.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

Adaptar projeções e traversals exaustivos de fixtures ao carrier: posição é transportada; neutro para contagem de pontos, tinta e medições de comportamento existente.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

## Política de hifenização — correção de integração 2026-09-28

### Medição anterior à decisão

correcao-politica-hifenizacao-2026-09-28/first-infra-tests.log mede 1117 passados e 5 falhas: três canaries esperam warnings de hyphenate descartado e dois testes de padrões en/pt ativam só lang, presumindo hifenização default. As 52 sondas bilaterais e a fonte vanilla text/mod.rs:545-565 refutam essa presunção: auto depende de justify. O fonte e este L0 ainda inalterados são conservados em before/ e têm os hashes de initial.json.

### Decisão e aceitação

Migrar os canaries preservando todas as contagens, spans distintos e identificação de propriedade/target: o simples e o repetido usam stroke:1pt; o trio usa overhang:true, baseline:3pt e stroke:1pt. Nos dois controles positivos dos padrões en/pt, explicitar hyphenate:true e preservar a exigência de pelo menos um hífen. Adicionar um controle de integração diferenciando false/true/auto/absent e justify; não atualizar snapshots nem tolerâncias. Estas mudanças de input mantêm a finalidade dos testes e são explicitamente justificadas pela nova semântica medida, não redução da asserção para aceitar output candidato.

## Composição de parágrafos — 2026-09-28

### Medição anterior à decisão

O lote before-pdf em correcao-composicao-paragrafos-2026-09-28 mede 34 entradas contra o pin 9f2b6e8715237cb086899a42873660fe744622e8, no estado initial.json e before/. Com 180pt e margem 8pt, a primeira linha justificada termina em 145.52pt no Tekt e 172.000002pt no vanilla; a última fica em 110.048pt em ambos. BEFORE/par[MIDDLE]/AFTER produz 2 linhas físicas contra 3. Empty-start põe AFTER em yMin 3.74 contra 18.14; empty-auto mede altura 24.568 contra 38.968. native-leading-two-lines demonstra que o escopo de leading precisa fechar antes de restaurar estilo. Fonte: line_commit.rs:195-236 já possui justificação; word_layout.rs usa flush ordinário em quebras; layout/mod.rs:1469 trata Par como transparente; style_scope.rs:18-77 restaura estilo depois do body; eval/rules.rs:663-705 agrupa fluxo para show. Agrupamentos XML do extrator não são linhas físicas: coordenadas coincidentes são medidas separadamente.

### Decisão vigente

Correção de paridade ADR-0127, em fluxo contínuo: sem nova assinatura pública, entidade, trait, flag ou fase. O núcleo pinado define invariantes compartilhados, não é owner de código. Preparação global e optimized ficam incompletos e pertencem à futura rodada-paragrafo-seletores, depois de medir escolhas/custos.

Adicionar controles de par nativo antes/dentro/depois, empty-start/empty-auto, justificação em linha continuada e largura natural da última, override nativo e paginação. Medir posições e texto, não só contagem de páginas. Não atualizar snapshots nem tolerâncias para aceitar regressão; expectativas vêm do vanilla ou da fórmula independente com FixedMetrics. Esta rodada não mede custos nem optimized.

### Refinamento de precedência medido antes da implementação

No estado precedence-state.json, regression-acceptance.json registra 191 verificações passando e duas falhas no controle show-par-sets-false. O Tekt emite extraordi-/interna- onde vanilla conserva palavras sem hifenização. show-par-full-tree.log mostra SetPage e o escopo global dentro de um Par sintetizado; a recipe fica exterior ao text.hyphenate true. eval/mod.rs:1532-1560 possui a proveniência exata dos escopos #set; rules.rs:660-727 apagou essa distinção ao tratar Styled sempre como inline. Não inferir origem pelo conteúdo do Styles.

Decisão interna: o fold dos escopos #set em eval_markup transporta marcador privado eval.flow.scope=true no Styles existente. Antes de formar o primeiro parágrafo material, realize_flow reconhece esse marcador e mantém o escopo exterior à realização dos seus parágrafos, preservando os carriers prévios SetPage/Empty/Space/SpaceFromNewline em ordem, sem os embrulhar em Par. Após conteúdo inline material o escopo permanece inline dentro do parágrafo corrente; wrappers de constructors/estilos sem marcador permanecem inline. Nenhuma entidade, assinatura, comportamento válido por defeito ou fase pública muda. Pin no núcleo compartilhado paragraph-composition documenta a distinção; não recorrer a heurística por chave de estilo. O consumer do hub só produz o marcador no fold; preparação e reconhecimento são do owner rules.

### Refinamento de escopo estilizado integral

precedence-pdf/resultados.json e proveniencia.json medem oito controles no binário anterior a este refinamento. native-text-inline-override e mid-par-set-override hifenizam em ambos (diferença -/U+AD já aberta). native-text-only-override diverge: o Tekt hifeniza e o vanilla não. Portanto a hipótese "todo constructor Styled permanece inline" é insuficiente. Um Styled que contém todo o material de um fluxo ainda sem material fornece o escopo inicial de realização do parágrafo; o show-set precisa ser aplicado dentro dele. Quando há material anterior ou posterior fora desse wrapper, ele permanece inline e não altera fronteiras.

Decisão interna: além do marcador #set, reconhecer em realize_flow um wrapper Styled com prefixo e sufixo contendo apenas Empty/Space/SpaceFromNewline/SetPage. É fatoração de escopo que cobre todo o material, não inferência por campos de Styles. Conservar os carriers em ordem e realizar o body no escopo exterior existente. Não promover fragmentos com texto fora do wrapper. A regra específica integral supersede apenas a exclusão absoluta dos constructors na cláusula anterior; nenhum contrato público muda.

## Observação de hifenização global — 2026-09-28

### Medição anterior à decisão

`correcao-hifenizacao-global-2026-09-28/infra-verified.log` registra três
falhas: en, pt e precedência lexical. `03_infra/src/integration_tests.rs:3657`
conta apenas Text com mais de um caractere terminando em hífen. O emissor
`01_core/src/compiler/layout/paragraph_emit.rs:68-73` preserva os fragmentos;
`paragraph_hyphen.rs` materializa o prefixo e o hífen em fragmentos separados.
As sondas fonte→PDF no mesmo diagnóstico medem palavras e posições coincidentes
em 66 dos 67 controles; o predicado de fragmento rejeita hifenização visível.
Isso mede um defeito do observador, não autoriza reduzir a expectativa de língua.

### Decisão e aceitação

O helper de hifenização dos fixtures `build_doc` deve aceitar tanto palavra+hífen
no mesmo Text quanto prefixo alfabético e Text hífen separados mas contíguos na
mesma baseline e página. `build_doc` usa `layout()` com FixedMetrics: calcular
adjacência com esse contrato, sem inferir largura de métricas de fontes reais.
Usar a projeção existente de grupos; não juntar por mera ordem de emissão.
Conservar todos os inputs e expectativas de opção/idioma. Adicionar controles
negativos de hífen isolado, separado por espaço e em outra linha e um positivo
contíguo. Não alterar código produtivo nem snapshots para acomodar o observador.

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

O canary debt49 precisa conservar três warnings distintos; substituir apenas overhang já suportado por costs:(hyphenation:100%), ainda scope-out. Preservar spans, contagem e baseline/stroke. Não reduzir expectativas nem aceitar warning indevido de overhang.

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

Conservar três warnings no canary debt49; substituir costs hyphenation agora capturado por kerning:false, campo conhecido ainda scope-out. Preservar spans, baseline/stroke e outras expectativas.

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

`lab/results/construction/diagnostics/correcao-viuvas-orfas-2026-09-28/before-pdf/` registra 78 entradas e `before-extra/` registra 24 controles, com estado completo em initial.json e before/, contra vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8. page-65-100%-0%-False deixa três linhas e uma no Tekt, mas duas e duas no vanilla; custos zero coincidem. Tiny/large-positive têm a mesma proteção vanilla. `lab/typst-original/crates/typst-layout/src/flow/collect.rs:196-229` mede habilitação estritamente positiva, primeiros dois/últimos dois e proteção integral de três linhas; `flow/distribute.rs:343-364` só avança por necessidade agrupada quando cabe na próxima região. paragraph_emit.rs emite linha por linha sem essa necessidade. A semântica observável é a distribuição de linhas/palavras em regiões, não os frames/Child do vanilla. Não inferir intenção do scope-out antigo.

### Decisão vigente

Correção interna/de paridade em fluxo contínuo ADR-0127: nenhum campo/trait/assinatura pública, flag ou fase nova. Supersede a exclusão anterior de widow/orphan exclusivamente para parágrafos preparados com métricas verticais uniformes, sem baseline offset, geometria paginada constante, sem colunas/células/subframes/notas/floats ou transação regional ativa. Os demais contextos conservam fallback e limites explícitos. Não promete algoritmo completo nem desempenho.

O módulo privado paragraph_pagination possui exclusivamente certificação vertical, agrupamento e admissão regional. Recebe ranges já escolhidos, não refaz seleção, não emite glyphs nem duplica a lógica de new_page. A certificação inteira precede emissão; todos os fragmentos devem ter size/arestas/leading iguais ao estilo exterior (negrito/itálico admissíveis se as métricas coincidirem), e finite nonnegative top/descent/leading. Bounds auto não precisam de proteção. Respeitar a baseline inicial e o colapso existente de block-chain antes de comparar necessidade real com page_bottom_limit. Novo espaço disponível é calculado com margens resolvidas para a página seguinte. Avançar só se há material na região atual e o grupo cabe na próxima. Grupos muito altos conservam progresso ordinário e conteúdo íntegro; nunca repetir new_page em laço.

Helpers privados de text_costs resolvem widow/orphan independentemente dos pesos hyphenation/runt, reutilizando FIELDS, com default Ratio 1 e teste estrito > 0. O default de estratégia do planejamento passa a Simple para parágrafo comum sem linebreaks, permitindo a preparação necessária à proteção padrão; a escolha automática por justify e todos os contextos recusados permanecem. Emitir sem alterar justificação, carriers, última linha aberta, fronteiras e restauração lexical. Preservar notas e colunas por não aplicar proteção incompleta nesses caminhos.

Aceitação: RED→GREEN com mesmas palavras distribuídas 2+2 versus 3+1, proteção integral de três, orphan desligado independentemente, grupo excessivo, restauração e default sem flag. Sondas bilaterais com caixas/palavras/páginas, tipos/spans anteriores preservados e controles de fallback. Não usar grupos XML como número físico de linhas. Resselo restrito aos owners, build e crystalline-lint sem erros.

## Identidade textual — controles do contrato aprovado

Medição anterior: initial.json, before/resultados.json e red.log em
`lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/` registram
texto/caixas e a perda de SHY. O dono confirmou o contrato
`00_nucleo/prompts/entities/layout_types.md`. A compilação de fixtures exaustivas
exige tratar a variante nova. Projeções visuais descem nos filhos; extractors
textuais usam text uma vez. Expectativas de identidade discricionária passam a
SHY, mantendo U+002D literal. Não relaxar caixas/seleção. A suíte de shaper
exercita fixture Noto obrigatória e ausência de fontes, conserva envelope e
prova que text substituto não altera glyphs/avanços nem contamina o cache.

## Migração do carrier bidi ratificado (2026-10-05)

Medição anterior à decisão: o inventário `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json` identifica as referências desta unidade e os owners 1:1. O contrato do manifesto 1ae7513e foi confirmado; nesta unidade ajustar constructors não certificados com None e patterns de leitura com `..`, preservando os observáveis de linguagem existentes. Transformações obedecem ao núcleo pinado de origem; sem ampliar fase/API.

## Inserção matemática automática — proposta sujeita ao gate ADR-0127

### Medição anterior à decisão — D06, 2026-10-06

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado,
UTC/diff/estado/fontes/binários nas coletas canônicas
lab/results/construction/diagnostics/math-bare-2026-10-06/{before,additional,refuters}.json.gz.
Vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8; proveniência no relatório
lab/results/construction/diagnostics/math-bare-medicao-2026-10-06.md.

before: frac, mat, sqrt, lr e op nus têm uma Equation no vanilla e zero no Tekt;
math.bold([x]) e texto comum têm zero nos dois. nested e nested-equation têm
uma nos dois; reuse tem duas no vanilla. block-style/local-block/number-style
resolvem o escopo de inserção. show-generated e context-generated expõem as
mesmas lacunas. additional: Equation explícita preserva o render da fração;
it.body é rejeitado no vanilla por maximum show rule depth exceeded, enquanto
it é válido; o show posterior afeta apenas a segunda inserção. refuters mede
valor frac cru e label original versus Equation explícita.

lab/typst-original/crates/typst-realize/src/lib.rs:245–265,302–334 transforma
Mathy antes de show fora de math. 01_core/src/compiler/layout/equation.rs:680–689
projeta math nu em texto; compiler/realize/mod.rs:155–199 prepara show/identidade
sem normalização math. entities/content/traversal.rs:14–108 é bottom-up e
recursa no corpo matemático; rules/apply.rs:564–607 trata apenas localizáveis
já existentes. Estes observáveis são semântica/morfologia/render de linguagem,
não requisito de vtable, buffers, igualdade Rust ou bytes PDF.

### Responsabilidade exclusiva

Cobrir produção paginada/query com World real: fração nua versus Equation
explícita, mesma geometria visível e Formula, frac/mat/root, estilo local block,
numbering/alt, show textual/it/gerado, contexto e contexto com show capturado,
alias repetido e controles sem matemática. Expectativas de linguagem/geometria
vêm de additional/refuters, não de output candidato ou bytes PDF. Não ajustar
fontes/tolerâncias/snapshots para aceitar regressão. O texto PDF sozinho não
comprova fração desenhada: comparar posições dos números e caixa do conteúdo.

Separar negativos independentes da morfologia de mat (D07), label de Mathy
original, export HTML amplo e catálogo math não medido. Nunca chamá-los de match
nem usá-los para ocultar falhas do escopo certificado. Reexecutar regressões D05
(29 controles) e query compilada (36), builds, suítes L1/L3 e lint com proveniência.

### Gate e aceitação

Somente materializar após confirmação humana do pacote math-bare-2026-10-06.md.
RED→GREEN e revalidação bilateral do escopo são obrigatórios. Esta cláusula
supersede restrições de fase anteriores apenas nas fronteiras aqui nomeadas,
sem ratificar paridade universal, D07, math label ou outros débitos.

## Imports e documentação de CI — 2026-10-13

### Medição anterior à decisão

`03_infra/src/integration_tests.rs` nas linhas 32 contém os avisos selecionados. Fontes/owners, spans, saída original do compilador, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, status e diffstat estão em `lab/results/construction/diagnostics/infra-imports-docs-ci-2026-10-13/before.json.gz`. A pesquisa distingue referências produtivas, imports necessários somente à suíte descendente e menções em comentários; não constitui nova medição de paridade.

### Decisão e aceitação

Remover somente os imports privados comprovadamente sem uso neste consumer; manter os reexports públicos e os imports exigidos por chamadas ou contratos. A lista exata de spans anterior à edição está no diagnóstico citado.

Manutenção interna sem mudança de assinatura pública, default, fase, fórmulas ou critérios de aceitação (fluxo contínuo ADR-0127). Não introduzir supressões de lint. Revalidar compilação de todos os targets, nomes e execução da suíte de infraestrutura, build, Clippy do workspace, Rustdoc e linhagem com resselo restrito. Não criar testes que apenas espelhem remoção de imports ou formatação de comentários.

## Fixtures ativos e resíduos de CI — 2026-10-13

### Medição anterior à decisão

`03_infra/src/integration_tests.rs:226,237,3570` exemplifica chamadas `let state = introspect(content)` cujo resultado não é lido. `01_core/src/compiler/introspect.rs:135` delega ao mesmo introspect_with_introspector chamado internamente por `01_core/src/compiler/layout/mod.rs:1703` em layout(content). Os spans do compilador no snapshot identificam exatamente as 63 ocorrências redundantes; as outras chamadas com resultado usado permanecem.

Fontes, owners, diagnósticos, HEAD, working tree não commitado, UTC, status e diffstat estão em `lab/results/construction/diagnostics/fixtures-ativos-ci-2026-10-13/before.json.gz`. O registro real da suíte antes da edição confirma ausência do teste matemático. Estas evidências não constituem nova comparação com upstream.

### Decisão e aceitação

Remover apenas as chamadas redundantes selecionadas pelo compilador e verificadas antes de layout(content), sem substituir por variáveis com underscore. Layout conserva a passagem interna de introspecção. Preservar todas as asserções, fixtures, calls cujo resultado é usado e nomes. Não alterar o contrato produtivo ou alegar medição de desempenho por remover chamadas.

Manutenção interna de testes (ADR-0127), sem assinatura pública, default ou fase nova. L0 precede código; resselo limitado aos owners envolvidos. Executar suítes shell e infra, compilar todos os targets, verificar formato, linhagem e inventário de Clippy. Contagens de testes não certificam paridade.

### Refinamento da busca por resultados descartados

A busca direta no estado intermediário encontrou `let _state = introspect(content)` nas linhas [831, 1708]; no snapshot inicial, linhas [863, 1749]. Ambas antecedem layout(content), não são lidas e escapam ao aviso pelo underscore. Fonte intermediária e UTC estão em `fixtures-ativos-ci-2026-10-13/refinement-before.json.gz`. Não são consumidores úteis a preservar. Remover também essas duas chamadas e o import introspect que fica sem uso; manter introspect_with_introspector, que tem consumidor real no helper. O total removido passa a 65 chamadas redundantes, sem asserções alteradas. A primeira validação está preservada em initial-validation e a aceitação final será repetida no estado refinado.


## Expressões equivalentes e CI estrito — 2026-10-14

### Medição anterior à decisão

`03_infra/src/integration_tests.rs` nas linhas [1503, 1532, 1554, 1558] contém as expressões apontadas pelo inventário
Clippy. `lab/results/construction/diagnostics/ci-simplificacoes-2026-10-14/before.json.gz`
conserva fonte/owner, spans, HEAD, working tree não commitado, UTC e diffstat;
red.log registra os lints selecionados como erros antes de alterar código.
Os tipos e usos foram conferidos no consumer e no toolchain 1.92.0 pinado.
Esta é manutenção da expressão Rust; não é medição nova de paridade upstream.

### Responsabilidade e aceitação

Nos quatro writes de bytes de fixture JPEG, fornecer o array de bytes diretamente ao AsRef<[u8]> de std::fs::write. Bytes, paths, sequência de I/O, expectativas e o caráter mínimo do fixture ficam iguais; não converter o fixture em nova certificação de decodificação JPEG.

Correção interna em fluxo contínuo ADR-0127, sem API pública, default, fase,
unsafe, dependência ou supressão de lint novos. L0 precede código e resselo
fica restrito ao owner. Usar os controles funcionais existentes; não alterar
fixtures, expectativas, tolerâncias ou registrar testes que apenas espelhem
esta simplificação. Os lints selecionados devem passar como erros, seguidos
de suíte completa, build, all-targets, rustdoc, fmt, linhagem e inventário total.


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

A suíte completa em workspace.log falhou na fixture que exigia fallback Color sem size (linha 1686). Migrar o cenário válido para Color com size explícito, exigir ausência de erros e PatternType 1/scn além do retângulo e cor vermelha dentro da célula. Preservar o cenário original sem size como teste de diagnóstico causal exigido pelo preparador. Não aceitar saída inválida por simples ausência de cor nem remover cobertura de integração.

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


## Expressões locais verificadas — Clippy 2026-10-16

### Medição anterior à decisão

03_infra/src/integration_tests.rs:43 (clippy::needless_lifetimes); 03_infra/src/integration_tests.rs:1270 (clippy::collapsible_else_if); 03_infra/src/integration_tests.rs:4803 (clippy::redundant_guards); 03_infra/src/integration_tests.rs:4808 (clippy::redundant_guards); 03_infra/src/integration_tests.rs:5832 (clippy::useless_vec); 03_infra/src/integration_tests.rs:5977 (clippy::useless_vec). O snapshot `lab/results/construction/diagnostics/clippy-expressions-2026-10-16/before.json.gz` conserva fontes/L0, HEAD, working tree não commitado, status, diffstat e UTC; initial-diagnostics.json conserva sugestões e spans. Os usos e tipos foram inspecionados antes da decisão. São representações Rust internas; nenhuma nova paridade upstream ou ganho de performance é alegado.

### Responsabilidade exclusiva e aceitação

Elidir o lifetime único de project_item mantendo o mesmo empréstimo de entrada/retorno e projeções geométricas. Nos controles de Glyph, incorporar x/𝑥 e y/𝑦 ao pattern em vez de guards. Usar arrays RGBA .repeat(4) nos mesmos fixtures de PNG, preservando bytes/tamanhos e expectativas. A correção do observador parse_pdf_objects é especificada abaixo, em vez de apenas achatar seu else.

### Observador PDF — correção funcional interna dos testes

Medição: integration_tests.rs:1273 reconhece qualquer `<` como hex antes de :1275 testar `<<`; o incremento de depth desse segundo ramo é inalcançável. :1285-1286 insere o corpo até i-2 mesmo se o loop terminou por EOF com depth positivo. Isso pode omitir campos após dicionário aninhado e aceitar observação truncada. É defeito do observador local, não evidência de defeito no PDF produzido.

O helper continua privado e limitado a objetos iniciados por `id 0 obj <<`. Reconhecer o delimitador `<<` antes do `<` de hex, manter depth simétrico com `>>`, proteger os mesmos literais e escapes e exigir fechamento completo antes de inserir o corpo. Truncamento deve falhar explicitamente, nunca conceder aceitação a corpo parcial. Não criar um parser PDF geral nem alterar observação de streams, catálogo, outlines ou expectativas produtivas. Duas regressões independentes: corpo conhecido com dicionários aninhados, literal com `>>`/escape e campos após o filho deve permanecer inteiro; objeto sem fechamento deve falhar. Controles plano e objetos subsequentes devem permanecer observáveis. Confirmar ambas RED no helper recebido antes de implementá-las, depois GREEN e revalidar as regressões reais de outlines.

Fluxo contínuo ADR-0127: sem API pública, default de produto, fase, dependência ou supressão novos. L0 precede código e resselo restrito. Preservar inputs e resultados dos controles existentes, sem relaxar tolerâncias. Gate dos lints selecionados RED→GREEN; executar suíte workspace, build, Clippy all-targets, formato, Rustdoc e linhagem. Não criar testes que apenas espelhem a sintaxe Rust.


## FrameItem — exaustividade de decisões existentes, V16, 2026-10-16

### Medição anterior à decisão

F058: 03_infra/src/integration_tests.rs:88 (walk); F059: 03_infra/src/integration_tests.rs:3884 (texto_do_doc); F060: 03_infra/src/integration_tests.rs:4824 (p1132b_stack_heading_e_stack_seguinte_paridade_sec43); F061: 03_infra/src/integration_tests.rs:5263 (p1132o_sec18_fracao_aninhada_suprime_spacing_em_script); F062: 03_infra/src/integration_tests.rs:5410 (items_de); F063: 03_infra/src/integration_tests.rs:5490 (items_de); F064: 03_infra/src/integration_tests.rs:5541 (p975_pipeline_preserva_ic_no_posicionamento). Os spans AST, patterns/guards/corpos e complements foram conferidos antes do código. `lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz` conserva HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, fontes/L0/configuração, hashes, status e git diff HEAD --stat. `classification.json` registra os 70 casos e seus complements em 26 owners, usando a árvore do parser pinado do CI. Cláusulas pertinentes do L0 foram lidas junto da fonte; não há proposta de nova arquitetura ou interpretação de intenção upstream. Padrões e exaustividade Rust são mecânica; semântica/morfologia, geometria, efeitos e diagnósticos do produto devem permanecer.

### Prescrição exclusiva desta unidade

- **F058 — `walk`**: Trocar somente o padrão '_' pelo grupo nominal `TextSeparator`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

- **F059 — `texto_do_doc`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `TextSeparator`, `Semantic`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

- **F060 — `p1132b_stack_heading_e_stack_seguinte_paridade_sec43`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link`. Usar o mesmo prefixo Rust em escopo (`typst_core::entities::layout_types::FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem. Guards ou campos restritos não cobrem a variante integralmente: manter no fallback os casos que falham na condição anterior. Medição específica: guard; guard; guard; Glyph: base_char: 'x' | '𝑥'; Glyph: base_char: 'y' | '𝑦'.

- **F061 — `p1132o_sec18_fracao_aninhada_suprime_spacing_em_script`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link`. Usar o mesmo prefixo Rust em escopo (`typst_core::entities::layout_types::FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem. Guards ou campos restritos não cobrem a variante integralmente: manter no fallback os casos que falham na condição anterior. Medição específica: Glyph: base_char: '+'; guard; guard.

- **F062 — `items_de`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `TextSeparator`, `Semantic`, `Line`, `Image`, `Shape`, `Group`, `Link`. Usar o mesmo prefixo Rust em escopo (`FI`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

- **F063 — `items_de`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `TextSeparator`, `Semantic`, `Line`, `Image`, `Shape`, `Group`, `Link`. Usar o mesmo prefixo Rust em escopo (`FI`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

- **F064 — `p975_pipeline_preserva_ic_no_posicionamento`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `TextSeparator`, `Semantic`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`, `Group`, `Link`. Usar o mesmo prefixo Rust em escopo (`FI`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem. Guards ou campos restritos não cobrem a variante integralmente: manter no fallback os casos que falham na condição anterior. Medição específica: guard; guard.

Manter callbacks, efeitos, metadados, clipping, fontes, positions, tinta e largura conforme os owners vigentes. Esta rodada não amplia travessias, não transforma wrappers em folhas nem usa text/ActualText como substituto da geometria. Helpers de teste mantêm fixtures, esperados, tolerâncias, contagens e falha Unknown; não aproveitar o saneamento para aceitar ausência de observação. Grupos nominais compartilham o mesmo corpo anterior: notas V19 de alternativas continuam visíveis, sem confundir cobertura de um braço com todas as alternativas.

### Gate e aceitação

Manutenção interna ADR-0127 em fluxo contínuo: nenhum enum/campo novo, método de trait/assinatura pública, default, dependência, compatibilidade ou fase nova. Não materializar propostas históricas pendentes por este adendo. L0 primeiro e resselo restrito em ambas as direções. Não adicionar supressões, alterar limiar ou reescrever match como if-let para esconder V16.

Antes do código, executar o controle novo de FrameVisitor e a suíte workspace nos mesmos esperados, e confirmar RED dos 70 avisos da família por leitura AST. Após código, zero V16 dessa família nos locais abrangidos e nenhum aviso novo; manter V16 externos como pendências. Nos matches do core produtivo alcançados por cargo check -p typst-core, usar cópia descartável com uma variante sentinela de FrameItem para medir ausência de E0004 nos locais antes e presença depois. A sonda cobre somente os locais efetivamente compilados: não extrapolar aos helpers cfg(test) ou L3 que a dependência impede de alcançar. Para todo o lote, conferir padrões/guards/corpos por replay, enum e assinaturas inalterados, build e testes reais. Não substituir corpos por stubs.

Revalidar suíte workspace completa, cargo build, Clippy workspace/all-targets com -D warnings e -D unfulfilled_lint_expectations, formato, Rustdoc -D warnings com quatro índices HTML, linhagem V1/V5/V15/V26 estrita/reversa, SARIF e diff --check. Fontes/L0/configuração ficam congelados durante comandos. Execuções pesadas sequenciais com MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1. Não apresentar saneamento estrutural como nova paridade global, ganho de memória ou benefício medido para IA.


## Ordem geométrica no stress de Grid — refinamento 2026-10-18

### Medição antes da decisão

mixed-style-metrics-2026-10-18/initial-workspace.log registra o único RED
do alvo typst-infra --lib em stress_81_5_row_height_e_maximo_da_linha:
41.449999999999996 >= 41.45
falha em integration_tests.rs:2212. O desvio é 7.105427357601002e-15pt;
um epsilon de f64 relativo à magnitude dessas coordenadas mede
9.203748874142547e-15pt. rounding-before conserva fontes/L0/UTC e os
valores exatos, antes desta decisão. Os demais controles de Grid da fixture
passam. O contrato é a ordem geométrica, não identidade de aritmética binária.
Não inferir desse RED isolado um erro de linha, altura ou quebra do produto.

### Responsabilidade e aceitação

Somente a desigualdade de ordem vertical desse teste admite erro numérico
limitado a f64::EPSILON * max(abs(y_linha1), abs(y_max_linha0), 1).
Exigir as duas coordenadas finitas. Comparar a diferença assinada com esse
limite; não arredondar o documento ou alterar a fixture, observador de itens,
posições esperadas, layout, snapshots e os demais testes/tolerâncias.
Isto supersede a proibição genérica de mudar tolerâncias apenas nesse teste:
um erro geométrico maior que esse bound deve continuar falhando. A tolerância
de paridade de 0.001pt não se aplica aqui e não é relaxada por este adendo.
Correção privada de observação ADR-0127/ADR-0107, com o RED preservado,
L0 antes do código, resselo e revalidação ampla no freeze refinado.

## Fixture relocation — 2026-10-18

The first post-migration test compilation failed at the twelve literal
`include_str!` sites in this consumer because the root `.typ` directory had
been retired. The exact failure and frozen source state are retained in
`lab/results/mature-layout-2026-10-18/attempt-1/`.

Use `lab/corpus/legacy-sections/sec_*.typ` as the active location for these ten
versioned Typst inputs. Their bytes must match the retained construction
fixtures; update only the include paths. Preserve every existing test input,
assertion, count, tolerance, and diagnostic. This test-only relocation neither
imports laboratory Rust code into the product nor authorizes behavior changes.
