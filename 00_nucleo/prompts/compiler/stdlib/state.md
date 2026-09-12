# Prompt L0 — `stdlib/state` — objeto `state` e métodos
Hash do Código: cbb13196

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/state/language-semantics.toml sha256:27acb21a5e0b2e0cb3b65de61bba5266158f3e9828392fe92a1a3b160e9d61a4

## P1339 — observação de state em contexto selecionado

### Medição anterior à decisão

HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, consumer intacto:
`state.rs:61-80,84-131,144-182` lê o snapshot em get/display/at/final;
`:348-364,378-395` contém também as nativas históricas state_final/state_at.
`display` lê o valor antes de chamar sua callback no mesmo EvalContext.
Os helpers get/at/final recebem `&EvalContext`, não `&mut EvalContext`.
A auditoria `diagnosticos/p1339-observation-integration.md` e seu recibo
registram fontes e estado exatos. São entradas contextuais, mesmo quando
lidas antes da demanda de um contador Element.

### Obrigação interna de registro e replay

Registrar as leituras realmente executadas no armazenamento privado aprovado
de `compiler/eval.md`, sem mudar as assinaturas Rust vigentes. Acesso por
referência compartilhada exige mutabilidade interior local ao contexto, não
mutação do TagIntrospector nem estado global. O registro começa com a avaliação
do bloco, sem depender de ele já ter alcançado uma leitura Element.

Cada requisição conserva key, init/fallback próprio da rota, Location ou label
original quando aplicável, span e resultado observado. Não substituir a rota
histórica por método moderno: state_at/state_final globais mantêm None onde
os métodos de State usam init. Resolução de label e sua ausência são parte
da requisição, não uma Location congelada que elimina a dependência do label.
O glue AST registra sua resolução no owner value_methods; estes helpers
continuam donos da leitura de valor.

Em display, observar o valor antes da callback/conversão. Leituras feitas
dentro da callback pertencem ao mesmo registro causal. Validar não reaplica
essa callback: repete somente as leituras registradas com o snapshot candidato
e os mesmos fallbacks. Erro posterior da callback continua sendo erro do corpo,
não um resultado bem-sucedido da leitura. A validação de leituras não executa
StateUpdate::Func nem muda sua fase preexistente de materialização.

Nenhuma operação state seleciona por si só o bloco para novas tentativas.
Sem demanda Element, o orquestrador conserva a passagem ordinária. Em bloco
selecionado, a validação usa a semântica legada da leitura, sem reparo de
paridade incidental. Criação de state/update/display diferido não é leitura.
Aceitação: state antes/depois de Element, at com label movido/ausente,
fallback init versus None e callback de display com leitura/erro. Comparar
valor opaco por repr ou ignorá-lo não certifica estabilidade; a cobertura do
comparador pertence ao owner de eval.

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/state.rs` (novo; funções exportadas para `rules/stdlib/mod.rs` e registadas em `rules/eval/mod.rs::make_stdlib`).
**Origem**: Passo 506 — fecho do gap P500 (runtime state mutável).
**ADRs**: ADR-0033 (paridade vanilla), ADR-0054 (graded parity), ADR-0107 (paridade linguagem), ADR-0118 (runtime state via context).
**Convenções partilhadas**: ver `00_nucleo/prompts/compiler/stdlib/_comum.md`.

---

## 1. Visão geral

Este módulo implementa o tipo/valor `state(key, init)` e os seus métodos `.update()`, `.get()` e `.display()`. A representação subjacente é `Value::State { key, init }` (ver `entities/value.md`).

**P737 — o binding `state` no scope global passou de `Value::Func` a
`Value::Type(Type::State)`** (paridade vanilla — medido: `type(state)` →
`type`; `type(state("y", 0)) == state` → `true`; `repr(state)` →
`"state"`). A chamabilidade mantém-se via o despacho de tipos chamáveis
de P685 (`eval/closures.rs`: `Type::State` → `native_state`).

A sintaxe vanilla suportada:

```typst
#let s = state("key", 0)
#s.update(5)
#context s.get()
#context s.display()
```

Paridade com Typst 0.15.0 para o subset identificado em P500/P506.

---

## 2. Construtor

### `native_state` — `state(key, init)`

**Assinatura**: `state(key: str, init: any) -> state`

**Argumentos**:
- `key`: string identificadora do estado.
- `init`: valor inicial.

**Semântica**:
- Rejeita argumentos nomeados.
- Primeiro argumento deve ser `Value::Str`.
- Retorna `Value::State { key, init }`.
- Quando esse valor aparece em posição de markup (não dentro de `#let` nem como argumento), o eval de markup converte-o para `Content::State { key, init }`, garantindo registo no `StateRegistry` durante o walk.

**Testes canônicos**:
```
state("total", 0) -> Value::State
state(1, 0)       -> Err "state() requer string como primeiro argumento"
state("total")    -> Err "state() requer 2 argumentos"
```

---

## 3. Métodos (field access)

`Value::State` expõe três campos funcionais. O dispatch é feito em `rules/eval/bindings.rs::eval_field_access` (braço `Value::State`).

### `.update(value)`

**Assinatura**: `state.update(value: any) -> content`

**Semântica**:
- Retorna `Content::StateUpdate { key, update: StateUpdate::Set(Box<Value>) }`.
- Não avalia o estado; apenas emite o content de update.

**Testes canônicos**:
```
let s = state("x", 0)
s.update(5) -> Content::StateUpdate("x", Set(5))
```

### `.get()`

**Assinatura**: `state.get() -> any`

**Semântica**:
- **Só pode ser chamado dentro de `context`.**
- Fora de context (`ctx.in_context == false`): erro descritivo `"state.get() can only be used inside context"`.
- Dentro de context: consulta `ctx.introspector.state.value_at(key, location)`.
  - Se houver valor, retorna-o.
  - Se não houver valor registado, retorna `init`.
- A `location` é a do `Content::ContextBlock` onde o `.get()` está contido, fornecida pelo mecanismo de expansão pós-introspecção (ADR-0118 §4).

**Testes canônicos**:
```
let s = state("x", 0)
context s.get() -> 0 (antes de update) / 5 (após s.update(5))
s.get() -> ERRO_DESCRITIVO
```

### `.display([callback])`

**Assinatura**: `state.display() -> content` / `state.display(callback: function) -> content`

**Semântica**:
- Sem callback: retorna `Content::text(value.to_string())` quando resolvido dentro de context.
- Com callback: aplica `callback(value)` e converte o resultado para `Content` (paridade `apply_state_displays`).
- Fora de context: erro descritivo.

**`value_to_content` (P821/P842)** — cobertura de tipos do display direto:
`Content`, `Str`, `Int`, `Float`, `Bool`, `Type` (nome curto, P821) e,
desde **P842 (achado #35 de P831)**, `Length`, `Ratio`, `Relative`,
`Angle` e `Fraction` com o display de `repr` (medido no vanilla:
`#context (10pt)` → "10pt", `(50%)` → "50%", `(30% + 1em)` →
"30% + 1em", `(45deg)` → "45deg", `(2fr)` → "2fr"; pré-P842 caíam no
braço `_ => Content::Empty` — página vazia).

**Testes canônicos**:
```
let s = state("x", 10)
context s.display() -> "10"
context s.display(v => [Valor: #v]) -> content
```

---

## 4. Interação com `Content::State` existente

O `Content::State` legacy (P171) continua a existir como representação locatável no content tree. A transição `Value::State → Content::State` acontece no eval de markup quando o valor é usado como content isolado. O `StateRegistry` continua populado pelo walk a partir de `Content::State` e `Content::StateUpdate`.

---

## 5. Paridade vanilla

- `state(key, init)` cria estado identificado por `key`.
- `.update(value)` marca update no documento.
- `.get()` retorna valor acumulado até o ponto do `context`.
- `.display()` formata o valor dentro de `context`.
- `.get()` fora de `context` é erro (semântica, não mecânica).

---

## 6. Nativas globais absorvidas de `foundations` (Passo 1032)

Para compatibilidade histórica, as seguintes funções de escopo global também
vivem neste módulo:

- `native_state_update(key, value)` → `Content::StateUpdate(Set(value))`.
- `native_state_update_with(key, fn)` → `Content::StateUpdate(Func(fn))` (stub P172).
- `native_state_display(key, [callback])` → `Content::StateDisplay(...)`.
- `native_state_final(key)` → valor final do state no introspector.
- `native_state_at(key, label)` → valor do state na `Location` do label.

## 7. Scope-outs

- `state.update(key, fn)` com callback funcional (typst vanilla) — continua a ser suportado via `state_update_with` existente; não faz parte deste prompt.
- Estados locais a um scope (vanilla permite state local em show-rules) — scope-out; este prompt cobre apenas estado documental global por key.

---

## 7. Testes obrigatórios

- `state("x", 0)` retorna `Value::State`.
- `s.update(5)` retorna `Content::StateUpdate` com key e valor corretos.
- `context s.get()` dentro de documento com `s.update(5)` retorna `5`.
- `s.get()` fora de context retorna erro descritivo.
- `context s.display()` retorna content textual.
- Dois estados com keys distintas não interferem.

---

## P844 (achados #49/#50/#51 de P831) — métodos `at`/`final`, `counter.at(Location)`, repr de array

- `state.at(selector)` ligado no dispatch de métodos (`bindings.rs::state_at_dispatch` → `state.rs::state_at_location`). Aceita `Location` directa (ex.: `here()`) ou `<label>` resolvida via introspector. Sem update prévio à Location, devolve o init (medido no vanilla 0.15.0). Mensagens verbatim medidas: `missing argument: selector`, `unexpected argument`, `expected label, function, location, or selector, found {type}`, `text is not locatable` (string), ``label `<x>` does not exist in the document``. Validação de argumentos precede o gate de contexto (`can only be used when context is known`) — ordem medida no vanilla.
- `state.final()` ligado no dispatch (`state.rs::state_final`). Devolve o valor final pós-walk via `Introspector::state_final_value` (P171/P240, two-pass real); sem updates, o init (medido: `state("s", 7).final()` → `7`).
- `value_to_content` usa o repr para `Value::Array` (#51): `#context ((3,))` → `(3,)` (medido; o join próprio com `.` foi removido). Reusa `eval/repr::repr_value`, a mesma rotina corrigida em P801 para o caminho directo.

## P886 (achado 2 de P885) — `value_to_content`: falta `Value::Dict`

**Sintoma medido**: `#for i in range(200) { context measure[lorem(10)] }`
(`07-context.typ`, benchmark de P872) produz página em branco no
cristalino (`stream` do content da página com `/Length 0`, confirmado nos
bytes do PDF — não é artefacto de extração de texto). O vanilla 0.15.1,
com a mesma fonte, produz `(width: 42.85pt, height: 7.24pt)` repetido por
iteração (medido via `pdftotext` em `vanilla-07-context.pdf`).

**Causa**: `measure(body)` (intercepção sintáctica em P712,
`eval/closures.rs:668`) retorna `Value::Dict` com as chaves `width` e
`height`. `value_to_content` (esta secção do módulo) tem braços
explícitos para `Content`, `Str`, `Int`, `Float`, `Bool`, `Type` (P821),
`Length`/`Ratio`/`Relative`/`Angle`/`Fraction` (P842) e `Array` (P844) —
mas **não para `Value::Dict`**, que cai no braço `_ => Content::Empty`.
O valor é calculado correctamente (P860/DEBT-69 já garante largura e
altura correctas); é o passo de conversão para `Content` visível que
descarta o dict.

**Correcção**: adicionar braço `Value::Dict(_) => Content::text(repr::
repr_value(value))`, reusando a mesma rotina de `eval/repr::repr_value`
já usada para `Array`/`Length`/etc. (mesmo padrão de P844 #51).
`repr_value` para `Value::Dict` (`eval/repr.rs:43-55`) já produz
`"{k}: {v}"` por entrada, join por `", "`, entre parênteses — formato
`(width: 42.85pt, height: 7.24pt)`, que bate com o output vanilla medido
acima (mesmos nomes de campo, mesma pontuação, mesmo formato de
`Length` via `repr_value` recursivo).

**Escopo confirmado como distinto de**: achado 3 de P885 (`table()` sem
stroke default, tratado em P887) — código e mecanismo diferentes
(`TableElem` constrói `stroke: None` em vez do default de linguagem
`1pt + black`; não passa por `value_to_content` nem por qualquer
conversão "valor computado → content"). Também confirmado como **não**
regressão do achado #34/P860 (`00_nucleo/diagnosticos/typst-passo-860-
relatorio.md`) — P860 corrigiu a exactidão numérica de
`measure_content_real` (largura+altura), validado com blocos `context`
que devolviam array/tupla (já coberto pelo braço `Array` desde P844);
nunca exercitou o retorno directo do dict de `measure()` sem
desestruturar. Caminho de código nunca coberto, não regressão.

**Testes canónicos**:
```
value_to_content(&Value::Dict({"width": Length::pt(42.85), "height": Length::pt(7.24)}))
  -> Content::text("(width: 42.85pt, height: 7.24pt)")
value_to_content(&Value::Dict(IndexMap::default()))
  -> Content::text("(:)")   // dict vazio, paridade com repr_value (P695)
```

## P1353 — retirada do hook temporário de `state.display`

A chamada destinada exclusivamente ao ledger condicionado por
`p1339_observation` foi sucedida por P1353 e deixa de ser obrigação
materializável. A evidência P1342 permanece histórica, sem legitimar código de
telemetria residual.

Continuam vigentes a leitura contextual produtiva, a aplicação única do
callback real, seus argumentos, resultado, erro e conversão em conteúdo.
Nenhum callback adicional nem evento fabricado substitui o hook removido.
Instrumentação futura requer nova medição, atualização L0 e `cfg` formalizado.
