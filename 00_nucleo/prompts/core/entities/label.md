# Prompt L0 — `entities/label`
Hash do Código: e3e968c7

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/label.rs`
**Reference:** ADR-0015.

## Evidence before public language contract

Historical upstream `a51e02804`, foundations/label.rs:49-99, declares Label
as a type with construct(name:Str). Measurements recorded
repr(type(label)) == "type", label("x") == <x>, str(label("a b")) == "a b",
and an error for a second positional argument. The earlier crystalline
function produced content; that was a public semantic/morphological mismatch.
These observations are tied to that historical pin.

## Contract

`pub struct Label(pub String)` derives Debug/Clone/PartialEq/Eq/Hash, not Copy.
It is pure semantic identity for Content/AST references and indexes, with no
I/O, global state or external dependency. The stored name excludes markup
< > delimiters. It is distinct from ast::markup::Label<'a>, a SyntaxNode view
whose get() strips delimiters. Native String follows this entity's ADR-0015
choice; it does not prohibit EcoString in other domains.

The callable language type label(name:str)->label accepts exactly one nonempty
string, yielding Value::Label(Label(name)), equivalent to code literal <name>.
It also accepts special names unavailable to literal syntax. repr uses <name>
for a valid literal identifier and label(<repr string>) otherwise; str returns
the internal name. Content/introspection transport is separately owned.

## Acceptance

Equal "intro" labels compare equal and hash equally; "intro" and "outro"
differ. AST extraction removes markup delimiters. A HashMap<Label,u32> can
store Label("intro") with value 1 and retrieve 1 using an equal new label.
Preserve callable-type, special-name repr/str and argument-rejection behavior,
colocated tests and a lineage header pointing to this owner.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/label.md`. Historical observations are not a new parity certification.
