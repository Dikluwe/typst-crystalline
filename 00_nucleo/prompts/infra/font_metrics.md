# L0 — infra/font_metrics
Hash do Código: 5e716e38
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/primary-resolution-cache.toml sha256:b18027804dddf2f9eb87a3d01168d95dffaa34ad6839bad618e758ffbff080b4


**Layer:** L3
**Exclusive consumer:** `03_infra/src/font_metrics.rs`

## Current evidence

- `03_infra/src/font_metrics.rs:27` — `FontBookMetrics`.
- `03_infra/src/font_metrics.rs:40` — `from_bytes`.
- `03_infra/src/font_metrics.rs:49` — `FallbackFontMetrics`.
- `03_infra/src/font_metrics.rs:68` — `new`.
- `03_infra/src/font_metrics.rs:80` — `clone`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/font_metrics.rs`.

## Owned contract

- Own FontBookMetrics/FallbackFontMetrics state, lifetimes, constructors, Clone and existing internal reexports only. Metric algorithms, caches, fallback, MATH and resolution belong to their child owners.
- FontBookMetrics::from_bytes parses face index zero, returns None for invalid bytes, uses 1000 for zero units-per-em and eagerly builds its reverse glyph map through the math child.
- FallbackFontMetrics::new borrows World and creates lazy Arc/Mutex caches, including the shared shaper face cache. Clone shares every cache and the same World; no global state or I/O enters L1.
- Preserve the L1 FontMetrics contract and existing crate-private edge_offset_pt/ssty/reverse-map access. ttf-parser/rustybuzz types stay inside L3; do not add public signatures or broaden private child visibility.

## Child owners

- `00_nucleo/prompts/infra/font_metrics/basic.md` owns only `03_infra/src/font_metrics/basic.rs`.
- `00_nucleo/prompts/infra/font_metrics/caches.md` owns only `03_infra/src/font_metrics/caches.rs`.
- `00_nucleo/prompts/infra/font_metrics/fallback.md` owns only `03_infra/src/font_metrics/fallback.rs`.
- `00_nucleo/prompts/infra/font_metrics/math.md` owns only `03_infra/src/font_metrics/math.rs`.
- `00_nucleo/prompts/infra/font_metrics/resolution.md` owns only `03_infra/src/font_metrics/resolution.rs`.

## Verification and boundary

Run `cargo test -p typst-infra --lib font_metrics:: -- --test-threads=1` when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Reuso limitado da resolução primária — 2026-10-10

### Medição anterior à decisão

`diagnosticos/content-cost-profile-2026-10-10/{resolution-calls,cpu-style-current,checked-phases}.json.gz` guarda HEAD/working tree/UTC, scripts, fontes, comandos e release `9ae0a3d9285b8d0b4bae0da1db058483e7f5543003fabff46bb7d93b17e474f2`. As tabelas de 1.600 células chamam select_pattern 48.019 vezes, resolve_primary 6.401 vezes e text_edges/vertical_metrics 3.200/3.201 vezes; a seleção aparece em 25–29% das amostras inclusivas de CPU. `font_metrics/resolution.rs:27–85` depende de FontList, variante efetiva e legibilidade da face para formar a cadeia base; não aplica tamanho nem variações. `:105–124` acrescenta MATH separadamente; `font_metrics.rs:49–86` já compartilha caches locais entre clones. Inferência refutável: reusar a última resolução pode amortizar scans sem retenção crescente; invalidar se identidades/cadeia não forem preservadas ou custo de lock/retenção anular o ganho.

### Responsabilidade exclusiva

Adicionar somente carrier privado para a última resolução primária completa, inicializado vazio em new e compartilhado em Clone. Tipos de chave/entrada e algoritmo pertencem exclusivamente ao child owner resolution. Aplicar o Núcleo pinado; não adicionar assinaturas nem campos públicos.

### Aceitação e classe

Mudança interna ADR-0127 em fluxo contínuo. Teste RED de consultas repetidas nos mesmos argumentos através de clones; controles de família/regex/ordem/variante, instâncias independentes, falhas transitórias e math/variações contra instância fresca. Gate quantitativo de alocações e tempo pareado, documentos equivalentes ao release anterior e refutadores de muitas identidades, core/infra/build/lint. Aceitação de linguagem por seleção, métricas, origem/efeitos e geometria; não copiar mecânica do vanilla nem inferir hit por hash. Retirar em divergência ou regressão robusta.
