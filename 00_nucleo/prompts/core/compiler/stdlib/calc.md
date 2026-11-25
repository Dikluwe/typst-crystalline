# L0 — stdlib/calc
Hash do Código: 72813eae

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/calc.rs`

## Contract

Own `make_calc_module`: the module's static bindings, native identities, constants
and child composition, plus the historical tests in this file. `pi`, `tau`, `e` and
`inf` are Float constants. Preserve every binding. calc.abs's intrinsic name is
`abs` through import, alias and With. `log10`, `deg` and `rad` are deliberate
extensions; removing one requires a compatibility decision.

Each child under `00_nucleo/prompts/core/compiler/stdlib/calc/` exclusively owns its
functions' types, values, domains, diagnostics and limits: abs, power, round,
extrema, trig, hyperbolic, logarithms, integer, combinatorics, norm and erf.
`numeric` owns their shared private projections. This owner restates none of it.
ADR-0101 governs numeric results. No general calc parity is claimed.

## Acceptance

Preserve the module surface, identities and existing test inventory. A test that
fixed a policy replaced by a child owner changes only with that owner. Build,
core suite, Clippy and lineage must pass.

## Evidence

Single-authority decision approved on 2026-11-24:
`00_nucleo/diagnostics/calc-numeric-package-2026-11-24.pt-BR.md`. The previous text,
with its numeric table and earlier records, is
`00_nucleo/prompts/core/compiler/stdlib/calc.md` at commit `f2c737125`.

## Fallible Source and file-registration integration

Human approval on 2026-10-22 is retained in
`lab/results/panic-elimination-2026-10-22/source-contract-approval.json`;
its review SHA-256 is `2c879e8afa66721a8952dc8b9e7cf3c92dce58bf91aae5e28da5d603f5aa4d42`.
Source::{new,new_with_parser,detached,detached_with_parser} now return the existing
SourceResult<Source>; SystemWorld::register_file returns FileResult<FileId>.
Preserve this unit's successful content/id/span/order/diagnostic behavior while
propagating construction/registration failures through its existing error channel.
A tuple-returning entry returns its error in the existing result component and
retains already accumulated warnings. A synthetic source slot stores a failed
construction as FileResult rather than inventing a Source. Test-only fixture
helpers must explicitly require their valid construction/registration result;
all previous substantive assertions remain. No productive unwrap/expect, panic
catch, default Source, identity reuse or silent error is permitted for this migration.
This clause supersedes only the old infallible return/panic expectations for the
five approved APIs; it does not authorize unrelated signatures, fields, phases
or broadening/loss of existing diagnostics or negative controls.
