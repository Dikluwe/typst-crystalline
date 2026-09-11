# P1340 — terminal R5 validado; lifecycle requer redesenho protocolar

## Resultado atual

A sessão privada de estabilização continua implementada parcialmente. Nesta
continuação, a autorização corrigiu exclusivamente as fontes inválidas
T01/T02/T03 do oráculo terminal, sem mudar política, expectativas, sinks,
inputs opacos ou comportamento produtivo. O adapter somente de teste passou
a consumir o sucessor R5 selado.

O compilador real executou **18 observações**: seis casos em ordem normal,
repetida e inversa. Todas passaram. O verificador independente classificou
o resultado como `PASS_TERMINAL_R5_CANDIDATE_FRAGMENT_ONLY`, com zero
`Unknown` nesse fragmento.

**P1340 permanece incompleto.** R4/lifecycle e NT01–NT06 continuam pendentes;
não houve aceitação F, fechamento global do P1339 ou commit.

Uma tentativa posterior de fechar R4/lifecycle dentro do mesmo passo produziu
R6 e R6b, sem ler nem executar o candidato. O veredito independente final foi
`NO_SEAL_STOP_REQUIRES_PROTOCOL_REDESIGN`: o orçamento contratual 2/2 foi
consumido e o gate F não pode ser iniciado com esse contrato.

## Correção delimitada das três fontes inválidas

O harness R2 exige que seus casos opacos terminem com corpos sem erro
próprio. Entretanto, as expressões intermediárias dos blocos produzem
arrays que depois são unidos a Content. O ponto e vírgula não descarta
esses resultados.

| Fonte congelada | Medição no vanilla ratificado |
|---|---|
| T01 — OpaqueAtCeiling | [erro de junção em 3:39](p1340-terminal-source-vanilla-r4.json) |
| T02 — MixedDifferentAndOpaque | [erro de junção em 3:28](p1340-terminal-mixed-vanilla-r4.json) |
| T03 — QueryDifferentThenStable | [erro de junção em 2:22](p1340-terminal-query-vanilla-r4.json) |

Os três retornavam `cannot join array with content`. T01 também falhava com
a mesma fonte no [binário preservado anterior ao P1340](p1340-terminal-source-baseline-r4.json).
O [diagnóstico reproduzível](p1340-terminal-input-diagnosis-r4.json) conserva
as fontes integrais, comprova sua igualdade com os literais do harness e
registra hashes dos binários e recibos. A referência é upstream
`a51e02804`, não uma string de versão.

Na [execução instrumentada real de T01](p1340-terminal-binding-focal-r4.json),
ocorreram cinco tentativas, a query observou o carrier opaco real e o
compilador preservou o erro original. A expectativa `selected_body_ok`
falhou. Os warnings foram exatamente GLOBAL e RETAINED; nenhuma sentinela
de descarte vazou e não ocorreu exportação.

A [autoridade R5](p1340-terminal-source-authority-r5.json) congelou a única
mudança permitida: quatro prefixos `let _ =` em T01–T03 para avaliar uma vez,
na mesma ordem, propagar erros e apenas descartar a contribuição à junção.
O [contrato R5](p1340-terminal-contract-r5.json) e seu
[verificador estático](p1340-terminal-contract-r5.py) provam a transformação
e a inversão byte a byte para o harness R2. T04–T06 e todas as expectativas
permaneceram idênticos.

Um primeiro autor de oráculo abriu por engano o baseline que continha o
candidato e foi descartado antes de produzir artefato. A skill de segregação
levou à substituição por um autor fresco; a delegação e a contaminação estão
registradas, não ocultadas. O autor substituto mediu T01–T03 somente no
vanilla ratificado e produziu o [harness R5](p1340-contract-terminal-harness-r5.rs).

## O que foi entregue nesta rodada

- Inventário [factual de interfaces L1](p1340-interface-inventory-r4.md),
  sem alterações nesses owners. Distingue Func de sua origem de chamada,
  identidade de recurso de ocorrência e span ausente de origem perdida.
- [Revisão contratual R4](p1340-contract-successor-r4.json): distingue
  features reais das solicitadas, exige origem causal de callbacks/eventos
  e propõe Dict com representação não ambígua. Documento e predicados
  permanecem **não selados e não executados**; não legitimam novas portas.
- [Adapter terminal](p1340-implementation-pipeline-observer.rs) ligado por
  hooks condicionais em pipeline e seu módulo filho. Usa o compilador real,
  inputs opacos autorizados, sinks reais e captura de erros antes da decisão.
  Não implementa outro ciclo de estabilização ou um DTO lifecycle fictício.
- [Selo independente de validade do input R5](p1340-verifier-terminal-r5-seal.json):
  45 rejeições de mutantes em ordens normal/repetida/inversa, score 1,0,
  três checagens positivas byte a byte e zero `Unknown` obrigatório.
- [Execução focal do candidato](p1340-terminal-r5-candidate-focal.json): um
  teste Rust passou e realizou as 18 observações reais previstas.
- [Veredito independente do fragmento](p1340-verifier-terminal-r5-candidate-verdict.json):
  `PASS_TERMINAL_R5_CANDIDATE_FRAGMENT_ONLY`; não concede F/global PASS.
- [Prova inversa dos hooks](p1340-terminal-final-inverse-r4.json):
  removê-los recupera exatamente os dois fontes anteriores à instrumentação.
  É prova estática delimitada, não equivalência geral.

O teste de topologia impossível continua sendo uma unidade do ramo terminal,
não prova de detecção de colisão válida. Identidades/origens do lifecycle
continuam sem binding completo; nenhum grant adicional L1 foi emitido.

## Verificações e proveniência

HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, **working tree não
commitado**. Os recibos conservam UTC, comandos, saídas completas, diff stat
e hashes antes/depois. O [estado local final](p1340-r4-final-local-state.json)
pina fontes, adapter integral e evidências; o [lint final](p1340-terminal-final-lint-r4b.json)
também congela o diff binário e novos fontes/L0.

| Verificação | Resultado |
|---|---|
| [Controles da pipeline com observer inativo](p1340-terminal-inactive-controls-r4.json) | 15 passaram |
| [Build normal do workspace](p1340-terminal-final-normal-build-r4.json) | exit 0 |
| [Check dos testes com instrumentação atual](p1340-terminal-final-cfg-check-r4b.json) | exit 0 |
| [Ownership V15/V26](p1340-r4-ownership-check.json) | exit 0 |
| [Contrato estático R5 final](p1340-terminal-r5-final-static.json) | exit 0; fontes congeladas intactas |
| [Fragmento terminal R5 no candidato](p1340-terminal-r5-candidate-focal.json) | 18/18 observações passaram |
| [Lint final R5](p1340-terminal-r5-final-lint.json) | exit 0; 0 erros, 251 warnings |

Não é zero violations. A [primeira checagem desta rodada](p1340-terminal-final-lint-r4.json)
detectou um novo wildcard no adapter; ele foi substituído por cobertura
nominal das variantes JSON. Os dois warnings anteriores das projeções
parciais L1 permanecem e não são atribuídos ao legado.

O focal R5 foi executado depois da troca do include e registrou o hash do
binário resultante sem transformá-lo em pin retroativo. `git diff --check` e
rustfmt dos dois fontes L3 e do adapter passaram. O harness R5 não foi
reformatado: sua representação byte a byte é parte do contrato selado e o
checker estático confirma seu hash e inversão. Somente os campos derivados
`Hash do Código` dos dois L0 haviam sido atualizados; o texto normativo não
mudou. Os pins históricos/R4 não foram reescritos.

A implementação anterior e seus RED→GREENs permanecem documentados nos
[15 focais anteriores](p1340-final-pipeline-tests-r1.json),
[RED de revisão](p1340-review-red-r1.json) e
[GREEN de revisão](p1340-review-green-attempt-r1.json).
Continuam abertas as matrizes públicas/lifecycle/terminais e seus mutantes,
assim como os riscos de seleção exclusivamente descendente e colisões
selecionadas não demonstrados por esses controles.

## Decisão e pendências reais

A autorização posterior cobriu somente a revisão R5 das fontes T01/T02/T03.
Essa revisão está concluída e aprovada dentro de seu fragmento. A
[política terminal](p1340-terminal-policy-approval.json) foi preservada; não
se mudou a semântica de junção do compilador.

O veredito R5 não cobre os ataques terminais NT01–NT06, nem resolve o contrato
R4/lifecycle `NOT_SEALED`: proveniência de invocação, identidade de ocorrência,
spans causais e a matriz pública/lifecycle continuam sem binding e sem testes
finais. O teste T05 prova a operação do ramo terminal real, não descoberta de
topologia impossível. Essas obrigações precisam de cadeia própria antes de
qualquer F ou declaração de paridade. Regime desta execução:
**executado sem atestação de isolamento**.

Proveniência dos fechos R5, no HEAD acima e na working tree não commitada:

- selo do input: SHA-256
  `999b397fbb17a5c7cde188adda2d3591152bc091b03452676ea2ade2a9ee5878`;
- recibo focal: SHA-256
  `e9ce7ab13bca0669a43c388a2aa6d13b936a716e7e04b5fba0d700f8e6eef384`;
- veredito independente: SHA-256
  `4a8533a98129d2c527e18ef9075535ba7ece8ccf498291a488d16f8634196683`;
- check estático final: SHA-256
  `e81d4d5ba29265fa540fe9c41f9ddc78209c76bb73ae9f5b63282c5bb4d06226`;
- lint final: SHA-256
  `3b47f8ca1d335aab8e939d968ba94eaad733999c9d490f85450ca6f7ab9e335a`.

## Tentativa de fechamento R6/R6b e condição de parada

A resposta “Faça o que precisa para finalizar o 1340 ou precisa de outro
passo?” autorizou uma cadeia segregada de fechamento, preservando o gate
ADR-0127. A [autoridade R6](p1340-closing-authority-r6.json) concluiu que um
novo número de passo não era necessário enquanto o contrato fosse apenas uma
continuação executável de R4. Uma auditoria de feasibility confirmou que
instrumentação `cfg(test)` seria suficiente e não exigiria API, default,
compatibilidade ou mudança de fase.

O primeiro contrato R6 foi rejeitado antes do candidato por três causas:
herança R4/R3 incompleta, `Unknown` aceitando ledger malformado e cobertura
autodeclarada aceitando omissões coerentes. R6b corrigiu esses três findings,
matou 12/12 mutantes focais correspondentes e consumiu a segunda revisão.

O gate adversarial amplo de R6b encontrou **22 sobreviventes em 41 mutantes**:
score `19/41 = 0,4634146341`, reproduzido em ordem normal, repetida e inversa.
As oito classes novas abrangem âncora real de features, origem/carrier de Func,
chave/ação e ocorrência de counter, papel do span, aciclicidade global da
linhagem, payload recursivo de Dict e curto-circuito semântico de `Unknown`.
Recibo: [resultado adversarial R6b](p1340-adversary-result-r6b.json), SHA-256
`b62b3be96d8148d5d739152ac94d143f39e1bce514cd2f55bc3a6a39bdef81a0`.

Além disso, o autor independente das expectativas classificou
`--r6-expectations` como `NOT_AUTHORABLE_EX_ANTE`: o contrato pede
`callsite_witness_id`, `value_id`, `snapshot_id`, `location` e `span` dinâmicos
numa única entrada por caso, mas as 144 células usam Worlds e registros frescos
e proíbem tokens de sucesso atribuídos pela fixture. Fabricar esses valores
violaria a própria independência. Recibo:
[authorability R6b](p1340-oracle-expectations-authorability-r6b.json), SHA-256
`86fd8a6c6d1e3ac41155b01cabd795a198fc122c3f6186e1e69ff5fa9071dd30`.

O [veredito independente R6](p1340-verifier-final-r6.json), SHA-256
`519d4791626c7a3bfe325d5b4d2bba302b57b75b5a13627dee8838df2f0975a0`,
é `NO_SEAL_STOP_REQUIRES_PROTOCOL_REDESIGN`. Nenhum código candidato foi
alterado ou executado nessa cadeia; F não foi executado nem aceito.

### Consequência operacional

O 1340 não deve receber novas revisões locais. O próximo trabalho precisa de
um passo próprio que redesenhe o protocolo antes de implementação: expectativas
externas usam coordenadas semânticas estáveis por célula, identidades dinâmicas
ficam somente no ledger real e sua bijeção é verificada internamente, enquanto
um manifesto estático separado pina callsites e prova completude. Só depois do
novo selo os hooks cfg-only e as matrizes públicas/lifecycle/NT01–NT06 podem
ser executados. Regime: **executado sem atestação de isolamento**.
