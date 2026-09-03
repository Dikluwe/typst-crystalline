# P1295 — recibo de implementação P5

## Veredito e regime

- Papel: P5, implementador segregado.
- Regime: protocolo completo de materialização segregada Tekt, fase exclusiva de
  implementação após contrato selado.
- Veredito local: implementação candidata produzida e compilada com sucesso dentro
  da allowlist; o veredito final pertence ao verificador independente.
- Atestação proporcional: segregação causal e por capacidades no workspace
  compartilhado; não se alega isolamento técnico do filesystem.

## Predecessor causal e entradas congeladas

Predecessor Git recebido: `76fb7336311bdb6497456ab5fdc0a8ce355ff39b`.
Os quatro hashes foram confirmados antes da primeira edição e novamente depois da
compilação:

| Entrada | SHA-256 confirmado |
|---|---|
| `00_nucleo/prompts/shell/watch.md` (W1) | `f39a63e6a1e2025d3539b61b34b46ffa3a3b0ea2aa64711ed7699abf3e3f7332` |
| `00_nucleo/prompts/wiring.md` (W2) | `81db17f53cdb344cfbee0a5aee230ae1e480cab1856618f846a5a43a9551e0b5` |
| `00_nucleo/prompts/wiring/tests/cli.md` (W3, somente hash/fronteira) | `5800231392fa0e8a5311a6a5d2664f0f06f85c80b273ffbe80c8fb5e04ed5d1c` |
| `00_nucleo/diagnosticos/p1295-contract-seal.json` | `cdd0f5ddc9740808fdee4c0c24d9d05d50c3243bab124678f9d6fba83b86d021` |

W1 e W2 foram as únicas especificações produtivas usadas. W3 foi tratado somente
como fronteira e verificado por hash; o seu consumer de teste não foi lido.

## Capacidade concedida e exercida

- Leitura causal: W1, W2, selo, corpos produtivos autorizados e instruções do
  protocolo. Em `03_infra/src/watch.rs`, a inspeção inicial parou antes de
  `#[cfg(test)]`; a conferência final de números de linha alcançou somente o
  marcador e o boilerplate de abertura do módulo, sem ler corpos de teste.
- Escrita no repositório: exclusivamente `03_infra/src/watch.rs`,
  `04_wiring/src/main.rs` e este recibo.
- Não foram lidos nem executados `03_infra/tests/p1295_watch_contract.rs`,
  `04_wiring/tests/cli.rs`, `p1295-red-tests-receipt.json`,
  `p1295-adversarial-plan.md` ou `p1295-discrimination-receipt.json`.
- Não foram lidas ou listadas as pastas `00_nucleo/materialization/` e
  `00_nucleo/context/`.
- Não foram editados L0, headers `@prompt-hash`, testes, manifesto, selo, build
  script, parsing CLI, P1293, P1294 ou qualquer outro ficheiro.
- Nenhum subagente foi usado e nenhum output privado foi recebido.
- `cargo check` escreveu somente artefactos de build fora das fontes versionadas,
  como autorizado pelo gate de compilação.

## Estado antes da implementação

Medição em `2026-09-02T23:59:10.792587633-03:00`:

- HEAD: `76fb7336311bdb6497456ab5fdc0a8ce355ff39b`.
- `03_infra/src/watch.rs`: SHA-256
  `2af4d792a162a306c8bc2f3065c475eaf566742d91c4279f871ec114091c0744`.
- `04_wiring/src/main.rs`: SHA-256
  `32d97b12fde4aaa163412542881f110e4fc12007fecd7974c93f7a41c0b68d06`.
- Nos dois ficheiros produtivos, o diff contra HEAD continha somente os headers
  `@prompt-hash` já ressellados por P1; esses headers foram preservados.

`git diff HEAD --stat` antes da implementação:

```text
 00_nucleo/prompts/shell/watch.md      | 142 +++++++++++++++++++---------------
 00_nucleo/prompts/wiring.md           |  70 ++++++++++++++++-
 00_nucleo/prompts/wiring/tests/cli.md |  75 +++++++++++++++++-
 03_infra/src/watch.rs                 |   2 +-
 04_wiring/src/main.rs                 |   2 +-
 04_wiring/tests/cli.rs                |  63 +++++++++++----
 6 files changed, 276 insertions(+), 78 deletions(-)
```

## Materialização

- L3: adicionou `WatchSnapshot` público com estado privado, `snapshot` com uma
  captura por path e `wait_for_change_since` que consome esse baseline sem o
  substituir. A API legada delega a essas duas operações.
- L4: normaliza dependências e captura o snapshot imediatamente após
  `run_compile_observed`, antes de `commit_output` ou `discard_output`; o mesmo
  valor é consumido depois de `crystalline_evict(10)`.
- Criação, remoção e conteúdo de mesmo tamanho continuam representados pela
  comparação `Option<Fingerprint>` já vigente. Rename fatal, staging, último
  artefacto válido, fallback exato e inventário transitivo foram preservados.

## Hashes depois da implementação

Medição em `2026-09-03T00:00:44.000389662-03:00`:

| Artefacto produtivo | SHA-256 depois |
|---|---|
| `03_infra/src/watch.rs` | `d543b1c32844b6f63085635ae34fa2beb01ba989c4ebed7f789bb5ae77e9ee41` |
| `04_wiring/src/main.rs` | `e119a46479e1d2e7b3f0052df2b57ab5d31be573969bbe841dd8ee2d0e3e9c48` |

`git diff HEAD --stat` depois da implementação e antes da criação deste recibo
(ficheiros untracked não aparecem nesse comando):

```text
 00_nucleo/prompts/shell/watch.md      | 142 +++++++++++++++++++---------------
 00_nucleo/prompts/wiring.md           |  70 ++++++++++++++++-
 00_nucleo/prompts/wiring/tests/cli.md |  75 +++++++++++++++++-
 03_infra/src/watch.rs                 |  33 ++++++--
 04_wiring/src/main.rs                 |  18 +++--
 04_wiring/tests/cli.rs                |  63 +++++++++++----
 6 files changed, 309 insertions(+), 92 deletions(-)
```

## Comandos e gates do implementador

| Comando | Exit | Observação |
|---|---:|---|
| `sha256sum` das quatro entradas congeladas | 0 | hashes iniciais coincidentes |
| `git diff --check -- 03_infra/src/watch.rs 04_wiring/src/main.rs` | 0 | primeira checagem |
| `rustfmt --edition 2021 --check 03_infra/src/watch.rs 04_wiring/src/main.rs` | 1 | detectou somente a quebra de linha de uma expressão em W1; corrigida por `apply_patch` |
| `cargo check -p typst-infra -p typst-wiring --bins` | 0 | terminou em 18.07 s; warnings preexistentes fora da allowlist |
| `rustfmt --edition 2021 --check 03_infra/src/watch.rs 04_wiring/src/main.rs` | 0 | formato final conforme |
| `git diff --check -- 03_infra/src/watch.rs 04_wiring/src/main.rs` | 0 | diff final limpo |
| `sha256sum` das quatro entradas e dos dois outputs produtivos | 0 | entradas inalteradas; hashes after registados |

Nenhum `cargo test`, teste P1295 protegido, teste global,
`crystalline-lint --fix-hashes` ou outro gate capaz de consumir os oráculos
privados foi executado. A confirmação funcional, arquitetural e adversarial
fica deliberadamente reservada ao verificador independente.
