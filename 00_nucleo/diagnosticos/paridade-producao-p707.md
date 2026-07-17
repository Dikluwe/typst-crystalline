# Paridade Produção — P707 — `Arguments` com métodos (`.pos()`, `.named()`)

**Data:** 2026-07-11
**Passo:** `00_nucleo/materialization/typst-passo-707.md`
**Hash do commit (implementação):** `bc21a2a7d`.
**HEAD base:** `e5181603b` (fim de P706, detached HEAD).
**Estado:** ÂMBITO DE P707 FECHADO — `.pos()`/`.named()` implementados e corretos, confirmado em isolamento. `cetz` continua bloqueado, mas por um **bug estrutural pré-existente e muito mais significativo**, descoberto ao correr a reprodução completa: parâmetros nomeados-com-default (`nome: default`) consomem incorretamente argumentos posicionais quando não são passados explicitamente — **não é um gap de API, é um bug de binding de argumentos**, afetando potencialmente qualquer closure do cristalino, não só `cetz`.

---

## 1. Sonda — assinatura completa de `Arguments`, não assumida

`foundations/args.rs:320-449` (`#[scope] impl Args`): `len()`, `at(key,
default:)`, `pos()`, `named()`, `filter(test)`, `map(mapper)`.
**`.pairs()` não existe em `Args`** — existe em `Dict`
(`foundations/dict.rs:270`); o uso em `coordinate.typ:90` (`c.bary.pairs()`)
é sobre um dict, não confirma nada sobre `Args`.

Medido com documento real (`args.pos()`, `.named()`, `.len()`, `.at(0)`,
`.at("x")`): `((1, 2), (x: 3, y: 4), 4, 1, 3)` — todos funcionam no
vanilla.

**Âmbito medido em `cetz`**: `grep` exaustivo confirma que só `.pos()` e
`.named()` são chamados sobre um `Args` real (sinks `..x` variádicos, em
`drawable.typ`, `util.typ`, `coordinate.typ`). Todos os `.at(`/`.len(`
encontrados operam sobre `Dict`/`Array` normais, já suportados.

**Estado cristalino antes do fix**: `args.pos()` → "campo desconhecido";
`args.named()` → "não é possível chamar dictionary" (o campo `.named`,
P504, resolvia para o `Dict`, e a chamada `()` sobre esse `Dict` falhava
— erro diferente e revelador, não assumido).

---

## 2. Implementação (âmbito de P707)

Novo bloco em `eval_func_call` (`01_core/src/engine/eval/closures.rs`),
mesmo padrão de P417/P423/P504/P466/P506/P702: se o callee é `FieldAccess`
com campo `"pos"` ou `"named"` e o alvo avalia para `Value::Args`, devolve
directamente `Value::Array(a.items)` / `Value::Dict(a.named)` — sem passar
pelo field access genérico de P504 (que continua intacto para
`.positional`/`.named`, sem parênteses).

**Scope-out explícito** (sem consumidor medido em `cetz`): `.len()`,
`.at(key, default:)`, `.filter(test)`, `.map(mapper)` em `Args`.

### Testes (3 novos, `eval/tests.rs`)

`.pos()`, `.named()`, e um teste combinado confirmando que `.positional`/
`.named` (campo, P504) **não regridem** ao lado de `.pos()`/`.named()`
(método, P707).

---

## 3. Validação do âmbito de P707 — confirmado

```
let f(..args) = args.pos()      ; f(1, 2, x: 3) → (1, 2)
let f(..args) = args.named()    ; f(1, 2, x: 3) → (x: 3)
```
Ambos **idênticos ao vanilla**. `.positional`/`.named` (campo) sem
regressão.

- `cargo test --workspace` → **3784 passed**, 0 failed (3781 de P706 + 3
  novos de P707); `typst-infra` inalterado (626/5).
- `crystalline-lint .` → 0 violations (8 ficheiros com hash realinhado —
  partilham a L0 `rules/eval.md`, só `closures.rs`/`tests.rs` tiveram
  lógica nova).

---

## 4. Repetição da reprodução de P700-706 — mesmo tempo, bug novo e maior

```
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
```

Tempo de compilação: **~30.4s** — igual à ordem de grandeza de P706
(~30.4s vs ~30.7s), sem regressão nem salto adicional visível neste
número (o bloqueio seguinte está muito próximo do de P706 na árvore de
avaliação).

### Novo bloqueio — não é gap de API, é bug de binding

`error: Line must have a minimum de two points` (mensagem própria de
`cetz`, `draw/shapes.typ:582`, `assert(pts.len() >= 2, ...)`), apesar de
`line((0,0), (2,1))` passar claramente 2 pontos.

**Isolado**: `cetz`'s `line` é `#let line(..pts-style, close: false, name:
none) = { let pts = pts-style.pos(); ... }` — `pts-style.pos()` devolvia
**menos items do que os passados**. Reproduzido minimamente:

```
#let f(..args, close: false) = args.pos().len()
#f(1, 2, 3)
```
→ vanilla: `3`. Cristalino: `2` — **1 item perdido**.

Mais grave, sem sink nenhum:
```
#let f(a, b, close: false) = (a, b, close)
#f(1, 2, 3)
```
→ vanilla: `error: unexpected argument` (só há 2 posições, `close` é
keyword-only, `3` não tem onde ir). Cristalino: `(1, 2, 3)` — **aceita
silenciosamente** e faz `close = 3` (um `Int`, não o `Bool` default),
sem erro nenhum.

### Causa raiz (identificada, não corrigida — fora do âmbito de P707)

`apply_closure` (`closures.rs:197-208`):
```rust
let mut pos_idx = 0;
for param in closure.params.iter() {
    let val = if let Some(v) = args.named.get(param.name.as_str()) {
        v.clone()
    } else if let Some(v) = args.items.get(pos_idx) {
        pos_idx += 1;              // <-- consome posicional mesmo para
        v.clone()                  //     parâmetros que só deviam
    } else {                       //     aceitar named!
        param.default.clone().unwrap_or(Value::None)
    };
    call_scopes.define(param.name.as_str(), val);
}
```
`ClosureParam { name, default }` (`entities/func.rs`) **não distingue**
parâmetro posicional de parâmetro nomeado-com-default (`nome: default` na
assinatura, keyword-only no vanilla — confirmado: `#let f(close: false) =
close; f(true)` → `error: unexpected argument` no vanilla, não pode ser
preenchido posicionalmente). O binding trata os dois tipos de forma
idêntica: se não vier por nome, tenta a próxima posição — isto está
**errado** para parâmetros keyword-only, que nunca deviam consumir
posicionais.

**Alcance do bug**: não é específico de `cetz` ou de sinks — afeta
**qualquer closure do cristalino** com pelo menos um parâmetro
`nome: default` que não seja passado por nome numa chamada com argumentos
posicionais extra. É provavelmente o achado de maior impacto potencial de
toda a cadeia P678-707 (mais amplo do que qualquer gap de função nativa
isolada), porque toca o mecanismo central de chamada de funções definidas
pelo utilizador.

### Próximo passo sugerido (P708, não iniciado)

1. Confirmar no vanilla a distinção exacta positional vs keyword-only ao
   nível do parser/AST (`entities/ast/expr.rs`, nó `Param`) — já existe a
   distinção sintáctica (`nome` vs `nome: valor` na assinatura)?
2. Adicionar a `ClosureParam` um marcador (`kind: Positional |
   NamedWithDefault`, ou equivalente) na construção da closure.
3. Reescrever o loop de binding em `apply_closure` em duas fases:
   parâmetros posicionais consomem `args.items` em ordem; parâmetros
   nomeados-com-default só olham para `args.named`, nunca `args.items`.
4. Confirmar que "argumento extra sem posição" produz erro
   (`"unexpected argument"`), não aceitação silenciosa.
5. Reexecutar as duas reproduções mínimas deste §4 como critério de
   fecho, depois a reprodução completa de `cetz`.

---

## 5. Estado da cadeia P678–707

`.pos()`/`.named()` (P707) fechado e correto — mas o próximo passo da
cadeia (`Line must have a minimum of two points`) não é mais um gap de
API isolado como P699-P706: é um bug de mecanismo central (binding de
argumentos de closures), com alcance potencial muito maior do que
`cetz`. Recomenda-se tratá-lo com prioridade alta e a mesma disciplina de
sempre (medir, L0, testar) — mas reconhecendo que é qualitativamente
diferente dos passos anteriores desta cadeia.

## 6. Critério de fecho do passo

- [x] Sonda completa, assinatura completa de `Arguments` confirmada
      (incluindo a correcção de que `.pairs()` não existe em `Args`).
- [x] Implementado e testado (`.pos()`, `.named()`).
- [x] Sem regressão em `cargo test --workspace` (3784 passed).
- [x] `crystalline-lint .` limpo.
- [x] `cetz` re-testado — **próximo bloqueio identificado e isolado, e
      classificado como bug estrutural, não gap de API** (binding de
      parâmetros keyword-only em closures).
- [x] Relatório com resultado exacto (este ficheiro).
