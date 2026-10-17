# D08B3-C2 — origem dos argumentos e valor lexical corrigidos

O dono ratificou o contrato C2 com `confirmo` em 2026-10-07. A correção conserva a origem da definição nos argumentos bare, aliases e spreads da Table; valores de Cell com set/show local conservam a forma lexical Styled e são realizados na inserção efetiva. O recorte C2 está corrigido e verificado. A composição mista com default show de Cell ainda tem um resíduo vertical medido de 6.04996pt; não declarar renderização ou D08 universalmente fechados.

## Medição antes da decisão e autorização

A coleta `table-origens-d08b3c2-2026-10-07/baseline.json.gz` encontrou sete divergências nas oito sondas: caret/trace de argumento bare, alias e spread, body local e func com set/show. O vanilla preserva a primeira origem e expõe Styled lexical. Fonte e file:line estão no relatório de medição e no contrato. A rodada anterior `before` usava str(function), inadequado para decidir forma; foi descartada e substituída por repr(function) em baseline.

Contrato público aprovado: `Content::Source(Arc<SourceContent>)`, com `SourceContent { pub body: Content, pub span: Span }`, e fronteira causal do join local. Não foi acrescentado campo público em Table/Cell. Contrato, quatro candidatos completos e manifesto original permanecem intactos em `00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/`; SHA-256 do manifesto ratificado: `5e5086404c6359dad6658b77dc465aa4a4a6bba4c99f7a505245092a0e07c79c`. As precisões internas seguintes foram medidas, redigidas no L0 antes do código e seguiram em fluxo contínuo ADR-0127.

## Mudança materializada

Source transporta a primeira origem sem criar Location ou elemento observável. Identidade, campos, func, repr, serialização, igualdade de língua, texto e morfologia delegam ao body; ObservationEq continua conservadora quanto à origem causal. Travessia, introspecção, HTML, math e layout visitam a contribuição sem duplicar efeitos. Os 57 owners afetados estão individualizados em owners.json; a transparência compartilhada usa o novo núcleo source-origin.toml, pinado por SHA-256 completo. Os helpers são privados à crate.

A captura de origem é restrita a ContentBlock/CodeBlock e valores nativos que já possuem span. Texto de raiz não ganha artificialmente uma nova árvore. O construtor Table conserva o valor original, fabrica a Cell implícita com seu primeiro span e mantém os campos observados separados do placement. O trace da chamada Table cobre somente a resolução lógica de grupos/placement: erros de callbacks e profundidade preservam seus próprios traces.

O CodeBlock conserva a cauda local Styled e um snapshot causal privado. O delta de regras locais se compõe com as regras atuais por RuleId na inserção; snapshots completos legados mantêm sua restauração. Corpo ocultado não executa efeitos, reutilização conserva uma execução por ocorrência, e Cell explícita não recebe uma Cell implícita adicional. Source de Sequence continua a concatenação plana no markup, levando a origem como fallback nos filhos e preservando suas origens conhecidas.

## RED → GREEN e refutadores

`red.log` registra os dois testes semânticos falhando antes da implementação: origem da definição perdida e valor local cell em vez de styled. Há quatro testes locais novos, incluindo transporte por clone/map e igualdade/hash da língua. O primeiro ensaio que usava API de range inexistente foi erro de compilação, não RED semântico.

Os corpus refutaram a primeira hipótese de aplicar Source a todo Content; a captura foi restringida. Também detectaram transparência de Empty no getter, equação display e um trace Table indevido em recursão fresh; essas fronteiras foram corrigidas estruturalmente, sem filtrar mensagens. A rodada debug complete-depth revelou stack overflow nos dois produtos fresh-growing. Descascar Source na travessia sozinho não resolveu (depth-recheck). A medição do append revelou Sequence encaixada a cada reconstrução; o aplainamento com transporte de primeira origem resolveu os dez controles debug e os dez release, sem aumentar pilha ou alterar o limite de show. As rodadas falhas permanecem como evidência histórica.

## Validação do estado final

Coletas terminais, todas bilaterais contra o pin abaixo:

| Recorte | Resultado da saída completa |
|---|---|
| C2 principal | 8/8 |
| C2 adicionais: imports, aliases, forwarding, reutilização e efeitos ocultos | 10/10 |
| Estilos de Cell C | 58/58 |
| Controles adicionais C | 8/8 |
| Cell isolada/fresh | 51/53; somente warnings antigos de arquivo vazio em raw-fields/raw-x |
| Profundidade debug e release | 10/10 em cada binário; sem abort |
| Presença, selector, equation, matrix e query | 92/92, 23/23, 29/29, 29/29+14/14, 36/36 |
| Table B3 e refutadores | 62/62; refutadores mantêm dois negativos já registrados |
| Raw e Raw bloco | saídas textuais preservadas; métricas preexistentes não são fechadas |

D06 conserva os três negativos conhecidos: texto PDF root-render e warnings de arquivo vazio em raw-eval/raw-mat-eval. Os dois refutadores Table antigos são serialização HTML de additional/html-table e order/table-root-equality. A comparação com a coleta final C não encontrou divergência nova nesses corpus. Não usar igualdade de HTML/PDF como gate universal de paridade.

Os vinte controles geométricos C e quatro controles de tinta conservam palavras, ordem e contribuição; delta vertical máximo 0.00002pt. Família/tamanho/cor coincidem, exceto set-text posicional já negativo antes de C2. Tinta e largura de stroke coincidem em RGB8, incluindo red 2pt; fill do plano permanece com produto substituído/ocultado. Resíduos horizontais antigos de align/inset explícitos, 5pt e 0.005pt, continuam abertos.

Os quatro controles geométricos locais C2 conservam palavras/fontes/cores. Zero inset, body local anterior e default local coincidem até 0.00002pt vertical e 0.000005pt horizontal. O produto misto nested-local-text conserva as palavras OUTER / LOCAL OUTER a, mas a segunda linha fica 6.04996pt acima do vanilla. O C congelado já era negativo nesse fixture e colocava OUTER LOCAL a numa só linha (additional-inherited.json.gz). A melhora de palavras/efeitos não fecha geometria. Próxima frente: D08B3-C3, composição de default show da Cell num produto misto, medindo margens/padding/parágrafos antes de decidir; não introduzir compensação numérica ad hoc.

Builds debug/release passaram. `cargo test --offline --workspace --lib`: 6630 core, 1195 infra e 90 shell passaram, cinco testes core ignorados. Integração `cargo test --offline -p typst-wiring --test cli query`: dez passaram. Total: 7925 testes aprovados. Uma tentativa intermediária apontou para o target inexistente cli_query; está preservada em validation.json e não serve como validação. A validação autoritativa é terminal-validation.json.

`crystalline-lint --format sarif .`: zero erros, 535 warnings e 1636 notes na árvore inteira. V5/V6/V15/V26 dos owners C2: zero. Não afirmar zero violations global: os avisos históricos permanecem. A rodada intermediária detectou pins fora do cabeçalho/ordem; posição canônica e ordenação foram corrigidas e resselo ficou restrito aos owners C2, sem fix global.

## Proveniência e preservação

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Baseline produtivo antes de código: before-code.json.gz, UTC 2026-10-07T17:49:33.587269+00:00. Estado final testado: tested-terminal.json.gz, UTC 2026-10-07T19:00:17.011702+00:00. Ambos conservam a lista exata de arquivos, conteúdos, SHA-256, git status e git diff HEAD --stat. As coletas e terminal-validation.json conservam início/fim UTC, fonte integral, argv/cwd/ambiente, estado e hashes. final-results.json verifica que a fonte produtiva/L0 não mudou após testes/coletas.

Vanilla upstream pin `9f2b6e8715237cb086899a42873660fe744622e8`, árvore/receita ratificadas em P1489; executável `lab/typst-original/target/release/typst`, SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. Não usar a string de versão como prova. Tekt release SHA-256 `ac889581d6423d2fcfdd77d2f7e5cf57562dbf9adb40e73aebc25f959864c04f`; debug `7c18ed7361f2724f00d49d7d78d56942acda078499a40ef323528f99204bba29`. Cada coleta congela os executáveis e registra o hash.

Em relação ao snapshot recebido, somente os 57 pares consumer/L0 e o novo núcleo mudaram no código produtivo/L0 (115 arquivos). Para arquivos inicialmente limpos, o verificador usa git show do HEAD como baseline; arquivos dirty recebidos usam o conteúdo congelado, não HEAD. implementation.patch, seals.json e final-results.json documentam o delta próprio, ownership e hashes. Nenhuma alteração herdada foi apagada; sem commit/reset/resselo global.

Evidências completas: `00_nucleo/diagnosticos/table-origens-d08b3c2-2026-10-07/`. A lista e métricas autoritativas estão em final-results.json. Rich casts, Table geral/multipágina, Raw/math/set-text universal, whitespace e PDF/UA continuam nos limites das etapas anteriores.
