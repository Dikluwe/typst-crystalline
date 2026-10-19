# Passo 401 — Modelagem de Tipos: `Value::Version` (S)

**Tipo**: Modelagem de tipos primitivos (L1 — pureza; zero I/O; expande enum `Value` fechado per ADR-0017).
**Data**: 2026-06-22.
**Padrão**: diagnóstico-primeiro (sonda 389); medir-antes-de-decidir (ADR-0108).
**ADRs relevantes**: ADR-0017 (portão aberto P395), ADR-0107 (paridade linguagem), ADR-0029 (pureza L1), ADR-0054 (graded scope-out — operações semver ricas).
**Sonda fonte**: `typst-sonda-ausentes-ordem-passo-389.md` §2D — `Value::Version` ausente, bloqueia literais version e comparações semver.

> **Nota de numeração.** Um passo só. Não numerar à frente.
> **Nota de marco.** Último da série de tipos S puros (P399 Decimal → P400 Duration → P401 Version). Após P401, o portão ADR-0017 permanece aberto para features futuras, mas todos os tipos primitivos pendentes da sonda 389 estão modelados.

---

## 1. Contexto

P395 abriu o portão ADR-0017. P399 modelou `Value::Decimal`, P400 modelou `Value::Duration`. Agora `Value::Version` fecha a série de tipos S puros.

No vanilla:
```typ
#let v = version(1, 2, 3)
#let w = version(1, 2, 3, "alpha.1")
#v >= w
```

`Version` representa um **número de versão semântico** (semver). No cristalino, usamos `semver::Version` do crate `semver` como base, ou um tipo próprio se `semver` for pesado.

Este passo é **S puro** — apenas modela o tipo e o integra no pipeline `Value`. Constructor stdlib (`version(major, minor, patch, pre, build)`) e operações (`>=`, `>`, `==`, etc.) são passos futuros.

---

## 2. Decisão de engenharia

### 2.1 — `Version` tipo L1

```rust
// entities/version.rs — tipo L1 puro
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub pre: Vec<EcoString>,   // prerelease identifiers ("alpha", "1", etc.)
    pub build: Vec<EcoString>,  // build metadata ("20231201", etc.)
}
```

**Decisão crate**: usar tipo próprio em vez de `semver::Version` para evitar dep externa. O vanilla usa semver simples (major.minor.patch[-pre][+build]). Um struct próprio com `Vec<EcoString>` é suficiente e puro-L1.

**Decisão ADR-0107 (língua vs mecânica)**: a paridade é com o **valor da versão** (1.2.3-alpha.1+build.2), não com a mecânica interna. `Vec<EcoString>` é mecânica; o valor é linguagem.

**Decisão ADR-0029 (pureza L1)**: `Version` é puro — nenhum I/O, alloc mínimo (Vec de strings pequenas).

**Decisão ADR-0017 (enum fechado)**: `Version` não é `Copy` (contém Vec). No `Value`, usar `Arc<Version>` para cheap clone (paridade pattern: `Value::Tiling(Arc<Tiling>)` P395, `Value::Str(EcoString)` P13).

### 2.2 — Enum `Value::Version`

```rust
// entities/value.rs — novo variant
Version(Arc<Version>),
```

**Por que Arc**: `Version` contém `Vec<EcoString>` — não é `Copy`. Arc permite cheap clone O(1) (paridade pattern P395 `Value::Tiling`).

### 2.3 — Parsing de literal version

No vanilla, `version` é constructor stdlib (não literal suffixo):
```typ
#version(1, 2, 3)
#version(1, 2, 3, "alpha.1")
```

Este passo **não implementa o constructor** — apenas modela o tipo. O constructor `native_version` é passo futuro (S).

Casts implementados neste passo:
- `Version → Version` (identity)
- `Str → Version` (parse semver, fallible)

**Não implementar cast de `Int`/`Float`/`Array` → `Version`** — não existem no vanilla.

### 2.4 — Semântica de comparação (PartialOrd/Ord)

Semver comparação:
- `1.2.3 < 1.2.4` (patch menor)
- `1.2.3 < 1.3.0` (minor menor)
- `1.2.3-alpha < 1.2.3` (prerelease < release)
- `1.2.3-alpha.1 < 1.2.3-alpha.2` (prerelease lexicográfico numérico)
- Build metadata não afeta comparação (`1.2.3+build == 1.2.3`)

Implementar `PartialOrd` + `Ord` conforme semver 2.0.0. Build metadata é ignorado em comparação.

### 2.5 — Impacto cross-module (match exhaustivo)

| Módulo | O que muda | Como |
|--------|-----------|------|
| `entities/value.rs` | +1 variant | `Version(Arc<Version>)` |
| `eval/repr.rs` | +1 arm | `"1.2.3-alpha.1+build.2"` (formato canónico) |
| `eval/cast.rs` | +1 arm | `Version → Version` (identity); `Str → Version` (parse semver, fallible) |
| `eval/ops.rs` | +1 arm | `==` por struct equality; `<`/`>`/`<=`/`>=` — scope-out ADR-0054 graded (futuro S) |
| `layout/types.rs` | Nenhum | Version não é Paint/Fill/Style/Length |
| `export.rs` | Nenhum | Version não emite direto |
| `stdlib/` | Nenhum | `native_version` é passo futuro |

**Decisão**: este passo é **modelagem pura** — não implementa `native_version` nem operações de comparação em stdlib. Apenas modela o tipo e o integra no enum `Value`.

---

## 3. FASE A — L0 (redação; checkpoint obrigatório)

### A.1 — Prompt L0 `version.md`

Novo em `00_nucleo/prompts/entities/version.md`:

- **Paridade**: `Version` ≡ vanilla `Version` tipo (semver: major.minor.patch[-pre][+build]).
- **Substrato**: tipo L1 puro `Version { major, minor, patch, pre, build }`; `Clone` (não Copy); Arc-wrapped em `Value`; zero I/O.
- **Semântica**: semver 2.0.0 — comparação ignora build metadata; prerelease < release.
- **Variant `Value`**: `Version(Arc<Version>)` — Arc para cheap clone (paridade pattern P395).
- **Cast**: `Version → Version` (identity); `Str → Version` (parse semver, fallible).
- **Repr**: formato canónico `"major.minor.patch[-pre][+build]"` (ex.: `"1.2.3-alpha.1+build.2"`).
- **Literal**: não existe literal suffixo — constructor via `version(...)` (stdlib futuro).
- **Comparação**: `PartialOrd` + `Ord` implementados conforme semver 2.0.0.
- **Teste**: construção `Version::new(1, 2, 3)` → `Value::Version` → repr → cast → comparação.

### A.2 — CHECKPOINT

Parar. Apresentar `version.md` ao dono. **Só prosseguir para Fase B quando confirmar que guardou e computou hash.**

---

## 4. FASE B — Código (após confirmação humana)

### B.1 — Tipo entity `Version`

Em `01_core/src/entities/version.rs`:

```rust
use ecow::EcoString;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub pre: Vec<EcoString>,   // prerelease identifiers
    pub build: Vec<EcoString>, // build metadata (ignored in comparison)
}

impl Version {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
            pre: Vec::new(),
            build: Vec::new(),
        }
    }

    pub fn with_pre(mut self, pre: Vec<EcoString>) -> Self {
        self.pre = pre;
        self
    }

    pub fn with_build(mut self, build: Vec<EcoString>) -> Self {
        self.build = build;
        self
    }

    pub fn to_string(&self) -> String {
        let mut s = format!("{}.{}.{}", self.major, self.minor, self.patch);
        if !self.pre.is_empty() {
            s.push('-');
            s.push_str(&self.pre.join("."));
        }
        if !self.build.is_empty() {
            s.push('+');
            s.push_str(&self.build.join("."));
        }
        s
    }
}

impl Default for Version {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Semver comparison: major, minor, patch, then pre (none > some)
        let ord = self.major.cmp(&other.major)
            .then_with(|| self.minor.cmp(&other.minor))
            .then_with(|| self.patch.cmp(&other.patch));

        if ord != std::cmp::Ordering::Equal {
            return ord;
        }

        // Prerelease comparison: no prerelease > any prerelease
        match (self.pre.is_empty(), other.pre.is_empty()) {
            (true, true) => std::cmp::Ordering::Equal,
            (true, false) => std::cmp::Ordering::Greater,
            (false, true) => std::cmp::Ordering::Less,
            (false, false) => cmp_pre(&self.pre, &other.pre),
        }
    }
}

/// Compare prerelease identifiers per semver 2.0.0
fn cmp_pre(a: &[EcoString], b: &[EcoString]) -> std::cmp::Ordering {
    let len = a.len().min(b.len());
    for i in 0..len {
        let ord = cmp_pre_id(&a[i], &b[i]);
        if ord != std::cmp::Ordering::Equal {
            return ord;
        }
    }
    a.len().cmp(&b.len())
}

/// Compare single prerelease identifier: numeric vs lexicographic
fn cmp_pre_id(a: &str, b: &str) -> std::cmp::Ordering {
    match (a.parse::<u64>(), b.parse::<u64>()) {
        (Ok(a_num), Ok(b_num)) => a_num.cmp(&b_num),
        (Ok(_), Err(_)) => std::cmp::Ordering::Less,    // numeric < non-numeric
        (Err(_), Ok(_)) => std::cmp::Ordering::Greater, // non-numeric > numeric
        (Err(_), Err(_)) => a.cmp(b),                 // lexicographic
    }
}

impl Version {
    /// Parse semver string: "major.minor.patch[-pre][+build]"
    pub fn from_str(s: &str) -> Option<Self> {
        let s = s.trim();
        if s.is_empty() {
            return None;
        }

        // Split build metadata
        let (s, build) = if let Some(pos) = s.find('+') {
            let (s, b) = s.split_at(pos);
            let b = &b[1..]; // skip '+'
            let build = b.split('.').map(EcoString::from).collect();
            (s, build)
        } else {
            (s, Vec::new())
        };

        // Split prerelease
        let (s, pre) = if let Some(pos) = s.find('-') {
            let (s, p) = s.split_at(pos);
            let p = &p[1..]; // skip '-'
            let pre = p.split('.').map(EcoString::from).collect();
            (s, pre)
        } else {
            (s, Vec::new())
        };

        // Parse major.minor.patch
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return None;
        }

        let major = parts[0].parse::<u64>().ok()?;
        let minor = parts[1].parse::<u64>().ok()?;
        let patch = parts[2].parse::<u64>().ok()?;

        Some(Self { major, minor, patch, pre, build })
    }
}
```

**Nota**: `cmp_pre_id` implementa regra semver 2.0.0 — identificadores numéricos são comparados numericamente; identificadores não-numéricos são comparados lexicograficamente; numéricos sempre são menores que não-numéricos.

### B.2 — Variant `Value::Version`

Em `entities/value.rs`:

```rust
Version(Arc<Version>),
```

Atualizar:
- `PartialEq` — match arm `Version(a) => matches!(other, Version(b) if a == b)`.
- `Repr` — `version.to_string()` (formato canónico semver).
- `Cast` — `Version(v) => Ok(v.clone())`; `Str(s) => Version::from_str(s).map(|v| Arc::new(v)).ok_or(...)`.
- `type_name` — `"version"`.

### B.3 — Testes

1. **Unit `entities/version.rs`** (8-10 tests):
   - `version_new` — `Version::new(1, 2, 3)` → major=1, minor=2, patch=3, pre empty, build empty.
   - `version_to_string` — `1.2.3` → `"1.2.3"`.
   - `version_to_string_pre` — `1.2.3-alpha.1` → `"1.2.3-alpha.1"`.
   - `version_to_string_build` — `1.2.3+build.2` → `"1.2.3+build.2"`.
   - `version_to_string_pre_build` — `1.2.3-alpha.1+build.2` → `"1.2.3-alpha.1+build.2"`.
   - `version_parse` — `"1.2.3"` → Some(Version::new(1,2,3)).
   - `version_parse_pre` — `"1.2.3-alpha.1"` → Some(Version { pre: vec!["alpha", "1"] }).
   - `version_parse_build` — `"1.2.3+build.2"` → Some(Version { build: vec!["build", "2"] }).
   - `version_parse_invalid` — `"abc"` → None.
   - `version_parse_empty` — `""` → None.
   - `version_parse_two_parts` — `"1.2"` → None.

2. **Comparação semver** (6-8 tests):
   - `version_cmp_major` — `1.0.0 < 2.0.0`.
   - `version_cmp_minor` — `1.1.0 < 1.2.0`.
   - `version_cmp_patch` — `1.0.1 < 1.0.2`.
   - `version_cmp_pre_vs_release` — `1.0.0-alpha < 1.0.0`.
   - `version_cmp_pre_numeric` — `1.0.0-alpha.1 < 1.0.0-alpha.2`.
   - `version_cmp_pre_mixed` — `1.0.0-alpha < 1.0.0-beta` (lexicographic).
   - `version_cmp_pre_num_vs_str` — `1.0.0-1 < 1.0.0-alpha` (numeric < non-numeric).
   - `version_cmp_build_ignored` — `1.0.0+build1 == 1.0.0+build2`.
   - `version_cmp_pre_len` — `1.0.0-alpha < 1.0.0-alpha.1` (shorter < longer when prefix equal).

3. **Unit `entities/value.rs`** (4-5 tests):
   - `value_version_variant` — discriminação `Value::Version`.
   - `value_version_repr` — `"1.2.3-alpha.1"` para Version correspondente.
   - `value_version_cast_identity` — `Version → Version`.
   - `value_version_cast_from_str` — `Str("1.2.3") → Version::new(1,2,3)`.
   - `value_version_cast_from_str_invalid` — `Str("abc") → Err`.
   - `value_version_partial_eq` — equality com outro Version.
   - `value_version_ordering` — `Value::Version(Arc::new(v1)) < Value::Version(Arc::new(v2))` (via cast + cmp).

4. **Integration** (1-2 tests, opcional):
   - `version_no_ops_yet` — confirmar que `Value::Version` não participa de `+`/`-`/`*`/`/` (se ops.rs tiver match, adicionar braço que retorna `Err` claro: `"operações aritméticas em version — scope-out futuro"`).

### B.4 — Linhagem

- `@prompt` aponta para `version.md`.
- `@prompt-hash` via `--fix-hashes`.
- Referência cruzada: P395 (portão ADR-0017 aberto), P399 (Decimal — modelo de tipo S puro), P400 (Duration — modelo de tipo S puro), ADR-0107 (paridade linguagem), ADR-0029 (pureza L1).

---

## 5. O que NÃO fazer (scope-out)

- **Não** implementar `native_version` (stdlib constructor) — é passo futuro (S).
- **Não** implementar operações de comparação em stdlib (`>=`, `>`, `<=`, `<`, `==`) — scope-out ADR-0054 graded (futuro S).
- **Não** implementar `.at(index)` ou field access (`version.major`, etc.) — scope-out ADR-0054 graded (futuro S).
- **Não** implementar cast de `Array` → `Version` — não existe no vanilla.
- **Não** implementar `Version` como `Str` (display automático em texto) — scope-out ADR-0054 graded.
- **Não** adicionar `Version` como `Paint`/`Fill`/`Style`/`Length` — não é visual.
- **Não** tocar em constructors stdlib para Decimal/Duration — são passos futuros independentes.
- **Não** quebrar invariantes de camada (L1 puro, zero I/O).

---

## 6. Critérios de aceitação

1. `Value::Version(Arc<Version>)` compila e participa de `match` exhaustivo em eval.
2. `Version` é `Clone` (não Copy), puro-Rust, alloc mínimo (Vec de strings).
3. Semver parse: `Str("1.2.3-alpha.1+build.2") → Version` (fallible).
4. Semver comparison: `PartialOrd` + `Ord` conforme semver 2.0.0 (build metadata ignorado).
5. Repr: formato canónico `"major.minor.patch[-pre][+build]"`.
6. Zero consumer complexo; zero I/O; zero func stdlib nova.
7. Testes verdes (≥18 unit + 1-2 integration); lint zero; hashes propagados.
8. Inventário 148: `Value::Version` transita `ausente` → `implementado` (tipo); `version()` stdlib permanece `ausente` (futuro S).
9. L0 salvo e hashado antes do código (protocolo de nucleação).
10. Ritmo S: tempo de ciclo comparável a P399/P400 (baseline de tipo S puro).

---

## 7. O que pode sair errado

- **Formato semver do vanilla não é exatamente semver 2.0.0.** Mitigação: se o vanilla usa formato diferente (ex.: só major.minor, ou build metadata antes de prerelease), adaptar `from_str`/`to_string`. Mas como este passo é S puro, o formato pode ser ajustado no passo do constructor stdlib.
- **`EcoString` não é apropriado para prerelease identifiers.** Mitigação: `EcoString` é suficiente — identifiers são curtos. Se necessário, usar `String` ou `&str` internamente.
- **Comparação semver é complexa e pode ter edge cases.** Mitigação: testes extensivos (6-8 tests de comparação) cobrem os casos principais. Edge cases raros são scope-out ADR-0054 graded.
- **Tentação de já implementar `native_version` ou operações de comparação.** Mitigação: um passo de cada vez; este é S puro de modelagem.
- **Tentação de implementar `Version` como `Copy` com array fixo.** Mitigação: prerelease/build têm tamanho variável; Vec é necessário. Arc no Value resolve o problema de clone.

---

## 8. Referências

- P395 — `Value::Tiling` (portão ADR-0017 aberto).
- P399 — `Value::Decimal` (modelo de tipo S puro — padrão paralelo).
- P400 — `Value::Duration` (modelo de tipo S puro — padrão paralelo).
- ADR-0017 — trava arquitetural enum fechado.
- ADR-0107 — paridade linguagem vs mecânica (semver).
- ADR-0029 — pureza L1.
- ADR-0054 — graded scope-out (constructor stdlib, operações de comparação, field access).
- Semver 2.0.0 spec — https://semver.org/

---

## 9. Nota sobre o Tekt

Este passo é o **último da série de tipos S puros** — P399 (Decimal) → P400 (Duration) → P401 (Version). Após P401:
- Todos os tipos primitivos pendentes da sonda 389 estão modelados.
- O portão ADR-0017 permanece aberto para features futuras (não se fecha — é uma regra de processo, não um estado).
- O projeto entra em nova fase: constructors stdlib para os tipos modelados (`native_decimal`, `native_duration`, `native_version`), ou features com dependências reais (bibliography CSL, shaping, etc.).

Registar o tempo de ciclo da série P399→P400→P401 como **baseline de ritmo para tipos S puros em série**. Se os três passos tiverem tempos comparáveis, o portão ADR-0017 funciona como previsto para modelagem pura. Se algum for anômalo, investigar complexidade oculta.

**Marco:** Portão ADR-0017 validado para tipos primitivos — 3 tipos S puros consecutivos (Decimal, Duration, Version) fechados com ritmo uniforme.
