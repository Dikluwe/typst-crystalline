# Prompt L0 — `entities/func` — Functions and partial application
Hash do Código: 4a620daa

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/selectors/format-native-identity.toml sha256:a933254df6e90a505910759d443fe8b06f2646e2d4a9949bc179326fc9c5c329

Layer L1. Sole productive consumer: `01_core/src/entities/func.rs`.
Args belongs solely to `00_nucleo/prompts/core/entities/args.md`.
Applicable historical deferrals: ADR-0016 Routines/ADR-0017 complete eval;
language parity ADR-0107, atomization ADR-0109.

## Function representation and public interfaces

Func represents document closures, builtins and user-element constructors.
Func clone shares Arc O(1), consistent with Module. FuncRepr is crate-private;
public payload types keep their specified fields. Ratified internal Format
addition below extends the baseline enum without a new public type contract.

```rust
#[derive(Clone)]
pub struct Func(pub(crate) Arc<FuncRepr>);

pub(crate) enum FuncRepr {
    Closure(ClosureRepr),
    Native(NativeFunc),
    NativeWithEngine(NativeFuncWithEngine),
    Element(ElementFunc),
    Plugin(crate::entities::plugin_func::PluginFunc),
    With(Arc<(Func, Args)>),
    Format(FormatElementKind, NativeFunc),
}

pub struct ClosureRepr {
    pub name: Option<String>,
    pub params: Vec<ClosureParam>,
    pub body: SyntaxNode,
    pub captured: Arc<Scope>,
    pub capturer: Capturer,
}
pub struct ClosureParam {
    pub name: String,
    pub default: Option<Value>,
    pub pattern: Option<SyntaxNode>,
}
```

The Func tuple above is the historical baseline representation; private diagnostic
span and cfg-only carrier below are subsequent approved additions, excluded from
identity. SyntaxNode clone O(1) via internal Arc. captured is eager immutable
value snapshot at definition, unlike original comemo lazy access (recorded DEBT.md
divergence). Later parent redefinitions cannot affect closure. capturer records
normal Expr::Closure versus Expr::Contextual at definition, mirroring pinned
CapturesVisitor::new(scopes,capturer), not inferred at call. apply_closure propagates
it into call Scopes. Current owners:
`00_nucleo/prompts/core/compiler/eval/closures.md`,
`00_nucleo/prompts/core/entities/scope.md`.

Pattern only for non-Ident Param::Pos destructuring/parenthesized/placeholder;
owned SyntaxNode reparsed with Pattern::from_untyped/bound by destructure_let,
as body uses Expr::from_untyped. Ident positional/Param::Named pattern None;
Some pattern has name="" (empty-key named lookup cannot match), default None.
No new kind field needed: default None is positional Param::Pos; Some(v) means
keyword-only Param::Named, never consumes position. Parser ast.rs:2078–2085
already distinguishes them; closure producer has only these two construction
sites. Old apply_closure ignored invariant, letting keyword-only consume position;
corrected binding keeps it. Witness f(close:false)=close;f(true) yields
`unexpected argument`, not true.

### Pattern evidence and remaining diagnostic debt

Historical cetz path-util.typ:453 segments.enumerate().filter(((i,segment))=>...)
isolated silent non-Ident Param::Pos drop (_=>None), producing too few parameters
and `unexpected argument`. Pinned call.rs:655–665 uses generic destructure shared
with let/for. Measured witnesses:

```
#let f = ((a, b)) => a + b; f((1, 2))                    → 3
#let f = ((a, b), c) => a+b+c; f((1, 2), 3)              → 6
#let g(x, (a, b)) = x+a+b; g(10, (1, 2))                → 13
#let f = (_, y) => y; f(1, 2)                           → 2
#let f = ((a, ..rest)) => rest.len(); f((1,2,3))          → 2
```

Placeholder now consuming positional is an accepted measured vanilla-parity
change. Preexisting missing-position debt stays: binds Value::None instead of
missing argument; destructuring None errors `cannot destructure none` instead of
`missing argument: pattern parameter`. Not closed by pattern support.

### Native payloads and attached namespaces

```rust
pub struct NativeFunc {
    pub name: &'static str,
    pub call: fn(&mut crate::compiler::eval::EvalContext, &Args, &dyn crate::contracts::world::World, FileId) -> SourceResult<Value>,
    pub namespace: Option<Arc<Scope>>,
}
pub struct NativeFuncWithEngine {
    pub name: &'static str,
    pub call: fn(&mut crate::compiler::eval::EvalContext, &Args, &dyn crate::contracts::world::World, FileId, &mut crate::compiler::scopes::Scopes<'_>, &mut crate::entities::engine::Engine<'_>) -> SourceResult<Value>,
    pub namespace: Option<Arc<Scope>>,
}
pub struct ElementFunc {
    pub name: String,
    pub ctor: crate::entities::element_registry::ElementCtor,
}
```

Engine-native function receives current Scopes/Engine. Typed signature is the
original specification; documentary source-path drift, if any, is not an
authorization to change ABI. Attached namespace supports native field access
(table.header/footer/cell). None for natives without subfunctions (heading/figure),
Some scope for table/list/enum. Native/Engine constructors without namespace
initialize None, namespace variants attach supplied Arc. Namespace clones O(1),
field access delegates Scope. Closure/Element have no attached namespace in this
scope, Plugin none; no mutable namespace field access.

```rust
impl Func {
    pub fn closure(repr: ClosureRepr) -> Self;
    pub fn native(name: &'static str, call: NativeFn) -> Self;
    pub fn native_with_namespace(name: &'static str, call: NativeFn, namespace: Arc<Scope>) -> Self;
    pub fn native_with_engine(name: &'static str, call: NativeFnWithEngine) -> Self;
    pub fn native_with_engine_and_namespace(name: &'static str, call: NativeFnWithEngine, namespace: Arc<Scope>) -> Self;
    pub fn element(name: impl Into<String>, ctor: ElementCtor) -> Self;
    pub fn plugin(p: crate::entities::plugin_func::PluginFunc) -> Self;
    pub fn with(self, args: Args) -> Self;
    pub(crate) fn repr(&self) -> &FuncRepr;
    pub fn element_name(&self) -> Option<&str>;
    pub fn name(&self) -> Option<&str>;
    pub fn native_fn_addr(&self) -> Option<NativeFn>;
    pub fn namespace(&self) -> Option<&Scope>;
    pub fn set_name(&mut self, name: String);
}
```

## Identity, morphology and plugin boundary

PartialEq accepts shared FuncRepr Arc, or same native kind/name, extended below
to compatible Native/Format grouping. Historical P742 vanilla probes color.rgb==rgb
and red.space()==rgb true: original static singleton identity versus fresh
crystalline native lookup Arcs motivated name-based equivalence. That measured
snapshot is historical, not a version-string-only current target. Closures/
Element/With/Plugin retain Arc identity. Eq/Hash required for CounterUpdate::Func
in ElementPayload: coherent native kind/name hash, others representation prefix/
Arc pointer; execute no function or inspect captured scope. Diagnostic span/
observation carrier never identity. Clone O(1) shares closure content.
Debug exact `<function>`, never panic. Closure repr `(..) => ..` regardless of
name/parameters/body; native repr public name (rgb), unnamed native keeps
#function(...). Printing owner is
`00_nucleo/prompts/core/compiler/eval/repr.md`, no repr-derived equality.

PluginFunc is callable WASM export; dynamic export name cannot fit native pointer,
call delegates captured PluginHost. Func::plugin wraps Arc<FuncRepr::Plugin>;
name Some export (diagnostics/named rejection), native_fn_addr None, namespace
None. Application delegates call_plugin(p,args,span) through existing call owner;
plugin payload stays `00_nucleo/prompts/core/entities/plugin_func.md`, stdlib
`00_nucleo/prompts/core/compiler/stdlib/plugin.md`.

## Partial application and causal Args

With Arc<(Func,Args)> works for Closure/Native/Engine/Element/Plugin/nested With.
Each recursive level merges own preargs before new, delegates inner. Pinned
func.rs:149–159,359–362,380–394 uses same With shape and pre.items.chain(new.items).
Unified original occurrence vector versus crystalline positional/named views is
mechanics; losing occurrences is language defect. Keep complete Args owned solely
by `00_nucleo/prompts/core/entities/args.md`, not a duplicate interface/side span map.

Positional order pre then new. Named view uses final value while every pre/new
occurrence/origin stays causally ordered. Earlier map-overwrite-only hypothesis
was refuted. Func::with constructs new Arc wrapper without validating inner arity/
names/types or executing function. name/namespace delegate inner; native_fn_addr
None. table.with(columns:2).cell returns original namespace subfunction without
inheriting table preargs, as measured/pinned scope() func.rs:269–277. Only calling
parent combines args, so cell called normally. json.with(...).encode returns
original member, json.encode.with(...) binds encoder itself. Original encoders
repr encode, partial repr (..) => ..; preserve Func::name/namespace.

Bound .with interception on once-evaluated Func returns new Func, no call; every
Func variant eligible, non-Func falls through ordinary field error. Existing
where/or-and/within/collection/state-counter priority remains call owner:
`00_nucleo/prompts/core/compiler/eval/call_dispatch.md`. No automatic validation
change in other owners or new callable representation.

### Causal occurrence measurement and gate boundary

HEAD `b303f1f15b610e09872b567027e0d806387fde8c`, preserved P1306 working tree:
func.rs:39–42,237–238 already Arc<(Func,Args)>.
with-json-invalid-first-occurrence/TOML reject first pretty:"bad" even last true;
with-spread-arguments-f/g preserve distinct-origin errors after factory. Pinned
func.rs:372–374 concatenates without erasure. Views alone not divergence, lost
values/origins are. This revision changed no Func signature/representation;
private span revision below separate. Preserve clone/alias/return/eager capture/
Eq-Hash. Future witnesses: aliases/chained With retain origins, parent/encoder With
distinct, invalid With creation still function, only later call validates. Historical
ADR-0127 gate belongs to Args/encoders; textual ownership cleanup creates no new
consumer/independent approval.

## Private diagnostic origin

Historical baseline stored only representation Arc; closure lexical parameter
range absent, body not same range. Pinned func.rs:380–389 stores Span/set-if-detached,
:405–409 preserves in With; args.rs:410–412 anchors filter bool failure at test.span().
Function origin, not filtered-value origin, is necessary measured data.

Authorized private Span separate from Arc, helpers pub(crate)
diagnostic_span(&self)->Span and with_diagnostic_span(self,span:Span)->Self.
Second fills detached only. No public field/method; public constructors unchanged,
initialize detached. Clone copies span, With keeps inner function span, set_name
unchanged. Helpers reconstruct no origin via AST/World/equal values/names/pointers;
eval owners capture. Exclude span from Eq/Hash/Debug/repr/namespace/name/Arc identity.
No eager capture/parameter/Args/application change. Synthetic closures without AST
stay legitimately detached. Tests equal/aliased functions with preserved origins,
With/repeated calls/no Source. Authorized internal diagnostic transport, no public
API/phase.

## Cfg-only callback occurrence observation

Historical measurement transports outer With Func intact in CounterUpdate::Func,
inner dispatch target same cloned Func: minimal real carrier, no ContextBlockElem/
CounterRegistry/Value/ElementPayload change needed. Under cfg(p1339_observation),
Func has optional third private carrier: shared append-only handle/cell/occurrence
ID/origin span. Constructors None; clone/alias share handle; counter owner attaches
only actual focal callback. With preserves previous carrier, outer-wrapper attach
never changes inner. Cfg-only crate helpers install/read carrier/local real-object
IDs. No Eq/Hash/Debug/repr/name/namespace/diagnostic_span/Arc/dispatch/normal
serialization effect. Without cfg no field/helper. No stable-pointer expectations/
global side table/name/output reconstruction. Observation-only entity, not normal
public ADR-0127 contract; ends at real P1342 binding, not general tracing/P1340
retention/NT01–NT06/new product policy.

P1343 R3 blocker measured insertion boundary between namespace end and impl Func
closing brace: associated Func items only; impl Content/CounterUpdate fails
`implementation is not supported in traits or impls`. Closing/reopening braces/
unlisted container is invalid capsule materialization. Func owner declares only
private carrier/ledger types and associated methods of Func/carrier. No Content/
CounterUpdate inherent impl/method/reexport. Their local projections belong to
`00_nucleo/prompts/core/entities/content.md` and
`00_nucleo/prompts/core/entities/counter_update.md`. Keep cfg field/clone/alias/
identity/morphology/name/namespace/dispatch/normal API/no-cfg behavior. No extension
trait/macro/nonlocal impl/generic ownership workaround.

## Explicit native ABI lint exception

func.rs:317 C024, before.json.gz in type-complexity-2026-10-15 retains HEAD/
uncommitted tree/UTC/diffstat/sources-owners/hashes. red-clippy.log fails lint-as-
error before edit. Inspection separates positional tuples/repeated FontKey alias/
already explicit static contracts. Rust maintenance, no AI benefit/functional bug/
fresh parity measurement.

Preserve native_fn_addr public contract/static pointer literally. Only method
may use exact machine metadata:
`#[expect(clippy::type_complexity, reason = "native_fn_addr: ABI nativa explícita preservada")]`.
Portuguese reason is literal metadata. No trait object/new public alias/call wrapper/
ABI/identity/None/variant change. Continuous internal ADR-0127/no public resolved
signature/entity/default/phase; abbreviation only with identical expansion proof,
new public/incompatible type needs own gate. Nominal type_complexity exceptions
only, no module/impl/crate/global threshold; overrides generic ban only for three
justified batch items, not other correction cases. Old functional tests unchanged;
selected gate RED→GREEN/unfulfilled expectations as error; build/all-targets
Clippy/full suite/fmt/Rustdoc/lineage/restricted reseal. No alias-field-name mirror tests.

## Exhaustive representation and identity barriers

### Evidence before decision

func.rs:398 source/L0 read/hash-checked. tekt-enums-2026-10-16/before.json.gz
retains HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/
sources-L0-config/status/exact diffstat. Prior SARIF11 warnings/one V16 note across
seven owners; latent nonemitted fallbacks included. Rust patterns/enum evolution
mechanics, preserve observable result/fields/identity/errors; infer no upstream
intent/AI gain.

### Current nominal projections and acceptance

element_name only Element Some, Closure/Native/Engine/Plugin/With and later Format
None. namespace None Closure/Element/Plugin, Native/Engine/With retain behavior,
Format mirrors Native below. PartialEq after Arc shortcut/native pairs uses nominal
unlike-variant false axis, extended to Format by approved clause. Hash
Closure/Element/Plugin/With pointer fallback retains discriminant/Arc::as_ptr.
No new name/namespace identity, span excluded, no function/ctor/host execution.
Initial six-representation clone/hash matrix covers same-name natives/same kind,
cross-kind rejection/Element/With/Plugin with no host-ctor call, later extended
seven-way below.

Internal continuous ADR-0127/no enum-definition/field/trait/public-signature/
default/dependency/compatibility/phase change in barrier-only batch. Preserve body/
order/guard/owner, no catchall-hiding helper. Future variant forces compile error,
neutrality comment is no suppression. Precode local functional tests/scope V16 RED.
Disposable real-workspace future Color/ColorSpace/MathStyleKind/FuncRepr/StateUpdate
sentinels only in copied definitions. cargo check --offline --locked -p typst-core,
one job/shared artifact cache, record sources/hashes/argv/UTC/diagnostics. Each21
selected match baseline lacks local E0004; after demands local E0004 independently
of other copied errors. Unsentinelled compile/behavior tests approve real code;
negative probe is no parity proof. Productive source unchanged during commands.

Restricted direct-reverse reseal, no global dirty-tree repair. Build/workspace-
all-targets Clippy -D warnings/-D unfulfilled_lint_expectations/fmt/full suite/
Rustdoc -D warnings/four HTML indexes/strict V1/V5/V15/V26/reverse/SARIF no new
warnings/diff --check. MemoryMax12GiB/MemorySwapMax0/CARGO_BUILD_JOBS1/heavy
sequential, no historical fixture/tolerance relaxation. External V16 including
latent T027/T158/T160 remain open.

## Native-format identity evidence and application boundary

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree.
`lab/results/construction/diagnostics/native-format-catalogue-2026-10-17/before.json.gz`
retains UTC/status/exact diffstat/hashes; baseline-reuse.json proves source/
configuration/binary equality with prior3956-case/eight-profile final matrix.
before-batch.json.gz measures144 detailed cases; before-compat.json.gz456 name/
alias/namespace controls. Probes keep argv/binaries/output. Historical pin
`9f2b6e8715237cb086899a42873660fe744622e8`, executable SHA
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
Fifteen remaining routes reject where/kind only in Tekt. format.rs:17–27/96–146
shares native_format across five formats, so pointer alone cannot distinguish
schemas. pinned-schema.json.gz retains nine upstream declarations/blob/hash/
lines/elem attributes. FuncRepr is pub(crate), entities/func.md:21/func.rs:24;
retificacao-func-repr.md refutes prior claim L0 made it public.
public-contract-before.json fixes public signatures/reexports/structs. Internal
representation/parity classification, not new public contract or render-inferred
intent.

Add closed crate-private Copy/Eq FormatElementKind{Svg,Png,Pdf,Html,Bundle}
and private FuncRepr::Format(FormatElementKind,NativeFunc). Keep literal current
external fields/contracts of Func/NativeFunc/NativeFuncWithEngine/ElementFunc/ClosureRepr/
ClosureParam. Only crate-private native_format(kind:FormatElementKind,native:NativeFunc)
->Self creates new variant/detached diagnostic span. No new public interface/ABI
alias/wrapper closure/side table/shared global allocation.

name/namespace/native_fn_addr Format mirrors four-argument Native payload.
element_name None both, reserved dynamic Element; set_name Closure only.
Arc clone O(1), With keeps name/namespace/span/native_fn_addr None. Public Eq
groups Native/Format on both axes with same-name comparison. Hash same prefix0/
name; Engine prefix1, rest prefix2/Arc. kind/span excluded from public Eq/Hash.
Recognition catalogue owns witness-based element capability. Existing
native_fn_addr Some/pointer/None preserved for every prior value: factories Native
→Format keep exact pointer, other six categories unchanged. RED/GREEN sizeof
Func/FuncRepr/NativeFunc/NativeFuncWithEngine printed to measure cost, no Rust-layout parity.

### Acceptance and scope

Consume pinned format-native-identity nucleus. Native/Format may share body
because payload/ABI/projection/execution are same; independently exercise both
in name/namespace/pointer/Eq-Hash/clone-With/dispatch tests. Inventory new V19
unions/link tests, no suppression. Explicitly name new variant in closed matches.
Continuous ADR-0127: no new external definition/signature/field/trait,
compatibility/default/phase. Before code, real functional RED for15 routes via
current factories plus real observation against unmarked Native. GREEN requires
full corpus/144 field-call cases/456 compatibility controls unchanged outside
where/kind. Compare public-contract-before/after and sizeof before/after.

L0 before code, ownership/V26 before restricted11-pair reseal. Build/full suite/
strict Clippy-Rustdoc/fmt/V1/V5/V15/V26/reverse/global differential without new
error-warning. Heavy tasks sequential MemoryMax12GiB/High10GiB/swap0/jobs1.
Track legitimate V19 with coverage. Closes where/kind admission only, not all
selection/show/query/synthesis/carrier morphology/counter. FG1/RM1/SY1/RG1/prior
V16 retain scope/gates; copy no old candidate or publish.

## Direct Format alternative coverage

candidate-1/state.json.gz/differential-review.json in native-format-catalogue-
2026-10-17 record eight new V19/four updated. Updated groups include Format/raise
count, not removed violation; initial exact-message comparator incorrectly rejects
update. Direct complementary tests only, no productive change/note suppression/
single-path group proof. Extend six-way matrix to seven: Format same name/
namespace, Native-compatible Eq-Hash, rejection versus Engine/Closure/Element/
Plugin/With, native_fn_addr/namespace per alternative. Five kinds with same name/
panicking-if-called callback must have distinct schemas by witness, while ordinary
Native/With lack capability. Execute no callback.

## Verification witnesses and scope

```
Func::closure(...).clone() == Func::closure(...)  → false
let f1 = Func::closure(...); let f2 = f1.clone(); f1 == f2  → true
format!("{:?}", func) → "<function>"
Args::positional(vec![]).is_empty() → true
Args::positional(vec![Value::Int(1)]).len() → 1
calc.round.with(digits: 2)(3.14159) → 3.14
{let g(a,b,c)=a+b+c; g.with(1,2)(3)} → 6
{let h(a,named:10)=a+named; h.with(named:20)(5)} → 25
{let f(a,b,c)=a+b+c; f.with(1).with(2)(3)} → 6
{let h(a,x:10,y:20)=a+x+y; h.with(x:100).with(y:200)(1)} → 301
type(table.with(columns:2)) → function
type(table.with(columns:2).cell) → function
```

Earlier pattern witnesses remain required. Namespace mutating set unsupported;
attached namespace not granted to Closure/user Element. With collisions are measured
causal-Args controls, no implicit validation changes to other owners.


## Fallible Source and file-registration integration

Human approval on 2026-10-22 is retained in
`lab/results/panic-elimination-2026-10-22/source-contract-approval.json`;
its review SHA-256 is `2c879e8afa66721a8952dc8b9e7cf3c92dce58bf91aae5e28da5d603f5aa4d42`.
Source::{new,new_with_parser,detached,detached_with_parser} now return the existing
SourceResult<Source>; SystemWorld::register_file returns FileResult<FileId>.
Preserve this unit's successful content/id/span/order/diagnostic behavior while
propagating construction/registration failures through its existing error channel.
A tuple-returning entry returns its error in the existing result component and
retains already accumulated warnings. A synthetic source slot stores a failed
construction as FileResult rather than inventing a Source. Test-only fixture
helpers must explicitly require their valid construction/registration result;
all previous substantive assertions remain. No productive unwrap/expect, panic
catch, default Source, identity reuse or silent error is permitted for this migration.
This clause supersedes only the old infallible return/panic expectations for the
five approved APIs; it does not authorize unrelated signatures, fields, phases
or broadening/loss of existing diagnostics or negative controls.
