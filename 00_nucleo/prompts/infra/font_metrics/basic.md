# L0 — infra/font_metrics/basic
Hash do Código: fffeb8c0

**Layer:** L3
**Exclusive consumer:** `03_infra/src/font_metrics/basic.rs`

## Current evidence

- `03_infra/src/font_metrics/basic.rs:24` — `typo_metrics`.
- `03_infra/src/font_metrics/basic.rs:53` — `edge_offset_pt`.
- `03_infra/src/font_metrics/basic.rs:110` — `advance`.
- `03_infra/src/font_metrics/basic.rs:164` — `vertical_metrics`.
- `03_infra/src/font_metrics/basic.rs:178` — `cap_height`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/font_metrics/basic.rs`.

## Owned contract

- Own typo_metrics, edge_offset_pt and FontMetrics for FontBookMetrics. Convert design units proportionally by size/upem, preserving missing-glyph/default fallback and zero-unit guards.
- Use OS/2 metrics before hhea and respect cap-height/baseline edge policy. Explicit Length edges resolve at the supplied size; unknown metrics preserve current defaults.
- Text ink unions use max(0,ymax) and max(0,-ymin). Glyph-by-id ink retains signed ymax/-ymin, so a glyph entirely above baseline can have negative bottom extent. Bbox methods use the requested codepoint or glyph id in this face, without remapping.
- Apply math italic correction only to the singular base outside TextItem provenance; variation selectors are not extra bases. Delegate MATH/ssty extraction to the math child and glyph_to_char to the facade's precomputed map.

## Verification and boundary

Run `cargo test -p typst-infra --lib font_metrics:: -- --test-threads=1` when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.


## Inicialização explícita dos fixtures — field_reassign_with_default

### Medição anterior à decisão

`03_infra/src/font_metrics/basic.rs` nas linhas 505, 513, 520 cria defaults e em seguida sobrescreve campos locais de testes. A fonte, os spans e os L0 anteriores estão em `lab/results/construction/diagnostics/field-reassign-2026-10-15/before.json.gz`, com HEAD, working tree, UTC e diffstat; initial-diagnostics.json conserva os inicializadores apontados. Esta manutenção trata representação Rust, sem nova medição de paridade ou mudança no default do produto.

### Responsabilidade e aceitação

Construir os três TextStyle locais com os mesmos top_edge/bottom_edge: comprimentos pt/em e métrica baseline, herdando os demais defaults. Conservar tamanho fornecido, conversão de unidades, chamadas text_edges e tolerâncias existentes.

Usar `..Tipo::default()` para os campos restantes e retirar `mut` apenas onde deixam de existir mutações. Sem novos tipos, builders, APIs ou supressões de lint. L0 precede código e resselo restrito (fluxo contínuo ADR-0127). O gate selecionado deve falhar antes e passar depois; preservar nomes, inputs, expectativas e tolerâncias dos testes. Revalidar Clippy all-targets, suíte workspace, build, formato, Rustdoc e linhagem. Não criar testes que apenas espelhem a sintaxe do inicializador.
