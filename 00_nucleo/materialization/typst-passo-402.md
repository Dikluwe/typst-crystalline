# Passo 402 — Modelagem de Tipos: `Value::Regex` (S)

**Tipo**: Modelagem de tipos primitivos (L1 — pureza; zero I/O; expande enum `Value` fechado per ADR-0017).
**Data**: 2026-06-22.
**Padrão**: diagnóstico-primeiro (sonda 389 + DEBT-52); medir-antes-de-decidir (ADR-0108).
**ADRs relevantes**: ADR-0017 (portão aberto P395), ADR-0107 (paridade linguagem), ADR-0029 (pureza L1), ADR-0054 (graded scope-out — operações regex ricas como match, replace, etc.).
**Sonda fonte**: `typst-sonda-ausentes-ordem-passo-389.md` §2D — `Value::Regex` ausente; Tabela C linha 376 `text.font` dict bloqueado por regex ausente em L1 (DEBT-52).

> **Nota de numeração.** Um passo só. Não numerar à frente.
> **Nota de marco.** Este passo é **fora da série de tipos S puros** (P399-P401) — é tipo primitivo, mas com consumer já existente (`Selector::Regex` P393). Destrava 2 features: `text.font` dict (DEBT-52) e refinamentos de `#show regex`.

---

## 1. Contexto

P393 implementou `#show regex(...)` com `Selector::Regex(Regex)` e `Value::Regex(Regex)` em `entities/value.rs`. No entanto, a sonda 389 e DEBT-52 identificaram que `Value::Regex` é **ausente como tipo de primeiro-cidadão** no pipeline:

- P393 criou `Value::Regex` como variant do enum `Value` (para uso em `Selector::Regex`).
- Mas `Value::Regex` não tem tipo L1 dedicado — usa `regex::Regex` diretamente.
- `text.font` dict (Tabela A.3 linha 376, gap 8 DEBT-52) requer `Value::Regex` para pattern matching em font names: `#set text(font: ("Name": ("Regular", "Bold")))` com regex keys.

No vanilla:
```typ
#let r = regex("a.*b")
#show regex("a.*b"): it => ...
#set text(font: ("Name": ("Regular", "Bold")))  // dict key pode ser regex
```

Este passo é **S puro** — modela o tipo `Regex` L1 e integra no enum `Value`. Não implementa operações regex (`match`, `replace`, etc.) nem `text.font` dict — são passos futuros (S-M cada).

---

## 2. Decisão de engenharia

### 2.1 — `Regex` tipo L1

```rust
// entities/regex.rs — tipo L1 puro
use regex::Regex;

#[derive(Clone, Debug)]
pub struct Regex {
    pub pattern: EcoString,           // pattern original (para repr, eq)
    pub compiled: Option<Arc<Regex>>, // compiled regex (lazy, cacheable)
}
```

**Decisão crate**: `regex` crate já é dependência do projeto (usada em P393 para `Selector::Regex`). Reusar.

**Decisão ADR-0107 (língua vs mecânica)**: a paridade é com o **pattern string** (a linguagem do regex), não com a mecânica de compilação. `compiled` é cache interno; `pattern` é o valor linguagem.

**Decisão ADR-0029 (pureza L1)**: `Regex` é puro — nenhum I/O, compilação é CPU-only.

**Decisão PartialEq**: regex compilado não é `Eq` (cache interno pode variar). Equality é por `pattern` string: `Regex { pattern: "a.*b", .. } == Regex { pattern: "a.*b", .. }`. Implementar `PartialEq` manualmente comparando `pattern`.

**Decisão Arc**: `regex::Regex` não é `Clone` barato (internamente contém compiled NFA/DFA). Arc permite cheap clone O(1). `Regex` struct L1 contém `Arc<regex::Regex>` para compartilhamento.

### 2.2 — Enum `Value::Regex`

```rust
// entities/value.rs — variant já existe desde P393?
// Verificar: P393 adicionou Value::Regex(Regex) ou Selector::Regex(Regex) apenas?
```

**Decisão**: se P393 já adicionou `Value::Regex(Regex)` ao enum, este passo é **refino** — extrair tipo L1 dedicado e ajustar PartialEq/repr/cast. Se não, adicionar agora.

**Verificação P393**: o relatório P393 menciona `Value::Regex(Regex) em entities/value.rs` — então o variant existe. Este passo é **refino de tipo** (extrair struct L1 dedicado) em vez de adição de variant.

### 2.3 — Impacto cross-module (refino de tipo)

| Módulo | O que muda | Como |
|--------|-----------|------|
| `entities/regex.rs` | **Novo arquivo** | `Regex` struct L1 com `pattern` + `compiled` |
| `entities/value.rs` | Ajuste | `Value::Regex(Regex)` — reusa tipo L1 (já existe desde P393) |
| `eval/repr.rs` | Ajuste | `"regex("pattern")"` — reusa `pattern` field |
| `eval/cast.rs` | Ajuste | `Regex → Regex` (identity); `Str → Regex` (compile, fallible) |
| `eval/ops.rs` | Ajuste | `==` por `pattern` equality (não por compiled ptr) |
| `entities/show.rs` | Ajuste | `Selector::Regex(Regex)` — reusa tipo L1 (já existe desde P393) |
| `layout/types.rs` | Nenhum | Regex não é Paint/Fill/Style/Length |
| `export.rs` | Nenhum | Regex não emite direto |
| `stdlib/` | Nenhum | `native_regex` já existe (P393); operações regex são futuro |

**Decisão**: este passo é **refino de tipo existente** — não adiciona variant novo. Extrai `Regex` de `value.rs` (ou de onde estiver inline) para arquivo L1 dedicado, com PartialEq por pattern, repr canónico, e cast de Str.

---

## 3. FASE A — L0 (redação; checkpoint obrigatório)

### A.1 — Prompt L0 `regex.md`

Novo em `00_nucleo/prompts/entities/regex.md`:

- **Paridade**: `Regex` ≡ vanilla `regex` tipo (pattern string compilado).
- **Substrato**: tipo L1 puro `Regex { pattern: EcoString, compiled: Option<Arc<regex::Regex>> }`; `Clone` (Arc); zero I/O; compilação lazy.
- **Semântica**: pattern string é o valor linguagem; compiled é cache interno.
- **PartialEq**: equality por `pattern` string (não por compiled ptr/cache).
- **Repr**: `"regex("pattern")"` — canónico, escaped.
- **Cast**: `Regex → Regex` (identity); `Str → Regex` (compile, fallible — regex inválido retorna Err).
- **Variant `Value`**: `Regex(Regex)` — já existe desde P393; este passo refina para tipo L1 dedicado.
- **Consumer**: `Selector::Regex(Regex)` — já existe desde P393; reusa tipo L1 refino.
- **Teste**: construção `Regex::new("a.*b")` → `Value::Regex` → repr → cast → equality.

### A.2 — CHECKPOINT

Parar. Apresentar `regex.md` ao dono. **Só prosseguir para Fase B quando confirmar que guardou e computou hash.**

---

## 4. FASE B — Código (após confirmação humana)

### B.1 — Tipo entity `Regex`

Em `01_core/src/entities/regex.rs` (novo arquivo):

```rust
use ecow::EcoString;
use regex::Regex as CompiledRegex;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Regex {
    pub pattern: EcoString,
    compiled: Option<Arc<CompiledRegex>>,
}

impl Regex {
    pub fn new(pattern: EcoString) -> Result<Self, regex::Error> {
        let compiled = CompiledRegex::new(&pattern)?;
        Ok(Self {
            pattern,
            compiled: Some(Arc::new(compiled)),
        })
    }

    /// Create without compiling (lazy compilation)
    pub fn from_pattern(pattern: EcoString) -> Self {
        Self {
            pattern,
            compiled: None,
        }
    }

    /// Get compiled regex, compiling if necessary
    pub fn compiled(&self) -> Result<Arc<CompiledRegex>, regex::Error> {
        if let Some(ref compiled) = self.compiled {
            Ok(compiled.clone())
        } else {
            let compiled = Arc::new(CompiledRegex::new(&self.pattern)?);
            // Note: we don't cache here because &self is immutable
            // Lazy compilation without caching is acceptable for S pure
            Ok(compiled)
        }
    }

    pub fn is_match(&self, text: &str) -> Result<bool, regex::Error> {
        let compiled = self.compiled()?;
        Ok(compiled.is_match(text))
    }

    pub fn pattern(&self) -> &str {
        &self.pattern
    }
}

impl PartialEq for Regex {
    fn eq(&self, other: &Self) -> bool {
        self.pattern == other.pattern
    }
}

impl Eq for Regex {}

impl std::hash::Hash for Regex {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pattern.hash(state);
    }
}

impl Default for Regex {
    fn default() -> Self {
        Self::from_pattern(EcoString::new())
    }
}
```

**Nota**: `compiled()` retorna `Result` porque compilação pode falhar (pattern inválido). Mas `new()` já valida, então `compiled()` em instâncias criadas via `new()` nunca falha. Instâncias via `from_pattern()` podem falhar em `compiled()` se pattern for inválido.

**Decisão**: para S puro, `new()` é suficiente — compilação em constructor. `from_pattern()` é refino futuro (lazy compilation). Implementar `new()` apenas neste passo; `from_pattern()` pode ser adicionado depois.

**Simplificação para S puro**:
```rust
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Regex {
    pub pattern: EcoString,
    compiled: Arc<CompiledRegex>,
}

impl Regex {
    pub fn new(pattern: EcoString) -> Result<Self, regex::Error> {
        let compiled = CompiledRegex::new(&pattern)?;
        Ok(Self {
            pattern,
            compiled: Arc::new(compiled),
        })
    }

    pub fn is_match(&self, text: &str) -> bool {
        self.compiled.is_match(text)
    }

    pub fn pattern(&self) -> &str {
        &self.pattern
    }
}
```

### B.2 — Ajuste `Value::Regex`

Em `entities/value.rs`:

```rust
// Antes (P393 — inline):
// Regex(regex::Regex),  // ou similar

// Depois (P402 — tipo L1):
Regex(Regex),  // entities::regex::Regex
```

Atualizar:
- `PartialEq` — `Regex(a) => matches!(other, Regex(b) if a == b)` (já funciona via `Regex::PartialEq` por pattern).
- `Repr` — `"regex("{}")".format(regex.pattern())` com escaping.
- `Cast` — `Regex(r) => Ok(r.clone())`; `Str(s) => Regex::new(s.clone().into()).map_err(|e| eco_format!("regex inválido: {e}"))?`.
- `type_name` — `"regex"`.

### B.3 — Ajuste `Selector::Regex`

Em `entities/show.rs` (ou onde `Selector` está definido):

```rust
// Antes (P393):
// Regex(regex::Regex),  // ou Value::Regex interno

// Depois (P402):
Regex(Regex),  // entities::regex::Regex
```

**Verificação**: se P393 usou `Value::Regex` dentro de `Selector::Regex`, ajustar para usar `Regex` L1 diretamente. Se já usou `regex::Regex` inline, migrar para `Regex` L1.

### B.4 — Ajuste `native_regex` (P393)

Em `rules/stdlib/text.rs` (ou onde `native_regex` está definido):

```rust
// Antes (P393):
// pub fn native_regex(args: &Args) -> SourceResult<Value> {
//     let pattern = args.expect::<Str>("pattern")?;
//     let regex = regex::Regex::new(&pattern)
//         .map_err(|e| eco_format!("regex inválido: {e}"))?;
//     Ok(Value::Regex(regex))  // ou Value::Regex(Regex { ... })
// }

// Depois (P402):
pub fn native_regex(args: &Args) -> SourceResult<Value> {
    let pattern = args.expect::<Str>("pattern")?;
    let regex = Regex::new(pattern.into())
        .map_err(|e| eco_format!("regex inválido: {e}"))?;
    Ok(Value::Regex(regex))
}
```

### B.5 — Testes

1. **Unit `entities/regex.rs`** (5-6 tests):
   - `regex_new_valid` — `Regex::new("a.*b")` → Ok, pattern="a.*b".
   - `regex_new_invalid` — `Regex::new("[")` → Err.
   - `regex_is_match` — `Regex::new("a.*b").is_match("axxxb")` → true.
   - `regex_is_match_no` — `Regex::new("a.*b").is_match("xxx")` → false.
   - `regex_equality` — `Regex::new("a") == Regex::new("a")` (por pattern).
   - `regex_equality_diff` — `Regex::new("a") != Regex::new("b")`.
   - `regex_clone` — clone compartilha compiled Arc.
   - `regex_hash` — `Hash` por pattern.

2. **Unit `entities/value.rs`** (4-5 tests):
   - `value_regex_variant` — discriminação `Value::Regex`.
   - `value_regex_repr` — `"regex("a.*b")"` para pattern "a.*b".
   - `value_regex_repr_escape` — `"regex("a\.b")"` com escaping.
   - `value_regex_cast_identity` — `Regex → Regex`.
   - `value_regex_cast_from_str` — `Str("a.*b") → Regex` (compile ok).
   - `value_regex_cast_from_str_invalid` — `Str("[") → Err`.
   - `value_regex_partial_eq` — equality por pattern.

3. **Unit `entities/show.rs`** (2-3 tests):
   - `selector_regex` — `Selector::Regex(Regex::new("a.*b"))` construção.
   - `selector_regex_equality` — equality por pattern.

4. **Integration / regression P393** (2-3 tests):
   - `native_regex_still_works` — `regex("a.*b")` retorna `Value::Regex` (regression P393).
   - `show_regex_still_works` — `#show regex("a.*b"): it => ...` funciona (regression P393).
   - `regex_in_dict_key_future` — placeholder para `text.font` dict (não funciona ainda, mas tipo existe).

### B.6 — Linhagem

- `@prompt` aponta para `regex.md`.
- `@prompt-hash` via `--fix-hashes`.
- Referência cruzada: P393 (`Selector::Regex` + `native_regex` baseline), P395 (portão ADR-0017 aberto), ADR-0107 (paridade linguagem), ADR-0029 (pureza L1), DEBT-52 (`text.font` dict bloqueado).

---

## 5. O que NÃO fazer (scope-out)

- **Não** implementar operações regex (`match`, `replace`, `find`, `split`, etc.) — scope-out ADR-0054 graded (futuro S cada).
- **Não** implementar `text.font` dict — depende de `Value::Regex` + `FontList` refactor (DEBT-52, futuro S-M).
- **Não** implementar `regex` como literal suffixo — não existe no vanilla.
- **Não** implementar lazy compilation (`from_pattern`) — refino futuro XS; `new()` compila eager.
- **Não** implementar `regex` flags (`i`, `m`, `s`, etc.) — scope-out ADR-0054 graded (futuro S).
- **Não** adicionar `Regex` como `Paint`/`Fill`/`Style`/`Length` — não é visual.
- **Não** quebrar invariantes de camada (L1 puro, zero I/O).

---

## 6. Critérios de aceitação

1. `Regex` struct L1 existe em `entities/regex.rs` com `pattern` + `compiled` (Arc).
2. `Value::Regex(Regex)` reusa tipo L1 (refino de P393).
3. `Selector::Regex(Regex)` reusa tipo L1 (refino de P393).
4. `PartialEq` por `pattern` string (não por compiled ptr).
5. `Repr`: `"regex("pattern")"` com escaping.
6. `Cast`: `Str → Regex` (compile, fallible); `Regex → Regex` (identity).
7. `native_regex` (P393) migrado para usar `Regex` L1 — sem regressão.
8. `#show regex(...)` (P393) continua funcionando — sem regressão.
9. Zero consumer novo; zero I/O; zero func stdlib nova (refino de existente).
10. Testes verdes (≥15 unit + 3-5 integration/regression); lint zero; hashes propagados.
11. Inventário 148: `Value::Regex` transita `parcial` (P393 inline) → `implementado` (tipo L1 dedicado); `text.font` dict permanece `ausente` (DEBT-52, desbloqueado para futuro).
12. L0 salvo e hashado antes do código (protocolo de nucleação).
13. Ritmo S: tempo de ciclo comparável a P399-P401 (baseline de tipo S puro).

---

## 7. O que pode sair errado

- **P393 não criou `Value::Regex` — apenas `Selector::Regex` com `regex::Regex` inline.** Mitigação: se `Value::Regex` não existe, adicionar ao enum agora (adição de variant, não refino). Se `Selector::Regex` usa `regex::Regex` inline, migrar para `Regex` L1.
- **`regex` crate não está no `Cargo.toml` de `01_core`.** Mitigação: verificar; se estiver em `03_infra` ou workspace, adicionar a `01_core` se necessário. Se `regex` é usado apenas em `03_infra`, considerar se `entities/regex.rs` em L1 pode depender dele (L1 deve ser puro, mas `regex` é puro-Rust).
- **`regex::Regex` não é `Send`/`Sync` — Arc pode ser problemático.** Mitigação: `regex::Regex` é `Send` + `Sync` (documentação confirma). Arc é seguro.
- **PartialEq por pattern pode ser surpreendente — `Regex::new("a") != Regex::new("a")` se pattern diferente.** Mitigação: é o comportamento correto — pattern string é o valor linguagem. Dois regex com mesmo pattern semantic mas sintaticamente diferente (ex.: `"a"` vs `"a"` com flags) são diferentes.
- **Escaping em repr pode ser complexo.** Mitigação: usar `pattern.escape_default()` ou similar para escaping canónico. Se não existir, usar substituição simples de `"` → `"` e `\` → `\`.
- **Tentação de já implementar operações regex ou `text.font` dict.** Mitigação: um passo de cada vez; este é S puro de refino de tipo.

---

## 8. Referências

- P393 — `#show regex(...)` + `Selector::Regex` + `native_regex` (baseline a refinar).
- P395 — `Value::Tiling` (portão ADR-0017 aberto).
- P399-P401 — série de tipos S puros (Decimal, Duration, Version — padrão paralelo).
- ADR-0017 — trava arquitetural enum fechado.
- ADR-0107 — paridade linguagem vs mecânica (pattern string é o valor).
- ADR-0029 — pureza L1.
- ADR-0054 — graded scope-out (operações regex, flags, text.font dict).
- DEBT-52 — `text.font` dict bloqueado por regex ausente em L1.
- `regex` crate docs — https://docs.rs/regex

---

## 9. Nota sobre o Tekt

Este passo é **refino de tipo existente** — não adiciona variant novo ao enum `Value` (P393 já adicionou), mas extrai `Regex` para tipo L1 dedicado com semântica correta (PartialEq por pattern, repr canónico, cast de Str). É o padrão "tipo primitivo deve ter arquivo L1 próprio" (sub-padrão #14, P227 Corners).

O benefício é **destravar 2 features** com um S puro:
1. `text.font` dict (DEBT-52) — requer `Value::Regex` como tipo de primeiro-cidadão para pattern matching.
2. Operações regex futuras (`match`, `replace`, etc.) — requerem tipo L1 com `is_match`, `find`, etc.

Registar o tempo de ciclo como **baseline de refino de tipo** (vs modelagem de tipo novo P399-P401). Se o refino for significativamente mais lento que a modelagem, investigar: refino de código existente é mais arriscado que modelagem de novo.
