# P1336 — resultado A/B independente

Autor `/root/p1336_tests`. Executado sem atestação técnica de isolamento. Nenhuma
implementação candidata foi lida; runner, fixtures e expectativas congelados foram
executados contra o binário fornecido pelo operador. Este recibo relata o fragmento
testado e não aprova arquitetura ou equivalência funcional geral.

Manifesto SHA-256:
`54864bfd67d50197560011d5aae055a0303fcc9cadc4230221dfd4f7a6daea88`.
Freeze SHA-256:
`ba3f1a49fb0f96dc0770336274f230f4c2dde5ccf390c770c03a8b3244098cfa`.

## Identidade e reprodução

Comando executado no cwd `/repos/Antigravity/typst-crystalline`:

```text
python3 00_nucleo/diagnosticos/p1336-tests-runner.py candidate --binary /tmp/p1336-target.QiOMGq/release/typst
```

Exit do runner: 0. Candidato release SHA-256:
`646a8d97400c0abe262a65c9b9559b47a3ecf7d10eafd82fa79a64ade0504497`.
Antecedente cristalino SHA-256:
`11e3164fa509030cc78dc048d5bb4f2426348e32a24edc7cd2f320c704e6ef61`.
Vanilla ratificado upstream/main `a51e02804`, SHA-256:
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

Medição candidata de `2026-09-09T18:20:13.266956+00:00` a
`2026-09-09T18:20:33.092501+00:00`, HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitada.
Before/after e lista exata de arquivos alterados com diff/stat estão no JSON
de execução. Cada processo conserva argv, UTC, cwd, identidade de executável,
fonte literal e SHA-256, exit e ambos os canais integrais em texto/base64.

## Resultado

Os 45 casos × quatro perfis × três ordens produziram 540 observações candidatas
válidas, zero Unknown obrigatório, zero instabilidade por ordem e zero violações.
Comparação exata de exit/stdout/stderr integral, incluindo todos os diagnósticos,
hints, traces e sublinhados emitidos pela CLI:

| Classe congelada | Comparações | Resultado |
|---|---:|---|
| CONVERGENCE_REQUIRED | 156 | Candidato coincide integralmente com vanilla |
| PRESERVE_PARITY | 264 | Candidato preserva baseline já coincidente |
| PRESERVE_EXISTING_DEBT | 120 | Candidato preserva dívida identificada antes de C |

Preservação das dívidas não recebe crédito de paridade. O desvio de Str.len
ligado sem chamada, namespaces int/str, Bool/Array/None/Auto, Content.text e
ordem panic permanece explicitamente aberto. A classificação, limites e a
revisão causal anterior ao freeze estão em p1336-tests-calibration.md.

O auditor congelado voltou a discriminar os seis controles de transporte:
observação válida, crash, bytes ausentes, opacidade deliberada, JSON inválido e
bytes inconsistentes. Unknown dos controles opacos testa o auditor; não conta
como observação produtiva bem-sucedida. Os cinco testes Rust locais e gates
complementares são executados pelo operador e julgados pelo revisor; este
recibo não inventa seus resultados.

## Artefatos íntegros

| Artefato em `00_nucleo/diagnosticos/` | SHA-256 |
|---|---|
| p1336-tests-baseline.json | eb814dcad2572dc79205b2489f5d81f82637233d6a11def5fdb8542e965ffc26 |
| p1336-tests-candidate.json | 9f692f1d097a17c94f9de1795cb7249f157f9d6589b1d7e5625f84ea87bf8706 |
| p1336-tests-comparison.json | dd51209af37f4385f6b8de2f0e2d09505db001833cb0ee483a61f4a48840b0cc |
| p1336-tests-expectations.json | 502f23e789dde6ab4c698a5e5212cf6892f41cb20548bdb504a6c00767385073 |
| p1336-tests-cases.json | 6b222d7eb188b00b224503eb30e26525cb6d57a3dc6f6e2aa3b05700a6c3f790 |
| p1336-tests-runner.py | 94c32049d8876f74199dbec6519012fb6d409fa2a045eabfa928e879340773bf |

O runner verificou os hashes protegidos e recusaria sobrescrita de resultados
existentes. Baseline, expectativas, catálogo, runner e freeze não foram editados
após congelamento. LocatedContent, text contextual e warnings conservam os
limites de cobertura reportados na calibração; não há alegação A/B adicional.
