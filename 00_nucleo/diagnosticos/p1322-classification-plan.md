# P1322 — preparação do classificador C

Regime: auditoria com autorias separadas, executada sem atestação técnica de isolamento. O classificador escreve somente `00_nucleo/diagnosticos/p1322-classification-*`; não escreve produto, L0, catálogo, medição bilateral nem seu próprio veredito. O revisor julga estes artefatos separadamente.

Entradas normativas lidas: passo `00_nucleo/materialization/typst-passo-1322.md`, skill Tekt e suas duas referências, ADR-0107/0108/0127/0129. Não foram consultados outros passos nem context. Inputs históricos: relatório global P1309, fontes/scripts/ledger de classificação P1309, diagnósticos finais P1310–P1321. A leitura inicial de `p1313-final-report.md` foi complementada por `p1313-implementation-report.md`: o primeiro é somente proposta histórica; o segundo é a implementação autorizada.

## Reconciliação a testar novamente

| Passo | Alegação histórica delimitada | Fronteira a preservar na decisão nova |
| --- | --- | --- |
| P1310 | Tipo público e origem do valor inválido nos cinco decoders | Symbol, missing, named/excesso e I/O não foram fechados |
| P1311 | Campo ausente em função nativa preserva identidade e marca o campo | Ausência bilateral de csv/xml/read.encode não é membro faltante |
| P1312 | Cast de read com origem; Bytes rejeitado corretamente | Antiga coorte conjunta read/CSV era falsa: CSV recebe DataSource |
| P1313 | CSV aceita Bytes e corrige cast inválido | Usar relatório de implementação, não relatório de proposta; Symbol continua divergente |
| P1314 | Todas ocorrências de opções são validadas e apontam ao valor | Duplicata sintática é estrato diferente; coerção Symbol não foi feita |
| P1315 | Ordinal de registro do erro CSV | Número correto sozinho não fecha diagnóstico completo |
| P1316 | Erro de parsing Bytes marca origem do argumento | Arquivo UTF-8 válido exige origem externa |
| P1317 | Causa Utf8 pública | Não antecipar Utf8 sobre erro anterior de campos |
| P1318 | Posição textual nos dados Bytes | Conversão CR/LF/Unicode/binário e origem do argumento são observáveis distintos |
| P1319 | Arquivo de buffer inválido identifica caminho virtual e origem | Arquivo de buffer válido, I/O e origem causal Str não foram fechados |
| P1320 | Auditoria focal com lacunas transversais | Retificação csv.encode; não há denominador global naquela matriz |
| P1321 | Fonte/casts → opções → primeiro remanescente → leitura/parser; missing/named/spans | Não quita I/O; missing completo exigiu loading + call_dispatch; math direto pode falhar antes de CSV |

As alegações desta tabela são históricas, não resultados P1322. Cada confirmação atual exige testemunha nova e íntegra; diferenças previstas contra oráculo antigo não viram regressões automaticamente. Mudanças de fixture/adapter/cwd também exigem isolamento causal.

## Método de decisão congelado antes das matrizes

1. Autenticar catálogo, manifestos, normal/repeat/reverse e suplemento por hash; recompor pelas chaves probe/perfil; Unknown e instabilidade obrigatórios bloqueiam seleção.
2. Classificar primeiro a observação de linguagem, depois a hipótese causal ancorada em fonte atual `file:line`, L0/consumer, intenção documentada e condição de refutação. Igualdade de lookup/repr não fecha chamadas não medidas. Sem atribuir intenção a extras calc por comportamento.
3. Publicar transição de todos os IDs P1309 sem sobrescrever histórico; registrar separadamente a reconciliação funcional P1310–P1321.
4. Formar coortes somente por causa demonstrada. Prioridade: regressão reproduzível; contradição específica L0 em rota canônica; diagnóstico isolado; valor/kind/identidade/repr; membro ausente com carriers existentes; membro ausente com contrato/entidade/fase nova; restante sem rota canônica demonstrada. Desempate: menos owners do observável completo, mais paths da mesma causa, menor superfície de regressão demonstrada, ID lexical. Risco desconhecido não é risco baixo.
5. Manter dívida de certificação separada da língua e dos ataques ao auditor. Nenhum mutante de produto será executado por este papel. P1308 não quita automaticamente F01–F20/S01–S12/A01–A05.
6. Recomendar uma única coorte elegível para P1323, ou registrar impedimento explícito; nenhuma implementação é autorizada pela seleção. Revisão de contagens/seleção pertence a outro papel.

Status desta preparação: aguarda catálogo, matrizes e sentinelas atuais. Nenhuma classificação atual nem vencedor inferido do histórico.
