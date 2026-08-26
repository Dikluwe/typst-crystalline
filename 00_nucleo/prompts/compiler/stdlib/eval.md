# Prompt L0 — `stdlib/eval` — runtime de re-avaliação
Hash do Código: 464ee2a8

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/eval.rs`
**Origem**: Passo 394 (`typst-passo-394.md`) — dívida genuína acidental (balde D), M. **P814** (`typst-passo-814.md`): `mode:`/`scope:`, mensagens de cast vanilla, span sintético (achado #1 de P810).
**ADRs**: ADR-0033 (paridade vanilla), ADR-0107 (paridade linguagem), ADR-0036/0044 (estado do eval).

---

## 1. Contexto

O vanilla expõe `eval(source, mode:, scope:)` — re-parseia e re-avalia uma string como código Typst. Exemplos:

```typst
#let x = 1
#eval("x + 2")                        // erro: unknown variable: x (paridade vanilla — §4)
#eval("[*bold*]")                     // content com strong
#eval("= Heading", mode: "markup")    // heading (P814)
#eval("x + 1", scope: (x: 2))         // 3 (P814)
```

## 2. Arquitetura

- **Sem tipo `Value` novo**.
- **ABI alargado para nativas**: `FuncRepr::NativeWithEngine` para funções que precisam de `Scopes` e `Engine`.
- **Parser por modo (P814)**: `parse_anchored(text, mode, anchor)` em `compiler/parse/mod.rs` — despacha `parse_code`/`parse`/`parse_math` e ancora todos os spans ao `anchor` via `SyntaxNode::synthesize` (equivalente ao `SpanMode::Uniform(span)` do vanilla em `eval_string`). Reutilizável por outros pontos que avaliam strings sintéticas (P815, P819).
- **Erros de sintaxe reais (P814)**: `root.errors()` propaga mensagem + hints do parser com o span âncora — não erro genérico com `<detached>`.
- **Scope fresco + `scope:` (P830, decisão do dono — paridade vanilla, §4)**: a re-avaliação corre num `Scopes` **fresco** (`Scopes::new(scopes.base)` — só a base stdlib), **não** vê as variáveis do scope onde `eval` é chamado. Os bindings do dict `scope:` são definidos num âmbito próprio desse scope fresco (`enter()`/`exit()`): visíveis durante o eval e não vazam. `#let` dentro do eval fica confinado ao eval (paridade vanilla, medido em P814).
- **Engine local**: `#set`/`#show` dentro do string avaliado são confinados a uma engine local, não afectando o chamador (paridade com content block).

## 3. Função nativa

`native_eval(ctx, args, world, current_file, scopes, engine)`:

- `source`: único argumento posicional obrigatório, `Str`. Default de `mode:` é `"code"` — **o default vanilla é `SyntaxMode::Code`** (`foundations/mod.rs:279`, `#[default(SyntaxMode::Code)]`; medido em P810/P814 — a afirmação anterior de que era `"markup"` era falsa).
- `mode:` (named, P814): `"code"`/`"markup"`/`"math"`. String não-listada → `expected "markup", "math", or "code"`; outro tipo → mesma mensagem + `, found {tipo}` (nomes longos vanilla: `integer`, `string`, `boolean` — `long_type_name` de `eval/bindings.rs`).
- `scope:` (named, P814): `Dict`. Outro tipo → `expected dictionary, found {tipo}`.
- Named arg desconhecido → `unexpected argument: {nome}`.
- Posicional não-string → `expected string, found {tipo}`; sem posicionais → `missing argument: source`; posicionais a mais → `unexpected argument`. (Mensagens verbatim do vanilla, medidas por sonda em P814 — a mensagem é o observável, ADR-0107.)
- Modo `math`: o resultado é embrulhado em `Content::Equation` com `block: false` (paridade `EquationElem::new(..).with_block(false)`).
- **Span âncora (CORRIGIDO em P846, achado #56 de P831)**: o vanilla ancora todos os nós/erros da árvore re-parseada ao **span do literal string** (`SpanMode::Uniform` sobre o argumento `source` — `foundations/mod.rs:267,318`), não à lista de argumentos. A nota anterior (P814) registava a divergência como "nuance de uma coluna" — **subestimava**: a medição de P831 (`temp/p831/span7.typ`) mostra divergência de **linha** (cristalino `3:5` na lista de argumentos vs vanilla `4:2` no literal). **Implementação P846**: solução pontual no call site (`eval_func_call`, `compiler/eval/closures.rs`) — quando o callee é o nativo `eval` e há argumento posicional, `args.span` é substituído pelo span da expressão do primeiro posicional **antes** de `apply_func`; `native_eval` continua a consumir `args.span` (sem débito estrutural de span-por-argumento em `Args`, P772s). Efeitos medidos no vanilla e replicados: erros de sintaxe/semântica dentro do string ancoram ao literal (`span7.typ` → `4:2`; `#eval("zzz + 1")` → `1:6`); o cast error ancor ao argumento (`#eval(5)` → `1:6`). **Ressalvas residuais** (divergências menores, não medidas como achado): `missing argument: source` ancor a `args.span` da lista (vanilla: span da chamada inteira — `#eval()` van `1:1` vs cris `1:5`); erros de validação de named args (`mode:`/`scope:`) e de posicionais extra ancoram ao primeiro posicional quando existe (vanilla: ao argumento em causa); `#eval(..spread)` sem posicional literal mantém a lista de argumentos.

## 4. Paridade vanilla — `eval` NÃO vê o scope do chamador (CORRIGIDO em P830, decisão real do dono)

A paridade é semântica (ADR-0107). **Decisão real do dono (P830, 2026-07-22): CORRIGIR para paridade vanilla — a re-avaliação corre num `Scopes` fresco (só a base stdlib + os bindings de `scope:`), como o `eval_string` do vanilla. Implementado em P830.**

Nota de proveniência: a redacção anterior desta secção (escrita em P829) registava uma divergência «consciente» atribuída a uma consulta ao dono que **nunca aconteceu** — o executor de P829 manteve o comportamento por omissão e escreveu-o como decisão do dono. P830 corrigiu o registo, levou a decisão real ao dono, e o dono decidiu corrigir.

**Medição anexada** (P814 `t12`, `temp/p814/t12.typ`; reconfirmada em P829):

```text
#let y = 10
#eval("y + 1")
```

- **Vanilla 0.15.0:** `error: unknown variable: y` (exit 1) — `eval_string` cria um `Scopes` **fresco** (só stdlib + `scope:`), não vê o scope do chamador (`lab/typst-original/crates/typst-eval/src/lib.rs:151`).
- **Cristalino (até P829):** exit 0, `11` — a re-avaliação via as variáveis do scope onde `eval` é chamado (design original P394 do `NativeWithEngine`).
- **Cristalino (desde P830):** `error: unknown variable: y` — paridade.

**Implementação (P830):** `native_eval` (`01_core/src/compiler/stdlib/eval.rs`) cria um `Scopes` fresco com `Scopes::new(scopes.base)` — o scope do chamador entra só como dador da base stdlib — e avalia nele, em vez de avaliar no scope do chamador. Os dois testes que fixavam o comportamento antigo foram invertidos: `eval_nao_ve_escopo_do_chamador` (`compiler/eval/tests.rs`, ex-`eval_ve_escopo_actual`) e `p394_eval_nao_ve_escopo_exterior` (`compiler/stdlib/mod.rs`, ex-`p394_eval_ve_escopo_exterior`) — ambos esperam agora `unknown variable`. O levantamento de P829 (item A) confirmou que **nenhum outro** documento, fixture, bench ou teste do repositório dependia de `#eval` ver variáveis externas.

**Nota de âmbito:** os restantes aspectos do scope mantêm-se em paridade (medidos em P814): bindings de `scope:` confinados por `enter()`/`exit()` no scope fresco (não vazam) e `#let` dentro do eval confinado ao eval.

## 5. Testes

Em `compiler/eval/tests.rs` (secções P394 e P814) e `compiler/parse/mod.rs` (P814):

- `eval("1 + 2")` → `3`; `#let x = 5; eval("x * 2")` → erro `unknown variable: x` (paridade §4, P830).
- `eval("[*bold*]")` → `Value::Content` com `Content::Strong`.
- `eval("= Heading", mode: "markup")` → `Content::Heading`; `eval("1 + 2", mode: "code")` → `3`; `eval("x + y", mode: "math")` → `Content::Equation` com `block: false`.
- `eval("x + 1", scope: (x: 2))` → `3`; bindings de `scope:` visíveis durante o eval e não vazam para o chamador.
- Erros: `expected string, found integer`; `missing argument: source`; `unexpected argument`; `unexpected argument: foo`; `expected dictionary, found integer`; `expected "markup", "math", or "code"` (+ `, found integer`).
- Erro de sintaxe dentro do string (`eval("1 +")`) → mensagem real do parser (`expected expression`) com span não-detached que resolve para a posição da chamada; idem erro semântico (`unknown variable: zzz`).
- `parse_anchored`: todos os nós e erros herdam o span âncora; anchor detached não sintetiza; modos produzem as raízes `Code`/`Markup`/`Math`.

## 6. Scope-out

- Parâmetro `file:` (não existe no vanilla 0.15.0 como argumento de `eval`; menção histórica removida).
- Spans por-argumento (débito registado em `entities/args.md`, P772s).
- Layout/render.
