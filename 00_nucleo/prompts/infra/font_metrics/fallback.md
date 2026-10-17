# L0 — infra/font_metrics/fallback
Hash do Código: 24aa0b1a

**Layer:** L3
**Exclusive consumer:** `03_infra/src/font_metrics/fallback.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/fallback-selection.toml sha256:faf6c20021b467fdb2a864625f4f6dd4386b5ef28c137b946e5cb4e4ce073d52
```

## Current evidence

- `03_infra/src/font_metrics/fallback.rs:23` — `face_kerning`.
- `03_infra/src/font_metrics/fallback.rs:46` — `advance`.
- `03_infra/src/font_metrics/fallback.rs:168` — `advance_shaped`.
- `03_infra/src/font_metrics/fallback.rs:184` — `vertical_metrics`.
- `03_infra/src/font_metrics/fallback.rs:226` — `cap_height`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/font_metrics/fallback.rs`.

## Owned contract

- Own FontMetrics for FallbackFontMetrics and face_kerning, using the resolution/cache children. Resolve per character and apply style variations to a cloned face, never mutate the cached base.
- The explicit `FontMetrics` size argument governs advance and shaped advance, including scaling and cache identity. For shaping, use a local style with that size when it differs from `style.size`; preserve all other style fields and the caller's style.
- Preserve legacy kern then kerx within one face, not cross-font kerning or a new GPOS path. advance_shaped delegates to the shaper with the shared face cache.
- Preserve FixedMetrics/0.6 defaults for unresolved vertical/ink/bbox metrics. Apply canonical math/ssty and TextItem predicates; never remap ink codepoints.
- Math constants come from the first applicable primary with MATH, while variants/assemblies/kern/italic-correction/attachments use the active face. Keep fallback behavior and neutral L1 return types.

## Verification and boundary

Run `cargo test -p typst-infra --lib font_metrics:: -- --test-threads=1` when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Imports e documentação de CI — 2026-10-13

### Medição anterior à decisão

`03_infra/src/font_metrics/fallback.rs` nas linhas 638, 639 contém os avisos selecionados. Fontes/owners, spans, saída original do compilador, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, status e diffstat estão em `00_nucleo/diagnosticos/infra-imports-docs-ci-2026-10-13/before.json.gz`. A pesquisa distingue referências produtivas, imports necessários somente à suíte descendente e menções em comentários; não constitui nova medição de paridade.

### Decisão e aceitação

Corrigir a separação/continuação de parágrafos e listas dos comentários Rustdoc. Preservar o conteúdo técnico e todas as asserções/casos de teste.

O mais textual entre os dois helpers vira a conjunção e, sem iniciar uma lista Markdown.

Manutenção interna sem mudança de assinatura pública, default, fase, fórmulas ou critérios de aceitação (fluxo contínuo ADR-0127). Não introduzir supressões de lint. Revalidar compilação de todos os targets, nomes e execução da suíte de infraestrutura, build, Clippy do workspace, Rustdoc e linhagem com resselo restrito. Não criar testes que apenas espelhem remoção de imports ou formatação de comentários.


## Inicialização explícita dos fixtures — field_reassign_with_default

### Medição anterior à decisão

`03_infra/src/font_metrics/fallback.rs` nas linhas 759, 896, 1041, 1160, 1225, 1263, 1322 cria defaults e em seguida sobrescreve campos locais de testes. A fonte, os spans e os L0 anteriores estão em `00_nucleo/diagnosticos/field-reassign-2026-10-15/before.json.gz`, com HEAD, working tree, UTC e diffstat; initial-diagnostics.json conserva os inicializadores apontados. Esta manutenção trata representação Rust, sem nova medição de paridade ou mudança no default do produto.

### Responsabilidade e aceitação

Inicializar no literal os campos contíguos de preparação dos sete estilos: math; font/size; font/size/lang/dir; font/size/weight. Preservar cada expressão e sua ordem, o clone style_800 e a posterior alteração de peso, testes MATH, fontes, kerning, RTL e todos os controles de shaping.

Usar `..Tipo::default()` para os campos restantes e retirar `mut` apenas onde deixam de existir mutações. Sem novos tipos, builders, APIs ou supressões de lint. L0 precede código e resselo restrito (fluxo contínuo ADR-0127). O gate selecionado deve falhar antes e passar depois; preservar nomes, inputs, expectativas e tolerâncias dos testes. Revalidar Clippy all-targets, suíte workspace, build, formato, Rustdoc e linhagem. Não criar testes que apenas espelhem a sintaxe do inicializador.
