# P1297/R1 — receipt de implementação P5-R1

## Veredito estreito

`R1_CAPABILITY` foi materializado nos dois consumers produtivos autorizados e
está pronto exclusivamente para verificação independente P6. Este receipt não
executa nem antecipa contrato protegido, testes, mutantes, superfícies, lint
arquitetural, certificado ou equivalência funcional geral.

Regime: protocolo Tekt completo, papel P5-R1 implementador. A execução foi
segregada por entradas congeladas, ordem causal, capacidades de escrita e
artefatos canônicos no workspace compartilhado. Não se alega isolamento
técnico de leitura do filesystem.

Estado terminal preservado:

```text
P1294_BLOCKED_P1295_BLOCKED_P1296_BLOCKED_P1297_NOT_CERTIFIED
```

## Papel, entradas e capacidades

Executor: sessão Codex `P5-R1_IMPLEMENTER`, posterior ao selo R1 e sem receber
output privado P3.

Entradas lidas e congeladas:

| Artefato | SHA-256 |
|---|---|
| skill `tekt-materializacao-segregada/SKILL.md` | `66990d349a9e89851686cd94590a84711c69364f76b6df501230f64daf3b0c48` |
| referência `papeis-e-capacidades.md` | `f59f44c4e53e89651963115c582872b4d3cd59d89689d103baa9ef8b464d2417` |
| referência `artefatos-e-gates.md` | `16db4af3a8a21a27e1bfc4a5dd00c976f0fc946ba46dc8df1a663171d823f72d` |
| `00_nucleo/materialization/typst-passo-1297.md` | `5116ffc535bb8cbd33ca8fb449b492ddfcf9e133246d7143c27a97960748c682` |
| `00_nucleo/prompts/shell/watch.md` | `0988aae004f94b52730c1b605b20249e261bdeac70249b7f99d2f0deeee23483` |
| `00_nucleo/prompts/wiring.md` | `4e0661d71303fc367d9f29992809e21ecc3b66fcb6e4869eb45d705f8fe5876d` |
| `00_nucleo/prompts/wiring/tests/cli.md` | `a1190628c91f0f8c958bad2babe68e5f425931e7f885e8d97bc8b95f9ecc13ad` |
| `00_nucleo/prompts/infra/tests/p1297_watch_capability_contract.md` | `4b30a7da9310892ad639ba2483fd8d995f6cae4adb4f5ec959247b7911f583e8` |
| `00_nucleo/diagnosticos/p1297-r1-l0-gate-receipt.md` | `11777b20d0fc62256077339c35b86c559ba90cce167da2aee2ca081a9bd446e2` |
| `00_nucleo/diagnosticos/p1297-r1-contract-seal.json` | `2710fbbbe8748176764d37be40b40c30d93d39f1442e02064f32c7ff31348e92` |
| baseline `03_infra/src/watch.rs` | `0232f1baa3b06bde809941938ea0633982a78ec066ce8e38466f346e63403a35` |
| baseline `04_wiring/src/main.rs` | `03dfc017bb832264b16d50c6314c1e0d48304f657f2054af378032944c747678` |

Capacidade de escrita exercida exclusivamente em:

- corpo produtivo de `03_infra/src/watch.rs`;
- corpo produtivo de `04_wiring/src/main.rs`;
- este receipt.

Não foram alterados headers `@prompt`/`@prompt-hash`, L0s, owner/consumer do
contrato, consumer CLI, manifesto, ledger, gate, selo, RED receipt, plano ou
receipt adversarial, predecessors, mutantes, cópias ou logs privados. O
consumer protegido, corpos de controle/mutantes e outputs privados P3 não
foram lidos nem recebidos. Nenhuma outra materialization ou entrada de
`00_nucleo/context/` foi listada ou lida.

## Materialização R1

Em `03_infra/src/watch.rs`:

- `ArmedWatch` é público e contém somente um campo privado `WatchSnapshot` já
  materializado;
- `arm(paths)` chama a captura imediata uma única vez e armazena o resultado;
- `publish(self, staging, destination)` faz uma única operação de rename,
  devolve o snapshot no sucesso e, na falha, tenta remover staging em
  best-effort antes de devolver o `io::Error` original;
- `abandon(self, staging)` remove staging em best-effort, não recebe nem toca
  destino e devolve o snapshot capturado;
- `WatchSnapshot`, `snapshot`, `wait_for_change_since` e `wait_for_change`
  permanecem; `wait_for_change_since` continua sem recaptura;
- `commit_output` e `discard_output` foram removidos da superfície pública.

Em `04_wiring/src/main.rs`, `run_watch` executa somente:

```text
compile -> normalize -> arm
  success: publish (erro fatal exit 2)
  error:   abandon
-> crystalline_evict(10) -> wait_for_change_since(snapshot retornado)
```

L4 não cria tipo de watch, não conhece fingerprints e não acrescenta rename,
unlink, polling ou outro I/O de watch.

## Hashes produtivos antes/depois

| Consumer | SHA-256 antes | SHA-256 depois |
|---|---|---|
| `03_infra/src/watch.rs` | `0232f1baa3b06bde809941938ea0633982a78ec066ce8e38466f346e63403a35` | `34188c0a3bdc129abc5c3eb9636a5fc5a701fc68f61c530c7a7f08a80a961b3c` |
| `04_wiring/src/main.rs` | `03dfc017bb832264b16d50c6314c1e0d48304f657f2054af378032944c747678` | `40a7885a2d4aa6c9a02921461be44928f14e75f072bcda203433c55d1771a6f7` |

Após a implementação, os hashes do Passo, dos três L0s, do owner textual, do
gate humano e do selo continuaram exatamente iguais aos valores da tabela de
entradas. As linhas de linhagem permaneceram `a0c963e0` em L3 e `248c887e` em
L4. A suíte CLI permaneceu fora da capacidade P5-R1.

## Verificações permitidas do implementador

Ordem de execução:

1. `rustfmt --edition 2021 --check 03_infra/src/watch.rs 04_wiring/src/main.rs`
   retornou exit `1` exclusivamente pela forma multilinha do literal de
   `ArmedWatch`;
2. a forma indicada foi corrigida focalmente por `apply_patch`, sem mudança de
   contrato;
3. repetição do mesmo comando `rustfmt` retornou exit `0`;
4. `cargo check -p typst-infra -p typst-wiring --bins --offline` retornou exit
   `0`; houve somente warnings preexistentes fora dos dois corpos alterados;
5. `git diff --check` retornou exit `0`;
6. busca restrita aos dois sources confirmou presença de
   `ArmedWatch`/`arm`/`publish`/`abandon` e ausência de
   `commit_output`/`discard_output`.

Testemunho final anterior à criação deste receipt:

- medido em `2026-09-03T10:05:15,456073431-03:00`;
- HEAD `76fb7336311bdb6497456ab5fdc0a8ce355ff39b`;
- commit `docs: record blocked step 1294 verification`;
- data do commit `2026-09-02T22:23:37-03:00`;
- working tree não commitida, SHA-256 do output exato de
  `git status --porcelain=v1`:
  `ea3d52cd692ecc3c7afda63deed078861575038c97a6f9932bee863e804bd889`;
- SHA-256 do output exato de `git diff HEAD --stat`:
  `279f1504face23af3fc9a528279229706da52bbc5fdddf249d74d07fb67d22e4`;
- `git diff --cached --quiet` retornou exit `0`.

Não foram executados `cargo test`, contrato protegido, mutantes, superfícies,
stress, build release, `crystalline-lint`, `--fix-hashes` ou qualquer gate
reservado ao verificador.

## Claim e próxima autoridade

Claim máxima deste receipt:

```text
O candidato produtivo P1297/R1 materializa a capacidade ArmedWatch e a ordem
de composição descritas pelos L0s confirmados, compila nos bins permitidos e
está pronto para verificação independente do fragmento selado; não há alegação
de equivalência funcional geral, certificação P1297 ou absolvição de
predecessores.
```

Próxima e única autoridade autorizada: `P6_FINAL_VERIFIER`. P6 deve tratar os
dois hashes produtivos “depois” e o SHA-256 externo deste receipt como entradas
somente leitura. P5-R1 não autoriza correção silenciosa, R2, R3, certificado,
relatório final ou qualquer outra escrita por si próprio.

```text
P1297_R1_IMPLEMENTED_READY_FOR_P6_VERIFICATION
```
