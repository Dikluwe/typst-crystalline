# Passo 406 — Activação de `Version`: constructor + comparações stdlib (S-M)

**Tipo**: Activação de tipo modelado (L1 — stdlib constructor + eval ops; zero I/O; zero tipo novo; reusa `Value::Version` P401).
**Data**: 2026-06-22.
**Padrão**: diagnóstico-primeiro (sonda 389 + P401); medir-antes-de-decidir (ADR-0108).
**ADRs relevantes**: ADR-0017 (portão aberto P395), ADR-0107 (paridade linguagem), ADR-0054 (graded scope-out — operações semver avançadas: `.at()`, `.pre`, `.build`, etc.).
**Sonda fonte**: `typst-sonda-ausentes-ordem-passo-389.md` §2D — `Value::Version` modelado em P401; `version()` stdlib + comparações stdlib ausentes.

> **Nota de numeração.** Um passo só. Não numerar à frente.
> **Nota de marco.** Último da série de activação de tipos S modelados: P404 Decimal → P405 Duration → P406 Version. Após P406, todos os tipos primitivos pendentes da sonda 389 estão **activos** (tipo + constructor + ops básicas).

---

## 1. Contexto

P401 modelou `Value::Version` (tipo L1 `Version { major, minor, patch, pre, build }` com semver parse/comparison). Este passo **activa o tipo** para uso user-facing: constructor stdlib + comparações stdlib.

No vanilla:
```typ
#let v = version(1, 2, 3)
#let w = version(1, 2, 3, "alpha.1")
#let x = version(1, 2, 3, pre: "alpha.1", build: "build.2")
#v >= w
#v == version(1, 2, 3)
```

Operações básicas: `==`, `!=`, `<`, `>`, `<=`, `>=` (Version vs Version).
Operações avançadas (scope-out): `.at(index)` (major/minor/patch), `.pre` (prerelease), `.build` (build metadata), bump, etc.

---

## 2. Decisão de engenharia

### 2.1 — `native_version` stdlib

```rust
// signature vanilla: version(major, minor, patch, pre, build)
// major, minor, patch: Int (obrigatórios, >= 0)
// pre: Str (opcional, default "")
// build: Str (opcional, default "")
// pre/build parse: "a.b.c" → vec!["a", "b", "c"]

native_version(
    major: Int,      // obrigatório, >= 0
    minor: Int,      // obrigatório, >= 0
    patch: Int,      // obrigatório, >= 0
    pre: Option<Str>, // default ""
    build: Option<Str>, // default ""
) -> Value::Version
```

**Decisão ADR-0107**: paridade com a **forma** do vanilla — `version(1, 2, 3)` ou `version(1, 2, 3, "alpha.1")` constrói um semver. A mecânica interna (Vec<EcoString>) é invisível.

**Validação**: major/minor/patch devem ser `Int` não-negativos. Negativo → `Err`. Overflow de u64 → `Err` (saturar ou rejeitar? Decisão: rejeitar com mensagem clara).

**Parsing de pre/build**:
```rust
fn parse_identifiers(s: &str) -> Vec<EcoString> {
    if s.is_empty() {
        Vec::new()
    } else {
        s.split('.').map(EcoString::from).collect()
    }
}
```

**Decisão**: strings vazias para `pre`/`build` → Vec vazio (sem prerelease/build). `"alpha.1"` → vec!["alpha", "1"]. `""` → vec![].

### 2.2 — Operações eval básicas

| Operador | Operandos | Resultado | Nota |
|----------|-----------|-----------|------|
| `==`, `!=` | Version vs Version | Bool | via PartialEq (struct: major, minor, patch, pre, build) |
| `<`, `>`, `<=`, `>=` | Version vs Version | Bool | via Ord (semver 2.0.0: major, minor, patch, pre; build ignorado) |

**Não implementar**: `+`, `-`, `*`, `/` — não existem no vanilla para Version.
**Não implementar**: Version vs Int/Str comparações — não existem no vanilla.

**Decisão**: `PartialEq` em `Version` (P401) já compara todos os campos (major, minor, patch, pre, build). `Ord` em `Version` (P401) já ignora build metadata. As operações eval delegam para estes traits.

### 2.3 — Cast adicional

P401 implementou cast: `Version → Version`, `Str → Version` (parse semver, fallible).

Este passo não adiciona novos casts — `Str → Version` já existe. `Version → Str` (display) pode ser adicionado como cast implícito se o vanilla suportar; se não, scope-out.

**Simplificação**: não adicionar cast novo. Focar em constructor + comparações.

---

## 3. FASE A — L0 (redação; checkpoint obrigatório)

### A.1 — Prompt L0 `version-stdlib.md`

Extensão de `00_nucleo/prompts/entities/version.md` (ou novo `00_nucleo/prompts/rules/stdlib/version-stdlib.md`):

- **Paridade**: `version(1, 2, 3)` ≡ vanilla constructor; `==`/`<`/`>` ≡ vanilla comparações semver.
- **Substrato**: reusa `Version { major, minor, patch, pre, build }` P401; stdlib constructor + eval ops.
- **Sem tipo novo**: reusa `Value::Version` P401.
- **Constructor**: `version(major, minor, patch, pre: "", build: "")` — major/minor/patch `Int` obrigatórios ≥0; pre/build `Str` opcionais.
- **Pre/build parse**: `"a.b.c"` → vec!["a", "b", "c"]; `""` → vec vazio.
- **Ops básicas**: `==`, `!=`, `<`, `>`, `<=`, `>=` (Version vs Version).
- **Erros**: major/minor/patch negativo → Err; overflow u64 → Err; pre/build não-string → Err.
- **Scope-out**: `.at(index)` (major/minor/patch), `.pre`, `.build`, bump, etc. — operações avançadas ADR-0054 graded.
- **Teste**: `version(1, 2, 3) == version(1, 2, 3)` → true; `version(1, 2, 3) < version(1, 2, 4)` → true; `version(1, 2, 3, "alpha") < version(1, 2, 3)` → true.

### A.2 — CHECKPOINT

Parar. Apresentar extensão `version.md` (ou `version-stdlib.md`) ao dono. **Só prosseguir para Fase B quando confirmar que guardou e computou hash.**

---

## 4. FASE B — Código (após confirmação humana)

### B.1 — `native_version` stdlib

Em `rules/stdlib/foundations.rs` (ou módulo version apropriado; confirmar path):

```rust
pub fn native_version(args: &Args) -> SourceResult<Value> {
    let major = args.expect::<Int>("major")?;
    let minor = args.expect::<Int>("minor")?;
    let patch = args.expect::<Int>("patch")?;
    let pre = args.find::<Str>("pre").unwrap_or_default();
    let build = args.find::<Str>("build").unwrap_or_default();

    // Validar não-negativos
    if major < 0 || minor < 0 || patch < 0 {
        bail!("major, minor e patch devem ser não-negativos");
    }

    // Validar overflow u64
    let major_u = major as u64;
    let minor_u = minor as u64;
    let patch_u = patch as u64;

    let pre_ids = parse_identifiers(&pre);
    let build_ids = parse_identifiers(&build);

    let version = Version::new(major_u, minor_u, patch_u)
        .with_pre(pre_ids)
        .with_build(build_ids);

    Ok(Value::Version(Arc::new(version)))
}

fn parse_identifiers(s: &str) -> Vec<EcoString> {
    if s.is_empty() {
        Vec::new()
    } else {
        s.split('.').map(EcoString::from).collect()
    }
}
```

**Nota**: `Version::new` e `with_pre`/`with_build` já existem em P401. Se não existirem, adicionar.

### B.2 — Operações eval

Em `eval/ops.rs` (ou onde as comparações são definidas):

```rust
// Comparisons: reusa PartialEq/Ord de Version (P401)
// Se ops.rs usa match em Value, adicionar arms:

// ==
(Value::Version(a), Value::Version(b)) => Ok(Value::Bool(a == b)),

// !=
(Value::Version(a), Value::Version(b)) => Ok(Value::Bool(a != b)),

// <
(Value::Version(a), Value::Version(b)) => Ok(Value::Bool(a < b)),

// >
(Value::Version(a), Value::Version(b)) => Ok(Value::Bool(a > b)),

// <=
(Value::Version(a), Value::Version(b)) => Ok(Value::Bool(a <= b)),

// >=
(Value::Version(a), Value::Version(b)) => Ok(Value::Bool(a >= b)),
```

**Nota**: se `ops.rs` usa trait genérico para comparações (ex.: `ops::eq(a, b)`), verificar se `Value::Version` já participa via `PartialEq`/`PartialOrd`. Se sim, nenhuma mudança é necessária. Se não, adicionar arms.

**Verificação**: P401 implementou `PartialEq` + `Eq` + `PartialOrd` + `Ord` para `Version`. Se `ops.rs` delega para estes traits (via `==`/`!=`/`<`/`>` operators em `Value`), nenhuma mudança é necessária. Verificar implementação de `ops.rs`.

**Se `ops.rs` usa match explícito em `Value`**: adicionar os 6 arms acima.

**Se `ops.rs` usa trait genérico com `where T: PartialEq`**: `Value::Version(Arc<Version>)` já é `PartialEq` (via `Arc` que delega para `Version::PartialEq`). Nenhuma mudança.

### B.3 — Registar em `make_stdlib`

```rust
scope.define("version", Func::native(native_version));
```

### B.4 — Testes

1. **Unit stdlib** (8-10 tests em `stdlib/mod.rs` ou `foundations.rs`):
   - `native_version_basic` — `version(1, 2, 3)` → major=1, minor=2, patch=3, pre empty, build empty.
   - `native_version_pre` — `version(1, 2, 3, "alpha.1")` → pre=["alpha", "1"].
   - `native_version_build` — `version(1, 2, 3, build: "build.2")` → build=["build", "2"].
   - `native_version_pre_build` — `version(1, 2, 3, "alpha.1", "build.2")` → pre=["alpha", "1"], build=["build", "2"].
   - `native_version_empty_pre_build` — `version(1, 2, 3)` → pre=[], build=[].
   - `native_version_negative_major` — `version(-1, 2, 3)` → Err.
   - `native_version_negative_minor` — `version(1, -2, 3)` → Err.
   - `native_version_negative_patch` — `version(1, 2, -3)` → Err.
   - `native_version_zero` — `version(0, 0, 0)` → valid.
   - `native_version_large` — `version(999999, 999999, 999999)` → valid (dentro de u64).

2. **Unit eval ops** (6-8 tests em `eval/ops.rs` ou `eval/tests.rs`):
   - `version_eq` — `version(1, 2, 3) == version(1, 2, 3)` → true.
   - `version_eq_diff` — `version(1, 2, 3) == version(1, 2, 4)` → false.
   - `version_eq_pre` — `version(1, 2, 3) == version(1, 2, 3, "alpha")` → false (pre diferente).
   - `version_eq_build` — `version(1, 2, 3, build: "a") == version(1, 2, 3, build: "b")` → true (build ignorado).
   - `version_lt_major` — `version(1, 2, 3) < version(2, 0, 0)` → true.
   - `version_lt_minor` — `version(1, 2, 3) < version(1, 3, 0)` → true.
   - `version_lt_patch` — `version(1, 2, 3) < version(1, 2, 4)` → true.
   - `version_lt_pre_vs_release` — `version(1, 2, 3, "alpha") < version(1, 2, 3)` → true.
   - `version_lt_pre_numeric` — `version(1, 2, 3, "alpha.1") < version(1, 2, 3, "alpha.2")` → true.
   - `version_lt_pre_mixed` — `version(1, 2, 3, "alpha") < version(1, 2, 3, "beta")` → true.
   - `version_gt` — `version(1, 2, 4) > version(1, 2, 3)` → true.
   - `version_le` — `version(1, 2, 3) <= version(1, 2, 3)` → true.
   - `version_ge` — `version(1, 2, 3) >= version(1, 2, 3, "alpha")` → true.

3. **Integration / E2E** (2-3 tests):
   - `version_pipeline` — parse + eval + `version(1, 2, 3) == version(1, 2, 3)` → true.
   - `version_comparison_pipeline` — `version(1, 0, 0) < version(2, 0, 0)` → true.
   - `version_repr` — `repr(version(1, 2, 3, "alpha.1"))` → `"1.2.3-alpha.1"`.
   - `version_str_cast` — `Str("1.2.3-alpha.1").cast::<Version>()` → Ok (se cast existe P401).

### B.5 — Linhagem

- `@prompt` aponta para extensão `version.md` (ou `version-stdlib.md`).
- `@prompt-hash` via `--fix-hashes`.
- Referência cruzada: P401 (Version tipo modelado), P404 (Decimal aritmética), P405 (Duration constructor + ops), ADR-0107 (paridade linguagem), ADR-0054 (graded scope-out operações avançadas).

---

## 5. O que NÃO fazer (scope-out)

- **Não** implementar `.at(index)` — scope-out ADR-0054 graded (acesso a major/minor/patch).
- **Não** implementar `.pre` ou `.build` — scope-out ADR-0054 graded (acesso a prerelease/build metadata).
- **Não** implementar bump (`.major()`, `.minor()`, `.patch()`) — scope-out ADR-0054 graded.
- **Não** implementar `Version → Str` cast — scope-out ADR-0054 graded (futuro XS; `repr` já funciona).
- **Não** implementar `+`/`-`/`*`/`/` — não existem no vanilla.
- **Não** implementar comparações Version vs Int/Str — não existem no vanilla.
- **Não** adicionar tipo novo — reusa P401.
- **Não** quebrar invariantes de camada (L1 puro, zero I/O).

---

## 6. Critérios de aceitação

1. `version(1, 2, 3)` compila e retorna `Value::Version`.
2. `version(1, 2, 3, "alpha.1")` → pre=["alpha", "1"].
3. `version(1, 2, 3, build: "build.2")` → build=["build", "2"].
4. `version(1, 2, 3) == version(1, 2, 3)` → `Value::Bool(true)`.
5. `version(1, 2, 3, "alpha") < version(1, 2, 3)` → `Value::Bool(true)` (prerelease < release).
6. `version(1, 2, 3, build: "a") == version(1, 2, 3, build: "b")` → `Value::Bool(true)` (build ignorado).
7. Negativo em major/minor/patch → `Err`.
8. Zero tipo novo; zero I/O; zero variant novo.
9. Testes verdes (≥18 unit + 2-3 integration); lint zero; hashes propagados.
10. Inventário 148: `version()` stdlib transita `ausente` → `implementado`; `Value::Version` permanece `implementado` (tipo já modelado P401).
11. L0 salvo e hashado antes do código (protocolo de nucleação).
12. Ritmo S-M: tempo de ciclo comparável a P404/P405 (baseline de activação de tipo S modelado).

---

## 7. O que pode sair errado

- **`Version::new`/`with_pre`/`with_build` não existem em P401.** Mitigação: verificar P401. Se não existirem, adicionar como parte deste passo (são trivial, parte do tipo).
- **`ops.rs` já delega comparações para `PartialEq`/`PartialOrd` de `Value` (via derive ou impl genérico).** Mitigação: se `Value::Version(Arc<Version>)` já é comparável via `Arc` que delega para `Version`, nenhuma mudança em `ops.rs` é necessária. Verificar se `ops.rs` tem match explícito ou trait genérico.
- **`args.expect::<Int>` retorna `i64` — verificar se `Int` em cristalino é `i64` ou `i32`.** Mitigação: adaptar cast. Se `Int` é `i64`, `as u64` é seguro após check `>= 0`.
- **Testes de `version(0, 0, 0)` — é valido?** Mitigação: sim, semver permite `0.0.0`. Verificar se `Version::new` aceita zeros.
- **Tentação de já implementar `.at()` ou `.pre`.** Mitigação: ADR-0054 graded; comparações são suficientes para S-M.
- **Tentação de implementar `Version` em `Paint`/`Fill`/`Style`/`Length`.** Mitigação: não é visual.

---

## 8. Referências

- P401 — `Value::Version` tipo modelado (baseline).
- P404 — `Decimal` aritmética (padrão paralelo de activação de tipo S modelado).
- P405 — `Duration` constructor + ops (padrão paralelo de activação de tipo S modelado).
- P395 — `Value::Tiling` (portão ADR-0017 aberto).
- ADR-0107 — paridade linguagem vs mecânica (constructor + comparações).
- ADR-0054 — graded scope-out (`.at()`, `.pre`, `.build`, bump, etc.).

---

## 9. Nota sobre o Tekt

Este passo é o **último da série de activação de tipos S modelados** — P404 Decimal → P405 Duration → P406 Version. Após P406:
- Todos os tipos primitivos pendentes da sonda 389 estão **activos** (tipo + constructor + ops básicas).
- O portão ADR-0017 permanece aberto para features futuras, mas não há mais tipos primitivos pendentes.
- O projeto entra em nova fase: features com dependências reais (bibliography CSL, shaping, etc.) ou refinamentos de features existentes.

Registar o tempo de ciclo da série P404→P405→P406 como **baseline de activação de tipo S modelado** — comparar com a série de modelagem P399→P400→P401. Se a activação for consistentemente mais lenta que a modelagem, investigar: constructors + ops eval são mais complexos que modelagem pura?
