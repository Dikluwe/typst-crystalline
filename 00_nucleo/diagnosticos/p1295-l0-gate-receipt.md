# P1295 — recibo do gate L0

## Veredito

```text
P1295_L0_AUTHORED_AWAITING_HUMAN_GATE
```

O protocolo completo foi iniciado sob segregação por capacidades e artefatos.
Esta fase foi executada como P1, sem alegação de isolamento técnico de leitura
no filesystem compartilhado. Nenhum teste, oráculo, mutante, selo, corpo Rust,
certificado ou relatório final P1295 foi escrito.

## Predecessor e instante

- HEAD: `76fb7336311bdb6497456ab5fdc0a8ce355ff39b`
- commit: `docs: record blocked step 1294 verification`
- data do commit: `2026-09-02T22:23:37-03:00`
- manifesto congelado:
  `00_nucleo/diagnosticos/p1295-manifest.json`
- SHA-256 do manifesto:
  `545c3c661dd11685ff152d889dab7e796bda0659d91ca447c6d3db3794fc2999`
- início congelado: `2026-09-02T22:50:58-03:00`
- medição pós-resselo final: `2026-09-02T22:56:04-03:00`
- `git diff HEAD --stat` inicial: vazio, SHA-256
  `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`
- não rastreados iniciais: somente
  `p1294-blocked-report.md` e `typst-passo-1295.md`, registrados com hashes no
  manifesto.

## Papel e capacidades exercidos

Executor: sessão Codex primária, papel P1 — autor da obrigação e L0.

Leu o Passo 1295, os L0s vigentes, seus consumers, ADR-0104, ADR-0107,
ADR-0108, ADR-0127 e ADR-0129, além das regras da skill de materialização
segregada. Não houve ADR local específica de materialização segregada
localizada por busca em `00_nucleo/adr/`.

Escritas realizadas somente na allowlist P1:

- três Prompts L0;
- uma linha `@prompt-hash` em cada consumer proprietário;
- manifesto P1295;
- este recibo.

## L0s autorados e resselo

| Obrigação | Prompt L0 | SHA-256 inicial | SHA-256 pós-resselo | Consumer | `@prompt-hash` |
|---|---|---|---|---|---|
| W1 | `00_nucleo/prompts/shell/watch.md` | `a40e3a6214da0cbc79b1cdd6fd6ad7305704e2054deae915348ff6acb4bb091d` | `f39a63e6a1e2025d3539b61b34b46ffa3a3b0ea2aa64711ed7699abf3e3f7332` | `03_infra/src/watch.rs` | `0420b249` |
| W2 | `00_nucleo/prompts/wiring.md` | `539815d1e892117baec6e5e8e51e37a155dadcaa682e3349c11cc66dd52f623c` | `81db17f53cdb344cfbee0a5aee230ae1e480cab1856618f846a5a43a9551e0b5` | `04_wiring/src/main.rs` | `77763e73` |
| W3 | `00_nucleo/prompts/wiring/tests/cli.md` | `7f48645d50982a0adc2bdeabbc5b62b40390da795d8ee9f2caca4c793e351e7b` | `5800231392fa0e8a5311a6a5d2664f0f06f85c80b273ffbe80c8fb5e04ed5d1c` | `04_wiring/tests/cli.rs` | `48bba6d8` |

O L0 `shell/watch.md` agora possui exclusivamente
`03_infra/src/watch.rs`, eliminando a descrição histórica multi-owner. W1
especifica `WatchSnapshot` opaco, captura única, espera que consome o snapshot
sem recaptura e compatibilidade de `wait_for_change`.

W2 fixa a ordem `compile -> normalize -> snapshot -> publish/discard -> evict
-> wait_since`, inclusive no caminho de erro de compilação. W3 fixa limpeza de
resíduo antes da fixture, diagnóstico por fase, liveness do child, mudança
única do asset e preservação de irrelevante/erro/último artefato/recuperação,
sem sleep de prontidão nem aumento de timeout.

B1 foi congelado como política evidencial, sem alteração de L0 ou código: hash
de binário identifica o artefato produzido no HEAD registrado e não é igualdade
semântica cruzada entre commits. `02_shell/build.rs` e
`00_nucleo/prompts/shell/cli.md` permaneceram byte a byte.

## Diff publicado

`git diff HEAD --stat` sobre os seis ficheiros rastreados P1:

```text
 00_nucleo/prompts/shell/watch.md      | 142 +++++++++++++++++++---------------
 00_nucleo/prompts/wiring.md           |  70 ++++++++++++++++-
 00_nucleo/prompts/wiring/tests/cli.md |  75 ++++++++++++++++++-
 03_infra/src/watch.rs                 |   2 +-
 04_wiring/src/main.rs                 |   2 +-
 04_wiring/tests/cli.rs                |   2 +-
 6 files changed, 227 insertions(+), 66 deletions(-)
```

- SHA-256 do texto do `git diff` dos três L0s:
  `140ea4c9a559b6539ac7619ad13cc112cf69827565f2f113b1e0a8b7855c2c52`
- SHA-256 do texto do `git diff` dos três consumers:
  `24b910b7d4694e7c865322f062216431272243caf5816ab44d783b3cee576950`
- SHA-256 do `git diff HEAD --stat` pós-resselo:
  `38df2cffc0baa5bc811debe4b5e73749c5a83c05dc4482d1341c928b8d4ce8cb`

Nos consumers, `git diff --unified=0` mostrou exatamente uma substituição de
linha por ficheiro, todas em `@prompt-hash`; os corpos Rust têm zero alteração.

## Gates executados

1. `crystalline-lint --fix-hashes --dry-run .` antes do resselo listou
   exatamente os três consumers autorizados.
2. `crystalline-lint --fix-hashes .` atualizou exatamente esses três headers e
   terminou com `0 drift warnings remaining`.
3. `crystalline-lint --checks v5,v15,v26 --fail-on warning .` terminou com
   exit 0 e `No violations found`.
4. `crystalline-lint --fix-hashes --dry-run .` pós-resselo terminou com exit 0
   e `Nothing to fix`.
5. O primeiro `git diff --check` detectou três espaços finais usados como
   quebra Markdown em `shell/watch.md`; eles foram removidos, o dry-run listou
   somente `03_infra/src/watch.rs`, e o resselo final atualizou apenas seu
   `@prompt-hash` de `1f245e2e` para `0420b249`.

Testes e builds não foram executados nesta fase porque o Passo 1295 proíbe
criar ou exercer oráculos antes do gate humano.

## Preservação P1294 e build congelado

Os hashes pós-resselo continuam exatamente:

| Artefato | SHA-256 |
|---|---|
| `00_nucleo/materialization/typst-passo-1294.md` | `10d6393ad9615f0ed38bc7ad8192d167a5bf91388e52cf38a1acb4271b1cc2e4` |
| `00_nucleo/diagnosticos/p1294-sanitization-manifest.json` | `0d3f24c9ac3f5691d4f07c1658440b58134bb9e94fd5d7e47f42ff84bedf7c41` |
| `00_nucleo/diagnosticos/p1294-terminal-seal.json` | `352ad6184b4c25c3ec5136ff56cca7ec8600885f69f89d248506fa698d4546a0` |
| `00_nucleo/diagnosticos/p1294-verification-receipt.json` | `0d31aa9b1e31440552c1d52f571923b40d68f7c65027bd093417c906c04535d1` |
| `00_nucleo/diagnosticos/p1294-blocked-report.md` | `b87dfe26f22bf46dd20e80a568a34b8f9da26138616932b3a84e6e522a8f959b` |
| `02_shell/build.rs` | `8fa42b2aecdaa6d8cbd31dfcfe901cef27474ab14cc42c0c05fe27d20fe175b5` |
| `00_nucleo/prompts/shell/cli.md` | `a2e050633516ae486a6a00968ee8e20a28990c9006582a4b82a820a0b254f30f` |

`p1294-certificate.json` e `p1294-final-report.md` não existem. O recibo
vermelho P1294 não foi absolvido nem substituído.

## Gate humano requerido

A confirmação deve abranger explicitamente:

- W1: API pública aditiva `WatchSnapshot`/`snapshot`/
  `wait_for_change_since` e compatibilidade de `wait_for_change`;
- W2: snapshot capturado antes de publicar ou descartar staging;
- W3: observador CLI sem sleep de prontidão, com limpeza e liveness;
- B1: proveniência de build vinculada ao HEAD, sem igualdade semântica de hash
  bruto entre commits.

Até essa confirmação, não estão autorizados P2–P6, testes, oráculos, mutantes,
selo, implementação, superfícies, certificado ou relatório final.

## Confirmação humana recebida

- resposta do dono: `Continue`
- recebida em: `2026-09-02T22:57:37-03:00`
- interpretação registrada: confirmação explícita para prosseguir com o
  conjunto W1/W2/W3/B1 apresentado imediatamente antes da resposta;
- efeito: P2 e fases causalmente posteriores ficam autorizados, cada qual
  somente após seus predecessores e dentro da allowlist segregada.

O recibo deixa de ser pendência operacional, mas permanece o registro do gate;
não autoriza saltar RED, discriminação, selo ou segregação de capacidades.

```text
P1295_L0_CONFIRMED_READY_FOR_P2
```
