# P1342 — inventário reversível do candidato rejeitado P1341 R1

Regime: **executado sem atestacao de isolamento**.

## Estado anterior à remoção

- HEAD: `2f42d64253547734564513a1159ee6b584c1c4b4`.
- UTC da medição preparatória: `2026-09-10T20:14:04Z`.
- `03_infra/src/pipeline/context_stabilization.rs`:
  `17043434105dabb55a14beaa8901d3127f8f36aa2f24969f9484b4afccfc4602`.
- `00_nucleo/diagnosticos/p1340-implementation-pipeline-observer.rs`:
  `fe78ff3ae2bfff7494fe014848248b7f5cbb320cdab2e7ed407986ed12b253af`.
- `01_core/src/compiler/eval/mod.rs`:
  `2382dc1229d0c5dd62c86a68f9e4eb21a726563a35cbe3f56946f5d415c1fe0c`.
- teste independente P1341 preservado sem inclusão produtiva:
  `a9a37bcc1d3b68138500100a051f3565f1cd71ea0ba5a7659f6a8a66923af5d0`.

## Hunks identificados como candidato rejeitado

1. Inclusão de `p1341-implementation-tests.rs` e fachada
   `p1341_real_ledger_focal` em `context_stabilization.rs`.
2. Import de `p1340_observation_counter_events`, thread-local
   `P1341_ACTIVE`, `P1341Capture` e `p1341_observe` no observer P1340.
3. Coleta P1341 agregada em `body_started`, `candidate_built` e
   `before_decision`.
4. Projeções paralelas `p1341_typed_value`/`p1341_capture_dict`.
5. Função `p1341_real_ledger_focal_impl`, que construía objetos, eventos,
   identidades e arestas depois da compilação.
6. Alteração da projeção P1340 de Dict para lista de pares, introduzida pelo
   candidato P1341 sem binding do ledger ao owner.

## Operação de candidate-free

Remover somente os hunks acima com patch textual explícito. Preservar:

- todos os artefatos P1341 e seu veredito como evidência;
- todos os hooks e testes P1340 preexistentes;
- toda a cadeia produtiva P1339/P1340 e alterações alheias do workspace;
- os Prompt L0, que serão reavaliados na Fase B antes do novo selo.

Não usar reset, checkout, restore ou reversão ampla. A restauração de qualquer
hunk rejeitado só pode ocorrer como implementação nova após o selo P1342.

## Estado candidate-free resultante

- `rg -n "P1341|p1341"` nos quatro consumers L1, no owner L3 e no observer
  P1340: zero ocorrências.
- `03_infra/src/pipeline/context_stabilization.rs`:
  `1e42852fefeabb5381174bc605d638b0459126c8a64cb76f75a8dd49f2951a7c`.
- `00_nucleo/diagnosticos/p1340-implementation-pipeline-observer.rs`:
  `669af6940bb21c47540913039779c51d8f492480fad8df12f6aff1fe4004fa7c`.
- `01_core/src/compiler/eval/mod.rs`:
  `6f885d48163c78c0d4644f8e940aa2583ead2072b0d4adac3f2500ae16e32ad5`.
- SHA-256 de `git status --porcelain=v1 -z`:
  `bfdb955fec65bf0ed3570db857ec3a4cf91c73c0a45c8f78b550992c76947552`.
- SHA-256 de `git diff --binary HEAD`:
  `6f9e75018a66a19fa55cc001f46fb29acfa886d6c87c8db4e935d1df3900cab3`.
- `git diff --check`: exit 0.
