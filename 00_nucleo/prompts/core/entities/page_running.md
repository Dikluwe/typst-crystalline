# L0 — Page marginal domain
Hash do Código: e25bc00b

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/page_running.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/page-running.toml sha256:9f5dc1dba49ac843612415dfe6018b40308686a4fef442d153bdbd06834466d2

## Evidence before specification

Historical upstream `a51e02804` evidence is
`typst-library/src/layout/page.rs:315-450` and
`typst-layout/src/pages/run.rs:141-233`, not a current-pin parity probe.

## Owned contract

`PageMarginal` preserves Auto, None and explicit Content, including empty
Content; default is Auto. Auto permits automatic numbering only when numbering
exists and vertical alignment selects this marginal. None suppresses it;
explicit Content takes precedence over automatic numbering in that marginal.
An omitted delta retains the previous value, distinct from all three states.
`PageNumberAlign` carries horizontal alignment and vertical Top/Bottom via
`PageNumberVAlign`; default is center plus bottom. Horizon is not a valid
vertical language value; casting/diagnostics belong to evaluation.
`PageMarginalOffset` is `Rel<Length>`; both offsets default to 30% with zero
absolute component. Header ascent uses the physical top margin and footer
descent bottom; add the absolute component without an empirical constant.
These types remain pure L1 and do not execute callbacks or inspect targets.

## Ownership and delegation

The pinned nucleus carries shared page-running invariants. Composition and rich
Content layout belong to `00_nucleo/prompts/core/compiler/layout/page_running.md`.
PageConfig belongs to `00_nucleo/prompts/core/entities/layout_types.md`, Content
transport to `00_nucleo/prompts/core/entities/content.md`, page-run data to
`00_nucleo/prompts/core/entities/elements/page_run.md`, numbering to
`00_nucleo/prompts/core/entities/numbering.md`, and supplement to
`00_nucleo/prompts/core/entities/page_supplement.md`.
No public page constructor is authorized here. Verify default alignment,
offsets and marginal distinctions. Historical composition sequences and
cross-owner obligations are retained in the diagnostic, not redefined here.

## Equivalent derived Default

### Evidence before specification

The former manual Default at `01_core/src/entities/page_running.rs:41`
was identified by Clippy. At HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b` with uncommitted working tree,
`lab/results/construction/diagnostics/expressoes-defaults-ci-2026-10-13/before.json.gz`
records sources/owners/spans/UTC/status/diffstat. The non-generic type gains
no parameter bounds and keeps the same value. This is implementation evidence,
not new parity or performance measurement.

### Contract and acceptance

Use derive(Default) with #[default] on PageMarginal::Auto, replacing only
that manual implementation. Keep PageNumberAlign::default manual and
center/bottom; do not change offsets, presence or marginals. Old manual
examples specify the same required value. No public
signature/field/variant, product default or phase change. Preserve all tests
and expectations; validate complete workspace targets, core suite, test
inventory, build, Rustdoc, Clippy, and lineage under continuous ADR-0127 flow.
