# Prompt L0 — `entities/syntax_text`
Hash do Código: 1fef178c

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/syntax_text.rs`
**References:** ADR-0004, ADR-0015, ADR-0024.

## Contract

SyntaxText strongly types terminal CST text for SyntaxNode, Span and parser
offsets. Its private Arc<str> backing shares immutable text with O(1) clone.
No I/O, global state or external dependencies; use std::sync::Arc and std::fmt.
The domain interface remains stable if backing is changed by a future contract.
ADR-0015 removed EcoString from this parser type; Value::Str's EcoString is a
separate eval choice. Historical source: `lab/typst-original/crates/typst-syntax/src/text.rs`.

Preserve `pub struct SyntaxText(Arc<str>)` deriving Clone/Eq/PartialEq/Hash.
new()->Self constructs an empty shared Arc; as_str(&self)->&str exposes a slice;
len(&self)->usize counts UTF-8 bytes, never graphemes/codepoints;
is_empty(&self)->bool checks emptiness. Default delegates to new.
From<&str> uses Arc::from(s); From<String> uses Arc::from(s.as_str()).
Preserve PartialEq<str> for SyntaxText, PartialEq<SyntaxText> for str and
PartialEq<&str> for SyntaxText without allocation. Display writes the string;
Debug delegates to str's Debug.

## Acceptance

Test the public interface, never expose Arc to tests. From("hello") has text
"hello", len 5 and is_empty false. new() has empty text, len 0 and is_empty true.
Cloning "shared" preserves content equality. Display("display-me") yields
"display-me"; From(String::from("world")) yields "world". From("hi") equals
"hi" and differs from "bye". The UTF-8 fixture "olá" has length 4 bytes.
Keep colocated tests and a lineage header naming this canonical owner.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/syntax_text.md`. Historical observations are not a new parity certification.
