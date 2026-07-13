# Paridade Produção — P723 — `assert.eq`/`assert.ne` + spread `..sink` em padrão de `for`

**Data:** 2026-07-13
**Passo:** `00_nucleo/materialization/typst-passo-723.md`
**Hash do commit (implementação):** `510c12a9c`.
**HEAD base:** `aa1d3fa43` (fim de P722, branch `Tekt`).
**Estado:** FECHADO — a premissa do passo (namespace de `curve` ausente)
foi **refutada** pela sonda (existe desde P513); os dois bloqueios reais
encontrados a seguir foram implementados e testados (`assert.eq`/`assert.ne`
com namespace; delegação do `for` ao destructuring genérico com spread).
`cetz` re-testado — avançou dois bloqueios; o bloqueio actual (3º) ficou
isolado com `file:line` para P724.

---

## 1. Sonda

### 1.1 Premissa do passo refutada (ADR-0108: a medição vence o enquadramento)

O passo afirmava: «o `Value::Func` nativo `curve` não tem namespace
registado no cristalino (`bindings.rs:1396`)». Medição directa do código
em `aa1d3fa43`:

- `eval/mod.rs:1173-1185` — o namespace de `curve` **existe desde P513**
  (`move`/`line`/`cubic`/`quad`/`close` via `Func::native_with_namespace`).
- Sonda funcional: `#curve(curve.move((0,0)), curve.line((1,1)),
  curve.cubic((0,0),(1,0),(1,1)), curve.close(mode: "straight"))` compila
  com **exit 0** no cristalino — o namespace funciona.

O erro que o passo via (`campo 'eq' desconhecido...`) foi reproduzido com
build instrumentado temporário na resolução de field access
(`bindings.rs:1394-1398`, braço `Value::Func(f) => match f.namespace()`,
patch revertido de seguida): a mensagem passou a mostrar
`func='assert' campo='eq'` — o `Value::Func` sem namespace era **`assert`,
não `curve`**. O sintoma da lista de controlo de P720 estava certo, a
função identificada estava errada.

### 1.2 Achado lateral (medido, não corrigido neste passo)

A sonda simples de `curve` compila (exit 0) no cristalino mas renderiza
**página em branco**; o vanilla desenha a forma. PNGs medidos:
`/tmp/p723-vanilla.png` (11572 bytes, forma visível) vs
`/tmp/p723-cristalino.png` (10122 bytes, em branco). Bug de **render** de
`curve`, separado do namespace — registado em `achados-adiados-cetz.md`.

### 1.3 Bloqueio real 1 — `assert.eq` (implementado neste passo)

Vanilla medido na fonte (`lab/typst-original/crates/typst-library/src/
foundations/mod.rs:198-247`):

- `assert.eq(left, right, message:)`: se `left != right` → erro
  `equality assertion failed: {message}` ou, sem `message`,
  `equality assertion failed: value {left.repr()} was not equal to {right.repr()}`. Sucesso → `none`.
- `assert.ne`: simétrico — `inequality assertion failed: {message}` /
  `inequality assertion failed: value {left.repr()} was equal to {right.repr()}`.
- Igualdade com coerção Int↔Float (o `==` da linguagem; cristalino já
  tem essa semântica no helper privado `operators.rs::value_eq:362-368`).

Consumidor real em `cetz` (file:line, pacote `@preview/cetz:0.5.2`):
`shapes.typ:151` (no caminho de `line`/`circle` do documento de
reprodução), `shapes.typ:249,484,897,1387`, `anchor.typ:123,186`,
`boolean.typ:189`.

### 1.4 Bloqueio real 2 — spread `..sink` em padrão de `for` (implementado neste passo)

Após `assert.eq`, o `cetz` avançou para
`error: cannot destructure 4 values into 2 bindings`
(`control_flow.rs:154-162`, P540). Segundo build instrumentado
temporário (revertido) mostrou:

```
bindings=["kind", "args"] item=("c", (-0.551784, 1.0, 0.0), (-1.0, 0.551784, 0.0), (-1.0, 0.0, 0.0))
```

Consumidor real: `path-util.typ:106` — `for (kind, ..args) in segments`.
O vanilla (`typst-eval/flow.rs:114-162`) chama `destructure(vm, pattern,
value)` por item — o destructuring genérico de `binding.rs`, **com
spread**. O cristalino tinha um bind manual sem spread; o destructuring
com spread já existia para `#let` (`bindings.rs:119-170`, P715).

Efeito colateral positivo (medido): a delegação fecha a divergência
pré-existente notada em P719 — a mensagem de aridade do `for` passa a ser
a do vanilla (`too many elements to destructure` + hint,
`wrong_number_of_elements`). Item correspondente em
`achados-adiados-cetz.md` marcado como fechado.

### 1.5 Bloqueio actual (3º) — isolado para P724, não corrigido neste passo

```
$ time ./target/release/typst /tmp/p723-cetz.typ /tmp/p723-cetz.pdf
/tmp/p723-cetz.typ:<detached>: error: unexpected argument
real    0m55,240s
Exit code: 1
```

Terceiro build instrumentado temporário (revertido) identificou:
closure **sem parâmetros** chamada com o argumento
`(0, ("c", (-0.551784, 1.0, 0.0), ...))` — um par de `enumerate()`.
Consumidor: `path-util.typ:453` —
`segments.enumerate().filter(((i, segment)) => ...)`.

Mecanismo: `eval_closure_expr` (`closures.rs:345`, braço
`_ => None, // Placeholder, Destructuring — adiado`) **descarta
silenciosamente** o parâmetro quando o pattern é destructuring → a
closure é criada com 0 params → `apply_closure` (`closures.rs:254-262`,
P708) rejeita o argumento extra. A correcção exige mudança no modelo
`ClosureParam` (guardar o pattern, não só o nome) + bind via
`destructure_let` em `apply_closure` — mudança estrutural com L0 próprio
(`entities/func.md`), fora do âmbito deste passo (o critério de fecho do
passo prevê explicitamente «se não [for sucesso completo], campos fixos
de progresso registados»).

## 2. Implementação

### 2.1 `assert.eq` / `assert.ne` (L0: `prompts/rules/stdlib/assert.md`)

- `01_core/src/rules/stdlib/assert.rs` — `native_assert_eq` e
  `native_assert_ne` (mesma assinatura de `native_assert`): validação de
  named args (só `message`), 2 posicionais obrigatórios, igualdade da
  linguagem via helper local `values_equal` (4 linhas duplicadas de
  `operators.rs::value_eq` — mesmo padrão já usado em `color.rs`),
  mensagens default **exactas** do vanilla com `repr_value` (P721),
  `message:` customizada (Str/Content→plain_text/outro→type_name, como
  `native_assert`).
- `01_core/src/rules/stdlib/mod.rs:65` — export das duas funções.
- `01_core/src/rules/eval/mod.rs` — `assert` passa a
  `Func::native_with_namespace` com `eq`/`ne` (mesma forma do bloco de
  `curve` de P513).

### 2.2 Spread em padrão de `for` (L0: `prompts/rules/eval.md` §P723)

- `01_core/src/rules/eval/control_flow.rs` — o bind manual de P540
  (`bindings.is_empty()` / `len()==1` define directo / resto destrói
  posicionalmente com mensagem própria) substituído por
  `destructure_let(loop_expr.pattern(), item, scopes, ctx, engine)?` —
  a mesma entrada do `#let`.
- `01_core/src/rules/eval/bindings.rs` — `destructure_let` passa a
  `pub(super)` (única mudança de visibilidade).
- Cobertura: `for x in arr`, `for _ in arr`, `for (k, v) in dict`
  (P719), `for (a, ..rest) in arr` (**novo**), `for (a,) in ((1,),)`
  (**corrigido** — antes ligava o array inteiro). Mudança aceite e
  registada no L0: `for () in (1,)` passa a errar como o vanilla
  (validação do pattern contra cada item) em vez de no-op.

### 2.3 Testes (14 novos, todos fail-first)

- 8 unitários em `stdlib/mod.rs` (P723): igualdade, mensagens default
  **verbatim** do vanilla, `message:` customizada, coerção Int↔Float,
  aridade.
- 6 E2E em `eval/tests.rs`: `assert.eq` via namespace (sucesso e erro
  aborta avaliação), spread `for (kind, ..args)` (resto e primeiro
  elemento), tuplo de 1 elemento, mensagem de aridade do vanilla.

## 3. Validação

- `cargo test -p typst-core --lib p723` → **14 passed, 0 failed**.
- `cargo test --workspace` → **0 failed** (core 3948 = 3934 de P722 + 14
  novos; resto do workspace verde, 5+3 ignored pré-existentes).
- `crystalline-lint --fix-hashes .` + `crystalline-lint .` → **0
  violations**. Nota: o `--fix-hashes` não actualizou
  `bindings.rs` (ficheiro com dois `@prompt`); o hash de `eval.md`
  (`01633311`) foi alinhado manualmente com os irmãos e o lint
  re-verificado limpo.
- `cetz` re-testado (campos fixos em §1.5): dois bloqueios resolvidos
  neste passo (`assert.eq` e spread no `for`), terceiro isolado.

## 4. ADRs

Grep a `00_nucleo/adr` pelos termos centrais (`assert`, namespace,
spread): apenas usos de `assert_eq!` em testes de Rust e a ADR-0025
(coerção Int↔Float — respeitada por `values_equal`). Nenhuma decisão
vigente rege namespaces de funções nativas ou spread em padrões de `for`
— nada a atualizar.

## 5. Critério de fecho do passo

- [x] Sonda completa, lista e comportamento confirmados — **com correcção
  da premissa**: o bloqueio não era `curve` (namespace existe desde P513)
  mas `assert.eq`; registado em §1.1.
- [x] Implementado e testado (2 correções: assert.eq/ne + for-spread).
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
- [x] `cetz` re-testado — não foi sucesso completo; campos fixos de
  progresso registados (§1.5), bloqueio actual isolado para P724.
- [x] Grep às ADRs em vigor (§4).
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p723.md`
  (hash do commit preenchido no commit seguinte).
