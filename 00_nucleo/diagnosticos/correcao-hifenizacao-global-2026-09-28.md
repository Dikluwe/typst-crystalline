# Hifenização integrada à seleção global de parágrafos — 2026-09-28

A hifenização deixou de depender apenas do overflow incremental no subconjunto certificado de parágrafos. Os seletores `simple` e `optimized` agora consideram os mesmos pontos dentro das palavras, incluindo palavras na origem da linha. A preparação, os custos e a emissão permanecem unidades privadas de L1, com owners L0 individuais.

## Estado e proveniência

Este é um relatório de correção, não um Prompt L0. A medição final é do HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`, com **working tree não commitada**, em `2026-09-28T20:31:54.365076+00:00`. O estado integral, a lista de alterações (`git diff HEAD --stat` e status incluindo untracked), hashes de fontes/L0 e logs estão em [finalreceipt.json](correcao-hifenizacao-global-2026-09-28/finalreceipt.json). A árvore já tinha alterações anteriores; não atribuir o diff integral a esta rodada.

| Referência | Identidade |
| --- | --- |
| Vanilla pinado | `9f2b6e8715237cb086899a42873660fe744622e8` |
| Árvore vanilla/receita | `6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, receita P1489 |
| Binário vanilla | `lab/typst-original/target/release/typst` |
| SHA-256 vanilla | `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea` |
| SHA-256 Tekt anterior | `c626ab4994885b54a998cdc2ce1de86108b557c009ecf18ac701a8145314cc10` |
| SHA-256 Tekt final | `32b58a3ef9be45776b43495c517ac89e827d5a0198d536594e9eddf502b77bf2` |

[initial.json](correcao-hifenizacao-global-2026-09-28/initial.json), before/, before-pdf/proveniencia.json e after-pdf/proveniencia.json conservam estado, fontes, comandos, inputs, fontes tipográficas e binários de cada medição. Depois das sondas PDF, somente o observador `cfg(test)` de infraestrutura e seu L0 mudaram. O rebuild final produziu bytes idênticos aos do binário das sondas: a ponte de identidade está no recibo. Não usar a string de versão como prova.

## Medição anterior à decisão

O owner anterior de `paragraph_prepare` excluía hifenização ativa; `paragraph_select` recebia apenas finais de palavras. `break_selection` tratava hifenização por overflow local. Portanto a incompletude vinha também do **L0**, que delimitava a implementação entregue. Isso documenta uma implementação adiada; não demonstra intenção ou “preguiça” de quem a escreveu.

O lote before-pdf mede 67 entradas, conservadas antes de editar L0/código. A fonte vanilla em `inline/linebreak.rs:170-209`, `:626-684` e `:815-854` mostra candidatos internos compartilhados entre as estratégias, custos de hífen e comprimentos em caracteres. `inline/shaping.rs:583-635` distingue o glyph desenhado do texto copiado. A nova decisão foi escrita nos owners e no núcleo compartilhado antes da materialização.

[red.log](correcao-hifenizacao-global-2026-09-28/red.log) registra duas falhas com o código anterior: preparação de política true e quebra da palavra longa na origem da linha. [first-green.log](correcao-hifenizacao-global-2026-09-28/first-green.log) registra a passagem após a implementação.

A primeira emissão global acertou as escolhas medidas, mas faltava margem óptica nas linhas justificadas terminadas em hífen. Em `en-140-optimized-True`, ex- ocupava xMin115.26/xMax132 no Tekt contra117.385197/134.1252 no vanilla: diferença de 2.1252pt. `inline/line.rs:518-526` e `:708-709` acrescentam 55% da largura do glyph ao espaço disponível **na emissão**. O L0 registrou essa medição antes da correção; os custos de escolha permaneceram baseados na largura natural. Os resultados intermediários estão em after-first-resultados.json e after-first-proveniencia.json.

## Correção e atomização

- **paragraph_prepare:** conserva política lexical resolvida em cada fragmento e prepara a palavra inteira. Não cria espaço entre fragmentos contíguos. Um estilo/política misturado dentro de palavra ativa recusa a preparação global antes de emitir conteúdo.
- **paragraph_hyphen:** novo owner privado. Converte pontos em offsets UTF-8 seguros, mede os intervalos exatos com FontMetrics injetado e mede o glyph de hífen separadamente. Os padrões existentes continuam no helper de hifenização. Idioma ausente usa English, conforme `text/mod.rs:473-475`; idioma sem padrões mantém a palavra sem candidatos internos.
- **Seleção:** `simple` e `optimized` recebem o mesmo grafo ordenado de quebras. O greedy pode quebrar uma palavra na origem. O DP compartilha o custo-base e acrescenta `135*(1+0.15*(max(5-left,0)+max(5-right,0)))`, mais 135 para duas linhas consecutivas hifenizadas. Somente as linhas escolhidas são materializadas.
- **paragraph_emit:** conserva estilos, carriers de separação, baselines e restauração lexical. Prefixo e glyph de hífen são fragmentos separados e contíguos. A compensação óptica afeta a distribuição dos espaços, sem adulterar a seleção ou somar largura duas vezes.
- **paragraph_plan:** certifica todos os segmentos antes de emitir o primeiro, escolhe a estratégia vigente e delega. Sem candidatos internos, conserva o seletor anterior sem hifenização.

Não foram adicionadas entidades, assinaturas públicas, flags, fases ou despacho dinâmico. Correção de paridade/interna em fluxo contínuo ADR-0127. O núcleo paragraph-selection compartilha invariantes pinadas; não é consumer nem legitima código diretamente. `scope.json`, `scoped-seals.json` e [review.diff](correcao-hifenizacao-global-2026-09-28/review.diff) documentam ownership e resselo. A revisão contra os hashes iniciais identificou **27 arquivos produtivos/L0 alterados**, sem alteração fora do escopo.

## Aceitação medida

| Verificação | Resultado final |
| --- | --- |
| Core | 6.346 passados, 2 ignorados, zero falhas |
| Infraestrutura | 1.127 passados, zero falhas |
| Build debug/release offline | Ambos concluídos |
| crystalline-lint SARIF | Zero erros; 504 warnings e 1.514 notes |
| Entradas bilaterais | 207: 67 novas + 140 regressões |
| Asserções do corpus | 699 passadas, zero falhas |
| Novos controles com geometria coincidente | 66 de 67 |
| Divergências geométricas anteriores corrigidas | 44 dos novos controles |
| Controles com diferença de identidade textual do hífen | 57 dos novos controles; limitação aberta |

Os números vêm do estado exato do recibo final. Logs: core-verified.log, infra-final.log, build-final.log, release-final.log e lint-final.sarif. [acceptance.json](correcao-hifenizacao-global-2026-09-28/acceptance.json) lista cada asserção; os scripts reproduzem as comparações.

As novas sondas cobrem en/pt/de, larguras 100/140/180pt, simple/optimized/auto, justify false/true, políticas false/auto/true, par nativo, bold, override local, pontuação, idioma sem padrões e múltiplos parágrafos. As regressões cobrem a seleção anterior, composição, precedência lexical, política, parágrafos vazios e controles de paginação já existentes.

A comparação geométrica usa palavras e suas caixas em coordenadas da página, tolerância inferior a 0.0005pt, normalizando SHY para “-” **somente nessa comparação visual**. Não declara igualdade de texto copiado, bytes PDF ou estruturas internas. Não confunde grupos XML do extrator com linhas físicas. `mixed-word` permanece divergente e seu fallback foi verificado contra a medição anterior, sem ser contado como paridade.

A suíte de infraestrutura inicialmente teve três falhas, preservadas em infra-verified.log: o observador contava apenas Text com palavra+hífen no mesmo fragmento. O L0 do harness foi atualizado primeiro; o helper agora aceita prefixo alfabético e hífen contíguos na mesma página/baseline sob FixedMetrics dos fixtures. Foram mantidos inputs e expectativas de idioma/política, com controles negativos para hífen isolado, espaço e mudança de linha. Nenhum código de produto foi alterado para acomodar o observador. infra-observer-targeted.log e infra-final.log registram a correção e a reexecução.

QA auxiliar: o recorte de en-140-optimized-True em 144dpi, 280×360px, foi inspecionado visualmente e teve pixels coincidentes. Resultado em visual/comparison.json. Essa identidade é evidência auxiliar, não gate de paridade da língua.

## Limites e próximas frentes

O cache de intervalos exatos custa O(k²) em espaço e número de medições por palavra com k cortes; o DP tem pior caso O(B²) para B breakpoints, com estado O(B). Não houve benchmark de desempenho. A exatidão das larguras e a separação das responsabilidades são avanços verificados; este lote não sustenta alegação de algoritmo no estado da arte.

Permanecem abertos:

- Texto copiado do PDF: o Tekt exporta hífen literal e o vanilla conserva SHY. A geometria já coincidente não fecha essa diferença de comportamento.
- Mistura de estilos/políticas **dentro da mesma palavra** e pontuação interna: preparação global ainda recusa esses casos, conservando o caminho incremental.
- overhang configurável e de pontuação comum; a correção atual cobre a margem padrão do hífen de quebra no subconjunto medido.
- CJK/UAX14 completo, tracking, smallcaps, matemática/objetos, decoração geral, larguras variáveis, colunas/células/subframes e interação com floats.
- Par nativo aninhado, recursão em show par/repr e viúvas/órfãs: continuam os controles explicitamente abertos das rodadas anteriores.

A próxima frente mais localizada é conservar a identidade textual do hífen de quebra até o exportador. Depois, a preparação de palavra composta por fragmentos permite ampliar hifenização sem transformar fronteiras de estilo em espaços nem misturar políticas lexicais. Ambas exigem medição e L0 próprios antes de implementar.
