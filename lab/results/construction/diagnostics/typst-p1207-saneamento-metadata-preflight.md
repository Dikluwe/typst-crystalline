# P1207 — saneamento de metadata e preflight

## Proveniência

- Execução: `2026-08-26T09:26:55-03:00`.
- HEAD: `7fb5bb6d9ee73298af4fd09bb858cb26e5fdd568`.
- Working tree: não commitado, acumulando P1181–P1207; índice Git vazio.
- O corpo produtivo dos 21 consumers não foi alterado: em cada um foi inserida
  exclusivamente a linha `//! @prompt-hash` imediatamente após `//! @prompt`.

## Consumers antes sem metadata canônica

| Consumer | Owner L0 | Selo |
|---|---|---|
| `entities/layouter_runtime_state.rs` | `entities/layouter_runtime_state.md` | `841431d2` |
| `entities/rel.rs` | `entities/rel.md` | `00000000` (owner sem hash efetivo publicado) |
| `entities/numbering.rs` | `entities/numbering.md` | `00000000` (owner pendente declarado) |
| `entities/elements/curve.rs` | `entities/elements/curve.md` | `4f9ae279` |
| `entities/elements/grid_hline.rs` | `entities/elements/grid_hline.md` | `376ff1cf` |
| `entities/elements/grid_vline.rs` | `entities/elements/grid_vline.md` | `39682162` |
| `entities/elements/label.rs` | `entities/elements/label.md` | `bdb4a645` |
| `entities/elements/ref.rs` | `entities/elements/ref.md` | `9b761671` |
| `entities/elements/table_hline.rs` | `entities/elements/table_hline.md` | `ab5afc83` |
| `entities/elements/table_vline.rs` | `entities/elements/table_vline.md` | `872a2d94` |
| `compiler/eval/cast.rs` | `compiler/eval/cast.md` | `41234a16` |
| `compiler/layout/footnote_flush.rs` | `compiler/footnote_overflow_columns.md` | `b7063a5c` |
| `compiler/stdlib/collections.rs` | `compiler/stdlib/collections.md` | `18df1a29` |
| `compiler/stdlib/gradients.rs` | `compiler/stdlib/gradients.md` | `0b693784` |
| `compiler/stdlib/layout.rs` | `compiler/stdlib/layout.md` | `27cb4cc8` |
| `compiler/stdlib/numbering.rs` | `compiler/stdlib/numbering.md` | `00000000` (owner sem hash efetivo publicado) |
| `compiler/stdlib/transforms.rs` | `compiler/stdlib/transforms.md` | `6fc7fe52` |
| `compiler/stdlib/shapes.rs` | `compiler/stdlib/shapes.md` | `61593773` |
| `compiler/stdlib/ref.rs` | `compiler/stdlib/ref.md` | `00000000` (owner sem hash efetivo publicado) |
| `compiler/stdlib/label.rs` | `compiler/stdlib/label.md` | `f7149845` |
| `03_infra/src/fontdb.rs` | `infra/fontdb.md` | `6fcb3340` |

V15 já provava ownership 1:1; o linter aceitou os quatro selos neutros e o
preview não propôs reparo para eles. Não se inventou hash fora do algoritmo.

## Reclassificações e custom CA

- `_convencoes.md` tornou-se ADR-0130 vigente, pois decide governança e não
  materializa consumer.
- O antigo `shell/custom-ca-cert.md` foi preservado como diagnóstico histórico.
- A auditoria confirmou implementação em L2/L3/L4: precedência flag/env,
  transporte, leitura/validação PEM, adição de roots, hostname verification e
  não exposição do segredo têm código e testes.
- As responsabilidades locais permanecem nos quatro owners 1:1. As invariantes
  compartilhadas foram extraídas para
  `_nuclei/network/custom-ca-cert.toml`, hash efetivo completo
  `0b28776068ad6b8e85a028a26cfc359679770b03f76d250bfd3ad68ff72643e3`,
  pinado pelos quatro prompts. Seus consumers foram ressellados focalmente.
- Nenhuma mudança de contrato/default/fase foi necessária; ADR-0127 não abriu
  novo gate.

## Gates

- Antes: V5=312, V7=2, V15=0, V26=0.
- Depois: V5=311, V7=0, V15=0, V26=0. Fora do escopo permanecem V16=210,
  V17=36, V18=2, V19=349, V20=600 e V21=24.
- Dois `--fix-hashes --dry-run` produziram o mesmo SHA-256:
  `43727b9d63432cee09c12fcd9ba2222400d468ff85a8cd1044a8778dad0e337c`.
  Nenhum reparo global foi escrito.
- `cargo test -p typst-shell cert`: 1 passed.
- `cargo test -p typst-infra custom_ca`: GREEN.
- `cargo test -p typst-wiring --test crystalline_lint`: 2 passed.
- `cargo build`: GREEN com warnings preexistentes.
- `git diff --check`: GREEN; índice Git vazio.

Resultado: o preflight transacional está desbloqueado e determinístico. O lote
global de 311 V5 permanece deliberadamente para passo próprio.
