# L0 — Public duration and watch-line presentation
Hash do Código: b8867f4e

**Layer:** L2
**Exclusive consumer:** `02_shell/src/duration.rs`

## Evidence and boundary

`02_shell/src/duration.rs:14`, `:27` and `:45` implement `format_duration`, `WatchStatus::message` and `format_watch_line`; the public literal vector is exercised in `02_shell/tests/duration_presentation.rs:19`. The latter has its own owner, `00_nucleo/prompts/shell/tests/duration_presentation.md`. Status selection, time capture, channels, color, terminal clearing and diagnostic ordering belong to the caller, not this formatter.

The text is observable presentation. Display wrappers and arithmetic mechanics are implementation choices, provided the literal outputs and boundaries are preserved. Historical oracle/harness evidence is in `lab/results/construction/diagnostics/sanitization-shell-history.md`; it is not a global parity claim.

## Public API

```rust
pub fn format_duration(duration: std::time::Duration) -> impl std::fmt::Display;
pub enum WatchStatus {
    Compiling,
    Success(std::time::Duration),
    PartialSuccess(std::time::Duration),
    Error,
}
impl WatchStatus {
    pub fn message(&self) -> String;
}
pub fn format_watch_line(hour: u8, minute: u8, second: u8, status: &WatchStatus) -> String;
```

All inputs are supplied values. Do not read a clock, environment or timezone; sleep, measure execution, perform I/O, create global state, import L3/L4 or private L1 repr logic, or add external dependencies.

## Duration contract

Emit nonzero day, hour and minute pieces, in that order. If a day or hour exists, stop before seconds/fractions. Otherwise, after any minute piece:

- At or above five minutes, emit integer residual seconds, including zero.
- At or above one second and below five minutes, emit residual seconds with fractions rounded to two decimal places.
- Below one second, select milliseconds at or above one millisecond, microseconds at or above one microsecond, otherwise integer nanoseconds.
- Zero is `0 s`.

Use one space between each number and unit and between pieces. Units are exactly `d`, `h`, `min`, `s`, `ms`, `µs` and `ns`. Normal numeric display does not force trailing decimal zeros. Rounded seconds do not carry into a minute.

Preserve this exact input/output vector; input notation is seconds plus nanoseconds:

| Seconds | Nanoseconds | Output |
| --- | --- | --- |
| 1000000 | 0 | `11 d 13 h 46 min` |
| 86400 | 0 | `1 d` |
| 3600 | 0 | `1 h` |
| 3840 | 0 | `1 h 4 min` |
| 364 | 770000000 | `6 min 4 s` |
| 264 | 776000000 | `4 min 24.78 s` |
| 3 | 0 | `3 s` |
| 2 | 849200000 | `2.85 s` |
| 1 | 0 | `1 s` |
| 0 | 999000000 | `999 ms` |
| 0 | 1000000 | `1 ms` |
| 0 | 294816000 | `294.82 ms` |
| 0 | 999000 | `999 µs` |
| 0 | 734000 | `734 µs` |
| 0 | 1000 | `1 µs` |
| 0 | 999 | `999 ns` |
| 0 | 1 | `1 ns` |
| 0 | 0 | `0 s` |
| 299 | 999000000 | `4 min 60 s` |
| 300 | 0 | `5 min 0 s` |
| 86340 | 0 | `23 h 59 min` |
| 300 | 6000000 | `5 min 0 s` |

These are contract inputs, not execution-duration metrics. Do not derive them from wall time, scheduling or the test's own runtime.

## Status and envelope literals

`Compiling` is `compiling ...`. `Success(d)` is `compiled successfully in {format_duration(d)}`. `PartialSuccess(d)` is `compiled with warnings in {format_duration(d)}`. `Error` is `compiled with errors`, with no elapsed time. Typed warnings determine success versus partial success; never infer it from diagnostic text.

`format_watch_line` returns `[{hour:02}:{minute:02}:{second:02}] {body}`, with no newline. The caller supplies valid time components. With 03:04:05 and 2.8492 seconds, preserve:

```text
[03:04:05] compiling ...
[03:04:05] compiled successfully in 2.85 s
[03:04:05] compiled with warnings in 2.85 s
[03:04:05] compiled with errors
```

The envelope is HH:MM:SS only, without date, offset or fraction.

## Acceptance

Check the literal vector, unit transitions, inclusive boundaries, omissions, rounding, repeatability and reversed input order. Boundary controls must distinguish 299.999 seconds from 300 seconds and preserve 300.006 seconds. Reject changes to units, meaningful thresholds and clock-derived inputs; do not credit an observationally equivalent mutation. Claim equivalence only for the specified strings and boundaries.
