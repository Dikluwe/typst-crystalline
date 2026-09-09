# P1330 — recibo final A/B público

Veredito: **PASS no recorte CLI congelado**, emitido em
2026-09-09T13:27:55Z por `/root/p1330_tests`. Regime executado sem atestação de
isolamento. Nenhum runtime, patch, baseline privada ou RED privado foi lido.
Não se executou cargo nem se alterou oráculo protegido nesta auditoria.

Manifesto R2 SHA-256
`fa90e9d05855842467eb7baac85d81a157af03bb285b9c1d40df0faf1f660acf`;
L0 normativo SHA-256
`9a5d734e086297d49d80d567bdb5527919049de19b7aea11f43b4467a972d2db`.
Freeze pré-C: `p1330-ab-freeze.md`, SHA-256
`637d1bf3de8480502a9c6b3be793101d5495dc9e4bbd932e6eb5d2e772be8edf`.
Os 12 pins de artefatos protegidos e três pins de executáveis foram recalculados
e coincidiram. A linha recíproca Hash do Código é a única exclusão no hash do L0.

## Proveniência e observações

HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado.
Cada recibo contém diff/stat integral, instante inicial, argv por célula e
hashes dos executáveis. Candidato `/tmp/p1330-target.f0lmDu/release/typst`,
SHA-256 `6f1db621bc0b2a7fe4fc9d05925fb96636f33232b8527b83c040b793970fbda0`.
BASE e vanilla conservam os pins do freeze; vanilla ratificado `a51e02804`.

| Recibo público | Início UTC | Células | SHA-256 |
|---|---|---|---|
| p1330-ab-cli-normal.json | 2026-09-09T13:23:45.185316+00:00 | 332 | `082b6437d31dcc65ee61d9159ab903d9a84f24b04bfae6f15b71daacc0743402` |
| p1330-ab-cli-repeat.json | 2026-09-09T13:23:50.969622+00:00 | 332 | `295c4ff6ca975d76b6dcaeed618206aa82acc119c24ecb98c1e7187ab3daf610` |
| p1330-ab-cli-reverse.json | 2026-09-09T13:23:56.512552+00:00 | 332 | `f62c5c2de1a8fe9f8af98a449af9f5759a41b5c753b40794d340466330aff77e` |

Comparadas literalmente 996 observações candidatas de exit/stdout/stderr,
sem normalização: zero divergências, zero Unknown. A auditoria recomputou
as comparações sem confiar nos booleans publicados pelo runner. Confirmou
83 casos distintos × quatro perfis por execução, sem células ausentes,
duplicadas ou extras, correspondência exata de expressões e argv, e ordem
efetiva normal/repetida/invertida de casos com ordem de perfis preservada.

Resultados de BASE e VANILLA coincidem integralmente com a captura pré-C em
todas as três execuções. Após indexação por caso/perfil, cada linha completa
do recibo repetido e do invertido é idêntica à linha correspondente normal.
As 248 expectativas históricas não relacionadas ao overflow permanecem
iguais, inclusive classificação; as quatro sucessoras históricas de overflow
coincidem com vanilla pré-C. As demais 80 células seguem os novos literais
pré-C. O nome externo calc.abs, dívida do parser do literal mínimo e
precedência dos guards seguem as dívidas declaradas no freeze.

## Reprodução e limite do veredito

Auditor somente leitura: `p1330-ab-audit.py`, SHA-256
`b8a1ce79fb9c16c6ed39d7e50a3a205b8ac10626bf62ad5d0f954492f59095a0`.
Comando executado com exit 0:

```sh
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/p1330-ab-audit.py
```

Uma execução de auditoria, sem revisão guiada por falha candidata. Este recibo
atesta somente os resultados públicos do corpus e sua integridade causal
declarada. Não atesta isolamento técnico, GREEN nativo, arquitetura, workspace
ou paridade geral; esses gates pertencem ao integrador e revisor final.
