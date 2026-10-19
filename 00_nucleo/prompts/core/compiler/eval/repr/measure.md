# L0 — compiler/eval/repr/measure
Hash do Código: 940ecf77

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/decimal-rounding.toml sha256:dcf8dc8e474613bab33bbf5dff39124b8711f2facb85b731e5d63363a347e6d3

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/eval/repr/measure.rs`. At HEAD `6b147bd213e07e1dc896a8e798b84bf4d8dbbdcb` with a clean tree, `01_core/src/compiler/eval/repr.rs:202-329` contains the separated behavior. SHA-256, time and state are recorded in `lab/results/construction/diagnostics/atomizacao-repr-2026-09-30-before.json.gz`. The purpose is to make algorithms discoverable by behavior, with no presumed performance gains.

## Rounded measure evidence

At HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree,
repr/measure.rs:72–78 always multiplies by a decimal power before rounding.
Measured repr(1e308em), large degrees and percentages become infinite; six
eval(repr(value), mode:"code") probes lose the value, while historical oracle
pin `9f2b6e8715237cb086899a42873660fe744622e8` preserves type/value. Snapshots,
UTC/diffstat/hashes/prior probes are in
`lab/results/construction/diagnostics/representacao-medidas-ci-2026-10-13/before.json.gz`
and evidence.json.gz's probe-before.json. Upstream typst-utils/src/round.rs:26–60
guards filled mantissas, extreme precision and negative scaling;
foundations/repr.rs:95–130 uses it for unit/component representation.

1e307pt lengths are not bilateral evidence for this correction: upstream
layout/abs.rs:32,265–273 overflows by multiplying 127 during construction;
division by 1pt is already nonfinite in the oracle but finite in Crystalline
before repr. That construction difference remains outside scope. The 0pt form
of float.nan * 1em likewise does not isolate formatter policy.

## Exclusive formatting and rounding contract

Format Duration, Relative, math.lr size, Length, Ratio and Angle with rounding/
units. Preserve sign, explicit parts, zeros and current nonfinite forms. Detailed
historical parity evidence remains in `00_nucleo/prompts/core/compiler/eval/repr.md`;
extraction changes no output/public signature/phase and permits no I/O or mutable
global state.

This ADR-0127 internal correction adds no signature/default/phase. Apply the
pinned shared decimal claims while preserving this formatter's policy: input
NaN/infinity pass through rounding unchanged; real result overflow remains
infinity in its existing representation. Internal precision remains i32 and
limits are handled before negation, including i32::MIN. Productive callers keep
two decimals with units and three for components. Add no I/O/dependency/public
API or shared function with calc.

## Acceptance

Preserve language morphology tests, build and valid owner/hash lint. Large
finite measures remain finite and preserve value after reevaluation; confirm
all six oracle RED→GREEN probes and retain the 2^53 control. Cover components,
extreme i32 precision, scale -308, signed zero, ties, nonfinite forms and true
overflow. Rust bytes/algorithm identity is not language acceptance.
