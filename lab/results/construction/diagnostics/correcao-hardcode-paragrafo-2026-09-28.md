# Hardcode funcional no parágrafo — 2026-09-28

A compensação óptica do hífen deixou de ser obrigatória. `overhang: false` agora atravessa `#set text`, o constructor `text(...)` e os escopos de preparação; desativa a margem óptica sem mudar candidatos ou quebras escolhidas. O padrão true permanece. O lote também elimina uma saturação arbitrária das contagens de caracteres e identifica as constantes normativas pelo significado.

## Proveniência da decisão

A medição final é do HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`, **working tree não commitada**, às `2026-09-28T21:03:37.010696+00:00`. [finalreceipt.json](correcao-hardcode-paragrafo-2026-09-28/finalreceipt.json) contém a lista exata de arquivos alterados no estado medido, `git diff HEAD --stat`, status integral, hashes de fontes/L0 e logs. A árvore já tinha alterações de outras rodadas; o diff integral não pertence somente a esta correção.

| Referência | Identidade |
| --- | --- |
| Vanilla pinado | `9f2b6e8715237cb086899a42873660fe744622e8` |
| Árvore e receita vanilla | `6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, P1489 |
| Binário vanilla | `lab/typst-original/target/release/typst` |
| SHA-256 vanilla | `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea` |
| SHA-256 Tekt anterior | `32b58a3ef9be45776b43495c517ac89e827d5a0198d536594e9eddf502b77bf2` |
| SHA-256 Tekt final | `df6b38883edc780ea1ae04c200f2bbf7089f856a8a0965d9930e5ff5b3b31595` |

O estado inicial está em initial.json e before/. Os runners conservam fontes, comandos, arquivos tipográficos, PDFs, extrações, stderr e proveniência. Não houve alteração produtiva/L0 depois das sondas finais; o recibo confirma os hashes do mesmo estado usado nos testes, build, lint e sondas. A string de versão não serve de identidade.

## Medição antes da implementação

before-pdf/resultados.json registra 20 entradas novas. Antes da correção, o Tekt aceitava overhang válido com warning no set e sem efeito no constructor. Auto, integer e string também eram aceitos indevidamente; o vanilla exige boolean. Nos controles false com justificação, o Tekt continuava aplicando margem óptica. A fonte pinada `text/mod.rs:391-393` declara bool default true e `inline/line.rs:518-525` consulta o estilo final antes da compensação.

O L0 da rodada anterior especificava a compensação padrão e deixava overhang configurável como scope-out. A implementação seguia essa incompletude. Ela foi corrigida no L0 antes do código; não se inferiu intenção histórica a partir do comportamento.

[red-certified.log](correcao-hardcode-paragrafo-2026-09-28/red-certified.log) mostra três falhas relevantes: política false ainda produz margem, set válido ainda gera warning descartado e constructor não transporta a opção. [first-green.log](correcao-hardcode-paragrafo-2026-09-28/first-green.log) mostra os quatro controles passando depois da implementação, incluindo a proteção da fórmula natural da última linha.

### Hipótese refutada durante a auditoria

A leitura isolada de raw_cost sugeriu que a última linha justificada deveria pagar badness positiva. Os chamadores refutaram essa hipótese antes de implementar: `inline/linebreak.rs:551-559` passa attempt.justify, que `inline/line.rs:138-139` desativa para mandatory. A fórmula produtiva foi conservada. O teste inicial com expectativa incorreta está preservado em red.log; não é contado como evidência de defeito. O L0 foi retificado e o teste numérico passou a proteger a linha final natural. red-refined.log conserva um erro de compilação do observador de teste, corrigido antes do RED certificado; não é prova funcional.

## O que foi limpo

| Valor ou decisão | Classificação e resultado |
| --- | --- |
| Overhang sempre ativo | Hardcode funcional confirmado. Substituído pela opção lexical do fragmento final, com default true da linguagem. |
| Saturação em 255 | Mecânica de representação desnecessária nos campos privados usize. Removida; offsets UTF-8 e contagens completas são conservados. A penalidade já usa distância limitada por saturating_sub. |
| Fator 0.55 | Constante normativa do glyph de hífen no pin. Nomeada e documentada; só se aplica quando a opção lexical está ativa. |
| 135, distância 5, incremento 0.15 | Defaults e fórmula medidos na fonte. Nomeados/documentados. **Isso não implementa custos personalizados.** |
| English quando idioma ausente | Default da língua confirmado em text/mod.rs:473-475; conservado. |
| Limite latino 0x024f | Limite declarado do subconjunto certificado, com fallback integral; não representa Unicode/UAX14 completo. Conservado como limitação explícita. |

[hardcode-inventory.json](correcao-hardcode-paragrafo-2026-09-28/hardcode-inventory.json) registra as classificações. Não se confundiu a remoção de configuração ignorada com a eliminação indiscriminada de literais matemáticos.

## Responsabilidades e L0

`eval/rules` e `stdlib/text/constructor` validam bool e transportam `text.overhang` no canal lexical existente. O constructor conserva o value_span da ocorrência correspondente para os diagnósticos. Não foi adicionado campo público a TextStyle nem alterada assinatura.

`paragraph_prepare` conserva a opção em Fragment privado e a considera ao fundir fragmentos adjacentes. Políticas diferentes dentro de uma palavra ativa continuam recusando a preparação global. A whitelist admite a opção tipográfica e a preparação restaura a chain exterior.

`paragraph_hyphen` consulta a opção ao materializar a linha escolhida. Custo natural, algoritmo, penalidades e fronteiras de quebra não mudam; somente a distribuição visual de espaço perde a compensação quando false. A atomização anterior permanece: preparação, candidatos/seleção e emissão têm owners separados. Não há despacho dinâmico nem lógica nova em L4.

O núcleo paragraph-selection foi atualizado e seus consumidores ressellados. [scope-review.json](correcao-hardcode-paragrafo-2026-09-28/scope-review.json) mede **33 arquivos produtivos/L0 alterados**, sem arquivo fora do escopo. A maioria dos consumidores do núcleo só mudou linhagem/pin. [review.diff](correcao-hardcode-paragrafo-2026-09-28/review.diff), scope.json e scoped-seals.json conservam o detalhe.

O canary de infraestrutura continua exigindo três warnings distintos. Apenas a propriedade agora suportada foi substituída por costs:(hyphenation:100%), ainda declaradamente incompleta. Contagem, spans distintos e controles baseline/stroke foram mantidos. O L0 do harness legitimou essa rotação antes do código.

Gate ADR-0127: correção interna/de paridade, sem contrato público, flag, mudança de fase ou quebra de compatibilidade; fluxo contínuo com L0 primeiro e resselo.

## Resultado verificado

| Verificação | Resultado |
| --- | --- |
| Core | 6.350 passados, 2 ignorados, zero falhas |
| Infraestrutura | 1.127 passados, zero falhas |
| Build offline debug/release | Concluídos |
| crystalline-lint | Zero erros; 504 warnings, 1.514 notes |
| Entradas bilaterais | 227: 20 novas + 207 regressões |
| Verificações finais | 803 passadas, zero falhas |
| Novos controles válidos | 17 de 17 coincidem geometricamente com o vanilla |
| Divergências geométricas corrigidas | 6 dos novos controles |
| Controles de tipo inválido corrigidos | 3, com bloco de erro integral e span coincidentes |

Os números pertencem ao estado exato do recibo final. Logs: core-final.log, infra-final.log, build-final.log, release-final.log e lint-final.sarif. [acceptance.json](correcao-hardcode-paragrafo-2026-09-28/acceptance.json) enumera cada verificação. Os controles novos cobrem ausência/true/false, simple/optimized, justify false/true, constructor, show par, override inline, restauração entre parágrafos e rejeição de auto/integer/string.

A comparação geométrica usa palavras e caixas em coordenadas da página, tolerância inferior a 0.0005pt. Normaliza SHY para hífen literal apenas nessa comparação; não fecha a identidade do texto copiado nem compara bytes PDF. Grupos XML não são contados como linhas físicas. Os casos anteriormente abertos mantêm suas verificações específicas de fallback ou limitação, sem serem promovidos a paridade.

A primeira versão do observador de erros selecionava a primeira linha de stderr e incluía spans de warnings anteriores. Em invalid-native, isso capturava o warning preexistente de suporte parcial de par, embora o erro e seu span já fossem corretos. O observador passou a comparar o bloco completo iniciado em error:, além da mensagem e dos spans. Stderr integral e a primeira aceitação falhada permanecem conservados; diagnostic-observer-retification.json documenta a correção. Nenhum código de produto ou expectativa de erro foi relaxado.

## Limites que continuam abertos

A correção cobre a margem do **hífen de quebra** no subconjunto global certificado. Overhang de pontuação comum e do percurso incremental, custos personalizados, palavras com políticas/estilos misturados, Unicode/UAX14 completo e larguras variáveis continuam incompletos. Os controles preexistentes de par aninhado, show/repr e viúvas/órfãs permanecem abertos.

A identidade textual do hífen no PDF também permanece aberta. O carrier atual conserva o glyph literal; deduzir origem a partir de posição, glyph ID ou caracteres vizinhos criaria outro hardcode. A frente deve preservar explicitamente a diferença entre desenho e texto copiado, com L0 e contrato adequados antes da materialização.

Não houve benchmark: esta rodada prova comportamento e integridade da atomização, não desempenho ou estado da arte.
