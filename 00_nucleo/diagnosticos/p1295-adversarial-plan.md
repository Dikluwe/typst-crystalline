# P1295 — plano adversarial, nova campanha rustdoc JSON

## Veredito

```text
P1295_DISCRIMINATION_PASSED_READY_FOR_SEAL
```

O protocolo completo foi reaberto como nova campanha causal de P3 após P2
substituir o oráculo inválido por um inventário rustdoc JSON pinado. A execução
ocorreu em workspace compartilhado, sem alegação de isolamento técnico de
leitura. Nenhuma implementação candidata P1295 existia ou foi lida. Controle,
mutantes, runners, logs e fixtures ficaram somente em
`/tmp/p1295-p3-r2.0Linlh`.

O controle positivo foi `Preserved` antes e depois dos ataques. MW1, MW2 e MW3
foram mortos no recorte focal, na ordem direta MW1→MW2→MW3 e na ordem inversa
MW3→MW2→MW1. Mutation score: `3/3 = 1.0`; survivors: zero; `Unknown`: zero.

## Cadeia causal preservada

O predecessor inconclusivo não foi apagado causalmente:

- plano anterior SHA-256
  `cba4c430a2312c55e7126cfec5c146183ed731dee7d6415aec7128ac8e120c4f`;
- recibo anterior SHA-256
  `5a02f2f0413bf8f9752e58bcae753a6ad80b4b81c963ef7dd00e0a70d93aeb8f`;
- veredito anterior:
  `P1295_DISCRIMINATION_INCONCLUSIVE_ORACLE_FALSE_NEGATIVE`;
- `ORACLE_OPACITY_PATTERN`: encerrado após duas revisões sem ganho e nunca
  reaberto nesta campanha.

P2 registrou a revisão intermediária
`ORACLE_OPACITY_STABLE_ENUMERATION_UNAVAILABLE`, inicialmente bloqueada porque
não havia inventário estável e machine-readable. A medição posterior com
`RUSTC_BOOTSTRAP=1` refutou somente a indisponibilidade instrumental e abriu a
classe nova `ORACLE_OPACITY_PINNED_RUSTDOC_JSON`. Não se tratou de terceira
variação de pattern.

## Entradas canônicas

Revalidadas em `2026-09-03T02:46:12.784481386Z`, HEAD
`76fb7336311bdb6497456ab5fdc0a8ce355ff39b`:

| Entrada | SHA-256 |
|---|---|
| `00_nucleo/diagnosticos/p1295-manifest.json` | `545c3c661dd11685ff152d889dab7e796bda0659d91ca447c6d3db3794fc2999` |
| `00_nucleo/diagnosticos/p1295-l0-gate-receipt.md` | `155f2c22359b98739b66507e6fc7c678d187a1d8030a3f7b7ed70a1f31b68d78` |
| `00_nucleo/prompts/shell/watch.md` | `f39a63e6a1e2025d3539b61b34b46ffa3a3b0ea2aa64711ed7699abf3e3f7332` |
| `00_nucleo/prompts/wiring.md` | `81db17f53cdb344cfbee0a5aee230ae1e480cab1856618f846a5a43a9551e0b5` |
| `00_nucleo/prompts/wiring/tests/cli.md` | `5800231392fa0e8a5311a6a5d2664f0f06f85c80b273ffbe80c8fb5e04ed5d1c` |
| contrato L0 revisado | `e39d5206b63b883f9a886a01ba2aaa7ff94259542a33693d5927a0a20445b233` |
| teste externo revisado | `8dae68ffc12f2708b6bbe3d503a7d0d90a2cb92681762ff5a992ca3f178c7abf` |
| P1137 W3 protegido | `cfd6ec5294f410c3625f7a6183db6ad5af776e4b3f8c5af25a96c284b7d5789f` |
| RED receipt revisado | `ac9a8fb41675026bafc884575b387998c45b8318f110508293e99faca1ec4d05` |

A confirmação humana continua `Continue`. O novo oráculo fixa rustc/rustdoc
1.92.0, cargo 1.92.0 e schema rustdoc JSON 56. Divergência de qualquer pin,
JSON ausente/inválido ou variante desconhecida é falha fechada.

## Capacidades e ambiente

- papel: P3 adversário/calibrador;
- escrita no repositório: somente este plano e
  `p1295-discrimination-receipt.json`;
- escrita temporária: somente `/tmp/p1295-p3-r2.0Linlh`;
- produto, L0s, testes protegidos, manifesto, RED, gate e selo: não editados;
- segregação alegada: causal por entradas, capacidades, ordem e allowlist;
- isolamento físico de leitura: não alegado.

Cópia e cache CoW:

```text
rsync -a --exclude=.git --exclude=target --exclude=lab \
  --exclude=00_nucleo/materialization --exclude=00_nucleo/context \
  /repos/Antigravity/typst-crystalline/ /tmp/p1295-p3-r2.0Linlh/work/
cp -a --reflink=always /repos/Antigravity/typst-crystalline/target \
  /tmp/p1295-p3-r2.0Linlh/target
```

- digest canônico da cópia com controle restaurado:
  `50d4cfcefa9bd535a63c36e10d5d3f4600f92752f0ab325e2fcbf6673c9fd350`;
- runner `run_case.sh`:
  `55fdaf2815c4066b969704ca4926a016bc47103dcec1405de23ccab45e460785`;
- testemunha `trace_witness.sh`:
  `d0103c6e4044c6aef78c41b793b52e798f0c1af12ad8f30494902285fb2200cf`;
- resultados TSV:
  `8fa6aa1a860e08bdfd7990e676f17c125c0bbd8d403f98147c489c4ff5b6377d`.

## Controle positivo

O controle temporário materializa somente o necessário para calibração:

- `WatchSnapshot` público com estado privado não vazio;
- captura única de paths e fingerprints;
- `wait_for_change_since` usa exatamente o snapshot recebido;
- compatibilidade de `wait_for_change`;
- L4 normaliza dependências, captura, publica/descarta, faz eviction e espera;
- nenhum campo público, método inherent público, trait direto ou função livre
  extra referenciando o token.

Hashes: `watch.rs`
`10c975f2c85a7cc479c79f60f7fdb9448873508efaf03151556c529b8d04e797`;
`main.rs`
`587d16bcc8f48a2272fe2753112836955b9197a684cde618145df5992419ad7b`.
É um controle sintético em `/tmp`, não uma implementação candidata.

## Instrumentação determinística

Controle e todos os mutantes contêm a mesma instrumentação privada, ativada
somente por `P1295_ADVERSARIAL_TRACE`. Ela acrescenta em `/tmp` eventos
append-only `snapshot:<paths>` e `commit` nas próprias chamadas reais. Não
altera ordem, dependências, fingerprints, polling, timeout, estímulos nem
resultado de compilação.

Consequentemente:

- MW1 é distinguido pela ordem real `commit -> snapshot`;
- MW3 é distinguido pelo argumento real de `snapshot`, contendo input mas não
  asset;
- não há atraso artificial, sleep corretivo, timeout maior ou
  retry-until-pass;
- MW2 usa diretamente o teste protegido de pré-mudança e não depende do traço.

## Mutantes e testemunhas

| ID | Mutação mínima | Patch SHA-256 | Ficheiro mutado SHA-256 | Testemunha específica |
|---|---|---|---|---|
| MW1 | move `snapshot(&dependencies)` para depois de publish/discard | `06f764465206af2dcc2d65e2aa124e00a038783e09325a06c4cb3304a00039d8` | `main.rs` `568e21e20fb31032665da724183a41c1d6f15bf58809ead44a863edb23784968` | trace: `Violated: commit line 1 precedes snapshot line 2` |
| MW2 | recaptura fingerprints em `wait_for_change_since`, ignorando as recebidas | `0ae7ba2388b0f33aed2315b1972ef70b6035a88f76cae4633c7a1902829e272c` | `watch.rs` `035e8430489610eab949a770c9cf233790b6574ef823ce4b1d0a842af6b67fa4` | teste `snapshot_anterior_detecta_alteracao_de_mesmo_tamanho_sem_recaptura`: não retorna para a alteração prévia |
| MW3 | troca inventário por `vec![input]`, descartando asset | `8d2bb54ed0ff7c2b48749c8503ef14435303c321c3630644d4ab7d10a5403984` | `main.rs` `61639bea7e568eb478dc6b7e60062af386ab0c306671515ad0b134a65b61189c` | trace: `Violated: snapshot retains input but discards asset` |

## Budget e ordem

Budget congelado: máximo duas revisões por `reason_code`; recorte focal antes
de corpus/ordens; duas revisões sem ganho bloqueiam; `Unknown` nunca conta.

Esta campanha consumiu zero revisões. O controle foi verde antes dos ataques,
logo foram autorizados sucessivamente:

1. focal MW1, MW2, MW3;
2. ordem direta MW1→MW2→MW3;
3. ordem inversa MW3→MW2→MW1;
4. repetição do controle após todas as trocas.

`ORACLE_OPACITY_PATTERN` não foi reaberto. O rustdoc oracle permaneceu no
schema/toolchain pinado e passou duas vezes.

## Resultados e custos

| Execução | Witness | Exit | Classificação | Custo ms |
|---|---|---:|---|---:|
| control-infra | contrato completo, 6 testes | 0 | Preserved | 29271 |
| control-p1137 | P1137 focal | 0 | Preserved | 36470 |
| control-order | trace de ordem | 0 | Preserved | 3078 |
| control-dependencies | trace de paths | 0 | Preserved | 2938 |
| focal-mw1 | ordem | 1 | Violated | 3230 |
| focal-mw2 | same-length prévia | 101 | Violated | 6422 |
| focal-mw3 | paths | 1 | Violated | 5245 |
| direct-mw1 | ordem | 1 | Violated | 3281 |
| direct-mw2 | same-length prévia | 101 | Violated | 6438 |
| direct-mw3 | paths | 1 | Violated | 5342 |
| inverse-mw3 | paths | 1 | Violated | 3088 |
| inverse-mw2 | same-length prévia | 101 | Violated | 6383 |
| inverse-mw1 | ordem | 1 | Violated | 5152 |
| repeat-control-infra | contrato completo, 6 testes | 0 | Preserved | 16674 |
| repeat-control-p1137 | P1137 focal | 0 | Preserved | 23663 |
| repeat-control-order | trace de ordem | 0 | Preserved | 3042 |
| repeat-control-dependencies | trace de paths | 0 | Preserved | 2969 |

Custo acumulado registrado: `162686 ms`. O custo inclui compilação incremental
e execução; nenhum valor foi usado para afrouxar fronteiras.

Resultados internos relevantes:

- contrato completo: `6 passed` nas duas execuções;
- P1137: `1 passed` nas duas execuções;
- MW1: mesma testemunha de ordem nas três execuções;
- MW2: mesmo timeout interno de 2 s e mesma fase nas três execuções;
- MW3: mesma testemunha de perda do asset nas três execuções.

## Vetores e score

- controle inicial: `[Preserved, Preserved, Preserved, Preserved]`;
- focal: `[MW1=Violated, MW2=Violated, MW3=Violated]`;
- direta: `[Violated, Violated, Violated]`;
- inversa: `[Violated, Violated, Violated]`;
- controle final: `[Preserved, Preserved, Preserved, Preserved]`;
- mutantes válidos únicos: 3;
- mutantes válidos mortos: 3;
- mutation score: `1.0`;
- survivors: 0;
- `Unknown`: 0.

O veredito autoriza somente P4 a avaliar/produzir o selo com estes hashes. Não
autoriza P3 a implementar nem declara equivalência funcional geral.

## Proveniência da árvore produtiva

Antes de sobrescrever os dois artefactos P3, em
`2026-09-03T02:46:12.784481386Z`:

- HEAD: `76fb7336311bdb6497456ab5fdc0a8ce355ff39b`;
- `git status --porcelain=v1` SHA-256:
  `1e378d5a9f430d7f328e60736c3bb3559f204ae1533f62e80612aa92219eb77c`;
- `git diff HEAD --stat` SHA-256:
  `8d85e81a64a47c2b9bf82628d9d4228db5a5e256d175a9d2b905ca27fdaafbfd`;
- tracked stat: `6 files changed, 276 insertions(+), 78 deletions(-)`;
- os dois paths P3 já eram untracked, portanto a substituição de conteúdo não
  muda o status porcelain nem entra no tracked diff stat;
- todas as entradas canônicas conservaram seus hashes.
