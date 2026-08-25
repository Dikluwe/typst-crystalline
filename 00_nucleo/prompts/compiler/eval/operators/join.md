# Prompt L0 — `compiler/eval/operators/join` — combinação sequencial de valores
Hash do Código: 9bc7688c

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/eval/operators/join.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/eval/operators.md`
**ADRs**: ADR-0107 (paridade língua)

---

## Contexto

`join` é a combinação dos valores produzidos pelas expressões de um code
block e dos corpos de `for`/`while` (paridade `ops::join`,
`foundations/ops.rs:24-45`; consumidores: `typst-eval/code.rs:57`,
`typst-eval/flow.rs:86,132`). No cristalino a **acumulação** vive nos
consumidores (`eval/mod.rs` para `Expr::CodeBlock`, `control_flow.rs` para
os loops — L0 `compiler/eval.md`); este nó possui só a **tabela de
combinação** e o seu erro.

## Instrução

### `join(lhs, rhs)` — tabela

| Combinação | Resultado |
|---|---|
| `(a, None)` / `(None, b)` | `a` / `b` — `None` é identidade nos dois lados |
| `Str + Str` | concatenação |
| `Symbol + Symbol`, `Str ↔ Symbol` | `Str` (grapheme clusters integrais concatenados) |
| `Bytes + Bytes` | concatenação de bytes |
| `Content + Content`, `Content ↔ Str`, `Content ↔ Symbol` | `Content` sequência (`Content::sequence`/`Content::text`) |
| `Array + Array` | concatenação (ordem preservada) |
| `Dict + Dict` | merge — direita vence, posição da primeira ocorrência preservada |
| `Args + Args` | merge de `items` e `named` |
| qualquer outra | **erro** `"cannot join {a} with {b}"` |

Medições vanilla: `{ "a"; "b" }` → `"ab"`; `{ none; (1,) }` → `(1,)`;
`{ (:); (a: 1) }` → `(a: 1)`; `{ 1; none }` → `1`; `{ 1; 2 }` → erro
"cannot join integer with integer".

### Nomes longos no erro de join — `long_type_name`, **não é deste nó**

O erro usa os nomes **longos** de tipo (paridade `Type::long_name` via
`mismatch!`; medido: `(1, 2).join("-")` no vanilla → "cannot join integer
with string"). Difere do nome curto só em três casos: `int → integer`,
`str → string`, `bool → boolean`; o resto delega em `type_name()`.

**P1015** — este nó **não possui** `long_type_name`. A redacção anterior
descrevia-a como se fosse a única implementação, quando existia uma cópia
privada aqui e outra em `stdlib/foundations.rs`, ambas duplicando a
`pub(crate)` de `compiler/eval/bindings/access.md`. As cópias foram
removidas; `join` importa
`crate::compiler::eval::long_type_name`. O dono da função é
**`compiler/eval/bindings/access.md`**.

## Restrições Estruturais

- L1 puro (ver hub).
- `join` não avalia — só combina `Value`s já avaliados. A ordem de
  avaliação e a acumulação são responsabilidade dos consumidores.

## Critérios de Verificação

```
join(Str("a"), Str("b"))                  == Str("ab")
join(Int(1), Int(2))                      == Err("cannot join integer with integer")
join(Array[1], Array[2])                  == Array[1, 2]
join(Dict{a:1}, Dict{b:2})                == Dict{a:1, b:2}
join(Args(..), Args(..))                  == merge de items e named
join(Content([a]), Content([b]))          == Content([a b])
join(none, x) / join(x, none)             == x

// via eval (consumidores)
eval("#let x = { (1,); (2,) }; #repr(x)") == "(1, 2)"
eval("#let x = { \"a\"; \"b\" }; #repr(x)") == "\"ab\""
eval("#let x = { 1; 2 }")                 == Err (cannot join)
eval("#let x = for i in (1,) { (1,); (2,) } #repr(x)") == "(1, 2)"
eval("#let x = for i in (1,) { 1; 2 }")   == Err (cannot join)
eval("#for i in (1, 2) [x]")              == Content("xx")  // sem regressão
join(Str("a"), Symbol("♥️"))              == Str("a♥️")
join(Content([a]), Symbol("👩‍💻"))         == Content("a👩‍💻")
```

## Resultado Esperado

- `join` e `long_type_name` conforme a tabela; testes unitários no ficheiro
  e E2E nos consumidores; zero regressão na suite do eval.
