---

# P465 — `repr()` completo: representação de valores para debug e paridade

> **Passo:** 465  
> **Data:** 2026-06-25  
> **Foco:** Completar a implementação de `repr()` para todos os tipos `Value`, alcançando paridade com o Typst vanilla na saída de debug.  
> **Trilha:** 8 — Refinos de stdlib e tipos.  
> **Tipo:** Materialização / Refacto mecânico.  
> **Tamanho:** S (~15 min).  
> **ADR-0117 Cláusula 4:** `repr()` é função pura de transformação de dados; não propõe estrutura em elementos existentes.

---

## Contexto

O Typst vanilla tem `repr(val)` que converte qualquer `Value` numa string representativa para debug. O cristalino tem `repr()` parcial — cobre `Int`, `Float`, `Str`, `Array`, `Dict`, mas **não cobre**:

| Tipo | Estado no cristalino | Estado no vanilla |
|------|----------------------|-------------------|
| `Content` | Ausente ou incompleto | `"heading(...)"`, `"figure(...)"`, etc. |
| `Symbol` | Ausente | `"sym.arrow"`, etc. |
| `Func` | Ausente | `"#function(...)"` ou nome |
| `Module` | Ausente | `"module(...)"` |
| `Type` | Ausente | `"type(...)"` |
| `None` | Parcial | `"none"` |
| `Auto` | Parcial | `"auto"` |
| `Color` | Ausente | `"rgb(...)"`, `"cmyk(...)"`, etc. |
| `Gradient` | Ausente | `"linear(...)"`, `"radial(...)"` |
| `Version` | Ausente | `"version(0, 11, 0)"` |
| `Bytes` | Ausente | `"bytes(...)"` |
| `Datetime` | Ausente | `"datetime(...)"` |
| `Duration` | Ausente | `"duration(...)"` |
| `Regex` | Ausente | `"regex(...)"` |
| `Label` | Ausente | `"label(...)"` |

Este passo materializa o subset que é **fechável agora** (tipos que já existem como `Value` no cristalino), deixando `Gradient` para Trilha 4 e `Color` avançado para quando os espaços de cor forem materializados.

---

## ADR-0108 — Medir antes de decidir

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `repr()` existe em stdlib? | Sim — parcial | ✅ |
| `Value` enum tem todos os tipos listados? | Sim — definidos em `entities/value.rs` | ✅ |
| `Value::Content` tem `Debug` ou `Display`? | Parcial — `Content` tem `Debug` | 🟡 |
| `Value::Color` tem `Debug`? | Sim — `Color` implementa `Debug` | ✅ |
| `Value::Symbol` existe? | Sim — `Symbol` em `entities/symbol.rs` | ✅ |
| `Value::Func` existe? | Sim — `Func` em `entities/func.rs` | ✅ |
| `Value::Module` existe? | Sim — `Module` em `entities/module.rs` | ✅ |
| `Value::Type` existe? | Sim — `Type` em `entities/ty.rs` | ✅ |
| `Value::Version` existe? | Sim — `Version` em `entities/version.rs` | ✅ |
| `Value::Bytes` existe? | Sim — `Bytes` em `entities/bytes.rs` | ✅ |
| `Value::Datetime` existe? | Sim — `Datetime` em `entities/datetime.rs` | ✅ |
| `Value::Duration` existe? | Sim — `Duration` em `entities/duration.rs` | ✅ |
| `Value::Regex` existe? | Sim — `Regex` em `entities/regex.rs` | ✅ |
| `Value::Label` existe? | Sim — `Label` em `entities/elements/label.rs` | ✅ |
| Bloqueadores? | Nenhum — função pura, zero I/O | ✅ |

**Reclassificação:** S (~15 min; completar match arms em `native_repr` + tests).

---

## Toques pontuais

### 1. Completar `native_repr` (`rules/stdlib/utility.rs` ou `rules/stdlib/debug.rs`)

```rust
fn native_repr(value: Value) -> EcoString {
    match value {
        Value::None => "none".into(),
        Value::Auto => "auto".into(),
        Value::Int(i) => format!("{}", i).into(),
        Value::Float(f) => format!("{}", f).into(),
        Value::Str(s) => format!("{:?}", s).into(),  // já existe
        Value::Array(a) => format!("{:?}", a).into(), // já existe
        Value::Dict(d) => format!("{:?}", d).into(),  // já existe

        // NOVO — tipos existentes mas sem repr:
        Value::Content(c) => format!("{:?}", c).into(),
        Value::Symbol(s) => format!("sym.{}", s.name()).into(),
        Value::Func(f) => {
            if let Some(name) = f.name() {
                format!("#{}", name).into()
            } else {
                "#function(...)".into()
            }
        }
        Value::Module(m) => format!("module({})", m.name()).into(),
        Value::Type(t) => format!("type({})", t.name()).into(),
        Value::Color(c) => format!("{:?}", c).into(),  // rgb/hex
        Value::Version(v) => format!("version({}, {}, {})", v.major, v.minor, v.patch).into(),
        Value::Bytes(b) => format!("bytes({})", b.len()).into(),
        Value::Datetime(dt) => format!("datetime({})", dt).into(),
        Value::Duration(d) => format!("duration({})", d).into(),
        Value::Regex(r) => format!("regex({:?})", r.pattern()).into(),
        Value::Label(l) => format!("label({:?})", l.name).into(),

        // Scope-out para Trilha 4:
        Value::Gradient(_) => "gradient(...)".into(),  // placeholder até Trilha 4
        Value::Tiling(_) => "tiling(...)".into(),       // placeholder
    }
}
```

**Nota:** `Value::Color` usa `Debug` existente. Se `Color` não tem `Debug` completo (RGB vs CMYK vs Oklab), o output pode ser genérico. Isso é aceitável — aperfeiçoamento de `Color::Debug` é Trilha 4.

### 2. Ajustar `Value::Content::Debug`

**Ficheiro:** `entities/content.rs` (ou onde `impl Debug for Content` está)

```rust
impl fmt::Debug for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Content::Text(t) => write!(f, "text({:?})", t),
            Content::Styled(s) => write!(f, "styled({:?}, {:?})", s.body, s.styles),
            Content::Sequence(seq) => write!(f, "sequence({:?})", seq),
            Content::Heading(h) => write!(f, "heading({:?})", h.body),
            Content::Figure(fig) => write!(f, "figure({:?})", fig.body),
            Content::Table(t) => write!(f, "table({}x{})", t.rows, t.columns),
            Content::Math(m) => write!(f, "math({:?})", m.body),
            Content::Link(l) => write!(f, "link({:?})", l.url),
            Content::Label(l) => write!(f, "label({:?})", l.name),
            Content::Ref(r) => write!(f, "ref({:?})", r.name),
            Content::Outline(o) => write!(f, "outline({:?})", o.title),
            Content::Bibliography(b) => write!(f, "bibliography({} entries)", b.entries.len()),
            // ... outros variants
        }
    }
}
```

**Decisão:** `Debug` para `Content` é suficiente para `repr()`. Não precisa de `Display` customizado.

### 3. Tests

- **L1:** 14 testes — um para cada tipo `Value` sem repr, verificando string exacta.
  - `repr(none)` → `"none"`
  - `repr(auto)` → `"auto"`
  - `repr(1)` → `"1"`
  - `repr(1.5)` → `"1.5"`
  - `repr("hello")` → `""hello""`
  - `repr(sym.arrow)` → `"sym.arrow"`
  - `repr(rgb(255, 0, 0))` → `"rgb(255, 0, 0)"` (ou equivalente)
  - `repr(label("sec1"))` → `"label("sec1")"`
  - `repr(version(0, 11, 0))` → `"version(0, 11, 0)"`
  - `repr(bytes(10))` → `"bytes(10)"`
  - `repr(datetime(...))` → `"datetime(...)"` (formato ISO)
  - `repr(duration(...))` → `"duration(...)"`
  - `repr(regex("a+"))` → `"regex("a+")"`
  - `repr(heading[Title])` → `"heading("Title")"`

- **L2:** 1 teste — `repr()` de `Array` misto (`(1, "a", none)`) → `"(1, "a", none)"`.

### 4. Spec L0

- `rules/stdlib/debug.md` — `repr(value)` com tabela de tipos e formatos de saída.
- `entities/value.md` — `Value` enum com nota: "todos os variants têm `repr()` implementado".

---

## Scope-out explícito

- **`Gradient` / `Tiling`** — placeholder `"gradient(...)"` / `"tiling(...)"`; materialização real é Trilha 4.
- **`Color` avançado** — `repr` de `Color` usa `Debug` existente; se incompleto (falta CMYK/Oklab), é scope-out de Trilha 4.
- **`Func` com closure captures** — `"#function(...)"` é aceitável; capturas internas não expostas.
- **`Module` com conteúdo detalhado** — apenas nome do módulo, não lista de membros.
- **`Bytes` com conteúdo** — apenas tamanho, não hex dump.
- **Pretty-printing** — `repr` é single-line; pretty-print é scope-out.
- **Cyclic references** — `repr` de `Dict`/`Array` com ciclos não é tratado; scope-out (raro na prática).

---

## Critério de fecho

- [ ] `native_repr` cobre todos os `Value` variants existentes (14+ tipos).
- [ ] `Value::Content` tem `Debug` implementado para todos os variants de `Content`.
- [ ] 15 tests verdes (14 L1 + 1 L2).
- [ ] Spec L0 atualizada (`rules/stdlib/debug.md`, `entities/value.md`).
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations.
- [ ] **Trilha 8: 1/8 completo** (`repr()` fechado).

---

## Próximo passo (Trilha 8 continua ou pivot)

- **P466** — Métodos restantes de `array`/`dict`/`str` (Trilha 8, S, ~15 min)
- **P466** — Tipos `Value`: `Relative` (Rel<Length>), `Symbol` refinado (Trilha 8, S, ~15 min)
- **P466** — Sonda `Selector::Where` (Trilha 3, S, ~15 min)
- **P466** — Bibliografia Fase 2: estilos numéricos (Trilha 6, M, ~40 min)

**Aguardando sua indicação:**

1. **Executar o P465** (`repr()` completo, ~15 min)?
2. **Escrever o P466** (próximo passo: métodos, Selector::Where, ou bibliografia)?
3. **Ajustar o escopo** do P465?

---

## Estado pós-P464 (para referência)

| Passo | Descrição | Estado | Trilha |
|-------|-----------|--------|--------|
| **P444** | `underline` / `overline` / `strike` | ✅ FECHADO | 8 |
| **P445** | Smart quotes | ✅ FECHADO | 8 |
| **P446** | Smallcaps | ✅ FECHADO | 8 |
| **P447** | Cobertura vanilla + DSM audit | ✅ FECHADO | — |
| **P448** | Subscript / Superscript | ✅ FECHADO | 8 |
| **P449** | Highlight | ✅ FECHADO | 8 |
| **P450** | Bibliography `.bib` de disco | ✅ FECHADO | 6 (Fase 1) |
| **P451** | Heading numbering | ✅ FECHADO | 1 |
| **P452** | Links / Hyperlinks | ✅ FECHADO | 2 |
| **P453** | Compilado de correções documentais | ✅ FECHADO | — |
| **P454** | Figure numbering | ✅ FECHADO | 1 |
| **P455** | Fecho correções retroativas + Cláusula 4 ADR-0117 | ✅ FECHADO | — |
| **P456** | Equation numbering | ✅ FECHADO | 1 |
| **P457** | Table of Contents (`outline()`) | ✅ FECHADO | 1 |
| **P458** | Sonda DEBT-2: premissa refutada | ✅ FECHADO | — |
| **P459** | Table numbering | ✅ FECHADO | 1 |
| **P460** | `label<x>`: Destinos nomeados | ✅ FECHADO | 2 |
| **P461** | Correção `table_counter` → `CounterRegistry` | ✅ FECHADO | 1 |
| **P462** | `ref<x>` / `@x`: Resolução de destino | ✅ FECHADO | 2 |
| **P463** | PDF `/GoTo` links internos | ✅ FECHADO | 2 |
| **P464** | Cleanup `Content::Label` vs `Content::Labelled` | ✅ FECHADO | — |
| **DEBT-2** | Closures eager | ✅ FECHADO | — |
| **DEBT-9** | Tracking contínuo | ℹ️ Processo | — |

**Inventário de débitos: LIMPO.**  
**Trilha 1: COMPLETA E COERENTE.**  
**Trilha 2: COMPLETA.**

