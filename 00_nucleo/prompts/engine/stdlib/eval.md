# Prompt L0 — `stdlib/eval` — runtime de re-avaliação
Hash do Código: f9a753b4

**Camada**: L1
**Ficheiro alvo**: `01_core/src/engine/stdlib/eval.rs`
**Origem**: Passo 394 (`typst-passo-394.md`) — dívida genuína acidental (balde D), M. **P814** (`typst-passo-814.md`): `mode:`/`scope:`, mensagens de cast vanilla, span sintético (achado #1 de P810).
**ADRs**: ADR-0033 (paridade vanilla), ADR-0107 (paridade linguagem), ADR-0036/0044 (estado do eval).

---

## 1. Contexto

O vanilla expõe `eval(source, mode:, scope:)` — re-parseia e re-avalia uma string como código Typst. Exemplos:

```typst
#let x = 1
#eval("x + 2")                        // 3 (cristalino: vê o scope do chamador — ver §4)
#eval("[*bold*]")                     // content com strong
#eval("= Heading", mode: "markup")    // heading (P814)
#eval("x + 1", scope: (x: 2))         // 3 (P814)
```

## 2. Arquitetura

- **Sem tipo `Value` novo**.
- **ABI alargado para nativas**: `FuncRepr::NativeWithEngine` para funções que precisam de `Scopes` e `Engine`.
- **Parser por modo (P814)**: `parse_anchored(text, mode, anchor)` em `engine/parse/mod.rs` — despacha `parse_code`/`parse`/`parse_math` e ancora todos os spans ao `anchor` via `SyntaxNode::synthesize` (equivalente ao `SpanMode::Uniform(span)` do vanilla em `eval_string`). Reutilizável por outros pontos que avaliam strings sintéticas (P815, P819).
- **Erros de sintaxe reais (P814)**: `root.errors()` propaga mensagem + hints do parser com o span âncora — não erro genérico com `<detached>`.
- **Scope actual + `scope:`**: a re-avaliação vê as variáveis do scope onde `eval` é chamado (cristalino — ver divergência §4). Os bindings do dict `scope:` são definidos num âmbito próprio (`scopes.enter()`/`exit()`): sombreiam o chamador durante o eval e não vazam. `#let` dentro do eval fica confinado ao eval (paridade vanilla, medido em P814).
- **Engine local**: `#set`/`#show` dentro do string avaliado são confinados a uma engine local, não afectando o chamador (paridade com content block).

## 3. Função nativa

`native_eval(ctx, args, world, current_file, scopes, engine)`:

- `source`: único argumento posicional obrigatório, `Str`. Default de `mode:` é `"code"` — **o default vanilla é `SyntaxMode::Code`** (`foundations/mod.rs:279`, `#[default(SyntaxMode::Code)]`; medido em P810/P814 — a afirmação anterior de que era `"markup"` era falsa).
- `mode:` (named, P814): `"code"`/`"markup"`/`"math"`. String não-listada → `expected "markup", "math", or "code"`; outro tipo → mesma mensagem + `, found {tipo}` (nomes longos vanilla: `integer`, `string`, `boolean` — `long_type_name` de `eval/bindings.rs`).
- `scope:` (named, P814): `Dict`. Outro tipo → `expected dictionary, found {tipo}`.
- Named arg desconhecido → `unexpected argument: {nome}`.
- Posicional não-string → `expected string, found {tipo}`; sem posicionais → `missing argument: source`; posicionais a mais → `unexpected argument`. (Mensagens verbatim do vanilla, medidas por sonda em P814 — a mensagem é o observável, ADR-0107.)
- Modo `math`: o resultado é embrulhado em `Content::Equation` com `block: false` (paridade `EquationElem::new(..).with_block(false)`).
- **Span âncora**: `args.span` (span da lista de argumentos, P772s — o cristalino não tem spans por-argumento; o vanilla ancora ao literal string. Nuance de uma coluna registada no relatório de P814).

## 4. Paridade vanilla

A paridade é semântica (ADR-0107). **Divergência declarada e mantida (decisão deste L0, medida em P814):** o cristalino avalia no **scope do chamador** (`#let x = 5` antes de `#eval("x * 2")` → `10`); o vanilla cria um `Scopes` fresco (só stdlib + `scope:`), logo `#let y = 10 \n #eval("y + 1")` → `error: unknown variable: y` no vanilla e `11` no cristalino. Registado como achado para decisão futura de L0 (alterar exige revisão deste prompt + do teste `eval_ve_escopo_actual`).

## 5. Testes

Em `engine/eval/tests.rs` (secções P394 e P814) e `engine/parse/mod.rs` (P814):

- `eval("1 + 2")` → `3`; `#let x = 5; eval("x * 2")` → `10` (divergência §4 declarada).
- `eval("[*bold*]")` → `Value::Content` com `Content::Strong`.
- `eval("= Heading", mode: "markup")` → `Content::Heading`; `eval("1 + 2", mode: "code")` → `3`; `eval("x + y", mode: "math")` → `Content::Equation` com `block: false`.
- `eval("x + 1", scope: (x: 2))` → `3`; bindings de `scope:` sombreiam o chamador e não vazam.
- Erros: `expected string, found integer`; `missing argument: source`; `unexpected argument`; `unexpected argument: foo`; `expected dictionary, found integer`; `expected "markup", "math", or "code"` (+ `, found integer`).
- Erro de sintaxe dentro do string (`eval("1 +")`) → mensagem real do parser (`expected expression`) com span não-detached que resolve para a posição da chamada; idem erro semântico (`unknown variable: zzz`).
- `parse_anchored`: todos os nós e erros herdam o span âncora; anchor detached não sintetiza; modos produzem as raízes `Code`/`Markup`/`Math`.

## 6. Scope-out

- Parâmetro `file:` (não existe no vanilla 0.15.0 como argumento de `eval`; menção histórica removida).
- Spans por-argumento (débito registado em `entities/args.md`, P772s).
- Layout/render.
