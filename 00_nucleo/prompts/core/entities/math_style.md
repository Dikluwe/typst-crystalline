# L0 — Mathematical style glyph mapping
Hash do Código: 838405d6

**Layer:** L1, pure domain. **Exclusive consumer:** `01_core/src/entities/math_style.rs`.

## Evidence and responsibility

`lab/results/construction/diagnostics/diagnostico-math-style-passo-311a.md` chose existing Content::MathStyled plus on-demand Unicode codepoints/compact exception table for twelve public math-style functions bb/bold/cal/frak/italic/mono/sans/scr/script/serif/sscript/upright. Historical upstream uses codex::styling::to_style/MathStyle::select;that external crate is not authorized here. This module owns MathStyleKind and pure mapping,not constructor/style application/layout. The old≈120LOC estimate and historical ADR references are chronology,not implementation-size/dependency authority.

Historical byte/codepoint measurements in the original mapping,Greek,Roundhand and symbol rounds,with unit guards math_style.rs:248–420,support mappings below. They do not certify font-byte identity. Latin default xversus𝑥/Greekαβversus𝛼𝛽 established separate causes in the cited diagnostics;later symbol probes used actual pdftotext/ToUnicode from both PDFs,not glyph appearance.

Roundhand=implicitly-bold Script was explicitly refuted:codex to_roundhand is script+VS2,not forced bold. Measured `$scr(A) cal(A) scr(L) cal(B) scr(B)$` emits `1D49C FE01 1D49C FE00 2112 FE01 212C FE00 212C FE01` in both. The later Roundhand/Greek-symbol rules below supersede only conflicting historical notes/table wildcard shorthand;all other limits remain.

## Public data and pure interfaces

```rust
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum MathStyleKind {
    Plain,
    SansSerif,
    Chancery,
    Roundhand,
    Fraktur,
    Monospace,
    DoubleStruck,
    Script,
    SScript,
    Display,
    Inline,
}
pub fn map_glyph(c: char, kind: MathStyleKind, bold: bool, italic: bool) -> char;
```

Seven glyph families select codepoints;bold/italic are orthogonal Content::MathStyled flags,not enum variants. Plain is default serif. Chancery is cal,Roundhand scr,DoubleStruck bb. Four size families Script/SScript/Display/Inline never change glyph or add VS. Their original delegated MathLayouter size factors are0.7/0.5/1.0/1.0;size computation remains separately owned math layout,not this char mapper.

map_glyph returns one char without allocation,I/O,state or cache;same inputs→same output. For size variants return original immediately. For covered Latin choose BMP exception then Mathematical Alphanumeric base+uppercase/lowercase offset. Digits use separate kind/bold bases,no independent italic digits. Unsupported characters/symbols/spaces pass through rather than panic;Greek Plain and seven symbol additions are explicit exceptions to old ASCII-only scope. Plain without flags is no-op. Callers may use this pure mapping on mathematical hot paths without requiring a particular table optimization.

map_glyph_vs(c,kind)->Option<char> selects VS1(U+FE00)for Chancery/cal,VS2(U+FE01)for Roundhand/scr,only for Latin letters,not digits/nonLatin. map_glyph normalizes Roundhand→Chancery for codepoint while retaining explicit bold. Bold Chancery/Roundhand uses bold-script U+1D4D0,not nonbold BMP letterlike exceptions. This later measured rule narrows the historical BMP-table `(Chancery,*,*)` shorthand for bold cases;it is not a new blanket wildcard policy.

## Codepoint bases and exceptions

The Mathematical Alphanumeric Symbols block U+1D400–U+1D7FF contains thirteen52-letter subblocks(uppercase26/lowercase26). Preserve this complete family table:

| Family bit-state | Base upper | Meaning |
|---|---|---|
| `(Plain, false, false)` | (unmapped; pass through) | Plain math text |
| `(Plain, true, false)` | U+1D400 | Bold |
| `(Plain, false, true)` | U+1D434 | Italic (exception `h` U+210E) |
| `(Plain, true, true)` | U+1D468 | Bold Italic |
| `(Chancery, false, false)` | U+1D49C | Script (9 exceptions) |
| `(Chancery, true, false)` | U+1D4D0 | Bold Script |
| `(Fraktur, false, false)` | U+1D504 | Fraktur (5 exceptions) |
| `(DoubleStruck, false, false)` | U+1D538 | DoubleStruck (7 exceptions) |
| `(Fraktur, true, false)` | U+1D56C | Bold Fraktur |
| `(SansSerif, false, false)` | U+1D5A0 | Sans-Serif |
| `(SansSerif, true, false)` | U+1D5D4 | Sans-Serif Bold |
| `(SansSerif, false, true)` | U+1D608 | Sans-Serif Italic |
| `(SansSerif, true, true)` | U+1D63C | Sans-Serif Bold Italic |
| `(Monospace, false, false)` | U+1D670 | Monospace |
| `(Roundhand, true, false)` | U+1D4D0 | Roundhand with explicit bold uses bold-script |

BMP exception table records original supported letterlike forms. `*` denotes flag wildcard in the historical table;later explicit bold-script rule above prevails for Chancery/Roundhand,while Fraktur/DoubleStruck exceptions apply only without bold. Do not normalize these Unicode forms by appearance:

| Variant + flags | Char | Codepoint exception |
|---|---|---|
| `(Plain, false, true)` Italic `h` | 'h' | U+210E ℎ |
| `(Chancery, *, *)` Script `B` | 'B' | U+212C ℬ |
| `(Chancery, *, *)` Script `E` | 'E' | U+2130 ℰ |
| `(Chancery, *, *)` Script `F` | 'F' | U+2131 ℱ |
| `(Chancery, *, *)` Script `H` | 'H' | U+210B ℋ |
| `(Chancery, *, *)` Script `I` | 'I' | U+2110 ℐ |
| `(Chancery, *, *)` Script `L` | 'L' | U+2112 ℒ |
| `(Chancery, *, *)` Script `M` | 'M' | U+2133 ℳ |
| `(Chancery, *, *)` Script `R` | 'R' | U+211B ℛ |
| `(Chancery, *, *)` Script `e` | 'e' | U+212F ℯ |
| `(Chancery, *, *)` Script `g` | 'g' | U+210A ℊ |
| `(Chancery, *, *)` Script `o` | 'o' | U+2134 ℴ |
| `(Fraktur, false, *)` Fraktur `C` | 'C' | U+212D ℭ |
| `(Fraktur, false, *)` Fraktur `H` | 'H' | U+210C ℌ |
| `(Fraktur, false, *)` Fraktur `I` | 'I' | U+2111 ℑ |
| `(Fraktur, false, *)` Fraktur `R` | 'R' | U+211C ℜ |
| `(Fraktur, false, *)` Fraktur `Z` | 'Z' | U+2128 ℨ |
| `(DoubleStruck, false, *)` DS `C` | 'C' | U+2102 ℂ |
| `(DoubleStruck, false, *)` DS `H` | 'H' | U+210D ℍ |
| `(DoubleStruck, false, *)` DS `N` | 'N' | U+2115 ℕ |
| `(DoubleStruck, false, *)` DS `P` | 'P' | U+2119 ℙ |
| `(DoubleStruck, false, *)` DS `Q` | 'Q' | U+211A ℚ |
| `(DoubleStruck, false, *)` DS `R` | 'R' | U+211D ℝ |
| `(DoubleStruck, false, *)` DS `Z` | 'Z' | U+2124 ℤ |

Digit bases,with other cases unchanged(cal/frak have no separate digit variants):

| Family bit-state | Base `'0'` |
|---|---|
| `(Plain, true, _)` Bold | U+1D7CE |
| `(DoubleStruck, _, _)` | U+1D7D8 |
| `(SansSerif, false, _)` | U+1D7E2 |
| `(SansSerif, true, _)` | U+1D7EC |
| `(Monospace, _, _)` | U+1D7F6 |

## Plain Greek, default italic and measured symbols

is_math_italic_default starts with ASCII alphabetic and lowercaseα..ω,then includes the seven explicitly measured characters below. Uppercase Greek is upright by default,as `$Gamma Delta Omega alpha$`→ΓΔΩ𝛼;only explicit modifiers change it. Digits never default italic. Bold composes with default italic,so bold(x)→U+1D499,not bold-upright.

For Plain lowercaseα..ω,use contiguous offset c−α,including finalς/σ:italic baseU+1D6FC,boldU+1D6C2,bold-italicU+1D736. For uppercaseΑ..Ω,use c−Α with U+03A2 gap matching mathϴ slot:italicU+1D6E2,boldU+1D6A8,bold-italicU+1D71C. Greek under other kinds(Sans/Fraktur/etc.)remains pass-through/uncovered.

Existing application boundaries in `00_nucleo/prompts/core/compiler/math/layout/mod.md` use actual codepoint mapping for one-letter MathIdent is_var instead of merely font italic;one-character MathText applies predicate including resolved alpha;apply_math_style defaults italic to is_math_italic_default so bold(x)composes. Entity does not own those consumers or add rendering state.

Symbol-measurement table(literal→historical oracle→prior local)refutes the old symbol pass-through scope only for these seven Plain-italic forms:

| char | vanilla | Crystalline before |
|---|---|---|
| ϵ U+03F5 | U+1D716 (𝜖) | U+03F5 |
| ϑ U+03D1 | U+1D717 (𝜗) | U+03D1 |
| ϖ U+03D6 | U+1D718 (𝜘) | U+03D6 |
| ϕ U+03D5 | U+1D719 (𝜙) | U+03D5 |
| ϱ U+03F1 | U+1D71A (𝜚) | U+03F1 |
| ϰ U+03F0 | U+1D71B (𝜅) | U+03F0 |
| ∂ U+2202 | U+1D715 (𝜕) | U+2202 |

Include these seven in default-italic predicate and map individually in greek_plain Plain+italic;contiguous offsets do not cover them. Do not change existing bold/bold-italic symbol branches;their prior scope remains. Verified pass-through controls stay untouched:ϐU+03D0,ϜU+03DC,ϝU+03DD,ϴU+03F4,∇U+2207. Canonical source group is codex styling.rs lowercaseα–ω+∂+ϵϑϰϕϱϖ. Greek/nonPlain,Arabic Initial/Tailed/Looped/Stretched and other unmeasured combinations remain out of scope. No perfect-hash/FSA/table optimization or result cache is authorized/needed by this contract.

## Required mapping controls

Preserve exact tests:Plain boldA→U+1D400𝐀,Plain italica→U+1D44E𝑎,SansSerifx→U+1D5D1𝗑,Frakturk→U+1D528𝔨,Monospace3→U+1D7F9𝟹;italic h→U+210Eℎ,DoubleStruckC→U+2102ℂ,ChanceryH→U+210Bℋ,FrakturR→U+211Cℜ. Plus under Fraktur and space under DoubleStruck unchanged. Plain italicα→U+1D6FC𝛼,∇unchanged. Script x unchanged,SScript boldA unchanged,Plain unflaggedx unchanged. SansSerif boldA→U+1D5D4𝗔,bold-italicx→U+1D66D𝙭. Preserve Copy/Clone/Eq/PartialEq/Hash,reflexive Plain equality/distinct variants,purity/repeatability and all mapping tables.

## Exhaustive enum decisions and verification

math_style.rs:128,203,214,248 and tekt-enums-2026-10-16/before.json.gz preserve HEAD06848cdcb5450f2c555626bf116bc7acd82bd82b/tree/UTC/source/L0/config/status/exact diffstat/hash-checked prior owner. Previous SARIF eleven warnings/one V16 note across seven owners,including latent fallbacks,measures Rust enum evolution rather than language defect/upstream intent/AI gain.

Only nominal patterns change:map_glyph_vs None names Plain/SansSerif/Fraktur/Monospace/DoubleStruck/Script/SScript/Display/Inline. letter_base final None names Roundhand/Display/Inline,retaining explicit Script/SScript and other flags/arms;private Roundhand helper stays None because map_glyph normalizes first. digit_base None covers(Plain,false)and(Chancery/Roundhand/Fraktur/Script/SScript/Display/Inline,_). bmp_exception final None names all eleven first-axis enum variants,retaining open flags/char `_` and exception precedence;char is not artificially enumerated. greek_plain's two char/bool fallbacks remain. Add corrected three Unicode examples and Display/Inline/Script/SScript unchanged glyph/VS controls,no pin/new-parity/variant/default/grammar change.

Keep bodies/order/guards/public enum-fields-traits-signatures/defaults/dependencies/compatibility/phases,no moved owner logic/helper hiding catch-all/suppression. New enum variants must require compiler review. Precode local behavior/V16 scoped RED;disposable workspace sentinel variants independently in Color,ColorSpace,MathStyleKind,FuncRepr,StateUpdate with offline locked typst-core check/one job/shared artifact cache/full source-hash-argv-UTC-diagnostics. Each21selected matches shows no E0004 there before,required E0004 after regardless of other copied-workspace errors. Real no-sentinel build/tests approve code;negative probe is no parity proof,no productive edits during commands.

Scoped direct/reverse lineage only,no global repair. Full build/workspace suite/strict all-target Clippy warnings+expectations/fmt/Rustdoc−Dwarnings/fourHTMLindexes/V1-V5-V15-V26/reverse/SARIF no new warning/diffcheck,12GiB/swap0/jobs1/sequential,frozen provenance,unchanged tolerance/fixture. External V16 including latent T027/T158/T160 stays outside this batch.

On an explicitly authorized upstream resync,inspect pinned math/style.rs and codex styling changes;update necessary enum/base/exception L0 before code and lineage,record any new parity class under applicable ADRs. Unicode additions to U+1D400–U+1D7FF require base review if they occur. This is synchronization policy,not permission to move the repository's pinned oracle unilaterally.

Archive: `lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/math_style.md`.
