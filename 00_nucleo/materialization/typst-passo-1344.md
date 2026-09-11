# Passo 1344 — realocar helpers de observação aos owners Tekt e concluir o binding P1342

## Objetivo

Corrigir exclusivamente a contradição física que encerrou o Passo 1343:
`P1343-FUNC-CARRIER-HELPERS` ocupa uma posição de associated items dentro de
`impl Func`, mas o contrato FINAL R2 exigiu ali também `impl Content` e
`impl CounterUpdate`, forma rejeitada pelo parser Rust.

O reparo divide esses helpers segundo a unidade dona, na mesma camada L1:

- helpers de `Func` permanecem em `01_core/src/entities/func.rs`, dentro de
  `impl Func`;
- helpers de `Content` passam a uma cápsula própria em
  `01_core/src/entities/content.rs`, dentro de `impl Content`;
- helpers de `CounterUpdate` passam a uma cápsula própria em
  `01_core/src/entities/counter_update.rs`, dentro de `impl CounterUpdate`.

Depois de reemitir e selar essa topologia, o passo retoma o RED e a
implementação focal H00D/H00S/H01–H16 originalmente autorizada por P1342.

Este documento é passo de execução, não Prompt L0. Regime:
**executado sem atestacao de isolamento**. O workspace é compartilhado; a
segregação é de autoridade, entradas, ordem e artefatos.

## Autorização e proveniência

O humano autorizou explicitamente em `2026-09-10` escrever e executar o
sucessor P1344 e perguntou se a execução segue a arquitetura Tekt.

Medição em `2026-09-10T23:05:31-03:00`:

- HEAD: `2f42d64253547734564513a1159ee6b584c1c4b4`;
- working tree não commitada;
- SHA-256 de `git status --porcelain=v1 -z`:
  `5a41980abcddfe219ca4b72c0555c5eb8d50f6f3c1ba1eaee10162df6b804c2d`;
- SHA-256 de `git diff --binary HEAD`:
  `f73da54bdbeb1bd086caeff259b9517bdbf0f1648efcb8119d36699f5bcf0be5`;
- stat: `72 files changed, 10889 insertions(+), 830 deletions(-)`.

Entradas causais protegidas:

- `p1343-oracle-blocker-r3.md`, SHA-256
  `c2af3f4b314660f620904d1fd7fa2bd0d5a5c9d8162d66edcf9aaa49e8a053d3`;
- `p1343-oracle-blocker-receipt-r3.json`, SHA-256
  `5db7551ed182042d045a58efb9ec3c04953f9d6738cf822555de8606f4441171`;
- contrato FINAL P1343 R2, SHA-256
  `d4e444f6966a2be34018854c2b3857452447af1741e799e4cc2cfd6f31c48967`;
- binding FINAL P1343 R2, SHA-256
  `216100a3970bdc79b3d77cd34f4528d46cd04e06dbdc9d63d552f723a557af58`;
- recibo FINAL P1343 R2, SHA-256
  `013c7bb1b36a5a182a69dcb5c8451af3915239ed292c83682dc1655b51243637`;
- baseline de 36 cápsulas P1343, SHA-256
  `db143d9fec795ec81ce7a69be5e08b09e451a9d6eb07a1a39cd1dcd41ee82977`.

Na medição, os dez consumers P1342 continuavam exatamente nos hashes
candidate-free do freeze. Os dois novos consumers têm SHA-256:

- `01_core/src/entities/content.rs`:
  `34838354261fa472c5efbc73c39ee6587b8b77be4d4606ca7587537818ed4a98`;
- `01_core/src/entities/counter_update.rs`:
  `412c80370dd64e983582c4a580cedaa823e6bad23203ab1c3b5a13b85f0d9f05`.

## Conformidade Tekt medida antes da decisão

`Content` já possui owner produtivo 1:1 em
`00_nucleo/prompts/entities/content.md` e implementação em
`01_core/src/entities/content.rs`. `CounterUpdate` possui owner 1:1 em
`00_nucleo/prompts/entities/counter_update.md` e implementação em
`01_core/src/entities/counter_update.rs`. Ambos estão em L1 e já participam do
caminho real do callback; não é necessário novo módulo órfão, import reverso,
trait dinâmico, side table global ou owner artificial.

A tentativa P1343 colocou três unidades semânticas numa cápsula fisicamente
pertencente a `impl Func`. Corrigi-la fechando/reabrindo braces, usando const
container ou declarando impls não locais manteria a lógica no arquivo errado e
violaria a atomização. A forma Tekt é realocar cada helper à unidade dona, no
seu próprio consumer, preservando a cardinalidade Prompt L0 ↔ consumer 1:1.

## Auditoria e delta L0 antes de código

Antes de qualquer candidato, uma autoridade de intenção deve atualizar e
resselar, em fluxo contínuo ADR-0127, somente estas cláusulas internas:

1. `entities/func.md`: esclarecer que a cápsula/helper de `Func` contém apenas
   associated items de `Func`; não é owner de impls de `Content` ou
   `CounterUpdate`;
2. `entities/content.md`: autorizar sob `cfg(p1339_observation)` somente o
   helper crate-private que lê o `Content::CounterUpdate` real e encaminha H11
   pelo carrier já existente, sem mudar enum, campos, Eq/Hash/repr ou API
   normal;
3. `entities/counter_update.md`: autorizar sob
   `cfg(p1339_observation)` somente o helper crate-private que consulta a
   variante `Func` real e preserva o mesmo carrier para H10/H12, sem mudar as
   variantes, Eq/Hash ou API normal.

Essa é correção de organização e observação interna sob cfg. Não muda contrato
público, comportamento por defeito, compatibilidade nem fase do pipeline;
portanto segue fluxo contínuo ADR-0127: L0 primeiro, resselo e revalidação, sem
paragem humana intermediária. Qualquer necessidade diferente dessas três
cláusulas aciona parada.

O novo freeze contém os dez pares P1342 intactos mais os dois pares acima,
totalizando 12 owners e 12 consumers. V5/V15/V26 devem passar antes do contrato.

## Topologia de cápsulas P1344

Uma autoridade candidate-free reemite o baseline como overlay fechado sobre
P1343:

- preserva as 36 cápsulas, dez arquivos, replacements, anchors e hashes P1343,
  exceto a interpretação impossível explicitamente substituída;
- restringe `P1343-FUNC-CARRIER-HELPERS` a associated methods de `Func` que
  cabem dentro do `impl Func` já medido;
- adiciona `P1344-CONTENT-CARRIER-HELPERS` no `impl Content` real;
- adiciona `P1344-COUNTER-UPDATE-CARRIER-HELPERS` no
  `impl CounterUpdate` real;
- redistribui apenas a cobertura/helper ownership de H10/H11/H12 necessária,
  sem alterar as 19 rows nem sua semântica runtime.

O resultado esperado é uma tabela fechada de **38 cápsulas em 12 arquivos**.
As duas novas cápsulas são `insert`, têm replacement vazio, cfg exato
`p1339_observation`, âncoras candidate-free únicas e corpo limitado a métodos
associados da própria unidade. Nenhum helper de `Content`/`CounterUpdate` pode
permanecer em `func.rs`, e nenhum helper de `Func` pode migrar para os novos
owners.

A autoridade deve provar com parser Rust, antes do contrato, que cada cápsula
fica numa posição que aceita sua categoria sintática. Se qualquer uma exigir
fechar/reabrir owner, impl não local, macro/const container ou arquivo fora dos
12 owners, parar.

## Contrato, oráculo e ataques reemitidos

O contrato P1344 compõe integralmente P1343 FINAL R2 e substitui somente:

- contagem/topologia `36/10` por `38/12`;
- a produção impossível de `P1343-FUNC-CARRIER-HELPERS` pelas três produções
  owner-local acima;
- pins do freeze/baseline/corpus correspondentes.

Toda gramática fechada, normalização inversa, skeleton do carrier/ledger,
separação source-vs-runtime, política de `Unknown`, precedência e obrigações
P1342 permanecem. O contrato novo deve ter schema fechado e não pode reabrir
regex/substring como prova.

O source verifier/oráculo é reemitido sem candidato e deve:

- validar por tokens/estrutura as 38 produções em seus owners reais;
- provar que remover/substituir cápsulas recupera os 12 hashes candidate-free;
- manter corpus/hash/root pinados por autoridade e JSON estrito;
- compor todos os casos P1343 R1/R2 e ataques adversariais já válidos;
- acrescentar negativos para helper no owner errado, impl aninhado, método
  associado na entidade errada, import/reexport que contorna owner, cápsula
  ausente/duplicada e normalização dos dois novos arquivos;
- usar runtime sintético apenas no corpus discriminatório; candidato continua
  exigindo teste Rust e evidência runtime real externa.

Um adversário independente ataca o novo owner split. Somente depois do focal
verde, um pré-verificador distinto executa uma única vez o corpus completo em
normal/repeat/reverse e exige mutation score `1.0` antes de emitir
`SEALED_FOR_IMPLEMENTATION`.

## RED e implementação

Depois do selo, o Testador A/B reemite o teste P1344 sem ler o patch. O teste
mantém fixture de 178 bytes e cria três World/Source/challenges realmente
frescos; chama a fachada uma vez por modo; valida raw pré-projeção, DTO,
identidades, carriers, spans, Locations, snapshots, cardinalidades e 19 hooks.
Candidate-free deve falhar por ausência específica da fachada/hook.

O implementador recebe apenas L0/freeze, contrato/oráculos selados e RED. Ele:

- implementa somente as 38 cápsulas autorizadas;
- mantém `Func`, `Content` e `CounterUpdate` atomizados nos próprios arquivos;
- usa um carrier/ledger privado, append-only e cfg-only, sem global/side table;
- liga H00D/H00S/H01–H16 aos callsites produtivos reais;
- mantém o caminho `not(p1339_observation)` byte-equivalente ao baseline;
- não edita contrato, baseline, corpus, teste congelado ou qualquer input
  protegido.

## Aceitação

- 12 L0/consumer pairs congelados, 1:1, com V5/V15/V26 verdes;
- 38 cápsulas/12 arquivos, todas parser-valid no owner declarado;
- normalização inversa recupera os 12 hashes integrais candidate-free;
- todos os positivos `Preserved`, todos os negativos válidos `Violated`, único
  opaco autorizado `Unknown`, mutation score completo `1.0`;
- pré-selo válido antes de RED/candidato;
- RED específico e GREEN repetido em três execuções frescas;
- `cargo test --no-run` observado, `cargo build` normal, rustfmt,
  `git diff --check`, V5/V15/V26 e `crystalline-lint .` sem violações novas;
- certificado independente `ACCEPTED` limitado ao binding focal P1342/P1344.

O certificado não fecha P1340/P1339, NT01–NT06, retenção, descarte,
invalidação, política terminal ou equivalência funcional geral.

## Budget e condições de parada

- um contrato P1344 inicial e no máximo uma revisão contratual;
- no máximo uma revisão focal do verificador por nova classe de falha;
- autoria executa apenas recortes focais;
- uma única execução completa antes do selo, pertencente ao pré-verificador;
- uma execução completa final;
- no máximo dois ciclos de correção do candidato;
- parar após duas tentativas com o mesmo vetor e causa;
- parar se qualquer produção não couber fisicamente no owner antes de contrato;
- parar diante de drift em input protegido, score menor que `1.0`, mudança
  ADR-0127, necessidade de 13º consumer ou enfraquecimento de negativo.

Budget nunca autoriza waiver, reinterpretação clandestina ou `Unknown` como
sucesso.
