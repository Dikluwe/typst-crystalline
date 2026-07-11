# Paridade Produção — P708 — Binding de parâmetros keyword-only não consome posicionais

**Data:** 2026-07-11
**Passo:** `00_nucleo/materialization/typst-passo-708.md`
**Hash do commit (implementação):** a preencher no commit seguinte.
**HEAD base:** `3065cf568` (fim de P707, detached HEAD).
**ADR-0114 EM VIGOR** — sonda antes da spec, cuidado redobrado (bug de mecanismo central).
**Estado:** ÂMBITO DE P708 FECHADO — bug corrigido, varredura ampla do corpus existente sem regressões. `cetz` avança **muito mais fundo** (tempo de compilação salta de ~30s para ~51.8s), novo bloqueio isolado: módulo `std` (acesso à stdlib não-sombreada) inexistente.

---

## 1. Sonda (ADR-0114) — confirmado antes de tocar código

### 1.1 Distinção positional/keyword-only já existe no parser

`typst-syntax/src/ast.rs:2078-2085` (vanilla): `enum Param { Pos(Pattern),
Named(Named), Spread(Spread) }`. `entities/ast/expr.rs` do cristalino
**já espelha isto exactamente** — o parser nunca foi o problema.

### 1.2 Onde a distinção se perdia

`eval_closure_expr` (`rules/eval/closures.rs`) constrói `ClosureParam
{name, default}` a partir de `Param::Pos` (`default: None`) e `Param::Named`
(`default: Some(v)`) — **dois únicos pontos de construção**, confirmado por
leitura exaustiva. `default.is_some()` já era, portanto, um discriminante
100% fiável entre "posicional" e "keyword-only" — só nunca tinha sido
**consultado** pelo loop de binding em `apply_closure`.

### 1.3 Casos medidos contra o vanilla (release build)

```
let f(a, b, close: false) = (a, b, close)
f(1, 2)               → (1, 2, false)
f(1, 2, close: true)  → (1, 2, true)
f(1, 2, 3)            → Err "unexpected argument"

let f(..args, close: false) = args.pos().len()
f(1, 2, 3)             → 3
```

`f(close: false) = close; f(true)` → `Err "unexpected argument"` (confirma
que parâmetros `nome: default` são **keyword-only**, nunca preenchíveis
por posição).

### 1.4 Achado lateral, mesma categoria mas código diferente

`f(a, b) = (a, b); f(1, 2, 3)` (sem NENHUM parâmetro keyword-only) — já
aceitava silenciosamente no cristalino antes de P708 (descartando o `3`);
vanilla erra `"unexpected argument"`. Confirma que a falta de validação de
argumentos extra é mais geral do que só "parâmetro keyword-only rouba
posicional" — mas a correcção é a mesma (verificar sobra de
`args.items` após o binding).

**Scope-out confirmado, não corrigido**: argumento nomeado que não
corresponde a nenhum parâmetro (`f(1, z: 2)` com `f(a)`) — vanilla erra
`"unexpected argument: z"`, cristalino continua a aceitar silenciosamente.
Código de validação diferente (named, não positional); registado para
passo futuro.

---

## 2. Implementação

### `apply_closure` (`01_core/src/rules/eval/closures.rs`)

Loop de binding: só tenta `args.items.get(pos_idx)` quando
`param.default.is_none()` (posicional). Para `param.default.is_some()`
(keyword-only), só `args.named` é consultado — nunca avança `pos_idx`.

Após o loop: se **não** há sink e sobram itens em `args.items[pos_idx..]`,
devolve `Err("unexpected argument")` (mensagem verbatim do vanilla) em vez
de descartar silenciosamente. Com sink, comportamento inalterado (P504).

### L0

- `entities/func.md` — nova secção documentando a invariante
  `default.is_some() ⟺ Param::Named (keyword-only)`, já garantida na
  construção, sem precisar de campo novo.
- `rules/eval.md` §P708 — algoritmo de binding corrigido, casos medidos,
  scope-out do achado lateral (named extra).

### Testes (6 novos, `eval/tests.rs`)

Keyword-only omitido (usa default), explícito (sobrepõe default),
argumento extra sem sink (erro), sink absorve todos os posicionais apesar
do keyword-only, closure só-positional sem regressão, closure só-sink sem
regressão.

---

## 3. Varredura ampla (ADR-0114) — sem regressões

Dado o alcance potencial do bug (qualquer closure do utilizador com
parâmetros `nome: default`), corri o **corpus completo de testes já
existente** antes de adicionar os testes novos, para confirmar que nenhum
caso já testado ao longo desta longa cadeia P1-P708 dependia
silenciosamente do comportamento errado:

- **Antes da correcção** (só a mudança de código, sem testes novos):
  `cargo test --workspace` → 3784 passed, 0 failed — **idêntico ao
  baseline de P707**. Nenhum teste existente dependia do bug.
- **Depois de adicionar os 6 testes novos**: 3790 passed, 0 failed.

---

## 4. Validação — âmbito de P708 confirmado

Reexecutados os 3 documentos `.typ` da sonda (release build): todos
**idênticos ao vanilla** — `(1,2,false)`/`(1,2,true)`, `"unexpected
argument"` para o extra sem sink, `3` para o sink com keyword-only.

- `cargo test --workspace` → **3790 passed**, 0 failed; `typst-infra`
  inalterado (626/5).
- `crystalline-lint .` → 0 violations (11 ficheiros com hash realinhado,
  partilham a L0 `rules/eval.md`; `func.rs` por editar `entities/func.md`).

---

## 5. Repetição da reprodução de P700-707 — salto real, novo bloqueio

```
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
```

**Tempo de compilação: ~51.8s** — salto real face aos ~30s de P706/P707
(quase 2×). Sinal forte de que o bug de binding bloqueava uma porção
substancial da lógica de `cetz` (funções com assinaturas
`(...positional..., named: default)`, um padrão extremamente comum em
Typst).

**Novo bloqueio**: `error: unknown variable: std`. `cetz` usa `std.length`,
`std.measure`, `std.color`, `std.stroke`, etc. (`canvas.typ`, `util.typ`,
`styles.typ`) para aceder à stdlib **não-sombreada** quando o próprio
`cetz` define algo com o mesmo nome. Confirmado standalone:
```
#let length = 5
#(std.length)
```
→ vanilla: exit 0, `length` (o tipo builtin, não o `5` sombreado).
Cristalino: `error: unknown variable: std` — o módulo `std` não existe.

### Próximo passo sugerido (P709, não iniciado)

1. Confirmar no vanilla a natureza exacta de `std` (módulo com todo o
   scope global padrão, construído uma vez, imutável a `#let`
   subsequentes).
2. Decidir onde registar `std` no scope global (`make_stdlib`,
   `rules/eval/mod.rs`) — provavelmente um snapshot do próprio scope
   base antes de quaisquer bindings do utilizador.
3. Reexecutar a reprodução deste §5 como critério de fecho.

---

## 6. Estado da cadeia P678–708

P708 é qualitativamente diferente dos passos anteriores: não foi um gap
de função nativa isolada, foi um bug no mecanismo central de chamada de
closures do utilizador, com alcance potencialmente amplo — confirmado sem
regressões no corpus existente, mas o salto de ~30s para ~51.8s em `cetz`
sugere que o bug afetava silenciosamente uma quantidade substancial de
código real. Pausada aqui, com P709 sugerido (`std` module).

## 7. Critério de fecho do passo

- [x] Sonda completa, distinção positional/keyword-only confirmada ao
      nível do AST (vanilla e cristalino, ambos já corretos).
- [x] Binding corrigido em duas fases, testado contra os casos de P707 e
      novos casos de borda.
- [x] Argumento extra produz erro, não aceitação silenciosa.
- [x] Varredura ampla do corpus de testes existente — confirmado sem
      casos afectados silenciosamente (3784 passed antes e depois da
      correcção, sem testes novos).
- [x] Sem regressão em `cargo test --workspace` (3790 passed com os
      testes novos).
- [x] `crystalline-lint .` limpo.
- [x] `cetz` re-testado — **salto substancial de profundidade** (~30s →
      ~51.8s), próximo bloqueio identificado (`std` module).
- [x] Relatório com resultado exacto (este ficheiro).
