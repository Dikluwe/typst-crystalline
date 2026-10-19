# Prompt L0 — `Regex` — L1 wrapper around `regex::Regex`
Hash do Código: 2dc79fe4

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/regex.rs`
**References:** ADR-0077/0017/0107/0029/0054. Value integration has its own owner.

## Contract

Regex has private pattern:EcoString and compiled:Arc<regex::Regex>.
Clone derives and shares compiled storage O(1), without recompiling.
Manual Hash/PartialEq/Eq use only pattern; manual Debug prints
Regex { pattern: "..." }. Default is empty pattern. Pure L1/no I/O;
regex dependency is authorized by ADR-0077 and crate/linter configuration.
RegexError derives thiserror::Error/Debug with Invalid(String) and exact
message `regex inválida: {0}`.

Public new(&str)->Result<Self,RegexError> validates with contextual errors;
pattern(&self)->&str returns original; is_match(&self,&str)->bool delegates to
compiled regex; captures_first(&self,&str)->Option<RegexMatch> returns the
first match or None. RegexMatch has public start:usize,end:usize,text:String,
captures:Vec<Option<String>>. Offsets are bytes; captures exclude the whole
match, preserve positional order including named groups, and distinguish
nonparticipating None from a participating empty string.

## Evidence and historical correction

Historical upstream e0e8ca4d foundations/str.rs:426-437 specifies match's
start/end/text/captures dictionary; captures_to_dict at 930-941 skips group 0
and maps missing groups to Value::None. The 2026-08-13 "ação: (x)" regex probe
returned start8/end11/text(x)/one capture on both upstream and local
HEAD 4f64e4e69, demonstrating bytes (prefix 8 bytes versus 6 codepoints).
The "ab" with a(x)?(b) probe found upstream (none,"b") versus historical
local ("","b"). The active API/prose explicitly specifies the corrected
Option captures, while the archived block still says not implemented and
labels the finding corrected. Keep the specified None behavior; the stale
execution-status contradiction needs receipt reconciliation before a closure
claim. This is a language value distinction, not harmless Rust mechanics.

## Integration and type identity

Selector::Regex and Value::Regex carry this type. Value's cast is identity for
Regex and fallible compilation for Str. Public str.match/str.position use
captures_first. The historical a51e02804 catalog measured type(PATH) as type
upstream versus function locally for seven callable types; post-atomization
catalog and 22 probes matched their structural baseline. Evidence lives in
`lab/results/construction/diagnostics/superficie-linguagem-p1140*`.
Global regex is Value::Type(Type::Regex), callable through static dispatch to
the existing constructor: type(regex)==type and repr(type(regex))=="type";
constructed values still have type_name()=="regex". Entity storage/operations
remain unchanged.

## Acceptance and deferred scope

Preserve exact test identifiers regex_new_valido_ok,regex_new_invalido_err,
regex_is_match_basico,regex_eq_via_pattern,regex_clone_preserva_semantica,
regex_hash_determinismo,value_regex_type_name,value_regex_cast_identity,
value_regex_cast_from_str,value_regex_cast_from_str_invalid,value_regex_partial_eq.
Verify valid/invalid compilation, match, pattern identity/hash, cheap semantic
clone, Value casts/type and optional-capture/byte-offset behavior.

Flags remain ADR-0054 scope-out. Historical text.font dictionary DEBT-52,
regex str.replace and Selector::Regex text-query stub were deferred in this
owner's original slice; their current status belongs to their own owners.
This entity neither reopens completed external work nor claims its closure.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/regex.md`. Historical observations are not a new parity certification.
