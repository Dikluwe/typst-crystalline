# Block/Align — posição do container, alinhamento das linhas e avanço causal

Correção interna de paridade em 2026-10-06. Sete consumers L1 e seus sete owners L0. Nenhum unsafe novo, API pública, flag, fase ou dependência nova. Não houve commit nem resselo global.

## Medição anterior à decisão

`baseline-verification.json.gz` registra os 14 controles iniciais; `baseline-extension.json.gz` acrescenta variações de par.spacing/leading; `linebreak-baseline.json.gz` refuta o no-op de quebras explícitas vazias. Before.json contém fontes dos módulos antes da implementação, HEAD/working tree, UTC, git diff HEAD --stat/status completo e identidade dos binários. O oráculo é vanilla pinado **9f2b6e8715237cb086899a42873660fe744622e8**, não main móvel nem /usr/local/bin/typst.

1. `#set text(dir: rtl); #block[#text(dir: ltr)[ABC 123]]`: Tekt em x20, vanilla em x152.949220. Ao especificar width=100pt, ambos começam em x100. Portanto, forçar RTL no corpo não é a correção: alteraria o trecho e faria o caso de largura explícita divergir. O observável separa posição do container automático de direção interna.
2. Em Align(left/right) com árabe seguido de uma linha LTR, o Tekt mantém a borda RTL da primeira e a borda esquerda da segunda. `cursor.rs:154-162` aplica alinhamento por direção; o envelope de `placement.rs:35-177` tenta alinhar tudo depois, pela maior borda, e não consegue aplicar a intenção a cada linha. A fonte vanilla `flow/collect.rs:193` resolve Align nas linhas; `flow/block.rs:145-166` distingue auto-size de expansão. Esta referência é de língua/posição, não objetivo de copiar a mecânica.
3. A primeira linha de Align começa em y11.642578 no Tekt e y19.999978 no vanilla. Subframe já preparado tipograficamente e subtração posterior de ascender misturam origens. Conteúdo depois de Align também perde a margem: y28.357418 contra41.557438; par.leading=2em não altera esse gap, enquanto par.spacing=3em altera o vanilla para61.357418.
4. O controle nested ainda perde 15.5074pt após corrigir alinhamento: Linebreak autoral numa linha vazia não cria causa para o flush. Isso também aparece fora de Align: uma/dois breaks iniciais deixam ABC em y20 no Tekt, contra35.507448/51.014828 no vanilla. `line_commit.rs` corretamente não avança um flush técnico vazio; o adapter de Linebreak é que precisa reservar a linha autoral.

Histórico com limites: a fonte de placement documenta extração no P96.7 e ajustes de origem/largura do envelope em P772g/P772j; P750-752/P762 documentam a migração de métricas para cap-height/text edges. O modelo do envelope e as compensações sobreviveram a esses ajustes. São pistas documentais, não prova do primeiro commit que introduziu cada defeito. A decisão vem dos controles atuais reproduzíveis.

## Correção e responsabilidades

- **Coordenação/checkpoint:** `explicit_line_alignment: Option<HAlign>` privado, inicial None, capturado/restaurado em rollback. Não se usa direção de run como substituto do alinhamento do container.
- **Cursor:** o override Left/Center/Right tem precedência sobre a âncora automática. Start/End são resolvidos na entrada do container. Sem override, conservar o caminho RTL/tracking anterior.
- **Placement:** certificado estrutural fechado para corpos textuais finitos, com VAlign ausente/Top. Instalar/restaurar override lexical, emitir uma vez no subframe geométrico existente e recompor com origem Y zero. Cada linha já vem alinhada, portanto não aplicar outra translação horizontal do envelope. Admitir margem após linha anterior ou frame pendente pelo protocolo causal; a margem de saída fica pendente, sem inflar o fundo de uma página final auto.
- **Subframe:** a cauda drenada manualmente também aplica o override, uma vez. Direção/bidi continuam isolados; o override horizontal é herdado lexicalmente e restaurado por placement.
- **Block:** largura explícita posiciona o container conforme alinhamento herdado ou direção externa. Block automático textual simples em região finita, sem paint/clip/inset/outset/mudança de página, posiciona os itens emitidos como unidade à direita quando necessário; usa frames e métricas existentes, sem executar novamente o body. Translada em lugar e move os novos segmentos de decoração.
- **Linebreak:** se falta uma linha causal, reservar Text vazio e baseline pelo helper existente; manter boundary transparente e commit normal. Não chamar Text layout nem consumir um segundo span bidi. O flush técnico vazio continua não avançando.

A suíte refutou inicialmente o caminho novo para Bottom/Horizon sob altura automática (P898). Esse eixo conserva a rota/deferimento anterior. O certificado positivo recusa outros carriers; não há troca geral do algoritmo de layout ou promessa de que todo Block/Align está fechado.

## Verificação

REDs reais de posição, baseline, margem de entrada/saída e linha autoral vazia em execution.json.gz. O teste de duas linhas preserva style.dir=LTR no trecho, e a geometria tem que seguir o alinhamento explícito. Também se verifica a posição após rollback e que Align(left) não muda a âncora RTL do parágrafo seguinte.

- **6.536 testes L1 passaram**, cinco ignored preexistentes; **1.172 L3 passaram** com relógio PDF fixo.
- Build debug/release passou; git diff --check passou. Lint: **0 erros**, **531 warnings preexistentes**, **1.578 notes** (duas V20 novas, informacionais, de profundidade do pattern Some(Value::Length) em placement). Nenhum novo warning; V15/V26 e drift do manifesto ausentes. Não é zero violations globais.
- **85/88** controles passam texto normalizado e caixas agrupadas, contra 62/88 antes. **23 melhoraram**, nenhum antes positivo regrediu. Maior delta entre aceitos **0.016206pt**, tolerância 0,05pt. Os 24 controles focados passaram; o corpus anterior inclui bidi, equações, hifenização, fonte efetiva e eixos variáveis.
- Quatro controles de Block/Align/nested inspecionados a 600 dpi: mesmas caixas de tinta e zero pixels escuros sem correspondência dentro de 1 px, nas duas direções, limiar 180. Isso não é igualdade de bytes/pixels absoluta. Logs, comandos, hashes e recortes estão em raster.json e scripts correspondentes.

A primeira suíte L3 teve duas falhas de igualdade de bytes nos testes CMYK P270.2: **só quatro bytes de data/hora** diferiam, em CreationDate/ModDate/XMP (10:22:51Z versus 10:22:52Z); arrays completos preservados no log. Esses testes não fixam o relógio antes das duas exportações. A suíte repetida com SOURCE_DATE_EPOCH=0 passou integralmente. Não se alterou o produto para esconder a falha nem se considera que a fragilidade temporal desses testes foi corrigida nesta frente.

## Tempo da CLI inteira

Coleta final **2026-10-06T10:30:06.699275+00:00**, dois aquecimentos e 15 rodadas pareadas em ordem aleatória; SOURCE_DATE_EPOCH=0/RAYON_NUM_THREADS=1, sem build/lint/render concorrentes. Medianas em ms; inclui aquisição de fontes, compilação e PDF, não isola L1:

| Controle | Antes | Depois | Vanilla | Delta pareado | IQR do delta |
|---|---:|---:|---:|---:|---:|
| grid320 | 196.10 | 196.04 | 148.32 | -0.37 | -0.88 a +2.10 |
| aligned-80 | 170.81 | 169.83 | 133.82 | -0.41 | -2.28 a +0.92 |
| blocks-80 | 165.12 | 166.15 | 130.47 | +0.48 | +0.14 a +1.40 |

Esta coleta não demonstra ganho consistente. Esta é uma correção da defasagem de geometria/causalidade; não uma redução comprovada da diferença total para o vanilla. Sem layout extra do body no reparo de Block automático.

## Proveniência e pendências

HEAD **06848cdcb5450f2c555626bf116bc7acd82bd82b**, working tree não commitado. execution.json.gz congela os sete módulos/owners finais, UTC, lista completa de alterações e logs. seals.json verifica os CodeHashes sem a linha @prompt-hash e guarda SHA completos; verification.json.gz/benchmark.json guardam estado/comandos/fontes de cada medição. Antes: `58f40a987ce5e50916b4112874acee9cec29832d6196975b443100b8b5de68e1`. Final: `1a4c5c101caceb4f87dd7b706961a0f43054702817de125ff3eb22daa6131df2`. Vanilla: `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. SHA das fontes em fonts-and-metadata.json; não se usa apenas --version como proveniência.

Restam três controles medidos: dois com controles bidi autorais e o espaço literal terminal. Fora da aceitação desta rodada: geometria de carriers desconhecidos, Block paginado/paint/clip/inset/outset, Align em largura automática e revisão geral do alinhamento vertical, paridade geral de HTML ou todos os contextos bidi em subframes. A proposta Coverage::from_ranges segue separada e não foi materializada.

Próxima frente delimitada: preservar a identidade dos controles bidi autorais na extração PDF, distinguindo ordem visual, origem lógica e formatação introduzida pelo extrator. Medir o carrier/CMap/ActualText existente antes de decidir um contrato.
