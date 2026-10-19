# L0 — compiler/eval/bindings/field_access
Hash do Código: 3947a3aa

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/builtin-module-identity.toml sha256:2b8888460a1e6008abda0ff71603fe4dace79ff0a2a7aad598f8aabfab3e3776
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
- 00_nucleo/prompts/_nuclei/eval/call-once.toml sha256:a922bbaa9c3b4c8435708af8e68b4557801f93ba09188768d48c711ba35a0c15
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/bindings/field_access.rs`
**Atomização interna:** a fachada conserva a interface e os testes; os Prompts L0 em `00_nucleo/prompts/compiler/eval/bindings/field_access/` possuem identidade, campos, erros de callee e métodos de Content. O contrato abaixo permanece comum.
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/bindings/field_access.rs:434` — `fn native_public_name`.
- `01_core/src/compiler/eval/bindings/field_access.rs:451` — `fn is_user_defined_closure`.
- `01_core/src/compiler/eval/bindings/field_access.rs:474` — `fn is_format_pdf_namespace`.
- `01_core/src/compiler/eval/bindings/field_access.rs:479` — `fn is_format_pdf_function`.
- `01_core/src/compiler/eval/bindings/field_access.rs:497` — `fn is_a11y_pdf_target`.
- `01_core/src/compiler/eval/bindings/field_access.rs:504` — `fn is_effective_format_module`.
- `01_core/src/compiler/eval/bindings/field_access.rs:511` — `fn is_effective_std_module`.
- `01_core/src/compiler/eval/bindings/field_access.rs:519` — `fn is_effective_symbol_module`.
- `01_core/src/compiler/eval/bindings/field_access.rs:537` — `fn warn_deprecated_symbol_binding`.
- `01_core/src/compiler/eval/bindings/field_access.rs:557` — `fn eval_field_access`.
- `01_core/src/compiler/eval/bindings/field_access.rs:707` — `fn eval_value_field_access`.
- `01_core/src/compiler/eval/bindings/field_access.rs:1057` — `fn element_or_type_with_name`.

Prior source readings supporting these clauses (snapshot provenance is
retained in the assigned history diagnostic, not asserted as a fresh measurement):

- `foundations/fields.rs:14–67`.
- `foundations/func.rs:280-308`.
- `foundations/func.rs:412-450`.
- `lab/typst-original/crates/typst-eval/src/code.rs:347-366`.
- `lab/typst-original/crates/typst-eval/src/code.rs:347–366`.
- `lab/typst-original/crates/typst-library/src/foundations/func.rs:280–308`.
- `lab/typst-original/crates/typst-library/src/foundations/func.rs:291–308`.
- `lab/typst-original/crates/typst-library/src/foundations/module.rs:139-150`.
- `lab/typst-original/crates/typst-library/src/foundations/value.rs:157–169`.
- `lab/typst-original/crates/typst-library/src/lib.rs:221-225,374`.
- `typst-eval/src/code.rs:347-367`.
- `typst-eval/src/code.rs:347–366`.
- `typst-library/src/symbols.rs:12-35`.

## Contract

Own eval_field_access, eval_value_field_access, field_callee_error,
content method projection, queried-content projection and deprecation adapters.
Keep the existing static catalogs and visibility; method implementation remains
delegated to its native owner. Resolve a receiver once and anchor pure lookup
errors at the supplied field span, without reconstructing syntax or origin.

Native function fields traverse the actual namespace (including With unwrapping).
Unknown native fields use the canonical function identity/name, not a lexical
alias. User closures, also through With, retain
`cannot access fields on user-defined functions`. Do not generalize that rule to
plugins or element functions. Module errors use the stored module name:
``module `<name>` does not contain `<field>` `` with no hint, at the field only.
Aliases of std refer to the object named global; an ordinary std.typ module stays
std. Never identify std by a local spelling or by comparing its bindings.

Dictionary, raw-content and float lookup preserve their distinct diagnostics and
field-only anchors. Other scalar misses use canonical long type names. Array
len/first/last keep their existing AST routing; unknown array fields fail as
array field access. Preserve the known distinction between isolated fields and
bound-call interception rather than making all methods first-class values.
In particular isolated Float is-nan/is-infinite/signum/to-bytes, Angle deg/rad,
Version at, Func where and SpotColorant tint do not become bound method values.
Type lookup exposes only its actual static methods; Type::Function is not thereby
a general constructor. Delegate static float/version/spot operations to stdlib.

Keep Duration days/hours/minutes/seconds, Version major/minor/patch,
Length abs/em, Relative length/ratio, Args positional/named, Stroke paint/thickness
and Alignment x/y projections (the corresponding single axis or None).
For contextual text.size, prefer the typed top style
size over a custom field and preserve derived script size; use the existing
default 11pt only when absent. text.lang keeps its existing contextual lookup.
Do not execute a style/font callback while projecting a field.

Content exposes func/has/at/fields/location through the closed ContentField
projection and existing helpers. Distinguish absent from a present None/Auto.
Consume self/default arguments coherently using the method_dispatch helpers.
OutlineEntry.body routing stays restricted to actual entry content; entry level,
element and fill projection remains distinct from body evaluation.

For LocatedContent Some, the pinned snapshot nucleus is authoritative: use its
complete ordered fields, not raw fallback, styles or reconstructed defaults.
Missing at uses the supplied default only when the field is absent. Preserve
Location independently. LocatedContent None keeps legacy projection, including
realized Equation. Raw projection uses causal presence masks. Equation exposes
body/block and explicit styles; alt is Str or explicit None and precedes body,
with top style precedence and omitted alt absent. MathPrimes exposes count;
MathLr exposes explicit size then body; MathStretch always exposes body and
exposes size only when explicit. Annotated math variants always expose body and
expose annotation only when present, including explicit None, in canonical order.

Feature gates use effective typed native/module identity, not homonymous
dictionaries or aliases by spelling. The three gated pdf members require
A11yExtras; absence produces the existing field diagnostic with its two hints.
HTML format fields require Html, bundle format fields and std.asset require
Bundle; features remain independent. Identity of the actual format namespace
is its existing surface, not a fabricated default projection. Active features
use ordinary lookup and the real constructor namespace, never a placeholder.

Symbol deprecation warnings are delegated to the sym owner and emitted on
successful root binding resolution with the existing causal anchor. Missing
roots do not warn; an invalid modifier of a deprecated root preserves the root
warning. Keep legacy join as a measured extension and the un-deprecated tack
bindings silent. Preserve the narrow tri fallback only for canonical gt/lt with
eq/not modifiers after normal modifier lookup fails; use sym lookup, preserve
the legacy routes and repr parents, and do not recognize arbitrary named Symbols.

Keep messages, severities, hint order, traces and spans unchanged. Public field
availability or generic method-value promotion requires a separate contract
decision; this owner does not authorize it.

## Reexportação usada somente nos testes — 2026-10-13

Medição anterior à decisão: a compilação de todos os targets, após condicionar
o alias no hub bindings, identifica o alias correspondente em
`01_core/src/compiler/eval/bindings/field_access.rs:451` como sem uso produtivo.
O consumidor externo ao módulo é o teste em
`01_core/src/compiler/stdlib/tests/introspection.rs:163`; chamadas produtivas
usam a função dentro de content_methods. HEAD
06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado; fontes,
UTC e diffstat em `lab/results/construction/diagnostics/imports-ci-2026-10-13/before.json.gz`
e `first-check-state.json.gz`.

Manter a reexportação de eval_content_method_at somente sob cfg(test),
preservando sua visibilidade no build de testes, a definição produtiva e
as demais reexportações. Composição interna ADR-0127, sem nova API pública,
fase ou default. Revalidar os testes existentes, build e linhagem.


## Continuação causal do gate — chamadas e set rules

Medição anterior à decisão: candidate-1/validation-green.log registra quatro
controles aprovados e falha real do teste de argumentos: std.html(panic(...))
executa panic antes do gate. before-set-probe.json.gz registra UTC, hashes,
fontes e saídas bilaterais no mesmo HEAD/árvore não commitada: std.html,
format.html, std.asset e format.bundle chamados avaliam argumentos antes do
gate; set std.html/std.format.html/alias apaga o erro do target e ignora os
argumentos. Variável/field ausente em set também é aceito indevidamente;
condition false conserva atalhamento. Html ativo em set por alias ignora panic
no Tekt e o executa no pin. São ordem causal, diagnóstico e efeito da linguagem.
collection_methods.rs:36 avalia argumentos antes de autenticar módulo; set.rs:
44 usa ok() na resolução e :326 autentica HTML pela sintaxe literal.

Somente reexportar check_module_field_feature do owner fields com visibilidade interna a eval.

L0 precede esta continuação; correção interna/paridade em fluxo ADR-0127.
O núcleo builtin-module-identity continua pinado e sem alteração. O contrato
público e os campos externos permanecem iguais; nenhum fallback novo ou
supressão. Os controles e a matriz dos oito perfis permanecem; acrescentar
verificação bilateral dos efeitos e de set com/sem feature. A suite completa
regressa após o source freeze novo, sem reaproveitar gates de fonte alterada.


## Avaliação única em chamadas — 2026-10-17

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
lab/results/construction/diagnostics/call-once-2026-10-17/before.json.gz registra UTC,
status, diff HEAD --stat, fontes/L0/config e hashes. Os 2124 hashes da etapa
anterior foram verificados exatamente antes do baseline. before-probe.json.gz
e before-math-probe.json.gz conservam binários/argv/fixtures/saídas bilaterais.
Pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8, SHA-256 do binário
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
calc.abs/array.len/str.from-unicode medem receiver=3/args=2 contra 1/1;
module.measure/layout/within/before/after/func/has/body chegam a receiver=4;
fallback mutante mede args=3 contra 1. Métodos ausentes executam panic de
argumento antes do erro causal. math.mat soma duas vezes; array prévio (1,)
fica (1,1,1) contra (1,1). Array vazio mascara o defeito com truncate.
call.rs:70–118 perde receiver; collection_methods.rs:43 avalia args antes de
reconhecer operação; contextual/content/selector reavaliam AST no fallthrough;
method_dispatch.rs:247 descarta args; invoke.rs:66–120 compensa por nome.
O L0 call_dispatch.md já exige receiver/args uma vez. pinned-source.json.gz
fixa call.rs/methods.rs upstream: interesse é ordem/efeito/diagnóstico da
linguagem, não copiar Vm, storage ou igualdade Rust.

A primeira fixture declarava palavra reservada or como função: é inválida e
está arquivada em invalid-fixture-attempt, sem crédito para efeitos de chamada.
Ela revelou panic 101 em import inválido; manter aberto para correção causal
própria no owner de parsing/import. O baseline válido substitui só a fixture.

### Decisão e responsabilidade exclusiva

Reexportar somente eval_field_access_with_target com visibilidade interna a eval; não absorver lógica. APIs puras existentes permanecem.

Consumir o núcleo eval/call-once.toml pinado. Mudança mecânica interna e
correção de paridade ADR-0127 em fluxo contínuo. Assinaturas pub fn/pub use e
structs públicos não mudam; os parâmetros novos/retornos/helpers acima têm
visibilidade crate/eval, sem novo trait público, campo público, default ou
fase. Esta cláusula supersede somente restrições históricas conflitantes de
assinatura interna e compensação por nome; manter extensões Tekt existentes.

### Aceitação e limites

Testes funcionais co-localizados antes do código: efeitos de receiver/args,
fallback mutante, erro antes de args, where/with/selector/coleções preservados
e matriz sem truncar efeito legítimo. Confirmar RED funcional e GREEN.
Comparar casos bilaterais válidos e controles; hits e misses de todos os
métodos de coleção, erros, spans/occurrences e complementos de Value. Não
mascarar dívida residual de diagnóstico como igualdade. Testar dispatch
Unhandled sem perder receiver ou executar callback de args; contar avaliações
pelo efeito observado, não por bytes de PDF, Rust Eq ou microbenchmark.

Ownership/V26 antes de resselo só dos owners do lote. Build, suíte workspace,
Clippy all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc estrito,
fmt, V1/V5/V15/V26 e reverse, SARIF diferencial sem erro novo. V19 uniforme
continua informativo, com matriz de alternativas e relação de comportamento;
sem supressão nem mudança de configuração. Comandos pesados sequenciais com
12 GiB/high10GiB/swap0/jobs1. Não publicar/commit/merge/release nem declarar o
objetivo global fechado por esta etapa.


### Atualização do controle Type::Array para o pin vigente

candidate-5/workspace/workspace.log registra uma falha, no controle antigo
p1338_preserve_other_categories_diagnostics: #array.nope ainda exige aspas
duplas e a âncora array.nope inteira. O pin 9f2b6e871 na sonda missing/array
emite type array does not contain field `__missing`, no field-only span, e a
fonte code.rs:347–368 usa field.span. Os REDs de chamadas e a comparação
bilateral integral fixaram esse contrato antes do código. Atualizar somente
a linha #array.nope daquele teste para mensagem com backticks e âncora nope.
Manter os casos length/boolean/none/auto/integer/string, sua severidade, spans,
hints, traces e perfis anteriores, sem retirar assertivas ou ignorar o teste.

## A1 — revisão do residual histórico text.text, 2026-10-17

Medição antes da decisão: workspace.log do diagnóstico
lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17
registra p1325_content_direct_alias_and_known_lookup_residual falhando porque
`[x].text` agora devolve o texto, conforme sonda bilateral before-pdf-probe.json.gz
contra vanilla bd5ebec285505889e8ff1d4f68804ab147a699f3. A asserção de erro era
um residual conhecido, não um contrato desejado. Retirar somente essa entrada
da tabela negativa; substituir por controle positivo explícito `[x].text == "x"`
no teste positivo existente. Conservar negativos de campos desconhecidos, alias,
Unicode e todos os perfis. Não mascarar regressões de presença nem alterar o
dispatch produtivo deste hub. L0 antes do teste, resselo e suíte completa.

## R2 — construção e seleção de Sequence, fluxo contínuo ADR-0127

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado.
lab/results/construction/diagnostics/sequence-constructor-2026-10-17/{before,expanded}.json.gz
e before-sources.json.gz conservam UTC, diffstat/status, binários/fontes e casos
bilaterais no pin bd5ebec285505889e8ff1d4f68804ab147a699f3. Vanilla
foundations/content/mod.rs:724–728 declara children requerido;
foundations/value.rs:644–650 define cast Content/string/symbol/none e
typst-macros/src/elem.rs:573–604 gera o constructor. Tekt
01_core/src/compiler/eval/bindings/field_access/content_methods.rs:358–410
ainda projeta fallback não chamável; o catálogo element_where não tem Sequence
e selector_matching.rs:272–337 não tem sua identidade. rules/set.rs:55–76
só recusa With de transforms e não reconhece set da Sequence. São semântica,
morfologia e diagnósticos de língua, não exigência de copiar storage upstream.
A mensagem de children() marcada pendente em R1 já coincide no baseline atual: o
getter recém-corrigido permite que o owner existente produza o erro correto.
A medição nova retifica essa pendência; não editar código de diagnóstico.

### Responsabilidade exclusiva

Somente reexportar native_sequence diretamente do owner content_methods para
o hub de bindings com visibilidade interna; sem lógica de construção ou seleção
nesta fachada. Preservar todos os demais exports e testes.

### Aceitação e limites

Esta seção completa a responsabilidade R2 nomeada no L0 de R1. Corrigir
continuamente tabelas/composição/callback interno, sem novos campos/traits/
assinaturas Rust externas, modo padrão, fase ou quebra de compatibilidade.
L0 antes de teste e código, RED funcional, GREEN e corpus bilateral. Não
certificar guardas globais de show, todos os casts/elementos, nem paridade
universal de render. Qualquer refutador residual deve permanecer no relatório
e ganhar owner/L0 antes de intervenção, sem mudar testes para esconder o caso.

Validar build/workspace/doctests em invocação separada, Clippy/Rustdoc estritos,
fmt, V1/V5/V15/V26 e reversa, SARIF diferencial sem erro/warning novo. V19 pode
registrar agrupamento nominal prescrito sem supressão, testando alternativas.
Heavy jobs sequenciais, MemoryMax12GiB/High8GiB/swap0/jobs1, debug0/incremental0
só no ambiente de validação. Resselo após V15/V26 válido e proveniência exata.
