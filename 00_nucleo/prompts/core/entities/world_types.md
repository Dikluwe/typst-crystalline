# world_types — World domain interfaces
Hash do Código: 0495e41f

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/world_types.rs`

## Scope and historical boundary

World's domain types began as opaque blocking interfaces whose public type
identity was stable while representation remained provisional. The original
construction schedule is archived, not a perpetual instruction to restore
stubs. Source and FontBook now have dedicated canonical owners:
`00_nucleo/prompts/core/entities/source.md` and
`00_nucleo/prompts/core/entities/font_book.md`. This file does not re-own them.
The old blanket zero-external-dependency claim conflicted even with its own
thiserror/time declarations; preserve explicit authorized dependencies and L1
purity rather than using that obsolete sentence to remove domain types.

## Specified contracts

Bytes has private Vec<u8>, with new(Vec<u8>)->Self, as_slice()->&[u8],
len()->usize and is_empty()->bool. Preserve binary content and opacity.

Library has public global:Scope. new() creates an empty scope for World mocks;
with_global(scope) carries the real stdlib/predefined colors/std/text/user
elements for eval bootstrap. This supersedes Library's old unit stub.
Scopes.base:Option<&Library> is a read-only fallback: get reads global last,
get_mut never accesses base. This prevents mutation of library constants
without adding BindingKind to Binding; never flatten library into mutable
Scopes.top again.

Datetime has private date:Option<time::Date> and time:Option<time::Time>, at
least one present through new_date/new_time/new_datetime/from_parts. Preserve
time-only values and existing year/month/weekday/day/hour/minute/second
accessors. The historical ordinal probe found 2024-02-29→60, time-only→none,
2023-12-31→365 in both binaries. Public ordinal(&self)->Option<u16> delegates
to Date::ordinal or None, without changing fields/representation/equality.
`lab/history/construction/records/00_nucleo/materialization/typst-passo-1146.md:296`
records the owner's "Continue" approving the two gated contracts, superseding
the stale pending ordinal heading.

FileError derives Debug/thiserror::Error with NotFound→"file not found",
AccessDenied→"access denied",Other(String)→its string. FileResult<T> is
Result<T,FileError>. Keep these diagnostic literals.

The internal tracked warn_diagnostic channel preserves the entire
SourceDiagnostic (hints,severity,trace,span) when forwarding realization
warnings to the existing sink; deduplication stays in Sink::record.

## Current type boundaries

`01_core/src/entities/world_types.rs` declares the public unit carriers
`Routines` and `Engine`, the optional-span wrapper `Traced`, and the
tracked route chain. The evaluator aggregate in
[00_nucleo/prompts/core/entities/engine.md](engine.md) has a distinct contract.
The [original Route design](https://github.com/Dikluwe/typst-crystalline/blob/58f30f0127615f89bdf111295aa48a46e92d46c7/00_nucleo/materialization/typst-passo-90.md#L50)
records the concrete tracked chain. Treat later public methods and depth-limit
checks as separate provenance items.

The tracked `Sink` methods live in this consumer. The
[original warning-channel decision](../../../adr/adr-0043-sink-output-channel.md)
establishes caller ownership. The direct warning API, record storage and
first occurrence ordering belong to
[00_nucleo/prompts/core/entities/sink.md](sink.md).

## Acceptance and specification debt

Bytes[1,2,3] returns that slice; empty Bytes is_empty true. Check NotFound and
Other("custom") Display literals. Library::new.global has len0;
with_global containing calc returns its Value. Preserve primitive datetime/
ordinal controls, private interiors and no meaningless Default for opaque
unit stubs. Historical Source(text="hello") and opaque FontBook checks belong
to their migrated owners, not duplicated definitions here.

The archived Font(Vec<u8>) description and construction-era migration dates
are incomplete relative to the consumer's Font storage/public constructors.
The original L0 also lacks complete Routines/Traced/Route/Engine and tracked
Sink interface contracts. These are explicit pre-existing specification gaps;
consumer inspection identifies them but cannot independently authorize their
public contracts. Preserve code and existing contracts until measured L0/
approval provenance is reconciled. This editorial pass neither revokes already
ratified interfaces nor invents approval from implementation.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/world_types.md`. Historical observations are not a new parity certification.

## Methods and dates package

`Datetime::kind`, `shifted` and `since` own date arithmetic: a date gains a time only off midnight, a time wraps around the day, and only values of one kind subtract. Calendar arithmetic is pure, over days since 1970-01-01.

Approved on 2026-11-25 (“Aprovo A–D”): `00_nucleo/diagnostics/methods-dates-package-2026-11-25.pt-BR.md`.
