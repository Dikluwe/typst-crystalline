# L0 — Show rule data
Hash do Código: f07493d1



Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/textual-recipe-frontier.toml sha256:983b118684c0ddab1fbb3c619744957c2e1f93af7708b82c97f665e47410fbf3

**Layer:** L1, pure domain; no I/O or global state. **Exclusive consumer:** `01_core/src/entities/show.rs`.

## Evidence, ratifications and approval boundary

Historical rule-order measurement in f-recon-lacuna-ii-passo-357.md used upstream0.14.2 and found no accumulation of same-kind Func transformations:one applies if output changes kind,while same-kind output recurses/errors. Reference demand measured zero examples. The prior claim that vanilla accumulates Func was conflated with show-set style fold. The measured correction was innermost-first function order,not general accumulation. Earlier A2/A3 alternatives(accumulating where upstream errors,reopening morphological termination)were declined. Reopen only on a measured real package/document pattern requiring exact-order accumulation,then add parity tests;not by hypothetical demand. The old version-only record is historical,not certification of the current pinned target.

At HEAD2f42d64253547734564513a1159ee6b584c1c4b4 with no productive diff,show.rs:23–86 had NodeKind but no general native-function base;Text meant literal pattern,not function text. selector_matching.rs:43–65 rejected unconvertible identities. lab/results/construction/diagnostics/p1339-full-show-fixture-* measures historical ratified native strong/emph/text filters;final-*.json preserves state and nondefault-profile CLI limitation,not matching proof. NativeElement is explicitly named current by this owner's original introductory contract and exists in the consumer. Its historical additive-public-enum proposal still requires ADR-0127 approval evidence;none was located for this selector gate during consolidation. A separate approval assertion for ElementPayload NativeElement does not prove this ShowSelector gate. Preserve this unresolved approval-record gap;implemented code alone never ratifies it or authorizes additional changes.

The compact Styles-payload gate below is independently ratified. Textual matching/style applicability is independently ratified by `lab/history/construction/records/00_nucleo/materialization/textual-realization-2026-10-18/review.md`,acceptance.json,manifest47e714dfbb813eae9f2be20b8f57de5c197cda13db1871c8bb3da49a08430302. style-before.json.gz/unit-candidate3.log preserves rejection RED/color GREEN/full provenance. This ratification supersedes historical Style-on-Text/Regex rejection,per-node-only matching and eager pending-recipe realization only within textual scope;no new public type/field/method follows. Pinned textual-recipe-frontier governs it.

## Compact carrier evidence and approval

`lab/results/construction/diagnostics/clippy-enums-publicos-2026-10-16/before.json.gz` records HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/source/L0/hashes/status/diffstat. probe-details.json records rustc1.92/argv/rlib/binary/probe hashes/output; probe-result.json records12 GiB/swap0/one-job/events. These synthetic real-domain-carrier/registration/copy probes are not compile/layout/RSS benchmarks or a measured workload distribution.

Tag is416 bytes, Box/Arc Start candidates48. For4096 Start/End pairs with heap-free Heading payload, inline requests3407872 bytes/1 allocation, Box2031616/4097 and Arc2097152/4097. Box clone allocates per Start, but inspected productive fixpoint.rs:86–136 and entrypoints.rs:44–66 use temporary Vec<Tag>/borrowed reads without vector clone; walk:325–356 indexes by reference before moving info. Synthetic clone frequency is not asserted productive.

Transformation/ShowRule are328/376 bytes; Box/Arc Styles candidates120/168. rules/show.rs:464–470 copies to_vec per registration; apply.rs:108–112,349–352 filters/clones. Across128 simple show-set registrations without old snapshots, inline requests12276960 bytes/384 allocations, Box8194560/8640 and Arc5530624/512. Retained-snapshot peak requests are3203856,4141744,1477808. With custom styles, copying4096 rules requests4107 allocations inline,8203 Box and11 Arc including Vec growth. apply/intercept/outline/table_cells borrow styles; application still clones deltas when needed.

Keep the counterexamples: building one collection of4096 simple show-set rules requests1540096 bytes inline,2031616 Box,2097152 Arc; rich128-registration final live bytes without old snapshots are78064 inline versus156432 Arc because Arc can retain spare custom capacity. Do not claim universal gain or hide atomic/indirection cost. A relevant real-workload memory/time regression refutes the favorable Arc copy-cycle inference. This is storage mechanics under ADR-0030/0107, not language parity or measured AI cognition.

The owner approved manifest `282da3f8f79080a4831e1898879775e32a62caa0b2ffa941b8fca9198136b950` on2026-10-16, receipt `lab/results/construction/diagnostics/clippy-enums-publicos-2026-10-16/approval-receipt.json`. It satisfies the original ADR-0127 public-payload/compatibility gate for this package and T1–T6, without release. Additional signatures/fields/phase changes require another proposal.

## Data owned by this module

ShowRule is (id,selector,transform) stored in EvalContext;RuleId is public u64,unique per eval session/assigned next_rule_id at creation. Preserve Clone snapshots of Vec<ShowRule>/Arc<[ShowRule]> avoiding active borrow conflicts;this storage does not give entities evaluation/matching ownership.

NodeKind identifies native types by enum rather than names. Existing consumer inventory is Heading,Figure,Strong,Emph,Raw,Equation,ListItem,Underline,Strike,Overline,Smallcaps,Subscript,Superscript,Highlight,Link,Quote,Footnote,List,Enum,Par. The original shorter inventory and future-type annotations are historical;do not remove later existing variants or infer authority to add more.

Show Selector remains a static closed vocabulary:

- Text(String):literal text pattern,distinct from native text function. Str uses text replacement;Content/Func splice into matched ranges preserving surrounding text. Func receives matched Content::Text and emits Content or promoted Str once per occurrence.
- NodeKind(NodeKind):native kind selection;DynKind(String):user Dynamic kind name/interned dyn_kind,including registered callout FuncRepr::Element without native fn pointer. Both follow owning compiler's static traversal;DynKind is not native-function authentication.
- NativeElement(Func):native identity base,without executing Func. Existing Where composes filters over this base;do not duplicate public filter-group presentation here. QuerySelector/repr own that presentation. Preserve identity and Func equality;entity imports no compiler or builtin-address recognition.
- Regex(Regex):existing regex text-pattern carrier,not a new entity extension from consolidation.
- Where{base:Box<Selector>,field:EcoString,value:Box<Value>}:existing field filter,static not vtable/closure. The boxed Value reflects current recursive storage;the old unboxed example is historical. Matching requires base,known field and semantic Value equality through the canonical matching owner. Original native-NodeKind subset/dynamic-field exclusion is not permission to silently broaden unsupported bases;current NativeElement composition is separately specified by matching/conversion owners.
- Label(Label):name-based association interception,not ordinary node traversal. Retroactive label wrapper is formed after body interception;dedicated intercept_labelled applies only label rules,preventing duplicate other recipes. it is labelled body,not a new label node;replacement consumes wrapper. Latest-declared eligible rule applies once with revocation,no generic label revisitation loop.
- And(Vec<Selector>)/Or(Vec<Selector>):recursive left-to-right short circuit,all/any respectively;contradictory heading.and(figure) false,no error. Both empty false. Node traversal classification accepts a combinator only if all children are node-like. N-ary data supports chained a.or(b).or(c),without dynamic dispatch.

Canonical behavior owner:`00_nucleo/prompts/core/compiler/eval/selector_matching.md`;capture/apply owners are `00_nucleo/prompts/core/compiler/eval/rules.md` and its separately owned descendants. Data references do not share productive ownership. Query selector is separately owned by `00_nucleo/prompts/core/entities/selector.md`.

## Transformations and immutable style payload

Preserve Func(Func),Content(Content),Str(EcoString),Style(std::sync::Arc<Styles>). Func receives it and returns Content or Str promoted to text;Content is direct static replacement. These consume the selected show pass;Str remains literal replacement rather than arbitrary node-element transformation. Style is captured show-set:declaration does not mutate global engine.styles;actual matched content/product receives confined Styles through existing Styled carrier. Style does not consume/rewrite the element transformation pass.

Only compact Style payload supersedes historical Style(Styles) construction. External code constructs Style(Arc::new(styles));clones share immutable Styles instead of copying StyleDelta. &Styles reads deref normally;actual application still performs its existing needed delta copies in order. Arc::make_mut on a copy before push preserves original/custom styles and value isolation;no new shared mutability. Func/Content/Str,Selector,NodeKind,ShowRule,RuleId,other fields/signatures remain unchanged. No phase,extra captured state,precedence/guard/global-show-set/vtable change.

For element show-set+Func composition,match the entering element,not transformed output. Matched styles augment chain under which Func realizes,and wrap its output so style survives output-kind change. Multiple show-set declarations fold by existing collapse in declaration order,latest overriding;Func eligibility is tried latest-first(innermost). Reversing function order does not reverse style fold or authorize blanket accumulation. Same-kind fresh returns may depth-error;prepared it aliases retain protection under canonical occurrence/recipe owners. Existing during-call active_guards skip active RuleId to avoid nested reentry. Historical morphological fixpoint claims do not override separately ratified locatable/Sequence occurrence and depth contracts;legacy nonlocatable helper scope remains with `00_nucleo/prompts/core/compiler/eval/show_rule_termination.md`,not prescribed anew by this entity.

For Text/Regex,ratified Style(Arc<Styles>) is valid and transports actual match/product styles. Literal/regex share eligible-group/UTF8-earliest/lexical precedence and causal product guards under pinned nucleus,not historical individual-Text-node matching. Source/prefix/suffix and distinct scopes remain,Str/Content/Func payloads unchanged. Entity defines data only;capture/grouping/splice/normalization/matching/application remain compiler-owned. No new Clone/equality/hash/enum-budget/State contract or unplanned mechanics. V19 explicit grouping notes do not require duplicated bodies.

## Acceptance and limits

Test structural And/Or construction;heading.or(figure) yields Value::Selector(Or([Kind(Heading),Kind(Figure)])),and likewise And. Test positive Or for either type,positive heading.where(level:1).and(heading),contradictory And false. E2E show heading.or(figure):set text(red) affects those types not paragraph;contradictory And affects nothing. Preserve NodeKind/DynKind identity/recursive field semantic equality,static one-pass node snapshot traversal and no substituted-node reentry by the same consumed rule under current recipe protocol.

For native-function base,verify old/new paths on same node,including Par,not compilation alone. A measured native identity needing information unavailable from Func refutes sufficiency;reopen L0 rather than adding incidental public metadata. Preserve existing Where/group conversion,Label and all other variants;no new NodeKind/Content/interface/phase/order/revocation/recursion policy from this data extension. Unverified historical public gate above remains visible.

## Compact carrier acceptance

L0 and tests precede implementation; require real RED→GREEN and restrict resealing to changed owners. Do not silence large_enum_variant, raise thresholds or add dependencies/flags/defaults/phases.

T1: Box<ElementInfo>/Arc<Styles> construction and relative carrier budgets (Tag at most three Locations; Transformation at most Content plus alignment) fail before and pass after, without asserting universal ABI. T2: preserve Start/End/Location/payload/label, value equality/structural hash, independent Box clone and mutation isolation; hashes do not become persisted formats. T3: existing introspection/convergence/nesting/heading/label/state/counter/query/metadata tests retain expectations/errors. T4: Transformation::Style clones share Arc; Arc::make_mut on a copy preserves original styles including custom, and Func/Content/Str variants remain. T5: show-set/func, precedence/scope/labels/outline/table.cell/textual/regex preserve semantics/morphology/errors and execute no extra rules. T6: full workspace/build/all-target Clippy and previously closed expectations/lints as errors, Rustdoc -D warnings and four HTML indices, fmt/diff/strict+reverse lineage/no increased Tekt diagnostics under12 GiB/swap0/one-job. Record HEAD/tree/hashes/diffstat/UTC; distinguish type sizes/requested bytes from RSS. No release.

Ratified textual acceptance R3 and existing enum/lineage controls both remain required. Historical show-rule coverage/where wiring receipts and revision tables remain in `lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/show.md`;they do not establish full matching/parity or current universal termination by themselves.
