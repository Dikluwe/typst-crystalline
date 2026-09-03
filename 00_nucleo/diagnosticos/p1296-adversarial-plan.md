# P1296 — plano adversarial e campanha focal O1

## Veredito

```text
P1296_DISCRIMINATION_BLOCKED_MO1_SURVIVED_RETURN_TO_P2
```

O protocolo completo foi exercido no papel P3, adversário/calibrador, com
segregação causal por entradas, capacidades, ordem e artefatos. O filesystem é
compartilhado e não se alega isolamento técnico de leitura. Nenhum teste, L0,
header, produto ou evidência P1295 foi editado no repositório. Cópia, runner,
mutantes e logs ficaram somente em `/tmp/p1296-p3.47dAI2`.

O controle foi `Preserved` antes e depois dos ataques. MO2 foi `Killed` na fase
`armamento após erro transitório`; MO3 foi `Killed` na fase `recuperação`.
MO1, contudo, foi `Survived`: o teste focal exato terminou com exit `0` e
`1 passed`. Mutation score: `2/3 = 0.6666666666666666`; survivors: `MO1`;
`Unknown`: zero. O score exigido `1.0` não foi atingido.

Por isso, a campanha parou no recorte focal. As ordens direta e inversa não
foram executadas: o próprio Passo 1296 autoriza ordens somente após o recorte
focal verde. Esta omissão não é `Unknown` nem crédito; é gate bloqueado por
survivor já demonstrado. P3 não sela, não certifica e devolve o contrato autoral
a P2 para revisão.

## Entradas congeladas

Revalidadas antes da campanha em `2026-09-03T01:52:17,466406250-03:00` sobre
HEAD `76fb7336311bdb6497456ab5fdc0a8ce355ff39b`:

| Entrada | SHA-256 |
|---|---|
| `00_nucleo/materialization/typst-passo-1296.md` | `5a7f7c5d364fa6a373c1d411b8710131c68198bfeaaa4b19ec53fb17f7b8c00b` |
| `00_nucleo/diagnosticos/p1296-manifest.json` | `7c9e2f75aab348ce434c293ea0b38259fc8c42b39360d6ca3b63a17491edd7d0` |
| `00_nucleo/prompts/wiring/tests/cli.md` | `ac81dcf4404d505cd10db05964351b35f5ba0e9b959d83ef903fd4b34241a49b` |
| `00_nucleo/diagnosticos/p1296-l0-receipt.md` | `224dd65d269f3a81b132f2679b2c373ab5bcf910d0ed21e94b554dbe761924a2` |
| `00_nucleo/diagnosticos/p1296-test-receipt.json` | `b958974c3d9dd895a10cfd736eb64fd230d6a5fb98c284eb4aa906ba89b88768` |
| `04_wiring/tests/cli.rs` | `addf970325c6b9df11482b6d7f28bd1099cbb0d225080044268e3dc3a304206b` |
| `04_wiring/src/main.rs` | `e119a46479e1d2e7b3f0052df2b57ab5d31be573969bbe841dd8ee2d0e3e9c48` |
| `03_infra/src/watch.rs` | `d543b1c32844b6f63085635ae34fa2beb01ba989c4ebed7f789bb5ae77e9ee41` |
| `03_infra/tests/p1295_watch_contract.rs` | `8dae68ffc12f2708b6bbe3d503a7d0d90a2cb92681762ff5a992ca3f178c7abf` |
| `00_nucleo/diagnosticos/p1295-verification-receipt.json` | `06104b7d41f8e456e67df90c8036ba4362af67f3247880278bfa587e1d2c0573` |
| `/tmp/p1295-p6-cli-1.log` | `919d16a5b6bb24050dcb6dcef20ef57ac8b9cd8f4fb82d018c31d130e6dd508b` |

Antes dos dois artefatos P3, a medição pós-campanha em
`2026-09-03T02:32:30,933515349-03:00` manteve o mesmo HEAD, SHA-256 de
`git diff HEAD --stat` igual a
`d7552c82635cd0f81f6995b0454860eb98fca3e0f6fe7649ea5eb91bc3943582`
e SHA-256 de `git status --porcelain=v1` igual a
`fb86d50e1d89ab509a4b3a57bac9930568af789a3c46dd63347984c2f2a9ec3c`.

## Capacidades e ambiente temporário

- escrita no repositório: somente este plano e
  `p1296-discrimination-receipt.json`;
- escrita temporária: `/tmp/p1296-p3.47dAI2`;
- contexto causal: Passo 1296 explicitamente autorizado, manifesto, L0,
  receipts P1/P2, consumer P2, produtos congelados e evidência P1295 pinada;
- materializações/contextos alheios: não listados nem lidos;
- produto, L0, headers, teste e predecessor: somente leitura no repositório.

Cópia e cache:

```text
rsync -a --exclude=.git --exclude=target --exclude=lab \
  --exclude=00_nucleo/materialization --exclude=00_nucleo/context \
  /repos/Antigravity/typst-crystalline/ /tmp/p1296-p3.47dAI2/work/
cp -a --reflink=auto /repos/Antigravity/typst-crystalline/target \
  /tmp/p1296-p3.47dAI2/target
```

Runner `run_case.sh`: SHA-256
`02872e4404142a507bc5e19d5f74c91f9d588c22218a26ae2eb3838ae5703062`.
Resultados TSV: SHA-256
`cda46fed6dfd87e6a152facad381ff2b35117c149f7e5640ba6a8a5538ea2cf0`.
Ao final, a cópia foi restaurada aos hashes do controle para `main.rs`,
`cli.rs` e `watch.rs`.

## Mutantes

| ID | Mutação | Patch SHA-256 | Hash(es) mutado(s) | Resultado focal | Testemunha |
|---|---|---|---|---|---|
| MO1 | no erro, `discard_output` antes de `snapshot` | `b2a2656a662b4910958a1d15343b5db0ff3364edb9ae7f777c3feb3b36578e0b` | `main.rs` `807c2432e3328a17827156959899596eff3e4ce4f792f1a54893918e9b1858c6` | `Survived` | exit `0`; `1 passed`; nenhuma fase falhou |
| MO2 | omite `discard_output` no erro | `20d3d2576457374747ec76fd16a6fdc4a7c1c30b995283a69d142b5bcac37c5b` | `main.rs` `4b0c3bda2385a214b5f53d365a075ec2abae42f37a05f870d7c1f0f09e2ee11f` | `Killed` | timeout causal em `armamento após erro transitório` |
| MO3 | restaura observador temporal antigo e atrasa snapshot de erro em 750 ms | `44cb5749d4e70b49082ec0fb120b8579749078cd865b94648ae4af6047439fd0` | `main.rs` `5f652da30f88378a65e6effb22395fd7b49fa5012bec4137bf1083fce4594b76`; `cli.rs` `e0dc7e8129729f1f15361fc76d1e82a908a63de2faeb1741fc540fd3bbc062e6` | `Killed` | timeout causal em `recuperação` |

O hash de `mo3-cli.rs` coincide exatamente com o consumer pós-resselo P1 e
pré-P2, confirmando a restauração byte a byte do observer temporal antigo.
Nenhum timeout foi aumentado e não houve retry-until-pass.

## Comando focal e execuções

Cada linha usou exatamente:

```text
CARGO_NET_OFFLINE=true CARGO_TARGET_DIR=/tmp/p1296-p3.47dAI2/target \
  cargo test -p typst-wiring --test cli \
  p1137_watch_dependencias_recuperacao_e_filtro -- --exact --nocapture
```

| Execução | Início | Fim | ms | Exit | Classificação | Log SHA-256 |
|---|---|---|---:|---:|---|---|
| control-initial | `2026-09-03T02:30:20,420993098-03:00` | `2026-09-03T02:30:27,172185350-03:00` | 6748 | 0 | Preserved | `12e8fb470976086e13529eb251b95d03099a683676a9fdc91496c2f8794ba6dc` |
| focal-mo1 | `2026-09-03T02:30:36,181062267-03:00` | `2026-09-03T02:30:39,830157904-03:00` | 3646 | 0 | Survived | `1f69e0f0a9014a6081966a9628b6e6603be14f93ec38aa489cb45f46ec772297` |
| focal-mo2 | `2026-09-03T02:30:45,183517386-03:00` | `2026-09-03T02:31:08,227260179-03:00` | 23041 | 101 | Killed | `a0433f40d119bd719e7e1cffc0e15883838f165b067f49c93bee44523ce98903` |
| focal-mo3 | `2026-09-03T02:31:21,516218832-03:00` | `2026-09-03T02:31:45,009357053-03:00` | 23490 | 101 | Killed | `8674260e8357332f94df754719e1360a323d9dd6ce1060a6eba2cae4ed22e633` |
| control-final | `2026-09-03T02:32:11,268955981-03:00` | `2026-09-03T02:32:14,944822699-03:00` | 3673 | 0 | Preserved | `282e871ae489a57a0a54e90d92ebf49ff260eb99eb9cea9ea03c5bdd55211e51` |

Custo acumulado: `60598 ms`. Os dois controles passaram `1/1`; MO2 e MO3
falharam `0/1` nas fases específicas registradas.

## Diagnóstico adversarial de MO1

O sentinel é removido por MO1 antes do snapshot, mas o observer consulta sua
existência em polls de 50 ms. Entre a remoção e o próximo poll, o processo pode
capturar o snapshot imediatamente. Quando o teste enfim observa a remoção e
escreve `Recovered`, essa escrita já é posterior ao snapshot e a recuperação
funciona. Foi exatamente o comportamento observado: o fault de ordem permaneceu
presente e o teste focal passou.

Logo, no contrato atual, “observei a remoção” não implica logicamente “o snapshot
já ocorreu” quando se ataca a própria ordem que sustenta essa inferência. O
observer discrimina ausência de descarte e a janela temporal antiga, mas não
discrimina a inversão mínima `discard -> snapshot`.

`reason_code`: `O1_MO1_REMOVAL_DOES_NOT_PROVE_PRIOR_SNAPSHOT_UNDER_ORDER_MUTATION`.
Refutador: o teste focal exato rejeitar o MO1 mínimo sem timeout maior, retry,
sleep corretivo ou comportamento/sincronização adicional do mutante. O resultado
observado não satisfaz esse refutador e sustenta a insuficiência.

## Budget, ordens e próxima autoridade

- revisões P3 do teste/oráculo: zero; P3 não possui essa capacidade;
- controle e MO1–MO3 focais: executados;
- ordem direta `MO1 -> MO2 -> MO3`: `NOT_RUN_BLOCKED_BY_FOCAL_SURVIVOR`;
- ordem inversa `MO3 -> MO2 -> MO1`: `NOT_RUN_BLOCKED_BY_FOCAL_SURVIVOR`;
- score exigido: `1.0`; score observado: `0.6666666666666666`;
- survivors: `[MO1]`; `Unknown`: `[]`.

P2 deve revisar o contrato autoral/observer para discriminar MO1. P3 não propõe
nem executa essa edição e uma campanha posterior precisa receber novo hash do
consumer/receipt, reiniciando da primeira fase afetada. Até lá, P4 não está
autorizado e permanece:

```text
P1295_BLOCKED_P1296_NOT_CERTIFIED
```
