# Prompt L0 — `stdlib/primitives-constructors` — constructors `decimal`, `duration`, `version`
Hash do Código: 890d9b08

**Camada**: L1
**Ficheiro alvo**: `01_core/src/engine/stdlib/primitives_constructors.rs`
**Origem**: Passo 403 (`typst-passo-403.md`) — materialização de constructors stdlib para tipos L1 modelados em P399–P401. Passo 405 estende `native_duration` para named args vanilla e adiciona operações básicas eval.
**ADRs**: ADR-0017 (portão aberto), ADR-0107 (paridade linguagem), ADR-0108 (medir-antes-de-decidir), ADR-0054 (graded scope-out operações avançadas).
**Convenção partilhada**: `rules/stdlib/_comum.md`.

---

## 1. Contexto

P399 (`Decimal`), P400 (`Duration`) e P401 (`Version`) modelaram os tipos L1 e os variants em `Value`. Este passo expõe constructors stdlib puros para cada um, sem criar novos tipos ou variants.

| Função | Constructor Typst | Tipo L1 | Parse |
|--------|-------------------|---------|-------|
| `native_decimal` | `decimal("1.23")` | `Value::Decimal` | `rust_decimal::Decimal::from_str` |
| `native_duration` | `duration("3d2h30m")` / `duration(seconds: 90)` | `Value::Duration` | string canónica ou named args |
| `native_version` | `version("1.2.3")` / `version(1, 2, 3)` / `version(1, 2, 3, 4, 5)` | `Value::Version` | string de inteiros ou componentes inteiros (sem pre/build) |

---

## 2. Assinatura e ABI

Todas seguem a ABI padrão de natives sem I/O:

```rust
fn native_X(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn World,
    _current_file: FileId,
) -> SourceResult<Value>
```

- Rejeitam argumentos nomeados via `expect_no_named`.
- Requerem exatamente 1 argumento posicional `Value::Str`.
- Erro de tipo ou aridade → mensagem clara em português.
- String inválida → erro eval com mensagem indicando o constructor.

---

## 3. `native_decimal(s: Str)`

- Delega parse a `Decimal::from_str(&s)` (já exposto em `entities::decimal`).
- Sucesso: `Value::Decimal(dec)`.
- Erro: `"decimal(): string inválida: '{s}'"`.

---

## 4. `native_duration(...)`

### Forma string (compatibilidade P403)

Parser minimal canónico `NdNhNmNs`, onde cada componente é opcional mas a ordem é fixa (dias → horas → minutos → segundos). Exemplos válidos:

- `"0s"` → 0 nanos
- `"1h30m"` → 5400s
- `"2h30m"` → 9000s
- `"3d2h30m15.5s"` → 3 dias + 2h + 30m + 15.5s
- `"0.001s"` → 1_000_000 nanos

Regras:

- String vazia → erro.
- Apenas os sufixos `d`, `h`, `m`, `s` são aceites, em qualquer combinação não vazia.
- Cada sufixo aparece no máximo uma vez.
- A parte numérica antes de cada sufixo deve ser um número decimal não negativo (`u64` para `d/h/m`; `f64` para `s` com fração).
- `d`, `h`, `m` rejeitam frações.
- Componentes podem estar ausentes, mas se presentes devem respeitar a ordem `d h m s`.
- Valores resultantes são convertidos para nanossegundos `u64`; overflow → erro.

Mensagem de erro padrão: `"duration(): string inválida: '{s}'"`.

### Forma vanilla (named args)

```typst
duration(days: 3, hours: 2, minutes: 30)
duration(seconds: 90)
duration()
```

Argumentos nomeados opcionais, todos `Int`, ≥ 0, default 0:

- `weeks` (P843 F1 — o vanilla aceita-o e o repr nomeado inclui a
  componente `weeks:`; antes era silenciosamente ignorado),
  `days`, `hours`, `minutes`, `seconds`, `milliseconds`, `microseconds`, `nanoseconds`

Nota P843: `milliseconds`/`microseconds`/`nanoseconds` são extensão
cristalina (P405) — o vanilla rejeita-os (`unexpected argument:
milliseconds`, medido em `temp/p843/f1_err_ms.typ`). Mantidos por
compatibilidade com a decisão P405; divergência registada.

Validação:

- Tipo errado → erro eval.
- Valor negativo → erro eval (`'{name}' não pode ser negativo`).
- Soma total > `u64::MAX` nanossegundos → erro eval (`duration excede o máximo suportado`).
- Mistura string posicional + named args → erro eval.
- Nenhum argumento → `Duration::ZERO`.

Cálculo usa `u128` intermédio e converte para `u64` após verificação.

---

## 5. `native_version(...)`

### Forma string (compatibilidade P403)

- Delega parse a `Version::from_str(&s)` (já exposto em `entities::version`) —
  **só inteiros** separados por `.`; `pre`/`build` textuais (`"1.2.3-alpha.1"`) → erro.
- Sucesso: `Value::Version(Arc::new(version))`.
- Erro: `"version(): string inválida: '{s}'"`.

### Forma de componentes (posicional) — P684

```typst
version()
version(1)
version(1, 2, 3)
version(1, 2, 3, 4, 5)   // qualquer número de componentes inteiros
```

- Qualquer número (≥ 0) de posicionais `Int` ≥ 0 → `Version::from_components`.
- Negativo ou não-`Int` em qualquer componente → erro eval (`as_nonneg_int`).
- **Sem `pre`/`build`** (não existem no Typst): qualquer argumento nomeado → erro
  `version(): não aceita argumentos nomeados`. Um 4º posicional de texto
  (`version(1, 2, 3, "alpha.1")`) → erro de tipo (`espera Int`).

### Forma array (P682/P684)

Um único posicional `array` equivale aos componentes — `version((1, 2, 3))` ≡
`version(1, 2, 3)`, `version((1, 2, 3, 4, 5))` ≡ `version(1, 2, 3, 4, 5)`.

- Qualquer número (≥ 0) de elementos `Int` ≥ 0; string dentro do array → erro de tipo.
- Argumentos nomeados → erro (mesma regra da forma posicional).

### Igualdade e ordem

Directa sobre todos os componentes com **zero-pad** (ver `entities/version.md`):
`version(1, 2, 3) == version(1, 2, 3, 0)`; mais curto (prefixo) < mais longo.
Sem qualquer tratamento especial de `pre`/`build` (removido em P684).

---

## 6. Operações eval básicas (P405)

Adicionar em `rules/eval/operators.rs` braços para `Value::Duration`:

| Operador | Operandos | Resultado | Notas |
|----------|-----------|-----------|-------|
| `+` | Duration + Duration | Duration | overflow u64 → erro |
| `-` | Duration - Duration | Duration | underflow → erro |
| `*` | Duration * Int / Int * Duration | Duration | Int negativo → erro; overflow → erro |
| `*` | Duration * Float / Float * Duration | Duration | Float negativo → erro; trunca sub-nano |
| `/` | Duration / Int | Duration | div/0 → erro; Int negativo → erro; trunca |
| `/` | Duration / Float | Duration | div/0 → erro; Float negativo → erro; trunca |
| `/` | Duration / Duration | Float | razão `a.nanos / b.nanos` como f64 |
| `==`, `!=`, `<`, `<=`, `>`, `>=` | Duration vs Duration | Bool | via `PartialEq`/`Ord` de `Duration` (P400) |

Não implementar coerção `Duration + Int`, `Duration * Duration`, etc.

## 6b. Operações eval básicas Version (P406)

`Value::Version` já implementa `PartialEq`/`Eq`/`PartialOrd`/`Ord` (P401). Em `rules/eval/operators.rs`, adicionar braços explícitos para ordenação:

| Operador | Operandos | Resultado | Nota |
|----------|-----------|-----------|------|
| `==`, `!=` | Version vs Version | Bool | via `PartialEq` (todos os campos) |
| `<`, `>`, `<=`, `>=` | Version vs Version | Bool | via `Ord` (lexicográfico zero-pad) |

`Eq`/`Neq` já funcionam via wildcard `(BinOp::Eq, a, b) => Ok(Value::Bool(a == b))`, mas os braços explícitos garantem prioridade e clareza. Não implementar `+`, `-`, `*`, `/` nem comparações com Int/Str.

## 7. Paridade vanilla

A paridade é com a **forma da linguagem**: uma função global que converte string no tipo. A mecânica interna (parser hand-rolled vs. biblioteca) é detalhe de implementação cristalino.

- `decimal("1.5")` ≡ literal decimal `1.5`.
- `duration("1h30m")` ≡ `5400s`.
- `duration(seconds: 90) + duration(seconds: 30)` ≡ `120s`.
- `duration(minutes: 2) > duration(seconds: 119)` ≡ `true`.
- `version(1, 2, 3) == version(1, 2, 3)` ≡ `true`.
- `version(1, 2, 3) == version(1, 2, 3, 0)` ≡ `true` (zero-pad).
- `version(1, 2, 3) < version(1, 2, 3, 4)` ≡ `true` (prefixo < mais longo).
- `version(1, 2, 3, 4, 5)` é válido (componentes arbitrários).

---

## 8. Testes

### `native_decimal`

```
native_decimal([Str("1.23")]) → Ok(Value::Decimal(1.23))
native_decimal([Str("abc")]) → Err
native_decimal([Int(1)]) → Err
native_decimal([]) → Err
native_decimal([Str("1.5"), Str("2.0")]) → Err
```

### `native_duration`

```
native_duration([Str("0s")]) → Ok(Duration::ZERO)
native_duration([Str("1h30m")]) → Ok(Duration::from_seconds(5400))
native_duration([Str("2h30m")]) → Ok(Duration::from_seconds(9000))
native_duration([Str("3d2h30m15.5s")]) → Ok(3d + 2h + 30m + 15.5s em nanos)
native_duration([Str("0.001s")]) → Ok(1_000_000 nanos)
native_duration([Str("abc")]) → Err
native_duration([Str("1x")]) → Err
native_duration([Int(1)]) → Err
native_duration([]) → Err
```

### `native_version`

```
native_version([Str("1.2.3")]) → Ok(Version(1,2,3))
native_version([Str("1.2.3.4.5")]) → Ok(Version(1,2,3,4,5))
native_version([Str("1.2.3-alpha.1")]) → Err  (P684: sem pre textual)
native_version([Str("1.2.3+build.2")]) → Err  (P684: sem build textual)
native_version([Str("invalid")]) → Err
native_version([Int(1)]) → Ok(Version(1))  (P684: qualquer número de componentes)
native_version([]) → Ok(Version())  (P684: zero componentes)
native_version(positional: [Int(1), Int(2), Int(3)]) → Ok(Version(1,2,3))
native_version(positional: [Int(1), Int(2), Int(3), Int(4), Int(5)]) → Ok(Version(1,2,3,4,5))
native_version(positional: [Int(1), Int(2), Int(3), Str("alpha.1")]) → Err  (P684: 4º posicional tem de ser Int)
native_version(positional: [Int(1), Int(2), Int(3)], named: {pre: Str("x")}) → Err  (P684: sem named args)
native_version(positional: [Array([Int(1), Int(2), Int(3)])]) → Ok(Version(1,2,3))  (P682: array ≡ posicional)
native_version(positional: [Array([Int(1), Int(2), Int(3), Int(4), Int(5)])]) → Ok(Version(1,2,3,4,5))
native_version(positional: [Array([Int(1), Str("x"), Int(3)])]) → Err  (elemento não-Int)
native_version(positional: [Array([Int(1), Int(2), Int(3)])], named: {pre: Str("x")}) → Err  (sem named args)
native_version(positional: [Int(-1), Int(2), Int(3)]) → Err
native_version(positional: [Str("1.2.3")], named: {pre: Str("alpha")}) → Err (mistura de formas)
```

### Operações eval Version

```
version(1,2,3) == version(1,2,3) → true
version(1,2,3) == version(1,2,4) → false
version(1,2,3) == version(1,2,3,0) → true   (P684: zero-pad)
version(1,2,3) == version(1,2,3,4) → false
version(1,2,3) < version(1,2,4) → true
version(1,2,3) < version(1,2,3,4) → true   (prefixo < mais longo)
version(1,2,3,0) < version(1,2,4) → true
version(1,2,4) > version(1,2,3) → true
version(1,2,3) <= version(1,2,3) → true
version(1,2,3,0) >= version(1,2,3) → true   (P684: zero-pad)
```

### `native_duration` named args

```
native_duration(named: {}) → Duration::ZERO
native_duration(named: {seconds: 90}) → 90s
native_duration(named: {days: 1, hours: 2, minutes: 3}) → 1d2h3m
native_duration(named: {seconds: -1}) → Err
native_duration(named: {seconds: Float(1.5)}) → Err
native_duration(positional: [Str("1h30m")], named: {seconds: 1}) → Err
```

### Operações eval Duration

```
90s + 30s → 120s
120s - 30s → 90s
30s - 120s → Err (underflow)
60s * 2 → 120s
60s * -1 → Err
60s * 1.5 → 90s
120s / 2 → 60s
120s / 0 → Err
120s / 2.0 → 60s
120s / 60s → Float(2.0)
59s < 60s → true
61s > 60s → true
60s == 60s → true
```

### Round-trip `repr`

- `repr(decimal("1.5"))` → `"1.5"` (depende de `Value::repr` para `Decimal`).
- `repr(duration("3d2h30m15.5s"))` → `"3d2h30m15.5s"`.
- `repr(version((1, 2, 3, 4, 5)))` → `"version(1, 2, 3, 4, 5)"`.

---

## 9. Scope-out

- Não criar novos variants de `Value`.
- Não tocar em `entities/`.
- Não adicionar operações aritméticas para `Decimal`/`Version`.
- Não adicionar field access além de `.major`/`.minor`/`.patch` (P411/P684); `.pre`/`.build` não existem no Typst.
- Não implementar `.at(index)`, bump, `.in(unit)`, `.display()`, extractores de componente.
- Não implementar cast `Duration → Int/Float` nem `Version → Str`.
- Não aceitar argumentos nomeados para `decimal`.
