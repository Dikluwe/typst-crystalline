---

# P467 — Sonda e materialização de `Selector::Where`

> **Passo:** 467  
> **Data:** 2026-06-25  
> **Foco:** (1) Verificar estado real de `Selector::Where` no código (contradição entre inventário "ausente" e P417/P423); (2) Se ausente, materializar `Selector::Where` para `#show elem.where(field: value)`; (3) Se parcial, completar.  
> **Trilha:** 3 — Selectors completos em show rules.  
> **Tipo:** Sonda + Materialização condicional.  
> **Tamanho:** S (~15 min) se parcial; S-M (~25 min) se ausente.  
> **ADR-0117 Cláusula 4:** Verificar decisões de fronteira sobre `Selector` antes de propor estrutura; P417/P423 já mexeram em `Selector`.

---

## Contexto

O Typst vanilla suporta `#show heading.where(level: 1): ...` para aplicar show rules condicionalmente a elementos com campos específicos. O cristalino tem `Selector` (usado em show rules), mas o inventário de cobertura (P447) marca `Selector::Where` como **ausente** (linha 111: `.where(field:)` precisa `Selector::Where`).

No entanto, **P417 e P423 mexeram em `Selector`** — adicionaram variants e lógica de matching. A sonda A.0 deve resolver essa contradição: `Selector::Where` existe parcialmente, está incompleto, ou realmente não existe?

Este passo é **independente** de P466 (métodos de array/dict/str) — toca em `entities/selector.rs`, `rules/eval/show.rs`, e `rules/stdlib/query.rs`, não em `Value::method` ou tipos de coleção.

---

## ADR-0108 — Medir antes de decidir

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `Selector` enum existe? | Sim — usado em show rules | ✅ |
| `Selector::Where` existe? | **A verificar** — grep em `entities/selector.rs` | 🟡 |
| `Selector::Elem` existe? | Sim — matching por tipo de elemento | ✅ |
| `Selector::Regex` existe? | Sim — P393 | ✅ |
| `show` rule com `where` funciona? | **A verificar** — teste E2E | 🟡 |
| `query` com `where` funciona? | **A verificar** | 🟡 |
| P417/P423 alteraram `Selector`? | Sim — verificar diff | ✅ |
| Bloqueadores? | Nenhum até sonda completar | 🟡 |

**Sonda a executar:**

```bash
grep -n "Where" 01_core/src/entities/selector.rs
grep -rn "Selector::Where" 01_core/src/
grep -rn "\.where(" 01_core/src/engine/eval/show.rs
grep -rn "where" 01_core/src/engine/stdlib/query.rs
grep -rn "where" 01_core/src/engine/eval/tests.rs | grep -i selector
```

**Reclassificação condicional:**
- **Se `Selector::Where` já existe e funciona:** XS (~5 min) — atualizar inventário, marcar como presente, zero código.
- **Se `Selector::Where` existe mas incompleto:** S (~15 min) — completar match arms.
- **Se `Selector::Where` não existe:** S-M (~25 min) — materializar do zero.

---

## Toques pontuais (condicional — aplicar conforme resultado da sonda)

### Caso A: `Selector::Where` já existe (XS, ~5 min)

**Acção:**
- Atualizar inventário de cobertura: `Selector::Where` = presente.
- Verificar se tests existem; se não, adicionar 1 teste L2.
- Fechar passo.

### Caso B: `Selector::Where` existe mas incompleto (S, ~15 min)

**Ficheiros a verificar:**
- `entities/selector.rs` — `Selector::Where` com campos/estrutura parcial.
- `rules/eval/show.rs` — matching de `Selector::Where` contra `Content`.
- `rules/stdlib/query.rs` — `query` com `where`.

**Completar:**

```rust
// entities/selector.rs
pub enum Selector {
    Elem(ElementKind),
    Regex(Regex),
    Where(ElementKind, Vec<(EcoString, Value)>),  // completar se parcial
    // ...
}
```

Matching em `show.rs`:
```rust
Selector::Where(kind, fields) => {
    if content.kind() != *kind { return false; }
    for (field_name, expected_value) in fields {
        let actual = content.field(field_name)?;
        if actual != *expected_value { return false; }
    }
    true
}
```

### Caso C: `Selector::Where` não existe (S-M, ~25 min)

**1. Entidade `Selector::Where`** (`entities/selector.rs`)

```rust
pub enum Selector {
    Elem(ElementKind),
    Regex(Regex),
    Where(WhereSelector),  // NOVO
    // ...
}

pub struct WhereSelector {
    pub kind: ElementKind,           // Tipo de elemento (heading, figure, etc.)
    pub fields: Vec<FieldFilter>,    // Filtros de campo
}

pub struct FieldFilter {
    pub name: EcoString,   // Nome do campo (ex: "level", "numbering")
    pub value: Value,      // Valor esperado
}
```

**2. Sintaxe `elem.where(field: value)`** (`rules/stdlib/query.rs` ou `rules/eval/show.rs`)

```rust
fn native_where(
    kind: ElementKind,           // Inferido do contexto (ex: heading)
    fields: Vec<(EcoString, Value)>,
) -> Selector {
    Selector::Where(WhereSelector {
        kind,
        fields: fields.into_iter().map(|(name, value)| FieldFilter { name, value }).collect(),
    })
}
```

**Nota:** O Typst vanilla usa `heading.where(level: 1)` como método em `ElementKind` (ou função associada). No cristalino, pode ser modelado como função nativa `where(kind, ...fields)` ou como método em `Type`.

**3. Matching em show rules** (`rules/eval/show.rs`)

```rust
fn selector_matches(selector: &Selector, content: &Content) -> bool {
    match selector {
        Selector::Elem(kind) => content.kind() == *kind,
        Selector::Regex(re) => content.plain_text().map(|t| re.is_match(&t)).unwrap_or(false),
        Selector::Where(where_sel) => {
            if content.kind() != where_sel.kind { return false; }
            for filter in &where_sel.fields {
                let actual = content.field(&filter.name);
                match actual {
                    Ok(val) if val == filter.value => continue,
                    _ => return false,
                }
            }
            true
        }
        // ... outros selectors
    }
}
```

**4. `Content::field`** — helper para introspecção de campos

```rust
impl Content {
    pub fn field(&self, name: &str) -> Result<Value, Error> {
        match self {
            Content::Heading(h) => match name {
                "level" => Ok(Value::Int(h.level as i64)),
                "body" => Ok(Value::Content(h.body.clone())),
                _ => Err(Error::UnknownField(name.into())),
            },
            Content::Figure(f) => match name {
                "caption" => Ok(f.caption.as_ref().map(|c| Value::Content(c.clone())).unwrap_or(Value::None)),
                "numbering" => Ok(f.numbering.as_ref().map(|n| Value::Str(n.clone())).unwrap_or(Value::None)),
                _ => Err(Error::UnknownField(name.into())),
            },
            // ... outros elementos
            _ => Err(Error::NotAnElement),
        }
    }
}
```

**5. Tests**

- **L1:** 2 testes — `Selector::Where` constrói com fields; `Selector::Where` não matcha kind errado.
- **L2:** 2 testes — `heading.where(level: 1)` matcha `Heading { level: 1 }`; não matcha `Heading { level: 2 }`.
- **L3:** 1 teste — `#show heading.where(level: 1): set text(red)` aplica apenas a headings nível 1.

**6. Spec L0**

- `entities/selector.md` — `Selector::Where`, `WhereSelector`, `FieldFilter`.
- `rules/eval/show.md` — matching de `Selector::Where` em show rules.
- `rules/stdlib/query.md` — `where(kind, fields)` ou `elem.where(...)`.

---

## Scope-out explícito

- **`#show regex(...)` split do trecho casado** — P393 scope-out; não é este passo.
- **Nested `where` (`.where(a: 1).where(b: 2)`)** — scope-out; apenas um nível.
- **`where` com operadores de comparação (`>`, `<`, `!=`)** — scope-out; apenas igualdade.
- **`where` em `query` com localização** — scope-out; `query` básico.
- **`where` com `and`/`or` lógicos** — scope-out; apenas conjunção (AND implícito).
- **Campos computados ou dinâmicos** — scope-out; apenas campos estáticos do elemento.

---

## Critério de fecho

- [ ] Sonda A.0 executada com evidência (grep output anexado).
- [ ] Estado real de `Selector::Where` documentado no relatório.
- [ ] Se Caso A: inventário atualizado, 1 teste adicionado.
- [ ] Se Caso B: `Selector::Where` completado, match arms adicionados, 4 tests.
- [ ] Se Caso C: `Selector::Where` materializado do zero, `Content::field` implementado, 5 tests, specs L0.
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations.
- [ ] **Trilha 3: 1/3 completo** (`Selector::Where` fechado).

---

## Próximo passo (Trilha 3 continua ou pivot)

- **P468** — `#show regex(...)` split do trecho casado (P393 scope-out, M, ~30 min)
- **P468** — `Relative` (Rel<Length>) (Trilha 8, S, ~15 min)
- **P468** — `pad`/`corners`/`sides` (Trilha 8, S, ~15 min)
- **P468** — Bibliografia Fase 2: estilos numéricos (Trilha 6, M, ~40 min)

**Aguardando sua indicação:**

1. **Executar o P467** (sonda + materialização `Selector::Where`, ~15-25 min)?
2. **Escrever o P468** (próximo passo: regex split, tipos Value, ou bibliografia)?
3. **Ajustar o escopo** do P467?

---

## Estado pós-P466 (para referência)

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
| **P466** | Métodos array/dict/str | 🔄 EM EXECUÇÃO | 8 |
| **DEBT-2** | Closures eager | ✅ FECHADO | — |
| **DEBT-9** | Tracking contínuo | ℹ️ Processo | — |

**Inventário de débitos: LIMPO.**  
**Trilha 1: COMPLETA E COERENTE.**  
**Trilha 2: COMPLETA.**  
**Trilha 8: 1/8 completo (P465); P466 em execução.**

