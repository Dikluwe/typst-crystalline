# P1345 — inventário candidate-free e encaixe físico R1

Regime: **executado sem atestacao de isolamento**.

Autoridade: `/root/p1345_baseline`. Este documento mede somente o estado anterior
ao contrato P1345; não contém candidato, oráculo, teste, selo ou veredito runtime.

## Proveniência da medição

- instante: `2026-09-11T07:05:41-03:00`;
- HEAD: `2f42d64253547734564513a1159ee6b584c1c4b4`;
- working tree: não commitado;
- `sha256(git status --porcelain=v1 -z)` antes destes outputs:
  `5c58d93a0437e494ff94a611ba59b799310469c9a411c102d5e5859823268cf0`;
- `sha256(git diff --binary HEAD)`:
  `099901758e0dda60cc4c16de0091adfdcedd3fcc63029e1270c34147b60aadef`;
- `sha256(git diff HEAD --stat)`:
  `9783f07bc86c21e8874ebacec9d92ce5a12208e127748377c571b091a1c92e15`;
- stat rastreado: `76 files changed, 10972 insertions(+), 833 deletions(-)`.

## Resultado candidate-free

A busca literal por `p1344`, `P1344`, `p1345` e `P1345` em `01_core/src`,
`02_shell/src`, `03_infra/src` e `04_wiring/src` devolveu zero ocorrências.
Logo não existe marker, helper, fachada ou corpo candidato P1344/P1345 nos
arquivos produtivos. Os três consumers ressellados no P1344 continuam nos
hashes candidate-free do freeze e a comparação normalizada P1344 já limita o
delta aos headers de linhagem.

Os 12 pares Prompt L0/consumer do freeze P1344 foram rehashados: 24 de 24
arquivos coincidem com os hashes protegidos. `crystalline-lint` para
V5/V15/V26 terminou sem violações.

## Topologia e encaixe físico

O overlay protegido `p1344-capsule-baseline-r1.json` foi recomposto sem
interpretação adicional:

- 38 IDs de cápsula, todos únicos;
- 12 arquivos, todos únicos;
- a concatenação dos manifests por arquivo coincide exatamente com a ordem dos
  36 IDs herdados seguida dos dois IDs owner-local;
- as quatro âncoras dos dois inserts P1344 ocorrem exatamente uma vez nos bytes
  candidate-free atuais;
- nenhum override P1345 foi aplicado à topologia.

Foram inseridos apenas métodos associados sintéticos e inertes em cópias dos
três consumers, sob `/dev/shm/p1345-owner-grammar.TE1qAe`, e cada arquivo
completo foi aceito pelo parser do `rustfmt --edition 2021`:

- `Func` dentro do `impl Func`: SHA-256 do probe formatado
  `d6e21caf118ee30ff07098b1ec6dc930bda7f4e88f2672a3d9a05c223cdfb2f7`;
- `Content` dentro do `impl Content` owner de `get_field`: SHA-256
  `b07e634bb860ade6fbf5e56a532ca49de072464e968f56036d890bfab370238b`;
- `CounterUpdate` dentro do `impl CounterUpdate`: SHA-256
  `41946d143f1057672b8cf36829a137a4cbaa271e357e76054ca110983db6dad8`.

Resultado: 3 de 3 categorias owner-local são fisicamente válidas sem fechar ou
reabrir `impl`, sem impl não local, wrapper, trait, macro ou container. Isso
prova somente o encaixe sintático da categoria; não aprova o futuro corpo
canônico nem comportamento produtivo.

## Toolchain congelada

`stable-x86_64-unknown-linux-gnu`, `rustc 1.92.0
(ded5c06cf 2025-12-08)`, `cargo 1.92.0`, `rustfmt 1.8.0-stable` e
`rustc-dev` ausente. Portanto, a opção HIR/MIR via `rustc_private` continua
fora do protocolo reproduzível deste passo.

Veredito: `CANDIDATE_FREE_TOPOLOGY_READY_FOR_P1345_CONTRACT_AUTHORSHIP`.
