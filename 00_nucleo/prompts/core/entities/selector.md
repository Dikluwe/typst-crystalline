# Prompt L0 — `entities/selector`
Hash do Código: 45965d51

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/selectors/native-bare-identity.toml sha256:7e7373acce2b00247d18b9c5ea1b9d53833275997cd2c304e9d919d2cce10648

**Layer:** L1, immutable pure selector data. **Exclusive consumer:** `01_core/src/entities/selector.rs`.

## Evidence and distinct approval states

The original minimal Kind-only specification and revision tables are historical,not a reduction of today's selector domain. Source inventory verified during consolidation includes Kind,NativeElement,Element,Label,Location,And,Or,Regex,Where,Within,Before,After. Preserve existing Debug/Clone/PartialEq/Hash;the later Value/NaN-bearing contract explicitly supersedes the old Eq derive. Entities import no compiler/AST/introspector or function execution. Representation is not blanket language parity.

Historical public type measurements in `lab/results/construction/diagnostics/superficie-linguagem-p1140*` at upstreama51e02804 found repr(type(selector))"function" versus"type";postatomization catalog/22public probes matched structural baseline byte-for-byte. The resulting public global selector is callable Value::Type(Type::Selector) through static dispatcher/existing L1 constructor,type(selector)==type/constructed type_name()=="selector",without entity-operation change. This is public semantics,not required Rust mechanics/current-oracle certification.

At HEAD2f42d64253547734564513a1159ee6b584c1c4b4/no consumer diff,selector.rs:28–61 could not represent native function plus ordered field group,distinguish empty filter from bare or identify Strong/Emph/Text through ElementKind. Historical strong.where(:)/text.where(text:"Hi") failed. Probe UTC2026-09-10T00:04:21.279738+00:00–00:04:21.402736+00:00 records sources/state/results:explicit empty filter differs from selector(element),aliases preserve equality,field reorder changes equality/repr,spread replacement keeps first key position. Upstreama51e02804 selector.rs:75–81,309–321 preserves identity/group presence/order;func.rs:430–450 validates element/field names. This supports Element data proposal,not approval by observation.

The original public Element proposal requires ADR-0127 because an added variant breaks exhaustive external Rust matches. No verified receipt for that selector gate was located during consolidation,although current code contains Element. Preserve this approval-record gap explicitly;do not infer ratification from code or from the distinct ElementPayload NativeElement approval assertion. Its contract/acceptance below remains preserved,not silently promoted into a new authorization.

The historical Before/After proposal likewise records a baseline ending in Within and measured rejected methods despite upstream base+limit+inclusive(defaulttrue). Current code already contains both variants,but no verified public-gate receipt was located. Earlier no-edit/before-after exclusion was narrowed by this explicit proposed design only;unverified approval is still pending reconciliation. Do not simulate Before/After through Within/And/Or/Where or treat such substitution as preserved behavior.

Bare Cell selector is a separate verified package. `lab/results/construction/diagnostics/math-residuos-table-cell-2026-10-06/{before,table-additional-before,table-aliases-before}.json.gz` preserves HEAD06848cdcb5450f2c555626bf116bc7acd82bd82b/tree/UTC/diffstat/inputs/commands/source/executable hashes,historical9f2b6e8715237cb086899a42873660fe744622e8. selector(table.cell)→cell,where()→cell.where(:),equalityfalse,query/counter reject nonlocatablecell. Real CodeBlock alias preserves this;previous document-only alias unavailable to eval was invalid positive evidence. Upstream selector.rs:309–321 separates dictNone/Some,:421–450 validates locatability independently. Local then only Element-present-group/catalog omitted cell;d08-owner-audit.json preserves other limits/hashes. Source distinguishes states explicitly,not inferred intent. Closed-carrier sufficiency is refuted by a public absent-group/identity case it cannot represent,no Cell realization assumption.

Human confirmo at2026-10-06T22:36:30.591466+00:00 approved manifestf80f85c4ac3ab747d20d98ef3f5758c59727bcf5f5d762082660099aef4dcc9c in `lab/history/construction/records/00_nucleo/materialization/table-cell-selector-aprovacao-2026-10-06.json`,ten promoted owners. This satisfies only bare-selector package's pending gate,not unrelated earlier Element/Before/After receipts or all Cell realization.

## Existing immutable data vocabulary

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Selector {
    Kind(ElementKind),
    NativeElement(crate::entities::func::Func),
    Element {
        function: crate::entities::func::Func,
        fields: ecow::EcoVec<(ecow::EcoString, crate::entities::value::Value)>,
    },
    Label(Label),
    Location(Location),
    And(ecow::EcoVec<Selector>),
    Or(ecow::EcoVec<Selector>),
    Regex(Regex),
    Where { base: Box<Selector>, field: EcoString, value: Box<Value> },
    Within { base: Box<Selector>, ancestor: Box<Selector> },
    Before { base: Box<Selector>, end: Box<Selector>, inclusive: bool },
    After { base: Box<Selector>, start: Box<Selector>, inclusive: bool },
}
```

This inventory preserves currently represented data while the approval-record distinctions above stay explicit. The old unboxed Where.value example is superseded by existing Box<Value> recursion boundary;Regex/Within are not omitted. No new ElementKind/NodeKind,Func/Value field,trait or extra public method is authorized here. Reexport existing selector module through entities/mod.rs;consumers use their own owner contracts.

Kind carries existing ElementKind including the later List,Enum,Par,Link,Raw,Quote,Footnote additions,not only initial Heading/Figure/Outline. Label carries exact Label string identity;Location exact opaque singleton identity. And/Or use EcoVec with shared O(1)clone,order-sensitive structural comparison/recursive hashing. Distinct variants remain distinct even when payload looks equivalent:Kind(Heading)not Label("Heading"),And(v)not Or(v),Where not bare Kind.

Where preserves base/field/value and nested multi-field chains. Producer builds one Where per field,inner field1 then outer field2;show matching recursively ANDs predicates. Legacy Where requires base+known field+semantic Value equality(including existing Int1/Float1.0 language relation),not name/Debug/hash approximation. Public language equality belongs to `00_nucleo/prompts/core/compiler/eval/operators/equality.md`;do not change Func/Value/Selector derives to implement it or add Eq over NaN-bearing Value. Deterministic field/discriminant hashing is internal,not a new persistent hash format.

NativeElement(Func)represents absent where group;Element{function,fields}represents explicitly present group,including empty. Empty group never collapses to Kind,And([]),Regex,Label or sentinel. Func is actual producer-recognized element identity,not callback to execute for type/field discovery. Fields are ordered unique names/values after producer normalization;spread replacement preserves first key position. EcoVec breaks Selector↔Value storage recursion,not a PropMap,type registry,vtable or dynamic-dispatch mechanism. Preserve empty/order/value/alias through clone/composition. Bare native recognition uses actual typed identity,no alias-name discriminator or constructor call.

Before/After data preserves full base/end-or-start/inclusive,clone/structural equality/hash,order/nesting without normalization. Public proposed default inclusive=true is fixed by language caller;Box is storage mechanics,not language semantics. Current query consumers already implement these operations;their existence does not resolve missing public approval record. No phase/default/compatibility decision follows from this documentation consolidation.

## Query and show boundaries

`00_nucleo/prompts/core/compiler/introspect/queries.md` owns actual queries over occurrence indexes;`00_nucleo/prompts/core/compiler/eval/selector_matching.md` owns conversion/show matching. Entity stores predicates only. Kind delegates kind index;Label query delegates query_by_label's compatible0/1return,Location yields singleton. And queries intersect subresults in first-vector order;empty And→vec![],the chosen explicit empty-domain policy rather than inferred universal set. Or returns deduplicated union preserving first appearance;empty Or→vec![]. Recursive nested composition retains these semantics. Native-element query ordering additionally follows canonical recorded causal order,never raw Location integer.

Legacy Regex and Where query arms remain explicit empty-vector limitations(verified queries.rs:167,173),not successful feature parity;Element native ordered-field queries are a distinct path under their owner. Do not accidentally treat the old Where stub as all native-filter behavior or silently add query-by-text support. Show Where may work while query Where remains limited. Earlier Rust-API-only composite exposure was narrowed by later language selector.and/or/within methods over existing variants,without an entity redesign. Counter/query validation independently checks every leaf's locatability;building a selector alone does not require it and never inserts table.cell into index.

The public Element proposal covers only where data boundary,not all ten surface routes. Before any additional materialization,each repr/query-introspector/counter consumer needs current L0;no silent wildcard/unconditional empty result/filter loss. Approval of one entity does not authorize pending consumer code. Existing native-filter acceptance requires measured public construction/repr/equality/application,not structural tests alone. If Func+ordered group cannot recover a measured identity/filter,reopen specification rather than add incidental public metadata.

## Acceptance and remaining scope

Preserve Kind same/different-kind equality/deterministic hash,Label exact-string equality,Location same/different identity,cross-variant inequality;And/Or structural order/empty data,empty query,ordered intersection/first-appearance union/nested recursion. Where identical base-field-value equal,each changed component different,distinct from Kind,deterministic hash. Test full public Element empty/ordered/replaced fields/aliases through clone/composition and construction/repr/language equality/application against recorded probes. No API-only structure test closes function.where.

Bare Cell package requires atomic confirmed-owner promotion,linguistic tests/valid RED before static safe Rust,valid1:1/V26 before restricted manifest seals/build/core suite/lint zero. Revalidate bare/where/true alias equality/direct and composite query-counter/homonym/partial/closure plus existing D05/D07/query controls. Carrier only transports Func/group absence,no compiler/introspector import/evaluation,new Cell ElementKind/NodeKind or derives defining linguistic equality.

Its bounded scope is carrier/construction/repr/equality/composition/locatability. Direct Cell show,body/field projection,explicit/implicit preparation,position/padding/spans/header-footer/lexical scope required separately measured owners and any public/phase gates. Later independently ratified Cell packages do not imply universal table parity or authorize hidden negatives. General filter/locatability/reflection/error limits remain visible. Preserve snapshots/effects/diagnostics and public pending-gate distinctions rather than infer acceptance from current Rust.

Archive: `lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/selector.md`.
