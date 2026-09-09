# P1336 — gate RED anterior à implementação

Veredicto: RED funcional confirmado; autorizada passagem a C no escopo L0 congelado. Não é aprovação de GREEN ou fechamento.

Manifesto SHA-256 `54864bfd67d50197560011d5aae055a0303fcc9cadc4230221dfd4f7a6daea88`; recibo `p1336-unit-red.json` SHA `9db024816f9cf70ec7b1baf867f9f4467501c40fda5d7b3d917fbee583f4f92c`. Execução `cargo test -p typst-core --release --locked --lib p1336_tests`, target `/tmp/p1336-target.QiOMGq`, de `2026-09-09T18:13:28.274145+00:00` até `2026-09-09T18:15:30.262354+00:00`, HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, árvore não commitada com before/after e diff/stat integrais no recibo.

A compilação terminou e o processo de testes executou cinco testes. Três falharam por assertions dos diagnósticos Int/Str; dois controles de preservação passaram. Exit 101 é falha funcional real, não compilação, timeout ou opacidade. Os testes de AST ainda interrompem na primeira divergência de mensagem, portanto este RED local não demonstra isoladamente a detecção do span; a medição bilateral já registra a divergência de origem e os mutantes de span deverão fornecer discriminação isolada após GREEN.

Fonte antes/depois da execução possui SHA `9a0ed12852814ad978e021b68fdd4f4990ad634cde5eff42cf4807fb990ef774`. Em `2026-09-09T18:14:00.058Z`, revisão determinística removeu somente o módulo `p1336_tests` e confirmou igualdade textual exata com `manifest.pre_candidate_source`. O módulo independente conserva SHA `b30cc9449bef081d51d44d664aed04c4c26c48bcde15ecf4b31938a1ecfd9934`. A causalidade é intenção → testes congelados → RED → candidato, sem patch produtivo antecipado.

O operador recebeu somente a classificação do gate, sem detalhes privados de assertions. Permanecem obrigatórios GREEN, canais completos nos perfis/ordens, exclusões e dívidas, discriminação dos mutantes, gates workspace/linhagem e preservação final.
