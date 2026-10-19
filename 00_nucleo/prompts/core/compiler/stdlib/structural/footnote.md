# L0 — compiler/stdlib/structural/footnote — construction
Hash do Código: 619bd258

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/structural/footnote.rs`

## Evidence before specification

The measured native-only boundary has three actual body commits c823fca5b/dfef736af/fac57c4d1, none touching Par; Quote shares only cross-cutting c4978547e/e1f09cc24. The quiet third criterion and separating fourth support one native owner. Pinned `typst-library/src/model/footnote.rs` has its own source unit. Construction validates/emits Content, while `00_nucleo/prompts/core/compiler/layout/footnote.md` owns layout/pending_footnote_bodies/format_pattern/superscript/position. This inspected phase distinction is not presumed or permission to move it.

Historical 2026-09-21 bilateral oracle `586e1bd43fae` shows missing function footnote.entry. Pinned footnote.rs:223-296 has required note Packed<FootnoteElem>, default separator Line content, clearance 1em/gap 0.5em/indent 1em; realization needs real note.location/Counter link-back and ShowSet leading 0.5em/size 0.85em. These are layout integration obligations, not constructor behavior. Historical probes against `9f2b6e8715237cb086899a42873660fe744622e8` retain their original snapshot limits and do not certify current `bd5ebec285505889e8ff1d4f68804ab147a699f3` without remeasurement. Language semantics, syntax, morphology and observable diagnostics govern acceptance; enum/match/algorithm structure is mechanics. Source inspection is not inferred upstream intent.

Human confirmation 2026-09-21 after causal commit 3ac60a578 ratifies qualified route/public FootnoteEntry entity in reduced construction-only scope. Integration measurement refuted analogous Portuguese sibling messages for the new route: oracle entry uses standard `missing argument: note`/`expected content, found integer`, and wrong-type separator/clearance plus existing `a length needs a unit - did you mean {i}pt?` via eval/rules::expected_length_error. Independent implementation/test roles exposed discrepancy; corrected suite GREEN/zero regression, without extending sibling's historical diagnostic divergence.

## Exclusive construction contract

Parent catalog `00_nucleo/prompts/core/compiler/stdlib/structural/mod.md` and conventions `00_nucleo/prompts/core/compiler/stdlib/mod.md` are boundaries, not shared productive ownership. footnote(body,numbering:?) requires one Content|Str positional; Str→Content::text. numbering Str|none preserves pattern/absence; invalid type exact `footnote(numbering:): espera string, recebeu {tipo}`. Emit Content::footnote_with_numbering(body,numbering). Missing body exact `footnote() exige body como argumento posicional`; wrong body `footnote() espera content ou string, recebeu {tipo}`. Any other named diagnoses deliberate graded cosmetic scope-out (ADR-0054), not generic unexpected argument. Pure L1 _ctx/_world/_current_file pass through unused, no note counter/number/position/no private helper. Moving phase requires ADR-0127 gate.

footnote.entry(note,separator:,clearance:,gap:,indent:) requires positional Content with same Str coercion; four optional named are Content separator/Length others, accepted/preserved without layout effect. Construct Content::FootnoteEntry(FootnoteEntryElem{note,separator,clearance,gap,indent}), entity owned by `00_nucleo/prompts/core/entities/elements/footnote_entry.md`. Use measured standard entry diagnostics, preserving existing expected_length_error rather than duplicate it.

Entry realization/link/superscript/counter, ShowSet automatic size/leading, visual effect of four named and replacement of existing footnote renderer are outside this reduced slice. Current layout keeps producing notes without consulting entry; show footnote.entry may intercept nothing until integration. That limitation is recorded as RED/open, not silently fixed. Any future layout integration must measure note.location↔page-position pipeline and obtain its own required gate.

## Acceptance and limits

Verify #footnote[x]/string coercion/pattern a/none; invalid numbering 1 yields `footnote(numbering:): espera string, recebeu integer`, no body/wrong 1 body preserve exact messages, unknown zz cites graded scope. Entry's six initial RED tests and measured note/separator/clearance error/unit hint become GREEN independently, with no inherited unmeasured sibling error policy. Preserve all fields and no visual effect. Valid L0/pins/V5/V15/V26/restricted lineage and full suites/build/lint remain required; ratification is only qualified construction/public entity scope, no universal footnote realization/show/render parity.
