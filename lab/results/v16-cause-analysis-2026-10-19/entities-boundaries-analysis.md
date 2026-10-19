# V16 cause analysis: entities and boundaries

Plan only, read against clean HEAD `23bcda80d8348b08e5199312fddeec8ece2eb0a5`. Source and L0 digests match all 35 inventory rows. No implementation, L0/config change, test execution or oracle probe.

Plan only. Before implementation, update the exclusive current L0 with the justified classification and focused acceptance, validate V15/V26, then locally reseal through the authorized workflow. Preserve public API, L1 purity, target/phase/order, original diagnostics and effects. No suppressions, linter changes, if-let/matches! replacement, hidden helper catchall, or generalized success/default. Proposed tests and compiler experiments below have NOT been executed.

The findings are not interchangeable. Preserve legitimate rejection, no-replacement, delegated processing, and conservative layout-proof outcomes. Investigate same-kind equality omissions and local HTML/root observer assumptions before nominally sealing their complement. The simple Located(ContextBlock) double-count theory is refuted by the explicit suppressed body-root callback. The empty raw Sequence group count is a static helper observation, not an executed language defect.

## V16-071 — ast::UnOp::from_kind

Site: `01_core/src/entities/ast/expr.rs:602`; owner `00_nucleo/prompts/core/entities/ast/expr.md`; `SyntaxKind (01_core/src/entities/syntax_kind.rs:12-293)` on `token`.

Classification: valid negative classification; missing evolution obligation.

Evidence: 01_core/src/entities/ast/expr.rs:597-604: closed positive token map; remainder returns None; 01_core/src/compiler/parse/code.rs:97-99,127-137: parser set guards; Not+In is composed separately; 01_core/src/entities/ast/expr.rs:575-582,625-638: AST scans children, so nonoperators legitimately reach from_kind.

L0: 00_nucleo/prompts/core/entities/operators.md:24-49: arity, precedence, NotIn has no single SyntaxKind and negative Not/Star controls; 00_nucleo/prompts/core/entities/ast/expr.md:144-150: operator dependency and accessors.

Complement: Exactly Plus/Minus/Not are accepted here. Every current other SyntaxKind is a nonoperator for this arity, including markup, literal/AST nodes, comments and delimiters. Star is binary/markup, not unary.

Repair plan: Keep each positive mapping unchanged. Build the arity-specific negative class from the actual SyntaxKind definition, with an explicit nominal match at this site. Cross-check scanner/operator-set coverage rather than mechanically copying the other arity’s list. Do not panic on nonoperators or merge the two public UnOp/BinOp types to remove lint.

Regression plan (not executed): Positive map and precedence/associativity; comments/Space/End/Error/Ident/literals/delimiters return None. Parser unary/binary precedence and not in regression; AST child scanning skips nonoperator children.

Compiler-negative plan (not executed): In a disposable copy, add a sentinel to SyntaxKind (01_core/src/entities/syntax_kind.rs:12-293) and require E0004 at this exact decision after repair; record baseline behavior and exact file/span independently of unrelated errors. For a tuple match test the first-axis new/same and new/new combinations; do not let another match’s error stand in for this site. For an external enum, use a disposable copied dependency with the pinned variants plus sentinel, never change the productive/vendor API.

Dependencies: SyntaxKind owner, parser/code owner and AST/operator owners must cross-check distinct maps; no public type consolidation in this repair.

Gate: ADR-0127 continuous flow for a behavior-preserving internal classification, L0 first; no new public contract/default/phase.

Uncertainty/refuter: A positive parser operator token classified None would refute the negative partition. The AST L0 describes operators as cross-owner dependencies although AST currently declares its own types; reconcile documentary responsibility separately without public type unification.

## V16-072 — ast::BinOp::from_kind

Site: `01_core/src/entities/ast/expr.rs:692`; owner `00_nucleo/prompts/core/entities/ast/expr.md`; `SyntaxKind (01_core/src/entities/syntax_kind.rs:12-293)` on `token`.

Classification: valid negative classification; missing evolution obligation.

Evidence: 01_core/src/entities/ast/expr.rs:687-694: closed positive token map; remainder returns None; 01_core/src/compiler/parse/code.rs:97-99,127-137: parser set guards; Not+In is composed separately; 01_core/src/entities/ast/expr.rs:575-582,625-638: AST scans children, so nonoperators legitimately reach from_kind.

L0: 00_nucleo/prompts/core/entities/operators.md:24-49: arity, precedence, NotIn has no single SyntaxKind and negative Not/Star controls; 00_nucleo/prompts/core/entities/ast/expr.md:144-150: operator dependency and accessors.

Complement: Exactly Plus,Minus,Star,Slash,And,Or,EqEq,ExclEq,Lt,LtEq,Gt,GtEq,Eq,In,PlusEq,HyphEq,StarEq,SlashEq are accepted here. Every current other SyntaxKind is a nonoperator for this arity, including markup, literal/AST nodes, comments and delimiters. Not is not a binary token: NotIn must remain composed by the parser/AST accessor.

Repair plan: Keep each positive mapping unchanged. Build the arity-specific negative class from the actual SyntaxKind definition, with an explicit nominal match at this site. Cross-check scanner/operator-set coverage rather than mechanically copying the other arity’s list. Do not panic on nonoperators or merge the two public UnOp/BinOp types to remove lint.

Regression plan (not executed): Positive map and precedence/associativity; comments/Space/End/Error/Ident/literals/delimiters return None. Parser unary/binary precedence and not in regression; AST child scanning skips nonoperator children.

Compiler-negative plan (not executed): In a disposable copy, add a sentinel to SyntaxKind (01_core/src/entities/syntax_kind.rs:12-293) and require E0004 at this exact decision after repair; record baseline behavior and exact file/span independently of unrelated errors. For a tuple match test the first-axis new/same and new/new combinations; do not let another match’s error stand in for this site. For an external enum, use a disposable copied dependency with the pinned variants plus sentinel, never change the productive/vendor API.

Dependencies: SyntaxKind owner, parser/code owner and AST/operator owners must cross-check distinct maps; no public type consolidation in this repair.

Gate: ADR-0127 continuous flow for a behavior-preserving internal classification, L0 first; no new public contract/default/phase.

Uncertainty/refuter: A positive parser operator token classified None would refute the negative partition. The AST L0 describes operators as cross-owner dependencies although AST currently declares its own types; reconcile documentary responsibility separately without public type unification.

## V16-073 — Pattern::bindings

Site: `01_core/src/entities/ast/expr.rs:909`; owner `00_nucleo/prompts/core/entities/ast/expr.md`; `Pattern and nested Expr` on `self`.

Classification: valid binding projection with nested enum coverage hole.

Evidence: 01_core/src/entities/ast/expr.rs:876-911: four Pattern variants; only Normal(Ident) binds directly; 01_core/src/entities/ast/expr.rs:888-891,919-930 and entities/ast/code.rs:29: casts and recursive destructuring consumers.

L0: 00_nucleo/prompts/core/entities/ast/expr.md:134-150: preserve four forms and bound identifiers.

Complement: Placeholder binds nothing; Normal(non-Ident Expr) contributes no identifiers. Parenthesized and Destructuring recurse; their empty results are not failures.

Repair plan: Make the outer Pattern match exhaustive: retain Parenthesized/Destructuring recursion, explicit Placeholder empty, and a Normal arm with a direct exhaustive Expr classification (Ident yields one, reviewed nonbinding variants yield empty). No generic Normal(_) default or helper moving the gap.

Regression plan (not executed): Nested parenthesized/destructuring identifiers, named and spread sinks retain order; underscore/literal/normal nonbinding expression returns empty. Add separate Pattern and Expr sentinel compile negatives.

Compiler-negative plan (not executed): In a disposable copy, add a sentinel to Pattern and nested Expr and require E0004 at this exact decision after repair; record baseline behavior and exact file/span independently of unrelated errors. For a tuple match test the first-axis new/same and new/new combinations; do not let another match’s error stand in for this site. For an external enum, use a disposable copied dependency with the pinned variants plus sentinel, never change the productive/vendor API.

Dependencies: Only this owner and the referenced enum owner; no ownership transfer.

Gate: ADR-0127 continuous flow for a behavior-preserving internal classification, L0 first; no new public contract/default/phase.

Uncertainty/refuter: Literal patterns are allowed structurally; do not turn no-bindings into a parse error. A current Expr form that is meant to declare bindings refutes its empty classification.

## V16-074 — Content::eq (Rust PartialEq)

Site: `01_core/src/entities/content/equality.rs:209`; owner `00_nucleo/prompts/core/entities/content/equality.md`; `Content × Content` on `(self,other)`.

Classification: mixed: valid heterogeneous inequality plus unresolved same-variant omissions.

Evidence: 01_core/src/entities/content/equality.rs:11-27: transparent carriers and Space/SpaceFromNewline equivalence; 01_core/src/entities/content/equality.rs:204-209: comment acknowledges only Metadata/State/StateUpdate always-unequal quirk; 01_core/src/entities/content.rs:233-1318: current enum additionally has HtmlElem,HtmlFrame,Title,ContextBlock without same-variant equality arms.

L0: 00_nucleo/prompts/core/entities/content/equality.md:13-16: preserve existing distinctions; morphology/hash separately owned; :26-27,43-46: body/visual/source transparency.

Complement: Cross-kind non-equivalent pairs are correctly false after transparent wrappers and space equivalence. Same-kind Metadata/State/StateUpdate false is recorded existing mechanics; HtmlElem/HtmlFrame/Title/ContextBlock also fall through false but are not justified by that old comment.

Repair plan: First separate unlike-kind inequality from the seven uncovered same-kind classes. Preserve approved carrier/space and payload arms. Give the three historical always-false classes explicit documented mechanical treatment; audit each of HtmlElem/HtmlFrame/Title/ContextBlock against its entity owner before deciding its same-kind rule. Do not freeze all omitted variants as false just to clear V16, and do not derive language equality from Rust equality. For the proven cross-kind remainder use an exhaustive first-axis partition forcing future same/new decisions.

Regression plan (not executed): Rust controls for all seven uncovered same-kind cases, clone/reflexivity observation clearly labeled mechanics, plus unlike kinds and symmetric transparent carrier combinations. Separate language ==/fields/morphology controls under equality operator owner; no oracle needed to prove the current Rust fallthrough.

Compiler-negative plan (not executed): In a disposable copy, add a sentinel to Content × Content and require E0004 at this exact decision after repair; record baseline behavior and exact file/span independently of unrelated errors. For a tuple match test the first-axis new/same and new/new combinations; do not let another match’s error stand in for this site. For an external enum, use a disposable copied dependency with the pinned variants plus sentinel, never change the productive/vendor API.

Dependencies: Entity owners for HtmlElem/HtmlFrame/Title/ContextBlock, compiler/eval/operators/equality and content morphology/hash owners. Existing quirk is not authorization for new language behavior.

Gate: Nominal preservation of justified mechanical cases is continuous flow. Changed public equality/compatibility semantics needs explicit classification and, if a deliberate public contract change, ADR-0127 confirmation; measured parity correction can flow after evidence/L0.

Uncertainty/refuter: Static omission is proven; incorrect language equality is NOT proven because semantic equality has a different owner. Current owner evidence supporting intentional always-false for a newer same-kind variant would refute the suspected omission.

## V16-075 — map_content_bottom_up_pai_ve_filhos_transformados callback

Site: `01_core/src/entities/content/tests/traversal.rs:89`; owner `00_nucleo/prompts/core/entities/content/tests/traversal.md`; `Content` on `node`.

Classification: test-only valid no-replacement classification; coverage must stay meaningful.

Evidence: 01_core/src/entities/content/tests/traversal.rs:71-94: Text uppercases; Strong observes already uppercase body and returns Ok(None).

L0: 00_nucleo/prompts/core/entities/content/tests/traversal.md:10-17: bottom-up behavior/terminal preservation, independent fixtures.

Complement: Other variants are deliberately unchanged by this test transform; Ok(None) retains the processed node, not empty Content or skipped children.

Repair plan: Retain Text/Strong observations. Name the current no-replacement classes in this callback after reviewing traversal behavior; keep one unchanged outcome body. Add a non-Text/non-Strong terminal and container witness so the complement is tested, not excluded as test code.

Regression plan (not executed): Original bottom-up Strong/Text assertion plus sibling preservation, no-op terminal, and a container whose Text descendant still transforms. Sentinel must break this test callback as well as productive traversal.

Compiler-negative plan (not executed): In a disposable copy, add a sentinel to Content and require E0004 at this exact decision after repair; record baseline behavior and exact file/span independently of unrelated errors. For a tuple match test the first-axis new/same and new/new combinations; do not let another match’s error stand in for this site. For an external enum, use a disposable copied dependency with the pinned variants plus sentinel, never change the productive/vendor API.

Dependencies: Only this owner and the referenced enum owner; no ownership transfer.

Gate: ADR-0127 continuous flow for a behavior-preserving internal classification, L0 first; no new public contract/default/phase.

Uncertainty/refuter: If the map contract skips descendants when a callback returns None, or a fixture depends on another transformation, that refutes the classification; traversal.rs:671-673 currently retains processed children.

## V16-076 — container_reuse_tests::same_storage

Site: `01_core/src/entities/content/traversal.rs:2286`; owner `00_nucleo/prompts/core/entities/content/traversal.md`; `Content × Content` on `(a,b)`.

Classification: test-only finite storage-qualification predicate, not universal identity.

Evidence: 01_core/src/entities/content/traversal.rs:2150-2240: cases enumerates a bounded set of Arc containers; 01_core/src/entities/content/traversal.rs:2242-2287,2295,2341,2414-2415: Arc::ptr_eq positives and false fallback used by no-op/change controls.

L0: 00_nucleo/prompts/core/entities/content/traversal.md:16-27: exhaustive pure traversal and local tests; later container reuse acceptance retains structural observations separately from language.

Complement: Different variants and unqualified carrier classes return false; no claim that all current Content clones have different storage. A missing same-kind case from cases() would be a test-helper defect, not evidence to reconstruct productive data.

Repair plan: Cross-check cases() against each same-variant ptr_eq arm. Explicitly distinguish unlike variants and unqualified same-kind classes in a nominal first-axis exhaustive partition. Preserve all tested Arc comparisons; do not broaden storage equality to Rust PartialEq, payload hashes or address of inline values.

Regression plan (not executed): Assert clone/no-op sharing and changed-parent/new allocation with unchanged-child sharing for every qualified fixture; include unlike-kind false and one unqualified class false. Compile-negative on Content at this helper.

Compiler-negative plan (not executed): In a disposable copy, add a sentinel to Content × Content and require E0004 at this exact decision after repair; record baseline behavior and exact file/span independently of unrelated errors. For a tuple match test the first-axis new/same and new/new combinations; do not let another match’s error stand in for this site. For an external enum, use a disposable copied dependency with the pinned variants plus sentinel, never change the productive/vendor API.

Dependencies: Only this owner and the referenced enum owner; no ownership transfer.

Gate: ADR-0127 continuous flow for a behavior-preserving internal classification, L0 first; no new public contract/default/phase.

Uncertainty/refuter: A cases() fixture reaching false for input/input refutes helper completeness. Representation qualification is explicitly selected here, not proof of language identity or productive workload frequency.

## V16-077 — TableCellProperties::style_inset

Site: `01_core/src/entities/elements/table_cell.rs:184`; owner `00_nucleo/prompts/core/entities/elements/table_cell.md`; `Value` on `value`.

Classification: valid bounded cast rejection; no rich-inset expansion.

Evidence: 01_core/src/entities/elements/table_cell.rs:154-185: Length, validated partial Dict; side→axis→rest precedence; otherwise None; 01_core/src/compiler/eval/rules/set.rs:205-210: Auto handled separately, None triggers typed mismatch; compiler/realize/table_cells.rs:682 stores Option.

L0: 00_nucleo/prompts/core/entities/elements/table_cell.md:327-350: Option<Sides<Option<Length>>> and certified Length/partial Length inset/Auto; percentage/rich casts incomplete.

Complement: Top-level non-Length/non-Dict variants are unsupported by this private schema. Dict keys outside left/top/right/bottom/x/y/rest or non-Length values reject the whole dictionary. Missing sides remain None; Auto is caller-owned absence/reset, not an error here.

Repair plan: Retain exact accepted schema, precedence and no partial success; explicitly classify the 37 other current Value categories as unsupported projection under this owner. Keep Auto caller handling. Do not add Ratio/Relative/function conversion or erase unknown dict entries to make cast succeed.

Regression plan (not executed): Uniform Length, partial axis/rest/directional Dict precedence, empty Dict partial-none, unknown key/non-Length dictionary atomic rejection; Auto through set caller, wrong Value diagnostic/span; existing insets/placement geometry unchanged.

Compiler-negative plan (not executed): In a disposable copy, add a sentinel to Value and require E0004 at this exact decision after repair; record baseline behavior and exact file/span independently of unrelated errors. For a tuple match test the first-axis new/same and new/new combinations; do not let another match’s error stand in for this site. For an external enum, use a disposable copied dependency with the pinned variants plus sentinel, never change the productive/vendor API.

Dependencies: Only this owner and the referenced enum owner; no ownership transfer.

Gate: ADR-0127 continuous flow for a behavior-preserving internal classification, L0 first; no new public contract/default/phase.

Uncertainty/refuter: Diagnostic says relative length but this measured private schema accepts Length only; that wording does not authorize relative/percentage support. Rich casts need their own evidence/L0; caller audit refutes classification if a promised category is lost.

## V16-078 — FontVariations::from_validated_dict

Site: `01_core/src/entities/font_variations.rs:98`; owner `00_nucleo/prompts/core/entities/font_variations.md`; `Value` on `val`.

Classification: explicitly specified lenient omission, missing nominal coverage.

Evidence: 01_core/src/entities/font_variations.rs:92-104: Int/Float cast; others continue; invalid tags also omitted; 01_core/src/entities/style_chain.rs:761-770: only Dict levels enter this path; from_value supplies separate diagnostics.

L0: 00_nucleo/prompts/core/entities/font_variations.md:16-22,26-38,51-59: lenient validated path, strict validating path, normalization/fold.

Complement: Nonnumeric dictionary entries are intentionally ignored without error; invalid tag is independently ignored. Empty normalized result is valid.

Repair plan: Keep Int/Float numeric conversions, tag validation and normalized ordering; enumerate the current nonnumeric Value rejection class directly with continue. Do not replace the public Self-returning function with Result or reuse strict diagnostics to silence lint.

Regression plan (not executed): Mixed numeric/nonnumeric entries, invalid tags, duplicate normalized tag later-wins, empty dictionary and folding inner-over-outer. Controls separate strict from_value errors from lenient from_validated_dict.

Compiler-negative plan (not executed): In a disposable copy, add a sentinel to Value and require E0004 at this exact decision after repair; record baseline behavior and exact file/span independently of unrelated errors. For a tuple match test the first-axis new/same and new/new combinations; do not let another match’s error stand in for this site. For an external enum, use a disposable copied dependency with the pinned variants plus sentinel, never change the productive/vendor API.

Dependencies: Only this owner and the referenced enum owner; no ownership transfer.

Gate: ADR-0127 continuous flow for a behavior-preserving internal classification, L0 first; no new public contract/default/phase.

Uncertainty/refuter: If the L0’s lenient contract is deliberately changed it requires an independent decision; malformed input is not proof of a bug here.

## V16-079 — Radial::sample_precise

Site: `01_core/src/entities/gradient/radial.rs:167`; owner `00_nucleo/prompts/core/entities/gradient/radial.md`; `ColorSpace` on `self.space`.

Classification: valid delegation for supported non-specialized spaces.

Evidence: 01_core/src/entities/gradient/radial.rs:125-180: empty/single-stop/interval/end handling, specialized Oklab/LinearRgb, general interpolate_in_space; 01_core/src/entities/color.rs:305-315: nine spaces including Spot; gradient/interpolation.rs:106 handles Spot.

L0: 00_nucleo/prompts/core/entities/gradient/radial.md:16-23: preserve domain sampling and parent gradient parity.

Complement: Srgb/Luma/Oklch/Hsl/Hsv/Cmyk/Spot delegate actual interpolation, not a neutral result. Hue and spot identity belong to interpolate_in_space.

Repair plan: Name those seven delegated spaces explicitly, preserving specialized arithmetic and selected interval/local f64→f32 boundary. Verify Spot delegation against same-colorant/process fallback contract instead of converting every result to sRGB.

Regression plan (not executed): Both specialized spaces, three hue spaces including wrap, Srgb/Luma/Cmyk, same/different Spot colorants, alpha, duplicate offsets, outside endpoints and zero/one stops; no new tolerance or adaptive budget.

Compiler-negative plan (not executed): In a disposable copy, add a sentinel to ColorSpace and require E0004 at this exact decision after repair; record baseline behavior and exact file/span independently of unrelated errors. For a tuple match test the first-axis new/same and new/new combinations; do not let another match’s error stand in for this site. For an external enum, use a disposable copied dependency with the pinned variants plus sentinel, never change the productive/vendor API.

Dependencies: Only this owner and the referenced enum owner; no ownership transfer.

Gate: ADR-0127 continuous flow for a behavior-preserving internal classification, L0 first; no new public contract/default/phase.

Uncertainty/refuter: A delegated space whose interpolation requires a different radial-specific precision route refutes grouping; oracle numeric probes against current pin are planned only if existing owner criteria are insufficient.

## V16-080 — operators::UnOp::from_kind

Site: `01_core/src/entities/operators.rs:27`; owner `00_nucleo/prompts/core/entities/operators.md`; `SyntaxKind (01_core/src/entities/syntax_kind.rs:12-293)` on `token`.

Classification: valid negative classification; missing evolution obligation.

Evidence: 01_core/src/entities/operators.rs:22-29: closed positive token map; remainder returns None; 01_core/src/compiler/parse/code.rs:97-99,127-137: parser set guards; Not+In is composed separately; 01_core/src/entities/ast/expr.rs:575-582,625-638: AST scans children, so nonoperators legitimately reach from_kind.

L0: 00_nucleo/prompts/core/entities/operators.md:24-49: arity, precedence, NotIn has no single SyntaxKind and negative Not/Star controls; 00_nucleo/prompts/core/entities/ast/expr.md:144-150: operator dependency and accessors.

Complement: Exactly Plus/Minus/Not are accepted here. Every current other SyntaxKind is a nonoperator for this arity, including markup, literal/AST nodes, comments and delimiters. Star is binary/markup, not unary.

Repair plan: Keep each positive mapping unchanged. Build the arity-specific negative class from the actual SyntaxKind definition, with an explicit nominal match at this site. Cross-check scanner/operator-set coverage rather than mechanically copying the other arity’s list. Do not panic on nonoperators or merge the two public UnOp/BinOp types to remove lint.

Regression plan (not executed): Positive map and precedence/associativity; comments/Space/End/Error/Ident/literals/delimiters return None. Parser unary/binary precedence and not in regression; AST child scanning skips nonoperator children.

Compiler-negative plan (not executed): In a disposable copy, add a sentinel to SyntaxKind (01_core/src/entities/syntax_kind.rs:12-293) and require E0004 at this exact decision after repair; record baseline behavior and exact file/span independently of unrelated errors. For a tuple match test the first-axis new/same and new/new combinations; do not let another match’s error stand in for this site. For an external enum, use a disposable copied dependency with the pinned variants plus sentinel, never change the productive/vendor API.

Dependencies: SyntaxKind owner, parser/code owner and AST/operator owners must cross-check distinct maps; no public type consolidation in this repair.

Gate: ADR-0127 continuous flow for a behavior-preserving internal classification, L0 first; no new public contract/default/phase.

Uncertainty/refuter: A positive parser operator token classified None would refute the negative partition. The AST L0 describes operators as cross-owner dependencies although AST currently declares its own types; reconcile documentary responsibility separately without public type unification.

## V16-081 — operators::BinOp::from_kind

Site: `01_core/src/entities/operators.rs:95`; owner `00_nucleo/prompts/core/entities/operators.md`; `SyntaxKind (01_core/src/entities/syntax_kind.rs:12-293)` on `token`.

Classification: valid negative classification; missing evolution obligation.

Evidence: 01_core/src/entities/operators.rs:90-97: closed positive token map; remainder returns None; 01_core/src/compiler/parse/code.rs:97-99,127-137: parser set guards; Not+In is composed separately; 01_core/src/entities/ast/expr.rs:575-582,625-638: AST scans children, so nonoperators legitimately reach from_kind.

L0: 00_nucleo/prompts/core/entities/operators.md:24-49: arity, precedence, NotIn has no single SyntaxKind and negative Not/Star controls; 00_nucleo/prompts/core/entities/ast/expr.md:144-150: operator dependency and accessors.

Complement: Exactly Plus,Minus,Star,Slash,And,Or,EqEq,ExclEq,Lt,LtEq,Gt,GtEq,Eq,In,PlusEq,HyphEq,StarEq,SlashEq are accepted here. Every current other SyntaxKind is a nonoperator for this arity, including markup, literal/AST nodes, comments and delimiters. Not is not a binary token: NotIn must remain composed by the parser/AST accessor.

Repair plan: Keep each positive mapping unchanged. Build the arity-specific negative class from the actual SyntaxKind definition, with an explicit nominal match at this site. Cross-check scanner/operator-set coverage rather than mechanically copying the other arity’s list. Do not panic on nonoperators or merge the two public UnOp/BinOp types to remove lint.

Regression plan (not executed): Positive map and precedence/associativity; comments/Space/End/Error/Ident/literals/delimiters return None. Parser unary/binary precedence and not in regression; AST child scanning skips nonoperator children.

Compiler-negative plan (not executed): In a disposable copy, add a sentinel to SyntaxKind (01_core/src/entities/syntax_kind.rs:12-293) and require E0004 at this exact decision after repair; record baseline behavior and exact file/span independently of unrelated errors. For a tuple match test the first-axis new/same and new/new combinations; do not let another match’s error stand in for this site. For an external enum, use a disposable copied dependency with the pinned variants plus sentinel, never change the productive/vendor API.

Dependencies: SyntaxKind owner, parser/code owner and AST/operator owners must cross-check distinct maps; no public type consolidation in this repair.

Gate: ADR-0127 continuous flow for a behavior-preserving internal classification, L0 first; no new public contract/default/phase.

Uncertainty/refuter: A positive parser operator token classified None would refute the negative partition. The AST L0 describes operators as cross-owner dependencies although AST currently declares its own types; reconcile documentary responsibility separately without public type unification.

## V16-082 — SyntaxNode::spanless_eq

Site: `01_core/src/entities/syntax_node/representation.rs:196`; owner `00_nucleo/prompts/core/entities/syntax_node/representation.md`; `NodeKind × NodeKind` on `(&self.0,&other.0)`.

Classification: valid unlike-representation inequality.

Evidence: 01_core/src/entities/syntax_node/representation.rs:27-34,191-197: Leaf/Inner/Error, three same-kind delegates.

L0: 00_nucleo/prompts/core/entities/syntax_node/representation.md:18-25: ignore only spans; retain kind/message/hints/text, COW.

Complement: The six heterogeneous Leaf/Inner/Error pairs are false. Equal text does not erase node representation/kind/error distinctions.

Repair plan: Use the six explicit heterogeneous pairs, preserving three same-kind bodies. This is a small reviewed cross-product, not a helper based on discriminant hash or generic false.

Regression plan (not executed): All 3×3 representation categories, same structure different spans true, changed kind/text/message/hint false, COW unaffected. Sentinel probe on each tuple axis.

Compiler-negative plan (not executed): In a disposable copy, add a sentinel to NodeKind × NodeKind and require E0004 at this exact decision after repair; record baseline behavior and exact file/span independently of unrelated errors. For a tuple match test the first-axis new/same and new/new combinations; do not let another match’s error stand in for this site. For an external enum, use a disposable copied dependency with the pinned variants plus sentinel, never change the productive/vendor API.

Dependencies: Only this owner and the referenced enum owner; no ownership transfer.

Gate: ADR-0127 continuous flow for a behavior-preserving internal classification, L0 first; no new public contract/default/phase.

Uncertainty/refuter: A documented equivalence across Leaf/Inner/Error would refute the current negative class; none is supplied by the current L0.

## V16-083 — content_to_semantic initial carrier dispatch

Site: `02_shell/src/cli.rs:1161`; owner `00_nucleo/prompts/shell/cli.md`; `Content` on `content`.

Classification: valid wrapper dispatch versus continue-to-projection complement.

Evidence: 02_shell/src/cli.rs:1152-1177: four carriers recurse then Label or field projection.

L0: 00_nucleo/prompts/shell/cli.md:153-165,479-483: authoritative snapshots separate; raw legacy projection, source transparency.

Complement: Every non-Source/ShowRecipe/Located/Realized variant continues to Label handling and legacy field projection. Empty here does not mean empty serialized content.

Repair plan: Keep four recursive returns, explicitly name current ordinary language variants in the continue class. Preserve subsequent Label ordering and do not serialize carrier or replay recipes. No public serializer redesign.

Regression plan (not executed): Nested carriers around Empty/Text/Raw/Label/heading, Realized visual≠original and authoritative Some snapshot versus None. JSON/YAML fields/order and labels unchanged.

Compiler-negative plan (not executed): In a disposable copy, add a sentinel to Content and require E0004 at this exact decision after repair; record baseline behavior and exact file/span independently of unrelated errors. For a tuple match test the first-axis new/same and new/new combinations; do not let another match’s error stand in for this site. For an external enum, use a disposable copied dependency with the pinned variants plus sentinel, never change the productive/vendor API.

Dependencies: Only this owner and the referenced enum owner; no ownership transfer.

Gate: ADR-0127 continuous flow for a behavior-preserving internal classification, L0 first; no new public contract/default/phase.

Uncertainty/refuter: Any variant in the continue class requiring its own carrier projection refutes that membership; no global inference that every wrapper is transparent.

## V16-084 — content_to_semantic legacy field fallback

Site: `02_shell/src/cli.rs:1412`; owner `00_nucleo/prompts/shell/cli.md`; `Content` on `content`.

Classification: valid bounded legacy projection; incomplete fields explicitly scoped.

Evidence: 02_shell/src/cli.rs:1178-1418: specialized field cases; fallback probes only get_field("body").

L0: 00_nucleo/prompts/shell/cli.md:153-165,544-596: raw/None legacy fallback is not full language-field certification; explicit Entry/Caption refinements.

Complement: Residual current variants get func plus body only if a body getter exists. Terminals without body are func-only; this is not authority to fabricate all fields or apply it to Some snapshots.

Repair plan: Audit the remaining variants against the current getter catalog; separate residual body-bearing and func-only classes with nominal patterns while retaining get_field behavior and field order. Preserve specialized MathMatrix/Cell/Caption/Entry cases and typed MathAttach presence. Do not replace legacy fallback with universal fields enumeration.

Regression plan (not executed): One residual body carrier and terminal, Empty, specialized Entry/Caption/Cell/Mat, label transport; Some(empty) and Some(known) remain authoritative and unchanged. Missing-field/cardinality diagnostics through query.

Compiler-negative plan (not executed): In a disposable copy, add a sentinel to Content and require E0004 at this exact decision after repair; record baseline behavior and exact file/span independently of unrelated errors. For a tuple match test the first-axis new/same and new/new combinations; do not let another match’s error stand in for this site. For an external enum, use a disposable copied dependency with the pinned variants plus sentinel, never change the productive/vendor API.

Dependencies: Only this owner and the referenced enum owner; no ownership transfer.

Gate: ADR-0127 continuous flow for a behavior-preserving internal classification, L0 first; no new public contract/default/phase.

Uncertainty/refuter: A currently promised explicit field omitted by this fallback is a semantic defect to repair under its actual owner after evidence, not a reason to bless every residual class as complete.

## V16-085 — content_semantic_func_name

Site: `02_shell/src/cli.rs:1445`; owner `00_nucleo/prompts/shell/cli.md`; `Content` on `content`.

Classification: valid canonical-name delegation after explicit overrides.

Evidence: 02_shell/src/cli.rs:1421-1446: carrier projection, Empty/Shape/Caption/SmartQuote/Attach/math overrides, elem_name fallback.

L0: 00_nucleo/prompts/shell/cli.md:155-165,531-536,591-595: public func names and transparency.

Complement: Other variants retain core elem_name; not empty and not unknown. Shape still selects rect/ellipse/line/polygon exhaustively.

Repair plan: Keep overrides and ShapeKind match. Name current delegated categories explicitly after comparing their core elem_name; no duplicate shell-owned public-name table and no unconditional default for future variants.

Regression plan (not executed): Core-delegated headings, table cell, raw and residual terminal; shape families, Empty, math text/symbol, caption, nested carriers, JSON/YAML matching.

Compiler-negative plan (not executed): In a disposable copy, add a sentinel to Content and require E0004 at this exact decision after repair; record baseline behavior and exact file/span independently of unrelated errors. For a tuple match test the first-axis new/same and new/new combinations; do not let another match’s error stand in for this site. For an external enum, use a disposable copied dependency with the pinned variants plus sentinel, never change the productive/vendor API.

Dependencies: Only this owner and the referenced enum owner; no ownership transfer.

Gate: ADR-0127 continuous flow for a behavior-preserving internal classification, L0 first; no new public contract/default/phase.

Uncertainty/refuter: Mismatch between elem_name and a promised public name refutes delegated membership; measure that named surface before adding an override.

## V16-086 — collect_top inner UA1 SemanticKind dispatch

Site: `03_infra/src/export/builder/metadata.rs:789`; owner `00_nucleo/prompts/infra/export/builder/metadata.md`; `SemanticKind` on `kind`.

Classification: valid boundary interception; complement continues tagging.

Evidence: 03_infra/src/export/builder/metadata.rs:775-811: UA1/outside artifact intercepts Parbreak and ExplicitLinebreak; other kinds flow to semantic_node or children; 01_core/src/entities/layout_types.rs:312-353: eleven semantic kinds.

L0: 00_nucleo/prompts/infra/export/builder/metadata.md:49-71,104-108: artifact exclusion, paragraph/link/table/MCID order.

Complement: Formula,Label,Artifact,Table,TableHead,TableBody,TableRow,TableHeaderCell,TableDataCell are not intercepted here; they must continue semantic-node construction/descendant handling. Empty body is not skip-output.

Repair plan: Name these nine continuation kinds, preserving both early returns, ua1/inside_artifact conditions, and subsequent artifact propagation. Do not group them into discarded structure.

Regression plan (not executed): UA1/non-UA1 and artifact nested boundaries; no empty paragraph nodes, table logical structure and Link associations, Formula alt and MCID increments/order. New SemanticKind sentinel breaks inner decision.

Compiler-negative plan (not executed): In a disposable copy, add a sentinel to SemanticKind and require E0004 at this exact decision after repair; record baseline behavior and exact file/span independently of unrelated errors. For a tuple match test the first-axis new/same and new/new combinations; do not let another match’s error stand in for this site. For an external enum, use a disposable copied dependency with the pinned variants plus sentinel, never change the productive/vendor API.

Dependencies: Only this owner and the referenced enum owner; no ownership transfer.

Gate: ADR-0127 continuous flow for a behavior-preserving internal classification, L0 first; no new public contract/default/phase.

Uncertainty/refuter: If continuation changes MCID/structure or suppresses meaningful descendants, grouping is wrong. These controls do not certify full PDF/UA or assistive-technology behavior.

## V16-087 — sample_gradient_precise_for_svg hue-index selection

Site: `03_infra/src/export/gradients/adaptive.rs:275`; owner `00_nucleo/prompts/infra/export/gradients/adaptive.md`; `ColorSpace` on `space`.

Classification: valid no-hue decision with dominating five-space guard.

Evidence: 03_infra/src/export/gradients/adaptive.rs:213-232: Conic and nonfive-space domain fallback; 03_infra/src/export/gradients/adaptive.rs:253-305: native components, hue route and reconstruction.

L0: 00_nucleo/prompts/infra/export/gradients/adaptive.md:25-45: f64 intervals, weights, short hue route, other-space domain sampling.

Complement: Oklab and LinearRgb are reachable no-hue paths. Srgb/Luma/Cmyk/Spot cannot reach this decision under the existing guard, and should remain routed to domain sample, not get new interpolation.

Repair plan: Keep Oklch index2/Hsl-Hsv index0 and explicit no-hue categories. Also make the dominating accepted-space gate and downstream output-space dispatch require an enum-growth decision; do not rely on a permissive guard to hide future variants. Retain all native precision/order/alpha/error-budget arithmetic.

Regression plan (not executed): Two nonangular reachable spaces and all three hue families across 360; verify four early domain fallback spaces, Conic, coincident stops and alpha. Compile-negative must identify this hue decision, not just reconstruction error.

Compiler-negative plan (not executed): In a disposable copy, add a sentinel to ColorSpace and require E0004 at this exact decision after repair; record baseline behavior and exact file/span independently of unrelated errors. For a tuple match test the first-axis new/same and new/new combinations; do not let another match’s error stand in for this site. For an external enum, use a disposable copied dependency with the pinned variants plus sentinel, never change the productive/vendor API.

Dependencies: Only this owner and the referenced enum owner; no ownership transfer.

Gate: ADR-0127 continuous flow for a behavior-preserving internal classification, L0 first; no new public contract/default/phase.

Uncertainty/refuter: If a space reaches the sampler despite the dominating guard, reachability classification is refuted. Do not add unsupported native sample spaces just to make this match total.

## V16-088 — export_html_with_frame_resolver visual map callback

Site: `03_infra/src/export/html/document.rs:37`; owner `00_nucleo/prompts/infra/export/html/document.md`; `Content` on `node`.

Classification: valid unchanged complement plus carrier-root semantic candidate.

Evidence: 03_infra/src/export/html/document.rs:27-39: unwrap Located/Realized/ShowRecipe/Label, Empty for State/StateUpdate/Metadata, None otherwise; 01_core/src/entities/content/traversal.rs:436,667-673: Located suppresses body-root callback; replacement is not reprocessed.

L0: 00_nucleo/prompts/infra/export/html/document.md:47-72,128-136: visual-only projection, Label body, markers Empty, unresolved producers not hidden; 00_nucleo/prompts/core/entities/content/traversal.md:16-22,30-39: bottom-up and Located root suppression.

Complement: Ordinary visual content correctly returns None to preserve processed descendants; unsupported content must remain for the renderer’s error. However a Located root containing State can be unwrapped into a State replacement without the marker arm being applied.

Repair plan: First add a targeted private projection control for Located(State), Located(Metadata), nested carrier chains and Located(Label(State)). If confirmed, make this owner’s visual projection classify the current unwrapped root before returning its replacement, using explicit carrier/marker semantics without executing callbacks or broad global transparency. Then classify ordinary unchanged variants nominally. Do not silence the defect by adding wrappers/markers to a no-op complement.

Regression plan (not executed): Direct vs Located/wrapped markers yield same Empty DOM contribution; Label retains body; unresolved ContextBlock/StateDisplay still errors; no repeated recipes/effects. Test replacement non-revisit behavior, explicit roots and paragraph boundaries.

Compiler-negative plan (not executed): In a disposable copy, add a sentinel to Content and require E0004 at this exact decision after repair; record baseline behavior and exact file/span independently of unrelated errors. For a tuple match test the first-axis new/same and new/new combinations; do not let another match’s error stand in for this site. For an external enum, use a disposable copied dependency with the pinned variants plus sentinel, never change the productive/vendor API.

Dependencies: Content traversal contract and HTML document/inline owners; no change to generic map_content semantics.

Gate: Internal bug/parity correction can use ADR-0127 continuous flow after focused RED and L0. No public API/default/phase expansion.

Uncertainty/refuter: Static callback path suggests a concrete inconsistency: root Located{body:State} → callback Located → Some(State), not revisited. End-to-end production reachability/actual export result has NOT been run; a prior preparation guarantee excluding such inputs or a renderer that deliberately handles them refutes external failure claim.

## V16-089 — visual_content

Site: `03_infra/src/export/html/encoding.rs:19`; owner `00_nucleo/prompts/infra/export/html/encoding.md`; `Content` on `content`.

Classification: valid terminal return after local carrier peeling.

Evidence: 03_infra/src/export/html/encoding.rs:12-21: loops over Source/ShowRecipe/Located/Realized then returns borrowed leaf.

L0: 00_nucleo/prompts/infra/export/html/encoding.md:88-95: exact local transparency and strict nontext rejection.

Complement: All other variants stop peeling and remain subject to raw/title textual validation. Returning input is not accepting it as text.

Repair plan: Explicitly name the ordinary stopping variants without extending transparency to Styled/Label/ContextBlock here; preserve downstream title style/quote handling and strict nontext errors.

Regression plan (not executed): Each carrier and nested visual≠original; wrapped Linebreak/Space separator absorption, title SmartQuote, rejected nontext and script/style guards.

Compiler-negative plan (not executed): In a disposable copy, add a sentinel to Content and require E0004 at this exact decision after repair; record baseline behavior and exact file/span independently of unrelated errors. For a tuple match test the first-axis new/same and new/new combinations; do not let another match’s error stand in for this site. For an external enum, use a disposable copied dependency with the pinned variants plus sentinel, never change the productive/vendor API.

Dependencies: Only this owner and the referenced enum owner; no ownership transfer.

Gate: ADR-0127 continuous flow for a behavior-preserving internal classification, L0 first; no new public contract/default/phase.

Uncertainty/refuter: A terminal erroneously accepted by downstream raw/title guard refutes that guard, not the local stop class. Never treat all body-bearing content as textual.

## V16-090 — collect cmap native-format selection

Site: `03_infra/src/fonts/coverage.rs:90`; owner `00_nucleo/prompts/infra/fonts/coverage.md`; `ttf_parser::cmap::Format` on `subtable.format`.

Classification: valid native-unsupported versus callback-supported dispatch.

Evidence: 03_infra/src/fonts/coverage.rs:78-117: is_unicode gate, native 4/12/13, checkpoint rollback; :118-149 callback; ttf-parser-0.25.1/src/tables/cmap/mod.rs:35-45: nine Format variants.

L0: 00_nucleo/prompts/infra/fonts/coverage.md:25-58: native exactly 4/12/13, other Unicode formats use callback; first-invalid-record and overflow rules.

Complement: ByteEncodingTable/HighByteMappingThroughTable/TrimmedTableMapping/MixedCoverage/TrimmedArray/UnicodeVariationSequences select no native reader, then callback semantics apply. None does not mean discard coverage.

Repair plan: Name six callback-path formats directly; retain Unicode gate, per-subtable checkpoint rollback and old Face fallback on unrepresentable exclusive endpoint. No native support expansion, glyph-zero filtering or approximate Unicode blocks.

Regression plan (not executed): All typed callback formats with supported fixtures, 4/12/13 positives, failed native rollback, non-Unicode, alias reuse/distinct offsets, malformed record termination and overflow fallback. External enum-growth compile-negative in copied dependency.

Compiler-negative plan (not executed): In a disposable copy, add a sentinel to ttf_parser::cmap::Format and require E0004 at this exact decision after repair; record baseline behavior and exact file/span independently of unrelated errors. For a tuple match test the first-axis new/same and new/new combinations; do not let another match’s error stand in for this site. For an external enum, use a disposable copied dependency with the pinned variants plus sentinel, never change the productive/vendor API.

Dependencies: Only this owner and the referenced enum owner; no ownership transfer.

Gate: ADR-0127 continuous flow for a behavior-preserving internal classification, L0 first; no new public contract/default/phase.

Uncertainty/refuter: Typed callback may enumerate no points for unsupported format (including variation sequences); keep its exact declared baseline behavior. A member-set mismatch against old callback is the refuter, not different allocation counts.

## V16-091 — sec_27 math integration test::equations observer

Site: `03_infra/src/integration_tests.rs:4888`; owner `00_nucleo/prompts/infra/integration_tests.md`; `Content` on `c`.

Classification: test-only observer completeness candidate, not productive math defect.

Evidence: 03_infra/src/integration_tests.rs:4873-4907: eval module tree, collect four equations, third ascent expected 11.4686 ±0.001; 01_core/src/entities/content/traversal.rs:30-39,436-439: transported-root and visual distinction.

L0: 00_nucleo/prompts/infra/integration_tests.md:24-40: language/geometry observations; never fix observer by weakening expected geometry.

Complement: No-op is valid for known non-equation terminals, but Source/Located/Realized/ShowRecipe and body-bearing containers can conceal an Equation from this hand-written partial walker.

Repair plan: Audit actual sec_27 tree and fixture scope before coding. Plan a direct Equation, Source(Equation), Located(Equation), Realized(original different, visual Equation), and nested container observation; classify intended carriers locally rather than assume global transparency. Add justified descendant cases and nominally classify true terminals. Keep Eq boundary (collect body once, no nested double count) and existing 4/ascent assertions.

Regression plan (not executed): Observer controls plus unchanged real-font equation geometry; no tolerances/fixture/fonts/layout changes. Sentinel at observer. Any needed current-pin geometry probe remains not executed.

Compiler-negative plan (not executed): In a disposable copy, add a sentinel to Content and require E0004 at this exact decision after repair; record baseline behavior and exact file/span independently of unrelated errors. For a tuple match test the first-axis new/same and new/new combinations; do not let another match’s error stand in for this site. For an external enum, use a disposable copied dependency with the pinned variants plus sentinel, never change the productive/vendor API.

Dependencies: Only this owner and the referenced enum owner; no ownership transfer.

Gate: ADR-0127 continuous flow for a behavior-preserving internal classification, L0 first; no new public contract/default/phase.

Uncertainty/refuter: An owner-stated fixture intentionally restricted to bare Sequence/Styled would make this a bounded observer rather than a bug; actual do_eval output and missing expected equation provide the discriminator. No runtime failure asserted.

## V16-092 — bundle_html_semantic_content

Site: `03_infra/src/pipeline/bundle.rs:379`; owner `00_nucleo/prompts/infra/pipeline/bundle.md`; `Content` on `content`.

Classification: valid identity for nonconverted elements; container completeness requires focused controls.

Evidence: 03_infra/src/pipeline/bundle.rs:318-379: four causal carriers, labels/headings and selected recursive containers; HtmlFrame remains unmodified at :325.

L0: 00_nucleo/prompts/infra/pipeline/bundle.md:36-41,83-92: final render copy, styles/order/heading warning, frame islands.

Complement: Residual leaves/elements keep their language carrier for the HTML exporter; some will intentionally diagnose unsupported HTML. HtmlFrame is already an explicit opaque island and must not be descended by this conversion.

Repair plan: Build explicit identity classes only after checking each residual container for whether a heading descendant needs this bundle conversion. Keep current heading offset/deep warning, inherited styles, Source/Located/Realized visual projection and frame boundary; add recursion only for an evidenced missing promised container, not every body getter.

Regression plan (not executed): Wrapped/styled heading, level>5 warning once, adjacent text, nested HtmlElem and unmodified HtmlFrame with heading inside; residual unsupported element diagnostic unchanged.

Compiler-negative plan (not executed): In a disposable copy, add a sentinel to Content and require E0004 at this exact decision after repair; record baseline behavior and exact file/span independently of unrelated errors. For a tuple match test the first-axis new/same and new/new combinations; do not let another match’s error stand in for this site. For an external enum, use a disposable copied dependency with the pinned variants plus sentinel, never change the productive/vendor API.

Dependencies: HTML document/inline/frame resolver owners and bundle global snapshot; no eval/callback replay.

Gate: ADR-0127 continuous flow for a behavior-preserving internal classification, L0 first; no new public contract/default/phase.

Uncertainty/refuter: A residual Link/Quote/etc container containing a heading is a minimal candidate for missing normalization, but exporter ownership may intentionally reject it. No blanket claim of hidden headings or automatic full-tree conversion.

## V16-093 — ordered_blocks

Site: `03_infra/src/pipeline/context_stabilization.rs:97`; owner `00_nucleo/prompts/infra/pipeline/context_stabilization.md`; `Content` on `node`.

Classification: valid producer-recognition complement; apparent Located duplicate refuted.

Evidence: 03_infra/src/pipeline/context_stabilization.rs:79-109: explicit bare ContextBlock and Located(ContextBlock) recognition; 01_core/src/entities/content/traversal.rs:436,667-673: immediate Located body callback suppressed.

L0: 00_nucleo/prompts/infra/pipeline/context_stabilization.md:28-34,53-55,123-127: causal ordering, duplicate handling, empty proof, legacy fallback.

Complement: All other current nodes contribute no producer at their own callback; L1 traversal already visited permitted descendants. Preserve detect all duplicate ids and preserve encounter order; None triggers legacy mode. Located with non-ContextBlock body also yields None locally.

Repair plan: Keep both bare and Located producer cases; nested match must also state the nonproducer Content classes rather than moving a wildcard into a helper. Nominally partition callback complement after confirming traversal/call semantics. Do not remove Located case as redundant or deduplicate by equality/hash.

Regression plan (not executed): Located{body:ContextBlock(id7)} produces one event, because map_content_inner(false) suppresses the bare root callback. Two distinct occurrences with same id still fail ordered uniqueness; distinct ids preserve order. Legacy mode keeps first occurrence/chain and unchanged number of evaluations. Realized.original-only producer stays hidden; empty by_id remains valid.

Compiler-negative plan (not executed): In a disposable copy, add a sentinel to Content and require E0004 at this exact decision after repair; record baseline behavior and exact file/span independently of unrelated errors. For a tuple match test the first-axis new/same and new/new combinations; do not let another match’s error stand in for this site. For an external enum, use a disposable copied dependency with the pinned variants plus sentinel, never change the productive/vendor API.

Dependencies: L1 Content traversal root-suppression contract and pipeline collect_context_blocks style map. Strict V15/V26 before any repair.

Gate: ADR-0127 continuous flow for a behavior-preserving internal classification, L0 first; no new public contract/default/phase.

Uncertainty/refuter: The simple double-count hypothesis is statically REFUTED by transform_root=false, matching L0:55. A future map contract delivering both callbacks would refute it and must fail these controls. No callback execution/probe performed.

## V16-094 — Session::new legacy fallback callback

Site: `03_infra/src/pipeline/context_stabilization.rs:182`; owner `00_nucleo/prompts/infra/pipeline/context_stabilization.md`; `Content` on `node`.

Classification: valid producer-recognition complement; apparent Located duplicate refuted.

Evidence: 03_infra/src/pipeline/context_stabilization.rs:158-223: explicit bare ContextBlock and Located(ContextBlock) recognition; 01_core/src/entities/content/traversal.rs:436,667-673: immediate Located body callback suppressed.

L0: 00_nucleo/prompts/infra/pipeline/context_stabilization.md:28-34,53-55,123-127: causal ordering, duplicate handling, empty proof, legacy fallback.

Complement: All other current nodes contribute no producer at their own callback; L1 traversal already visited permitted descendants. Preserve retain only first id occurrence, one ordinary evaluation per id, and impossible flag. Located with non-ContextBlock body also yields None locally.

Repair plan: Keep both bare and Located producer cases; nested match must also state the nonproducer Content classes rather than moving a wildcard into a helper. Nominally partition callback complement after confirming traversal/call semantics. Do not remove Located case as redundant or deduplicate by equality/hash.

Regression plan (not executed): Located{body:ContextBlock(id7)} produces one event, because map_content_inner(false) suppresses the bare root callback. Two distinct occurrences with same id still fail ordered uniqueness; distinct ids preserve order. Legacy mode keeps first occurrence/chain and unchanged number of evaluations. Realized.original-only producer stays hidden; empty by_id remains valid.

Compiler-negative plan (not executed): In a disposable copy, add a sentinel to Content and require E0004 at this exact decision after repair; record baseline behavior and exact file/span independently of unrelated errors. For a tuple match test the first-axis new/same and new/new combinations; do not let another match’s error stand in for this site. For an external enum, use a disposable copied dependency with the pinned variants plus sentinel, never change the productive/vendor API.

Dependencies: L1 Content traversal root-suppression contract and pipeline collect_context_blocks style map. Strict V15/V26 before any repair.

Gate: ADR-0127 continuous flow for a behavior-preserving internal classification, L0 first; no new public contract/default/phase.

Uncertainty/refuter: The simple double-count hypothesis is statically REFUTED by transform_root=false, matching L0:55. A future map contract delivering both callbacks would refute it and must fail these controls. No callback execution/probe performed.

## V16-095 — collect_html_frames map callback

Site: `03_infra/src/pipeline/html.rs:186`; owner `00_nucleo/prompts/infra/pipeline/html.md`; `Content` on `node`.

Classification: nonframe no-op valid; Located frame observer candidate.

Evidence: 03_infra/src/pipeline/html.rs:158-200: frame_count bottom-up, HtmlFrame collection, Styled ancestors for last count frames; :224 consumer resolves frames; 01_core/src/entities/content/traversal.rs:436,667-673: Located suppresses immediate child-root callback.

L0: 00_nucleo/prompts/infra/pipeline/html.md:26-32: occurrence order/effective styles, nested islands and fail-closed topology.

Complement: Ordinary nonframe nodes do nothing at their callback while descendants are visited. Direct Located(HtmlFrame) is a candidate exception because neither callback matches HtmlFrame; frame_count may report zero for that same shape.

Repair plan: Test exact direct-transported frame shape in both frame_count and collection, then repair root recognition in this owner if reachable/permitted; keep nested island ordering and outer StyleChain calculation aligned. Classify true nonframe remainder explicitly. Do not globally unwrap all carriers or emit a replacement for unsupported islands.

Regression plan (not executed): Compare direct HtmlFrame with Located{body:HtmlFrame} under one/two Styled ancestors, sibling frames and nested frames; assert collected occurrence order, frame count and intrinsic positions/visible styles without a global PageStore.

Compiler-negative plan (not executed): In a disposable copy, add a sentinel to Content and require E0004 at this exact decision after repair; record baseline behavior and exact file/span independently of unrelated errors. For a tuple match test the first-axis new/same and new/new combinations; do not let another match’s error stand in for this site. For an external enum, use a disposable copied dependency with the pinned variants plus sentinel, never change the productive/vendor API.

Dependencies: Content traversal semantics, HtmlFrame producer owner, intrinsic frame layout and resolver; no public page identity/default/phase change.

Gate: ADR-0127 continuous flow for a behavior-preserving internal classification, L0 first; no new public contract/default/phase.

Uncertainty/refuter: Helper-level direct Located frame omission follows the static callback contract, but language production of this shape has NOT been demonstrated. A producer invariant forbidding it refutes the end-to-end issue. Any repair must preserve fail-closed collision/ambiguity behavior.

## V16-096 — page_independent_style text.font value classification

Site: `03_infra/src/pipeline/paged.rs:20`; owner `00_nucleo/prompts/infra/pipeline/paged.md`; `Value` on `value (under key == "text.font")`.

Classification: valid fail-closed optimization proof.

Evidence: 03_infra/src/pipeline/paged.rs:13-52: only Str or all-Str Array accepted for text.font; 03_infra/src/pipeline/paged.rs:528-529: both contextual/realized proof required before reuse.

L0: 00_nucleo/prompts/infra/pipeline/paged.md:64-78: narrow physical candidate reuse whitelist; unknown values retain full cycle.

Complement: All current other Value categories cannot prove a physical font-family scalar, so false selects full layout/stabilization; it does not reject language font data or choose fallback fonts. Mixed Array already false.

Repair plan: Name the non-Str/non-Array proof-rejection categories directly, preserve empty-array all predicate and other key branches. Do not broaden accepted scalars or change actual font validation to reduce lint.

Regression plan (not executed): Str/all-Str/mixed/empty Array and each representative nonfamily category; verify full cycle still compiles valid unproved content and retains diagnostics. No new font catalog/shaping work in the proof.

Compiler-negative plan (not executed): In a disposable copy, add a sentinel to Value and require E0004 at this exact decision after repair; record baseline behavior and exact file/span independently of unrelated errors. For a tuple match test the first-axis new/same and new/new combinations; do not let another match’s error stand in for this site. For an external enum, use a disposable copied dependency with the pinned variants plus sentinel, never change the productive/vendor API.

Dependencies: Only this owner and the referenced enum owner; no ownership transfer.

Gate: ADR-0127 continuous flow for a behavior-preserving internal classification, L0 first; no new public contract/default/phase.

Uncertainty/refuter: If false prevents compilation rather than only reuse in its call path, classification must be revisited. Do not infer purity of arbitrary Value from current shape.

## V16-097 — initial_layout_independent

Site: `03_infra/src/pipeline/paged.rs:123`; owner `00_nucleo/prompts/infra/pipeline/paged.md`; `Content` on `content`.

Classification: valid conservative negative dependency classification.

Evidence: 03_infra/src/pipeline/paged.rs:55-124: explicit terminals/containers/styles/Table/Grid/SetPage; fallback false; 03_infra/src/pipeline/paged.rs:528-529: two independent proofs gate candidate reuse.

L0: 00_nucleo/prompts/infra/pipeline/paged.md:70-78: explicit whitelist and every unlisted variant fails closed; Source delegates only body.

Complement: Current unlisted variants may contain context, callbacks, numbering, unknown geometry or unproved effects, and therefore keep the full cycle. False is not evidence that content is semantically invalid or render-empty.

Repair plan: Construct the nominal unproved set against exactly the L0 whitelist: preserve ContextBlock, residual ShowRecipe, Ref/Outline/Entry/Heading, all math, state/counter/bibliography, Dynamic/PageRun etc as false. Preserve current accepted recursion, Realized checking both original and visual, all Table/Grid slots and SetPage restrictions. No automatic true for simple-looking new variant.

Regression plan (not executed): Known positive physical tables/styles, negative header/background/Heading/context/Dynamic/recipe; rejected proof still uses full cycle with identical text/pages/geometry/errors/effect counts. Sentinel demands local decision.

Compiler-negative plan (not executed): In a disposable copy, add a sentinel to Content and require E0004 at this exact decision after repair; record baseline behavior and exact file/span independently of unrelated errors. For a tuple match test the first-axis new/same and new/new combinations; do not let another match’s error stand in for this site. For an external enum, use a disposable copied dependency with the pinned variants plus sentinel, never change the productive/vendor API.

Dependencies: Only this owner and the referenced enum owner; no ownership transfer.

Gate: ADR-0127 continuous flow for a behavior-preserving internal classification, L0 first; no new public contract/default/phase.

Uncertainty/refuter: A newly measured proof can justify a future whitelist expansion, but is not part of this lint repair. Reuse benefit or same final page count alone cannot prove independence.

## V16-098 — realize_outlines_for_pipeline same_carrier

Site: `03_infra/src/pipeline/paged.rs:219`; owner `00_nucleo/prompts/infra/pipeline/paged.md`; `Content × Content` on `(content,&first)`.

Classification: valid limited immutable-root continuity proof.

Evidence: 03_infra/src/pipeline/paged.rs:181-220: runtime realization once, exact Arc ptr equality for Sequence/Styled/Located/Realized; :221-236 fast return or prefix update/rerun.

L0: 00_nucleo/prompts/infra/pipeline/paged.md:58-62: those four root proofs only, otherwise previous path.

Complement: All unlike or nonqualified same-kind pairs lack this continuity proof and return false; they proceed to observed-prefix comparison, not loss of output. Address is not language identity.

Repair plan: Keep four Arc::ptr_eq bodies. Exhaustively partition the first tuple axis into qualified same-kind comparison and explicitly unqualified classes, preserving prefix observation/rerun/warning accumulation. Do not add more ptr_eq classes from convenience or substitute Content::eq/hash.

Regression plan (not executed): Four shared-root positives, rebuilt roots and unlike kinds false; prefix change runs second realization with warnings retained, error propagation unchanged; terminal no-op remains semantically correct via general path.

Compiler-negative plan (not executed): In a disposable copy, add a sentinel to Content × Content and require E0004 at this exact decision after repair; record baseline behavior and exact file/span independently of unrelated errors. For a tuple match test the first-axis new/same and new/new combinations; do not let another match’s error stand in for this site. For an external enum, use a disposable copied dependency with the pinned variants plus sentinel, never change the productive/vendor API.

Dependencies: Only this owner and the referenced enum owner; no ownership transfer.

Gate: ADR-0127 continuous flow for a behavior-preserving internal classification, L0 first; no new public contract/default/phase.

Uncertainty/refuter: A fallback path with visible extra effects can refute current reuse/general equivalence and needs its own evidence, not blanket return true. New/same sentinel must force this proof site to decide.

## V16-099 — collect_context_blocks_into L1 fallback visitor

Site: `03_infra/src/pipeline.rs:488`; owner `00_nucleo/prompts/infra/pipeline.md`; `Content` on `node`.

Classification: valid no-action callback complement after exhaustive delegated traversal.

Evidence: 03_infra/src/pipeline.rs:402-491: borrowed Sequence/Table/Cell, generic L1 map fallback, carrier/style reentry and insert overwrites; 01_core/src/entities/content/traversal.rs:436,667-673: Located callback suppression.

L0: 00_nucleo/prompts/infra/pipeline.md:28,52-59: exhaustive L1 descent, exact raw/resolved/standalone topology and captured chains.

Complement: At each residual ordinary node there is no direct producer/chain update; descendants were already visited. Outer other arm is a substantive delegated traversal, not no-op. Preserve reinsertion under Styled/ShowRecipe effective chain.

Repair plan: Nominally classify callback nonproducer classes without replacing outer L1 traversal with a partial L3 terminal table. Keep explicit Located/Realized/ShowRecipe/Styled reentry and exact map insertion/order. Check root callback suppression separately.

Regression plan (not executed): ContextBlock inside each borrowed/fallback container, nested style chain and ShowRecipe snapshot, Located root, Realized visual-only, Table raw/resolved/standalone choices and repeated ids. No loss/reexecution; compare collected chain not Debug.

Compiler-negative plan (not executed): In a disposable copy, add a sentinel to Content and require E0004 at this exact decision after repair; record baseline behavior and exact file/span independently of unrelated errors. For a tuple match test the first-axis new/same and new/new combinations; do not let another match’s error stand in for this site. For an external enum, use a disposable copied dependency with the pinned variants plus sentinel, never change the productive/vendor API.

Dependencies: Only this owner and the referenced enum owner; no ownership transfer.

Gate: ADR-0127 continuous flow for a behavior-preserving internal classification, L0 first; no new public contract/default/phase.

Uncertainty/refuter: If a current container is seen only in ordering and not this collector, the empty-by_id proof is false. Adding a no-op for such a carrier would hide the defect; use the existing L1 topology.

## V16-100 — substitute_context_blocks map callback

Site: `03_infra/src/pipeline.rs:529`; owner `00_nucleo/prompts/infra/pipeline.md`; `Content` on `node`.

Classification: valid unchanged complement; marker preservation critical.

Evidence: 03_infra/src/pipeline.rs:503-532: Located ContextBlock handled outside preserved marker; direct producer yields marker+result; otherwise None.

L0: 00_nucleo/prompts/infra/pipeline.md:57-59: original Located marker before result; never Sequence inside the same Located.

Complement: Nonproducer callbacks retain transformed node; Located(nonproducer) also unchanged. Missing resolved id intentionally contributes Empty result but retains the marker for later Position/PageStore.

Repair plan: Keep the two producer substitutions and exact result lookup/marker placement; enumerate ordinary unchanged classes and nested Located nonproducer cases directly. Do not erase unknown producers or move generated result inside the marker’s Located body.

Regression plan (not executed): Bare and Located ContextBlock with present/missing map id, nested producer descendants, Realized visual only, Source preservation, here/location position and single evaluation/effect count.

Compiler-negative plan (not executed): In a disposable copy, add a sentinel to Content and require E0004 at this exact decision after repair; record baseline behavior and exact file/span independently of unrelated errors. For a tuple match test the first-axis new/same and new/new combinations; do not let another match’s error stand in for this site. For an external enum, use a disposable copied dependency with the pinned variants plus sentinel, never change the productive/vendor API.

Dependencies: Only this owner and the referenced enum owner; no ownership transfer.

Gate: ADR-0127 continuous flow for a behavior-preserving internal classification, L0 first; no new public contract/default/phase.

Uncertainty/refuter: An unresolved producer reaching output is a separate preparation issue; do not fix it by broad Empty defaults. Structural sentinel at callback must be independent of other productive match failures.

## V16-101 — count_element_in_content

Site: `03_infra/src/query_helpers.rs:242`; owner `00_nucleo/prompts/infra/query_helpers.md`; `ElementKind` on `kind`.

Classification: valid public zero projection for non-document kinds, guarded in summary.

Evidence: 03_infra/src/query_helpers.rs:154-167: summarize_query selects content helper only for is_document_element_kind; 03_infra/src/query_helpers.rs:225-243: seven approximate document categories; others zero; 01_core/src/entities/element_kind.rs:16-109: full current kind set.

L0: 00_nucleo/prompts/infra/query_helpers.md:31-37: approximate projection, all other kinds zero, no located-value promotion.

Complement: Heading/Figure/Citation/Metadata/State/StateUpdate/Outline/OutlineEntry/Bibliography/Equation/CounterUpdate/Table/StateDisplay/CounterDisplay/ContextBlock all return zero here. The summary takes their actual counts from the introspector instead. Public direct helper call still has specified zero.

Repair plan: Name those fifteen non-document categories explicitly; retain List/Enum/Par/Link/Raw/Quote/Footnote logic. Cross-check is_document_element_kind so a future new kind cannot silently fall into zero through a mismatched gate.

Regression plan (not executed): Direct helper Heading/Metadata zero versus summarize_query positive introspector count; all seven document branches and unknown selector diagnostics. No fake locations or promotion of approximate paragraph/list results.

Compiler-negative plan (not executed): In a disposable copy, add a sentinel to ElementKind and require E0004 at this exact decision after repair; record baseline behavior and exact file/span independently of unrelated errors. For a tuple match test the first-axis new/same and new/new combinations; do not let another match’s error stand in for this site. For an external enum, use a disposable copied dependency with the pinned variants plus sentinel, never change the productive/vendor API.

Dependencies: Only this owner and the referenced enum owner; no ownership transfer.

Gate: ADR-0127 continuous flow for a behavior-preserving internal classification, L0 first; no new public contract/default/phase.

Uncertainty/refuter: A new kind that actually belongs to document projection requires explicit decision and measured semantics, not default zero. Existing L0 deliberately preserves direct helper negatives.

## V16-102 — count_list_groups

Site: `03_infra/src/query_helpers.rs:256`; owner `00_nucleo/prompts/infra/query_helpers.md`; `Content` on `content`.

Classification: documented approximate projection with reproducible empty/singleton/wrapper limitations.

Evidence: 03_infra/src/query_helpers.rs:246-258: Sequence all predicate, otherwise sum; item recurses body; remainder zero; 03_infra/src/query_helpers.rs:154-167,225-228: legacy summary path selects group helper.

L0: 00_nucleo/prompts/infra/query_helpers.md:33-37: retain current approximate group walkers and documented singleton discrepancy; compiled-query path separately owned.

Complement: Sequence consisting only of ListItem counts one group, otherwise recurse. Bare ListItem counts only groups in its body, so plain singleton gives zero. Every other current variant, including wrappers, returns zero under this bounded legacy model.

Repair plan: Do not blindly enumerate the old zero catchall and claim correct list semantics. First record focused empty/singleton/wrapper characterization and owner decision about this deliberately approximate API. If retaining the approximation for this bounded repair, explicitly name terminal and intentionally untraversed container classes with the limitation in L0/tests. Any semantic correction (nonempty group test, singleton group policy or added container descent) requires new measurement/L0 with actual grouping boundaries, not reusing generic map_content node counts.

Regression plan (not executed): Minimal static input is raw Content::Sequence(Arc::from([])), then count_element_in_content(..., List/Enum) → helper → all(empty)==true →1. Content::sequence(vec![]) normalizes differently and is NOT the same witness. Also bare item(text), Sequence[item], Styled(Sequence[item]), nested mixed content and real markup run. Existing L0 describes these as approximate/current; future oracle probes at bd5... are NOT executed and cannot be assumed to require one count.

Compiler-negative plan (not executed): In a disposable copy, add a sentinel to Content and require E0004 at this exact decision after repair; record baseline behavior and exact file/span independently of unrelated errors. For a tuple match test the first-axis new/same and new/new combinations; do not let another match’s error stand in for this site. For an external enum, use a disposable copied dependency with the pinned variants plus sentinel, never change the productive/vendor API.

Dependencies: Query summary/ElementKind and normalization/grouping owners. No change to compiled located query or public signatures.

Gate: Explicit preservation classification is continuous internal flow. A measured parity correction may flow L0-first; deliberate change to documented approximate public behavior/compatibility must be classified under ADR-0127 before implementation.

Uncertainty/refuter: The empty raw-Sequence value is a source-proven function-level result, not an executed CLI/oracle failure. A producer invariant excluding empty raw sequences refutes the CLI claim, not this total public-input observation. Owner’s documented preservation means no semantic fix may be smuggled in as lint-only.

## V16-103 — count_enum_groups

Site: `03_infra/src/query_helpers.rs:270`; owner `00_nucleo/prompts/infra/query_helpers.md`; `Content` on `content`.

Classification: documented approximate projection with reproducible empty/singleton/wrapper limitations.

Evidence: 03_infra/src/query_helpers.rs:260-272: Sequence all predicate, otherwise sum; item recurses body; remainder zero; 03_infra/src/query_helpers.rs:154-167,225-228: legacy summary path selects group helper.

L0: 00_nucleo/prompts/infra/query_helpers.md:33-37: retain current approximate group walkers and documented singleton discrepancy; compiled-query path separately owned.

Complement: Sequence consisting only of EnumItem counts one group, otherwise recurse. Bare EnumItem counts only groups in its body, so plain singleton gives zero. Every other current variant, including wrappers, returns zero under this bounded legacy model.

Repair plan: Do not blindly enumerate the old zero catchall and claim correct list semantics. First record focused empty/singleton/wrapper characterization and owner decision about this deliberately approximate API. If retaining the approximation for this bounded repair, explicitly name terminal and intentionally untraversed container classes with the limitation in L0/tests. Any semantic correction (nonempty group test, singleton group policy or added container descent) requires new measurement/L0 with actual grouping boundaries, not reusing generic map_content node counts.

Regression plan (not executed): Minimal static input is raw Content::Sequence(Arc::from([])), then count_element_in_content(..., List/Enum) → helper → all(empty)==true →1. Content::sequence(vec![]) normalizes differently and is NOT the same witness. Also bare item(text), Sequence[item], Styled(Sequence[item]), nested mixed content and real markup run. Existing L0 describes these as approximate/current; future oracle probes at bd5... are NOT executed and cannot be assumed to require one count.

Compiler-negative plan (not executed): In a disposable copy, add a sentinel to Content and require E0004 at this exact decision after repair; record baseline behavior and exact file/span independently of unrelated errors. For a tuple match test the first-axis new/same and new/new combinations; do not let another match’s error stand in for this site. For an external enum, use a disposable copied dependency with the pinned variants plus sentinel, never change the productive/vendor API.

Dependencies: Query summary/ElementKind and normalization/grouping owners. No change to compiled located query or public signatures.

Gate: Explicit preservation classification is continuous internal flow. A measured parity correction may flow L0-first; deliberate change to documented approximate public behavior/compatibility must be classified under ADR-0127 before implementation.

Uncertainty/refuter: The empty raw-Sequence value is a source-proven function-level result, not an executed CLI/oracle failure. A producer invariant excluding empty raw sequences refutes the CLI claim, not this total public-input observation. Owner’s documented preservation means no semantic fix may be smuggled in as lint-only.

## V16-104 — armed watch signature_mentions_id

Site: `03_infra/tests/armed_watch_contract.rs:255`; owner `00_nucleo/prompts/infra/tests/armed_watch_contract.md`; `serde_json::Value` on `value`.

Classification: test-only valid JSON leaf negative classification.

Evidence: 03_infra/tests/armed_watch_contract.rs:240-256: recurse Array/Object; only resolved_path.id via as_u64 matches target; 03_infra/tests/armed_watch_contract.rs:435: strict rustdoc public-signature inventory consumer; serde_json-1.0.149/src/value/mod.rs:116-175: Null/Bool/Number/String/Array/Object.

L0: 00_nucleo/prompts/infra/tests/armed_watch_contract.md:14-24: strict token exposure/public inventory, unsupported schema fails.

Complement: Null/Bool/Number/String contain no structured resolved_path node; a numeric leaf equal to target is not a type reference. Object resolved_path.id is authoritative, with recursive descent through values and arrays.

Repair plan: Replace wildcard with explicit four scalar variants, preserving recursive object/array and exact resolved_path/id/as_u64 detection. Do not loosen unknown rustdoc schema handling or scan strings/numbers for incidental target IDs. Keep test-owned independent helpers rather than moving them into productive Watch code.

Regression plan (not executed): Nested array/object positive; target number/string/plain id property negative; unrelated resolved_path negative; absent/malformed id and nested positive; retain full public-method/function allowlist and schema/version failures. Copied dependency sentinel compile negative.

Compiler-negative plan (not executed): In a disposable copy, add a sentinel to serde_json::Value and require E0004 at this exact decision after repair; record baseline behavior and exact file/span independently of unrelated errors. For a tuple match test the first-axis new/same and new/new combinations; do not let another match’s error stand in for this site. For an external enum, use a disposable copied dependency with the pinned variants plus sentinel, never change the productive/vendor API.

Dependencies: Pinned rustdoc JSON schema, external serde_json enum. Watch product API/purity/transition contracts unchanged.

Gate: ADR-0127 continuous flow for a behavior-preserving internal classification, L0 first; no new public contract/default/phase.

Uncertainty/refuter: If pinned rustdoc represents a reference outside resolved_path.id, the detector’s schema assumptions need evidence and failing controls, not wildcard-based acceptance. Missing rustdoc identity/version/schema must still fail closed.

## V16-105 — snapshot signature_mentions_id

Site: `03_infra/tests/watch_snapshot_contract.rs:189`; owner `00_nucleo/prompts/infra/tests/watch_snapshot_contract.md`; `serde_json::Value` on `value`.

Classification: test-only valid JSON leaf negative classification.

Evidence: 03_infra/tests/watch_snapshot_contract.rs:174-190: recurse Array/Object; only resolved_path.id via as_u64 matches target; 03_infra/tests/watch_snapshot_contract.rs:343: strict rustdoc public-signature inventory consumer; serde_json-1.0.149/src/value/mod.rs:116-175: Null/Bool/Number/String/Array/Object.

L0: 00_nucleo/prompts/infra/tests/watch_snapshot_contract.md:14-24: strict token exposure/public inventory, unsupported schema fails.

Complement: Null/Bool/Number/String contain no structured resolved_path node; a numeric leaf equal to target is not a type reference. Object resolved_path.id is authoritative, with recursive descent through values and arrays.

Repair plan: Replace wildcard with explicit four scalar variants, preserving recursive object/array and exact resolved_path/id/as_u64 detection. Do not loosen unknown rustdoc schema handling or scan strings/numbers for incidental target IDs. Keep test-owned independent helpers rather than moving them into productive Watch code.

Regression plan (not executed): Nested array/object positive; target number/string/plain id property negative; unrelated resolved_path negative; absent/malformed id and nested positive; retain full public-method/function allowlist and schema/version failures. Copied dependency sentinel compile negative.

Compiler-negative plan (not executed): In a disposable copy, add a sentinel to serde_json::Value and require E0004 at this exact decision after repair; record baseline behavior and exact file/span independently of unrelated errors. For a tuple match test the first-axis new/same and new/new combinations; do not let another match’s error stand in for this site. For an external enum, use a disposable copied dependency with the pinned variants plus sentinel, never change the productive/vendor API.

Dependencies: Pinned rustdoc JSON schema, external serde_json enum. Watch product API/purity/transition contracts unchanged.

Gate: ADR-0127 continuous flow for a behavior-preserving internal classification, L0 first; no new public contract/default/phase.

Uncertainty/refuter: If pinned rustdoc represents a reference outside resolved_path.id, the detector’s schema assumptions need evidence and failing controls, not wildcard-based acceptance. Missing rustdoc identity/version/schema must still fail closed.
