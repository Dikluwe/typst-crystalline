# Prompt L0 — `entities/plugin_func` — callable PluginFunc and memoization
Hash do Código: c94f4ab8

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
entities→compiler import. PluginFunc derives Debug/Clone and privately captures
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

Manual PartialEq/Eq/Hash use exactly(module,name), excluding host consistently.
Memoized public call(&self,args:Vec<Bytes>)->Result<Bytes,PluginError>
delegates host.call(module,name.as_str(),&args). This preserves equality→hash
and repeated-call cache semantics under the identity assumption below.

Explicit refutable assumption: module IDs are meaningful only in their issuing
host; two hosts reusing the same ID/name/args collide in comemo's process-shared
cache. The original 'one host per World' rationale does not itself prove
uniqueness across Worlds. Preserve this known collision risk as pending
identity-scope work, not a new guarantee. Test-only distinct export names/args
avoid harness collisions but do not establish language correctness across
hosts. No identity repair is authorized by editorial consolidation.

## Transition contract

Memoized public transition(&self,Vec<Bytes>)->Result<Module,PluginError>
invokes host.transition(module,name,&args), obtains exports from the derived
ID and builds Module::new("plugin",scope) with one PluginFunc using the same
host/new ID per export. Calls on derived module observe mutation; original
module remains unchanged. Repeated identical(module,name,args) transitions
invoke host once and reuse the same derived ID. Host exclusion has the same
identity assumption/risk as call. Return Module/PluginError must satisfy
Clone+Send+Sync+'static, checked by build; use canonical entity Module without
a new layer import.

## Acceptance and boundaries

Callable bytes→bytes behavior, one function per export, bytes-only validation
and repeated-call semantics are language requirements. Captured trait object,
Arc representation and cache-key implementation are chosen mechanics, not
required copies of upstream internals. Compile in L1 using only approved Arc/
EcoString/domain contracts, no wasmi or I/O; V3/V4/V14 clear. Memoization must
compile with manual Self Hash and required input/return bounds.
CountingHost with AtomicUsize called twice using identical bytes increments
once; nonbytes reject and bytes return Value::Bytes. Preserve transition
original/derived behavior and error propagation. Canonical lineage and
crystalline-lint validation remain required. The original human reseal schedule
is archived, not a new requirement to pause this editorial change.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/plugin_func.md`. Historical observations are not a new parity certification.
