# Certificação Tekt — P1457

**Veredito:** `VERIFIED` no escopo do P1457  
**Regime:** executado sem atestação de isolamento  
**Instante:** 2026-09-18T22:28:28-03:00  
**Commit de produto:** `80a99f8f7c037ac426ec5962961d800bd10b8454`  
**Árvore de produto:** `a11d6006bf9f27acbd4f96bb2764ba2c8f0a9251`

## Identidade verificada

| Artefacto | SHA-256 |
|---|---|
| `00_nucleo/prompts/compiler/layout/bib_csl.md` | `f938a45fb6557998d02106d98df2879f8a092783f86aa2da1be702a4d78a4a16` |
| `01_core/src/compiler/layout/bib_csl.rs` | `741325e68722499c1bc6b7722b276e2dc5aaaa20452dce8a7cbc77dab7df4590` |
| `crystalline.toml` | `930af768be0ac69fd5587dafd16d3a2de68b5e5a01376367254fd34bc6185496` |
| `00_nucleo/materialization/typst-passo-1457.md` | `0b694d46dd16fa5e6b1689f7ca28a839e6e00ec9a65c71619435861320be7b99` |
| `p1457-manifest.json` | `1674992de23dafd78bb9da01308efe9a4359f45b9a04271fe551a215ae0368eb` |
| `p1457-contract.json` | `5ad76b4d203134035b800cf61f2a80adcb86aed18822b9e87223163dd0081a06` |
| `p1457-oracles.json` | `07122999442f5995243daa4f1f9b59f2e7454e51168a779dde48f348b79b9533` |
| `p1457-mutants.json` | `8aedc0b0c1b439cfe50c1d80e3ce445dff7adeecc9b4fbd9969cd940b45cd528` |
| `p1457-preseal.json` | `9d19a42c30471f2fcc44f3542a953def46a3288bfc177cc3a45dc5dd81a62dc4` |
| `p1457-mutation-results.json` | `a6c3c6beef6f1fd88d112be1636d9d7fea7ed29381f3886f8c5acc1b819976df` |
| `p1457-mutation-runner.py` | `6e0cb5e466cfb27e95686ce1b333f1859529e6688935a2cf9cb9898ed5b37807` |

O preseal foi capturado antes do RED e do candidato. O movimento posterior do
L0 restringiu-se ao resselo mecânico de `Hash do Código`; a obrigação P1457
permaneceu inalterada.

## Evidência executada

- RED: `plain text` ficou sem aspas e `True` virou booleano YAML no baseline.
- GREEN focal: `cargo test -q -p typst-core p1457_ -- --nocapture` — 3/3.
- Owner: `cargo test -q -p typst-core compiler::layout::bib_csl::tests -- --nocapture`
  — 36/36.
- Ataques: `python3 00_nucleo/diagnosticos/p1457-mutation-runner.py`
  — 12/12 mutantes mortos, score `1.0`, zero sobreviventes.
- Campanha bilateral IEEE em forward, reverse e forward-repeat: vanilla
  ratificado e cristalino compilaram `title = {True}` e extraíram a mesma
  entrada `A. Alpha, “True,” Journal, 2024.` nas seis execuções.
- Auditoria de fonte: `needs_yaml_quotes` e o escaping manual por `s.replace`
  não existem mais no consumer.
- `serde_json::to_string` foi adicionado ao allowlist da crate já autorizada
  em L1; não houve crate, tipo público ou dependência nova.
- Gates: build release, `cargo build`, formatação, `crystalline-lint .` e
  `git diff --check` concluíram com código zero e zero violations.

## Limite global conhecido

`cargo test --workspace` terminou com 5915/5923 testes aprovados e as mesmas
oito falhas históricas fora de `bib_csl`: P1292 `math_vec`; quatro casos
P981/P1132R de delimitadores `math lr`; align point; e duas contagens HTML
P1423/P1168. Nenhuma falha referencia YAML bibliográfico, escalares, P1457 ou
os artefactos da campanha.

Os PDFs recompilados não foram tratados como evidência de igualdade binária:
os bytes variaram entre execuções, mas o texto público congelado foi estável.

## Conclusão

O produto identificado satisfaz o contrato: todo campo textual da ponte usa
uma representação string total, spellings tipados pelo YAML permanecem texto,
escapes e Unicode fazem round-trip, e o catálogo lexical hardcoded foi
eliminado sem sucessor. O veredito certifica somente o fragmento P1457 e não
alega equivalência geral nem isolamento de papéis.
