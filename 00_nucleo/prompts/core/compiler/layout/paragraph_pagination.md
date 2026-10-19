# L0 — paragraph_pagination
Hash do Código: e7d76886

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/installed-line-ascent.toml sha256:993c94c8c186448b35b1ba8a0d84c1aadd95e9db0a09b19394fe95b08db52c7e
- 00_nucleo/prompts/_nuclei/paragraph-costs.toml sha256:d6c54bdd10c4ee5884a9a46bbdabaad3ffdea3aa0e4ad905411f6b43d1b6286b

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/paragraph_pagination.rs`

## Evidence before specification

`lab/results/construction/diagnostics/correcao-viuvas-orfas-2026-09-28/{before-pdf,before-extra}/` preserves 78 inputs/24 controls/full state in initial.json/before/ at historical vanilla `9f2b6e8715237cb086899a42873660fe744622e8`. page-65-100%-0%-False gives 3+1 Tekt versus 2+2 vanilla; zero costs match and tiny/large-positive retain protection. Pinned `flow/collect.rs:196-229` enables strictly-positive first-two/last-two and whole-three-line grouping; `flow/distribute.rs:343-364` advances only if required group fits next region. Emit previously progressed line by line. Distribution of words/lines in regions is language, not copied Frame/Child structure or inferred old-scope intent.

mixed-style-metrics-2026-10-18first-after/first-bib-after/first-regions-after/candidate1-sources preserve first candidate changing only line_commit with seven passing tests. audit-candidate1 finds matching SVG heading y in six bibliography cases and italic→roman, but forced still differs 0.156. linebreak.rs:46–58 appends ExplicitLinebreakBoundary with empty exterior-style Text: causal for bidi, not material face. Twenty regional PDF controls preserve same binaries/pin/sources as before-sources: at 39.44pt Tekt sends ITALIC to page 2 versus vanilla page 1;55.14pt forced 2 versus 1 pages;70pt wrap 3+1 versus 2+2 lines. Initial fix corrects wrapped y but not regional decisions. Invalid multipage SVG-without-template attempt stays in regions-before as a failed command, not SVG parity proof; use valid PDF and future regional script requests PDF only.

region_progress.rs:83–88 installs one-face baseline without updating assumed ascent;`:143–152` admits forecast before actual text. paragraph_pagination.rs:43–59 rejects differing edges and disables positive-cost grouping despite known selected ranges. Pinned flow/collect.rs:196–239 protects first/last actual-height frames; distribute.rs:343–364 advances group only if it fits next region. Language geometry/words/pages, not Profile shape/SVG IDs. Premature jump/recount/new callbacks/Group-based metrics/width-selection-leading changes or column/note/float/NoTypography regression refute correction.

Candidate inspection shows cursor ignored KnownTop in initial_baseline_pending/block_chain_active: pagination could request top6 while Block branch tests gap+top7 and advances too early. paragraph_pagination ensure_line_baseline links to both cursor top computations; pinned inline/line.rs:559–568 bases top on actual maximum frame. Required prechange RED: prepared text after actual material block boundary, prior baseline 7/below6.5/page 20/italic top6 stays baseline 19.5 on current page; outer top7 must not evict it. Zero widow/orphan isolates admission rather than grouping.

## Exclusive ownership and current contract

Layer L1. Sole consumer: `01_core/src/compiler/layout/paragraph_pagination.rs`. Own private certification/grouping/admission; existing Cursor/region_progress still calculate initial/block protocols and transitions. The following shared boundary description does not transfer those owners.

`00_nucleo/prompts/core/compiler/layout/paragraph_pagination.md` exclusively certifies vertical geometry/groups/regional admission from already-selected ranges, without reselection/glyph emission/duplicate new_page. Initial scope is prepared uniform vertical metrics/zero baseline offset/constant paginated geometry, outside columns/cells/subframes/notes/floats/active regional transaction; refused contexts retain fallback. Full certification precedes emission. Initial size/edges/leading match exterior, bold/italic allowed when metrics equal, finite nonnegative top/descent/leading. Later same-size/leading mixed-face certificate refines only edge uniformity as specified by that owner. Auto bounds need no protection; honor initial baseline/block collapse before real need versus page_bottom_limit. Next-region capacity uses next physical-page resolved margins; advance only with material now and group fitting there. Too-tall groups use ordinary progress/full content, never a new_page loop.

Private text_costs resolves widow/orphan independently of hyphenation/runt via FIELDS, default Ratio 1 and strict>0. Ordinary paragraph without linebreaks defaults to Simple to enable default protection; automatic-by-justify and all refused contexts remain. Preserve justification/carriers/terminal-open line/boundaries/lexical restore; do not apply incomplete protection to notes/columns.

Current mixed-face refinement allows differing tops/descents only for whole certified prepared set with equal size/leading, zero offsets and finite nonnegative edges, retaining all other exclusions. Private Profile borrows Prepared+selected ranges or prepared-hyphenated-line slice, with no Content clone/profile vector. Read exactly emitted fragment/Literal-gap styles. Calculate per-line maximum top/descent and group need as current descent plus sums of subsequent leading+top+descent, in a window of at most three lines. Admit current baseline through ensure_line_baseline(top); compare reconciled baseline preserving Some(0) with bottom. Next baseline is margin.top+first-group-line top. Advance only with material/current group fitting next, without loops. No future objects/callbacks, break recalculation or relaxed column/note/float exclusions; no dyn/vtable/appearance-based metric.

Profile supplies KnownTop only; Cursor/region_progress must install/test certified top also in initial and active-block branches, retaining their protocols. Generic/Text admission retains active-face query. Preserve font/size/leading/margin, no body execution to measure or created paint/height. Store the actually installed Some(top), including zero.

## Acceptance and limits

RED→GREEN covers identical words 2+2 versus 3+1, whole-three protection, independently disabled orphan, too-large group, restoration and default without flag. Bilateral boxes/words/pages retain prior types/spans/fallback; XML groups do not count physical lines. Restricted owners/reseal/build/lint remain required. This supersedes old widow/orphan exclusion only inside measured certificates, not full algorithm/performance/parity.

Mixed-face refinement is private continuous-flow ADR-0127 without public/product-flag/default/phase/callback-order change, preserving earlier refused/scalar paths. Regional certification expands only same-size/leading prepared text, zero baseline_offset, finite nonnegative edges, faces with different tops/descents; variable sizes/leading remain outside it. Require RED→GREEN four added regressions/prior controls,30+20 regional PDFs,132+94+21 corpus, broad suite/Clippy/Rustdoc/lineage under 12GiB/High8GiB/swap0/jobs1, recording failed attempts. No full variable-metrics parity claim.

Mixed-face distribution test requires words 6+5 in 2+2 lines versus 9+2 in 3+1, while zero costs keep 3+1. KnownTop block-entry test above must establish actual preimplementation RED and preserve default protocols. Variable size/leading remain uncertified; no extra public field/trait/signature/mode/default/phase or callback-order change is authorized.
