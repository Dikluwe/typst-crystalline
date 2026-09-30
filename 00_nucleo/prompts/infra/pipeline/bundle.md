# L0 — infra/pipeline/bundle
Hash do Código: f0d9d76b

**Layer:** L3
**Exclusive consumer:** `03_infra/src/pipeline/bundle.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
- 00_nucleo/prompts/_nuclei/export/format-option-precedence.toml sha256:68c5c10f124c2c098879318df16c59f6c1477e9172cfa334b603f45ec698ae11
- 00_nucleo/prompts/_nuclei/export/svg-destination-context.toml sha256:13cad5ab1322bad4c569eec2aaf544452530cb972c3421130d0b9cb127170ef1
- 00_nucleo/prompts/_nuclei/export/svg-glyph-font-context.toml sha256:4d185c303f0262799e475ff76459118a64fdbd406dfd704d95a836b4b254985c
```

## Current evidence

- `03_infra/src/pipeline/bundle.rs:13` — `introspect_bundle_with_runtime_for_pipeline`.
- `03_infra/src/pipeline/bundle.rs:55` — `BundleFile`.
- `03_infra/src/pipeline/bundle.rs:63` — `BundleCompileOptions`.
- `03_infra/src/pipeline/bundle.rs:77` — `bundle_error`.
- `03_infra/src/pipeline/bundle.rs:82` — `compile_to_bundle_files`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/pipeline/bundle.rs`.

## Owned contract

- Own `BundleFile`, `BundleCompileOptions`, ordered entry collection, path validation/collision checks, global snapshot composition and per-format rendering. Return the complete collection or diagnostics, never partial success; filesystem publication belongs to wiring.
- Preserve all option fields and explicit absence (`None`) until the relevant format owner resolves document precedence. Keep PDF stream/serialization modes independent and `is_html_document` based on semantic format, not an asset extension.
- Evaluate the root as Bundle only with its explicit feature. Accept documents, assets, tags and whitespace separators at top level; reject loose content, invalid/duplicate normalized paths, unknown formats and multi-page image documents before publication. HTML additionally requires its own feature.
- Preserve ordered bodies and inherited styles. Use one global introspector and causal Location seed per body; delegate preparation of the complete set to `00_nucleo/prompts/infra/pipeline/context_stabilization.md`. Never export bodies from different validated generations or fall back to isolated snapshots.
- Keep heading/state/label history global and Page boundaries local per body. Explicit HTML Page updates remain observable. Do not fabricate metadata padding, derive seeds from element count, hardcode page 1 or repair rendered numbering strings.
- Font resource collection and acquisition follow the resource child owner, without an unconditional catalog request for an empty set.
- Paged bodies use seeded layout; HTML remains semantic except its intrinsic `html.frame` islands. Bind logical page numbers from the final global PageStore with an offset advanced only by paged PDF/SVG/PNG bodies; HTML and assets consume no pages. Misalignment is an error, not a physical-index fallback.
- For a paged body with nonzero offset, project only marginal `logical_numbers` and `visible_numberings` into the layout copy's local indices. Keep total, reference numberings, spans and supplements global and leave the original bundle introspector untouched.
- Treat `SpaceFromNewline` as a separator only outside top-level Document/Asset entries. Preserve its origin inside bodies, preparation and HTML semantic projection until the L1 normalization owner acts.
- Cross-document `.final()` generalization is not authorized by the Page-boundary contract. No new public API, target/default or partial-write policy is introduced here.

The resource owner's private matching-axis predicate governs the early instancer dependency check. Unknown-axis-only requests must not produce a missing-tool diagnostic; retain dependency errors for actual font axes and preserve export dispatch/options.

Para PDF dentro do bundle, delegar o gate e diagnóstico do instanciador ao
helper privado de `infra/pipeline/resources.md`. A medição causal nesse L0
mostra a frase inglesa divergente; preservar a mensagem portuguesa já usada
pelo owner de recursos e pelo PDF isolado. Não alterar o despacho PDF existente.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## PDF interno conserva validações do target — 2026-09-30

### Medição anterior à decisão

`00_nucleo/diagnosticos/auditoria-composicao-construcao-2026-09-29/probe.json` mostra um PDF/UA-1 sem título rejeitado como PDF isolado e aceito dentro do bundle, que publica `one.pdf` com `pdfuaid:part=1` e sem título. A fonte `03_infra/src/pipeline/bundle.rs:463-513` resolve standards e chama diretamente o exporter; `03_infra/src/pipeline/pdf.rs:404-426` executa primeiro as validações PDF-only. A hipótese de omissão seria refutada se a rota bundle chamasse os mesmos gates antes da exportação; não chama.

### Contrato deste owner

Para cada entrada PDF, após resolver páginas, tags e standards, delegar ao helper privado do owner `00_nucleo/prompts/infra/pipeline/pdf.md` as validações pré-exportação de título UA-1, alt UA-1 e caminhos de anexos duplicados, antes de resolver fontes e emitir bytes. Propagar o diagnóstico sem prefixo ou conversão e abortar a coleção completa, preservando a política de publicação atômica. Não replicar as regras PDF, não alterar HTML/SVG/PNG, defaults ou APIs públicas. A rota bundle pode conservar o seu despacho atual de fontes; a equivalência desse despacho exige medição própria.

### Aceitação

O mesmo body PDF/UA-1 inválido por título falha no bundle e no PDF isolado com mensagem e hint iguais; com título válido, o bundle publica o PDF. A ausência de standard UA-1 não cria obrigação de título. Gates de alt/anexos seguem o owner PDF. Fluxo contínuo ADR-0127: esta é a correção de um invariante PDF já vigente, sem contrato público ou fase nova; verificar RED→GREEN.

### Transporte dos metadados por documento — medição complementar

O primeiro GREEN do teste integrado falhou no controle com título: `03_infra/src/pipeline/bundle.rs:156-194` descarta `title/author/keywords` do `Content::Document`, e `render_bundle_document` não atribui `document.document_info`; o PDF isolado atribui `module.document_info()` em `03_infra/src/pipeline/paged.rs:463`. O construtor já valida e preserva `title: Content`, `author` e `keywords` em `01_core/src/compiler/stdlib/structural/document.rs:25-102` e no respectivo L0. A sonda `00_nucleo/diagnosticos/correcao-bundle-pdf-validacoes-2026-09-30/green.log` refuta a hipótese de que bastava delegar a validação: o bundle rejeita também o controle com título enquanto os metadados não forem transportados.

Projetar os metadados de cada `Content::Document` no `DocumentInfo` do seu próprio `PagedDocument` antes da validação/exportação. `title: Some(Content)` produz `Some` do seu texto plano, inclusive quando o conteúdo é vazio; ausência produz `None`. `author` e `keywords` preservam a ordem e são unidos por `, ` como os arrays de `set document` do percurso isolado. A data permanece no carrier existente, sem novo campo em `DocumentInfo` nesta correção. Não propagar o último título de um documento a outro e não inferir título pelo nome do arquivo. O `#set document` de escopo global não substitui os metadados explícitos por entrada nesta fatia.
