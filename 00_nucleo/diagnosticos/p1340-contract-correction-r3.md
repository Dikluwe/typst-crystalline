# P1340 — correção contratual R3 anterior à implementação

Autor independente `/root/p1340_contract_correction`, papel de contrato/testes;
executado sem atestação de isolamento. Aplica o protocolo completo da skill
tekt-materializacao-segregada e ambas as referências. Não escreve solução nem
veredito. A autoridade limita as escritas a `p1340-contract-correction-*`.
Não houve leitura de candidato P1340, materialization ou context. Não foi
encontrada ADR de materialização segregada na busca dirigida em `adr/`.

## Evidência antes da decisão

O predecessor congelado é `p1340-contract-r2.json`, SHA-256
`9ea5ecc71e26e1ee707fe4eecdf6d5cd2768f2e2be8cd6e6d0605b1ef38ae434`.
As fontes de teste continuam exatamente as do lifecycle typed-r3 histórico,
SHA-256 `6d8c257997631133fb1a7a55816daaa50e1633c129fafd17f6d1ddeb378ca541`.
Proveniência: HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, working tree
não commitado; a medição inicial foi feita em 2026-09-10T13:53:29Z. O JSON
companheiro conserva o diff stat integral e os hashes das entradas.

`compiler/eval.md:382-412` exige sequência tipada de operações reais e fold
completo, incluindo erro posterior mesmo quando get devolve apenas prefixo.
O owner existente `stdlib/counter.rs:326-333` registra CounterFold como
`Ok([])` ou `Ok([[Location, counter_values], ...])`; `:385-388` registra
CounterFinal separadamente como vetor de valores, `[0]` quando não há eventos.
As fontes fixas dos três casos stable e do caso request-chain-points contêm
uma chamada final por corpo estável, sem evento que case com seu contador.
O v3 histórico `:35,53` exige `Ok([0])` para toda requisição desses corpos.
Esse predicado mistura uma dependência interna com uma projeção da linguagem.

`compiler/eval.md:398-406,441-445` exige completude para validação positiva;
uma observação alterada produz false, e incompletude nunca produz true.
`:527-530` distingue Same, Different e Unproven. A base existente em
`eval/mod.rs:5796-5804` visita registros em ordem e interrompe no primeiro
resultado diferente de Same. O v2 histórico exige replay integral inclusive
quando a validação devolve false.

## Classificação e correção mínima

O requisito universal de zero é malformado face ao L0. Substituí-lo por um
par exato `CounterFold -> Ok([]), CounterFinal -> Ok([0])` nos quatro corpos
fixos preserva tanto a dependência quanto o valor público, e rejeita omissão
do fold, projeção apagada, payload adulterado ou ordem trocada. Não se usa
essa forma vazia para folds dos contadores que mudam: seus eventos completos,
Location, valores e erros continuam no transcript sem transformação.

Inferência explícita: o L0 autoriza fail-fast negativo porque uma testemunha
nonSame já refuta a conjunção necessária à reutilização; avaliar o restante
não pode tornar a validação positiva. O texto não impõe execução de todos
os replays depois dessa refutação. Uma cláusula que exija efeitos observáveis
dos replays posteriores refutaria esta inferência; não foi encontrada.
Isso preserva a exigência normativa de registro completo do corpo inteiro.

R3 exige: true implica replay integral, na ordem original, e todos Same;
false implica prefixo não vazio exato, Same antes da última entrada, e
Different ou Unproven real na última. Não se permite apagar registro original,
inventar replay, inferir Different de um booleano ou converter Unproven em
sucesso. Erro do mecanismo exige vetor diagnóstico integral e testemunha
real `mechanism_error`, distinta de um Err da leitura comparado pelo owner.

O campo adicional `relation` de RequestReplay é projeção passiva da relação
privada efetivamente calculada no ponto de validação. Preserva `result` inteiro.
Não pode ser preenchido a partir de expected, igualdade JSON, Debug ou do
booleano final. A ligação tardia exige auditoria independente do ponto de
observação. Isso não exige API pública ou alteração semântica produtiva.

## Herança e precedência

Esta é a única revisão semântica R3 restante. Os originais são imutáveis.
Executar `p1340-contract-correction-predicate-r3.py`, que chama a cópia
explícita `p1340-contract-correction-predicate-v2-r3.py` e o v1 histórico.
Somente as duas cláusulas identificadas são sucedidas. Todas as demais
assertivas v1/v2/v3, fontes, perfis, ordens, capturas, retenção, descendentes,
recursos, erros, sinks e testemunhas estruturais permanecem obrigatórias.
O predecessor R2, seus testes terminais T01–T06, sua política Unknown e seus
casos públicos permanecem integrais. A extensão DTO acima é obrigatória;
dados ausentes são falha de observabilidade, nunca Preserved.

## Gates e limite

O verificador precisa aceitar esta sucessão e auditar as projeções antes do
selo. Calibração focal deve rejeitar true truncado, false sem testemunha,
ordem/duplicação errada, Unproven positivo, fold apagado ou convertido em
zero, resultado/erro de replay ausente e payload final diferente de zero.
Controles incluem par lossless e validação positiva completa/negativa com
prefixo. Esses são testes do predicado, não execução de produto nem prova de
mutação produtiva. Não se abre outra matriz. As obrigações de produto e os
ataques do R2 continuam pendentes do verificador; este autor não emite selo.
