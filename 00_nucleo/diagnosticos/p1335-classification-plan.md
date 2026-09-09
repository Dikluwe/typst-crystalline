# P1335 — plano do papel de classificação

Regime: auditoria com inventário, operação, classificação e revisão separados,
executada sem atestação técnica de isolamento. Executor `/root/p1335_classifier`;
filesystem compartilhado, contexto recebido limitado ao mandato de classificação.
Escrita restrita a novos `p1335-classification-*`; não escreve produto, L0,
testes produtivos, mutantes, medições bilaterais ou veredito dos próprios ledgers.

Foram lidos integralmente o passo autorizado P1335, a skill
tekt-materializacao-segregada e suas duas referências, ADR-0107/0108/0127/0129,
os relatórios finais P1323–P1334 e a análise final P1322. Não se consultaram
outros passos/context. Não foi localizada ADR local que nomeie materialização
segregada na pesquisa textual específica da pasta adr.

## Entradas e ordem causal

1. Pinar relatórios/closures e artefatos históricos efetivamente usados;
   extrair fechamento alegado, fronteira e sentinela sem assumir estado atual.
2. Receber baseline/manifesto P1335 e catálogo global sucessor. Exigir
   reconciliação por cada ID P1322, separando universo principal/suplementar.
3. Receber três matrizes e suplementos atuais. Comparar exit/stdout/stderr
   completos, flags e identidade dos executáveis; nunca promover projeção JSON,
   DOM ou mensagem primária a igualdade integral. Unknown bloqueia a decisão
   quando afeta observável obrigatório. Repetições não entram no denominador.
4. Registrar transições por ID/perfil. MATCH histórico divergente só vira
   regressão após isolar fixture, localização, adapter e ambiente. Relatórios
   anteriores e sua conservação de contratos não substituem vanilla atual.
5. Para cada causa candidata, ler integralmente L0s atuais, conferir owners,
   pins e fontes com linhas; registrar hipótese, refutação, observável e gate.
   Presença/kind/repr não fecham comportamentos de chamada não executados.
6. Ordenar coortes por: regressão reproduzível; contradição L0 canônica;
   diagnóstico isolado; valor/kind/identidade/repr; membro ausente com carriers;
   membro que exige contrato/entidade/fase; restante. Desempate: menos owners
   necessários ao observável completo, mais paths de mesma causa demonstrada,
   menor risco demonstrado, ID lexical. Risco Unknown não é baixo.
7. Entregar tabela/seleção e dívida de certificação à revisão separada. A
   classificação não fornece seu próprio veredito de contagens/seleção.

## Correções metodológicas herdadas como regra, não resultado

P1322 final escolheu hints HTML após revisão de substância; a decisão inicial
Some não é vigente. Uma promessa específica de diagnóstico vanilla abrange
seus hints mesmo sem transcrevê-los. Não exigir strings literais no L0 para
reconhecer contradição. A suficiência de um owner para math.join foi depois
refutada em P1325: um módulo de arquivo math.typ também pode ter nome math e
conteúdo ausente. Sua elegibilidade histórica não pode ser copiada.

O leitor TSV usa tab como separador literal; aspas das expressões são conteúdo.
Não contar linhas suplementares como probes principais. Não herdar contagens,
elegibilidade, vencedor ou refs de linha como medição nova. Extensão documentada
é diferença intencional, sem igualdade bruta; só extensão fundamentada em L0
atual pode ajustar denominador. Omissão intencional de warning não é extensão.

Sem recomendação antecipada. A falta da matriz/suplementos atuais é estado
pendente de integração, não prova de fechamento nem blocker definitivo.
