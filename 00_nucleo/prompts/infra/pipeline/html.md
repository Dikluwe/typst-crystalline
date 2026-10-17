# L0 — infra/pipeline/html
Hash do Código: e24ee592

**Layer:** L3
**Exclusive consumer:** `03_infra/src/pipeline/html.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
- 00_nucleo/prompts/_nuclei/export/format-option-precedence.toml sha256:68c5c10f124c2c098879318df16c59f6c1477e9172cfa334b603f45ec698ae11
- 00_nucleo/prompts/_nuclei/export/svg-glyph-font-context.toml sha256:4d185c303f0262799e475ff76459118a64fdbd406dfd704d95a836b4b254985c
```

## Current evidence

- `03_infra/src/pipeline/html.rs:64` — `resolve_html_pretty`.
- `03_infra/src/pipeline/html.rs:71` — `compile_to_html_string_with_features_and_serialization_and_pretty_override`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/pipeline/html.rs`.

## Owned contract

- Own the semantic HTML compilation facade, HTML option resolution and intrinsic `html.frame` preparation/position collection. The external evaluation target remains Html; its feature gate precedes evaluation and is never inferred from output extension or serialization mode.
- Evaluate, collect document format options, delegate contextual preparation, resolve intrinsic frame islands and export through the selected `HtmlSerializationMode`. Do not paginate the whole HTML document or create a global PagedDocument/PageStore.
- Resolve pretty once as `pretty_override.unwrap_or(options.html.pretty)`. Wrappers without a decision pass None; the explicit boolean wrapper passes Some. Deliver the same resolved decision to external HTML and inline SVG frames; inline frames do not consult `format.svg`.
- Crystalline/Vanilla modes affect encoding only, not features, content, context or diagnostics. No CLI/env access belongs here.
- Traverse frames in occurrence order with effective styles and invoke the intrinsic L1 paged facade locally. Nested frames collapse into the outer island; distinct occurrences retain distinct causal identities. Layout/render failure aborts rather than dropping a frame or emitting a placeholder.
- For actually observed HTML `location.position()` reads, supply intrinsic positions to selective stabilization before terminal marker projection. `location.page()` stays none: an intrinsic Page is never a public HTML page.
- Missing ids, collisions, ambiguous topology or unmatched attempts fail closed. General query-derived Location identity outside an island remains outside this contract; do not associate it by ordinal.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.


## Reaproveitamento do índice realizado — 2026-10-05

A inspeção em `03_infra/src/pipeline/html.rs:102` revelou reconstrução do índice inicial já produzido pela realização. Consultar Module::realization_introspector segundo `00_nucleo/prompts/entities/module.md`, usando clone independente se disponível e o fallback vigente se ausente. Preservar feature/target, format options, preparação contextual e resolução de posições das ilhas html.frame. Não tratar o snapshot inicial como resultado dos contextos/posições finais. Revalidar HTML e frames por observáveis de linguagem.


## Materialização do conjunto ratificado — 2026-10-05

Preparar snapshots Page não paginados antes da descoberta, e selecionar/revalidar produtores cuja leitura efetivamente observada muda quando só as vistas Page são substituídas. Sem criar páginas físicas HTML, e sem permitir filtered-counter HTML antes proibido.


## Preparação de bindings de exportação — 2026-10-05

Medição: `00_nucleo/diagnosticos/identidade-fonte-shaped-2026-10-05/probe.txt`. Consumir helper privado resolve_export_fonts: fontes originais vinculadas, com gate de instanciação existente. Instanciação fica no consumidor exportador, na mesma fase de render/export, preservando diagnósticos/API/seleção de páginas. O nome antigo prometia bytes já instanciados e não descreve este contrato.


## Manutenção local e observação de testes — 2026-10-16

### Medição anterior à decisão

`03_infra/src/pipeline/html.rs` nas linhas 147 contém os avisos registrados em `00_nucleo/diagnosticos/clippy-locais-2026-10-16/before.json.gz`. O snapshot conserva fontes/L0, hashes, HEAD, working tree não commitado, UTC e git diff HEAD --stat. A inspeção dos consumidores e usos precedeu esta decisão; o gate Clippy selecionado deve falhar antes do código. Manutenção de representação/observação Rust, sem nova medição de intenção upstream ou paridade.

### Responsabilidade e aceitação

Mover somente o módulo #[cfg(test)] p1465_tests para o fim do consumer, após os helpers produtivos. Manter nome, corpo, atributos, imports e todos os testes; nenhum helper muda de módulo/visibilidade. Ordem de declarações não muda o pipeline.

Fluxo contínuo ADR-0127: sem contrato público, default de produto ou fase novos. L0 precede código e resselo restrito. Nenhuma supressão de lint nova. Conferir redução exata do inventário e ausência de avisos novos, gate Clippy RED→GREEN e unfulfilled_lint_expectations como erro; executar suíte workspace, build, formato, Rustdoc e linhagem sob 12 GiB, swap zero e um job. Não criar testes que apenas espelhem derive, Entry ou posicionamento de módulos. O caso de bbox exige observação real conforme especificado; os demais controles existentes mantêm fixtures e expectativas.
