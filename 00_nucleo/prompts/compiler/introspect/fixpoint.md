# L0 — introspect/fixpoint
Hash do Código: 5ecbdee5

```text
```

## Ownership and evidence

Layer: L1. Sole consumer: `01_core/src/compiler/introspect/fixpoint.rs`.
`run_fixpoint` at `:75` returns `Result<TagIntrospector, FixpointError>`;
`introspect_to_fixpoint` at `:149` is its direct wrapper.
The current L3 consumers use single-walk runtime entrypoints from
`00_nucleo/prompts/compiler/introspect.md`, not this full reevaluation loop.
Source call-site evidence and its reproduction are recorded in
`00_nucleo/diagnosticos/sanitization-introspection-columns-history.md`.

## API and bounded loop

Preserve MAX_FIXPOINT_ITERATIONS = 5 and these signatures:

```rust
pub fn run_fixpoint<F>(
    engine: &mut Engine<'_>,
    ctx: &mut EvalContext,
    eval_step: F,
) -> Result<TagIntrospector, FixpointError>
where
    F: FnMut(&mut Engine<'_>, &mut EvalContext) -> SourceResult<Content>;

pub fn introspect_to_fixpoint<F>(
    engine: &mut Engine<'_>,
    ctx: &mut EvalContext,
    eval_step: F,
) -> Result<TagIntrospector, FixpointError>
where
    F: FnMut(&mut Engine<'_>, &mut EvalContext) -> SourceResult<Content>;
```

Begin with an empty previous introspector and no previous tag hash. On each
bounded iteration, clone that introspector into ctx.introspector, evaluate the
closure, then walk its content with a fresh Locator, tag vector, introspector,
auto-label counter and default StyleChain; the language argument is None.
No separate CounterStateLegacy value is created or returned.

Compute the tag hash and coordinate the child-owned passes in current order:
state functions, counter functions, state displays, counter displays,
equation numbering, equation supplements, equation realization.
After those passes, equality with the preceding iteration's tag hash returns
the current introspector. Otherwise retain its hash/introspector and repeat.
The first hash alone cannot establish convergence. Exhaustion returns
FixpointError::NotConverged; evaluation and fallible postprocessing errors
return FixpointError::Eval immediately, not a partial success.

## Boundaries and acceptance

Hashing is owned by `00_nucleo/prompts/compiler/introspect/convergence.md`;
callback semantics by `00_nucleo/prompts/compiler/introspect/from_tags.md`;
the causal walk by `00_nucleo/prompts/compiler/introspect.md`.
This helper does not independently add the parent's bibliography conversion,
outline-entry synthesis or parent-index preparation. Do not infer equivalence
of every prepared store with the production entrypoint.

Keep the linear bounded mechanism without new memoization or I/O.
Test stable successive evaluations, dependence on the preceding introspector,
bounded non-convergence, evaluation/runtime errors and wrapper equivalence.
Hash equality is this helper's mechanical stopping rule, not proof of all
language observables or authorization to substitute it for pipeline
stabilization. No promised future migration or production adoption is implied.

## Helpers e armazenamento sem uso — 2026-10-13

### Medição anterior à decisão

`01_core/src/compiler/introspect/fixpoint.rs` nas linhas 170 contém os itens sinalizados no inventário Clippy. HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado; fontes, owners, referências, UTC, status e diffstat em `00_nucleo/diagnosticos/helpers-testes-ci-2026-10-13/before.json.gz`. A pesquisa de chamadas e os caminhos produtivos precedem esta classificação; não é nova medição de paridade nem prova de desempenho.

### Decisão e aceitação

Remover somente o helper de teste ck sem chamadas. Conservar sel, fixtures, testes, assertivas e todo o algoritmo de fixpoint.

Preservar as rotas produtivas, nomes e corpos dos testes existentes, APIs externas, defaults e fase. Alteração interna em fluxo contínuo ADR-0127. Revalidar build e check de todos os targets do workspace, suíte core, inventário de testes, Clippy e linhagem; não eliminar expectativas para passar o gate.

## A1 — callbacks de suplemento de Heading, 2026-10-17

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c e working tree/UTC/hash
registrados em 00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17/before.json.gz.
before-pdf-probe.json.gz: heading-supplement-panic termina em sucesso no Tekt
e em A1_HEADING no vanilla bd5ebec285505889e8ff1d4f68804ab147a699f3.
heading-supplement-valid aceita ambos, mas só o vanilla usa Custom na referência.
Fonte upstream model/heading.rs:249-289 sintetiza supplement por ocorrência e
model/reference.rs:393-415 aplica a função e display. Set.rs:286-320 ignora
supplement e limpa numbering; snapshot_fields heading.rs:55-68 fixa tradução.
Isto é callback/valor observável, não exigência de mecânica upstream.

### Decisão e responsabilidade exclusiva

No helper de fixpoint existente, finalizar Heading junto aos demais
pós-processadores com Engine antes da verificação de convergência. Propagar
erro como FixpointError::Eval. Não mudar cap, critério, número de passes ou
adotar o loop completo em novos callers.

Correção de paridade em fluxo ADR-0127: sem nova API Rust, campo público ou
transferência de lógica existente entre eval/layout. O pós-walk com Engine já
é a fronteira de aplicação de callbacks. L0 primeiro, RED e controles bilaterais,
sem callbacks em encoder/walk/layout e sem alegar paridade de todos os campos.


## A1-S2 — leitura por objeto aprovada em 2026-10-17

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado;
UTC/status/diffstat/fontes em before.json.gz e final-freeze.json.gz de
00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17.
state-counterproof.json.gz registra RED bilateral: init 1/10 da mesma chave
após +1 exige 2/11; callback não lido não executa; atualização reutilizada
executa por ocorrência. Pin bd5ebec285505889e8ff1d4f68804ab147a699f3,
upstream introspection/state.rs:226–245, 419–455 e 477–520. O init do leitor
inicia a sequência; a ordem vem do documento, não do inteiro de Location.
O método atual trata Func como Set e o pós-passe eager perde essa semântica.
Estes são valores/erros de linguagem, não igualdade Rust ou algoritmo exigido.
A1-S1 foi retirada; gate A1-S2 confirmado pelo dono nesta sessão, revisão
00_nucleo/materialization/state-reader-runtime-2026-10-17/review.md,
SHA-256 6d6a176b8e27cc53c38fe6650c03c217448059f4e7287fd8848fe54e80247fa4.

### Responsabilidade exclusiva — supersede a fase histórica de State

Retirar aplicação automática eager de StateUpdate::Func; os leitores com Engine
resolvem as ações registradas. Coordenar display interno fallible e preservar
cap=5, hash/critério de convergência, precedência dos demais passes e APIs.
Testes que pretendem observar valor funcional usam leitura runtime com Engine,
não assumem que o trait legado sem Engine materialize Func automaticamente.

### Aceitação e fronteiras

Gate A1-S2 ratificado para as cinco assinaturas e a fase de leitura. L0 primeiro,
RED funcional antes do código; GREEN contra o pin indicado para linguagem e
controles próprios das extensões Tekt. Não ampliar outros contratos públicos.
Build/workspace/doctests, Clippy/Rustdoc estritos, fmt, linhagem V1/V5/V15/V26 e
reversa; manter V16 sem aumento. MemoryMax12GiB/High8GiB/swap0/jobs1, tarefas
pesadas sequenciais. Medir consumo sem inferir ganho global só da representação.


## A1-H2 — suplemento antes das consultas, aprovado em 2026-10-17

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado;
final-freeze.json.gz e target-controls.json.gz no diagnóstico
00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17 registram
UTC/diffstat/fontes/argv. Pin bd5ebec285505889e8ff1d4f68804ab147a699f3.
heading-raw-constructor-func passa no baseline/vanilla e falha no candidato:
Styled de transporte altera func. Query também vê wrapper causal temporário;
HTML ignora callback panic. Fonte walk.rs:930–945; pipeline/paged.rs:419–427 e
pipeline/html.rs:101–106 preparam a consulta pelo walk puro. São valores e erros
de linguagem, não um requisito de copiar algoritmo/estrutura do vanilla.
Gate A1-H2 confirmado pelo dono nesta sessão: revisão
00_nucleo/materialization/heading-supplement-runtime-2026-10-17/review.md,
SHA-256 66739b13596013f4343bdc3ffacbdb9081151e3d9863e5d962e8ff13fd68775c.

### Responsabilidade exclusiva

Atualizar somente chamada interna do finalizador Heading para usar o índice e
pendência do snapshot; preservar limite, convergência e a retirada eager de
State aprovada separadamente em A1-S2. Sem novas APIs deste owner.

### Aceitação e limites

H1–H8 da revisão ratificada, RED bilateral antes do código. Nenhum contrato
público além da fachada Heading está autorizado por H2; A1-S2 cobre separadamente
cinco helpers State. Sem novos campos públicos. Styled ordinário e demais
callbacks conservam suas fronteiras. Workspace/doctests/build/Clippy/Rustdoc,
fmt, V1/V5/V15/V26/reversa e V16 sem aumento; 12GiB/High8GiB/swap0/jobs1,
comandos pesados sequenciais. Ganho de RAM exige medição, não intenção.


### A1-S2 — regressão histórica de erro no fixpoint

Medição anterior: pre-repair-validation.json.gz, complete-workspace.log, HEAD
b1b486f3d42431f8ea360bc5712637ee1e3d4f6c e working tree/UTC/diffstat no freeze
ali preservado. fixpoint_propaga_erro_state_update_callback exige erro sem leitor,
mas State não lido não executa callback no pin bd5ebec285505889e8ff1d4f68804ab147a699f3
(state-counterproof.json.gz). É uma expectativa histórica incompatível com A1-S2.
Preservar sua asserção de FixpointError::Eval e a mensagem da callback, adicionando
à fixture uma leitura efetiva por StateDisplay após o update. O controle separado
a1_state_unused_callback_waits_for_a_read continua exigindo ausência de execução
sem leitura. Não reintroduzir o pós-passe eager nem alterar o resolvedor para
satisfazer a expectativa antiga. Correção de teste interna, sem contrato novo.
