# P1333 — recibo público da autoridade B

**Veredito: PASS no fragmento CLI congelado, nas três execuções.**
Regime A/B executado sem atestação técnica de isolamento, sem selo de
refinamento e sem mutation score. Este recibo não julga build, workspace,
RED/GREEN nativo ou conteúdo da implementação: B não leu esses recibos
privados, o runtime ou o diff produtivo.

A referência normativa continua sendo o L0 calc, SHA-256 normalizado
`517e1544fbd419a5b1c13e524c02570d3c22606a7b97d25562723c278bdd2e83`.
Manifesto `p1333-manifest.json`:
`75eea7c2a28cd8c75098d4d8fc5b2c733c30c09b7eca52a2ce7a772a24e0e99b`.
Freeze `p1333-ab-freeze.json`:
`bdbadb8d0c91dc9823c085489ae2c058f0a20e0d1db844ae1346176a316c1199`.
Todos os hashes de inputs e artifacts declarados nesse freeze foram
recalculados e coincidiram; nenhum oráculo congelado foi alterado.

## Proveniência e reprodução

Os recibos registram HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`,
working tree não commitado, diff stat, UTC, argv, caminhos dos binários e
saídas integrais. Os SHA-256 dos próprios binários também foram conferidos:

- BASE: `dfe7c3ab89eaa145d79e5b56ac72c657e49c6c994f9a88a4338915808ec0e8ea`.
- Vanilla ratificado `a51e02804`: `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
- Candidato: `79470612fc121fa42a6898846f85d0b0325edf517c90a830c01cde947f748ffe`.

Executar `p1333-ab-cli.py --candidate /tmp/p1333-target.MDDkou/release/typst
--output <novo-p1333-ab-recibo.json> --order normal`, repetir com novo output
e depois usar `--order reverse`. O runner deve ser executado pelo Python
com `PYTHONDONTWRITEBYTECODE=1`; a invocação completa está no freeze.

| Recibo | UTC inicial | SHA-256 |
|---|---|---|
| `p1333-ab-cli-normal.json` | 2026-09-09T15:37:43.033014+00:00 | `700fad895acfce0d5f284d8cbbc260320a739e507a71ec74ccccc8f59f720213` |
| `p1333-ab-cli-repeat.json` | 2026-09-09T15:40:03.820101+00:00 | `a7777f479f3ea01eaf4f47cff37be98d0d42491cf3d26c01a1b0612a419ba71f` |
| `p1333-ab-cli-reverse.json` | 2026-09-09T15:40:07.423627+00:00 | `55c891c9d685d6f463ac279482af2da0318c2d10bfc614aeaaaac4caf0b746bf` |

## Auditoria literal e alcance

B recalculou as comparações a partir dos JSON, sem confiar apenas no
booleano de sucesso do runner: 712 chaves únicas caso/perfil por execução,
mesmas expressões e argv, candidato igual ao expected congelado em
exit/stdout/stderr completos. BASE e VANILLA permaneceram iguais à coleta
pré-candidato. Os resultados foram idênticos entre execuções; a ordem
invertida foi conferida preservando os quatro perfis de cada caso.

As 616 células históricas coincidem com BASE antes da migração. Dessas,
576 permanecem intactas e 40 migram exclusivamente pela falha do primeiro
posicional. Das 96 células novas, 36 demonstram a mesma correção. Portanto,
há 76 mudanças, todas iguais ao vanilla, e a coincidência literal passa
de 532 para 608 das 712 células. As 104 dívidas restantes ficam preservadas;
não há alegação de paridade geral. Nenhum Unknown satisfaz esta aceitação.

Fixtures nativas com metadados deliberadamente incoerentes ensaiam
robustez fora do domínio causal; não provam paridade nem legitimam Args
stale. O ganho acima é sustentado independentemente pelas chamadas
públicas coerentes, incluindo With, spreads, alias/import e suas origens.
Ausência, primeiro valor válido, eager e dívida de resolução matemática
continuam cobertos pelas observações literais preservadas.
