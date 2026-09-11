# P1340 — sucessor R5 dos inputs terminais

Proposta congelada para revisão independente; não é selo. Autor do contrato:
`/root/p1340_contract_correction`, com protocolo completo da skill
tekt-materializacao-segregada e ambas as referências já lidas. Regime:
executado sem atestação de isolamento. O autor não lê candidato, adaptador
terminal ou saídas do focal candidato, e não escreve o harness sucessor.

## Evidência e intenção

`p1340-terminal-source-authority-r5.json` registra a autorização humana
`Autorizo`, limitada às fontes T01/T02/T03 e à revalidação focal independente.
O diagnóstico R4 fixa os textos originais e seus hashes e registra erro de
join entre array e content no vanilla ratificado. O gate terminal R4 localiza
a falha na validade dos inputs, antes da aceitação da política terminal.
Este contrato recebe somente esses artefactos autorizados; não reproduziu
medições nem inferiu a correção de uma implementação candidata.

O L0 vigente `infra/pipeline/context_stabilization.md:74-83,95-110,159-171`
seleciona por demanda realmente alcançada e preserva os recursos causais.
`:220-250` mantém o erro terminal exato após tentativas permitidas, a precedência
dos erros originais, os sinks e a distinção entre Different e Unproven.
Nenhuma dessas obrigações muda no R5. O objetivo é que o descarte de um valor
de leitura deixe de participar do join de saída, sem eliminar a própria leitura.

## Única transformação permitida

O autor do oráculo copia byte a byte o harness R2 e modifica somente os
source literals de OpaqueAtCeiling, MixedDifferentAndOpaque e
QueryDifferentThenStable. Nos statements indicados abaixo, substituir
`expressão;` por `let _ = expressão;`. RHS, argumentos, ordem, delimitadores
das operações restantes e todo o restante da fonte ficam idênticos.

| Caso | Statements cujo resultado é descartado | Contribuição final preservada |
|---|---|---|
| T01 OpaqueAtCeiling | c.final(); depois query(<opaque>); | [] |
| T02 MixedDifferentAndOpaque | query(<opaque>); | c.update(c.final().first() + 1) |
| T03 QueryDifferentThenStable | c.final(); | metadata(query(<made>).len()) e produtor contextual Metadata(1) |

O pattern `_` deve ser um descarte eager: cada RHS executa exatamente uma
vez, antes da operação seguinte, e qualquer Err continua a propagar. Não
introduzir thunk, função, nova scope contextual, branch, antecipação de
consulta, retry, binding lido depois, assertion, conversão ou resultado
fabricado. Se o parser/eval ratificado não cumprir essa propriedade, parar
dentro do orçamento; não trocar silenciosamente a intenção ou o outcome.

T01 conserva a construção counter(heading.where()), a demanda final seguida
de query(<opaque>) e [] como saída do corpo. T02 conserva query(<opaque>)
antes da demanda final, seu .first(), a soma de 1 e a construção do update.
T03 conserva final antes de query(<made>), .len() e metadata, sem mudar o
segundo bloco produtor. O registro das operações reais e seus resultados
integrais continua obrigatório mesmo quando o resultado é descartado pelo
statement; descarte da saída não autoriza descarte da dependência.

## Inputs opacos, resultados e invariantes protegidos

Os Metadata(0)<opaque> e a fronteira post-eval/pre-discovery de injeção
continuam idênticos. O mesmo Func::element real, cujo constructor entra em
panic se chamado, é colocado somente no input Metadata. O binding deve
observar query retornando esse carrier real, sem substituir retornos,
registros, relações, bools ou resultados de corpo. O R5 não muda o binding.

Todos os bytes fora dos três literals modificados são idênticos ao harness
R2: enum/input/observation, sentinel warnings, carrier opaco, asserts,
ordens, condições e métodos de diagnóstico. Logo T04/T05/T06, suas fontes,
expectativas e todos os expected outcomes permanecem byte-idênticos. O
verificador reconstrói o R2 por inversão das três substituições e exige
igualdade integral com o original pinado; não aceita apenas diff semântico.

T01/T02 continuam a exigir corpos selecionados Ok, A1..A5, o único Error
`contextual stability could not be verified` em root span da Source real,
sem hints/trace, zero exportações e warnings GLOBAL/RETAINED nessa ordem.
T03 continua a exigir sucesso após pelo menos duas tentativas e metadata
ordenada [1,1], conservando os mesmos warnings. O texto dos assertions
permanece no R2; este resumo não o substitui. Os novos offsets das leituras
decorrem da fonte nova e não permitem editar expectativas para mascarar erro.

## Gates, orçamento e entrega

Uma revisão de oráculo; no máximo duas iterações focais; nenhum corpus ou
suíte terminal completa antes de o recorte focal passar; teto de 45 minutos.
Primeiro verificar integridade das substituições e executar as três fontes
no vanilla ratificado a51e02804 sem transformação opaca, registrando bytes,
comandos, binário pinado, stdout/stderr/exit e artefactos. Warning esperado
de não convergência em T02 não é erro de validade da fonte. Essa etapa
verifica a precondição de linguagem; não prova opacidade ou política terminal.

Separadamente, o verificador confere a preservação/eagerness/ordem das
operações e o novo harness, antes de selar sua ligação prospectiva. Depois
do selo, o executor autorizado pode trocar somente o include do adaptador
para o sucessor e executar os focais reais T01/T02/T03 com os mesmos inputs
opacos e expected outcomes. Somente essa execução pode demonstrar a condição
terminal pretendida. Não contar compilação, ausência de erro lexical, leitura
AST ou vanilla sem carrier opaco como sucesso terminal produtivo.

Uncertainty sobre leitura removida, ordem, resultado ou condição terminal é
Unknown e bloqueia aceitação. Erro de linguagem alheio persistente, mesma
causa em duas iterações, necessidade de mudança de compilador ou mudança
de resultado esperado ativa a parada. Não alterar política terminal, R4,
lifecycle, fontes históricas ou dar fechamento global. Recibos distinguem
autoria, validade de fonte, selo e resultado focal real, com delta e custo.
