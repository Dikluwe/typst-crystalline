# P1490 — morfologia de `repr(stroke())`

## Resultado

P1490 foi concluído em fluxo contínuo conforme ADR-0127. `repr(stroke())`
passou de `"1pt + black"` para `"stroke()"`, em paridade morfológica com o
vanilla pinado. Os três estados explícitos de `paint`/`thickness` conservam as
formas existentes. Nenhum contrato público, campo de entidade ou fase do
pipeline mudou.

Os L0 proprietários foram atualizados antes do código e ressellados. A
implementação usa os bits existentes de `StrokeFields`: ausência bilateral
com valores default identifica o stroke totalmente automático. O caso
`P1489-B-001` foi rebaselined de `VIOLATED` para `MATCH` somente depois do
GREEN.

## Medição e decisão

Medição inicial em 2026-09-24T13:41:18-03:00, no HEAD
`9e3a267f554b89107d66e5001400de56d3cc6bb7`, working tree não commitado:

| Forma | Vanilla `9f2b6e871` | Cristalino pré-P1490 |
|---|---|---|
| `repr(stroke())` | `"stroke()"` | `"1pt + black"` |
| `repr(stroke(thickness: 1pt))` | `"1pt"` | `"1pt"` |
| `repr(stroke(paint: black))` | `"luma(0%)"` | `"luma(0%)"` |
| `repr(stroke(paint: black, thickness: 1pt))` | `"1pt + luma(0%)"` | `"1pt + luma(0%)"` |

Essa medição mostrou que somente o estado totalmente automático precisava de
correção. A hipótese inicial de generalizar qualquer stroke sem bits explícitos
foi refutada por valores internos não-default; por isso o fallback histórico
foi preservado. O teste RED falhou com `"1pt + black" != "stroke()"`; depois da
alteração, o teste focal passou 1/1.

## Verificação final e proveniência

Verificação dos produtos em 2026-09-24T14:06:26-03:00, no mesmo HEAD e working
tree não commitado herdado do P1489. O snapshot exato anterior está no fim de
`typst-p1489-resync-vanilla.md`; sobre ele, P1490 altera somente:

- `00_nucleo/prompts/compiler/eval/repr.md`;
- `00_nucleo/prompts/entities/geometry.md`;
- `01_core/src/compiler/eval/repr.rs`;
- o selo de `01_core/src/entities/geometry.rs`;
- o caso `P1489-B-001` em `lab/parity/matrix/manifest.yaml`;
- `lab/parity/matrix/fixtures/stroke-repr.typ`;
- este diagnóstico e o estado do passo 1490.

| Produto medido | SHA-256 |
|---|---|
| `lab/typst-original/target/release/typst` | `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea` |
| `target/debug/typst` | `74502211c30b453e18017e76e10dea5cc41653da2206c9174e013ab63adfd178` |

A execução `python3 lab/parity/matrix/runner.py --case P1489-B-001
--crystalline target/debug/typst --output /tmp/p1490-final.json` produziu
`MATCH=1`, `VIOLATED=0`, expectativa satisfeita e ordem bilateral idêntica.
Os hashes forward/reverse foram ambos
`29475665a92d3430dd6c277221eef544c3be283d0e18e9ca638b5573dde68cdc`.

Gates finais:

- `CARGO_BUILD_JOBS=1 cargo test -p typst-core p1490_repr_stroke -- --nocapture`: 1 passou;
- `CARGO_BUILD_JOBS=1 cargo build`: passou;
- `python3 -m unittest lab/parity/matrix/test_runner.py`: 56/56 passaram;
- `crystalline-lint .`: código 0, zero violações; avisos informativos preexistentes;
- `git diff --check`: passou.

## Limite explícito

O round-trip por argumento posicional da suite vanilla,
`stroke(eval(repr(case)))`, permanece lacuna separada: o constructor cristalino
ainda rejeita essa superfície. P1490 cobre `eval(repr(stroke()))` e reconstrói
os estados explícitos pelos argumentos nomeados já aceitos; não amplia o
contrato público para esconder a lacuna.
