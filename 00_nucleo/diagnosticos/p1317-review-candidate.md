# P1317 — revisão do candidato

Auditoria independente em 2026-09-08T15:07:29.565Z, HEAD
`bc8213f36b7a29b4fdc30cfc74ddc23586117c64`, working tree não commitado.
Reprodução: `node 00_nucleo/diagnosticos/p1317-review-audit.cjs` no root;
acesso host necessário para conferir fixtures e binários `/tmp`.
Script somente leitura, SHA-256
`306a5a17c93cf9f0c6425d64ef27cd5cb6205608dd297732dfa8e92c13a7e6e8`.

**GO de inspeção; gates dinâmicos pendentes.** Source SHA-256
`39cf6ef6c3cd55872dfc40cbe0787faab7379013beb3e399183d8cc97d22d86c`,
L0 raw SHA-256 `99f8f50dc524eaa0e6742cfc6a3bb1db160895cd27ed04635721871cd5d6d7c2`.
Ao remover unicamente o braço Utf8 e restaurar o header de linhagem, o source
retorna ao hash exato do RED. Isso prova preservação literal dos testes novos
e antigos e ausência de outra mudança produtiva. O sufixo P1315/P1316 foi
também comparado ao snapshot anterior ao P1317. A obrigação normativa L0 é
idêntica ao freeze; só a linha `Hash do Código` mudou no resselo.

O novo braço seleciona `csv::ErrorKind::Utf8` em `loading.rs:956–958`, retorna
o literal aprovado pelo helper existente e deixa UnequalLengths/fallback
intactos. Não há varredura UTF-8 antecipada, comparação Display/fixture,
mudança de parser, valor, origem, assinatura ou caminho de leitura.

Foram conferidos 24 inputs congelados, 59 artefatos anteriores e dois
binários históricos, sem divergência. 330 casos históricos e 57 novos
totalizam 387 casos/1548 expectativas; 208 expectativas eram RED. O script
reconfere replays e transforma somente primeira linha nas causas explícitas.

Lint, fmt, diff-check e lineage dry-run registrados com exit 0. GREEN local,
build, suíte workspace e A/B candidato normal/repeat/reverse ainda pendentes
neste recibo. Esses resultados não são antecipados. Sem achado impeditivo
na implementação inspecionada; sem atestação técnica de isolamento.
