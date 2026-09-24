# P1491 — completar `measure().baseline`

## Resultado

P1491 foi concluído em fluxo contínuo conforme ADR-0127. `measure()` agora
expõe `width`, `height` e `baseline` nessa ordem, com a baseline derivada
da geometria causal do frame. O helper público histórico
`measure_content_real -> (f64, f64)` conserva a assinatura; a coordenada
adicional fica num helper de visibilidade interna ao compilador. Não houve
campo novo de entidade, método de trait, mudança de fase nem quebra de
compatibilidade.

O caso `P1489-L-001` foi rebaselined de `VIOLATED` para `MATCH` somente
depois de passar bilateralmente para retângulo, texto, caixa com shift, vazio
e composto. Os controles de largura composta, sombra de `measure`, contexto
e rejeição de argumentos continuam ativos.

## Medição anterior à decisão

Medição direta em 2026-09-24T14:12:42-03:00, no HEAD
`b169a78c6c6bc17764b294f501ac09b63d678b03`, antes do código P1491. O
vanilla foi o binário ratificado do pin
`9f2b6e8715237cb086899a42873660fe744622e8`.

| Conteúdo | `width` | `height` | `baseline` |
|---|---:|---:|---:|
| `rect(width: 10pt, height: 20pt)` | 10pt | 20pt | 20pt |
| `[A]` | 7.65pt | 7.24pt | 7.24pt |
| `box(baseline: 3pt, rect(...))` | 10pt | 20pt | 17pt |
| vazio | 0pt | 0pt | 0pt |
| `[A#box(...)]` | 17.65pt | 20pt | 17pt |

A caixa refutou explicitamente a hipótese cômoda `baseline = height`. A fonte
upstream confirma a causa: `measure.rs` devolve `frame.baseline()`, e
`inline/box.rs` desloca a baseline natural pelo shift solicitado. O RED
cristalino foi o import inexistente de
`measure_content_real_with_baseline`; a sonda de matriz permaneceu
`VIOLATED` antes da implementação.

## Implementação

- `compiler/layout` deriva o triplo a partir das arestas reais de texto e
  das extensões inline `(ascent, descent)`; conteúdo sem baseline natural
  alinha pela borda inferior.
- `boxed` deixou de descartar `baseline`, aplica o shift à baseline natural,
  isola fisicamente a linha do pai durante o layout do body e rebasa itens de
  frame para a origem inline da caixa.
- `call_dispatch` apenas projeta o triplo no dicionário ordenado
  `width`, `height`, `baseline`.
- A fixture bilateral inclui os cinco corpos, largura composta e função local
  homônima para impedir interceptação indevida.

## Linhagem ressellada

| Owner | `@prompt-hash` | `Hash do Código` |
|---|---|---|
| `compiler/eval/call_dispatch` | `4751e254` | `493f95ec` |
| `compiler/layout/boxed` | `18cb46f6` | `c6dc5c19` |
| `compiler/layout` | `24b71d80` | `5c644e22` |
| `compiler/stdlib/layout` | `c0c28126` | `0ccb3a17` |

`crystalline-lint --fix-hashes .` terminou com zero drift e
`crystalline-lint .` terminou com código 0. Os avisos V16–V20 exibidos são
informativos preexistentes.

## Verificação final e proveniência

Verificação final em 2026-09-24T14:41:53-03:00, no mesmo HEAD e working tree
não commitado contendo P1491:

| Produto medido | SHA-256 |
|---|---|
| vanilla release pinado | `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea` |
| cristalino debug | `1824b999f3e38f8b718018a3055123cc13ea70bd1cda50a556bf133fa92ca9ff` |
| cristalino release | `91cc79e8d7c13aa1baa81522ff20a29e5bd5c421b66dacfc316d71e03aaad3b8` |

A execução final
`python3 lab/parity/matrix/runner.py --case P1489-L-001` produziu
`MATCH=1`, `VIOLATED=0`, expectativa satisfeita e ordem bilateral
idêntica. Os hashes forward/reverse foram ambos
`8a76cfb8baf54dfd118e00ce8ad43a4885ad8cea68ce7c3091ed1e3071279313`.

Gates finais:

- teste focal P1491: 1/1 passou;
- controles P222 de argumentos: 6/6 passaram;
- controles P712 de medida, contexto e sombra: 8/8 passaram;
- `cargo test -p typst-core`: 6144 unitários, 8 integrações e 3 doctests passaram;
- `CARGO_BUILD_JOBS=1 cargo build`: passou;
- `CARGO_BUILD_JOBS=1 cargo build --release`: passou;
- fixture HTML bilateral com `--features html`: vanilla e cristalino, código 0;
- `python3 -m unittest lab/parity/matrix/test_runner.py`: 56/56 passaram;
- `crystalline-lint .`: código 0;
- `git diff --check`: executado após o registo final.

## Estado exato do working tree medido

O estado final continuava não commitado sobre
`b169a78c6c6bc17764b294f501ac09b63d678b03`. A lista exata de ficheiros
mostrada por `git status --short` era:

- `00_nucleo/materialization/typst-passo-1491.md`;
- `00_nucleo/prompts/compiler/eval/call_dispatch.md`;
- `00_nucleo/prompts/compiler/layout.md`;
- `00_nucleo/prompts/compiler/layout/boxed.md`;
- `00_nucleo/prompts/compiler/stdlib/layout.md`;
- `01_core/src/compiler/eval/call_dispatch.rs`;
- `01_core/src/compiler/layout/boxed.rs`;
- `01_core/src/compiler/layout/mod.rs`;
- `01_core/src/compiler/stdlib/layout.rs`;
- `lab/parity/matrix/fixtures/measure-baseline.typ`;
- `lab/parity/matrix/manifest.yaml`;
- `00_nucleo/diagnosticos/typst-p1491-measure-baseline.md` (novo).

O `git diff HEAD --stat` dos 11 ficheiros já rastreados registrou 283 inserções
e 36 remoções; o diagnóstico novo não entra nesse total por ainda estar
untracked.

## Limite explícito

P1491 não amplia `measure(width:)` nem `measure(height:)`; esses argumentos
continuam rejeitados pelo contrato atual. Também não transforma a baseline em
campo público de `Frame`: a coordenada é preservada apenas no caminho interno
necessário à superfície de linguagem.
