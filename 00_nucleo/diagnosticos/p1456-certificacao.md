# Certificação Tekt — P1456

**Veredito:** `VERIFIED` no escopo do P1456  
**Regime:** executado sem atestação de isolamento  
**Instante:** 2026-09-18T22:14:07-03:00  
**Commit de produto:** `cabaa9c668fedd1d5f1bf4e9fedf39b07addf038`  
**Árvore de produto:** `4b98b3c74a465fcc7545b6bfa8d42ee67b43ffdd`

## Identidade verificada

| Artefacto | SHA-256 |
|---|---|
| `00_nucleo/prompts/compiler/layout/bib_csl.md` | `b17c8ef1911dbf6a7d71d1833369970094c9dd02308b4c1576b76e86a1fdfbfe` |
| `01_core/src/compiler/layout/bib_csl.rs` | `2e14ab8de2a4c617ec68c55578979e48b0fdeb7742a5db3f6ca616649a3c8c74` |
| `00_nucleo/materialization/typst-passo-1456.md` | `e323027e2dd37baa4567b6287dc9767f2e7a9554bb946cd45be59d14fa20eed2` |
| `p1456-manifest.json` | `5059ee1cdacff722aefaf3d37beb0db5a1ad65a4db2aae4c64f64019d4ced22d` |
| `p1456-contract.json` | `89ad920761d7208b406dbf65af961c0d10181cf876570f25bb05a6990ca29728` |
| `p1456-oracles.json` | `06fe51277d30f15a4578e5baec5cbd494c167f5d8f4ef57a31ea967efaf40397` |
| `p1456-mutants.json` | `c6aa7c45ee28ba64aa081530ee1d0cc5a1c6499b9b34406ee63d09d746357e97` |
| `p1456-preseal.json` | `7d490de009aa8a5fbbcc063a6d809ce0e20f481d660f4551fe3324d5f80978d4` |
| `p1456-mutation-results.json` | `adc51ff21fddbb1838a260fdbb1089ef468050821b0d394b70e3d2faf0bb9ae7` |
| `p1456-mutation-runner.py` | `5ddbc94ba8d0c31ba06bc75a6cc02004b397934aed86f94edad5c197e2071a12` |

O preseal foi capturado antes do RED e do candidato. O único movimento
posterior do L0 foi o resselo mecânico de `Hash do Código` exigido pelo
`crystalline-lint`; a obrigação semântica P1456 permaneceu inalterada.

## Evidência executada

- RED focal observado: `45---67` chegava ao hayagriva como `45--67`.
- GREEN focal: `cargo test -q -p typst-core p1456_ -- --nocapture` — 2/2.
- Owner: `cargo test -q -p typst-core compiler::layout::bib_csl::tests -- --nocapture`
  — 34/34.
- Ataques: `python3 00_nucleo/diagnosticos/p1456-mutation-runner.py`
  — 12/12 mutantes mortos, score `1.0`, zero sobreviventes.
- Bilateral público IEEE com vanilla ratificado e binário cristalino release:
  `45---67` produziu `pp. 45–67` nos dois; casos canônico e prefixado
  permaneceram em paridade.
- Ausência do hardcode removido: a busca
  `rg -n 'replace\("--",\s*"-"\)' 01_core/src/compiler/layout/bib_csl.rs`
  não encontrou ocorrências.
- Gates: `cargo build`, `cargo fmt --all -- --check`,
  `crystalline-lint .` e `git diff --check` concluíram com código zero.

## Limite global conhecido

`cargo test --workspace` terminou com 5913 testes aprovados e 8 falhas
preexistentes e alheias ao consumer P1456: P1292 `math_vec`; quatro testes
P981/P1132R de delimitadores `math lr`; `cases_nao_panica_com_align_point`;
e as contagens HTML P1423/P1168. Nenhuma falha referencia `bib_csl`, page
range, os dois testes P1456 ou os artefactos da campanha.

O caso de ranges múltiplos evidencia uma diferença anterior: o vanilla
ratificado extrai `pp. 1–37–9`, enquanto o cristalino extrai
`pp. 1–3, 7–9`. O P1456 não a criou nem a usa para alegar paridade ampla.

## Conclusão

O candidato satisfaz a obrigação congelada: reduz cada run ASCII maximal de
dois ou mais hífens a um separador, preserva hífens isolados e demais
caracteres, limita a transformação a `pages` e elimina o `str::replace`
produtivo. O veredito não certifica a suíte global nem independência de
papéis; certifica somente o produto identificado acima no escopo P1456.
