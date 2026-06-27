---

# P469 — `Value::Relative` (`Rel<Length>`): comprimentos relativos

> **Passo:** 469  
> **Data:** 2026-06-25  
> **Foco:** Materializar o tipo `Rel<Length>` (comprimento relativo) para expressões como `50% + 2cm`, `100% - 1em`, e integrá-lo no eval de `Value`.  
> **Trilha:** 8 — Refinos de stdlib e tipos.  
> **Tipo:** Materialização / Refacto mecânico.  
> **Tamanho:** S (~15 min).  
> **ADR-0117 Cláusula 4:** Tipo puro de entidade; não propõe estrutura em elementos existentes. Verificar `Value` enum antes de adicionar variant.

---

## Contexto

O Typst vanilla suporta comprimentos relativos via `Rel<Length>`:
- `50%` — relativo a uma dimensão de contexto (largura do container, altura da página).
- `100% - 1em` — combinação de relativo e absoluto.
- `1fr` — fração de espaço restante (em grids/tables).

O cristalino tem `Value::Length` (absoluto: `cm`, `mm`, `pt`, `em`) mas **não tem** `Value::Relative`. Isso bloqueia:
- Margens relativas (`set page(margin: 10%)`).
- Larguras de coluna relativas (`#block(width: 50%)`).
- Espaçamento flex em grids (`1fr`, `2fr`).

Este passo materializa o tipo `Rel<Length>` e sua integração básica no eval, deixando consumers de layout (margins, widths, fr) para passos futuros de Trilha 7.

---

## ADR-0108 — Medir antes de decidir

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `Value::Length` existe? | Sim — `Length` absoluto (`cm`, `mm`, `pt`, `em`) | ✅ |
| `Value::Relative` existe? | Não | ❌ |
| `Length` tipo existe com unidades? | Sim — `Abs`, `Em` | ✅ |
| `Fr` (fraction) tipo existe? | Não — zero infra de fração | ❌ |
| Eval de `%` em expressões? | Parcial — `%` é operador de remainder em `Int` | 🟡 |
| `block(width: ...)` aceita `Length`? | Sim — aceita `Value::Length` | ✅ |
| `block(width: ...)` aceitaria `Relative`? | Não — só `Length` | ❌ |
| Bloqueadores? | Nenhum técnico | ✅ |

**Reclassificação:** S (~15 min; tipo `Rel<Length>` + eval de `%` como relativo + integração em `Value` + tests).

**Nota:** O operador `%` em Typst é ambíguo:
- Em `Int`: `5 % 2` = `1` (remainder).
- Em `Length`: `50%` = `Rel(0.5, Length::zero())` (relativo).
- O contexto (tipo do operando esquerdo) determina o significado.

---

## Toques pontuais

### 1. Entidade `Rel<T>` (`entities/rel.rs` — novo)

```rust
/// Comprimento relativo: percentual de um contexto + offset absoluto.
/// Ex: `50% + 2cm` = Rel { rel: 0.5, abs: Length::cm(2.0) }
/// Ex: `100% - 1em` = Rel { rel: 1.0, abs: Length::em(-1.0) }
#[derive(Clone, Debug, PartialEq)]
pub struct Rel<T> {
    pub rel: f64,      // Fração do contexto (0.5 = 50%)
    pub abs: T,        // Offset absoluto
}

impl Rel<Length> {
    pub fn zero() -> Self {
        Self { rel: 0.0, abs: Length::zero() }
    }

    pub fn from_percent(pct: f64) -> Self {
        Self { rel: pct / 100.0, abs: Length::zero() }
    }

    /// Resolve para comprimento absoluto dado um contexto.
    pub fn resolve(&self, context: Length) -> Length {
        self.abs + context * self.rel
    }
}

// Operações aritméticas
impl Add for Rel<Length> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self { rel: self.rel + rhs.rel, abs: self.abs + rhs.abs }
    }
}

impl Sub for Rel<Length> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self { rel: self.rel - rhs.rel, abs: self.abs - rhs.abs }
    }
}

impl Mul<f64> for Rel<Length> {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self { rel: self.rel * rhs, abs: self.abs * rhs }
    }
}

impl Div<f64> for Rel<Length> {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self { rel: self.rel / rhs, abs: self.abs / rhs }
    }
}
```

**Decisão:** `Rel<T>` genérico para permitir `Rel<Length>`, `Rel<Abs>` (futuro), etc. Mas neste passo, apenas `Rel<Length>` é materializado.

### 2. `Value::Relative` (`entities/value.rs`)

```rust
pub enum Value {
    // ... existing variants
    Relative(Rel<Length>),  // NOVO
}
```

**Impacto:** Adicionar braço `Value::Relative` em todos os `match` de `Value`:
- `rules/eval/repr.rs` — `repr` de `Relative`.
- `rules/eval/ops.rs` — operações aritméticas com `Relative`.
- `rules/eval/cast.rs` — cast de `Relative` para `Length` (requer contexto).
- `rules/eval/tests.rs` — tests de `Relative`.

### 3. Eval de `%` como relativo (`rules/eval/ops.rs`)

```rust
// No eval de operador `%`:
// Se operando esquerdo é Length: `50%` → Rel::from_percent(50.0)
// Se operando esquerdo é Int: `5 % 2` → 1 (remainder, existente)

match (lhs, rhs) {
    (Value::Int(a), Value::Int(b)) => Value::Int(a % b),  // existente
    (Value::Length(_), Value::Int(pct)) => {
        Value::Relative(Rel::from_percent(*pct as f64))
    }
    (Value::Length(_), Value::Float(pct)) => {
        Value::Relative(Rel::from_percent(*pct))
    }
    // ...
}
```

**Nota:** O Typst vanilla usa `50%` como sintaxe especial (não operador `%`). No cristalino, modelar como operador `%` aplicado a `Length` é aproximação aceitável para subset minimal. A sintaxe exacta `#set page(width: 50%)` requer parser especial; usar `50%` como operador é suficiente.

**Alternativa:** Se o parser já suporta `50%` como literal relativo (não operador), reaproveitar. A sonda deve verificar.

### 4. `repr` de `Relative` (`rules/eval/repr.rs`)

```rust
Value::Relative(rel) => {
    let pct = rel.rel * 100.0;
    if rel.abs.is_zero() {
        format!("{}%", pct).into()
    } else {
        format!("{}% + {:?}", pct, rel.abs).into()
    }
}
```

### 5. Cast de `Relative` para `Length` (`rules/eval/cast.rs`)

```rust
// Cast implícito de Relative para Length requer contexto (não disponível em eval puro).
// No eval: Relative permanece como Relative.
// No layout: resolve com contexto (width do container, etc.).

impl Cast<Length> for Value {
    fn cast(self) -> Result<Length, Error> {
        match self {
            Value::Length(l) => Ok(l),
            Value::Relative(r) => Err(Error::NeedsContext("Relative length needs layout context to resolve")),
            _ => Err(Error::TypeMismatch),
        }
    }
}
```

### 6. Operações aritméticas com `Relative` (`rules/eval/ops.rs`)

```rust
// Relative + Length → Relative (abs += Length)
// Relative + Relative → Relative (rel += rel, abs += abs)
// Relative * Float → Relative
// Relative / Float → Relative

match (lhs, rhs) {
    (Value::Relative(a), Value::Relative(b)) => Value::Relative(a + b),
    (Value::Relative(mut r), Value::Length(l)) | (Value::Length(l), Value::Relative(mut r)) => {
        r.abs = r.abs + l;
        Value::Relative(r)
    }
    (Value::Relative(r), Value::Float(f)) | (Value::Float(f), Value::Relative(r)) => {
        Value::Relative(r * f)
    }
    // ...
}
```

### 7. Tests — 8 testes

- **L1:** `50%` eval → `Value::Relative(Rel { rel: 0.5, abs: zero })`.
- **L1:** `100% - 1em` eval → `Rel { rel: 1.0, abs: -1em }`.
- **L1:** `50% + 2cm` eval → `Rel { rel: 0.5, abs: 2cm }`.
- **L1:** `repr(50%)` → `"50%"`.
- **L1:** `repr(50% + 2cm)` → `"50% + 2cm"`.
- **L1:** `50% * 2` → `100%`.
- **L2:** `Relative + Length` → `Relative` com offset.
- **L2:** Cast `Relative` para `Length` sem contexto → erro `NeedsContext`.

### 8. Spec L0

- `entities/rel.md` — `Rel<T>` struct, operações, `resolve`.
- `entities/value.md` — `Value::Relative` variant.
- `rules/eval/ops.md` — `%` como relativo quando aplicado a `Length`.

---

## Scope-out explícito

- **`Fr` (fração)** — `1fr`, `2fr` em grids. Requer infraestrutura de distribuição de espaço em layout. Trilha 7.
- **Resolução de `Relative` em layout** — `block(width: 50%)` resolve para metade do container. Requer passar contexto (largura do container) do layout para o eval. Trilha 7.
- **`Relative` em `page` (margins, width, height)** — requer layout engine com contexto de página. Trilha 7.
- **`Relative` com unidades diferentes no `abs`** — `50% + 2cm` é aceitável; `50% + 2em` também. `50% + 2pt` também. Misturas mais complexas são suportadas via `Length`.
- **Sintaxe `50%` como literal (não operador)** — se o parser já suporta, reaproveitar. Se não, operador `%` sobre `Length` é aceitável.
- **`Rel<Abs>`** — apenas `Rel<Length>` neste passo.
- **Comparação de `Relative`** (`<`, `>`, `==`) — requer contexto para resolver. Scope-out; apenas `==` entre dois `Relative` (comparação estrutural).

---

## Critério de fecho

- [ ] `Rel<T>` struct implementado em `entities/rel.rs` (genérico, instanciado para `Length`).
- [ ] `Value::Relative(Rel<Length>)` adicionado ao enum `Value`.
- [ ] Eval de `%` como relativo quando aplicado a `Length`.
- [ ] Operações aritméticas `+`, `-`, `*`, `/` com `Relative`.
- [ ] `repr` de `Relative` implementado.
- [ ] Cast de `Relative` para `Length` retorna erro `NeedsContext` (sem contexto).
- [ ] 8 tests verdes (5 L1 + 2 L2 + 1 L3).
- [ ] Spec L0 atualizada (`entities/rel.md`, `entities/value.md`, `rules/eval/ops.md`).
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations.
- [ ] **Trilha 8: 3/8 completo** (repr P465 + métodos P466 + Relative P469).

---

## Próximo passo (Trilha 8 continua ou pivot)

- **P470** — `pad`/`corners`/`sides` inset modeling (Trilha 8, S, ~15 min)
- **P470** — Parâmetros configuráveis de `sub`/`super`/`highlight`/decorações (Trilha 8, S, ~15 min)
- **P470** — `Symbol` refinado / `Value::Symbol` (Trilha 8, S, ~15 min)
- **P470** — Back-references + `ibid`/`op. cit.` (Trilha 6, S-M, ~30 min)
- **P470** — `#show regex(...)` split do trecho casado (Trilha 3, M, ~30 min)

**Aguardando sua indicação:**

1. **Executar o P469** (`Rel<Length>`, ~15 min)?
2. **Escrever o P470** (próximo passo: pad/corners/sides, parâmetros decorações, Symbol, back-references, ou regex split)?
3. **Ajustar o escopo** do P469?

---

## Estado pós-P468 (para referência)

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
| **P465** | `repr()` completo | ✅ FECHADO | 8 |
| **P466** | Métodos array/dict/str | ✅ FECHADO | 8 |
| **P467** | Sonda `Selector::Where` | ✅ FECHADO | 3 |
| **P468** | Bibliografia Fase 2: estilos numéricos | 🔄 EM PREPARAÇÃO | 6 |
| **DEBT-2** | Closures eager | ✅ FECHADO | — |
| **DEBT-9** | Tracking contínuo | ℹ️ Processo | — |

**Inventário de débitos: LIMPO.**  
**Trilha 1: COMPLETA E COERENTE.**  
**Trilha 2: COMPLETA.**  
**Trilha 3: 1/3 completo.**  
**Trilha 8: 2/8 completo.**

