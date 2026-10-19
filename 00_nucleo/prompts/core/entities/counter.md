# Prompt L0 — `entities/counter` — document counter key
Hash do Código: 2e3c279e

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/counter.rs`
**References:** ADR-0107, ADR-0127.

## Evidence before namespace contract

Historical upstream introspection/counter.rs:527 distinguishes CounterKey::Str
from CounterKey::Selector. The former crystalline EcoString key merged user
counter("heading") with the automatic heading counter, a language-level
collision. Keep these namespaces independent.

## Contract

CounterKey derives Debug/Clone/PartialEq/Hash with Str(EcoString),
Selector(Selector), and Page. Counter derives Debug/Clone/PartialEq with
public key:CounterKey. Hash supports CounterRegistry maps; Selector must
preserve Clone/PartialEq/Hash requirements through its own owner.

counter("foo") yields Str("foo"); counter(heading) or its selector yields
Selector(Selector::Kind(ElementKind::Heading)); counter(page), when supported,
uses Page, never a string alias. User-string counters advance only through
update/step/get, independently of automatic Heading/Figure/Table/Footnote
walk updates. repr and diagnostics use language key representation: heading
for its selector and quoted string repr for user "foo".

## Acceptance and reopening

After stepping counter("heading"), its get is (1,) while counter(heading)
is (0,). After a heading, manual remains (1,) and automatic becomes (1,).
Build and crystalline-lint must validate. A new upstream key kind or changed
Selector key reopens this contract.

## Narrow lint exception: evidence and scope

`lab/results/construction/diagnostics/clippy-locais-2026-10-16/clippy-inventory.json`
records 79 warnings. `lab/results/construction/diagnostics/clippy-excecoes-2026-10-16/before.json.gz`
rechecks source/spans/reasons/signatures/hashes and preserves HEAD,
uncommitted tree, UTC and diffstat. Concrete functions/data were read first;
arity/type heuristics are Rust mechanics, not new parity/cognitive evidence.

Only test counter_key_str_e_selector_sao_distintos receives
#[expect(clippy::mutable_key_type, reason="CounterKey deste teste usa apenas Str e Selector::Kind imutáveis; o HashMap verifica namespaces distintos")].
It constructs only Str("heading") and Selector::Kind(Heading), with no mutable
identity payload. Keep HashMap, two inserts, len=2 and both gets. This does not
certify general Selector/Value keys or NaN equality. Different variants/keys
reopen the decision and require hash/equality stability tests.

The ADR-0127 continuous-flow exception changes lint metadata only: no executable
body, cfg, public contract, default, phase, dependency or architecture changes.
It overrides generic suppression bans only for the named item/lint and cannot
hide pending fixes. L0 first; restricted reseal; removing added attributes and
lineage header must reproduce prior rustfmt-normalized source. Measure RED;
then all-targets inventory must show exactly 46 arities and five test warnings
satisfied by 49 expectations, no new warnings, and unfulfilled_lint_expectations
as an error. The 11 arities designated for correction remain visible. Require
build/fmt, strict/reverse lineage, Tekt without new errors/warnings, existing
Counter tests/coverage,12GiB/no swap/one job. Prior full-suite evidence remains
for identical executable code; no tests merely mirroring lint attributes or
new performance comparison for metadata.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/counter.md`. Historical observations are not a new parity certification.
