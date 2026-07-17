# Prompt L0 — `stdlib/eval` — runtime de re-avaliação
Hash do Código: 3c6de891

**Camada**: L1
**Ficheiro alvo**: `01_core/src/engine/stdlib/eval.rs`
**Origem**: Passo 394 (`typst-passo-394.md`) — dívida genuína acidental (balde D), M.
**ADRs**: ADR-0033 (paridade vanilla), ADR-0107 (paridade linguagem), ADR-0036/0044 (estado do eval).

---

## 1. Contexto

O vanilla expõe `eval(source)` — re-parseia e re-avalia uma string como código Typst no contexto actual. Exemplos:

```typst
#let x = 1
#eval("x + 2")      // 3
#eval("[*bold*]")   // content com strong
```

## 2. Arquitetura

- **Sem tipo `Value` novo**.
- **ABI alargado para nativas**: adiciona `FuncRepr::NativeWithEngine` para funções que precisam de `Scopes` e `Engine` (apenas `eval` neste passo).
- **Reutiliza parser e eval existentes**: `Source::detached_with_parser(source, parse_code)` → itera as expressões do bloco de código → `eval_expr(..., scopes, ctx, engine_local)`.
- **Scope actual**: a re-avaliação vê as variáveis do scope onde `eval` é chamado.
- **Engine local**: `#set`/`#show` dentro do string avaliado são confinados a uma engine local, não afectando o chamador (paridade com content block).

## 3. Função nativa

`native_eval(ctx, args, world, current_file, scopes, engine)`:

- `source`: único argumento posicional obrigatório, `Str`.
- Rejeita argumentos nomeados.
- Tipo errado → erro.
- Erros de sintaxe no string avaliado propagam como erro semântico.
- Constrói `Source::detached_with_parser(source, parse_code)`, avalia cada expressão e devolve o `Value` da última.

## 4. Paridade vanilla

A paridade é semântica (ADR-0107): `eval(source)` avalia o string como código Typst e devolve o valor. Cristalino adopta **modo código por default**; modo `markup`, `scope:` e `file:` ficam fora deste passo.

## 5. Testes

- `eval("1 + 2")` → `3`.
- `#let x = 5; eval("x * 2")` → `10`.
- `eval("[*bold*]")` → `Value::Content` com `Content::Strong`.
- `eval("123")` → `123`.
- String com sintaxe inválida → erro.
- `eval(123)` → erro de tipo.
- Argumentos nomeados (ex: `mode:`) → erro.

## 6. Scope-out

- `mode: "markup"` (vanilla default; cristalino default é code).
- Parâmetros `scope:` / `file:`.
- Layout/render.
