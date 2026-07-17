# Paridade Produção — P722 — `Array * Int` / `Int * Array` (repetição)

**Data:** 2026-07-13
**Passo:** `00_nucleo/materialization/typst-passo-722.md`
**Hash do commit (implementação):** `b9dc92fb1`.
**HEAD base:** `cd5981056` (fim de P721, branch `Tekt`).
**Estado:** FECHADO — repetição de array por inteiro implementada nas
duas ordens, com paridade ao vanilla incluindo a mensagem de erro para
contagem negativa; `Dict * Int` confirmado inexistente no vanilla
(scope-out). `cetz` re-testado — mesmo bloqueio de P720 (`curve`
namespace), sem regressão.

---

## 1. Sonda

### 1.1 Comportamento exacto no vanilla (medido + fonte)

Binário `lab/typst-original/target/release/typst`, repositório em
`cd5981056`. Cada caso sondado isoladamente (o caso negativo aborta o
documento, o que esconderia os anteriores num só ficheiro):

```
(0,) * 3    → (0, 0, 0)
3 * (0,)    → (0, 0, 0)     (ordem inversa funciona)
(1, 2) * 0  → ()
(1, 2) * -1 → error: number must be at least zero
(:) * 2     → error: cannot multiply dictionary with integer
```

Fonte (não assumido):

- `foundations/ops.rs:274-275` — dois braços, um por ordem:
  `(Array(a), Int(b)) => Array(a.repeat(...cast()?)?)` e simétrico.
- O `Int` é convertido a `usize` via `cast`: negativo →
  `"number must be at least zero"` (`foundations/int.rs:507`).
- `Array::repeat` (`array.rs:140-147`): `len.checked_mul(n)`; overflow
  → `"cannot repeat this array {n} times"`; caso contrário
  `iter().cloned().cycle().take(count)`.
- `Dict * Int` **não tem braço** no vanilla — cai na fronteira genérica
  de `Mul`. Scope-out confirmado por medição, não por inferência.

### 1.2 Estado do cristalino antes

Mesmos casos, cristalino em `cd5981056` (binário de P721):

```
(0,) * 3    → error: cannot apply Mul to array and int
3 * (0,)    → error: cannot apply Mul to int and array
(1, 2) * 0  → error: cannot apply Mul to array and int
(1, 2) * -1 → error: cannot apply Mul to array and int
(:) * 2     → error: cannot apply Mul to dictionary and int
```

Todos os pares caíam no fronteira genérico
(`operators.rs:336-339`). `Dict * Int` já era erro — comportamento
correcto mantido (só a redacção difere da do vanilla, coerente com os
demais pares inválidos de `Mul`; pré-existente, fora do scope).

### 1.3 Consumidor real em `cetz`

`hobby.typ:77,78` — as mesmas linhas que motivaram P720:
`(0,) * (n - 1)` (repetição de coordenadas de path). Medido por P720
durante a sonda de `+` e adiado conscientemente para este passo
(`achados-adiados-cetz.md`).

---

## 2. Implementação

L0 actualizado primeiro: `00_nucleo/prompts/engine/eval/ops.md` — secção
P722 (mecanismo vanilla com `file:line`, scope-out de `Dict * Int`),
testes canónicos, histórico. Hash actualizado via
`crystalline-lint --fix-hashes` (`operators.rs` → `424a219c`).
Testes escritos antes do código: 7 testes `p722_*` em `eval/tests.rs`
— **6 a falhar** no estado base (o de `Dict * Int` já passava: fronteira
genérica já era o comportamento correcto).

Código (`01_core/src/engine/eval/operators.rs`, secção Multiplicação):

```rust
(BinOp::Mul, Value::Array(a), Value::Int(n)) | (BinOp::Mul, Value::Int(n), Value::Array(a)) => {
    if n < 0 {
        return Err("number must be at least zero".into());
    }
    let count = a
        .len()
        .checked_mul(n as usize)
        .ok_or_else(|| format!("cannot repeat this array {n} times"))?;
    Ok(Value::Array(a.iter().cloned().cycle().take(count).collect()))
}
```

Um braço com guarda compartilhada cobre as duas ordens — espelho directo
de `Array::repeat` (`cycle().take(count)`, incluindo `() * n → ()`).

---

## 3. Validação

Estado da medição: working tree com exactamente as alterações deste
passo (`00_nucleo/prompts/engine/eval/ops.md`,
`01_core/src/engine/eval/operators.rs`,
`01_core/src/engine/eval/tests.rs`), commitado de seguida. Binário
release reconstruído desse estado.

### 3.1 Casos do passo vs vanilla

5/5 conformes:

| Caso | Vanilla | Cristalino |
|---|---|---|
| `(0,) * 3` | `(0, 0, 0)` | `(0, 0, 0)` ✓ |
| `3 * (0,)` | `(0, 0, 0)` | `(0, 0, 0)` ✓ |
| `(1, 2) * 0` | `()` | `()` ✓ |
| `(1, 2) * -1` | erro `number must be at least zero` | idem, mensagem exacta ✓ |
| `(:) * 2` | erro (fronteira) | erro (fronteira) ✓ |

### 3.2 Suites

- `cargo test --workspace` → **0 failed** em todos os crates
  (`typst-core`: 3934 passed — 3927 de P721 + 7 novos).
- `crystalline-lint .` → **0 violations**.

### 3.3 Reprodução `cetz` — campos fixos de progresso

Documento do passo (`/tmp/p722-cetz.typ`):

- **Exit code:** 1. **Tempo:** 53,6s (P720 pós-commit: 52,3s — ruído de
  máquina; o documento continua a falhar na fase de eval, antes de
  qualquer layout).
- **Bloqueio inalterado** (o esperado): `error: esta função não tem
  campos` — field access sobre o `Value::Func` nativo `curve`, sem
  namespace registado (`bindings.rs:1396`, braço
  `Value::Func(f) => match f.namespace() { None => Err(...) }`).
  Consumidor real medido em P720: `canvas.typ:151,156,159,166`
  (`curve.move/.line/.cubic/.close`).
- Os dois operadores de `hobby.typ:77` (`+` de P720, `*` deste passo)
  estão fechados; a ordem de avaliação atinge as chamadas `curve.*` de
  `canvas.typ` antes, daí o bloqueio ser o mesmo nas duas medições.

---

## 4. Gate das ADRs (critério do passo)

Grep a `Mul`/repetição/`repeat` em `00_nucleo/adr/`: todas as
ocorrências de `repeat` referem-se ao elemento de layout `repeat()`
(P156J) e ao `repeat` de header/footer de tabela (P157C) — nenhuma ao
operador `*` ou a repetição de arrays. Nenhuma ocorrência de `Mul`.
ADR-0107/0108/0109 não tocam o tema. **Nada contradiz o decidido.**

---

## 5. Critério de fecho do passo

- [x] Sonda completa — ordem inversa, zero, negativo (mensagem exacta) e
  `Dict * Int` (inexistente no vanilla) confirmados por medição + fonte.
- [x] Implementado e testado — 7 testes novos, incluindo casos de borda
  (zero, negativo, array vazio, ordem inversa, E2E padrão `cetz`).
- [x] Sem regressão — `cargo test --workspace` verde (3934/0 no core).
- [x] `crystalline-lint .` limpo.
- [x] `cetz` re-testado — campos fixos registados (53,6s, exit 1,
  bloqueio `curve` inalterado de P720, com `file:line` do consumidor).
- [x] Grep às ADRs pelos termos centrais — nada contradiz (§4).
- [x] Relatório com hash do commit.
- [x] Item marcado como fechado em `achados-adiados-cetz.md` (P722).
