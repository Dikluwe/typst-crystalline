# Prompt L0 — `stdlib/state` — objeto `state` e métodos
Hash do Código: 7479f57f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/state/language-semantics.toml sha256:27acb21a5e0b2e0cb3b65de61bba5266158f3e9828392fe92a1a3b160e9d61a4

## P1339 — observação de state em contexto selecionado

### Medição anterior à decisão

HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, consumer intacto:
`state.rs:61-80,84-131,144-182` lê o snapshot em get/display/at/final;
`:348-364,378-395` contém também as nativas históricas state_final/state_at.
`display` lê o valor antes de chamar sua callback no mesmo EvalContext.
Os helpers get/at/final recebem `&EvalContext`, não `&mut EvalContext`.
A auditoria histórica
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

## A1 — reparo de paridade medido em 2026-10-17

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c; estado/UTC/fontes em
00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17/before.json.gz.
Vanilla bd5ebec285505889e8ff1d4f68804ab147a699f3. before-pdf-probe.json.gz
no mesmo diagnóstico congela controles bilaterais de campo, admissão, callback
e init. A auditoria anterior paridade-novo-vanilla-2026-10-17/inventario.md
E02–E04 registra os limites de carrier e as fontes discriminantes; não se usa
dump/estrutura Rust como critério. Estes observáveis são linguagem.

### Responsabilidade exclusiva e aceitação

state(key) é válido, com init Value::None; state(key,init) conserva init.
Rejeitar chave não-string, nomeados e aridade fora de 1–2 sem avaliar nada extra.
Esta seção supersede o teste histórico que rejeitava state("total"). A correção
de paridade não muda campo/assinatura Rust. A proposta A1-S1 de transportar init no update foi retirada: upstream
introspection/state.rs:226–245 e 477–520 exige init do objeto leitor e callbacks
sob demanda. Replay funcional depende de revisão corrigida separada; não simular essa informação
por estado global, Sequence auxiliar ou execução antecipada de callback.

L0 primeiro, RED funcional e GREEN bilateral. Não alterar contrato público,
default de produto, fase ou compatibilidade; a revisão corrigida de State é independente. Preservar
1:1 e núcleos pinados; resselo após grafo válido. Build/workspace/Clippy estrito,
Rustdoc/fmt/linhagem estrita e reversa, jobs sequenciais sob MemoryMax12GiB,
High8GiB/swap0/jobs1. Debug0/incremental0 somente no ambiente de validação para
reduzir picos; comparar consumo medido sem inferir ganho do produto.


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

Modernos get/at/final resolvem a sequência de ações a partir do init do receiver.
Capturar o resultado completo (inclusive erro) na observação causal existente.
Display de State lê o mesmo valor antes de sua callback; essa extensão Tekt não
é método do vanilla atual. Replay delega a leitura com Engine, conserva contexto
capturado e não reaplica a callback externa de display nem o corpo contextual.

Alterar apenas as cinco assinaturas ratificadas: state_get/state_final recebem
(state: &State, ctx: &mut EvalContext, engine: &mut Engine<'_>, span: Span);
state_at_location recebe state, Location, ctx mutável, Engine mutável, Span.
Todas retornam SourceResult<Value>. native_state_final/native_state_at passam
a (ctx,args,world,current_file,scopes,engine) no ABI NativeFuncWithEngine já
existente. Sem novos campos públicos, variantes públicas ou mudança de traits.

Declarar descendente privado state/resolve.rs para o fold. Adapter crate-private
state_method_update projeta Func em StateUpdate::Func e delega outros valores
a state_update; este helper público antigo permanece construtor Set com mesma
assinatura. Leituras legadas por key começam em init legado elegível ou None,
compartilham ações modernas/legadas, e mantêm ausência de chave/label/validação.
Adapters sem ações preservam fixtures/valores materializados da API legada.
O helper privado de leitura legada com registry explícito também serve displays.

Testes R1–R12 antes do código, incluindo callback não lido, init por objeto,
Func/Set/Func, erro, captura e contexto restaurado. Preservar coerções de display
que não pertencem a este reparo. As cláusulas antigas que fixavam leituras sem
Engine, update Func fora do escopo ou replay sempre materializado são sucedidas.

### Aceitação e fronteiras

Gate A1-S2 ratificado para as cinco assinaturas e a fase de leitura. L0 primeiro,
RED funcional antes do código; GREEN contra o pin indicado para linguagem e
controles próprios das extensões Tekt. Não ampliar outros contratos públicos.
Build/workspace/doctests, Clippy/Rustdoc estritos, fmt, linhagem V1/V5/V15/V26 e
reversa; manter V16 sem aumento. MemoryMax12GiB/High8GiB/swap0/jobs1, tarefas
pesadas sequenciais. Medir consumo sem inferir ganho global só da representação.
