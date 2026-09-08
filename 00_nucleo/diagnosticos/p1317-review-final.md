# P1317 — veredito independente final

**PASS no recorte P1317.** Revisão executada sem atestação técnica de isolamento.
Nenhum achado impeditivo na implementação nem na evidência inspecionada.
Revisor `/root/p1317_review` escreveu somente seus artefatos `p1317-review-*`;
não alterou fonte, L0, testes, oráculos, fixtures ou recibos julgados.

Em `loading.rs:956–958`, ErrorKind::Utf8 agora produz exatamente
`failed to parse CSV (file is not valid UTF-8)`. P1317 em `loading.md:243`
é a obrigação anterior ao código. O candidato é literalmente o estado RED
mais esse braço e o resselo: retirar ambos recupera o hash completo do RED,
incluindo testes. Não muda precedência, API, parsing, valores ou origem P1316.
Fonte anterior e todos os testes P1315/P1316 continuam preservados.

## Proveniência e cadeia

HEAD `bc8213f36b7a29b4fdc30cfc74ddc23586117c64`, working tree não commitado.
Auditoria final em 2026-09-08T15:14:51.447Z:
`p1317-review-final-audit.json`, produzida pelo comando somente leitura
`node 00_nucleo/diagnosticos/p1317-review-audit.cjs` no root.
O recibo inclui diff/stat exato e hashes do script, freeze, código, L0 e gates.
Script SHA-256 `306a5a17c93cf9f0c6425d64ef27cd5cb6205608dd297732dfa8e92c13a7e6e8`.

Source SHA-256 `39cf6ef6c3cd55872dfc40cbe0787faab7379013beb3e399183d8cc97d22d86c`;
L0 raw SHA-256 `99f8f50dc524eaa0e6742cfc6a3bb1db160895cd27ed04635721871cd5d6d7c2`.
L0 normativo preserva o hash congelado
`059aad946b30515b24ecfdef4c8490ab735d1bb86d374ce1371c899130ab0d51`.
Candidato `/tmp/p1317-target.y5u9ah/release/typst`, SHA-256
`9fcb4cbe830c74ec589506b0b86f52982abfdaab558f62cbf7a8f7de8665ccab`.
Baseline P1316 e vanilla upstream `a51e02804` mantêm os hashes do preflight.
Foram reconferidos 24 inputs congelados e 59 artefatos P1315/P1316, sem drift.

O RED local de três falhas de mensagem e um controle que passa precedeu
o freeze A/B de 15:02:45Z; a revisão pré-patch precedeu a implementação.
GREEN local: quatro testes passam. O teste Path/Str via World observa o novo
texto e detached preservado; Bytes com World proibido prova ausência de I/O.

## Gates conferidos

- A/B: 387 casos (330 replays históricos), quatro perfis, 1548 expectativas;
  208 eram RED. Normal/repeat/reverse produziram 4644 comparações, todas
  iguais às expectativas congeladas; zero falhas/Unknown e nenhuma observação
  ausente ou duplicada. Conferência própria dos dados brutos, além do recibo
  `p1317-ab-comparison.json` do testador.
- `cargo build --workspace --release`: exit 0.
- `cargo test --workspace --release --no-fail-fast`: 6656 passam, zero falhas,
  três ignorados; execução de 15:08:47.836130 a 15:13:52.306525 UTC, mesmo código.
- `crystalline-lint .`: exit 0, zero errors, 240 warnings e 1137 infos.
  Não confundir ausência de erros bloqueantes com ausência de avisos.
- `cargo fmt --all --check`, `git diff --check` e lineage dry-run: exit 0;
  resselo canônico informa `Nothing to fix`.

As contagens acima provêm dos recibos identificados no audit final, não de
execuções sem proveniência. O script independente verificou a preservação
literal do baseline exceto a primeira linha dos casos Utf8 declarados,
inclusive todos os replays antes da transformação.

## Limites do veredito

Não é paridade geral CSV, certificado de refinamento, mutation score ou
isolamento atestado. Sufixos `at l:c`, localização/path externos e origem
Path/Str permanecem dívida. A colisão parsing+excesso conserva a precedência
legada: sua causa UTF-8 muda, embora vanilla rejeite excesso antes. A limitação
CLI de decoder puro/World/Args sintético é coberta pelos testes locais e pela
inspeção do delta, sem transformar esses testes em oráculos independentes.
Os avisos arquiteturais existentes não foram reparados neste lote.
