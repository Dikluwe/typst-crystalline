# L0 — Query parsing, summaries and located results
Hash do Código: b0a8a829

**Layer:** L3
**Exclusive consumer:** `03_infra/src/query_helpers.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24
```

## Current evidence

- `03_infra/src/query_helpers.rs:110` — `parse_selector`.
- `03_infra/src/query_helpers.rs:154` — `summarize_query`.
- `03_infra/src/query_helpers.rs:225` — approximate content counts.
- `03_infra/src/query_helpers.rs:274` and `:357` — text/variant walkers.
- `03_infra/src/query_helpers.rs:455` and `:475` — summary and located-result entrypoints.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Own selector parsing, the existing approximate summary projection and transport of located query values. Do not implement JSON/YAML/repr formatting, field realization, CLI --field/--one selection or a new L1 selector model. Presentation belongs to `00_nucleo/prompts/shell/cli.md`; command composition belongs to `00_nucleo/prompts/wiring/query.md`.

`parse_selector` trims input and recognizes simple ElementKind names, nonempty angle-bracket labels and the `math.equation` alias. Preserve the internal standalone equation spelling. Quoted-text/regex selectors produce the distinct `text is not locatable` error. Empty/unknown/complex selectors preserve their current QueryError messages; dots/parentheses are rejected after the alias check. Diagnostic literals containing a historical case identifier remain exact output, not chronology authorizing a contract.

`query_to_summary(world: &dyn World, source: &Source, selector: &str)` returns `Result<QuerySummary, QueryError>`. Parse before eval; reuse the pipeline eval adapter, obtain module content, and introspect the pre-show tree when supplied. Preserve EvalFailed, NoContent, InvalidSelector and WorldError. This legacy entrypoint discards evaluation warnings. `summarize_query` also takes the raw selector string and retains it in the result. QuerySummary keeps selector, SelectorKind, count, kind_name, label_found and metadata_values without Serialize. Label summaries count zero/one and retain a matched label. Kind summaries count introspector matches except the existing approximate document-element projection. Metadata uses its current plain-text/debug-string projection, not generic language serialization.

`count_element_in_content` retains its distinct List/Enum group walkers, boolean paragraph-text approximation and Link/Raw/Quote/Footnote variant counts; other kinds return zero. Do not promote approximate matches into located values. The list/enum walkers count all-item Sequences and recurse as currently implemented; the singleton discrepancy is recorded in `00_nucleo/diagnosticos/sanitization-infra-wiring-history.md`. The text/variant walkers descend through their explicit content containers, including PageRun.body without interpreting PageConfig. OutlineEntry text consults element; variant counting descends element/fill after the predicate precheck. A matching predicate returns once without counting descendants again.

Mathematical carriers are terminal in this approximate projection, including limits/class/style wrappers, root/fraction/sequence, lr/mid/stretch, primes, accents and over/under decorations. MathUnderline, MathVec and Flush do not invent text/located occurrences or descend into math internals. Space and SpaceFromNewline remain neutral, without guessing newline origin or removing CJK spacing. Public structural traversal/repr/layout semantics remain with their L1 owners; these approximate walkers do not redefine them.

Preserve the current public located-result signature:

```rust
pub fn query_elements(
    world: &dyn World,
    source: &Source,
    selector: &str,
) -> (Result<Vec<Value>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>);
```

Invalid selectors become detached SourceDiagnostics without panic. Evaluation errors and warnings remain separate; a module without content returns an empty successful vector. Build original/pre-show and realized introspectors, prefer the realized index only when it has more matches, otherwise retain the original index. Preserve selected location order and cardinality for every recognized locatable kind and labels, without a heading-only filter.

For an entry with captured fields (`Some`), clone the complete IntrospectedContent and transport its exact Location as LocatedContent. Preserve the captured label; never rebuild a captured-field entry from a selector/registry, discard its snapshot or replace Some by None. For an entry with no captured fields, preserve the legacy labeled-content projection by constructing a separate None entry when required, without modifying the original entry. A location without an element fails with `query result has no locatable element` instead of silently dropping it. No output serialization or default-field fabrication belongs to L3.

## Verification boundary

Existing tests cover selectors, summaries, match ordering, label transport and neutral math/newline-space projections. Located values preserve public fields/labels through the L2 serializer. This prompt records the current consumer; it does not resolve archived approval/provenance gaps or certify fresh upstream parity.
