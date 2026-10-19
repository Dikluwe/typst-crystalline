---

# P468 — Bibliografia Fase 2: Estilos numéricos (`[1]`, `[2]`)

> **Passo:** 468  
> **Data:** 2026-06-25  
> **Foco:** Materializar estilos de citação numéricos (`[1]`, `[2]`) para bibliografia, com numeração por ordem de aparição no documento e back-references.  
> **Trilha:** 6 — Bibliografia Fase 2 (desbloqueada por Trilha 1 numeração + Trilha 2 label/ref).  
> **Tipo:** Materialização.  
> **Tamanho:** M (~40 min).  
> **ADR-0117 Cláusula 4:** Reaproveita `CounterRegistry` (P451), `label`/`ref` (P460/P462), `CounterRegistry` com chave `"bibliography"` (P450), e `format_counter` (P451). Não propõe estrutura em elementos existentes sem verificar P365/P450.

---

## Contexto

O Typst vanilla suporta múltiplos estilos de citação bibliográfica:
- **Autor-data** (Fase 1, P388): `(Kirsch, 1973)` — já implementado no cristalino.
- **Numérico** (`[1]`, `[2]`): citação por número, com bibliografia ordenada por ordem de primeira aparição.
- **Alfabético**: ordenado por sobrenome do autor.

O cristalino tem Fase 1 (autor-data + alfabética, P388/P450) mas **não tem** estilo numérico. Este passo materializa o estilo numérico, que é o mais comum em documentos técnicos e científicos.

**Dependências desbloqueadas:**
- Trilha 1 (P451–P459): `CounterRegistry` com numeração por ordem de aparição.
- Trilha 2 (P460–P463): `label`/`ref` para back-references.
- P450: `BibliographyEntry` + parser BibTeX + `BibStore` com chave determinística.

---

## ADR-0108 — Medir antes de decidir

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `BibliographyEntry` existe? | Sim — P450 | ✅ |
| `BibStore` com chave determinística existe? | Sim — P429/P450 | ✅ |
| Parser BibTeX (`parse_bibtex`) existe? | Sim — P450 (L1) | ✅ |
| `CounterRegistry` com chave `"bibliography"`? | Não — nova chave | ❌ |
| `label`/`ref` para back-references? | Sim — P460/P462 | ✅ |
| `Introspector` com ordenação por aparição? | Sim — P451/P454/P456/P459 | ✅ |
| `format_counter` com pattern `"[1]"`? | Sim — P451 | ✅ |
| Estilo de citação configurável? | Parcial — P388 tem `style` na chain | 🟡 |
| Bloqueadores? | Nenhum técnico | ✅ |

**Reclassificação:** M (~40 min; contador de citação + estilo numérico + layout de citação + bibliografia ordenada + back-references + tests).

---

## Toques pontuais

### 1. Chave `"citation"` no `CounterRegistry`/`Introspector`

**Ficheiro:** `rules/introspect.rs` (ou onde `CounterRegistry` é populado)

Adicionar contador de citações no walk de introspecção:

```rust
// No walk de eval/layout, ao encontrar Content::BibliographyRef (ou Content::Ref para refs bibliográficas):
let citation_number = counter_registry.step("citation");
```

**Decisão:** Contador `"citation"` é separado de `"bibliography"` (entradas bibliográficas). Uma entrada pode ser citada múltiplas vezes; cada citação tem um número sequencial de aparição.

### 2. `BibliographyRef` — novo elemento ou reaproveitar `Content::Ref`?

**Decisão:** Reaproveitar `Content::Ref` (P462) com `supplement` indicando tipo de citação. Adicionar campo `style: CitationStyle` ao `RefElem` (ou ler da `StyleChain`).

```rust
pub enum CitationStyle {
    AuthorDate,   // P388 existente
    Numeric,      // NOVO
    Alphabetic,   // P388 existente
}

// RefElem já existe (P462):
pub struct RefElem {
    pub name: EcoString,           // key bibliográfica (ex: "kirsch1973")
    pub supplement: Option<Content>, // texto antes do número
    pub style: Option<CitationStyle>, // NOVO: override local
}
```

**Alternativa rejeitada:** `Content::BibliographyRef` separado — duplicaria `Ref` sem necessidade. `Ref` já resolve para número de elemento; bibliografia é apenas mais um tipo de elemento numerado.

### 3. Estilo numérico — `native_cite` ou `ref`?

O Typst vanilla usa `@key` para citação bibliográfica, que é sugar para `ref` com contexto bibliográfico. No cristalino:

```rust
// Em stdlib, função nativa `cite` (ou reaproveitar `ref` com detecção de bibliografia):
fn native_cite(key: EcoString, style: Option<CitationStyle>) -> Content {
    Content::Ref(RefElem {
        name: key,
        supplement: None,
        style: style.or_else(|| /* ler da chain: bibliography.style */),
    })
}
```

**Decisão:** Adicionar `native_cite` como alias semântico de `ref` para bibliografia. Registar no stdlib como `"cite"`.

### 4. Layout de citação numérica

**Ficheiro:** `engine/layout/ref.rs` (ou `engine/layout/bibliography.rs`)

```rust
// Ao resolver Content::Ref onde o target é uma entrada bibliográfica:
match ref_elem.style {
    Some(CitationStyle::Numeric) | None if chain_has_numeric => {
        let citation_number = introspector.flat_counter_at("citation", location);
        let formatted = format_counter(&[citation_number], "[1]");
        // Renderizar: "[1]" como FrameItem::Text
        FrameItem::Text(TextItem { text: formatted, ... })
    }
    Some(CitationStyle::AuthorDate) | None => {
        // P388 existente: "(Kirsch, 1973)"
        ...
    }
}
```

### 5. Bibliografia ordenada por ordem de aparição

**Ficheiro:** `engine/layout/bibliography.rs` (ou `rules/eval/bibliography.rs`)

```rust
// Ao renderizar Content::Bibliography:
// 1. Coletar todas as entradas do BibStore.
// 2. Ordenar por ordem de primeira citação (via Introspector.counter_appearance_order("citation", entry.key)).
// 3. Atribuir números sequenciais: 1, 2, 3...
// 4. Renderizar cada entrada como: "[1] Kirsch, R. (1973). ..."
```

**Nota:** A ordenação por ordem de aparição requer que o `Introspector` mantenha não apenas o counter flat, mas a **ordem de primeira aparição** de cada entrada. Isso pode ser inferido do `CounterRegistry` se cada citação incrementa o counter e o `Introspector` registra qual entrada foi citada em cada passo.

**Implementação:** Adicionar `citation_order: Vec<EcoString>` ao `Introspector` — append da key da entrada quando primeiro citada.

### 6. Back-references ("ver [3]")

**Scope-out para P469:** Back-references requerem que cada entrada bibliográfica saiba quais citações a referenciam. Isso é um mapa inverso (`entry_key -> Vec<citation_number>`). Materializável, mas complexidade adicional. Scope-out neste passo para manter M em ~40 min.

### 7. Tests

- **L1 (eval):** 2 testes — `native_cite("kirsch1973")` emite `Content::Ref` com style `Numeric`; `cite` com `style: AuthorDate` preserva comportamento P388.
- **L2 (layout):** 3 testes — citação numérica renderiza `"[1]"`; segunda citação da mesma entrada renderiza `"[1]"` (não incrementa); citação de entrada diferente renderiza `"[2]"`.
- **L3 (E2E):** 2 testes — documento com 2 entradas BibTeX, citação em ordem invertida (entry B antes de A), bibliografia ordenada por aparição (B=[1], A=[2]).

### 8. Spec L0

- `rules/stdlib/bibliography.md` — `cite(key, style?)` com tabela de estilos.
- `engine/layout/bibliography.md` — layout de citação numérica e bibliografia ordenada.
- `entities/bibliography.md` — `CitationStyle` enum.

---

## Scope-out explícito

- **Back-references** (`"ver [3]"`) — P469.
- **`ibid` / `op. cit.`** — P469 ou posterior.
- **Múltiplas bibliografias num documento** — P420 scope-out.
- **CSL-JSON** — P450 scope-out; apenas BibTeX.
- **Estilos personalizados (CSL)** — scope-out; apenas built-in: AuthorDate, Numeric, Alphabetic.
- **Citação de página específica** (`@kirsch1973[p. 42]`) — scope-out; requer argumento adicional em `cite`.
- **Citação múltipla** (`@kirsch1973 @other2020`) — scope-out; `cite` recebe uma key.
- **Ordenação alfabética numérica** (alguns estilos numéricos ordenam alfabeticamente) — scope-out; apenas ordem de aparição.

---

## Critério de fecho

- [ ] `CounterRegistry`/`Introspector` com chave `"citation"` populado no walk.
- [ ] `Introspector` mantém `citation_order: Vec<EcoString>` para ordenação por aparição.
- [ ] `RefElem` ganha campo `style: Option<CitationStyle>` (ou lê da `StyleChain`).
- [ ] `native_cite` registada no stdlib como `"cite"`.
- [ ] Layout de `Content::Ref` com style `Numeric` renderiza `"[N]"`.
- [ ] Bibliografia ordenada por ordem de primeira citação.
- [ ] 7 tests verdes (2 L1 + 3 L2 + 2 L3).
- [ ] Spec L0 atualizada (3 prompts).
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations.
- [ ] **Trilha 6: 1/5 completo** (Fase 2: estilos numéricos).

---

## Próximo passo (Trilha 6 continua ou pivot)

- **P469** — Back-references (`"ver [3]"`) e `ibid`/`op. cit.` (Trilha 6, S-M, ~30 min)
- **P469** — List of Figures / List of Tables (Trilha 6, S-M, ~25 min)
- **P469** — `Relative` (Rel<Length>) (Trilha 8, S, ~15 min)
- **P469** — `pad`/`corners`/`sides` (Trilha 8, S, ~15 min)
- **P469** — `#show regex(...)` split do trecho casado (Trilha 3, M, ~30 min)

**Aguardando sua indicação:**

1. **Executar o P468** (bibliografia estilos numéricos, ~40 min)?
2. **Escrever o P469** (próximo passo: back-references, LoF/LoT, tipos Value, ou regex split)?
3. **Ajustar o escopo** do P468?

---

## Estado pós-P467 (para referência)

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
| **DEBT-2** | Closures eager | ✅ FECHADO | — |
| **DEBT-9** | Tracking contínuo | ℹ️ Processo | — |

**Inventário de débitos: LIMPO.**  
**Trilha 1: COMPLETA E COERENTE.**  
**Trilha 2: COMPLETA.**  
**Trilha 3: 1/3 completo.**  
**Trilha 8: 2/8 completo.**

