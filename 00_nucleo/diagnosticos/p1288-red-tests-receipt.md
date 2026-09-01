# P1288 — receipt dos testes A/B pós-gate (RED)

## Escopo e segregação

- Papel: testador A/B pós-gate, depois da confirmação humana do gate.
- Protocolo aplicado: `tekt-materializacao-segregada`; expectativas congeladas antes de executar a candidata.
- O papel leu somente as entradas expressamente autorizadas para o contrato, os oráculos e a regressão P1287. Não leu código ou diff produtivo L1–L4, não implementou a funcionalidade, não alterou harness, oráculos, contrato, L0 ou fixtures e não emite veredito funcional.
- A única candidata observada foi o executável black-box `target/release/typst`.
- Não há atestado de isolamento ambiental forte. A segregação desta fase repousa em ordem, capacidades, allowlist de leitura e hashes.

## Estado medido

- Instante do replay RED: `2026-08-31T12:12:23-03:00`.
- HEAD: `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.
- Working tree: `não commitado e partilhado`.
- Candidata executada: `target/release/typst`, SHA-256 `09e8e40605da622e2421f471950768a77b24568b284803a628e99f200defd8a8`.
- Suite congelada antes da execução: `lab/parity/matrix/test_p1288_candidate_contract.py`, SHA-256 `645073eb286c8530593f2dff3e30a0827f1445642c2d14f4f6b565b623dab51e`.

## Entradas P1288 pinadas

| Entrada | SHA-256 |
|---|---|
| `00_nucleo/materialization/typst-passo-1288.md` | `3858860e2d3e9916b051dfbbea0ff66009382bb5daa3fcec8025e9a2492a3e28` |
| `00_nucleo/diagnosticos/p1288-contract-seal.json` | `22042fef8243a7bf9ce0f8e4ce1b5300dbc0bd1a13da97e97059bf59983b397e` |
| `00_nucleo/diagnosticos/p1288-manifest.json` | `0e7282b6b5b445d0532bb5456abfbd765e7878fc44d318f96a0b8b960351ed3a` |
| `00_nucleo/diagnosticos/p1288-contract-receipt.md` | `69d8ea63d2eb6f0267e135cdcd37a479c1b0a0b72814a1170a01be28cf48d34a` |
| `00_nucleo/diagnosticos/p1288-pre-gate-l0-receipt.md` | `b259334c262683a2e62979b67cbf3b09f57f65ac9e07c4e5c504b6dda5436c6b` |
| `lab/parity/matrix/p1288_oracles.py` | `98bb462601c255708a646fdc7dc27e2f59455ade5198b2d26105299934723e48` |
| `lab/parity/matrix/test_p1288_oracles.py` | `59f3c8ea71df9fee0221c4daf50c5fb53c4fde6dd0992eb1fd0aa3e849c77f99` |
| `lab/parity/matrix/p1288-oracle-baseline.json` | `ac0d97379820eb09f89c69e52858b231794bb4db6455d16527be2d04f1fd7a3d` |
| `00_nucleo/diagnosticos/p1288-oracle-receipt.md` | `894bf70aef8b0cce128f1c3a050a7ed8a2cb895d48feafbc2f97a3c22f108ff4` |

O selo pinado acima contém também os hashes individuais das 12 fixtures vanilla usadas pela suite. Nenhuma fixture foi alterada por este papel.

## Comando e RED observado

```text
sha256sum target/release/typst lab/parity/matrix/test_p1288_candidate_contract.py
python3 -m unittest lab/parity/matrix/test_p1288_candidate_contract.py
```

Resultado real:

```text
.FFFFFF..F
Ran 10 tests in 1.790s
FAILED (failures=7)
```

As expectativas não foram modificadas depois da observação da candidata.

| Teste | Resultado | Obrigação observada |
|---|---:|---|
| `test_01_sealed_inputs_and_frozen_oracle_match` | PASS | selo, baseline e fixtures permanecem pinados |
| `test_02_default_features_and_active_trio_are_independent` | FAIL | feature, trio ativo, defaults e independência |
| `test_03_casts_defaults_and_public_diagnostics_match` | FAIL | casts, defaults e diagnósticos públicos |
| `test_04_summary_scope_level_and_data_survive_to_pdf` | FAIL | `summary`/`scope`/`level`/`data` até PDF |
| `test_05_tags_disabled_remove_structure_without_visual_change` | FAIL | remoção estrutural com tags desativadas sem mudança visual |
| `test_06_multipage_ids_parent_tree_and_logical_header_survive` | FAIL | multipágina, IDs, parent tree e header lógico |
| `test_07_pdf_metadata_does_not_leak_into_other_targets` | FAIL | isolamento de HTML/SVG/PNG |
| `test_08_forward_and_reverse_execution_are_identical` | PASS | payload determinístico forward/reverse |
| `test_09_html_profile_surface_remains_preserved` | PASS | superfície HTML focal congelada de P1288 |
| `test_10_registration_only_stub_is_rejected` | FAIL | 43 de 46 testemunhos decisivos ficaram diferentes de `Preserved` |

Detalhes discriminatórios observados incluem `compile exit 2` no caso multipágina e falha de compilação nos pares HTML/SVG/PNG. Portanto, o simples registo das funções não satisfaz o contrato congelado.

## Gate histórico P1287 (72/77)

As entradas adicionais autorizadas foram apenas inspecionadas como artefatos black-box:

| Entrada | SHA-256 |
|---|---|
| `00_nucleo/diagnosticos/typst-passo-1287-relatorio.md` | `7ca2da4a24bcc04e588e28090010062d68c7b32290dda8a02da966da647e9d76` |
| `00_nucleo/diagnosticos/p1287-verification-receipt.md` | `8a970f4bf25aa0f47317ab85a9270f2268bc65e74cd533ee8e025659dc805b83` |
| `lab/parity/matrix/p1287_candidate_adapter.py` | `152fe7ae7f65dcd015a24951e33cd682bda8590609dd20ef3fcc7ad2c8dc7679` |
| `lab/parity/matrix/p1287-oracle-baseline.json` | `ff09c8146b73684b2e746c631a6103817b6df7070aa8248e2d88dbca34749488` |
| `lab/parity/matrix/p1287_global.py` | `745a4c470704de54f9a550d220a3092bfa8452e3b02cb7010846758eab36bd37` |
| `lab/surface-inventory/run_probes.py` | `3bd082751fbd05c89f24a312353d81f74f2882902db5a203180e5ee5fb2c10cf` |
| `lab/surface-inventory/probes.json` | `1e7534a0cc8ebcfa1f5ed6652918711b738d7471d94a883a500becf297357b78` |
| `lab/surface-inventory/summarize.py` | `bc6f3af1eb2972e52a44c88cd66b3905e86e5f1e7a28ac5ab3fe9b481c14d5aa` |
| `00_nucleo/diagnosticos/p1282-probes-default.json` | `ff66dfe25da4a4780171f5e05b5d640a88d96c3360079b5006d727864a46d728` |
| `00_nucleo/diagnosticos/p1282-probes-html.json` | `de33ef5b1c724d022e569471b48caebe5a65752933d19490151a748121615f44` |

O relatório P1287 fixa os resultados históricos de 72 `MATCH` no perfil default e 77 `MATCH` no perfil HTML. Porém, os snapshots finais de inventário que selecionam exatamente os corpus de 111 e 115 probes não constam da allowlist desta fase: o `probes.json` autorizado contém somente o inventário atual de 31 probes, e os outputs P1282 autorizados correspondem a corpus diferentes. O comando `p1287_global.py` cobre 23 casos C/E e não reproduz 72/77.

Consequentemente, 72/77 é aqui registado como **gate externo invocado e não reproduzível com as entradas autorizadas**, sem inventar um teste substituto e sem transformar um proxy em prova. O teste focal HTML P1288 foi executado e passou, mas não é alegado como reprodução dos 77 casos históricos.

## Estado desta fase

- Estado: `RED_OBSERVED`.
- Contagem: `10` testes, `7` falhas, `3` passes.
- Evidência suficiente para rejeitar a candidata atual segundo o contrato congelado; isto é um resultado de teste, não um veredito funcional final nem aprovação humana.
- Artefatos criados exclusivamente por este papel: `lab/parity/matrix/test_p1288_candidate_contract.py` e este receipt.
