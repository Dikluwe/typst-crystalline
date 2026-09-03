# P1297/R1 — recibo P1 de autoria L0 e gate humano

## Veredito

Regime completo Tekt, papel P1-R1 — autor independente do L0 do redesenho
ativo. A execução foi segregada por entradas causais, capacidades de escrita,
ordem e artefatos canônicos no workspace compartilhado. Não se alega
isolamento técnico de leitura do filesystem nem equivalência funcional geral.

P1295 está confirmado e materializado, mas não certificado. P1296/O1 está
refutado, terminalmente bloqueado após duas revisões e supersedido por R1. No
eixo watch existe uma única obrigação produtiva ativa: `R1_CAPABILITY`.

## Papel, entradas e capacidades

Executor: sessão Codex P1-R1, autora somente dos três Prompts L0, do resselo
mecânico dos três headers proprietários e deste receipt.

Entradas causais principais:

| Artefato | SHA-256 |
|---|---|
| `00_nucleo/materialization/typst-passo-1297.md` | `5116ffc535bb8cbd33ca8fb449b492ddfcf9e133246d7143c27a97960748c682` |
| `00_nucleo/diagnosticos/p1297-manifest.json` | `5bf3f29e353403006b90971e05f786ad2548648c053b9db59f879841aecdf322` |
| `00_nucleo/diagnosticos/p1297-redesign-ledger.json` | `fae6dfbd8af6b07bd33b07b5d8e0d63a896114dcef8ee02c4973ec2781c3b4db` |
| `00_nucleo/diagnosticos/p1295-l0-gate-receipt.md` | `155f2c22359b98739b66507e6fc7c678d187a1d8030a3f7b7ed70a1f31b68d78` |
| `00_nucleo/diagnosticos/p1295-verification-receipt.json` | `06104b7d41f8e456e67df90c8036ba4362af67f3247880278bfa587e1d2c0573` |
| `00_nucleo/diagnosticos/p1296-l0-receipt.md` | `224dd65d269f3a81b132f2679b2c373ab5bcf910d0ed21e94b554dbe761924a2` |
| `00_nucleo/diagnosticos/p1296-test-receipt.json` | `be8aaf3537bb068aab8eff1c5b656fe74c56cec8bd144f22e48a12b486d3e8a0` |
| `00_nucleo/diagnosticos/p1296-adversarial-plan.md` | `8b8a8435ff794f7d0d8ba5ecc0c0af2c8f88ac2b3133bc05dd8eaa2c2eb9ce6b` |
| `00_nucleo/diagnosticos/p1296-discrimination-receipt.json` | `51073959b26cc834f1f1dbe69b5c4e70b5e7f1c71d28282b7424e0306e641525` |

Foram lidos integralmente a skill `tekt-materializacao-segregada`, suas duas
referências obrigatórias, o Passo 1297 explicitamente autorizado, manifesto,
ledger, ADR-0107, ADR-0108, ADR-0127, ADR-0129, os três L0s, headers dos três
consumers e receipts causais necessários. Busca em `00_nucleo/adr/` não
localizou ADR local própria de materialização segregada. Nenhuma outra pasta ou
entrada de `00_nucleo/materialization/` ou `00_nucleo/context/` foi listada ou
lida.

Escritas realizadas exclusivamente na allowlist P1-R1:

- `00_nucleo/prompts/shell/watch.md`;
- `00_nucleo/prompts/wiring.md`;
- `00_nucleo/prompts/wiring/tests/cli.md`;
- somente a linha `@prompt-hash` em `03_infra/src/watch.rs`;
- somente a linha `@prompt-hash` em `04_wiring/src/main.rs`;
- somente a linha `@prompt-hash` em `04_wiring/tests/cli.rs`;
- este receipt.

Manifesto e ledger permaneceram somente leitura. Não foram editados corpos
Rust, contratos, testes, mutantes, selos, certificados, reports ou
predecessores.

## Proveniência temporal e HEAD

- início da medição P1-R1: `2026-09-03T08:41:53,951750646-03:00`;
- medição imediatamente anterior ao resselo: `2026-09-03T08:46:05,775775517-03:00`;
- medição imediatamente posterior ao resselo: `2026-09-03T08:46:07,406437615-03:00`;
- testemunho final anterior a este receipt: `2026-09-03T08:47:03,084485967-03:00`;
- HEAD durante toda a fase: `76fb7336311bdb6497456ab5fdc0a8ce355ff39b`;
- commit: `docs: record blocked step 1294 verification`;
- data do commit: `2026-09-02T22:23:37-03:00`;
- working tree: não commitida, cadeia P1294/P1295/P1296/P1297 preservada;
- índice: vazio no testemunho anterior ao receipt.

## Medição e decisão R1

No produto congelado, `04_wiring/src/main.rs:95-101` já executa
`compile -> normalize -> snapshot`; o ramo de erro descarta staging em
`:114-115`, e `:119-122` espera com o snapshot capturado. Em L3,
`03_infra/src/watch.rs:42-45` captura imediatamente e `:49-59` não recaptura.

P1296 demonstrou que o sentinel não discrimina a inversão mínima: MO1
`discard -> snapshot` sobreviveu à revisão 1, resultando em score `2/3`, um
survivor e zero `Unknown`. A revisão 2 por FIFO regrediu o primeiro controle
positivo durante recompilação por asset após 20 segundos, não executou MO1 e
foi removida. O consumer voltou byte a byte ao baseline. Portanto O1 é
histórico refutado, não obrigação vigente.

Classificação ADR-0107/0108: a ordem é mecânica, mas a mecânica é o observável
causal deste processo, pois recuperação incorporada ao baseline pode deixar de
provocar compilação. A decisão R1 torna finalização consumidora de uma
capacidade `ArmedWatch` que já contém o `WatchSnapshot`; a inversão deixa de ser
expressável pelo consumer L4. O refutador permanece explícito: captura lazy,
helper cru chamável por L4, recaptura, troca do snapshot, survivor R1 ou
regressão positiva encerram este desenho.

## Bytes L0 autorados e estado histórico reconciliado

| Prompt | SHA-256 baseline P1297 | SHA-256 autorado antes do resselo | SHA-256 final ressellado | `Hash do Código` final |
|---|---|---|---|---|
| `00_nucleo/prompts/shell/watch.md` | `f39a63e6a1e2025d3539b61b34b46ffa3a3b0ea2aa64711ed7699abf3e3f7332` | `b21a465132329b79cfcdd9958705a467001c4f0c68c1af6016493c7d043c4569` | `0988aae004f94b52730c1b605b20249e261bdeac70249b7f99d2f0deeee23483` | `67a936f2` |
| `00_nucleo/prompts/wiring.md` | `81db17f53cdb344cfbee0a5aee230ae1e480cab1856618f846a5a43a9551e0b5` | `0cc335d4d4c76539d9ba1c9d804d7c5e6f87b1b2faf15c5e4f0d9ef505ecd098` | `4e0661d71303fc367d9f29992809e21ecc3b66fcb6e4869eb45d705f8fe5876d` | `0d0307f2` |
| `00_nucleo/prompts/wiring/tests/cli.md` | `ac81dcf4404d505cd10db05964351b35f5ba0e9b959d83ef903fd4b34241a49b` | `dc9b7a6088a007e870453c50c80956bed30232b8670b85f5c47377b7fa52e9d0` | `a1190628c91f0f8c958bad2babe68e5f425931e7f885e8d97bc8b95f9ecc13ad` | `f6ff401b` |

Os bytes finais fixam:

- `ArmedWatch` contendo somente o snapshot materializado por `arm(paths)`;
- `publish(self, ...)` e `abandon(self, ...)` consumindo a capacidade e
  devolvendo exatamente o snapshot capturado;
- rename atômico, cleanup best-effort em falha e preservação do erro original;
- descarte best-effort sem tocar no destino válido;
- helpers crus de commit/discard não chamáveis por L4;
- compatibilidade explícita de `WatchSnapshot`, `snapshot`,
  `wait_for_change_since` e `wait_for_change`, sem permitir contorno da
  finalização consumidora;
- composição `compile -> normalize -> arm`, ramos `publish/abandon`, depois
  `evict -> wait_for_change_since` com o mesmo snapshot;
- L4 sem tipo próprio, fingerprint, rename, unlink ou I/O de watch;
- P1137 funcional preservado, mas sem reivindicar que o sentinel discrimina a
  inversão;
- prova determinística transferida ao futuro owner/consumer externo exclusivo
  R1, cuja criação pertence a P2 somente após este gate.

P1295 foi marcado como confirmado/materializado, porém não certificado;
P1296/O1 foi marcado como histórico refutado/supersedido. Nenhuma medição,
receipt ou refutador foi apagado.

## Resselo mecânico e reconstrução dos consumers

O dry-run inicial executou:

```text
crystalline-lint --fix-hashes --dry-run .
```

Exit 0 e allowlist resolvida exatamente:

```text
03_infra/src/watch.rs       0420b249 -> a0c963e0
04_wiring/src/main.rs       77763e73 -> 248c887e
04_wiring/tests/cli.rs      58c349d6 -> cf02a8f0
```

Nenhum quarto path foi listado. `crystalline-lint --fix-hashes .` terminou
exit 0, aplicou exatamente esses três headers e reportou
`0 drift warnings remaining`.

| Consumer | SHA-256 inicial | SHA-256 final | `@prompt-hash` final | SHA-256 sem a linha `@prompt-hash` |
|---|---|---|---|---|
| `03_infra/src/watch.rs` | `d543b1c32844b6f63085635ae34fa2beb01ba989c4ebed7f789bb5ae77e9ee41` | `0232f1baa3b06bde809941938ea0633982a78ec066ce8e38466f346e63403a35` | `a0c963e0` | `67a936f2555207c8a01df72b778709c189eb1d3f926fc254cace1e1bed2e5602` |
| `04_wiring/src/main.rs` | `e119a46479e1d2e7b3f0052df2b57ab5d31be573969bbe841dd8ee2d0e3e9c48` | `03dfc017bb832264b16d50c6314c1e0d48304f657f2054af378032944c747678` | `248c887e` | `0d0307f25a414fb465af2ee8d96c12280123c29d2d92e93a561a02510d1aa5f6` |
| `04_wiring/tests/cli.rs` | `addf970325c6b9df11482b6d7f28bd1099cbb0d225080044268e3dc3a304206b` | `56d989a4f79112d59558a71b0dbf609aa4022f9df0290e56afebc1b2b218a42b` | `cf02a8f0` | `f6ff401b72f8ff1723673a3f9ce3070d27954b804283cf76c564ada35ca3bbbd` |

Reconstrução mecânica sobre cada consumer final, substituindo somente o novo
`@prompt-hash` pelo valor inicial acima, produziu respectivamente:

```text
d543b1c32844b6f63085635ae34fa2beb01ba989c4ebed7f789bb5ae77e9ee41
e119a46479e1d2e7b3f0052df2b57ab5d31be573969bbe841dd8ee2d0e3e9c48
addf970325c6b9df11482b6d7f28bd1099cbb0d225080044268e3dc3a304206b
```

Os três resultados coincidem exatamente com os SHA-256 congelados no
manifesto P1297. Isso prova que P1-R1 não alterou corpo Rust, testes ou
contratos: somente as três linhas de linhagem mudaram nesta fase.

## Diff e estado da árvore

Como o baseline P1297 já era uma working tree não commitida, o diff contra HEAD
é cumulativo e inclui as materializações predecessoras preservadas. No
testemunho anterior a este receipt:

```text
00_nucleo/prompts/shell/watch.md      | 228 +++++++++++++++++++++++-----------
00_nucleo/prompts/wiring.md           | 156 ++++++++++++++++++++++-
00_nucleo/prompts/wiring/tests/cli.md | 221 ++++++++++++++++++++++++++++++++-
03_infra/src/watch.rs                 |  33 +++--
04_wiring/src/main.rs                 |  18 +--
04_wiring/tests/cli.rs                |  80 +++++++++---
6 files changed, 630 insertions(+), 106 deletions(-)
```

- SHA-256 do output exato de `git diff HEAD --stat`:
  `ef79219437e7ed01ed4db348e9f15c39d6dc3bcb062d2461cd76110eedfe8ab3`;
- SHA-256 do `git diff` cumulativo dos três L0s:
  `b2d25bd1420f27d2324fc4549872dd7aaae8935b0eb4bf2d3d20db01aeedb3e7`;
- SHA-256 do `git diff` cumulativo dos três consumers:
  `276c5a349eda5b8b1494ebd029a41ad05e6ed0c2478cb5cf25ad0001b968fe11`;
- SHA-256 do mesmo diff dos consumers com `--unified=0`:
  `b5a31ba61736c0fba55354fc829371750d5e16a77275a8a54073e99387f23abe`;
- SHA-256 de `git status --porcelain=v1` antes deste receipt:
  `1a2d80f115a6b7b0e800e7c422956e03e998a3264ebf843baa475e4dde3199ed`;
- entradas status antes deste receipt: `27`;
- índice: nenhum path.

Os hashes inicial/final dos L0s delimitam especificamente a autoria P1297; a
reconstrução acima delimita especificamente as únicas mudanças P1297 nos
consumers, sem atribuir a P1-R1 corpos predecessores já presentes no diff
cumulativo.

## Gates executados e não executados

Executados:

1. `crystalline-lint --fix-hashes --dry-run .` inicial: exit 0, exatamente os
   três consumers allowlisted;
2. `crystalline-lint --fix-hashes .`: exit 0, exatamente três headers e zero
   drift restante;
3. `crystalline-lint --checks v5,v15,v26 --fail-on warning .`: exit 0,
   `No violations found`;
4. `crystalline-lint --fix-hashes --dry-run .` final: exit 0,
   `Nothing to fix`;
5. `git diff --check`: exit 0;
6. ausência verificada de
   `00_nucleo/prompts/infra/tests/p1297_watch_capability_contract.md` e
   `03_infra/tests/p1297_watch_capability_contract.rs`.

Por exigência do gate ADR-0127, contrato R1, RED, mutantes e código produtivo
não foram criados, escritos, compilados, testados nem executados nesta fase.
Nenhum teste, build, stress, superfície, selo, certificado, relatório final ou
staging foi executado. A ferramenta de linhagem apenas ressellou headers; ela
não materializou nem executou o comportamento R1.

## Gate humano requerido

R1 adiciona o tipo público `ArmedWatch`, a função pública `arm`, métodos
consumidores `publish/abandon`, retira helpers crus da superfície de L4 e muda
a forma arquitetural da finalização. ADR-0127 exige confirmação humana após
estes bytes concretos e antes de qualquer contrato, RED, mutante ou código.

Somente após confirmação explícita P2-R1 poderá criar simultaneamente o owner
`00_nucleo/prompts/infra/tests/p1297_watch_capability_contract.md`, seu único
consumer `03_infra/tests/p1297_watch_capability_contract.rs` e o receipt RED
permitido. Criá-los agora deixaria um owner sem consumer durante a parada ou
violaria a ordem causal; ambos permanecem corretamente ausentes.

Este receipt não altera manifesto ou ledger e não autoriza fases posteriores.
O estado continua bloqueado aguardando exclusivamente a confirmação humana do
L0 R1 concreto.

## Confirmação humana recebida

- resposta literal do dono: `Continue`;
- recebida e registrada em: `2026-09-03T09:01:08,941208957-03:00`;
- contexto imediatamente apresentado: resumo explícito do contrato público R1
  `ArmedWatch`/`arm`/`publish`/`abandon` e links para os três L0s concretos;
- interpretação estrita: a resposta confirma exclusivamente os bytes e o
  contrato público de `R1_CAPABILITY`; não confirma R2 nem R3;
- efeito causal: P2-R1 fica autorizado a iniciar somente sua fase segregada,
  dentro da allowlist do manifesto e sem saltar RED, discriminação ou selo.

Esta confirmação não absolve P1295/P1296, não altera a política de `Unknown` e
não autoriza implementação produtiva antes dos predecessores próprios de R1.

P1297_R1_L0_CONFIRMED_READY_FOR_P2
