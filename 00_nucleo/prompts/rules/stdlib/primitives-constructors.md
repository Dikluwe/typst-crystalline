# Prompt L0 — `stdlib/primitives-constructors` — constructors `decimal`, `duration`, `version`
Hash do Código: e8068caa

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/stdlib/primitives_constructors.rs`
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
| `native_version` | `version("1.2.3-alpha")` / `version(1, 2, 3, pre: "alpha")` | `Value::Version` | semver string ou major/minor/patch + pre/build |

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

- `days`, `hours`, `minutes`, `seconds`, `milliseconds`, `microseconds`, `nanoseconds`

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

- Delega parse a `Version::from_str(&s)` (já exposto em `entities::version`).
- Sucesso: `Value::Version(Arc::new(version))`.
- Erro: `"version(): string inválida: '{s}'"`.

### Forma vanilla (positional + named pre/build)

```typst
version(1, 2, 3)
version(1, 2, 3, "alpha.1")
version(1, 2, 3, pre: "alpha.1", build: "build.2")
```

Argumentos:

- `major`, `minor`, `patch`: 3 posicionais obrigatórios do tipo `Int`, ≥ 0.
- `pre`: named arg opcional `Str`, default `""`. Parse `"a.b.c"` → vec!["a", "b", "c"]; `""` → vec vazio.
- `build`: named arg opcional `Str`, default `""`. Parse idem.

Validação:

- major/minor/patch negativos → erro eval.
- Tipo errado em qualquer arg → erro eval.
- Mistura da forma string com a forma vanilla → erro eval.

Constrói `Version::new(major, minor, patch).with_pre(pre_ids).with_build(build_ids)`.

### Forma array (P682)

Um único argumento posicional do tipo `array` equivale aos componentes
posicionais — medido contra o vanilla `0.15.0 (969087ec)`:

```typst
version((0, 2, 2))   ≡ version(0, 2, 2)
```

- Fiel ao vanilla `0.15.0`: o array tem de ter **exactamente 3 inteiros ≥ 0**
  (`major, minor, patch`), **sem argumentos nomeados** e **sem string** dentro
  do array.
- Erros claros: `arr.len() != 3` → "requer exactamente 3 inteiros … recebeu N";
  elemento não-`Int` ou negativo → erro de tipo/sinal (via `as_nonneg_int`);
  named args presentes → "a forma de array não aceita argumentos nomeados".
- Confirmado por sonda que o vanilla `0.15.0` **rejeita** `version((0,2,2,"beta"))`
  ("expected integer"), `version(0,2,2,"beta")`, `version(0,2,2, pre:"beta")` e
  `version((0,2,2), pre:"beta")`. O suporte a `pre`/`build` (4º posicional e
  named) **existe só na forma posicional** do cristalino (pré-existente, fora do
  scope de P682) e **não** se aplica à forma de array — ver débito registado no
  diagnóstico de P682.

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
| `<`, `>`, `<=`, `>=` | Version vs Version | Bool | via `Ord` (semver; build metadata ignorado) |

`Eq`/`Neq` já funcionam via wildcard `(BinOp::Eq, a, b) => Ok(Value::Bool(a == b))`, mas os braços explícitos garantem prioridade e clareza. Não implementar `+`, `-`, `*`, `/` nem comparações com Int/Str.

## 7. Paridade vanilla

A paridade é com a **forma da linguagem**: uma função global que converte string no tipo. A mecânica interna (parser hand-rolled vs. biblioteca) é detalhe de implementação cristalino.

- `decimal("1.5")` ≡ literal decimal `1.5`.
- `duration("1h30m")` ≡ `5400s`.
- `duration(seconds: 90) + duration(seconds: 30)` ≡ `120s`.
- `duration(minutes: 2) > duration(seconds: 119)` ≡ `true`.
- `version("1.2.3")` compara via semver (operadores futuros; constructor só constrói).
- `version(1, 2, 3) == version(1, 2, 3)` ≡ `true`.
- `version(1, 2, 3, "alpha") < version(1, 2, 3)` ≡ `true` (prerelease < release).
- `version(1, 2, 3, build: "a") == version(1, 2, 3, build: "b")` ≡ `true` (build ignorado).

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
native_version([Str("1.2.3")]) → Ok(Value::Version(1,2,3,"",""))
native_version([Str("1.2.3-alpha.1")]) → Ok(pre=["alpha","1"])
native_version([Str("1.2.3+build.2")]) → Ok(build=["build","2"])
native_version([Str("invalid")]) → Err
native_version([Int(1)]) → Err
native_version([]) → Err
native_version(positional: [Int(1), Int(2), Int(3)]) → Ok(Value::Version(1,2,3,"",""))
native_version(positional: [Array([Int(1), Int(2), Int(3)])]) → Ok(Value::Version(1,2,3,"",""))  (P682: forma array ≡ posicional)
native_version(positional: [Array([Int(1), Int(2)])]) → Err  (P682: array com !=3 componentes)
native_version(positional: [Array([Int(1), Int(2), Int(3), Int(4)])]) → Err  (P682: array com !=3 componentes)
native_version(positional: [Array([Int(1), Str("x"), Int(3)])]) → Err  (P682: elemento não-Int)
native_version(positional: [Array([Int(1), Int(2), Int(3)])], named: {pre: Str("x")}) → Err  (P682: array não aceita named args)
native_version(positional: [Int(1), Int(2), Int(3)], named: {pre: Str("alpha.1")}) → Ok(pre=["alpha","1"])
native_version(positional: [Int(1), Int(2), Int(3)], named: {build: Str("build.2")}) → Ok(build=["build","2"])
native_version(positional: [Int(-1), Int(2), Int(3)]) → Err
native_version(positional: [Str("1.2.3")], named: {pre: Str("alpha")}) → Err (mistura de formas)
```

### Operações eval Version

```
version(1,2,3) == version(1,2,3) → true
version(1,2,3) == version(1,2,4) → false
version(1,2,3) == version(1,2,3,"alpha") → false
version(1,2,3, build:"a") == version(1,2,3, build:"b") → true
version(1,2,3) < version(1,2,4) → true
version(1,2,3,"alpha") < version(1,2,3) → true
version(1,2,3,"alpha.1") < version(1,2,3,"alpha.2") → true
version(1,2,3,"alpha") < version(1,2,3,"beta") → true
version(1,2,4) > version(1,2,3) → true
version(1,2,3) <= version(1,2,3) → true
version(1,2,3) >= version(1,2,3,"alpha") → true
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
- `repr(version("1.2.3-alpha.1+build.2"))` → `"1.2.3-alpha.1+build.2"`.

---

## 9. Scope-out

- Não criar novos variants de `Value`.
- Não tocar em `entities/`.
- Não adicionar operações aritméticas para `Decimal`/`Version`.
- Não adicionar field access (`.major`, `.minor`, `.patch`, `.pre`, `.build`, `.hours()`, `.seconds()`, etc.).
- Não implementar `.at(index)`, bump, `.in(unit)`, `.display()`, extractores de componente.
- Não implementar cast `Duration → Int/Float` nem `Version → Str`.
- Não aceitar argumentos nomeados para `decimal`.
