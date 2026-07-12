# Paridade Produção — P720 — Concatenação de arrays (`array + array`) e merge de dicts

**Data:** 2026-07-12
**Passo:** `00_nucleo/materialization/typst-passo-720.md`
**Hash do commit (implementação):** `7926a979f`.
**HEAD base:** `ccb5dde1c` (fim de P719).
**Estado:** FECHADO — `Array + Array` (concatenação) e `Dict + Dict`
(merge, direita vence em colisão, posição preservada) implementados com
paridade ao vanilla, casos vazios e de colisão confirmados.

---

## 1. Sonda

### 1.1 Comportamento completo no vanilla

Binário `lab/typst-original/target/release/typst`, repositório em
`ccb5dde1c`. Documento do passo (`/tmp/p720-concat.typ`):

```
(1, 2) + (3, 4)   → (1, 2, 3, 4)
() + (1, 2)       → (1, 2)
(1, 2) + ()       → (1, 2)
(a: 1) + (b: 2)   → (a: 1, b: 2)
(a: 1) + (a: 99)  → (a: 99)
```

**`Dict + Dict` existe e foi confirmado**, ao contrário do que a
formulação inicial do passo deixava em aberto. Medição adicional de
colisão com chaves não coincidentes em ambos os lados:

```
(a: 1, b: 2) + (b: 99, c: 3) → (a: 1, b: 99, c: 3)
```

Chave `b` (colisão) fica na **posição original** (índice 1), com o
**valor do lado direito** (99); `c` (só no lado direito) é acrescentada
no fim. Semântica de `IndexMap::insert`/`extend`: actualiza in-place,
não move para o fim.

### 1.2 Mecanismo vanilla (`foundations/array.rs:1203-1216`, `dict.rs:388-404`)

```rust
impl Add for Array { fn add(mut self, rhs) { self += rhs; self } }
impl AddAssign for Array { fn add_assign(&mut self, rhs) { self.0.extend(rhs.0); } }
// Dict: idêntico, sobre IndexMap::extend
```

Ambos delegam directamente para `extend` das colecções subjacentes — sem
lógica adicional (dedup, ordenação, etc.).

### 1.3 Uso real em `cetz` (0.5.2)

`grep -n "+ ("` em `path-util.typ`/`bezier.typ`/`hobby.typ` (os três
ficheiros apontados por P719): **só `Array + Array`** —
`path-util.typ:423,430`, `bezier.typ:413,522,524`,
`hobby.typ:77,78,126`. **Nenhum `Dict + Dict`** encontrado nestes
ficheiros.

### 1.4 `Dict + Dict` — sem consumidor confirmado, implementado por partilhar a mesma decisão estrutural

Decisão registada (não estimada): apesar de sem consumidor medido em
`cetz` nestes três ficheiros, `Dict + Dict` foi implementado no mesmo
passo porque (a) o vanilla trata os dois pares lado a lado na mesma
posição estrutural do `match` (`ops.rs:39-40,140-141`); (b) o custo de
a acrescentar é uma linha idêntica à de `Array`, reaproveitando a mesma
técnica (`extend`); (c) omiti-la criaria uma assimetria arbitrária no
mesmo braço `BinOp::Add` sem ganho de rigor — mesmo raciocínio já usado
em P718 para `Value::Args` em `eval_args`.

### 1.5 Achado adicional medido, fora do scope deste passo — `Array * Int`

Durante a sonda, medido que `hobby.typ:77,78` (as mesmas linhas citadas
como consumidoras de `+`) também usam `Array * Int` (repetição,
`(0,) * (n - 1)`), **também ausente** do cristalino
(`error: cannot apply Mul to array and int`, medido com `#((0,) * 3)`
vs `(0, 0, 0)` no vanilla). **Não implementado aqui** — o título e o
critério de fecho deste passo são especificamente "concatenação de
arrays" (`+`); `*` é um operador e mecanismo distintos (repetição, não
concatenação). Registado como o próximo candidato a bloqueio de `cetz`
mesmo antes de re-testar (a expressão completa de `hobby.typ:77` só
funciona com os dois operadores).

### 1.6 Estado do cristalino antes

`/tmp/p720-concat.typ` → `error: cannot apply Add to array and array`
na primeira linha, confirmando o bloqueio relatado por P719.

---

## 2. Implementação

L0 actualizado primeiro: `00_nucleo/prompts/rules/eval/ops.md` §P720
(hash `dc990d82` via `crystalline-lint --fix-hashes` — note-se que
`operators.rs` tem o seu **próprio** L0, distinto de `eval.md`, seguido
correctamente aqui). Testes escritos antes do código: 6 testes `p720_*`
em `eval/tests.rs`, todos a falhar no estado P719.

- **`operators.rs`**: dois braços novos em `eval_binary_op`, adjacentes
  aos de `Str + Str`/`Content + Content` (mesmo agrupamento "Adição"):
  `(BinOp::Add, Value::Array(mut a), Value::Array(b)) => { a.extend(b);
  Ok(Value::Array(a)) }` e o par simétrico para `Value::Dict`. Sem
  lógica adicional — `Vec::extend`/`IndexMap::extend` já implementam
  exactamente a semântica medida.

---

## 3. Validação

Estado da medição: working tree com exactamente as alterações deste
passo, commitado de seguida como `7926a979f` (o diff do commit é o
estado medido). Re-verificação pós-commit em §3.4.

### 3.1 Documento do passo vs vanilla

`/tmp/p720-concat.typ` → `pdftotext` **idêntico** ao vanilla:
`(1, 2, 3, 4) (1, 2) (1, 2) (a: 1, b: 2) (a: 99)`.

### 3.2 Suites

- `cargo test --workspace` → **3918 passed, 0 failed** no `typst-core`
  (3912 em P719 + 6 novos), restantes crates verdes.
- `crystalline-lint .` → **0 violations**.

### 3.3 Reprodução `cetz` — campos fixos de progresso

Documento do passo (`/tmp/p720-cetz.typ`):

- **Exit code:** 1. **Tempo:** 51,8s (ver §3.4 para a medição
  pós-commit).
- **Bloqueio de P719 resolvido** — `cannot apply Add to array and
  array` desapareceu.
- **Próximo bloqueio, com `file:line`:** `error: esta função não tem
  campos` — field access (`.move`/`.line`/`.cubic`/`.close`) sobre o
  `Value::Func` nativo `curve` (módulo built-in de construção de
  curvas), que não tem namespace registado no cristalino
  (`bindings.rs:1396`, braço `Value::Func(f) => match f.namespace() {
  None => Err(...) }`, P493b). Consumidor real: `canvas.typ:151,156,
  159,166` (`curve.move(...)`, `curve.line(...)`, `curve.cubic(...)`,
  `curve.close(...)`). Candidato a P721 — implementar o namespace de
  `curve` (mecanismo distinto: registo de funções associadas a um
  `Value::Func` nativo, não operador binário).

### 3.4 Re-verificação pós-commit (proveniência)

Executada no commit `7926a979f`, working tree limpa:

- `/tmp/p720-concat.typ` → mesmo resultado: `(1, 2, 3, 4) (1, 2) (1, 2)
  (a: 1, b: 2) (a: 99)`.
- `cetz` → mesmo bloqueio (`esta função não tem campos`), tempo real
  52,3s (51,8s pré-commit — variação normal de ruído de máquina; o
  documento continua a falhar na fase de eval, antes de qualquer
  layout).

---

## 4. Gate das ADRs (critério do passo)

Grep a `Add`/concatenação em `00_nucleo/adr/*.md`: ocorrências
encontradas (`typst-adr-0034`: `impl Add for Align2D`, hipotético e não
relacionado; demais são acertos genéricos da palavra inglesa "add" em
contextos de roadmap sem relação com o operador `+`). ADR-0107/0108/0109
não mencionam o tema. **Nada contradiz o decidido.**

---

## 5. Critério de fecho do passo

- [x] Sonda completa — `Array + Array` confirmado incluindo casos
  vazios; `Dict + Dict` confirmado (não assumido) com o comportamento
  exacto de colisão (posição preservada, direita vence); uso real em
  `cetz` confirmado com `file:line` (só `Array + Array`); scope-out de
  `Dict + Dict` registado como decisão medida de implementar mesmo sem
  consumidor directo (§1.4).
- [x] Implementado e testado (6 testes novos), incluindo casos vazios e
  de colisão de chaves.
- [x] Sem regressão — `cargo test --workspace` verde (3918/0).
- [x] `crystalline-lint .` limpo.
- [x] `cetz` re-testado — campos fixos registados (tempo 51,8s, exit 1,
  próximo bloqueio com `file:line`: `canvas.typ:151,156,159,166`,
  namespace de `curve` ausente).
- [x] Grep às ADRs pelos termos centrais — nada contradiz (§4).
- [x] Relatório com hash do commit.
