# Passo 208C — `native_locate(selector)` com `Kind` apenas

**Série**: 208 (sub-passo `C`).
**Marco**: M9c (Bloco IV — `locate()`).
**Tipo**: implementação stdlib trivial.
**Magnitude**: S (~30min-1h).
**Pré-condição**: P208B concluído; `native_here()`
materializado; `EvalContext.current_location` exposto;
trait 26 métodos; `Introspector::query(Selector)`
(P175); `Value::Location` (P179); tests 1903 verdes;
0 violations; ADR-0076 PROPOSTO anotado §P208B.
**Output**: 1 ficheiro (relatório curto).

---

## §1 Trabalho

Materializar `native_locate(selector) -> Value::Location`
stdlib func. Reusa `Introspector::query` (P175) +
`Value::Location` (P179). Limitação herdada (per P208A
C2): `Selector` cristalino só tem `Kind` variant —
`locate("kind-as-string")` funciona; `locate(<label>)`
exige P209 `Selector::Label`.

Reuso de dados P208A + P208B:

- Pattern stdlib uniforme `native_X(ctx, args, world,
  current_file, figure_numbering)`.
- `native_query` em
  `01_core/src/rules/stdlib/foundations.rs` (per P208A
  A2 + P208B implementation pattern) — parsea
  `selector` arg + invoca `ctx.introspector.query(&selector)`.
- `native_here` registado no scope global (P208B C3
  pattern).
- Selector cristalino: `Kind(ElementKind)` only.

---

## §2 Cláusulas (4)

### C1 — Verificação curta de pré-condições

Antes de tocar código:

1. **`native_query` empírico**: confirmar que existe em
   `01_core/src/rules/stdlib/foundations.rs` (per P208A
   A2) e tem o pattern de parsing de selector arg
   que pode ser reusado.
2. **Pattern de parsing selector arg em cristalino**:
   identificar literalmente onde args
   "heading"/"figure"/etc. são convertidos a
   `Selector::Kind(ElementKind)`. Esperado:
   `ElementKind::from_name` (per P207B usage).
3. **Selector arg type**: vanilla aceita `selector(...)`
   ou string directa. Cristalino actual: confirmar
   qual é o input format aceite por `native_query`.

Se C1.1 ou C1.2 mostrarem que pattern não existe ou é
significativamente diferente do esperado, registar
`P208C.div-N` e fixar fallback.

### C2 — Materializar `native_locate`

Reusa pattern literal de `native_query`:

L1 — `01_core/src/rules/stdlib/foundations.rs`:

```text
pub fn native_locate(
    ctx:              &mut EvalContext,
    args:             &Args,
    _world:           &dyn World,
    _current_file:    FileId,
    _figure_numbering: Option<&str>,
) -> SourceResult<Value> {
    let selector = /* reusa parsing de native_query */;
    let first = ctx.introspector.query(&selector).first().copied();
    Ok(match first {
        Some(loc) => Value::Location(loc),
        None      => Value::None,
    })
}
```

L1 — `01_core/src/rules/stdlib/mod.rs`:
- `pub use native_locate` em block existente.

L1 — scope global da stdlib (mesmo ponto que P208B
registou `here`):
- `scope.define("locate", Value::Func(Func::native("locate", native_locate)));`

L0 — seguir convenção emergente P208B §3: stdlib funcs
P169+ inline-documentadas em código, sem L0 separado.
**Aplicar mesma convenção a `locate`** (não criar L0
novo).

### C3 — Tests

Tests dedicados (~3-5):

- `p208c_locate_kind_existente_retorna_some` —
  `locate("heading")` em corpus com headings retorna
  `Value::Location(loc)`.
- `p208c_locate_kind_inexistente_retorna_none` —
  `locate("figure")` em corpus sem figures retorna
  `Value::None`.
- `p208c_locate_kind_invalido_retorna_err` —
  `locate("nonexistent")` retorna `SourceResult::Err`
  com mensagem coerente.
- `p208c_locate_label_args_pendente_p209` —
  `locate(<label>)` retorna erro coerente documentando
  pendência P209. Test sentinela.

### C4 — Verificação final

```
cargo test --workspace 2>&1 | tail -10
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério: 1906+ verdes (1903 + 3+); 0 violations.

**Regra empírica P207B §5 não accionada** — `locate()`
é stdlib func, não trait method. Trait mantém 26
métodos.

Anotar ADR-0076 §P208C: `✅ MATERIALIZADO {data}`.

---

## §3 Output

1 ficheiro:
`00_nucleo/materialization/typst-passo-208C-relatorio.md`.

Estrutura (~3-5 KB) com 6 §s padrão (P207B/C/D format).

---

## §4 Não-objectivos

- `Selector::Label` (P209).
- `Selector::And`/`Or`/`Regex`/`Location` (P209+).
- `Selector::Before/After` (não no roadmap M9c; per
  P207A Q-decisions Q3=α explicita Regex+Location;
  Before/After fora).
- Trait method extensions.
- Propagação a `CountingIntrospector`.
- Page-aware captura (P207E deferred).
- L0 prompt novo para `locate.md` (convenção emergente).

---

## §5 Riscos a evitar

1. **Inflar parsing de selector**: reusa literal o
   pattern de `native_query`. Não inventar novo
   parsing; copiar.
2. **Esquecer registo no scope global**: stdlib func
   sem registo não invocável. C2 nota explicitamente o
   ponto.
3. **Test `locate_label_args_pendente_p209`** — confirmar
   que test produz erro **antes** de P209 materializar
   `Selector::Label`. Após P209, este test torna-se
   regressão potencial — documentar como sentinela
   temporária a remover/actualizar em P209.
4. **Convenção L0 stdlib funcs** — manter consistência
   com P208B §3 (P169+ inline-documentadas). Se humano
   pediu correção em P208B follow-up, esta decisão
   muda; mas pré-condição mantém convenção emergente.

---

## §6 Composição com `here()`

Per P208B paridade vanilla, `locate(...)` pode ser
combinado com `here()` em consumers futuros (ex:
`Selector::Before/After` que delegam a `here()`). P208C
não materializa essa composição — apenas `locate(kind)`
standalone.

Test composição (`p208c_locate_apos_here_basico`) é
**opcional** e adicionado apenas se trivial; caso
contrário fica para P209+.
