# Prompt L0 — `compiler/eval/operators/error_formatting` — mensagens de fronteira
Hash do Código: 67e7ee2b

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/eval/operators/error_formatting.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/eval/operators.md`
**ADRs**: ADR-0107 (a mensagem de erro é o observável — aqui a mecânica **é** a língua)

---

## Contexto

Quando nenhuma combinação de operandos tem braço, o erro emitido é um
observável da linguagem: o vanilla tem formatos verbatim por operador e usa
os **nomes longos** de tipo. Neste domínio a paridade mede-se ao texto
(ADR-0107: a mecânica é o observável quando o observável é a mensagem).

## Instrução

### Braço de fronteira (último do `match` de `eval_binary_op`)

`(op, lhs, rhs) => Err(binary_mismatch(op, &lhs, &rhs))` — sempre em último
lugar (a ordem dos braços é semântica, invariante 2 do hub).

### `binary_mismatch` — formatos verbatim do vanilla

Fonte: `foundations/ops.rs:170,214,284,340,500`, medido nos dois binários.

| Op | Formato | Exemplo medido |
|---|---|---|
| `Add` | `cannot add {a} and {b}` | `cannot add length and direction` |
| `Sub` | `cannot subtract {b} from {a}` (**ordem invertida**) | `cannot subtract direction from length` |
| `Mul` | `cannot multiply {a} with {b}` | `cannot multiply integer with direction` |
| `Div` | `cannot divide {a} by {b}` | `cannot divide integer by direction` |
| `Lt`/`Leq`/`Gt`/`Geq` (par incomparável) | `cannot compare {a} and {b}` (emitido pelo braço combinado de ordenação, não por `binary_mismatch`) | `cannot compare direction and integer` |
| restantes ops | `cannot apply {op:?} to {a} and {b}` (formato pré-verbatim, com nomes longos) | — |

### `vanilla_type_name` — nomes longos de tipo (ponto único de verdade)

Nome longo de cada variant de `Value`, igual ao `long_name` de cada
`#[ty]` do vanilla — **distinto** de `Value::type_name()` (nomes curtos do
`type()`/`repr`). Mapeamento completo (variant → string):

`none`, `auto`, `boolean`, `integer`, `float`, `string`, `array`,
`dictionary`, `module`, `datetime`, `function`, `content`, `length`,
`relative length`, `ratio`, `angle`, `color`, `stroke`, `fraction`,
`alignment`, `location`, `gradient`, `regex`, `tiling`, `bytes`, `decimal`,
`duration`, `version`, `selector`, `symbol`, `arguments`, `state`,
`counter`, `label`, `direction`, `type`.

**P1015/P1017** — esta função é o **ponto único de verdade** para nomes de
tipo longos em todo o L1. A equivalência com as implementações anteriores
(`long_type_name` em `eval/bindings/access.rs`, `eval/operators/join.rs`,
`stdlib/foundations.rs`, e `vanilla_type_name` em `stdlib/loading.rs` e
`stdlib/pdf.rs`) foi provada no P1015 pela bijectividade de
`Value::type_name()` (36 variantes → 36 strings distintas). A tabela
exaustiva aqui foi escolhida como canónica porque falha de compilação se
`Value` ganhar uma variante sem entrada — preferível a herança silenciosa de
nome via `type_name()`.

Usado por `binary_mismatch`, pelo braço combinado de ordenação, e por todos
os outros módulos que precisam de nomes longos de tipo em mensagens de erro.

### Fronteira unária (divergência registada)

`eval_unary_op` erra `"cannot apply {op:?} to {type_name()}"` com o nome
**curto** do tipo — divergência de texto face ao vanilla (nomes longos),
registada como candidata a correcção futura; não faz parte deste nó mudar
isso sem medição nova.

## Restrições Estruturais

- L1 puro (ver hub); estas funções só formatam strings — nunca avaliam.
- `vanilla_type_name` tem de cobrir **todas** as variants de `Value`
  (match exaustivo — uma variant nova sem entrada é erro de compilação,
  por design).

## Critérios de Verificação

```
eval_binary_op(Mul, Int(2), Dir(LTR))     == Err("cannot multiply integer with direction")
eval_binary_op(Mul, Dir(LTR), Int(2))     == Err("cannot multiply direction with integer")
eval_binary_op(Add, Length(1em), Dir(_))  == Err("cannot add length and direction")
eval_binary_op(Sub, Length(1pt), Dir(_))  == Err("cannot subtract direction from length")
eval_binary_op(Div, Int(1), Dir(_))       == Err("cannot divide integer by direction")
eval_binary_op(Lt, Dir(LTR), Int(2))      == Err("cannot compare direction and integer")
eval_binary_op(Add, Bool(true), Int(1))   == Err("cannot add boolean and integer")
eval_binary_op(Add, Relative(..), Dir(_)) == Err("cannot add relative length and direction")
```

## Resultado Esperado

- `binary_mismatch` e `vanilla_type_name` conforme as tabelas; o teste
  unitário de fronteira (mensagens verbatim) no próprio ficheiro.
