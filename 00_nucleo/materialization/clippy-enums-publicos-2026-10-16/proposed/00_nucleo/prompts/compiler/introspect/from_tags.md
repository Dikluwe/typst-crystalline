# L0 — introspect/from_tags
Hash do Código: 175c75ac

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24
```

## Ownership and evidence

Layer: L1. Sole consumer: `01_core/src/compiler/introspect/from_tags.rs`.
The current module has no from_tags constructor. Delegate filtered and ordinary counter replay to `counter_updates.md`, state function replay to `state_updates.md`, state/counter display to `displays.md`, and equation realization to `equations.md`. This consumer remains the import boundary and retains its tests. Its productive bodies are
resolve_filtered_counter (`:36`), apply_state_funcs (`:123`),
apply_counter_funcs (`:157`), bundle Page replay (`:185`), state/counter
display passes (`:310`, `:411`) and equation passes (`:462`, `:523`, `:570`).
`01_core/src/compiler/introspect.rs:1376` builds stores during the walk.
Both runtime entrypoints and the bounded helper coordinate these passes;
none of that construction belongs to this child.

## State and ordinary counter updates

Keep state-function evaluation after the walk, with Engine/EvalContext and an
empty local Scopes whose closures retain their lexical captures.
For each StateUpdate::Func Start, read the state at its Location and apply
the function to that value as one positional argument. Store a successful
result at the update Location. Propagate callback diagnostics; an update
without an available initialized value is ignored. The pure walk does not
evaluate Func.

Ordinary counter Func receives the current components as separate positional
integer arguments, not a single array argument. Missing state uses [0].
Accept a non-negative integer or an array of non-negative integers as the new
complete state; reject other returns with the existing diagnostics.
Store Set at the same Location and propagate errors. Do not promise a causal
replay correction for all ordinary keys: the observed implementation appends
function results after deterministic walk snapshots.

For bundle runtime, clear and rebuild only Page snapshots, then replay each
body boundary Page = [1] and its Page Set/Step/Func in source order.
A Page function receives its own body's state; later actions see its result,
and the next body starts at 1. Other keys retain their existing paths.
Do not execute a callback twice or conceal an error. Boundary/seed collection
belongs to `00_nucleo/prompts/compiler/introspect.md`.

## On-demand filtered counter resolution

For a selector key containing Element, use the immutable event log, not
differences between previously computed counter snapshots.
Select automatic events by the concrete selector query and manual events by
language counter-key equality. That equality is owned by
`00_nucleo/prompts/compiler/eval/operators/equality.md`; derived Rust equality
is not a substitute. Sort selected events by Location and fold from [0].
Apply Step(level), Set and Func in causal order, retaining owned history.

Resolve the entire selected sequence before a caller chooses an at/get
prefix or final state. An invalid callback later on the same key can therefore
fail an earlier get. Unrequested keys are not eagerly executed merely to
prepare a contextual read. The ordinary global counter-function pass skips
these Element-containing keys.

Each replay Func uses a new EvalContext without introspective context,
preserving the observer's target/features and closure captures. Pass components
as separate positional arguments and validate the same return domain as
ordinary updates. Capturing a Counter is allowed; querying one from inside
this callback must not inherit the outer contextual read or recursively
initiate contextual replay. Do not mutate the original event log or public
eval snapshot into artificial Sets. Cache/optimization must not change error
ordering or observed effects without its own evidence and specification.
Context-generated actions and stabilization are coordinated by eval/pipeline
owners, not by a global preparation pass here.

## Display conversion

After update passes, state display reads its value at (key, Location), using
None when absent. Its callback receives that single value.
Without callback, Content passes through, LocatedContent explicitly converts
with into_content(), and String becomes text; other values become Empty.
With callback, Content/String become display content and other successful
returns or errors become Empty, preserving the current defensive behavior.

Counter display callbacks receive one array of integer components (empty when
absent), unlike update callbacks' separate arguments. Without callback, join
existing components with dots; missing state becomes Empty. Callback
Content/String succeeds; other returns/errors become Empty.
Store realized display Content by (typed key, Location), never execute these
callbacks in physical layout or rewrite Values held in state, arrays or closures.

## Equation realization and snapshots

For block equations whose captured numbering gate is active, obtain the
location-aware number. Format a captured string pattern (numeric fallback on
format failure), or call the numbering function with that single integer.
Require Content/String from numbering callbacks and propagate errors.
Store realized number Content by Location; keep the raw Func/pattern view.

Resolve supplement from the captured target specification: Auto uses
`compiler::lang::equation_supplement_for_lang` through its existing owner;
None is Empty; Content/String use display conversion; Func receives the base
public equation Content and its result uses display conversion. Callback errors
propagate. Never use the reference's current language or a private translation
table.

After numbering and supplement passes, replace each equation's canonical
entry with its public Styled view: raw numbering, default/effective
number-align, realized supplement, effective alt, unchanged body/block.
Use IntrospectedContent with absent fields to preserve that existing realized
projection; do not erase the realization or alter Heading's present snapshots.
Only explicit conversion to rendering discards a snapshot, not the original
stored Value. Shared invariants belong to the pinned content-snapshot nucleus.

## Acceptance and boundaries

Verify state errors versus display defensive errors, separate update arity
versus array display arity, non-negative return validation, filtered
Step/Func/Step and later-error ordering, absence of callback introspective
context, immutable logs, bundle Page resets and no duplicate callbacks.
Check lexical equation numbering/supplement/alt and unchanged Heading snapshots.
Do not reinstate retired observation carriers/hooks; instrumentation needs its
own approved specification. Consumer existence proves implementation, not
independent parity or completion of every contextual stabilization gate.


## Materialização do conjunto ratificado — 2026-10-05

Reexportar realize_page_counter_snapshot com a assinatura pública já ratificada no owner counter_updates; nenhuma fórmula nesta fachada.


## Carriers compactos de introspecção/show — proposta 2026-10-16

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. before.json.gz em 00_nucleo/diagnosticos/clippy-enums-publicos-2026-10-16 registra UTC, fontes/L0, hashes, status e git diff HEAD --stat. Os owners atuais foram lidos antes de propor esta alteração. probe-details.json registra rustc 1.92, argv, rlib/binário/sonda com SHA-256 e saída; probe-result.json registra teto de 12 GiB, swap zero, um job e eventos. A sonda é sintética de carriers e do algoritmo de registro/cópia, não benchmark de compilação/layout/RSS nem distribuição de workloads reais. Os candidatos são espelhos com tipos reais de domínio; o caso inline usa Tag e ShowRule produtivos.

Tag mede 416 bytes; Start boxed e Arc medem 48. Para 4.096 pares Start/End com payload Heading sem heap, inline solicita 3.407.872 bytes numa alocação; Box solicita 2.031.616 em 4.097; Arc solicita 2.097.152 em 4.097. Clone de Tag boxed também aloca por Start; os caminhos produtivos lidos em introspect/fixpoint.rs:86–136 e entrypoints.rs:44–66 usam Vec<Tag> temporário e leituras emprestadas, sem clonar o vetor. Walk :325–356 popula o índice por referência antes de mover info para Start. A decisão não toma o clone sintético como frequência produtiva.

Transformation mede 328 e ShowRule 376 bytes; candidatos com Box/Arc<Styles> medem 120 e 168. rules/show.rs:464–470 copia to_vec antes de cada registro; apply.rs:108–112,349–352 filtra/clona regras. Em 128 registros 100% show-set simples sem reter snapshots antigos, inline solicita 12.276.960 bytes/384 alocações, Box 8.194.560/8.640, Arc 5.530.624/512. Com snapshots retidos, picos solicitados são 3.203.856, 4.141.744 e 1.477.808. No controle com canal custom, as cópias de 4.096 regras solicitam 4.107 alocações inline, 8.203 Box e 11 Arc (crescimento do Vec incluído). As leituras de estilos são emprestadas em apply/intercept/outline/table_cells; a aplicação ainda clona o delta quando necessário.

Contraexemplo obrigatório: construir uma única coleção de 4.096 regras 100% show-set simples pede 1.540.096 bytes inline, 2.031.616 Box e 2.097.152 Arc. Arc pode reter capacidade sobrante do canal custom; no controle rico de 128 registros sem snapshots antigos, o final vivo foi 78.064 bytes inline e 156.432 Arc. Não afirmar ganho universal ou ignorar custo atômico/indireção. É mecânica de armazenamento sob ADR-0030/0107, não paridade de linguagem ou ganho cognitivo por IA medido. A inferência favorável a Arc no ciclo de cópias seria refutada por regressão de memória/tempo em um workload real relevante; manter visíveis estes controles.

### Responsabilidade exclusiva proposta

Migrar somente os construtores Tag::Start das fixtures deste consumer para Box::new do mesmo ElementInfo. Preservar Locations, payloads, labels, End hashes, chamadas, assertions e todos os corpos produtivos. Leituras emprestadas e hashes por valor conservam os contratos existentes; não criar algoritmo de hashing novo, aceitar novo erro ou atualizar esperados para mascarar diferença. O payload público pertence exclusivamente a 00_nucleo/prompts/entities/tag.md.

### Gate e aceitação

Proposta não vigente até ratificação humana ADR-0127 §2.1/§2.4: há mudança de payload público e compatibilidade de construção/acesso. Não editar código ou owners ativos por este pacote antes da aprovação. Após ratificação instalar estes L0, testes primeiro e RED real, implementação e resselo estritamente dos owners alterados. Não silenciar large_enum_variant, alterar limiar ou adicionar dependências/flags/defaults/fases.

T1: contratos de construção Box<ElementInfo>/Arc<Styles> e orçamento relativo dos carriers (Tag até três Location; Transformation até Content+alinhamento) devem falhar no estado antigo e passar no novo, sem fixar ABI universal. T2: Start/End/Location/payload/label, igualdade e hashes estruturais por valor, clone independente da Box e mutation isolation preservados; hash não vira formato persistido. T3: testes existentes de introspecção/convergência, nesting, heading/label, state/counter/query/metadata continuam com os mesmos esperados e erros. T4: clone de Transformation::Style compartilha o Arc; Arc::make_mut em cópia preserva estilos do original, incluindo custom, e as variantes Func/Content/Str permanecem. T5: controles existentes de show-set/func, precedência, escopo, labels, outline, table.cell e textual/regex mantêm semântica/morfologia/erros; nenhuma regra executa a mais. T6: workspace completo, build, Clippy all-targets e expectativas/lints já fechados como erro, rustdoc -D warnings com quatro índices, fmt, diff, linhagem estrita/reversa e Tekt sem aumento; 12 GiB, swap zero, um job. Registrar HEAD/working tree/hashes/diffstat/UTC e distinguir tamanhos/bytes solicitados de RSS. Não publicar release. Alguma mudança de assinaturas, campos ou fase além destes payloads exige outra proposta.
