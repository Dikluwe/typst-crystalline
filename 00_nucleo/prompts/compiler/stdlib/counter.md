# Prompt L0 — `stdlib/counter` — objeto `counter` e métodos
Hash do Código: 4ac1e283

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/counter.rs` (novo; funções exportadas para `rules/stdlib/mod.rs` e registadas em `rules/eval/mod.rs::make_stdlib`).
**Origem**: Passo 506 — fecho do gap P500 (runtime state mutável).
**ADRs**: ADR-0033 (paridade vanilla), ADR-0054 (graded parity), ADR-0107 (paridade linguagem), ADR-0118 (runtime state via context).
**Convenções partilhadas**: ver `00_nucleo/prompts/compiler/stdlib/_comum.md`.

---

## 1. Visão geral

Este módulo implementa o construtor `counter(selector)` e os seus métodos `.update()`, `.step()`, `.get()`, `.display()` e `.at()`. A representação subjacente é `Value::Counter { key: CounterKey }` (ver `entities/counter.md`).

**P1018** — corrige colisão entre `counter("nome")` (string de utilizador)
e `counter(elemento)` (selector de elemento). A chave passa a ser
`CounterKey::Str(EcoString)` vs `CounterKey::Selector(Selector)`, espelhando
o `CounterKey` do vanilla (`typst-library/src/introspection/counter.rs:527`).

**P737 — o binding `counter` no scope global passou de `Value::Func` a
`Value::Type(Type::Counter)`** (paridade vanilla — medido: `type(counter)` →
`type`; `type(counter("x")) == counter` → `true`; `repr(counter)` →
`"counter"`). A chamabilidade mantém-se via o despacho de tipos chamáveis
de P685 (`eval/closures.rs`: `Type::Counter` → `native_counter`).

A sintaxe vanilla suportada:

```typst
#counter(heading).update(1)
#context counter(heading).get()
#context counter(heading).display("1.")
#counter(heading).step()
= Heading <my-label>
#context counter(heading).at(<my-label>)
```

Paridade com Typst 0.15.0 para o subset identificado em P500/P506.

---

## 2. Construtor

### `native_counter` — `counter(selector)`

**Assinatura**: `counter(selector: str | selector) -> counter`

**Argumentos**:
- `selector`: string de kind (ex: `"heading"`, `"figure"`, `"equation"`) ou `Value::Selector` que designa um kind.

**Semântica**:
- Rejeita argumentos nomeados.
- Aceita `Value::Str` ou `Value::Selector`.
- Converte o selector para `CounterKey`:
  - `"heading"` → `CounterKey::Str("heading")`
  - `Selector::Kind(Heading)` → `CounterKey::Selector(Selector::Kind(Heading))`
  - outras forms de selector → `CounterKey::Selector` correspondente ou erro
    se não mapeável.
- Retorna `Value::Counter { key }`.

- Aceita também `Value::Func` quando é a **função nativa de um elemento com
  counter**, resolvida por `fn_addr_eq`. Tabela (P1016 acrescenta `footnote`;
  P1018 converte para `CounterKey::Selector`):

  | função | chave |
  |---|---|
  | `heading` | `Selector::Kind(Heading)` |
  | `figure` | `Selector::Kind(Figure)` |
  | `table` | `Selector::Kind(Table)` |
  | `footnote` | `Selector::Kind(Footnote)` |

  Função de elemento fora desta tabela → erro. A tabela cresce quando um
  elemento é promovido a locatable com counter próprio; não antes.

**Testes canônicos**:
```
counter("heading")  -> Value::Counter { key: CounterKey::Str("heading") }
counter(heading)    -> Value::Counter { key: CounterKey::Selector(Kind(Heading)) }
counter(footnote)   -> Value::Counter { key: CounterKey::Selector(Kind(Footnote)) }  (P1016)
counter("footnote") -> Value::Counter { key: CounterKey::Str("footnote") }  — distinto
                       do de elemento; paridade vanilla, onde
                       `counter("footnote").get()` é `(0,)`
counter(1)          -> Err "counter() requer string ou selector"
```

---

## 3. Métodos (field access)

`Value::Counter` expõe cinco campos funcionais. O dispatch é feito em `rules/eval/bindings.rs::eval_field_access` (braço `Value::Counter`).

### `.update(value)`

**Assinatura**: `counter.update(value: int) -> content`

**Semântica**:
- Retorna `Content::CounterUpdate { key, action: CounterUpdate::Update(value) }`.
- Define o counter para `[value]` (reseta hierarquia).

**Testes canônicos**:
```
counter("heading").update(5) -> Content::CounterUpdate("heading", Update(5))
```

### `.step()`

**Assinatura**: `counter.step() -> content`

**Semântica**:
- Retorna `Content::CounterUpdate { key, action: CounterUpdate::Step }`.
- Equivalente a incrementar o counter na posição onde é inserido.

**Testes canônicos**:
```
counter("heading").step() -> Content::CounterUpdate("heading", Step)
```

### `.get()`

**Assinatura**: `counter.get() -> array`

**Semântica**:
- **Só pode ser chamado dentro de `context`.**
- Fora de context: erro descritivo `"counter.get() can only be used inside context"`.
- Dentro de context: consulta `ctx.introspector.counters.value_at(key, location)`.
  - Se houver valor, retorna `Value::Array(Vec<Value::Int>)` com o slice.
  - Se não houver valor registado, retorna `Value::Array(vec![Value::Int(0)])`.

**Testes canônicos**:
```
counter("heading").update(1)
context counter("heading").get() -> (1,)
```

### `.display(pattern?)`

**Assinatura**: `counter.display() -> content` / `counter.display(pattern: str) -> content` / `counter.display(callback: function) -> content`

**Semântica**:
- Sem argumento: formata o slice com join `"."` (ex: `[1, 2]` → `"1.2"`).
- Com string `pattern`: aplica o pattern de numbering (paridade vanilla `counter.display("1.")`).
- Com callback: passa `Value::Array(Vec<Value::Int>)` ao callback e converte o resultado para `Content`.
- Fora de context: erro descritivo.

**Testes canônicos**:
```
counter("heading").update(1)
context counter("heading").display()     -> "1"
context counter("heading").display("1.") -> "1."
```

### `.at(label)`

**Assinatura**: `counter.at(label: label | str) -> array`

**Semântica**:
- Aceita `Value::Label` ou `Value::Str` com nome do label.
- Resolve a `Location` do label via `ctx.introspector.query_by_label`.
- Retorna o valor do counter nessa location como `Value::Array(Vec<Value::Int>)`.
- Pode ser usado dentro ou fora de context (paridade vanilla 0.15.0).
- Se label/counter inexistente → `Value::Array(vec![])`.

**Testes canônicos**:
```
counter("heading").step()
= Heading <my-label>
context counter("heading").at(<my-label>) -> (1,)
```

---

## 4. Interação com `CounterRegistry` existente

O `CounterRegistry` passa a indexar por `CounterKey` (não por `String`). É
populado pelo walk a partir de `Content::CounterUpdate` (com `CounterKey`
devolvido pelo counter) e dos arms `Content::Heading`/`Figure`/`Table`/
`Footnote` com `CounterKey::Selector(Selector::Kind(...))`. Os métodos
`.update()` e `.step()` apenas emitem `Content::CounterUpdate`; o runtime
state real vive no `CounterRegistry`/`TagIntrospector`.

---

## 5. Counter automático para elementos locatable

Mantém-se o comportamento existente, mas a chave passa a ser
`CounterKey::Selector(Selector::Kind(...))`:

- `Content::Heading` → `apply_hierarchical_at(CounterKey::Selector(Kind(Heading)), level, loc)`.
- `Content::Figure` → `apply_at(CounterKey::Selector(Kind(Figure)), Step, loc)`.
- `Content::Table` → `apply_at(CounterKey::Selector(Kind(Table)), Step, loc)`.
- `Content::Footnote` → `apply_at(CounterKey::Selector(Kind(Footnote)), Step, loc)`.

---

## 6. Paridade vanilla

- `counter(selector)` cria counter identificado pela chave do selector.
- `.update(n)` define o valor do counter.
- `.step()` incrementa o counter.
- `.get()` retorna array de valores dentro de `context`.
- `.display(pattern)` formata o counter dentro de `context`.
- `.at(label)` retorna valor do counter no ponto do label.

---

## 7. Scope-outs

- Counters sobre tipos de elemento sem kind string mapeável.
- Counters por `Label` ou `Location` (paridade futura).

---

## 8. Nativas globais absorvidas de `foundations` (Passo 1032)

Para compatibilidade histórica, as seguintes funções de escopo global também
vivem neste módulo:

- `native_counter_display(key, [callback])` → `Content::CounterDisplayCallback(...)`.
- `native_counter_at(key, label)` → `Value::Str` formatado hierarquicamente.
- `native_counter_final(key)` → `Value::Str` com o valor final do counter.
- `native_counter_step(key)` → `Content::CounterUpdate(Step)`.

---

## 9. Testes obrigatórios

- `counter("heading")` retorna `Value::Counter { key: CounterKey::Str("heading") }`.
- `counter(heading)` retorna `Value::Counter { key: CounterKey::Selector(Kind(Heading)) }`.
- Ambos os counters anteriores coexistem sem colidir no `CounterRegistry`.
- `c.update(5)` retorna `Content::CounterUpdate` correto.
- `c.step()` retorna `Content::CounterUpdate` com `Step`.
- `context c.get()` retorna array correto após update.
- `context c.display("1.")` retorna content textual correto.
- `c.at(<label>)` retorna array correto.
- `c.get()` fora de context retorna erro descritivo.

## P1148 — fields estáticos do valor-tipo

Medição nos dois binários vanilla ratificados confirmou seis functions em
`Type::Counter`: `get`, `display`, `at`, `final`, `step`, `update`. Cada forma
estática recebe `self: Counter` como primeiro positional e deve ser
semanticamente equivalente ao método de instância.

O owner expõe `counter_type_field` por match fechado. Wrappers estáticos
delegam aos mesmos helpers `counter_get/display/at/final/step/update`; não
usam as nativas históricas baseadas em string. Funções contextuais usam ABI
com Engine. `at(<label>)` deve preservar a semântica do literal label mesmo
quando a avaliação genérica do argumento não o representar; se necessário,
o eval intercepta somente essa forma sintática e continua a delegar ao owner.

### Gate descoberto durante a execução

A auditoria literal refutou a classificação inicial de “somente glue”. A
fonte ratificada mede `step(level: 1)` e `update` com inteiro, array ou
callback, enquanto o domínio cristalino público só transporta `Step` sem
nível e `Update(usize)`. P1148 propõe a substituição documentada em
`entities/counter_update.md`: `Set(Vec<usize>)`, `Step(NonZeroUsize)` e
`Func(Func)`.

Essa substituição é mudança de contrato público. Este L0 fica nucleado como
proposta, mas testes e implementação devem parar no gate ADR-0127 até
confirmação explícita do dono. Depois da aprovação, a materialização segue em
fluxo RED→GREEN e os seis wrappers estáticos são implementados junto com a
paridade das formas de instância.

## P1149 — fechamento contextual de `display`, `at` e callbacks

Medição reproduzida nos dois binários do vanilla ratificado `a51e02804`:

```text
update((2,3)); update((a,b)=>(a+1,b+2)) → get() == (3,5)
display("1.1") == counter.display(self,"1.1") == "3.5"
display("1 / 1", at:<probe>, both:true) == "3 / 4"
at(<probe>) == counter.at(self,<probe>) == (3,5)
```

Callbacks de `update` e numbering recebem cada componente do estado como
argumento posicional separado. `both: true` forma
`estado_at + [final.first()]` antes do numbering. `at: auto` usa a location
contextual; label/location custom resolve o estado nessa location. As formas
estática e de instância partilham um único owner semântico. Literal label na
chamada estática é preservado por intercepção sintática mínima, pois a
avaliação genérica ainda pode apagá-lo. Numbering omitido/`auto` mantém o
lookup da chain e fallback `"1.1"`.

É correção interna de paridade sobre o contrato P1148 já aprovado; não cria
assinatura Rust pública, default novo ou fase nova. Fluxo contínuo ADR-0127.

---

## P844 (achados #49/#50/#52/#53 de P831) — `final`, `at(Location)`, `display` real

- `counter.final()` ligado no dispatch (`counter.rs::counter_final` + `Introspector::counter_final_values`): devolve array de inteiros com os valores no fim do documento; counter nunca tocado → `(0,)` (medido no vanilla 0.15.0).
- `counter.at()` aceita `Location` directa (ex.: `here()`) além de label/string (#50) — `counter.rs::counter_at_location`; fallback `[0]` como `counter_get`. Mensagens verbatim medidas: `missing argument: selector`, `unexpected argument`, `expected label, function, location, or selector, found {type}`. Nota: `<label>` inexistente mantém o comportamento pré-P844 (array vazio `()`; vanilla erro ``label `<x>` does not exist in the document``) — divergência conhecida, fora do escopo do achado.
- `counter.display(pattern)` (#53): o stub "Pattern minimal" foi removido; usa `structural::format_pattern` (P793) — estilos romano/alfabético/circled (`①`), descarte de tokens extra e repetição do último token, paridade medida (`II B ii ② 2` para counter=2).
- `counter.display()` sem argumento (#52): usa o numbering activo do contexto via custom `"{key}.numbering.pattern"` da chain (canal `rules.rs`); sem pattern na chain, mantém o join hierárquico.

## P1159 — `counter(page)` alimenta o número lógico

O constructor reconhece a função nativa `page` e produz `CounterKey::Page`.
O snapshot final desse counter fornece o início lógico ao ciclo de realização;
as páginas seguintes incrementam a partir dele e o total passado ao callback é
o último número lógico, nunca apenas `pages.len()`. A mecânica física de layout
permanece separada do observável lógico.

## P1307-R4 — consumo coerente de Args aprovado

### Medição anterior à decisão

Baseline R4 SHA-256
`52df1c661c20d9eb612bbd5c89ae3cae8c44735aeab27c11e4a1da0d4d145e57`,
HEAD `b303f1f15b610e09872b567027e0d806387fde8c` mais working tree P1306 e
L0 R3 aprovado, contém diff/stat e fontes. Em
`01_core/src/compiler/stdlib/counter.rs:158-160` um clone perde o receiver
somente na view; `:216-220` consome level nominal ou positional. Após eval_args
produzir Some, essas operações deixariam ocorrências consumidas no carrier.

### Decisão

Usar `remove_positional(0)` no clone do wrapper estático e
`remove_named("level").or_else(|| remove_positional(0))` no consumo de level,
conforme a API já aprovada de `entities/args.md`. Preservar o clone de origem,
último valor nominal, prioridade nominal/positional, default 1, casts,
mensagens, âncoras agregadas e critérios de sobra. Some regenera views; None
mantém a política legada. Não adicionar casts de todas as duplicatas a estas
nativas nem alterar CounterAction, callbacks, numbering, contexto ou fases.

Testar os wrappers e o consumo com Some/None, inclusive default, sobra e
receiver ausente; os argumentos consumidos não podem permanecer no carrier.
É adaptação interna em fluxo contínuo ADR-0127 à API Args aprovada, não novo
contrato público de Counter nem aprovação de dívidas históricas deste L0.
