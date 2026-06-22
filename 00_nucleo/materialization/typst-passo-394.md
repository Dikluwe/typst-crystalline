# Passo 394 — Materialização: `eval(string)`

**Tipo**: Materialização (L1 — stdlib runtime de re-eval; sem tipo `Value` novo).
**Data**: 2026-06-22.
**Padrão**: diagnóstico-primeiro (sonda 389 cumprida); medir-antes-de-decidir (ADR-0108).
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0107 (paridade linguagem), ADR-0036 (atomização progressiva), ADR-0044 (`Engine` como centro de estado do eval).
**Sonda fonte**: `typst-sonda-ausentes-ordem-passo-389.md` §2D — `eval(string)`, M, runtime de re-parse + re-eval.
**Passo anterior**: P393 (`#show regex`).

> **Nota de numeração.** Um passo só. Não numerar à frente.

---

## 1. Contexto

A sonda 389 identificou `eval(string)` como dívida genuínea acidental (balde D), tamanho M: dado um string, re-parseá-lo como código Typst e re-avaliá-lo no contexto actual, devolvendo o `Value` resultante.

Exemplo vanilla:
```typst
#let x = 1
#eval("x + 2")  // 3
#eval("1 + 2")  // 3
#eval("[*bold*]")  // content com strong
```

A paridade (ADR-0107) é semântica: o string é avaliado como expressão Typst e o valor resultante é devolvido.

---

## 2. Decisão de engenharia

O substrato principal é que as funções nativas cristalinas têm um ABI fechado (`EvalContext`, `Args`, `World`, `FileId`) e **não têm acesso** ao `Scopes` e `Engine` actuais. `eval(string)` precisa desse acesso para re-avaliar o string no contexto corrente.

Opções consideradas:

1. **Estender o ABI de todas as nativas** para passar `&mut Scopes` e `&mut Engine`. Demasiado invasivo para um passo M — tocaria em dezenas de nativas.
2. **Thread-local / global state**. Hacky; viola ADR-0036/0044.
3. **Nova variante de `FuncRepr`** (`NativeWithEngine`) com assinatura alargada, usada apenas por `eval`. Mínima intrusão: aplicação `apply_func` dispacha a variante e passa scopes/engine.

**Escolhida: Opção 3.** Adiciona `FuncRepr::NativeWithEngine` e `Func::native_with_engine`. Não cria tipo `Value` novo; o ABI geral das nativas permanece inalterado.

A re-avaliação:
- Constrói `Source::detached_with_parser(string, parse_code)`.
- Itera as expressões do bloco de código parseado.
- Avalia cada expressão com `eval_expr(..., scopes, ctx, engine_local)`.
- Retorna o `Value` da última expressão.

A engine passada ao eval é **clonada localmente** (`styles`, `show_rules`, `sink`), pelo que `#set`/`#show` dentro do string avaliado não afectam o chamador — paridade com content block (`[]`).

> **Nuance:** `eval(string)` em vanilla aceita opcionalmente `mode: "markup" | "code"`. Este passo implementa apenas o modo código (code block), que é o caso base para as operações aritméticas e construção de content. `mode: "markup"` fica como scope-out documentado.

---

## 3. FASE A — L0 (redação; checkpoint obrigatório)

### A.1 — Prompt L0 `eval.md`

Novo em `00_nucleo/prompts/rules/stdlib/eval.md`:

- **Paridade**: `eval(string)` re-parseia e re-avalia `string` no contexto actual; devolve `Value`.
- **Substrato**: variante `FuncRepr::NativeWithEngine` para dar acesso a `Scopes`/`Engine`.
- **Sem tipo novo**: não adiciona variant a `Value`.
- **Parâmetros**: `source` (obrigatório, `Str`). Erro se não for string.
- **Implementação**:
  1. `Source::detached(source)`.
  2. `parse(source.root())`.
  3. `eval_markup(source.root(), scopes, ctx, engine)`.
  4. Retorna `Value`.
- **Teste**: `eval("1 + 2")` → `3`; `#let x = 1; eval("x + 2")` → `3`; sintaxe inválida → erro; tipo errado → erro.
- **Scope-out**: `mode: "code"`; `scope:`; `file:`.

### A.2 — CHECKPOINT

Parar. Apresentar `eval.md` ao dono. **Só prosseguir para Fase B quando confirmar que guardou e computou hash.**

---

## 4. FASE B — Código (após confirmação humana)

1. **Adicionar `NativeFuncWithEngine`** em `entities/func.rs`:
   ```rust
   pub struct NativeFuncWithEngine {
       pub name: &'static str,
       pub call: fn(
           &mut EvalContext,
           &Args,
           &dyn World,
           FileId,
           &mut Scopes<'_>,
           &mut Engine<'_>,
       ) -> SourceResult<Value>,
   }
   ```
2. **Adicionar `FuncRepr::NativeWithEngine(NativeFuncWithEngine)`** e `Func::native_with_engine`.
3. **Actualizar `apply_func`** em `rules/eval/closures.rs` para despachar a nova variante, passando `scopes` e `engine`.
   - Para obter `scopes` em `apply_func`, é preciso que a função receba `&mut Scopes` (actualmente não recebe).
   - `apply_func` é chamada a partir de vários sítios; a assinatura tem de mudar para incluir `scopes`. Isso propaga para os callers.
   - **Alternativa**: em vez de alterar `apply_func`, criar um caminho especial apenas para `eval`. Mas `apply_func` é o despacho natural.
   - **Decisão**: alterar `apply_func` para receber `&mut Scopes<'_>` e propagar a mudança aos callers (`eval_show_rule`, `apply_regex_rules`, closure application, etc.).
4. **Implementar `native_eval`** em novo ficheiro `01_core/src/rules/stdlib/eval.rs`:
   - Recebe `Scopes` e `Engine` via `NativeWithEngine`.
   - Parse do string com `Source::detached_with_parser(source, parse_code)`.
   - Erros de sintaxe propagam como erro semântico.
   - Avalia cada expressão com `eval_expr` numa engine local (clone de `styles`/`show_rules`/`sink`).
   - Retorna o `Value` da última expressão.
5. **Registar `eval`** em `make_stdlib` (`rules/eval/mod.rs`) usando `Func::native_with_engine`.
6. **Testes**:
   - `eval("1 + 2")` → `3`.
   - `#let x = 5; eval("x * 2")` → `10`.
   - `eval("[*bold*]")` → `Value::Content` com strong.
   - `eval("123")` → `123`.
   - String inválido → erro de parse/eval.
   - Tipo errado → erro.
7. **Linhagem**: `@prompt` aponta para `eval.md`; `@prompt-hash` via `--fix-hashes`.
8. **Validação**:
   - `cargo test --workspace` — verde.
   - `crystalline-lint .` — zero violations novas.
   - `git diff --stat` dos `.rs`: apenas ABI mínimo + `native_eval` + testes.

---

## 5. O que NÃO fazer (scope-out)

- **Não** implementar `mode: "markup"` — code block default é suficiente para este passo.
- **Não** implementar `scope:` ou `file:` — fora de escopo.
- **Não** criar tipo `Value` novo.
- **Não** alterar o ABI geral das nativas — usar variante dedicada.
- **Não** tocar em layout/render.
- **Não** abrir reservas.

---

## 6. Critérios de aceitação

1. `eval("1 + 2")` devolve `Value::Int(3)`.
2. `eval("x + 2")` vê a variável `x` do scope actual.
3. Zero tipo `Value` novo.
4. Testes verdes; lint zero; hashes propagados.
5. Inventário 148: `eval(string)` transita `ausente` → `implementado` (modo código; documentar scope-out de `mode: "markup"`).
6. L0 salvo e hashado antes do código.

---

## 7. O que pode sair errado

- **`apply_func` propagar `Scopes` é intrusivo.** Mitigação: mudança mecânica de assinatura; todos os callers são no módulo `eval`.
- **Ciclo de vida do `Source` detacheado.** Mitigação: `Source::detached` já é usado em testes; spans ficam detacheados, o que é aceitável para eval runtime.
- **`eval` avaliar em scope errado.** Mitigação: passar o `scopes` actual (não uma cópia vazia) e o mesmo `engine`.
- **Recursão / stack overflow.** Mitigação: o `Route` do engine já tem `check_call_depth`; `eval` aninhado respeita o limite.

---

## 8. Referências

- `typst-sonda-ausentes-ordem-passo-389.md` §2D — confirmação de `eval(string)` como M.
- `entities/func.rs` — `FuncRepr` e `NativeFunc`.
- `rules/eval/closures.rs` — `apply_func`.
- `rules/eval/mod.rs` — `eval_markup`, `make_stdlib`.
- ADR-0036 / ADR-0044 — estado do eval.
