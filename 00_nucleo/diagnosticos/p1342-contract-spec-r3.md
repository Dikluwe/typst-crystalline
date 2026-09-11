# P1342 — contrato R3: baseline histórico e candidato live

Regime: **executado sem atestacao de isolamento**. Veredito de autoria:
**CONTRACT_REVISED_NOT_SEALED**. Esta é a segunda e última revisão contratual
permitida pelo Passo 1342. R1 e R2 permanecem intactos.

## Correção do blocker

`baseline_file_sha256` identifica exclusivamente o arquivo candidate-free visto
antes da implementação. Ele é evidência histórica que o pré-selo R3 deve
atestar junto do manifesto exato, dos 19 rows, dos 18 hooks aplicáveis por
célula, das âncoras Rust por símbolo/branch e dos snippets pinados. Esse hash
**nunca** é comparado ao arquivo live depois do patch. Alterar os consumidores é
necessário para instalar os hooks; a desigualdade de hashes não decide aceitação.

O binding R3 é uma composição fechada: pina o binding R2 por hash, exige suas
19 linhas na mesma ordem e adiciona a cada linha o hash exato do snippet de
âncora candidate-free. H00D e H00S continuam mutuamente exclusivos, portanto
cada célula aplica 18 linhas. Nenhuma regra de carrier, Location, snapshot,
ledger, attempt-kind/R, `Unknown` ou B01–B05 da R2 foi removida.

## Duas cadeias de evidência

O baseline é selado antes do patch. Um pré-selo R3 reemitido deve pinar o
contrato/binding R3 e conservar uma visão candidate-free legível em bytes/AST,
com arquivos, símbolos, branches, âncoras e snippets suficientes para comparar
limites de mudança depois.

A evidência do candidato só nasce depois do patch e é produzida por um
verificador de source independente. Ela contém:

- SHA-256 integral de cada arquivo live referido pelo manifesto;
- um registro por cada uma das 19 linhas, resolvido por parser Rust, com
  símbolo, branch, âncora única e adjacência do hook sob
  `cfg(p1339_observation)`;
- limites exatos de diff em bytes e AST contra a visão candidate-free, todos
  cfg-only e pertencentes a um row ou à fachada test-only autorizada;
- prova de igualdade de tudo fora da união desses limites;
- auditoria de um único writer append-only, todos os seus callsites, ausência
  de writer/constructor alternativo e projeção somente leitura;
- coverage runtime `normal`, `repeat` e `reverse` ligando recibos crus a cada
  row antes da projeção.

O artefato de evidência é externo ao DTO e à fachada. O verificador calcula seu
hash e entrega path+hash diretamente ao checker. DTO e
`p1342_run_fixture_for_test` não podem declarar nem autenticar preseal, hashes
de source, limites de diff, binding estrutural ou veredito de writer/projeção.

## Checker candidato

O checker recebe por canal controlado pelo verificador os pins exatos do
contrato R3, binding R3, pré-selo R3 reemitido, evidência externa pós-patch,
fixture e freeze L0. Primeiro confirma que o pré-selo atesta o baseline
histórico. Depois rehasha o artefato externo e cada arquivo candidato completo,
comparando o live somente com `candidate_file_sha256` desse artefato.

Em seguida ele refaz a resolução Rust-aware, os limites permitidos, writer
único, projeção read-only e coverage; só então executa todos os predicados R2 do
ledger, carriers, Location, snapshots, eventos, cardinalidades e payload. É
proibido exigir `current candidate hash == baseline_file_sha256` ou aceitar
evidência estática autodeclarada no JSON final.

A06 (IDs fixos) e A07 (rechain pós-hoc) continuam negativos válidos. O primeiro
falha pelos challenges/identidades disjuntas; o segundo pela visão raw,
writer/projeção e coverage externos. Nenhum é descartado por conveniência.

## Invalidação causal

A mudança do contrato invalida todos os artefatos posteriores que pinam R2:
oráculos/checkers/corpus, o pré-selo R2 e o teste/recibo RED. Todos devem ser
reemitidos na ordem R3 antes de implementação. O RED atual ainda é evidência
histórica útil, mas não pode julgar R3.

O freeze L0
`2fb962c3edd8cdd83848d2cd7c9158c39a5d0218f510e81bb5e8011a540e8c0e`
e a fixture
`98159f5ac529520590a197521dfb23383cec0ba6373b431b8b33f426cfc3a714`
permanecem válidos se esses hashes conferirem. Não é necessária alteração L0
nem paragem ADR-0127: esta revisão corrige apenas a topologia da evidência de
aceitação, sem mudar API, comportamento padrão, fase ou compatibilidade.

Nada aqui autoriza P1340 lifecycle/profile, NT01–NT06, política terminal,
retenção geral, paridade geral, candidato ou selo. O estado permanece
`NOT_SEALED` até a cadeia R3 independente ser reemitida e obter score 1.0.
