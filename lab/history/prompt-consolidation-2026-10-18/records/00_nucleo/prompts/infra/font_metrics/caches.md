# L0 — infra/font_metrics/caches
Hash do Código: e559f648

**Layer:** L3
**Exclusive consumer:** `03_infra/src/font_metrics/caches.rs`

## Current evidence

- `03_infra/src/font_metrics/caches.rs:20` — `CachedFace`.
- `03_infra/src/font_metrics/caches.rs:29` — `new`.
- `03_infra/src/font_metrics/caches.rs:41` — `face`.
- `03_infra/src/font_metrics/caches.rs:58` — `ShapedWidthKey`.
- `03_infra/src/font_metrics/caches.rs:75` — `AdvanceWidthKey`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/font_metrics/caches.rs`.

## Owned contract

- Own CachedFace, ShapedWidthKey/AdvanceWidthKey and cache helpers. Font bytes own the face lifetime; preserve the existing encapsulated unsafe and Arc sharing.
- Load faces lazily by slot and share successful parses across clones. Failed reads/parses are not cached. Never replace per-instance state with a global cache.
- Width keys preserve text, effective measurement size bits, family, bold/italic/weight, direction, language, variation hash and explicit math mode (which changes the primary/fallback chain); advance keys additionally distinguish TextItem provenance and math size.
- Advance cache helpers receive the explicit `FontMetrics::advance` size; never substitute `style.size` for that argument. Shaped keys use the effective style supplied by the fallback owner.
- Local regressions compare shared-cache and fresh-instance widths in both request orders, using real embedded fonts with discriminating math fallback and explicit-size inputs; they characterize correctness rather than performance or new upstream parity.
- Preserve tracking bypass in shaped-width caching; None results are not inserted. Read under the original lock scope, compute outside it and insert afterward; do not alter lock order or failure retry behavior.

## Verification and boundary

Run `cargo test -p typst-infra --lib font_metrics:: -- --test-threads=1` when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.


## Auxiliares de teste e bindings produtivos — 2026-10-14

### Medição anterior à decisão

`03_infra/src/font_metrics/caches.rs` nas linhas [129, 210] apresenta os avisos selecionados.
`lab/results/construction/diagnostics/ci-helpers-testes-2026-10-14/before.json.gz`
conserva fonte/owner, HEAD, working tree não commitado, UTC e diffstat.
references-before.json.gz registra chamadas, imports, módulos e referências;
o estado da suíte/inventário anterior foi verificado por hashes. A lista de
testes foi capturada antes da edição. Não se infere inutilidade funcional
apenas de dead_code: as chamadas privadas e sua compilação foram auditadas.

### Responsabilidade e aceitação

Remover as duas atribuições locais puras variant sem consumidor e seu import exclusivo. axis_variations_for_text_style já chama text_style_to_font_variant internamente e continua sendo usado para o hash de eixos, com as mesmas variações explícitas e precedência. Preservar integralmente campos/chaves, ordem do hash, tamanhos, math, tracking bypass, locks e política de cache. Não substituir estas chamadas por locals com underscore.

Manutenção interna em fluxo contínuo ADR-0127: nenhuma API pública, campo,
default de produto, fase, dependência ou supressão de lint muda. O gate de
diagnósticos selecionados deve falhar antes e passar depois; ele não é um
novo teste funcional nem prova de paridade. Preservar a lista e as expectativas
dos testes existentes, executar a suíte completa, compilar lib/all-targets,
verificar build/rustdoc/fmt/linhagem e registrar o inventário total. Não criar
testes que apenas espelhem cfg ou remoção de bindings. Resselo restrito ao owner.


## Inicialização explícita dos fixtures — field_reassign_with_default

### Medição anterior à decisão

`03_infra/src/font_metrics/caches.rs` nas linhas 282, 285 cria defaults e em seguida sobrescreve campos locais de testes. A fonte, os spans e os L0 anteriores estão em `lab/results/construction/diagnostics/field-reassign-2026-10-15/before.json.gz`, com HEAD, working tree, UTC e diffstat; initial-diagnostics.json conserva os inicializadores apontados. Esta manutenção trata representação Rust, sem nova medição de paridade ou mudança no default do produto.

### Responsabilidade e aceitação

Construir style_a e style_b com weight 400 e 700 nos próprios literais TextStyle. Preservar as comparações de axis_hash/chaves e o controle repetido do mesmo peso; não tocar caches, locks ou derivação de variações.

Usar `..Tipo::default()` para os campos restantes e retirar `mut` apenas onde deixam de existir mutações. Sem novos tipos, builders, APIs ou supressões de lint. L0 precede código e resselo restrito (fluxo contínuo ADR-0127). O gate selecionado deve falhar antes e passar depois; preservar nomes, inputs, expectativas e tolerâncias dos testes. Revalidar Clippy all-targets, suíte workspace, build, formato, Rustdoc e linhagem. Não criar testes que apenas espelhem a sintaxe do inicializador.
