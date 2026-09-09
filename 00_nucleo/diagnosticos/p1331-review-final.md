# P1331 — parecer final independente C2/R2

**Veredito: PASS_SCOPED.** A materialização C2 satisfaz o fragmento de
diagnóstico do fallback de `calc.abs` definido no L0 vigente e verificado
pelos testes R2. O fechamento pode registrar este parecer e os gates.
Não se afirma equivalência geral de abs, calc ou dos tipos da linguagem.

## Autoridade e entradas

Revisor `/root/p1331_review`, em filesystem compartilhado. Regime A/B
executado sem atestação técnica de isolamento e sem selo de refinamento.
Root redigiu L0/candidato; `/root/p1331_tests` redigiu testes e oráculos;
este revisor não editou produto, L0, oráculos ou relatório. A reabertura
R1 foi explícita e preservou as evidências, como detalhado abaixo.

Li integralmente o relatório final, SHA-256
`b36ea5065c3d7a88e154bc87bb2e4f35f4025cc5270fb1c638e1394b4f66ca0b`,
e o parecer A/B `p1331-ab-receipt.md`, SHA-256
`d9a698acc2ca5f07df8400738199addad072320c7372508dcf78d3f835bdc5cd`.
Ambos são coerentes com os dados auditados. O parecer A/B limita sua
autoridade à CLI pública; os gates compilados e arquiteturais foram
verificados separadamente por este revisor.

Manifesto ativo: `p1331-manifest-r2.json`, SHA-256
`6383d89ef0fccf78290182c1180fccaba290c1a11f36f4fe44ba75df53de81c9`.
Norma L0: `ef406128181e4cfa5b802b86ec0a9c58370cc4f7dab09365a088eef74523956a`.
Owner sem metadata: `8d7359a9585e354c370c032b573a7133d004806b3a3c84cd881f8665bb992521`.
Binário C2: `a8d6e2f4472fefc9e63a123783feac852445191dcb82ac269d366a6419ae1a47`.
Vanilla ratificado: upstream/main `a51e02804`, executável pinado no manifesto.

## Auditoria reproduzível e proveniência

`p1331-review-final-audit.cjs` terminou exit 0 e produziu
`p1331-review-final-audit.json`, SHA-256
`472038d72cbcad8d764f559e39bbab00ca7897543506ce29e6a70693a17fa197`,
em `2026-09-09T14:17:10.394Z`. O JSON identifica comandos, horários e hashes
dos onze gates finais e dos três recibos CLI; seu snapshot é o after de
GREEN R2, idêntico a before/after de todos os gates finais. Esses recibos
conservam inventário e diff/stat completos sobre HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitada.

Recalculei os hashes de cada arquivo do inventário produtivo atual,
artefatos congelados R1/R2 e evidências históricas do baseline. Todos
coincidem. Fora de calc L0/owner, o produto coincide com o baseline P1330.
HEAD atual e index foram conferidos pelo git no shell; nenhum arquivo
produtivo novo não rastreado apareceu nas camadas ou em prompts.
Nenhum stage, commit ou push foi feito nesta revisão.

## Escopo implementado e cadeia causal

O delta runtime é exclusivamente o corpo do braço `[other]`: nomes longos
locais Str/string e Bool/boolean, demais nomes existentes, mensagem da união
aceita e primeiro value_span posicional. Ausência de origem, ocorrência
vazia/named-only ou value_span detached não fabricam posição. Não muda
aceitação, guard, dispatcher, outro braço de abs, helper ou entidade.
P1328/P1329/P1330 permanecem vigentes, com migração exclusiva dos dois
diagnósticos históricos de string/symbol e preservação do controle sqrt.

R1/C1 não foi fechado: o primeiro teste denso ocultou uma fixture Path
sem resolução. O default de World::resolve_path falhava antes de abs;
atribuir a causa somente a Source::detached seria incorreto. C1 foi
restaurado ao runtime pré-C antes de nova autoria. R2 acrescentou apenas
resolver virtual em memória e sentinela de construção; as sete funções
anteriores e todas as expectativas CLI permaneceram intactas.

A sentinela Path passou isoladamente no próprio World da fixture antes
de C2. Novo RED R2 compilado: 25 passados e seis falhas das obrigações
diagnósticas. C2 reaplicou o mesmo fallback C1, comprovado por hash após
troca apenas do snippet de fixture. GREEN executou exatamente os mesmos
31 testes R2 e passou todos, incluindo a rejeição pública de Path. O caso
não foi removido, desviado ou enfraquecido para produzir sucesso.

## Gates e poder da conclusão

Os onze gates finais terminaram exit 0 sobre o mesmo inventário:
build release/locked; GREEN; workspace release/locked; fmt; diff;
linhagem bidirecional; V5/V15/V26 estritos; linter geral; CLI normal,
repetida e reversa. A soma independente dos resumos workspace é 6.717
passados, zero falhas e três ignorados; GREEN focal não foi somado de novo.

Em cada ordem, as 504 chaves caso/perfil são únicas e completas. Recalculei
exit/stdout/stderr de todas as saídas candidatas contra o oráculo, sem
confiar no booleano do runner: 1.512 comparações literais aprovadas.
Também conferi as saídas BASE/vanilla/C2 por chave entre ordens e a reversão
real dos casos. Permanecem 324 células históricas literais, oito migrações
string/symbol e 172 células novas. R2 não alterou nenhuma expectativa R1.

Por execução, igualdade integral com vanilla cresce de 252 para 404
observações, ganho 152 sem regressão de igualdade anterior. C2 altera 164;
as outras 12 mudanças preservam a dívida congelada do nome externo no trace.
Essas 12 não foram contabilizadas como igualdade integral. As 100 diferenças
restantes são dívidas classificadas do corpus, não Unknown convertido em
sucesso. O relatório não extrapola essas contagens para a linguagem inteira.

O SARIF tem zero erros, 240 warnings e 1.146 notes. A comparação por
multiconjunto regra/nível/mensagem/arquivo, desconsiderando deslocamento de
linhas, não remove nenhum achado anterior; adiciona somente uma note V16
pela delegação a other.type_name no fallback. Não é linter sem achados.
Os checks V5/V15/V26 selecionados encerram sem violação.

## Limitações mantidas

O aceite cobre diagnóstico e origem de tipos já rejeitados, com semântica
e preservações declaradas. Permanecem fora do recorte nomes de traces no
dispatcher, precedência named/aridade, parser do literal mínimo, construção
NaN dimensional, operações anteriores e outras funções. Path sem argumento
falha no construtor e não é testemunha de abs. Location nativa testa valor
já construído, sem alegar cobertura de produção introspectiva ou mudar fase.

Não identifiquei Unknown obrigatório pendente dentro do fragmento aceito.
Os limites anteriores são explícitos e não ampliam a aprovação. O relatório
preserva a tentativa R1, a correção causal da fixture e a cadeia válida R2;
os hashes e vereditos permitem registrar o fechamento escopado agora.
