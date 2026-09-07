# Prompt L0 — `compiler/eval/closures` — criação e aplicação de closures
Hash do Código: 6edc6204

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/eval/closures.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/eval.md` (dono de `compiler/eval/mod.rs`)
**ADRs relevantes**: ADR-0107 (paridade língua), ADR-0108 (medir antes de decidir), ADR-0109 (atomização)
**Técnica**: aplicação de closures com captura de scope e binding de parâmetros

---

## Contexto

Este nó contém a **criação e aplicação de closures** do eval:

- `eval_closure_expr` constrói um `Value::Func` a partir de uma expressão de closure, capturando o scope actual.
- `apply_closure` aplica uma closure a argumentos: cria um scope filho do captured, injeta auto-referência para recursão, liga parâmetros (posicionais/nomeados/default/patterns/sink), avalia o body num engine local, e consome `FlowEvent` ao sair.

Extraído do monólito `compiler/eval/closures.rs` no Passo 1012 conforme ADR-0109 (atomização — forma B, free function no arquivo da unidade). O dispatch geral de chamadas (`eval_func_call`, `apply_func`, avaliação de args, etc.) mudou-se para o nó `compiler/eval/call_dispatch.rs`.

---

## Instrução

### 1. Contrato público

```rust
pub(super) fn apply_closure(
    closure: &ClosureRepr,
    func: &Func,
    args: Args,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value>;

pub(super) fn eval_closure_expr(
    closure_expr: ClosureNode<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value>;
```

### 2. Comportamento

Manter exatamente o comportamento actual:

- `eval_closure_expr` faz captura eager do scope por snapshot (`scopes.snapshot()`), extrai parâmetros (posicionais, nomeados com default, patterns, sink spread), e constrói um `ClosureRepr`.
- `apply_closure` verifica profundidade de chamada (`MAX_CALL_DEPTH = 80`), cria scope filho do captured via `Arc::clone`, injeta auto-referência se a closure tiver nome, liga parâmetros com named-sobre-posicional e regras P708/P724/P733/P504, avalia o body num `Engine` local com route/styles/sink próprios, e consome `FlowEvent::Return`.

### 3. Gatilhos de reabertura

- Mudança de semântica de captura (lazy vs eager, scope vs environment).
- Mudança no binding de parâmetros (ordem, default, sink, patterns).
- Mudança de fase (eval ↔ layout) na execução de closures.

---

## Critérios de verificação

```
Dado #let f(x) = x * 2; f(3) → 6
Dado #let g(a, b: 1) = a + b; g(2) → 3
Dado #let fib(n) = if n <= 1 { n } else { fib(n - 1) + fib(n - 2) }; fib(6) → 8
Dado #let h(..args) = args.pos().len(); h(1, 2, 3) → 3
```

Aplicação final: `cargo build && crystalline-lint .` — zero violations.

## P1160 — parâmetro posicional obrigatório ausente

Medição no vanilla ratificado `a51e02804`: aplicar dois argumentos a
`(a, b, c) => ...` falha com `missing argument: c`; o parâmetro não recebe
`none` implicitamente. `apply_closure` deve emitir esse diagnóstico no binding
do primeiro positional obrigatório ausente. Defaults nomeados e sinks mantêm
as regras vigentes; argumentos excedentes continuam `unexpected argument`.

## P1307-R3 — consumo e sink preservam ocorrências

**Estado**: `DRAFT_L0_AWAITING_ADR0127`; sem aprovação de implementação.

### Medição anterior à decisão

Baseline HEAD `b303f1f15b610e09872b567027e0d806387fde8c` mais P1306:
`01_core/src/compiler/eval/closures.rs:72-114` consome named via mapa e
posicionais por índice; `:117-131` reconstrói o sink apenas das views.
A matriz `00_nucleo/diagnosticos/p1307-r2-measurement.json`, SHA-256
`847faabad41df603a82f7fc5c5d0435180cdec66c33ae9b3a1fd55d7ee320fa2`,
mede `with-spread-closure-f/g`: as origens `51..58` e `88..95` sobrevivem
ao sink e factory no vanilla. A fonte
`lab/typst-original/crates/typst-eval/src/call.rs` aplica closures consumindo
Args, sem recriar origens a partir da AST da chamada final.

### Decisão condicionada

Usar o contrato `entities/args.md` sem mudar assinatura deste módulo.
O binding mantém seus critérios vigentes P708/P724/P733/P1160: ao consumir
um named, `remove_named` devolve o último valor e retira todas as ocorrências
desse nome; ao consumir um positional, `remove_positional(0)` retira
exatamente o próximo positional. Não somar índice antigo a uma lista já
reduzida. Default omitido é valor da definição, não uma ocorrência fornecida
pelo caller. Patterns consomem uma ocorrência antes de destructure.

O sink recebe os argumentos restantes com carrier coerente e o span
agregado da chamada original, na ordem refinada por R4 abaixo. Não deve
reintroduzir parâmetros consumidos,
trocar os spans dos sobreviventes pela lista local nem conservar metadata
stale após shift_remove/skip. Sem sink, a precedência e os diagnósticos
preexistentes de excedentes permanecem: transporte não autoriza refazer a
política geral de erros de closures.

Passar um Value comum a chamada escrita no corpo gera nova ocorrência
naquela AST; encaminhar `..args` de sink preserva a origem anterior.
Captura eager, recursão, profundidade, flow, estilo/contexto e fase não mudam.
Closure/factory que retorna With mantém a origem por Args transportado, não
por armazenamento artificial do ambiente local no Func retornado.

Verificar após gate: parameters retirados não reaparecem no sink; defaults
não entram no sink; as ocorrências sobrevivem na ordem do sink definida abaixo;
origens f/g e cross-source permanecem distintas; clone e chamadas repetidas
não consomem o Args da aplicação seguinte. Unknown obrigatório bloqueia.

### P1307-R4 — ordem do sink medida antes do candidato

A tentativa focal independente de `p1307-r4-measurement.json`, encerrada em
`2026-09-07T17:37:05.017565+00:00` sobre baseline R4 SHA-256
`52df1c661c20d9eb612bbd5c89ae3cae8c44735aeab27c11e4a1da0d4d145e57`,
refuta a premissa de ordem intercalada no sink: `args.sink-only` produz
`arguments(key: 2, 1)` nos dois produtos, enquanto o constructor arguments
preserva ordem lexical. `args.sink-mixed` vanilla conserva duplicatas como
`arguments(z: 11, z: 33, a: 44, 22, 55)`. A fonte ratificada
`lab/typst-original/crates/typst-eval/src/call.rs:693-715,728-735` consome
posicionais para o sink e os anexa aos named restantes. O recibo focal foi
copiado integralmente em `00_nucleo/diagnosticos/p1307-r4-contract-refinement.json`
antes desta decisão; R3 permanece como histórico, não expectativa vigente.

Depois de consumir parâmetros conforme os critérios vigentes, o resultado
do sink ordena primeiro todas as ocorrências named restantes, na ordem
relativa original, e depois as posicionais restantes, também na ordem
relativa original. Conserva valores, duplicatas e ambos spans de cada uma;
somente reordena a sequência causal do novo Args, reconstruindo por
from_occurrences. Não é permissão para ordenar mapa ou descartar duplicatas.
Default ausente não entra no sink e consumed named não reaparece.

Preservar o binding vigente de parâmetros: o caso de sink não-terminal
`args.sink-before-positional` mede dívida separada de qual positional é
capturado; este ajuste não altera ClosureRepr nem registra posição do sink.
Essa dívida é controle cristalino explícito, não paridade afirmada. Corrigir
a ordem observável do sink é paridade interna em fluxo contínuo ADR-0127
dentro do carrier aprovado, sem nova API, default ou fase.

## P1308 — origem da closure capturada nos parâmetros

Medição anterior à decisão: baseline P1308 SHA-256
`62c53690cbe3dd36b5b79168ea59a32f6eb88cc52ea52a8ca97365c5a9459394`,
`closures.rs:243–252` cria Func só com body/params já reduzidos. Vanilla
ratificado `typst-eval/src/call.rs:638` ancora Func em `self.params().span()`.

Na criação, anexar o span de `closure_expr.params()` ao carrier privado do
owner Func antes de devolver Value. Não usar body, chamada futura, nome ou
primeiro valor passado à closure. A origem deve sobreviver a alias, factory,
With e passagem por Args. Não muda binding, capture, sink, fluxo ou aplicação.
Este dado serve ao cast de retorno de arguments.filter; erros ocorridos na
execução do callback continuam seus erros originais. Sem campo público novo.
