# P1339 — contrato revisão 2: fases de prova e preservação causal

Autor `/root/p1316_review`, apenas contrato; regime completo, executado sem
atestação de isolamento. Contexto herdado P1316 e autoria P1339 r1, nenhum
candidato P1339 lido. Nenhuma edição de L0, produto, oráculos ou veredito.

## Medição que provoca a revisão

O contrato r1 `p1339-contract.json`, SHA-256
`e062fb551fcffd78d21c1ae2a0a185aa376dd790c3e0825cec045a7f1e65b509`,
permanece intacto. O diagnóstico independente
`p1339-verifier-phase-precondition-r1.md`, SHA-256
`91b68a01f3d58ef33ceebdabe30d378776ece2ed964cc2cdccd9c563bd6b1dcb`,
identifica a circularidade: `closed_state` exigia observação compilada de
APIs futuras antes do selo que, por sua vez, antecede o código dessas APIs.
O intake do verificador é `p1339-verifier-intake-r1.json`, SHA-256
`f0c4e210c7717a453294419370bc99d01f7c57ed4ee05daedb3cf0e291d361e9`.
Essa é evidência de desenho, não uma execução de discriminação.

O L0 congelado `compiler/eval.md:343-355,418-430` mede ausência de registro
e especifica os três métodos futuros. `:499-514` exige fixar cobertura e
precondições antes do selo, mas mantém código posterior ao selo/RED.
`:527-563` proíbe certificar Same por incompletude e bloqueia caso obrigatório
real Unproven. `infra/pipeline.md:763-772` exige nucleação/completude e
simultaneamente contrato/selo/RED antes de código. `entities/selector.md:299`
e `entities/element_payload.md:359` distinguem testes de dados e aceitação
pública posterior. Esses textos não fornecem uma implementação presente.

Interpretação explícita: a demonstração prévia de completude é a cobertura
integral de categorias, fixtures, predicados, recursos e operações cuja
viabilidade se fundamenta nos carriers/baseline medidos; não pode afirmar
que a implementação futura já preserva essas entradas. A prova executada
dessa preservação é gate final obrigatório. Se o mapa prévio deixar uma
categoria ou condição de observabilidade sem teste definido, o selo continua
bloqueado. Não se substitui prova por promessa genérica de testes posteriores.

O passo autorizado, Fase C, requer vinte mutantes reais compiláveis e score
1.0; Fases D/E colocam RED e selo antes do candidato; Fase F requer execução
dos testes independentes e arquitetura. A skill/referência de gates fixa a
mesma cadeia. A distinção de fase satisfaz essas condições sem reduzir
qualquer mutante, obrigação final ou política de Unknown.

## Revisão dos gates, não das obrigações

Antes do selo devem existir fixtures e predicados independentes completos
para toda obrigação, inclusive as privadas. Cada entrada identifica qual
operação/carrier real será observado, construção dos dados, resultado
exigido, controle discriminante, comando ou harness, fase e recursos.
Quando a API só existe no candidato, congelar o teste independente e sua
interface de ligação local a testes: entradas e asserts não podem ser
redigidos depois de ler o candidato. Uma ligação mecânica futura pode somente
resolver símbolos/configuração pré-declarados, sem calcular resultados ou
substituir a semântica sob teste. O verificador inspeciona a ligação.

Na Fase C executam-se todos os positivos/opacos públicos e os vinte mutantes
sobre programas e carriers disponíveis na referência/baseline reais. M12
continua exigindo fonte/grafo e compilação efetiva; M20 exige execução real
e preservação de Unknown bruto. Uma API futura não recebe um mock positivo,
RED por falha de compilação, Preserved ou crédito de cobertura executada.

O bool público `context_reads_valid_for == false` não distingue Different
de Unproven e não pode ser renomeado Unknown. O caso opaco interno exige
teste local ao owner que observe a variante real do comparador privado,
com ligação test-only congelada e auditoria estrutural. Não adiciona API
pública nem implementa um segundo comparador. O teste público separado
exige apenas não certificar reutilização; são duas evidências distintas.

O ledger da cobertura futura registra `NotDue` somente como estado de
agendamento de uma obrigação final, nunca como classificação de observação.
Se uma observação obrigatória devida na fase atual não puder ser obtida, é
Unknown e bloqueia. Nenhum dos vinte mutantes pode ser movido a NotDue.

Na Fase F, todos os testes congelados de carriers aprovados, observações
Same/Different/Unproven, recursos, projeções, retenção e arquitetura devem
executar contra o candidato real. Ausência de símbolo, falha de compilação,
adapter inconclusivo, caso omitido ou Unknown obrigatório impede fechamento.
Não há crédito final por cobertura da referência, revisão de texto ou testes
que só repetem os valores esperados em JSON.

## Correção causal dos controles mistos de ocorrência

A fonte pinada `p1339-where-occurrence-probe-runs.json`, caso
`text_style_controls`, contém estilos visuais e um nó `strong` semântico,
seguidos por query/where novos. `set_strong_delta` e `explicit_delta` também
misturam produtores fora do recorte e consumidores novos. A política r1 que
preservava a saída inteira poderia congelar um erro do consumidor justamente
autorizado a mudar. Isso contradiz W02, não representa uma preservação útil.

R2 exige decomposição causal independente antes do selo: preservar o produtor
isolado conforme baseline; testar o consumidor autorizado, com produtor
suportado/default, conforme vanilla; conservar a medição mista original e
mapear explicitamente cada projeção e delta permitido. Para
`text_style_controls`, estilo sozinho não cria ocorrência, mas o `strong`
real cria exatamente sua ocorrência sem duplicação pelo estilo envolvente.
Não existe licença para reparar constructor/set delta ou aceitar silenciosamente
qualquer nova saída mista. Um efeito inseparável sem predicado prévio bloqueia
o selo. `constructor_projection` continua preservação integral do baseline.

## Rendimento e limites

Revisão contratual 2 de no máximo 3; zero execuções completas realizadas por
este autor. O diagnóstico informa que nenhum selo/run completo havia sido
emitido; não reivindicamos resultados dos focais de terceiros. Hipótese:
remover a circularidade de fase sem perder cobertura e decompor a preservação
de produtores sem congelar a ausência dos novos consumidores. Refutação:
qualquer cláusula sem teste pré-definido, mutante retirado do gate C, teste
interno adaptado ao candidato ou preservação fora do lote relaxada.

A revisão só torna esses critérios auditáveis. Não afirma ganho empírico,
score, RED, GREEN ou selo. O verificador deve primeiro auditar o delta focal;
repetição do corpus completo continua subordinada ao budget congelado.

Manifesto r2 e suplemento prospectivo de implementadores permanecem iguais.
O futuro selo deve apontar o sucessor `p1339-contract-r2.json` e o predecessor,
sem reescrever o contrato r1 nem o manifesto histórico. Todos os hashes de
entrada/saída são registrados no contrato e recibo r2.
