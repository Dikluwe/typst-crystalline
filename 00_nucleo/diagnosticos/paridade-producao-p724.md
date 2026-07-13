# Paridade Produção — P724 — Patterns de desestruturação em parâmetros de closure

**Data:** 2026-07-13
**Passo:** `00_nucleo/materialization/typst-passo-724.md`
**Hash do commit (implementação):** a preencher após o commit.
**HEAD base:** `630a32f73` (fim de P723, branch `Tekt`).
**Estado:** FECHADO — `ClosureParam` passa a guardar o pattern completo
(`Option<SyntaxNode>`); `apply_closure` liga patterns não-`Ident` via
`destructure_let`, mirror do vanilla. 7 testes novos verdes, workspace
sem regressão. `cetz` re-testado — avançou um bloqueio; o bloqueio actual
(4º da cadeia) ficou isolado com operando medido e `file:line` para P725.

---

## 1. Sonda (ADR-0114 — sonda antes da spec, cumprida)

### 1.1 Comportamento exacto no vanilla (medido)

Documento da sonda do passo (`/tmp/p724-destr-param.typ`), binário
`lab/typst-original/target/release/typst`, exit 0:

```
#let pairs = ((1, "a"), (2, "b"), (3, "c"))
#pairs.map(((n, s)) => str(n) + s)     → ("1a", "2b", "3c")
#let f = ((a, b), c) => a + b + c
#f((1, 2), 3)                           → 6
#let g(x, (a, b)) = x + a + b
#g(10, (1, 2))                          → 13
```

Confirmado: funciona em closures anónimas (`=>`), em definições
nomeadas (`#let g(...) = ...`), e misturado com parâmetros normais na
mesma assinatura. Sondas adicionais (mesmo binário):

```
#let f = (_, y) => y; f(1, 2)                       → 2   (placeholder consome)
#let f = ((a, ..rest)) => rest.len(); f((1, 2, 3))  → 2   (spread no pattern)
```

### 1.2 Mecanismo exacto do vanilla (fonte, não assumido)

`lab/typst-original/crates/typst-eval/src/call.rs:655-665` — no bind de
parâmetros de `eval_closure`:

```rust
ast::Param::Pos(pattern) => match pattern {
    ast::Pattern::Normal(ast::Expr::Ident(ident)) => {
        vm.define(ident, args.expect::<Value>(&ident)?)
    }
    pattern => {
        crate::destructure(&mut vm, pattern,
            args.expect::<Value>("pattern parameter")?)?;
    }
},
```

`Param::Pos` não-`Ident` liga via `destructure` — a mesma entrada
genérica de `let`/atribuição/`for` (`binding.rs:45-58`). O vanilla
guarda a closure inteira como `SyntaxNode` owned
(`ClosureNode::Closure(self.to_untyped().clone())`, call.rs:583) e
reparseia os params em cada chamada — o cristalino pré-extrai os params
em `ClosureParam`, daí a necessidade de guardar o pattern.

### 1.3 Estado do cristalino antes

Mesmo documento, cristalino em `630a32f73` (binário de P723):

```
/tmp/p724-destr-param.typ:<detached>: error: unexpected argument   (exit 1)
```

Mecanismo (isolado em P723 com build instrumentado): `eval_closure_expr`
(`closures.rs:345`, braço `_ => None, // Placeholder, Destructuring —
adiado`) **descartava silenciosamente** o parâmetro → closure criada com
params a menos → `apply_closure` rejeita o argumento extra (P708).
Consumidor real: `path-util.typ:453` de `cetz`
(`segments.enumerate().filter(((i, segment)) => ...)`).

## 2. Implementação (L0: `prompts/entities/func.md`)

- **`01_core/src/entities/func.rs`** — `ClosureParam` ganha
  `pub pattern: Option<SyntaxNode>` (clone O(1) via Arc interno).
  `Some` só para `Param::Pos` não-`Ident`; nesse caso `name` é `""`
  (nunca consultado) e `default` é `None`. A invariante P708
  (`default` como discriminante posicional vs keyword-only) fica intacta.
- **`01_core/src/rules/eval/closures.rs` `eval_closure_expr`** — o braço
  `_ => None` é substituído por `Param::Pos(pattern) =>` que guarda
  `pattern.to_untyped().clone()` — cobre destructuring, parenthesized e
  placeholder (este último passa a consumir o posicional, paridade com o
  vanilla — mudança de comportamento medida e registada no L0).
- **`apply_closure`** — no bind de cada parâmetro: `pattern: Some(node)`
  → `Pattern::from_untyped(node)` + `destructure_let(pattern, val,
  &mut call_scopes, ctx, engine)?` (mesma entrada do `#let` e do `for`,
  P723); `None` → `define` directo como antes. Mesmo padrão de reparse
  já usado para o `body` (`Expr::from_untyped`).

### 2.1 Divergência residual (pré-existente, fora do scope)

Posicional em falta liga `Value::None` (o vanilla erra `missing
argument`); sobre um pattern, `destructure_let(None)` erra `cannot
destructure none` em vez de `missing argument: pattern parameter`.
Registada no L0; não introduzida por este passo.

### 2.2 Testes (7 novos, fail-first confirmado: 7/7 FAILED antes)

Em `eval/tests.rs` (junto aos P708): closure anónima com destructuring,
mistura com posicional, definição nomeada, placeholder, spread no
pattern, o padrão exacto do cetz (`enumerate().map(((i, seg)) => ...)`),
e tipo errado (`cannot destructure`). Regressão P708 coberta pelos
testes existentes (keyword-only, sink, extra posicional).

## 3. Validação

- `cargo test -p typst-core --lib p724` → **7 passed, 0 failed**.
- `cargo test --workspace` → **0 failed** (core 3955 = 3948 de P723 + 7
  novos; resto do workspace verde). Atenção redobrada pedida pelo passo:
  os testes P504/P702/P708/P716 de chamada de closures (mecanismo
  tocado) continuam verdes; nenhum teste existente foi alterado.
- Documento da sonda: cristalino agora produz
  `("1a", "2b", "3c") / 6 / 13` — **conteúdo idêntico ao vanilla**
  (diff de `pdftotext`: só uma linha em branco de espaçamento entre
  parágrafos no vanilla, divergência de render pré-existente, fora do
  scope).
- `crystalline-lint --fix-hashes .` + `crystalline-lint .` → **0
  violations** (`func.rs` → hash `da2e4b21`; `closures.rs`/`tests.rs`
  ligados a `eval.md`, inalterado neste passo).

## 4. `cetz` re-testado — campos fixos (bloqueio 4 da cadeia)

```
$ time ./target/release/typst /tmp/p724-cetz.typ /tmp/p724-cetz.pdf
/tmp/p724-cetz.typ:<detached>: error: cannot apply Mul to float and length
real    0m54,505s
Exit code: 1
```

O destructuring em parâmetros de closure (bloqueio 3) está resolvido —
o documento avançou para o bloqueio seguinte. Identificado com build
instrumentado temporário na fronteira genérica de `operators.rs`
(patch revertido de seguida):

- **Operando medido:** `2.0 * 28.35pt` (= 2.0 × 1cm).
- **Consumidor (file:line):** `canvas.typ:146-147` (e `182-186`) —
  escala de coordenadas por `length` (`(x - offset) * length`), com
  `length` default `1cm` (`canvas.typ:25`).
- **Vanilla (medido):** as 4 combinações funcionam — `2.0 * 1pt → 2pt`,
  `1pt * 2.0 → 2pt`, `2 * 1pt → 2pt`, `1pt * 2 → 2pt`.
- **Cristalino:** zero braços `Mul` com `Length` em `operators.rs`
  (só `Length / Int|Float`, P713).
- Registado em `achados-adiados-cetz.md` para P725 (correção pequena:
  4 braços + testes, L0 `rules/eval/ops.md`).

## 5. ADRs

Grep a `00_nucleo/adr` pelos termos centrais (`ClosureParam`,
desestruturação/destructuring): nenhuma decisão vigente rege o modelo de
`ClosureParam` ou destructuring em parâmetros de closure (ADR-0007
menciona destructuring só no contexto de `rustc-hash`). **ADR-0114**
(sonda antes da spec, invocada pelo passo) cumprida — §1. ADR-0108:
toda a classificação foi precedida de medição (§1.1-1.3, §4). Nada a
atualizar.

## 6. Critério de fecho do passo

- [x] Sonda completa, comportamento confirmado (anónimas, nomeadas,
  mistura, placeholder, spread; mecanismo vanilla com file:line).
- [x] Implementado e testado, incluindo mistura de parâmetros normais e
  de desestruturação.
- [x] Sem regressão em `cargo test --workspace` (mecanismo P708 intacto,
  testes de chamada verdes sem alteração).
- [x] `crystalline-lint .` limpo.
- [x] `cetz` re-testado — campos fixos registados (§4), bloqueio actual
  isolado para P725.
- [x] Grep às ADRs (§5).
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p724.md`
  (hash do commit preenchido no commit seguinte).
