# P1345 — verificação independente de insuficiência final R1

Regime: `executado sem atestacao de isolamento`.

Papel: verificador independente de insuficiência. Esta verificação não corrigiu
contrato, oráculo, corpus, probe, L0, passo, teste ou código produtivo; não leu
nem executou candidato P1345 e não executou `--full`.

## Entradas congeladas verificadas

- passo P1345: `c2ef7ff7b0555a4f44ca6811ff3e687ef6d6a30f55e0cd106fba4118f3a18fb0`;
- manifesto P1345 R1: `2ed02b31b39c6d588db7772cf932af40e344e87947662e12b7a8fe5c7e96b2c2`;
- contrato/binding/recibo FINAL R2: `f40c2b42fe83a4b75e276ab2ea8a9639fa1553e80c06581caf8c53d948f9d38d`,
  `ecc3a8a403d9d14d51808a35876f1f43d7773732c1d20f58574dff42d5187778`,
  `875f937affb30486c9f63fa0aaf84d1e9a6e93214a8fdfb95db320e76a6c14c2`;
- relatório/recibo adversarial R2: `3130b8d6a2d5aab7c27c2ede92edfa955e390acafe1a6f78e3c2d7d2f6a0afdf`,
  `012c19161aa88c99819204b14da4abb8da1a61a65760d24c52921f6299e5fc4c`;
- checker/caller/corpus R3: `67063acf2687a9662d7b213a8aa047c2a38f7b69a0310aff59f2dbe34809979f`,
  `a87f8f2f355ed7950ebe10a05f252816e5c0e5ddf561240c8a710991250c817e`,
  `6c5f6ce6173818e3c04373cdd0183e27fa1d5d208271322dc9ed5fb0c6cc1e88`;
- autoria/recibo R3: `528ab182f76d1bdaf31fa0d7ccad4533ff82200f9f551dc1c01d8c242c6f9422`,
  `3a1179eb086c7d8aaae60184e6165ebf5b387a5f43746bcce0f1907d045c67f9`;
- probe R1: `863fa1588083638ec9f52b7ee663023e260a09880244c61cc948df36e946e2dc`.

## Reprodução focal

A invocação canônica pelo caller, com os hashes atuais e raiz autoral calculada
`e0f1bb65c1f1212246519cbd5f0e197684b924ac32ba50e66a1079deb90c3e1f`,
terminou com exit `2` antes de executar o focal:

```text
AUTHORITY_ROOT: receipt identity/verdict
```

O motivo é reproduzível diretamente: o checker R3 exige na linha 183 que o
recibo tenha veredito `FOCAL_AUTHORED_NOT_VERIFIED_NOT_SEALED`, enquanto o
recibo protegido declara na linha 30
`FINAL_REVISION_SURVIVORS_NOT_SEALED`. Logo, a cadeia canônica R3 já não é
reexecutável a partir dos próprios pins. O stdout ficou vazio
(`e3b0c442...b855`) e o stderr teve SHA-256
`9f4732dadee53fa015aae5e64df5676159e3c40a661eff0a1b98b0b039f52573`.

Para auditar somente o vetor declarado sem editar entrada, foi feita uma
instrumentação efêmera em memória que substituiu exclusivamente o predicado
inconsistente de identidade/veredito do recibo por comparação exata da triple
canônica atual. Esse ensaio focal reproduziu:

- `122` negativos válidos;
- `120` corretamente `Violated`;
- mutation score `120/122 = 0.9836065573770492`;
- quatro controles positivos herdados e quatro controles de fronteira sem
  regressão;
- sobreviventes `P1344-A21-alternate-corpus-and-digest` e
  `P1344-R2A17-alternate-corpus-root`, ambos `Preserved/PRESERVED`;
- `full_corpus_runs = 0` e budget restante de revisão `0`.

Esse ensaio confirma o defeito discriminatório, mas não repara nem substitui a
reprodução canônica falha e não constitui selo.

## Causa dos dois sobreviventes

O adaptador histórico em `p1345-adversary-runner-r1.py:391-403` chama o checker
somente com `--focus --corpus <alternativo>` e considera a mutação rejeitada
apenas quando há simultaneamente exit `2` e o literal `AUTHORITY_ROOT` no
stderr. A reprodução exata terminou com exit `2`, mas `argparse` avaliou antes
os argumentos obrigatórios ausentes:

```text
error: the following arguments are required: --authorship-receipt,
--authorship-receipt-sha256, --expected-checker-sha256,
--expected-authoring-root-sha256
```

O stderr não contém `AUTHORITY_ROOT` (SHA-256
`0b6b87dd589c75566ec1bc8510166084a3aacbc00f122427b615f13f74a10ab6`).
Assim, a precedência de `argparse` impede o reason esperado e o adaptador
converte uma rejeição física da CLI em `Preserved`. A mesma função gera os dois
IDs, portanto ambos sobrevivem pela mesma causa pública já repetida.

## Gate Tekt V5/V15/V26

Comando reproduzido:

```text
crystalline-lint --checks v5,v15,v26 .
```

Resultado: exit `1` antes dos checks semânticos, com:

```text
error: Erro de sintaxe: Syntax error detected in AST [PARSE]
   --> ./00_nucleo/diagnosticos/p1345-opaque-probe-r1.rs:117
```

O arquivo novo causa o `PARSE` do parser do `crystalline-lint`. O mesmo arquivo
passa `rustfmt --edition 2021 --check` com exit `0` e compila com o `rustc`
estável congelado, também com exit `0`, para um executável temporário em
`/dev/shm`. Portanto não é erro sintático aceito pelo compilador Rust: é uma
incompatibilidade do parser do linter localizada na expressão da linha 117:

```rust
let (challenge, invocation) = request(&raw).expect("closed P1345 request");
```

Enquanto esse arquivo permanecer no alcance do linter, V5/V15/V26 não podem ser
atestados por essa invocação.

## Ausência de candidato e budget

`rg -n -i "p1345|P1345" 01_core 02_shell 03_infra 04_wiring` não encontrou
marcador P1345 em código produtivo. Os recibos R2/R3 e esta reprodução focal
registram `full_corpus_runs = 0`; nenhuma execução full foi feita por este
verificador.

## Proveniência

Medição consolidada em `2026-09-11T08:46:22,061203722-03:00`:

- `HEAD = 2f42d64253547734564513a1159ee6b584c1c4b4`;
- working tree não commitado;
- `sha256(git status --porcelain=v1 -z) =
  78a9bbbd9894824c30f160215f8cba7b3c0e2cee756b3e1f48cc7f4d92f1694f`;
- `1151` entradas no snapshot `porcelain` medido;
- `sha256(git diff --binary HEAD) =
  099901758e0dda60cc4c16de0091adfdcedd3fcc63029e1270c34147b60aadef`;
- `sha256(git diff HEAD --stat) =
  9783f07bc86c21e8874ebacec9d92ce5a12208e127748377c571b091a1c92e15`;
- stat rastreado: `76 files changed, 10972 insertions(+), 833 deletions(-)`.

## Veredito

`BLOCKS_PRESEAL_AND_IMPLEMENTATION`.

Há sobreviventes válidos com score menor que `1.0`, a reprodução canônica do
próprio R3 falha por inconsistência de recibo e o gate Tekt é interrompido por
`PARSE`. O budget de revisão do oráculo está esgotado. Pelas regras do passo e
da materialização segregada, não há autoridade para pré-selo, RED A/B,
implementação ou certificado neste passo.
