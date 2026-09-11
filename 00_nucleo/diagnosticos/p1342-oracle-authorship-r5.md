# P1342 — revisão focal R5 interrompida pelo budget

Regime: **executado sem atestacao de isolamento**. Papel: autor independente
de oráculos, sem leitura de candidato produtivo P1342. Veredito:
**ORACLE_REISSUED_NOT_VERIFIED_NOT_SEALED**.

R1–R4 permanecem intactos. Foram criados checker e corpus R5 candidate-free
para responder ao adversário R3, mas a calibração não ultrapassou o gate focal.
Não houve execução completa R5, selo, pré-selo nem autorização de
implementação.

## Hipótese R5

A revisão removeu `checker_fixture` da entrada externa: comparator, alias ou
decoy agora é chave extra de schema e nunca é executado. Para a evidência de
source, o checker focal deriva diretamente de `baseline_bytes` e
`candidate_bytes` sintéticos externos:

- balanceamento estrutural e funções/markers Rust por row;
- símbolo, branch, âncora e bloco cfg adjacente;
- hashes de arquivo, snippet, símbolo e região;
- ligação entre hook encontrado e coverage/runtime hit;
- diff real baseline→candidate e igualdade entre seus spans, a união de
  boundaries declaradas e os blocos cfg encontrados.

Os booleans `rust_parse_ok`, `outside_allowed_boundaries_equal` e
`outside_region_unchanged` não fundamentam esses resultados. Nenhum arquivo
produtivo foi aberto ou usado como candidato; os pares de source são fixtures
sintéticos fechados.

## Focal e condição de parada

O recorte executado foi exatamente Y01, Y02 e os cinco sobreviventes
Z01/Z02/Z06/Z07/Z12, em `normal`, `repeat` e `reverse`.

Na primeira execução focal:

- Y02 e Z01/Z02/Z06/Z07/Z12 foram `Violated`;
- Y01, esperado `Preserved`, foi `Violated` com a causa
  `real baseline-to-candidate diff is not exactly the union of declared
  cfg-only boundaries`.

Uma revisão focal tentou eliminar ambiguidade do algoritmo de diff tornando o
fim de cada bloco cfg sintético único. A segunda execução produziu exatamente o
mesmo vetor e a mesma causa: os seis negativos continuaram `Violated` e Y01
continuou `Violated`.

Isso consumiu as duas tentativas permitidas para a mesma classe sem ganho
observável. A condição de parada foi acionada. Não foi feita uma terceira
correção nem a execução completa de 63 casos. Portanto não há score R5 final;
o `1.0` do subconjunto negativo focal não compensa a regressão do controle
positivo.

## Estado

A R5 permanece artefato diagnóstico inconclusivo. Os cinco Z não sobrevivem ao
checker focal, mas essa observação não autoriza fechamento porque Y01 regrediu.
É necessária revisão do modelo de diff/fixture antes de um novo ciclo
explicitamente autorizado. Continuam fora de escopo P1340, NT01–NT06,
retenção/invalidação, política terminal, candidato e equivalência geral.
