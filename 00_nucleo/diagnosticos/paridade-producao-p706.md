# Paridade Produção — P706 — Operador `in` / `not in`

**Data:** 2026-07-11
**Passo:** `00_nucleo/materialization/typst-passo-706.md`
**Hash do commit (implementação):** a preencher no commit seguinte.
**HEAD base:** `e08e728e2` (fim de P705, detached HEAD).
**Estado:** ÂMBITO DE P706 FECHADO — `in`/`not in` implementado para todas as combinações medidas (`Str`/`Dict`/`Str`/`Array`, incluindo tipos mistos e arrays aninhados), paridade vanilla confirmada. `cetz` avança **substancialmente mais fundo** (tempo de compilação salta de ~7s para ~30.7s), novo bloqueio isolado fora da área de operadores: `Arguments::pos()`.

---

## 1. Sonda — gap maior do que P705 assumia

`grep -rn " in "` a todo o `cetz` (filtrando `for x in`) localizou 9 usos
reais do operador de pertença: `mark.typ:75,174,200` (`Str in Dict`),
`mark.typ:81,118` + `draw/projection.typ:187` (`Str in Array`),
`matrix.typ:252,255,383,406` (`Array in Array-de-Arrays`), e
`mark.typ:157` (`not in` com tipos mistos, `slant not in (none, 0%)`).

**Testado isoladamente no cristalino, cada combinação** (não assumido):
**nenhuma** funcionava — nem `1 in (1,2,3)`, a mais simples. O parser já
reconhecia `in`/`not in` (`BinOp::In`/`NotIn` já existiam e chegavam a
`eval_binary_op`), mas caíam sempre no fronteira genérico. O gap era mais
amplo do que P705 supunha ("talvez só falte `Str in Dict`").

### Combinações confirmadas contra o vanilla

| Expressão | Vanilla | Notas |
|---|---|---|
| `"a" in (a:1,b:2)` | `true` | `Str in Dict`, chave existe |
| `"z" in (a:1,b:2)` | `false` | chave não existe |
| `"ell" in "hello"` | `true` | `Str in Str`, substring |
| `1 in (1,2,3)` | `true` | `Int in Array` |
| `1 not in (1,2,3)` | `false` | negação |
| `none in (none, 0%)` | `true` | tipos mistos no array |
| `(1,2) in ((1,2),(3,4))` | `true` | array-de-arrays, igualdade recursiva |
| `1 in (1.0, 2.0)` | `true` | coerção Int/Float, mesma do `==` |
| `1 in "hello"` | `Err "cannot apply 'in' to integer and string"` | tipos incompatíveis |

---

## 2. Implementação

### `eval_binary_op` (`01_core/src/rules/eval/operators.rs`)

Novos braços antes do fronteira genérico:
- `(In, Str, Dict)` → `dict.contains_key(s.as_str())`.
- `(In, Str, Str)` → `haystack.contains(needle)`.
- `(In, any, Array)` → `arr.iter().any(|item| value_eq(&needle, item))`.
- Espelhos `NotIn` (negação do mesmo cálculo, sem duplicar lógica).

Nova função `value_eq(a, b)` — mesma coerção Int/Float do `BinOp::Eq`
(medido: `1 in (1.0, 2.0)` → `true`), delegando ao `PartialEq` derivado de
`Value` para todo o resto — cobre arrays aninhados e tipos mistos **sem
código extra** (o `PartialEq` de `Value::Array` já é recursivo).

Combinações sem braço específico (`Int in Str`) caem no fronteira
genérico já existente — mensagem diverge do texto exacto do vanilla
(mecânica, ADR-0107), mas continua a ser erro.

### Testes (9 novos, `eval/tests.rs`)

`Str in Dict` (existe/não existe), `Str in Str`, `Int in Array`,
array-de-arrays (reproduz `matrix.typ:252`), tipos mistos (reproduz
`mark.typ:157`), coerção Int/Float, `not in`, tipos incompatíveis → erro.

---

## 3. Ficheiros tocados

- **L0**: `00_nucleo/prompts/rules/eval/ops.md` — nova secção "P706 —
  Operador `in`/`not in`" com a tabela de combinações e a semântica de
  implementação.
- **Código**: `01_core/src/rules/eval/operators.rs` (braços `In`/`NotIn` +
  `value_eq`), `01_core/src/rules/eval/tests.rs` (9 testes).

---

## 4. Validação — âmbito de P706 confirmado

Reexecutados todos os casos da sonda (release build): todos os resultados
**idênticos ao vanilla**, incluindo `not in`, tipos mistos, array-de-arrays,
coerção Int/Float, e o erro para tipos incompatíveis (`Int in Str`).

- `cargo test --workspace` → **3781 passed**, 0 failed (3772 de P705 + 9
  novos de P706); `typst-infra` inalterado (626/5).
- `crystalline-lint .` → 0 violations (hash de `operators.rs` realinhado
  com `--fix-hashes`).

---

## 5. Repetição da reprodução de P700-705 — salto real de profundidade

```
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
```

**Tempo de compilação: ~30.7s** — salto real face aos ~7-7.3s dos passos
P702-P705 (mais de 4×). Sinal forte de que `in`/`not in` desbloqueou uma
fase de avaliação bem mais extensa de `cetz` (usado em pelo menos 4
ficheiros-fonte diferentes: `mark.typ`, `matrix.typ`,
`draw/projection.typ`), não um ajuste marginal.

**Novo bloqueio**: `error: campo desconhecido em arguments: 'pos'`.
Isolado: `coordinate.typ:391`, `drawable.typ:70,92`, `util.typ:33,34,39`
usam `algo.pos()` — **método**, não campo. Confirmado standalone:
```
#let f(..args) = args.pos()
#f(1, 2, x: 3)
```
→ vanilla: exit 0, `(1, 2)`. Cristalino: `error: campo desconhecido em
arguments: 'pos'`.

**Causa**: `Value::Args` no cristalino expõe `"positional"` como **campo**
(`rules/eval/bindings.rs:609-616`, P504), mas o vanilla usa `.pos()` como
**método** (`Arguments::pos()`), não campo. Não é a mesma coisa que
`Str in Str`/`Dict` — é um gap de API diferente, numa área diferente
(`Arguments`, não operadores). Fora do âmbito de P706, registado para
próximo passo.

### Próximo passo sugerido (P707, não iniciado)

1. Confirmar no vanilla a assinatura completa de `Arguments` (`.pos()`,
   `.named()`, `.pairs()`? — não assumir que só falta `.pos()`).
2. Decidir: renomear o campo `positional`/`named` existentes para métodos,
   ou adicionar `.pos()`/`.named()` como intercepção de method-call
   (mesmo padrão de P702, `eval_func_call`) que delega ao mesmo dado já
   exposto pelo campo.
3. Reexecutar a reprodução deste §5 como critério de fecho.

---

## 6. Estado da cadeia P678–706

Progresso cumulativo real: plugin WASM (P699-700), `cbor.encode` (P701),
`.with()` (P702), `rgb(hex)` (P703), `range(step:)` (P704), `luma()`
percentagem (P705), e agora `in`/`not in` (P706) — todos fechados e
testados. O salto de ~7s para ~30.7s de tempo de compilação é o sinal de
progresso mais forte desde P702 — `cetz` está a avaliar uma porção bem
maior do seu código antes do próximo bloqueio. Pausada aqui, com P707
sugerido (`Arguments::pos()`).

## 7. Critério de fecho do passo

- [x] Sonda completa, linha de `cetz` localizada (9 usos, 4 ficheiros),
      todas as combinações confirmadas contra o vanilla.
- [x] Implementado e testado (9 testes).
- [x] Sem regressão em `cargo test --workspace` (3781 passed).
- [x] `crystalline-lint .` limpo.
- [x] `cetz` re-testado — **progresso substancial** (~4× mais tempo de
      compilação), próximo bloqueio identificado e isolado
      (`Arguments::pos()`), não sucesso completo.
- [x] Relatório com resultado exacto (este ficheiro).
