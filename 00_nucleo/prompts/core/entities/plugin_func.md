# Prompt L0 — `entities/plugin_func` — callable PluginFunc and memoization
Hash do Código: ddfc1a0b

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/plugin_func.rs`
**Dependencies:** `00_nucleo/prompts/core/contracts/plugin_host.md`,
`00_nucleo/prompts/core/entities/func.md`,
`00_nucleo/prompts/core/compiler/stdlib/plugin.md`.
**References:** ADR-0107/0108/0109.

## Callable domain

Each WASM function export becomes a PluginFunc bound in the plugin Module
scope, corresponding to historical upstream foundations/plugin.rs:205-231,
366-380. The entity lives here so FuncRepr can reference it without a reverse
entities→compiler import. PluginFunc has manual Debug and public fields capturing
host:Arc<dyn PluginHost>,module:PluginModuleId,name:EcoString. Host's Send+Sync
supertraits are required because callable Values travel through Send+Sync
World contexts; calls do not retrieve it from Engine/World.

The Func owner supplies FuncRepr::Plugin(PluginFunc), public Func::plugin,
name()->Some(export name),native_fn_addr()->None,namespace()->None.
The static apply_func branch delegates to call_plugin. Validate every args.items
as Value::Bytes; otherwise report
`plugin function `{name}` expects bytes arguments, found {tipo}`.
Collect Vec<Bytes>,call p.call; return Value::Bytes or
SourceDiagnostic::error(Span::detached(),PluginError.message) verbatim.
Exact error propagation is observable; the original claim that every message
is merely mechanics does not authorize changing it.

## Measured memoization contract

The original comemo0.4.0 source audit (lib.rs:95, macro memoize.rs,input.rs,
cache.rs) established: hashed &self requires Self:Hash, Vec<Bytes> uses the
Hash→Input blanket, return must be Clone+Send+Sync+'static, not Hash.
Result<Bytes,PluginError> satisfies this with Clone PluginError.

### Measurement before decision (2026-11-05)

At uncommitted HEAD `3c6b0ce7b7632cc74b004cea6681e4746e63d28d`,
`plugin_func.rs:83-98` excludes host from equality/hash. The independently
executed public tests in `lab/experiments/catalog-tests/tests/c25_plugin_namespace.rs`
reproduce collisions between hosts with the same local module/name/args,
including real WASM, errors and transitions. The actual watch test
`tests/watch_plugin_identity.py` publishes A after module replacement by B.
See `00_nucleo/diagnostics/code-performance-tests-2026-11-05.md` for the
measurements and exact incoming state. A fresh load in a different host is
not the same language function merely because its host-local ID coincides.

Comemo 0.4.0 `src/input.rs:45-71` hashes ordinary inputs with unit constraints;
`src/cache.rs:23-81` stores constraints/output rather than retaining hashed
inputs. Thus adding a bare host pointer alone does not prevent address reuse
once a call's host is dropped. This second source check limits the repair.

### Classification and corrected identity

Result bytes, error messages and transition isolation are language observables;
Rust Eq/hash and Arc identity are mechanics (ADR-0107). Correct the internal
identity without changing any entity field, public signature, public trait,
dependency, mode or phase. This follows continuous ADR-0127 flow. The repair
is refuted by cross-host result/error reuse, stale identity after host release,
or loss of same-host repeated-call/transition memoization.

Manual PartialEq/Eq/Hash distinguish the host allocation as well as module and
export name, consistently. Arc clones of the same host retain identity; two
separate hosts never share a memoized call or transition even if local IDs,
export names and argument buffers match. Do not key by vtable, host Debug,
module ID alone, or bytes that the domain cannot inspect.

Keep public call(&self,args:Vec<Bytes>)->Result<Bytes,PluginError> and transition
signatures and public fields unchanged. Private memoized helpers may retain
an Arc to the issuing host alongside the cached Result, including errors.
That retained owner must survive for the entire lifetime of the cache entry,
preventing its numeric address from being recycled while the key is reusable.
The public wrapper returns only the original Result. Comemo eviction releases
retention; do not introduce a global registry/counter/allocator, trait method,
new public namespace field or retain hosts forever outside comemo. Retain
same-host/module/name/args one-call memoization, full args discrimination and
verbatim errors. `host.call(module,name.as_str(),&args)` remains the operation.

## Transition contract

Memoized public transition(&self,Vec<Bytes>)->Result<Module,PluginError>
invokes host.transition(module,name,&args), obtains exports from the derived
ID and builds Module::new("plugin",scope) with one PluginFunc using the same
host/new ID per export. Calls on derived module observe mutation; original
module remains unchanged. Repeated identical(host identity,module,name,args) transitions
invoke host once and reuse the same derived ID. The same host identity and cache-lifetime retention rules apply to transition. Return Module/PluginError must satisfy
Clone+Send+Sync+'static, checked by build; use canonical entity Module without
a new layer import.

## Acceptance and boundaries

Callable bytes→bytes behavior, one function per export, bytes-only validation
and repeated-call semantics are language requirements. Captured trait object,
Arc representation and cache-key implementation are chosen mechanics, not
required copies of upstream internals. Compile in L1 using only approved Arc/
EcoString/domain contracts, no wasmi or I/O; V3/V4/V14 clear. Memoization must
compile with manual Self Hash and required input/return bounds.
Acceptance includes cross-host AB/BA with identical local IDs/exports/args,
errors and missing exports, derived-module calls with the correct host, Arc
clones of one host, sequential host release/recreation, and actual WASM watch
module replacement. Demonstrate RED before the repair. Retaining the host
in a cached result must cover failures as well as successes; verify release
with cache eviction in the laboratory test
`lab/experiments/catalog-tests/tests/c25_plugin_lifetime.rs`; subprocess creation
and environment access stay outside L1. Pure regression tests remain inline.
Reject targeted mutations dropping the namespace or
lifetime protection. Update the old host-excluding equality test because its
assertion contradicts this corrected identity; do not remove memoization tests.
CountingHost with AtomicUsize called twice using identical bytes increments
once; nonbytes reject and bytes return Value::Bytes. Preserve transition
original/derived behavior and error propagation. Canonical lineage and
crystalline-lint validation remain required. The original human reseal schedule
is archived, not a new requirement to pause this editorial change.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/plugin_func.md`. Historical observations are not a new parity certification.
