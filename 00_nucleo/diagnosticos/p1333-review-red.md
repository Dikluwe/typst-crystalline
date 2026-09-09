# P1333 — gate RED antes de C

Revisor `/root/p1333_review`; manifesto
`75eea7c2a28cd8c75098d4d8fc5b2c733c30c09b7eca52a2ce7a772a24e0e99b`.
Recibo RED conferido, SHA-256
`ba33d5af68a0ad0240d4b716bbf4c222b44d032ca6bdb6842da1a96cf2c89a5b`.

Comando registrado: `cargo test --release --locked -p typst-core
compiler::stdlib::calc::p13 -- --nocapture`, target
`/tmp/p1333-target.MDDkou`, UTC de 2026-09-09T15:26:52.206584+00:00
a 2026-09-09T15:28:38.453121+00:00. HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, árvore não commitada;
before/after preservam inventário e diff/stat no recibo.

A compilação terminou e executou a suíte: 39 testes, 32 passam, 7 falham,
exit 101. As falhas são asserções de mensagens: named/aridade legado
contra tipo content/boolean, Length misto ou overflow esperados. Não são
erro de compilação, fixture que falha ao construir valor ou ausência de
origem. As testemunhas públicas `#calc.abs(false, 2)` e With também falham
pela mesma precedência sem precisar das fixtures sintéticas incoerentes.
Controles de primeiro válido, avaliação eager e resultados históricos
passam. Uma falha inicial de cada loop limita quais combinações chegaram
a executar nesta rodada; GREEN posterior deve percorrer os mesmos loops.

O freeze nativo precede o início do RED. O freeze CLI completo foi gravado
durante sua compilação e antes de C; seus hashes nativos são idênticos.
Essa ordem conserva o requisito de testes congelados antes do candidato.

Veredito: RED real confirmado, gate permite materializar C no recorte
congelado. Não há objeção bloqueante concreta. Permanecem GREEN dos mesmos
testes, CLI integral e gates arquiteturais/finais. Este parecer não atesta
isolamento técnico nem paridade de todos os guards.
