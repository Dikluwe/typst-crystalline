# L0 — paragraph
Hash do Código: f4f8a8d5

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-segment-scope.toml sha256:77528cd6193277b9f776b29da06327d974f5cea26fb89abf821879b363185b0a
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8
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

## Direção de parágrafo distinta de direção de run — 2026-10-05

Medição anterior à decisão: `00_nucleo/diagnosticos/direcao-alinhamento-2026-10-05/initial.json.gz` preserva HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree, UTC, fontes, binários e treze sondas. rtl_lines põe ABC em x=20 no Tekt e x=152.949220 no vanilla pinado; rtl_inline_only e rtl_scope coincidem à esquerda. ltr_lines põe árabe em x=174.567870 no Tekt contra x=20 no vanilla. Em `cursor.rs:156–162` e `line_commit.rs:210–216`, a âncora depende do primeiro item da linha. Vanilla `flow/collect.rs:129–136` obtém trunk_from_pairs e `inline/mod.rs:189,232` resolve direção/alinhamento compartilhados antes das linhas. Semântica observável: direção dos trechos e âncora de todas as linhas do parágrafo. Estratégia de state/preflight é mecânica interna, não cópia do vanilla. Inferência: preservar direção compartilhada evita perder a âncora em linhas só com override; refutada por controles integralmente estilizados/parbreak/wrap.

Correção interna/de paridade ADR-0127, fluxo contínuo L0 primeiro e RED→GREEN. Apenas campos privados de Layouter e helpers descendentes, nenhuma entidade ou assinatura pública/fase/default novo. Esta rodada certifica conteúdo textual estrutural (Text, Empty, Space/SpaceFromNewline, Linebreak, Sequence, Styled e Strong/Emph); unidades desconhecidas conservam fallback anterior. Não prometer ordem bidi mista, align explícito, HTML ou todas as formas de parágrafo. As equações existentes de largura/tracking permanecem.

Responsabilidade deste owner: Possuir resolução pura da direção compartilhada do segmento textual certificado. Single substantive Styled/Strong/Emph/Sequence pode fornecer o estilo integral; múltiplos siblings independentes usam a direção exterior, nunca consenso de valores de runs. Certificar antes de emitir; desconhecido retorna None. Par explícito usa sua cadeia exterior, não o primeiro filho. Escopo ativo conserva direção entre wraps; na saída guarda direção para a cauda ainda aberta e restaura o escopo pai. Consulta para alinhamento usa ativa, depois cauda, depois fallback histórico do primeiro Text quando nenhum contexto certificado existir. Preservar direção de cada TextStyle.

Aceitação: RED→GREEN em parágrafo RTL com linha LTR e inverso, escopo integral e reset por parbreak; checar checkpoints e subframes, texto/posições em CLI bilateral, build e lint sem novos findings. Slots e helper não transportam metadados ao L3; divergências de ordenação bidi ou align explícito ficam registradas separadamente.

Refinamento por sonda causal: tree-state.json.gz registra fonte real `#set text(dir: rtl)` como Styled de fluxo com Sequence iniciada por Space de newline e depois texto/linebreak/override. first-candidate.json.gz refutou o tratamento desse Space vazio inicial como produtor que impede o escopo integral, deslocando até a primeira linha RTL. Para resolução do estilo compartilhado, Empty e os carriers fracos Space/SpaceFromNewline não são produtores substantivos; continuam certificados e emitidos normalmente. Texto literal whitespace não é descartado. O preflight não muda o conteúdo, as larguras ou a ordem de emissão. Aceitação inclui fonte real com #set e newline, não só construção manual do enum.

## Migração do carrier bidi ratificado (2026-10-05)

Medição anterior à decisão: o inventário `00_nucleo/diagnosticos/bidi-contexto-2026-10-05/inventory.json` identifica as referências desta unidade e os owners 1:1. O contrato do manifesto 1ae7513e foi confirmado; nesta unidade ajustar constructors não certificados com None e patterns de leitura com `..`, preservando os observáveis de linguagem existentes. Transformações obedecem ao núcleo pinado de origem; sem ampliar fase/API.

## Transporte lógico textual — contrato ratificado em 2026-10-05

Medição anterior à decisão: `00_nucleo/diagnosticos/bidi-contexto-2026-10-05/probe.json.gz` refuta resolver 456 789 isoladamente; a medição de fonte e os locais afetados estão no inventário e initial.json.gz daquele diagnóstico. A API imutável e os três campos opcionais foram ratificados pelo humano (manifesto 1ae7513e). Correção interna de paridade sem mudar fase.

Esta unidade participa do transporte por Cursor privado, input Arc e spans exatos. Preparar antes de emitir o parágrafo textual inteiro, base lexical já certificada; conservar offsets por ordem de visita, não matching de strings. Sequence/Par criam escopo, nested Sequence herda; checkpoint conserva cursor e span corrente; subframe isola e restaura. Text fornece subspans para palavras e espaços literais; Space consome seu segmento mesmo quando fraco; Linebreak consome fronteira. Word emite span somente se texto exato não foi transformado.

Escopo inicial: incremental simple, fora de células/colunas/subframes, sem tracking/hifenização/justificação/smallcaps, Text sem newline/tab/segmentação linguística. Região recusada tem None. Preparação global anterior permanece para parágrafos que não precisam do contexto bidi; para o subconjunto certificado com base RTL ou caracteres RTL/override oposto, encaminhar pelo incremental sem mudar os breaks permitidos. Não interferir em optimized explicitamente escolhido. O owner privado bidi_origin prepara/valida; esta unidade só coordena o seu papel. Testes de origem por ocorrências, UTF8, restauração e parbreak; validação bilateral de wraps.

Guard privado bidi_scope_active distingue ausência de cursor certificado de ausência de escopo; após recusa integral, filhos não certificam um subconjunto parcial. Checkpoint e subframe conservam/suspendem/restauram este guard.

## Envelopes de identidade no pipeline completo

Medição anterior à decisão: `00_nucleo/diagnosticos/bidi-contexto-implementacao-2026-10-05/verification.json.gz` preserva o primeiro candidato; RTL mixed/LTR numeric ainda coincidem com baseline errado. `pipeline-context.log` registra RED pelo pipeline completo: todas as folhas sem span; `realize/mod.rs:38–44` insere Located em ocorrências. Avaliação direta não exercitava esse envelope. Correção interna: Located é transparente somente para certificação da sua body textual integral, sem ignorar corpos opacos nem alterar registro de Location. Direção lexical e coleta devem atravessar o mesmo envelope. O teste do pipeline exige spans em todas as folhas textuais e ordem RTL com primeiro trecho latino; preservar artefatos do primeiro candidato.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: 00_nucleo/diagnosticos/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em materialization/table-origens-d08b3c2-2026-10-07/contract.md.

Certificação do contexto de parágrafo e direção compartilhada atravessam Source sem mudar âncora/direção nem criar fragmento independente; refutadores workspace-second exigem mesmos limites bidi e leading.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.


## Escopo textual e cauda fracionária — 2026-10-14

### Medição anterior à decisão

`00_nucleo/diagnosticos/transformacoes-2026-10-14/scope-tail-before/results.json.gz`
registra HEAD06848cdcb5450f2c555626bf116bc7acd82bd82b, árvore não commitada,
UTC/status/diffstat e hashes de fontes/binários contra vanilla pinado9f2b6e871.
Os lotes block-par-spacing-after e block-par-extra-after anteriores já
refutam escopo por consenso de runs: um wrapper18 em Lead More exige margem
21.6pt, enquanto dois wrappers18 adjacentes mantêm o escopo exterior.
sequence.rs:196–245 fecha prefixos na chain exterior; paragraph.rs:12–90 já
resolve direção pelo ancestral lexical comum. Fonte vanilla flow/collect.rs:
129–145 resolve estilos comuns antes de compor linhas e espaçamento.
A cauda TAIL top10/bottom0 após Block1fr ocupa página2 contra página1 no pin:
cursor.rs:127–135 rejeita igualdade com o limite embora a cauda esteja
certificada. Cauda Styled18 fica10.52pt acima; fractional_preflight.rs:170–211
mede nós individualmente e só corrige a altura de Text direto. Linebreak
explícito também excede a região. A nova sonda inclui wraps, fonte8/18,
Strong, wrappers irmãos, Align e colunas. Isso é geometria e paginação de
linguagem; não exige estrutura ou bytes iguais. before-edit.json.gz em
scope-tail conserva os consumers/owners anteriores.

### Classificação

Correção interna de paridade ADR-0127: L0 primeiro, RED→GREEN e revalidação.
Sem API pública, nova entidade, fase eval/layout ou default novo. Refutar
por wrappers irmãos, preservação da ordem, última linha, corpos opacos,
colunas e ausência de efeitos/estado vazando da medição. Não copiar defeito
upstream nem declarar paginação universal para cauda maior que a região.

### Responsabilidade exclusiva

Extrair a certificação textual e ancestral lexical comum já usados pela
direção para helpers privados. Source/Located são transparentes; Styled
aplica a chain somente se cobre todo material do segmento. Strong/Emph não
fabricam novo escopo de par.spacing. Não deduzir escopo por valores de run.
Oferecer fechamento do segmento textual antes de fronteira Block/Align,
instalando temporariamente a chain/style comum para Parbreak e restaurando
em seguida. Nada se fecha para segmento vazio/opaco ou sem linha causal.
Oferecer medição geométrica do segmento textual certificado em Layouter
isolado com métricas emprestadas e Introspector vazio: somente nós textuais,
sem callbacks, Location, consulta externa ou mutação do emissor. Retornar
altura causal e margem resolvida na chain comum. Medição inclui wrap,
Linebreak e estilos; não usa soma de alturas de folhas nem tinta como frame.


### Refutadores de extensão e primeira região

scope-tail-after-first/results.json.gz confirma26/28 documentos. Os dois
restantes são caudas com cinco linhas que excedem a primeira região; reservar
altura integral deixa o início32/17pt acima. O primeiro fragmento deve ser
medido sob a capacidade disponível, usando a mesma admissão de viúvas/órfãs,
e somente sua altura causal entra na reserva anterior ao1fr. Próximas regiões
preservam sua capacidade normal; não somar margem terminal de parágrafo.

scope-tail-refutators/results.json.gz mede os três controles antigos que
exigiam caudas desalinhadas. Três fragmentos numa linha agora coincidem
integralmente. Multiword21 e par40/8 com fill ainda divergem em entrelinha:
a medição isolada reproduz essa geometria defeituosa, não a certifica.
O novo fallback de prefixo geométrico exige uma única baseline textual;
prefixos multiline conservam os certificados escalares anteriores ou a rota
opaca. Caudas multiline continuam sob medição integral/primeiro fragmento.
Não alterar as asserções opacas desses dois prefixos para declará-los verdes.
O aninhamento parcial OUTER18 + Styled8 contendo Block também continua
incorreto no pin; um segmento local que entrou com linha exterior aberta
não prova o escopo do parágrafo inteiro e não recebe novo fechamento.

A medição da primeira região usa Layouter isolado, região inicial finita,
margens zero e métricas/chain emprestadas. Nós são certificados textuais,
e somente a altura causal do primeiro Page é lida por baseline + bottom-edge
lógico (incluindo carriers e envelopes semânticos), nunca bbox de tinta.
Unidade de frame não reconhecida recusa a medição, não vale zero. Não altera
política de paginação nem reexecuta callbacks no emissor.


### Separador não é linha causal

scope-tail-layout-final-candidate.log mede One37.45 contra43.5 na fixture
unwrapped FixedMetrics, embora todos os28 documentos PDF coincidam no pin.
O certificado inspecionou a baseline do TextSeparator como texto material;
separadores são neutros para altura (núcleo text-separators), e sua posição
não prova uma segunda linha. Ignorá-los tanto na contagem de baselines
quanto no fundo causal do primeiro fragmento. Preservar seus bytes/posição
no emissor. Não alterar os valores esperados de One nem a largura novamente.


### Linha já fechada conserva sua fronteira

scope-tail-green-final.log refuta que somente ignorar TextSeparator resolve
P1383: One ainda37.45 em vez43.5. O prefixo pode fechar sua última linha
antes de chegar ao Block. close_segment recusava current_line vazio e
perdia a margem, enquanto sub_frame devolvia o cursor da próxima baseline
como altura. Prefixo geométrico certificado usa o fundo textual lógico dos
items, não a baseline preparada. Para linha já fechada, com testemunha
causal válida, segmento completo e cadeia ainda ausente, registrar a margem
fraca compartilhada sobre esse fim, sem emitir outra linha nem repetir flush.
Origem inicial, testemunha inválida e cadeia existente não recebem essa rota.
O ajuste de separadores continua correto por sua neutralidade, mas não era
a causa suficiente deste RED. Cauda multiline mantém sua medição integral.


### Exaustividade do certificado de frame

A validação scope-tail/validation/followup-architecture.log registra V16 no
wildcard de text_frame_bottom: rejeição genérica de variante do domínio.
Manter a recusa dos carriers não textuais, enumerando explicitamente Glyph,
Image, Shape, Group, Link e Line. Não atribuir altura zero nem aceitar esses
carriers. Nova variante exige decisão explícita por match exaustivo. É uma
correção interna sem mudança do conjunto atualmente aceito pelo certificado.


## Prefixo multiline e aninhamento parcial — 2026-10-14

### Medição anterior à decisão

`00_nucleo/diagnosticos/transformacoes-2026-10-14/prefix-nesting-before/results.json.gz`
registra HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não
commitado, status/stat/UTC, fontes e binários contra o pin vanilla
9f2b6e8715237cb086899a42873660fe744622e8. Multiword21 gera duas páginas
contra uma; Long/More estão 6.5/13pt abaixo. O prefixo40/8 com fill gera duas
páginas; More está39.906pt abaixo. Nested OUTER18/INNER8 tem BODY12pt acima
e página auto12pt menor. sequence/fractional_preflight.rs:151–171 recusa
multiline; word_layout.rs:49 fecha dentro do estilo local; line_commit.rs:
398–480 usa esse estilo para floor/leading/top previsto. sequence.rs:239
recusa fechar segmento parcial; Block sem cadeia anterior usa spacing8pt.
Vanilla flow/collect.rs:129–145,179–214 resolve trunk dos filhos, spacing e
leading comuns; cada frame contribui sua geometria. Esses são observáveis
textuais, geométricos e de paginação da linguagem (ADR-0107), não obrigação
de copiar buffers, algoritmos ou bytes. A intenção histórica não é inferida.

### Decisão e limite

Correção interna de paridade ADR-0127 em fluxo contínuo: sem entidade,
trait/assinatura pública, nova flag/default ou fase. O núcleo text-segment-scope
pinado define as invariantes compartilhadas. Aceitação RED→GREEN dos três
casos, controles de fontes, cores, margens, colunas, restauração, checkpoint
e subframes, mais suíte completa e linhagem. A nova rota só amplia prefixos
multiline textuais positivamente certificados; não declara paginação universal,
bidi/floats nem conteúdo opaco. Preservar os certificados escalares anteriores.

### Responsabilidade exclusiva

Separar fechamento sob chain explícita do fechamento de segmento integral. Medir prefixo multiline em Layouter isolado com métricas emprestadas, região de largura finita/margens zero e altura não constrangida, modo geométrico e chain comum. Fechar a última linha sob chain comum e obter fundo textual lógico pelos carriers certificados; exigir mais de uma baseline material e altura finita positiva. Não tratar TextSeparator como linha nem altura. Não reavaliar efeitos.

O prefixo multiline novo exige também a certificação positiva de paragraph_prepare::prepare_parts antes de medir. O conjunto é o subconjunto tipográfico desse preparador; Linebreak explícito, overrides não suportados e unidades opacas continuam recusados por esta extensão. A medição não reutiliza a estrutura preparada como output nem muda seleção global.


## FrameItem — exaustividade de decisões existentes, V16, 2026-10-16

### Medição anterior à decisão

F008: 01_core/src/compiler/layout/paragraph.rs:686 (texts). Os spans AST, patterns/guards/corpos e complements foram conferidos antes do código. `00_nucleo/diagnosticos/tekt-frame-items-2026-10-16/before.json.gz` conserva HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, fontes/L0/configuração, hashes, status e git diff HEAD --stat. `classification.json` registra os 70 casos e seus complements em 26 owners, usando a árvore do parser pinado do CI. Cláusulas pertinentes do L0 foram lidas junto da fonte; não há proposta de nova arquitetura ou interpretação de intenção upstream. Padrões e exaustividade Rust são mecânica; semântica/morfologia, geometria, efeitos e diagnósticos do produto devem permanecer.

### Prescrição exclusiva desta unidade

- **F008 — `texts`**: Trocar somente o padrão '_' pelo grupo nominal `TextSeparator`, `Text`, `TextShaped`, `Line`, `Glyph`, `Image`, `Shape`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem. Guards ou campos restritos não cobrem a variante integralmente: manter no fallback os casos que falham na condição anterior. Medição específica: guard.

Manter callbacks, efeitos, metadados, clipping, fontes, positions, tinta e largura conforme os owners vigentes. Esta rodada não amplia travessias, não transforma wrappers em folhas nem usa text/ActualText como substituto da geometria. Helpers de teste mantêm fixtures, esperados, tolerâncias, contagens e falha Unknown; não aproveitar o saneamento para aceitar ausência de observação. Grupos nominais compartilham o mesmo corpo anterior: notas V19 de alternativas continuam visíveis, sem confundir cobertura de um braço com todas as alternativas.

### Gate e aceitação

Manutenção interna ADR-0127 em fluxo contínuo: nenhum enum/campo novo, método de trait/assinatura pública, default, dependência, compatibilidade ou fase nova. Não materializar propostas históricas pendentes por este adendo. L0 primeiro e resselo restrito em ambas as direções. Não adicionar supressões, alterar limiar ou reescrever match como if-let para esconder V16.

Antes do código, executar o controle novo de FrameVisitor e a suíte workspace nos mesmos esperados, e confirmar RED dos 70 avisos da família por leitura AST. Após código, zero V16 dessa família nos locais abrangidos e nenhum aviso novo; manter V16 externos como pendências. Nos matches do core produtivo alcançados por cargo check -p typst-core, usar cópia descartável com uma variante sentinela de FrameItem para medir ausência de E0004 nos locais antes e presença depois. A sonda cobre somente os locais efetivamente compilados: não extrapolar aos helpers cfg(test) ou L3 que a dependência impede de alcançar. Para todo o lote, conferir padrões/guards/corpos por replay, enum e assinaturas inalterados, build e testes reais. Não substituir corpos por stubs.

Revalidar suíte workspace completa, cargo build, Clippy workspace/all-targets com -D warnings e -D unfulfilled_lint_expectations, formato, Rustdoc -D warnings com quatro índices HTML, linhagem V1/V5/V15/V26 estrita/reversa, SARIF e diff --check. Fontes/L0/configuração ficam congelados durante comandos. Execuções pesadas sequenciais com MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1. Não apresentar saneamento estrutural como nova paridade global, ganho de memória ou benefício medido para IA.


## V16 — Source, corpo vazio e fronteira Par (2026-10-17)

### Medição antes da decisão

No HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não
commitado, `01_core/src/compiler/layout/paragraph.rs:340-363` reconhece Par
somente por Styled e classifica corpos vazios/texto vazio sem atravessar Source.
`01_core/src/compiler/layout/style_scope.rs:23-31` precisa desse reconhecimento
para fechar a linha anterior antes do estilo; `paragraph.rs:385-422` decide
avanço vazio/virtual top pelos outros dois predicados. O eval anexa Source a
ContentBlock em `01_core/src/compiler/eval/expression_dispatch.rs:49-55`.

`00_nucleo/diagnosticos/tekt-v16-paragraph-2026-10-17/before.json.gz` e
`before-measurement.json.gz` conservam UTC, HEAD/status/diffstat, hashes de
fontes/L0/configurações/binários/fonte tipográfica e 18 casos bilaterais (36
execuções) contra `9f2b6e8715237cb086899a42873660fe744622e8`. Com DejaVu Sans
12pt, página auto e margem 8pt, empty-before e source-empty põem AFTER em
8.000013pt versus 22.400001pt; empty-after mede altura 25.1172pt versus
39.51719pt; styled-empty põe AFTER em 38.717213pt versus 60.317183pt.
space-par também perde 14.4pt. Text vazio já coincide e precisa preservar seu
top-edge virtual. `lab/typst-original/crates/typst-layout/src/flow/collect.rs:100-101,164-190`
conserva a fronteira e o spacing de Par. A fonte foi comparada ao objeto Git do
pin (reference.json). Isso mede linguagem, não intenção ou equivalência de bytes.

### Decisão e prescrição exclusiva

Correção interna/de paridade, fluxo contínuo ADR-0127. A inferência de que Source
causa as perdas será refutada se a mudança não restaurar altura/espaçamento dos
casos de origem ou alterar controles materiais/opacos. O núcleo source-origin
já exige transparência; a mecânica do match não pertence à paridade.

Nos três predicados is_paragraph/is_empty_body/contains_empty_text, Source
recorre somente ao body. Preservar Styled e Sequence exatamente onde já eram
admitidos; não promover Sequence a Par, Strong/Emph/Located/Realized/ShowRecipe
ou unidade opaca a corpo vazio por esta revisão. Reconhecimento via Source
permite fechar a linha anterior antes de instalar o estilo do Par. Corpo vazio
com Source conserva a fronteira e spacing explícito; Text vazio continua a
reservar seu top-edge uma única vez. Não emitir tinta, reavaliar callbacks,
fabricar Location, mudar a primeira origem ou mover decisões de eval para layout.

A auditoria nominal content-variants.json enumera 120 variantes Content antes
da decisão de representação. Evitar repetir o mesmo complemento extenso nos
três predicados: criar uma projeção privada emprestada ParagraphBody com
Empty, Space, Text(&str), Sequence(&[Content]), Styled(&Content), Par e Opaque.
Um único match Content exaustivo possui a política de admissão: Source recorre
à projeção do body; Space/SpaceFromNewline compartilham Space; as variantes
não admitidas são enumeradas explicitamente em Opaque. Não executar nem
percorrer callbacks de Opaque. Isso é mecânica privada, sem novo contrato.

Os três predicados fazem match exaustivo dessa projeção pequena, preservando
os quantificadores all/any de Sequence e a recursão de Styled. Cada grupo V19
compartilha uma política única: espaços equivalentes na projeção, unidades
opacas no domínio ou recusa do predicado. Manter as notas e a justificativa,
sem braços idênticos separados nem supressão V16. A nota antiga do grupo de
espaços pode ser substituída pelas notas explícitas da nova representação;
nenhuma redução de V19 é critério de sucesso. Variante Content futura exige
revisão na fronteira e variante ParagraphBody futura exige revisão nos três
predicados, por exaustividade Rust. Não reescrever como if-let ou matches
para ocultar os avisos. Demais funções, fórmulas, certificados, corpos opacos
e estados privados deste owner permanecem inalterados.

### Aceitação e pendências

Antes da implementação: testes funcionais RED de geometria/espaçamento e
restauração de estilo com Source e controles de Text vazio/material/objeto;
não contar falha de compilação ou leitura AST como RED funcional. Depois:
GREEN, repetição dos 18 casos com páginas/palavras/caixas e tolerância absoluta
0.002pt para arredondamento PDF (nunca bytes nem normalização de texto).
Nenhum caso previamente coincidente pode regredir. Sonda rustc isolada com os
quatro matches reais, todas as formas Content vigentes e payloads mínimos deve
compilar o controle, rejeitar Content futura com E0004 na projeção e rejeitar
ParagraphBody futura com E0004 nos três predicados.
Essa sonda mede cobertura estrutural, não execução completa do compilador.

Resselo somente deste owner/consumer; build, fmt, Clippy estrito/all-targets,
Rustdoc -D warnings/quatro índices, suíte workspace, V1/V5/V15/V26, linhagem
reversa e diff check. Diferencial SARIF remove exatamente três V16, substitui somente a nota V19
do antigo grupo de espaços pelas seis notas dos grupos prescritos e preserva
os demais diagnósticos. Comandos
pesados sequenciais, cada gate em seu cgroup com MemoryMax=12GiB, swap zero,
MemoryHigh=10GiB e Cargo jobs=1; não atribuir cache agregado a um processo.

O corpus inicial com text(18pt) mediu quatro falhas independentes do constructor
antes de Par. Elas permanecem abertas em positional-size.json.gz e não são
resolvidas usando size nomeado nos controles. Par aninhado, outros envelopes,
HTML e demais limites anteriores não são certificados por esta rodada.


## Refutação do primeiro candidato — margens de Par vazio (2026-10-17)

### Medição anterior à nova decisão

first-candidate/ conserva integralmente código/L0, PDFs e before/after do
primeiro reparo de Source. A checagem falhou: styled-empty ainda perde 7.2pt,
e styled-empty-text, antes coincidente verticalmente, perde os mesmos 7.2pt.
Isso refuta a suficiência da projeção. supplement-before-measurement.json.gz e
refuters-before-measurement.json.gz no diagnóstico desta revisão acrescentam
24 casos bilaterais: tamanho 8/18, terminais, consecutivos, somente vazios,
spacing explícito/zero/negativo e caixa/blocos. Estado exato por coleta em
HEAD/working tree/UTC/hashes/status/diffstat. Com 12pt externo e 18pt interno,
AFTER está em y53.117213 versus60.317183; com8pt, y41.117213 versus45.917187.
Dois vazios8→18 sem tinta medem16pt de página versus37.6pt. O constructor
`01_core/src/compiler/stdlib/structural/par.rs:153-157` descarta spacing.
A fonte pinada `lab/typst-original/crates/typst-library/src/model/par.rs:212-225`
define margens de ambos os lados, colapsadas pelo maior valor. A distribuição
em `lab/typst-original/crates/typst-layout/src/flow/distribute.rs:244-260`
confirma a força das margens. Objetos Git conferidos no pin vigente.

### Fórmula e estado próprios deste owner

Esta revisão supersede somente a proibição de mudar a fórmula do ramo vazio.
Tratar Par vazio como unidade de altura zero (ou virtual_top quando contém
Text vazio), com fim causal próprio. Descartar a margem acima somente no
início vazio do fluxo; havendo unidade anterior, colapsar acima com o pending
anterior por máximo para margens fracas. Margem forte de Block anterior tem
precedência e conserva seu valor, inclusive zero. O fim é a âncora anterior
mais o gap admitido e virtual_top, contado uma vez.

Depois, instalar o fim em cursor_y e prev_line_baseline, zerar descida da
unidade anterior, confirmar last_block_descent_y e testemunha de fluxo;
reservar somente spacing próprio em prev_block_below_pending. A margem abaixo
fica pendente para o consumidor seguinte e colapsa separadamente com seu above.
Conservar block_chain_active/prev_margin_is_parbreak para o protocolo vigente;
initial_baseline_pending torna-se false porque há uma unidade causal de
altura conhecida, mesmo sem tinta. Limpar empty_paragraph_weak_spacing: não
preaplicar a margem terminal, logo não há soma fraca fictícia a remover.
A admissão já existente de Cursor para block-chain acrescenta top-edge real
do próximo texto e resolve seu gap. Não modificar Cursor, Finish, checkpoint,
layout hub nem criar estado/campo público ou privado novo neste pacote.

Conservar a projeção Source e os três predicados já prescritos. Não modificar
fórmulas de corpos materiais, regras de texto vazio fora de Par, callbacks,
fonte, bidi, migração regional, direções, margens fortes ou conteúdo ocultado.
A nova inferência é refutável por qualquer regressão dos controles construíveis
ou perda da geometria própria de dois vazios consecutivos sem tinta.

### Aceitação refinada, sem encobrir diferenças preexistentes

Adicionar RED funcional para tamanho8/18 inicial/intermediário/terminal, dois
vazios, Text vazio virtual e transporte/erro de spacing no constructor. Preservar
os nove testes anteriores e todos os controles. Repetir os18 casos principais,
12 supplement e12 refuters; edges acrescenta6 controles construíveis e2
limitações conhecidas de Block negativo, que permanecem abertas. Tipos de
spacing devem concordar com o pin. Quaisquer falhas fora do reparo continuam
inventariadas, sem crédito de paridade universal.

A tolerância vertical e de xMin continua0.002pt, com texto/páginas exatos.
O xMax do pdftotext já difere até cerca de0.0051pt antes do reparo (AFTER
46.148 versus46.15039; MIDDLE18pt77.966 versus77.960938); não alargar a
tolerância para chamar isso de equivalente. Registrar o resíduo horizontal
preexistente e exigir xMin/xMax inalterados dentro de cada binário entre
coletas. A sua natureza (geometria de glyph ou metadado PDF) continua por
classificar, não é declarada corrigida por este lote de fluxo vertical.
Páginas, palavras e yMin/yMax devem coincidir com o pin nos casos abrangidos;
controles antes coincidentes não podem regredir verticalmente.

Adicionar o segundo par owner/consumer ao resselo e à proveniência congelada.
O diferencial pode substituir somente a nota V19 do antigo grupo que incluía
spacing no constructor pela nota do grupo residual de três propriedades ainda
incompletas. Nenhum V16 extra é admitido; demais gates e teto de RAM mantêm-se.

### Métricas locais e fallback de Text vazio — 2026-10-17

Medição anterior: empty-fonts-before-measurement.json.gz e descents-before-
measurement.json.gz no diagnóstico deste lote medem16 casos adicionais com
estado/UTC/hashes completos. Text vazio local18pt produz altura25.1172pt no
Tekt versus29.675781pt no pin; local8pt exige22.078125pt. top-edge:0pt ou
-5pt exigem altura16pt sem tinta, mas Tekt mantém25.1172pt. Texto seguinte
com top-edge:20pt deve começar42.400003pt em vez31.517213pt. Descida local10
acrescenta10pt ao frame. O booleano contains_empty_text perde o estilo local.

A hipótese inicial de combinar máximos de siblings foi refutada ANTES do
código: mixed-local-edges produz y41.51719, usando a aresta do último Text
vazio, e não a aresta20pt do primeiro. A fonte pinada
lab/typst-original/crates/typst-layout/src/inline/line.rs:311-315 substitui o
fallback a cada Text vazio, e :224-228 o usa quando não há outro texto material;
:559-568 limita as arestas do frame por zero. fallback-before-measurement.json.gz
mede as ordens18→8, aresta20→default e espaços finais separadamente. Isso é
geometria observável; não obriga copiar buffers ou a estrutura do algoritmo.
Não há evidência nesta coleta de defeito dessa política upstream.

Substituir o terceiro predicado privado por empty_text_style, que devolve
Option<StyleChain> da última ocorrência Text vazia do corpo certificado.
ParagraphBody::Styled preserva referência ao StyledContent inteiro; os outros
dois predicados continuam consultando somente seu body. Source permanece
transparente. Sequence procura o último Text vazio em ordem; Styled empurra
sua Styles antes de descer. Empty/Space/Par/Opaque e Text material não oferecem
strut. Não resolver fonte para ramos descartados nem avaliar callbacks.

Na realização de Par vazio, resolver TextStyle da cadeia escolhida e consultar
FontMetrics.text_edges uma vez. Top e descida são não negativos, incluindo
baseline_offset do estilo; altura virtual é sua soma, uma única vez. Sem Text
vazio, altura virtual zero e nenhuma consulta de fontes. O frame virtual não
emite tinta nem novos callbacks. Conservar o protocolo de margens do ramo vazio.

Esta cláusula supersede somente o booleano e a métrica exterior. A sonda futura
da projeção deve rejeitar a variante nova em is_paragraph, is_empty_body e
empty_text_style. Manter os seis grupos V19 previstos, com None na recusa do
terceiro consumidor. RED de fonte8/18, top-edge0/negativo/20, descida10 e ordem
de siblings; o teste prospectivo de máximo foi corrigido pela medição e não
conta como prova funcional. Repetir todos os casos sem reduzir os esperados de
controles estabelecidos. Não certificar campos do constructor de texto ainda
aceitos sem efeito nem geometria universal de wrappers opacos.

## Fronteira Par/Box e região finita — 2026-10-17

### Medição anterior à decisão

`00_nucleo/diagnosticos/par-box-fronteira-2026-10-17/before.json.gz`,
`before-measurement.json.gz`, `before-analysis.json`,
`supplement-before-measurement.json.gz` e `finite-before-measurement.json.gz`
guardam HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não
commitado, status/diffstat, UTC, fontes, argv, hashes de binários e o pin
vanilla 9f2b6e8715237cb086899a42873660fe744622e8.
Em EmptyPar→Box8×7→AFTER, Tekt yMin31.108413 contra22.400001pt;
em BEFORE→EmptyPar→Box→AFTER,54.625613 contra45.917187pt.
Espaçamento30pt inverte o erro:70.225613 contra77.117188pt.
Block8 abaixo0 seguido só de Box tem altura33.1172 contra31pt.
Em alturas finitas60–63pt, AFTER desaparece no Tekt e fica visível na página2
no pin;64–70pt deixam tinta na página1 mas página terminal extra no Tekt.
Box alto45pt na primeira região30pt cria duas páginas contra uma no pin.
boxed.rs:22–42 usa constantes normalizadas e ensure_initial_baseline;
line_commit.rs:501–539 só admite antes do commit em altura reduzida;
region_progress.rs:118–133 pode avançar após tinta já comitada fora da página.
Fonte pinada inline/box.rs:35–68 publica a baseline do frame;
flow/distribute.rs:321–373 admite frames pela geometria real.
São posições, dimensões e páginas da linguagem, não exigência de copiar
buffers/algoritmos Rust ou bytes PDF. A intenção histórica não é inferida.
A hipótese de fronteira geométrica é refutada por controles inline,
margens fortes, fontes, baseline deslocada, assentamento tardio, posições,
primeiro frame alto, subframes ou reservas/rollback alterados.

### Decisão e limites

Correção interna/de paridade ADR-0127: L0 primeiro, RED→GREEN e resselo
restrito, sem API pública, novo modo/default, dependência ou fase.
A composição fracionária de Box permanece incompleta na etapa B anterior:
a testemunha width:1fr tem largura0 contra145.849609pt. Sua rota de
calibração é conservada neste reparo e não recebe crédito de paridade.
A admissão nova certifica somente página raiz finita ordinária, sem colunas,
reservas/floats, transação/checkpoint pendente ou collector externo de
decoração. Esses mecanismos conservam os protocolos próprios, não são
silenciados nem declarados corrigidos. Aceitação inclui matriz bilateral,
controles sem fronteira, finite60–70, fonte maior posterior, primeira caixa
alta e rebase de origem/contexto; suíte completa, build, Clippy, rustdoc,
linhagem bidirecional e inventário SARIF diferencial sob12GiB e swap zero.

### Responsabilidade exclusiva deste owner

Expor ao módulo irmão apenas helper privado spacing_for_size(chain,size):
resolver Length com o tamanho fornecido, ou o fator canônico PAR_SPACING.
spacing(chain) delega usando seu TextStyle lexical; isso preserva seus
consumidores anteriores. O curinga existente trata ausência/tipos distintos
como default e continua visível no inventário, sem silenciamento.


## TS1 — tamanho completo de texto (PROPOSTO; gate ADR-0127)

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

### Ratificação obrigatória

Este candidato altera dois contratos públicos, descritos em api.md. Não é
L0 vigente até aprovação explícita. Hash do Código antigo é apenas contexto,
não resselo de implementação inexistente. Materializar somente após ratificar
TS1; sem release/merge/push. Testes S1–S16 e gates sob12GiB, swap zero.

### Responsabilidade exclusiva adicional

Somente fixtures/adapters de testes: substituir Style::Size(Pt) por Size(Length absoluto), e expectativas de StyleDelta.size por Option<Length>. Conservar os observáveis linguísticos e os contratos funcionais existentes; não trocar geometria correta por snapshots da representação nova.
