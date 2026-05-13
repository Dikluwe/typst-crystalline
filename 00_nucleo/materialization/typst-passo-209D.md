# Passo 209D — `Selector::Regex` + ADR-0077 PROPOSTO + dep `regex` L1

**Série**: 209 (sub-passo `D`).
**Marco**: M9c (Bloco VI — Selector extensions; variant
5 de 5).
**Tipo**: implementação + ADR nova + dep nova em
allowlist L1.
**Magnitude**: M (~1-1.5h).
**Pré-condição**: P209C concluído; Selector cristalino
5 variants (`Kind`, `Label`, `Location`, `And`, `Or`);
query arms 5; trait 26 métodos; allowlist L1 com 11
entries (sem `regex`); tests 1924 verdes; 0 violations;
ADR-0076 anotado §P209C.
**Output**: 2 ficheiros (relatório curto + ADR-0077
PROPOSTO).

---

## §1 Trabalho

Materializar o 5º variant do Selector enum + 3
componentes anexos:

- **Wrapper L1 `Regex`** em
  `01_core/src/entities/regex.rs` sobre `regex::Regex`
  crate. Hash/Eq/PartialEq manuais (pattern string como
  key).
- **`Selector::Regex(Regex)`** variant.
- **Query arm `Regex`** — **stub `vec![]` documentado**
  per P209A A3 (cristalino single-pass não tem Content
  text durante query phase).
- **`regex` crate em allowlist L1** —
  `crystalline.toml:64` + `01_core/Cargo.toml`.
- **ADR-0077 PROPOSTO** documentando justificação.
- **Stdlib func `native_regex(pattern)` constructor**
  — Opção stdlib paralela a vanilla (Caminho A per
  P209A C2).

Reuso de dados P209A + P209B + P209C:

- Estrutura literal fixada em P209A C1.
- Caminho A (`regex` full) fixado em P209A C2.
- Stdlib API Regex via `regex(pattern)` constructor
  per P209A C3.
- Pattern de wrapper L1 sobre dep externa não tem
  precedente directo cristalino — registar como pattern
  novo.

---

## §2 Cláusulas (6)

### C1 — Verificação curta de pré-condições

Antes de tocar código:

1. **Allowlist L1 actual**: confirmar
   `crystalline.toml:64` ainda tem 11 entries sem
   `regex`. Per P209A A4.
2. **`regex` crate versão**: identificar versão actual
   estável (esperado 1.x). Verificar features default
   vs gated; `regex` 1.x tem feature `unicode` (default
   on) — manter ou desactivar?
3. **Padrão ADR para deps L1**: examinar ADR-0023
   (indexmap) ou ADR-0024 (ecow) como template para
   ADR-0077. Confirmar estrutura.
4. **Cristalino `Hash` manual existentes**: identificar
   precedentes (Position via `to_bits()` per P204D;
   SealedPositions; outros). Pattern para Regex
   wrapper.

Se algum falhar, registar `P209D.div-N`.

### C2 — Materializar wrapper L1 `Regex`

**L0 primeiro**:

Novo prompt `00_nucleo/prompts/entities/regex.md`:
- Contexto: wrapper L1 sobre `regex::Regex` crate.
- Interface: `pub struct Regex { pattern: String, compiled: regex::Regex }`.
- Hash/Eq/PartialEq manuais via `pattern` string como
  key (compiled deriva do pattern; mesma pattern =
  mesma regex).
- Constructor `Regex::new(pattern: &str) -> Result<Self, RegexError>`.
- Method `is_match(text: &str) -> bool`.
- Tests obrigatórios (estrutural + Hash + match).

**L1 depois**:

`01_core/src/entities/regex.rs` (novo):

```text
use std::hash::{Hash, Hasher};

pub struct Regex {
    pattern:  String,
    compiled: regex::Regex,
}

#[derive(Debug, thiserror::Error)]
pub enum RegexError {
    #[error("regex inválida: {0}")]
    Invalid(String),
}

impl Regex {
    pub fn new(pattern: &str) -> Result<Self, RegexError> {
        let compiled = regex::Regex::new(pattern)
            .map_err(|e| RegexError::Invalid(e.to_string()))?;
        Ok(Self { pattern: pattern.to_string(), compiled })
    }

    pub fn pattern(&self) -> &str { &self.pattern }
    pub fn is_match(&self, text: &str) -> bool { self.compiled.is_match(text) }
}

impl Hash for Regex {
    fn hash<H: Hasher>(&self, state: &mut H) { self.pattern.hash(state); }
}
impl PartialEq for Regex {
    fn eq(&self, other: &Self) -> bool { self.pattern == other.pattern }
}
impl Eq for Regex {}
impl Clone for Regex {
    fn clone(&self) -> Self {
        Self::new(&self.pattern).expect("regex previamente válida")
    }
}
impl std::fmt::Debug for Regex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Regex").field("pattern", &self.pattern).finish()
    }
}
```

Tests inline (4-6):
- `regex_new_valido_ok`.
- `regex_new_invalido_err`.
- `regex_hash_determinismo` — mesma pattern → mesmo
  hash.
- `regex_is_match_basico`.
- `regex_clone_preserva_semantica`.

### C3 — Dep `regex` em allowlist L1 + Cargo.toml

L0/configs:

- `crystalline.toml:64` — adicionar `regex` à allowlist
  `l1_allowed_external.rust`.
- `01_core/Cargo.toml` — adicionar:
  ```text
  regex = "1"
  ```
  Decisão sobre features: Caminho A per P209A C2 = full
  Unicode default. Se C1.2 mostrar custo binário
  excessivo, considerar `default-features = false` +
  features mínimas — mas Caminho A = aceitar custo
  binário (ADR-0077 justifica).

### C4 — ADR-0077 PROPOSTO

`00_nucleo/adr/typst-adr-0077-regex-l1.md` (novo,
estrutura paralela a ADR-0023/0024):

§1 Contexto:
- M9c P209 série materializa `Selector::Regex` per Q3=α.
- Cristalino actual sem dep regex em L1 (allowlist 11
  entries).
- Wrapper L1 isola dep em 1 ponto.

§2 Decisão:
- `regex` 1.x autorizado em L1 com features default
  (Unicode full).
- Wrapper `entities::regex::Regex` é único consumer
  directo do crate em L1.
- Alternativas rejeitadas: `regex-lite` (subset; sem
  paridade vanilla); sem regex (semântica degenerada).

§3 Consequências:
- Positivas: paridade vanilla regex syntax; consumer
  Rust standard.
- Negativas: binário +~1.5 MB por features Unicode.
- Mitigação: pattern wrapper L1 permite swap futuro
  (sem refactor cross-modular).

§4 Plano de validação:
- 4-6 tests em `entities::regex`.
- `cargo build --workspace` verde.
- `crystalline-lint` 0 violations.
- ADR-0077 transita PROPOSTO → ACEITE em P209E ou
  P212 (encerramento M9c).

§5 Cross-references:
- ADR-0029 (allowlist deps L1 política).
- ADR-0023, ADR-0024 (deps anteriores em L1 — templates).
- P209A C2 (decisão Caminho A `regex` full).

### C5 — `Selector::Regex` variant + query arm stub

`01_core/src/entities/selector.rs`:

```text
+ use crate::entities::regex::Regex;

pub enum Selector {
    Kind(ElementKind),
    Label(Label),
    Location(Location),
    And(EcoVec<Selector>),
    Or(EcoVec<Selector>),
+   Regex(Regex),
}
```

Tests estruturais (~2):
- `p209d_selector_regex_estrutural` — igualdade enum +
  Hash determinismo.
- `p209d_selector_regex_in_or` — composição com Or.

`01_core/src/entities/introspector.rs` query arm:

```text
+ Selector::Regex(_re) => {
+     // P209D: stub documentado. Cristalino single-pass
+     // não tem Content text durante query phase. Per
+     // ADR-0076 §P209D + ADR-0077: variant materializado
+     // estructuralmente; semântica de match-by-text fica
+     // para passo dedicado quando Content text durante
+     // query for acessível (P212+).
+     Vec::new()
+ }
```

Tests (~2):
- `p209d_introspector_query_regex_devolve_empty_stub`.
- `p209d_introspector_query_regex_in_or_compoe_com_kind`
  — confirma que composição estrutural funciona; query
  retorna apenas matches do Kind (Regex retorna empty).

### C6 — Stdlib func `native_regex(pattern)` constructor +
verificação final

`01_core/src/rules/stdlib/foundations.rs`:

```text
pub fn native_regex(
    ctx:              &mut EvalContext,
    args:             &Args,
    _world:           &dyn World,
    _current_file:    FileId,
    _figure_numbering: Option<&str>,
) -> SourceResult<Value> {
    let pattern = /* args[0] espera Value::Str */;
    let regex = Regex::new(&pattern)
        .map_err(|e| /* SourceDiagnostic::error com hint */)?;
    Ok(/* envelopar em Value */)
}
```

**Decisão crítica em C6**: como expor `Regex` em
`Value`?
- **Opção α** — Novo `Value::Regex(Regex)` variant.
  Modifica `01_core/src/entities/value.rs` enum.
  Magnitude: M+ (toca múltiplos call-sites de match
  exhaustive de `Value`).
- **Opção β** — `Value::Other(format!("regex:{}", pattern))`
  per convenção P206B (DTO catch-all). Cristalino
  consumer não consegue extrair regex back — semântica
  degenerada.
- **Opção γ** — Adiar stdlib `native_regex`; expor
  apenas Rust API. Per pattern "Caminho 1 anti-inflação"
  + per Q3=α que pediu materializar `Selector::Regex`
  variant mas não necessariamente stdlib func.

**C6 fixa Opção γ** por hipótese provável (consumer
real ainda inexistente; pattern P208B/P208D anti-inflação;
stdlib expressão deferred até consumer real).

**Reavaliar em P209E**: se C1 P209E mostrar consumer
real em horizonte próximo, Opção α justifica-se;
caso contrário, mantém γ.

Se C6 = γ, **sem stdlib func nova**. Apenas variant +
query arm + wrapper.

```
cargo test --workspace 2>&1 | tail -10
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério: 1930+ verdes (1924 + 6+); 0 violations.

**Regra empírica P207B §5 não accionada** — Selector
extension não toca trait Introspector. Trait mantém 26
métodos.

Anotar ADR-0076 §P209D: `✅ MATERIALIZADO {data}` +
sumário (variant + ADR-0077 + dep + stub).

---

## §3 Outputs

**2 ficheiros**:

1. `00_nucleo/materialization/typst-passo-209D-relatorio.md`
   — relatório curto (~4-6 KB) com 6 §s padrão.
2. `00_nucleo/adr/typst-adr-0077-regex-l1.md` — ADR
   PROPOSTO (~3-4 KB).

---

## §4 Não-objectivos

- `Selector::Where`/`Before`/`After` (fora roadmap
  M9c).
- Trait method extensions.
- Semântica funcional de `query(Regex)` (stub deferred).
- `Value::Regex` variant (Opção α C6 deferred per γ).
- ADR-0077 transição PROPOSTO → ACEITE (P209E ou P212).
- `regex-lite` ou alternativas (Caminho A fixado).
- Performance benchmarking de regex (out of scope).

---

## §5 Riscos a evitar

1. **Stub `query(Regex)` confundir auditor**: variant
   materializado mas query retorna empty. **Documentar
   explícitamente** em L0 selector.md + introspector.md
   + ADR-0076 §P209D + comments inline. Pattern "materialização
   estrutural sem semântica completa" é honesta mas
   exige documentação clara.
2. **`Hash`/`Eq`/`PartialEq` manuais inconsistentes**:
   Hash via pattern; Eq via pattern. Sempre simétrico.
   `regex::Regex` derive ausente — wrapper é a única
   forma.
3. **`Clone` panic em pattern inválida**: `Regex::new`
   pode falhar; `Clone` re-invoca `new`. Em prática, se
   Regex original é válida, pattern é válida, recompila
   OK. Mas `unwrap` é semanticamente arriscado se pattern
   for corrompida em runtime (improvável). Documentar.
4. **Dep `regex` features default Unicode pode inflar
   binário**: aceitar per ADR-0077 (Caminho A); medir
   pós-build se relevante.
5. **C6 Opção γ é Caminho 1 anti-inflação 6ª aplicação**:
   `native_regex` stdlib deferred. Documentar.
6. **ADR-0077 paralela mas distinta de ADR-0029**:
   0077 é dep específica (regex); 0029 é política
   geral de allowlist. Não confundir.
7. **Test estructural de `Regex` em composição com `Or`**:
   confirmar que recursive Hash em `Selector::Or(vec![
   Regex(...), Kind(...)])` funciona (per P209C
   confirmação de recursive Hash). Esperado OK.

---

## §6 Hipótese provável

C2-C5 directos per spec. C6 = Opção γ (deferred
stdlib func) — 6ª aplicação Caminho 1 anti-inflação.

ADR-0077 PROPOSTO escrito; transita ACEITE em P209E
(se P209E for tipo encerramento com transição ADR) ou
P212 (encerramento M9c).

Custo real estimado: M (~1-1.5h) — wrapper L1 + ADR +
dep + variant + query stub + 6-8 tests.

Mas é hipótese, não decisão. C1-C6 fixam-se com base
em evidência empírica.
