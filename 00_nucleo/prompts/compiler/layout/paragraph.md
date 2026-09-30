# L0 — paragraph
Hash do Código: 643e2958

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/paragraph-composition.toml sha256:6eec17984449d21b626379651b9b0a511a2dfe8e1a1f0a0af230cce52a47b827
- 00_nucleo/prompts/_nuclei/paragraph-selection.toml sha256:e9038b453bbb1971c9a75a76eced99c35db742f57bebbb09b03c4922b3862800

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/paragraph.rs`


## Composição de parágrafos — 2026-09-28

### Medição anterior à decisão

O lote before-pdf em correcao-composicao-paragrafos-2026-09-28 mede 34 entradas contra o pin 9f2b6e8715237cb086899a42873660fe744622e8, no estado initial.json e before/. Com 180pt e margem 8pt, a primeira linha justificada termina em 145.52pt no Tekt e 172.000002pt no vanilla; a última fica em 110.048pt em ambos. BEFORE/par[MIDDLE]/AFTER produz 2 linhas físicas contra 3. Empty-start põe AFTER em yMin 3.74 contra 18.14; empty-auto mede altura 24.568 contra 38.968. native-leading-two-lines demonstra que o escopo de leading precisa fechar antes de restaurar estilo. Fonte: line_commit.rs:195-236 já possui justificação; word_layout.rs usa flush ordinário em quebras; layout/mod.rs:1469 trata Par como transparente; style_scope.rs:18-77 restaura estilo depois do body; eval/rules.rs:663-705 agrupa fluxo para show. Agrupamentos XML do extrator não são linhas físicas: coordenadas coincidentes são medidas separadamente.

### Decisão vigente

Correção de paridade ADR-0127, em fluxo contínuo: sem nova assinatura pública, entidade, trait, flag ou fase. O núcleo pinado define invariantes compartilhados, não é owner de código. Preparação global e optimized ficam incompletos e pertencem à futura rodada-paragrafo-seletores, depois de medir escolhas/custos.

Unidade dona da realização paginada de Content::Par. Fornecer is_paragraph para reconhecimento de Par através de Styled apenas. layout fecha o fluxo anterior com parbreak, renderiza o body e fecha sua última linha material com parbreak, armando terminal_paragraph_line após esse fechamento. Parbreak repetido sem material colapsa, não cria linha vazia fictícia. Para body vazio (Empty, Text vazio, spaces, Sequence/Styled recursivos), preservar avanço de par.spacing: no início usar cursor + spacing; depois de texto usar prev_line_baseline + spacing. Acrescentar spacing ao orçamento de bloco pendente e limpar a evidência de linha terminal. O caso de Par aninhado fica fora da aceitação desta correção: o stderr vanilla e inline/collect.rs:247 mostram rejeição explícita com warning; o comportamento atual preserva B, mas não constitui paridade aceita e exige uma correção própria de admissão/diagnóstico. Todo cálculo é puro, campos privados existentes mais a evidência do hub, nenhuma entidade ou interface pública nova.

### Medição adicional: parágrafos vazios e altura automática

`00_nucleo/diagnosticos/correcao-composicao-paragrafos-2026-09-28/empty-details/results.json` registra pin, binários, HEAD, working tree e hora. O vanilla distingue corpo vazio de Text(""): sozinho mede altura 16pt contra 24.568pt, com margens 8pt; antes de AFTER, Text vazio acrescenta o top-edge 8.568pt à posição de par[]; após BEFORE a altura automática é 38.968pt para vazio e 47.536pt para Text vazio. O teste composition_empty_styled_terminal_auto_height_keeps_explicit_spacing falhou no estado após a primeira implementação (styled-empty-red.log): fechar antes do estilo perdeu a evidência causal anterior.

Decisão interna: o fechamento anterior a Styled/Par preserva terminal_paragraph_line quando houve material. Body vazio mantém spacing entre fluxos; Text vazio, atravessando Styled/Sequence, também reserva seu top-edge, sem tinta. Acumular em campo privado empty_paragraph_weak_spacing os espaçamentos iniciais enquanto initial_baseline_pending; no término inteiramente vazio, sem bloco material, remover apenas esse spacing fraco, preservando o top-edge virtual. Incluir o campo no checkpoint/rollback. Não reduzir spacing depois de material nem de vazios consecutivos entre fluxos. Nenhuma interface pública muda.

## Preparação e seletores — 2026-09-28

### Medição anterior à decisão

O lote bilateral `00_nucleo/diagnosticos/correcao-seletores-paragrafo-2026-09-28/before-pdf/resultados.json` mede 32 entradas, com estado exato em initial.json e before/, contra vanilla 9f2b6e8715237cb086899a42873660fe744622e8. Em 140pt/margem 8pt, simple escolhe delta epsilon zeta eta, optimized escolhe delta epsilon zeta e junta eta à linha seguinte; Tekt ignora a estratégia. Em 240pt justificado, optimized admite eta na primeira linha reduzindo os espaços. Fonte: eval/rules.rs:3070-3100 e stdlib/structural/par.rs:131-150 validam e descartam linebreaks; text.rs:41 e word_layout.rs:29 escolhem incrementalmente; vanilla inline/mod.rs:194-196 resolve auto pela justificação, linebreak.rs:raw_ratio/raw_cost aplica redução até 1/3 e custos cúbicos/quadráticos; model/par.rs:Limits::SPACING_DEFAULT fixa 2/3..1.5. Estilos adjacentes, literal leading/trailing e conteúdo opaco estão medidos separadamente. Observação não prova intenção histórica; inferência refutável: os scope-outs L0 explícitos de preparação/optimized legitimaram o adiamento, pois ainda vedam a implementação global apesar do transporte aceito.

### Decisão e escopo

Correção interna de paridade ADR-0127: sem novos contratos públicos, flags ou fase. Primeira preparação global certificada: parágrafo latino composto por Text, Space, SpaceFromNewline, Empty, Sequence, Styled tipográfico e Strong/Emph, com hifenização desligada, direção LTR, sem tracking/math/smallcaps, largura finita e constante, fora de colunas/células/floats. O conjunto completo deve ser certificado antes de emitir; uma unidade recusada preserva o caminho anterior. Hifenização, Unicode UAX14 completo, links/objetos/linebreaks, largura variável, limites customizados e seleção incremental restante continuam explicitamente incompletos; este lote não declara estado da arte nem otimização completa da língua.

Depois de fechar anterior e antes de render body, tentar paragraph_plan no body inteiro; fallback anterior se recusado. Empty e fechamento terminal permanecem.

### Refinamento da fixture

strategy-red.log refuta o descarte inicial de estratégia, mas strategy-green.log mostrou que aquela fixture FixedMetrics também possui o mesmo ótimo depois da implementação; ela não distingue um seletor correto do greedy. A fixture vigente usa comprimentos [1,5,2,7,5,2,1,7,9,9,6,6,1,3,3] em 124pt, em que os ranges diferem pelo custo global. A prova bilateral do lote com Noto Sans é independente desse refinamento.

O teste de guard usa prefixo já confirmado/fechado, seguindo FlowSuffixCheckpoint: checkpoint por comprimento não restaura items do current_line que tenham sido drenados pelo próprio teste. core-first.log mostrou a inadequação da primeira fixture (prefixo aberto que a tentativa fazia flush), não uma autorização para clonar buffers ou redefinir rollback.

## Hifenização na origem — 2026-09-28

Medição: break_selection.rs:18 exige cursor além da origem para cortar; before-pdf da rodada correcao-hifenizacao-global-2026-09-28 difere do vanilla; inline/linebreak.rs:170-209 enumera os cortes desde a origem. Testar através de Par que a política true/simple corta uma palavra overlong, preserva todos os caracteres e produz baselines distintas. Implementação fica nos owners de preparo, seleção e emissão; Par só coordena sua fronteira já vigente.

## Identidade textual — materialização aprovada

### Medição anterior à decisão

As sondas bilaterais e o estado exato em
`00_nucleo/diagnosticos/identidade-hifen-discricionario-2026-09-28/initial.json`
e `before/` medem U+002D versus SHY com geometria coincidente em oito controles.
A inspeção do percurso FrameItem deste consumer está arquivada em approved-before/.
O dono confirmou o contrato em `00_nucleo/prompts/entities/layout_types.md`.

### Responsabilidade exclusiva

Tratar TextReplacement como envelope visual transparente nos percursos deste owner; descender nos filhos preservando text, ordem, coordenadas e o cálculo existente, sem novo algoritmo ou avanço.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.

## Text vazio dentro de Par — medição e contrato (2026-09-29)

A suíte `00_nucleo/diagnosticos/correcao-auto-proximas-2026-09-29/core-tests.log` identificou três testes existentes de Par vazio com altura duplicada após `text.rs` preservar Text vazio como unidade de linha. `paragraph.rs:28-34,52-73` já classifica body vazio e calcula seu `virtual_top` tipográfico para Par; `:47-50` ainda renderiza o mesmo Text vazio, repetindo a causa. O corpus bilateral `before/results.json` distingue `#text("")` isolado de `#text("")` dentro de Par. Para body classificado vazio, conservar o caminho virtual de Par sem emitir de novo o body; conteúdo não vazio continua pelo planeador/fallback. Isto preserva altura de parágrafos vazios com spacing explícito e sem spacing, e não muda o contrato de Text fora de Par. Correção interna ADR-0127, teste RED→GREEN e resselo do owner.

### Refinamento: espaçamento explícito após linha causal

`00_nucleo/diagnosticos/correcao-auto-proximas-2026-09-29/green-paragraph.log` preserva o RED de dois testes existentes: `Content::text("A")` seguido de `Content::par(Content::Empty)` deve fechar em 22.8pt, mas o fundo da linha confirmado por Parbreak faz `page_geometry.rs` ignorar o cursor do Par vazio. Quando já há fundo causal confirmado, o Par vazio explícito confirma também seu avanço de spacing/virtual top como novo fundo, sem criar tinta. Se não há fundo causal, manter o cancelamento de weak spacing de um documento apenas vazio.
