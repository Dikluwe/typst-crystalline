# Certificação Tekt — P1458

**Veredito:** `VERIFIED` no escopo do P1458  
**Regime:** executado sem atestação de isolamento  
**Instante:** 2026-09-18T22:43:02-03:00  
**Commit de produto:** `e95b6a08dd4b51dcfa9864b9f970f1d23dddbb4b`  
**Árvore de produto:** `f2fa8fb66f2cfa5014cac8c22d734cb62d589263`

## Identidade verificada

| Artefacto | SHA-256 |
|---|---|
| `00_nucleo/prompts/compiler/layout_references.md` | `4d51cc94a6d520c87fec7094b8fb5c9b4cc0c6980f472e6ac41a9d50e6724ba1` |
| `01_core/src/compiler/layout/references.rs` | `bd7b1219657b6a01200c3b8ad3902ddc6a29237de198eef91eede2f8590f71c0` |
| `00_nucleo/materialization/typst-passo-1458.md` | `a1882df7815d2f863c6a5893a07a7559719aba89bf3b2d58965448be3f2e1bd8` |
| `lab/parity/p1458_figure_ref_de.typ` | `8890c287edf359d5a6a62fa658ed5a2138b5e7df4fdd53ba6cc1bf5144020825` |
| `p1458-manifest.json` | `69e21242343b89398104bbd447a536f0fd014832910db6ebbdf17a2ec97f0b73` |
| `p1458-contract.json` | `afc9f5918170e2c488ce4c9729889e7e665c3a9c9d2868682573fc10f20d86e1` |
| `p1458-oracles.json` | `ed3c30cf487c5b38bd07d41b04bf5f6e86d174fe717a2cca9649eca903f6760e` |
| `p1458-mutants.json` | `fa1c2c730c1a6818b756a378b9390126ce16d5c1112f9826fdb931869819a41f` |
| `p1458-preseal.json` | `2158eb4fb661922b1dd387f663b6cb33996ebd3cee3b2e77f4ebe8fd6c14fd1a` |
| `p1458-mutation-results.json` | `150d32d9c817a09d95b9821edf67eda6a0b3b5079976f7bbe9f8f62b4e665543` |
| `p1458-mutation-runner.py` | `d88c87c49b96ffeea16830fe0e01e67c01216d0b4d2904dc82c73a4490ca36a8` |

O preseal preserva contrato, oráculos, mutantes, L0 semântico e o teste RED
antes do candidato. O hash do runner final difere do congelado porque a
primeira campanha revelou que `delegate-image-only` também delegava table/raw;
o ataque foi corrigido para corresponder ao mutante declarado. Não se atribui
ao runner corrigido independência temporal nem de autoria.

## Evidência executada

- RED focal: `image/de` obteve `Figure`, esperado `Abbildung`.
- GREEN focal: 1/1.
- Owner `figure_supplement`: 8/8; regressões P1073: 4/4.
- Campanha pública alemã nas ordens forward, reverse e forward-repeat:
  vanilla ratificado e cristalino produziram `Abbildung 1` na legenda e na
  referência; nenhuma extração conservou `Figure 1`.
- Ataques finais: 10/10 mortos, score `1.0`, zero sobreviventes e nenhuma
  promoção de Unknown.
- Auditoria de fonte: os caminhos numérico e legacy de figure chamam
  `figure_supplement_for_lang`; o consumer não mantém match de kind nem
  literais linguísticos para esses caminhos.
- `cargo build --release --bin typst`, `cargo build`, formatação,
  `crystalline-lint .` e `git diff --check`: sucesso; zero violations.

## Limite global conhecido

`cargo test --workspace` concluiu com 5916/5924 testes aprovados. As oito
falhas são as históricas já presentes antes do recorte: P1292 `math_vec`;
quatro P981/P1132R de `math lr`; `cases` com align point; e duas contagens HTML
P1423/P1168. Nenhuma referencia P1458, supplements de figure ou referências.

## Conclusão

O produto identificado satisfaz o contrato: referências de figure deixaram de
possuir uma segunda decisão bilingue e agora reutilizam o owner linguístico em
ambos os caminhos. O veredito certifica somente este fragmento e não alega
equivalência geral nem isolamento de papéis.
