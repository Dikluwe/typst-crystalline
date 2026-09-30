# Política de hifenização e validação — 2026-09-28

Esta rodada faz `hyphenate` chegar ao seletor e rejeita argumentos inválidos nas formas set e constructor. `false` desativa a hifenização, `true` a permite, e `auto`/ausência seguem o `par.justify` lexical, cujo default é false. As opções deixam de ser descartadas silenciosamente nesse caminho. A justificação visual e a escolha de estratégia de parágrafo permanecem incompletas.

## Medição anterior à classificação

As [52 entradas iniciais](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/correcao-politica-hifenizacao-2026-09-28/before-pdf/resultados.json) confirmavam que as oito combinações de hyphenate/justify eram equivalentes no Tekt. O vanilla pinado distingue essas políticas. Os controles inválidos compilavam no Tekt e falhavam no vanilla.

A comparação [depois da correção](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/correcao-politica-hifenizacao-2026-09-28/after-pdf/resultados.json) e os [seis controles de escopo](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/correcao-politica-hifenizacao-2026-09-28/lexical-pdf/resultados.json) medem os resultados abaixo. Fontes, fontes tipográficas, PDFs, XMLs, stderr, comandos e identidade dos binários estão junto dos resultados.

| Controle | Antes | Corrigido | Limite da prova |
|---|---|---|---|
| false, com ou sem justify | Hifeniza | Não hifeniza | Não prova composição justificada |
| true, sem justify | Hifeniza | Hifeniza | Escolha dos pontos ainda pode divergir |
| auto/ausência, sem justify | Hifeniza | Não hifeniza | Linhas simples do controle coincidem |
| auto/ausência, com justify | Hifeniza | Hifeniza por opção transportada | Optimized e ajuste dos espaços ainda faltam |
| Boolean/auto inválido | Aceita ou ignora | Erro | Mensagem principal e linha/coluna iguais ao vanilla |
| Estratégia inválida | Aceita ou ignora | Erro | Não implementa a estratégia optimized |
| Override local false/auto | Opção descartada | Aplica localmente e restaura a opção externa | Verificado também com bloco lexical e show-set de par |
| Duas colunas do controle terminal | Hifenização divergente | Uma página e mesmas linhas visíveis | Este controle, não toda a superfície de colunas |

## Correção e relação com os prompts

Os pontos medidos eram [set rules](/repos/Antigravity/typst-crystalline/01_core/src/compiler/eval/rules.rs:3000), [constructor text](/repos/Antigravity/typst-crystalline/01_core/src/compiler/stdlib/text/constructor.rs:153), [constructor par](/repos/Antigravity/typst-crystalline/01_core/src/compiler/stdlib/structural/par.rs:105) e [seleção de quebras](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/break_selection.rs:40). O L0 nativo de par declarava deliberadamente `justify` aceito e ignorado; o L0 da atomização proibia honrar propriedades descartadas naquela separação. Essas cláusulas precisavam ser atualizadas, não apenas o código.

Os L0 foram editados primeiro. Eval valida e transporta Bool/Auto pelo canal custom já existente. [WordLayout](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/word_layout.rs:27) resolve a opção na cadeia lexical e passa um boolean ao seletor privado. A opção resolvida não fica armazenada em TextStyle. O seletor desabilitado retorna Wrap sem consultar padrões de hifenização; o habilitado conserva o percurso Unicode anterior. Medição, emissão, segmentação e progresso regional mantêm seus donos.

O novo [Núcleo Tekt](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/_nuclei/paragraph-break-policy.toml) compartilha domínio dos valores, resolução lexical, fronteira eval/layout e vocabulário dos diagnósticos. Ele é consumido e pinado pelos cinco owners produtivos; não é owner de código. O lote tem nove pares Prompt L0 ↔ consumer: cinco produtivos e quatro consumers que incluem testes. Nenhuma entidade, campo, trait, assinatura pública, flag ou fase de pipeline foi acrescentada.

## RED → GREEN e integração

Nove regressões comportamentais falharam antes da implementação. Dois erros na montagem das novas fixtures foram corrigidos antes desse RED e seus logs foram preservados. A primeira implementação passou cinco testes, mas quatro mensagens diziam `int` em vez de `integer`; os novos casts passaram a usar nomes longos locais, preservando `type_name` e o helper global existente.

A integração encontrou canaries antigos de opções descartadas. Eles foram migrados para stroke, first-line-indent ou overhang, conservando contagens, identificação da propriedade/target e distinção entre spans. Os dois testes positivos de padrões en/pt passaram a pedir `hyphenate: true`, preservando a exigência de hífen. Um novo teste de infraestrutura cobre seis combinações de política e justify. Essas mudanças de inputs são explícitas; nenhuma referência de snapshot ou tolerância geométrica foi alterada.

Resultado final:

- **6.317 testes de domínio passaram**, zero falhas, dois ignorados.
- **1.123 testes de infraestrutura passaram**, zero falhas ou ignorados, incluindo snapshots existentes.
- Builds debug/release concluídos; **lint sem violações**, com 496 avisos na árvore atual.
- **58 entradas bilaterais e 193 verificações delimitadas passaram**, sem alegar paridade plena desses documentos. Os observáveis do oráculo nas 52 entradas repetidas permaneceram iguais.
- Paginação terminal, continuação, quebras explícitas e contagem de páginas de notas/flutuantes conservaram as correções anteriores. Tracking, nota terminal e flutuante inferior mantiveram sua geometria de referência.
- Inspeção visual confirmou a restauração lexical da hifenização e o resultado de colunas.

Os controles e critérios executáveis estão em [acceptance.py](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/correcao-politica-hifenizacao-2026-09-28/acceptance.py) e [acceptance.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/correcao-politica-hifenizacao-2026-09-28/acceptance.json). Logs RED, intermediários e GREEN permanecem arquivados.

## Pendências delimitadas

`justify` já participa de auto, mas ainda não estica/encolhe as linhas. Set true avisa essa limitação. `linebreaks` valida o domínio, mas ainda não seleciona simple/optimized; o set conserva seu aviso de scope-out e a chamada nativa continua parcial. Os L0 nomeiam `rodada-config-composicao` como a etapa que completa esses subconjuntos.

A fronteira do `par` nativo continua transparente. A palavra longa na origem da linha ainda não é reavaliada como no vanilla com hifenização ativa. A emissão usa hífen literal, enquanto o PDF vanilla destes controles extrai U+00AD; a comparação de linhas visíveis normaliza apenas esse caractere e não fecha sua semântica de extração. Show-set com seletor string já falhava no estado inicial e permanece aberto; o show-set de par medido funciona. Tracking por clusters e distribuição de notas/flutuantes também continuam abertos.

A próxima frente é preparar o parágrafo com suas fronteiras e opções, distinguir linha final de linha continuada e então compor a justificação. Isso precede a comparação de seletores intercambiáveis e a decisão de wiring.

## Proveniência

Working tree não commitado; HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`. As alterações anteriores foram preservadas. [initial.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/correcao-politica-hifenizacao-2026-09-28/initial.json) conserva UTC, lista exata de arquivos alterados, `git diff HEAD --stat`, hashes de código/L0 e binários antes do lote. [final-validation-state.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/correcao-politica-hifenizacao-2026-09-28/final-validation-state.json) registra o estado final em `2026-09-28T17:19:04.763509+00:00`. [final-receipt.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/correcao-politica-hifenizacao-2026-09-28/final-receipt.json) verifica os nove pares alterados, o novo núcleo e a ausência de alterações de código/L0 fora desse escopo.

As medições começaram em `2026-09-28T16:51:50.980759+00:00` (antes), `2026-09-28T17:13:15.199118+00:00` (depois) e `2026-09-28T17:14:56.053953+00:00` (escopo). Cada diretório contém sua proveniência com estado e hashes completos. O rebuild release final confirmou a identidade do binário medido.

Vanilla: pin `9f2b6e8715237cb086899a42873660fe744622e8`, árvore ratificada `6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, receita P1489 e SHA-256 do binário `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. Tekt corrigido: SHA-256 `c4af44ed9693be06c033cee7a1cd48f3026ecebb69794eaed1a3a634e62dd3d5`. A versão textual não foi usada como identidade de build.
