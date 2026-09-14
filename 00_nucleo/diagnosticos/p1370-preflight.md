# P1370 — preflight da retificação S02

- medição: `2026-09-13T12:25:41-03:00`;
- HEAD: `8ceea20f92dff06b6ca5a34828135c0fd3e35248`;
- working tree não commitado: 25 tracked files no `git diff HEAD --stat`,
  2.129 inserções e 223 remoções;
- resultado P1369: 31.718 bytes, SHA-256
  `1e6c38c2c7fe387e4877d18eb6890e11a25d8572e3a39483bbaf3261dded7b05`;
- fonte vanila `typst-pdf/src/lib.rs`: SHA-256
  `0143692141c42e5a5ee64bea71f26fa0b5ae217544795adbe28d73d81b033b2d`;
- fonte vanila `typst-cli/src/compile.rs`: SHA-256
  `a943a87637dd1f85a95f39c29175225c378cbf6d7a0e8d30645eb5cca7470703`;
- pin vanila: `586e1bd43fae6c9a973218163d3165c53ab8d16d`.

O P1369 preservou o expected refutado e registrou quatro `DIFFERENCE`, embora
as observações brutas dos dois produtos fossem iguais. P1370 é uma revisão
nova, causalmente posterior: muda o oráculo com base na medição e na fonte,
sem adulterar os artefatos anteriores e sem executar outra vez os binários.

