# L0 — stdlib/foundations/datetime
Hash do Código: 80a7510a

**Baseline**: vanilla ratificado `a51e02804`

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/foundations/datetime.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/foundations/datetime.rs:19` (type NativeResult = SourceResult<Value>;); `01_core/src/compiler/stdlib/foundations/datetime.rs:21` (pub fn datetime_type_fieldfield: &str) -> Option<Value> {); `01_core/src/compiler/stdlib/foundations/datetime.rs:37` (fn errorargs: &Args, message: impl Into<String>) -> Vec<SourceDiagnostic> {) identifies the current owner and its local entrypoints. This is implementation inspection, not a new bilateral parity measurement or an inference of upstream intent. The historical decision evidence and its original snapshot qualifications are retained in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own datetime_type_field's closed static surface: today, display, year, month, weekday, day, hour, minute, second and ordinal. The constructor remains with `00_nucleo/prompts/core/compiler/stdlib/foundations/cast.md`; do not implement an unmeasured instance-field surface.

Each accessor takes only self:Datetime and returns Int or None when that component is absent. Delegate calendar rules to Datetime: weekday Monday=1 through Sunday=7; ordinal 1..365/366. display(self,pattern:auto) defaults to date [year]-[month]-[day], time [hour]:[minute]:[second], or both separated by one space. Custom patterns use the existing time description language; preserve description errors and `failed to format datetime (insufficient information)`.

today(offset:auto) uses injected World::today(None); integer offsets become hours, Duration preserves sub-hour precision. Reject invalid types, positional offset, unknown named/excess arguments; absence from World is `unable to get the current date`. No L1 clock or filesystem. Preserve the current World ABI without treating an old pending gate as a new signature change.

## Acceptance

Preserve the specified language semantics, syntax, morphology, diagnostics, spans, defaults and public identities. Do not change signatures, compatibility, feature defaults or phase ownership through this documentary revision. Shared pinned claims remain authoritative; this prompt owns only its exclusive consumer. Architectural decisions follow ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.
