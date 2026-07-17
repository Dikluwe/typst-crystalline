---

# P506 — Materialização de Runtime State: `state`, `counter`, `context`

> **Passo:** 506
> **Data:** 2026-06-29
> **Foco:** Fechar empiricamente o último gap de paridade funcional: runtime state mutável (`state`, `counter`, `context`). Não declarar conclusão — medir antes e depois de cada sub-tarefa.
> **Tipo:** Implementação arquitetural L-size com validação via bateria P500.
> **Tamanho:** L (~3-4 dias de implementação + 1 dia de validação).
> **ADR-0075 ACEITE** — mecanismo de comparação via `typst query --format json` vs `Introspector::query_*`.
> **ADR-0054 ACEITE** — graded parity: MATCH / DIFF / ERRO_DESCRITIVO / PANIC / AUSENTE.
> **ADR-0109 ACEITE** — atomização de código.
> **ADR-0110 NOVO** — runtime state mutável no cristalino (este passo).
> **Dependências:** P505 (todos os gaps S/M/XS fechados), P500 (audit que identificou o gap).

---

## 1. Contexto

O audit P500 identificou que o cristalino não implementa o **runtime state mutável** do vanilla Typst. Este é o último gap de paridade funcional:

```typst
// test-state-counter.typ (P500)
#let s = state("key", 0)
#s.update(5)
#context s.get()

#counter(heading).update(1)
#context counter(heading).get()
```

- **Vanilla 0.15.0:** `ok` — state e counter funcionam com delayed evaluation via `context`.
- **Cristalino (pré-506):** `ERRO_DESCRITIVO` — `state` não reconhecido; `counter` não reconhecido; `context` não reconhecido.
- **Classificação P505:** **AUSENTE** (runtime state não existe no cristalino)

Este gap é **L-size** porque requer:
1. Novos tipos de dados (`Value::State`, `Value::Counter`).
2. Novo mecanismo de execução (delayed evaluation via `context`).
3. Modificação do layout engine para aplicar updates e resolver context blocks.
4. Sistema de scoping para state/counter (global vs. local ao documento).

---

## 2. Análise Arquitetural do Vanilla

### 2.1 Modelo Mental

No vanilla Typst, o documento é avaliado em **duas fases**:

1. **Fase 1 (Eval):** O AST é avaliado de forma pura e funcional. `state` e `counter` criam **referências** a valores mutáveis, mas não os avaliam. `context` cria um **closure** que será avaliado posteriormente.
2. **Fase 2 (Layout):** O layout engine percorre o conteúdo, aplica `update`s de state/counter em ordem, e resolve `context` blocks com os valores atuais.

Isso significa que `state.get()` e `counter.get()` **não podem ser chamados fora de `context`** — não há valor "atual" durante o eval puro.

### 2.2 Semântica de `state`

```typst
#let s = state("key", 0)     // cria state com chave "key" e valor default 0
#s.update(5)                   // marca update para 5 (não avalia imediatamente)
#context s.get()               // avaliado no layout: retorna 5
#context s.display()           // avaliado no layout: retorna "5"
```

- `state(key, default)` → cria `State` com identificador único (chave + localidade).
- `state.update(value)` → registra um update no documento. O update é aplicado quando o layout engine processa esse ponto.
- `state.get()` → só válido dentro de `context`. Retorna o valor acumulado até aquele ponto.
- `state.display()` → só válido dentro de `context`. Formata o valor.

### 2.3 Semântica de `counter`

```typst
#counter(heading).update(1)    // define contador de heading para 1
#context counter(heading).get() // retorna 1
#context counter(heading).display("1.") // retorna "1."
```

- `counter(selector)` → cria `Counter` associado a um tipo de elemento (heading, figure, etc.).
- `counter.update(value)` → define o valor do contador.
- `counter.step()` → incrementa o contador (usado implicitamente por headings).
- `counter.get()` → só válido dentro de `context`. Retorna array de valores (um por nível).
- `counter.display(pattern)` → só válido dentro de `context`. Formata o array.
- `counter.at(label)` → retorna valor do counter no ponto do label (0.15.0).

### 2.4 Semântica de `context`

```typst
#context { s.get() + 1 }
#context [O valor é #s.get()]
```

- `context { expr }` → cria um `ContextBlock` que delaya a avaliação de `expr`.
- Durante o layout, quando o engine encontra um `ContextBlock`, ele avalia `expr` com o estado atual.
- `context` pode conter qualquer expressão, mas só expressões que leem state/counter precisam dele.

---

## 3. Arquitetura Proposta para o Cristalino

### 3.1 Princípios de Design

1. **Mínima invasão:** Não alterar o modelo de eval funcional existente. Adicionar uma **camada de runtime** sobre o eval.
2. **Lazy evaluation:** `state` e `counter` são lazy values. `context` é um lazy block.
3. **Determinismo:** O resultado do layout deve ser determinístico, independente da ordem de avaliação.
4. **Paridade:** Comportamento idêntico ao vanilla 0.15.0.

### 3.2 Componentes

```
┌─────────────────────────────────────────────────────────────┐
│  Fase 1: Eval (pura, funcional)                            │
│  - state(key, default) → Value::State(key, default)         │
│  - counter(selector) → Value::Counter(selector)             │
│  - context { expr } → Value::ContextBlock(closure)         │
│  - state.update(v) → Value::StateUpdate(state, v)           │
│  - counter.update(v) → Value::CounterUpdate(counter, v)     │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│  Fase 2: Layout (com runtime state)                          │
│  - StateStore: HashMap<StateId, Value>                     │
│  - CounterStore: HashMap<CounterId, Vec<i64>>             │
│  - Para cada ContextBlock: avaliar com estado atual        │
│  - Para cada StateUpdate/CounterUpdate: aplicar ao store  │
└─────────────────────────────────────────────────────────────┘
```

### 3.3 Novos Tipos de Dados

**Arquivo alvo:** `src/entities/value.rs` (ou equivalente)

```rust
pub enum Value {
    // ... variants existentes
    State(State),
    Counter(Counter),
    ContextBlock(ContextBlock),
    StateUpdate(StateUpdate),
    CounterUpdate(CounterUpdate),
}

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    pub key: EcoString,
    pub default: Box<Value>,
    pub id: u64, // identificador único (auto-incrementado)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Counter {
    pub selector: Selector,
    pub id: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ContextBlock {
    pub closure: Func, // closure que será avaliada no layout
}

#[derive(Clone, Debug, PartialEq)]
pub struct StateUpdate {
    pub state: State,
    pub value: Box<Value>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CounterUpdate {
    pub counter: Counter,
    pub value: i64, // ou enum Step/Set(i64)
}
```

### 3.4 Runtime State Store

**Arquivo alvo:** `src/runtime/state_store.rs` (novo)

```rust
use std::collections::HashMap;

pub struct RuntimeState {
    states: HashMap<u64, Value>,       // StateId → Value
    counters: HashMap<u64, Vec<i64>>,   // CounterId → [level1, level2, ...]
}

impl RuntimeState {
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
            counters: HashMap::new(),
        }
    }

    pub fn get_state(&self, state: &State) -> Value {
        self.states.get(&state.id)
            .cloned()
            .unwrap_or_else(|| *state.default.clone())
    }

    pub fn set_state(&mut self, state: &State, value: Value) {
        self.states.insert(state.id, value);
    }

    pub fn get_counter(&self, counter: &Counter) -> Vec<i64> {
        self.counters.get(&counter.id)
            .cloned()
            .unwrap_or_else(|| vec![0]) // default [0]
    }

    pub fn set_counter(&mut self, counter: &Counter, value: Vec<i64>) {
        self.counters.insert(counter.id, value);
    }

    pub fn step_counter(&mut self, counter: &Counter) {
        let mut values = self.get_counter(counter);
        if let Some(last) = values.last_mut() {
            *last += 1;
        }
        self.set_counter(counter, values);
    }

    pub fn update_counter(&mut self, counter: &Counter, value: i64) {
        let mut values = self.get_counter(counter);
        if let Some(last) = values.last_mut() {
            *last = value;
        }
        self.set_counter(counter, values);
    }
}
```

### 3.5 Modificação do Layout Engine

**Arquivo alvo:** `src/engine/layout/mod.rs` (ou equivalente)

O layout engine precisa de:
1. Um `RuntimeState` que persiste durante todo o layout.
2. Lógica para processar `StateUpdate`, `CounterUpdate`, `ContextBlock`.

```rust
pub fn layout_content(
    content: &Content,
    ctx: &mut LayoutContext,
    runtime: &mut RuntimeState,
) -> Vec<Frame> {
    match content {
        Content::StateUpdate(update) => {
            runtime.set_state(&update.state, *update.value.clone());
            vec![] // StateUpdate não produz output visual
        }
        Content::CounterUpdate(update) => {
            runtime.update_counter(&update.counter, update.value);
            vec![]
        }
        Content::ContextBlock(block) => {
            // Avaliar o closure com o estado atual
            let result = block.closure.call_with_runtime(runtime)?;
            layout_content(&result, ctx, runtime)
        }
        // ... outros casos existentes
    }
}
```

### 3.6 Modificação do Eval

**Arquivo alvo:** `src/eval/constructors.rs` e `src/eval/bindings.rs`

O eval precisa criar `Value::State`, `Value::Counter`, `Value::ContextBlock` sem avaliá-los:

```rust
// eval/constructors.rs
fn native_state(args: Args) -> SourceResult<Value> {
    let key = args.expect::<Str>("key")?;
    let default = args.expect::<Value>("default")?;
    static NEXT_ID: AtomicU64 = AtomicU64::new(1);
    let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);

    Ok(Value::State(State {
        key: key.into(),
        default: Box::new(default),
        id,
    }))
}

fn native_counter(args: Args) -> SourceResult<Value> {
    let selector = args.expect::<Selector>("selector")?;
    static NEXT_ID: AtomicU64 = AtomicU64::new(1);
    let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);

    Ok(Value::Counter(Counter {
        selector,
        id,
    }))
}

fn native_context(args: Args) -> SourceResult<Value> {
    let closure = args.expect::<Func>("body")?;
    Ok(Value::ContextBlock(ContextBlock { closure }))
}
```

**Field access para `State`:**

```rust
Value::State(state) => match field.as_str() {
    "get" => Ok(Value::Func(StateGet { state: state.clone() })),
    "update" => Ok(Value::Func(StateUpdateFn { state: state.clone() })),
    "display" => Ok(Value::Func(StateDisplay { state: state.clone() })),
    _ => Err("state does not have field '{}'"),
},
```

**Nota:** `state.get()` e `state.display()` só podem ser chamados dentro de `context`. Se chamados fora, devem retornar erro.

```rust
fn state_get(args: Args) -> SourceResult<Value> {
    let state = args.expect::<State>("self")?;
    // Verificar se estamos dentro de um context block
    if !args.in_context() {
        return Err("state.get() can only be used inside context");
    }
    let runtime = args.runtime()?;
    Ok(runtime.get_state(&state))
}
```

---

## 4. Implementação Passo a Passo

### 4.1 Fase 1: Infraestrutura (Dia 1)

1. Criar `src/runtime/state_store.rs` com `RuntimeState`.
2. Adicionar `Value::State`, `Value::Counter`, `Value::ContextBlock`, `Value::StateUpdate`, `Value::CounterUpdate` ao enum `Value`.
3. Adicionar `Content::StateUpdate`, `Content::CounterUpdate`, `Content::ContextBlock` ao enum `Content` (se necessário — ou usar `Value` diretamente no layout).
4. Implementar `Clone`, `Debug`, `PartialEq` para os novos tipos.

### 4.2 Fase 2: Construtores (Dia 1-2)

1. Implementar `native_state`, `native_counter`, `native_context` em `eval/constructors.rs`.
2. Registrá-los no scope global.
3. Implementar field access para `State` (`get`, `update`, `display`).
4. Implementar field access para `Counter` (`get`, `update`, `step`, `display`, `at`).

### 4.3 Fase 3: Layout Engine (Dia 2-3)

1. Modificar `LayoutContext` para conter `RuntimeState`.
2. Modificar `layout_content` para processar `StateUpdate`, `CounterUpdate`, `ContextBlock`.
3. Implementar `ContextBlock` evaluation: avaliar o closure com o `RuntimeState` atual.
4. Garantir que `state.get()`/`counter.get()` só funcionam dentro de `context`.

### 4.4 Fase 4: Counter Automático (Dia 3)

1. Implementar `counter.step()` automático para headings.
2. Quando o layout engine encontra um `Content::Heading`, chamar `counter.step()` para o counter associado.
3. Implementar `counter.display(pattern)` com formatação de numbering.

### 4.5 Fase 5: Testes e Validação (Dia 4)

1. Criar `test-state-counter.typ` com casos básicos e avançados.
2. Rodar contra vanilla 0.15.0 e cristalino.
3. Verificar não-regressão em toda a bateria P490-P505.

---

## 5. Validação

### 5.1 Testes Básicos

```typst
// test-state-counter.typ

// State básico
#let s = state("my_state", 0)
#s.update(5)
#context s.get()

// Counter básico
#counter(heading).update(1)
#context counter(heading).get()

// Counter display
#context counter(heading).display("1.")
```

### 5.2 Testes Avançados

```typst
// State com múltiplos updates
#let s = state("key", 0)
#s.update(1)
#s.update(2)
#context s.get() // deve ser 2

// Counter com headings
= Capítulo 1
#context counter(heading).get() // deve ser [1]
== Secção 1.1
#context counter(heading).get() // deve ser [1, 1]
= Capítulo 2
#context counter(heading).get() // deve ser [2]

// Counter at label
#counter(heading).step()
= Heading <my-label>
#context counter(heading).at(<my-label>)

// Context com expressões complexas
#let s = state("x", 10)
#context [O valor é #s.get() e o dobro é #(s.get() * 2)]
```

### 5.3 Testes de Erro

```typst
// State.get fora de context → erro
#let s = state("key", 0)
#s.get() // deve dar ERRO_DESCRITIVO

// Counter.get fora de context → erro
#counter(heading).get() // deve dar ERRO_DESCRITIVO
```

---

## 6. Formato do Relatório de Resultados

```
| 506a state básico | Vanilla: ok | Cristalino: ok | MATCH | get/update/context |
| 506b counter básico | Vanilla: ok | Cristalino: ok | MATCH | get/update/step/display |
| 506c counter com headings | Vanilla: ok | Cristalino: ok | MATCH | step automático |
| 506d counter at label | Vanilla: ok | Cristalino: ok | MATCH | counter.at(label) |
| 506e context complexo | Vanilla: ok | Cristalino: ok | MATCH | expressões aninhadas |
| 506f erro fora de context | Vanilla: ERRO | Cristalino: ERRO | MATCH | semântica de erro |
```

Colunas: `sub-tarefa | vanilla | cristalino | classificação | notas`

---

## 7. Critério de Fecho

- [ ] 506a implementado: `state(key, default)`, `state.update()`, `state.get()` dentro de `context`.
- [ ] 506b implementado: `counter(selector)`, `counter.update()`, `counter.step()`, `counter.get()`, `counter.display()` dentro de `context`.
- [ ] 506c implementado: `counter.step()` automático para headings.
- [ ] 506d implementado: `counter.at(label)` (0.15.0).
- [ ] 506e implementado: `context` com expressões complexas (texto, operações, etc.).
- [ ] 506f implementado: `state.get()`/`counter.get()` fora de `context` → ERRO_DESCRITIVO.
- [ ] 6 testes unitários novos passam (`p506_state_basic`, `p506_counter_basic`, `p506_counter_headings`, `p506_counter_at_label`, `p506_context_complex`, `p506_error_outside_context`).
- [ ] Bateria P490: 20/20 MATCH preservado (não-regressão).
- [ ] Bateria P500: 17/17 MATCH (test-state-counter.typ agora MATCH).
- [ ] PANICs: 0 (preservado).
- [ ] Documentação arquitetural atualizada (ADR-0110: Runtime State Mutável).
- [ ] Documentação L0 atualizada (`rules/stdlib/state.md`, `rules/stdlib/counter.md`, `rules/stdlib/context.md`).
- [ ] ADR-0107 checklist atualizado (6 itens marcados implementado).
- [ ] Sentinela `p506_runtime_state` adicionada em `lab/parity/tests/structural_parity.rs`.
- [ ] `00_nucleo/diagnosticos/paridade-funcional-p506.md` produzido com tabela de resultados.
- [ ] **Bateria P490 + P500: 37/37 MATCH** — paridade funcional completa alcançada.

---

## 8. Próximo Passo (P507)

Com P506 fechado, o cristalino atinge **paridade funcional completa** com o Typst 0.15.0.

**Recomendações para P507+:**

1. **P507 = DEBT-42 Benchmark** — comparar tempo de compilação cristalino vs vanilla 0.15.0, avaliar impacto da passagem dupla do P498 e do runtime state do P506.
2. **P508 = Documentação de paridade final** — produzir relatório consolidado P490-P506 com matriz de cobertura completa.
3. **P509 = Lookahead Layout Engine** — implementar a proposta de lookahead de 2 páginas (inovação arquitetural, não paridade).
4. **P510 = Publicação** — artigo sobre a arquitetura cristalina e a metodologia de paridade funcional.

---

## A. Apêndice — Referência Rápida

```typst
// 506a: state básico
#let s = state("key", 0)
#s.update(5)
#assert.eq(context s.get(), 5)

// 506b: counter básico
#counter(heading).update(1)
#assert.eq(context counter(heading).get(), (1,))
#assert.eq(context counter(heading).display("1."), "1.")

// 506c: counter com headings
= Capítulo 1
#assert.eq(context counter(heading).get(), (1,))
== Secção 1.1
#assert.eq(context counter(heading).get(), (1, 1))

// 506d: counter at label
#counter(heading).step()
= Heading <my-label>
#assert.eq(context counter(heading).at(<my-label>), (1,))

// 506e: context complexo
#let s = state("x", 10)
#context [Valor: #s.get(), Dobro: #(s.get() * 2)]

// 506f: erro fora de context
#let s = state("key", 0)
// #s.get() // ERRO: state.get() can only be used inside context
```

---

## B. Apêndice — Decisão Arquitetural: Por que Runtime State é L-size

| Aspecto | Complexidade | Justificativa |
|---------|-------------|---------------|
| Novos tipos de dados | Média | `State`, `Counter`, `ContextBlock` requerem variants novos em `Value` e `Content` |
| Runtime state store | Média | `HashMap` global precisa de lifecycle management (criação/destruição por documento) |
| Delayed evaluation | Alta | `context` blocks precisam de closures que capturam o ambiente de eval |
| Layout engine modification | Alta | O layout engine precisa processar updates em ordem e resolver context blocks |
| Counter automático | Alta | Headings precisam de hook no layout para chamar `counter.step()` |
| Semântica de erro | Média | `state.get()` fora de `context` precisa detectar se está em context block |
| Determinismo | Alta | Múltiplos `state.update` e `context` blocks precisam de ordem determinística |

**Total:** 5-6 aspectos de complexidade média-alta = **L-size** (3-4 dias).
