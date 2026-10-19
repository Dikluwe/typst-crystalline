# Atomização comportamental de `stdlib/layout.rs` — 2026-09-30

## Proveniência

Medições na working tree não commitada sobre HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`. O estado anterior está em `atomizacao-stdlib-layout-2026-09-30-before.json.gz`; o estado final, com `git status`, `git diff HEAD --stat`, SHA-256 dos owners e linhas de código, está em `atomizacao-stdlib-layout-2026-09-30-after.json.gz`. Os snapshots descrevem árvores não commitadas, não commits novos.

## Inventário

O estado anterior tinha 2966 linhas em `stdlib/layout.rs`. O hub final contém 235 linhas, com `native_align`, `extract_alignment`, os reexports estáticos e os testes históricos. Foram movidas 41 funções para 15 owners comportamentais, cada qual com Prompt L0 exclusivo. Os corpos dos algoritmos foram preservados; a visibilidade interna necessária aos reexports não muda a superfície publicada pelo módulo `stdlib`.

| Owner L1 | Comportamento |
|---|---|
| `layout/place.rs` | place e flush |
| `layout/grid.rs` | grid e tracks |
| `layout/page.rs` | page e geometria |
| `layout/casting.rs` | conversões compartilhadas de medidas, lados e stroke |
| `layout/pad.rs` | padding |
| `layout/hide.rs` | ocultação |
| `layout/spacing.rs` | h, v e linebreak |
| `layout/block.rs` | block |
| `layout/stack.rs` | stack |
| `layout/box_elem.rs` | box |
| `layout/repeat.rs` | repeat |
| `layout/columns.rs` | columns e colbreak |
| `layout/measure.rs` | layout e measure |
| `layout/stroke.rs` | stroke |
| `layout/pagebreak.rs` | pagebreak |

`layout.md` mantém o contrato de `align`, os bindings e o mapa histórico das funções. Cada prompt filho possui um consumer L1 único. O despacho é estático. Não foi alterado default, assinatura, fase do pipeline nem comportamento de linguagem intencionalmente.

## Verificação

- Antes da extração: `cargo test -p typst-core --lib --offline --locked --quiet` — 6401 passaram, 2 ignorados (`/tmp/layout-baseline.log`).
- Depois do resselo: mesmo comando — 6401 passaram, 2 ignorados (`/tmp/layout-final-test.log`).
- `cargo build --offline --locked` — exit 0 (`/tmp/layout-final-build.log`).
- `crystalline-lint .` — exit 0 (`/tmp/layout-final-lint.log`); avisos informativos e warnings V16 do código existente permanecem.
- `git diff --check` — exit 0.

A suíte verifica ausência de regressões no comportamento coberto. Isto não é benchmark nem uma nova medição bilateral contra o vanilla pinado. Para pesquisa algorítmica, os maiores arquivos resultantes são `stroke`, `casting` e `page`, cujos comportamentos agora estão localizados em arquivos próprios.
