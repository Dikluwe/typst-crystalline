# P1339 — revisão focal do desenho de opacidade C

Autoridade: verificador independente `/root/p1311_review`; executado sem
atestação de isolamento. Data da auditoria: 2026-09-10T03:41:30Z. Manifesto
r2 SHA-256 `842d6526739014022c073800148a47b3c886831e2198ab65bbfdbe73ce59411b`.
Contrato vigente r3 SHA-256
`c0cd1826679ddaf77e937a677839c5cbe64fdae6ec71bfa34e635d584d9c3e17`.
Nenhuma alteração de contrato, L0, produto, fixture ou expectativa foi feita
pelo verificador. Não é selo nem autorização de matriz completa.

## Evidência anterior à decisão

Os recibos imutáveis do autor dos oráculos identificam HEAD
`2f42d64253547734564513a1159ee6b584c1c4b4`, árvore não commitada com lista
e diff-stat integrais, fontes, comandos, ambiente, tempos e binários pinados:

- `p1339-ab-opaque-focal-r1-runs.json`, SHA-256
  `fe298f26f5bf8423f3b6b92b85bebb31279ecc0d4406fefe7214bfa9629da8b3`:
  início 03:35:40.986360 UTC, 16 processos. O programa `while true` produz
  erro conhecido nos dois produtos; o controle finito produz 3 nos dois.
- `p1339-ab-opaque-focal-r2-runs.json`, SHA-256
  `ab2c3fef3bdd09e0b3cce989c10cd72ba129eb71965b999c167edac8f5a0fa29`:
  início 03:36:53.357710 UTC, 16 processos. O workload finito aninhado produz
  quatro raw Unknown por timeout no vanilla e quatro erros conhecidos da
  guarda de 1000000 iterações no baseline; os oito controles finitos passam.

São medidas do autor, lidas e auditadas, não reexecução independente nem
prova de repetição/reordenação. Nenhum resultado anterior é recodificado.

## Juízo de desenho

R3 `classification.Opaque`, `Aggregation`, `mutation_protocol.calibration`
e `phase_policy` exigem controles opacos reais declarados e distinguem as
classes de execução. Não exigem que um único programa seja opaco em ambos
os produtos. Um timeout é opacidade de observação sob o orçamento do
executor, não prova de opacidade semântica de um valor Typst.

É admissível o autor congelar explicitamente, antes da primeira matriz,
o controle r2 assimétrico: mesma fonte, vanilla limitado produz raw Unknown
por timeout; baseline preserva seu erro conhecido integral. A classificação
é específica à identidade/condições do executável, não uma expectativa de
paridade do workload. Deve haver IDs/políticas distintas ou células
explicitamente distintas, sem converter o erro baseline em Unknown e sem
converter o timeout em Preserved. A guarda baseline permanece fora do escopo.
O controle finito positivo e a repetição/reordenação continuam obrigatórios.
O controle opaco recebe zero crédito positivo; qualquer deriva de resultado
do controle nas ordens exigidas bloqueia sua alegação de determinismo.

Isso não permite excluir opacos congelados, reduzir obrigações nem conceder
cobertura interna: os quatro Angle NaN condicionais são exceções declaradas,
não execução do comparador; F06 Dynamic permanece NotDue em C com definição
congelada obrigatória; M20 continua negativo independente com raw Unknown e
rejeição `mandatory_unknown`. Nenhum deles substitui silenciosamente o
controle opaco C. Uma nova tentativa bilateral exigiria hipótese causal nova;
não há motivo contratual para continuar ajustando o mesmo loop para forçar
ambos os produtos a timeout. As duas tentativas malsucedidas são preservadas.

## Dois rótulos residuais de r2 no contrato r3

Além do rótulo de `gates.preimplementation`, `oracle_interface.phase_policy`
diz `this r2 contract`. A interpretação é estritamente a autoridade sucessora
r3 e seus dispositivos específicos, sem mudança de bytes: oráculos e selo
devem pinar r3 `c0cd1826...` e sua cadeia r2/r1. Nenhum selo r2 autoriza
implementação. Este registro não é quarta revisão de contrato nem licença
para alterar semântica, fixtures ou política de fases.

Orçamento mantido: três revisões publicadas de contrato; zero matrizes
completas preseal do verificador. A suficiência de todos os artefatos
canônicos continua pendente.
