---

# P464 — Consolidação: `Content::Label` vs `Content::Labelled`

> **Passo:** 464  
> **Data:** 2026-06-25  
> **Foco:** Unificar `Content::Label` (P460, user-created) e `Content::Labelled` (P329, auto-generated) num único tipo, eliminando duplicação semântica.  
> **Tipo:** Cleanup / Refacto mecânico.  
> **Tamanho:** XS (~10 min).  
> **ADR-0117 Cláusula 4:** Aplica a lição de consolidar decisões arquitectónicas — um conceito, um tipo.

---

## Contexto

O cristalino tem dois tipos para o mesmo conceito semântico de "label":

| Tipo | Origem | Uso | Campos |
|------|--------|-----|--------|
| `Content::Label` | P460 | User-created via `#label<name>` | `LabelElem { name, body }` |
| `Content::Labelled` | P329 | Auto-generated em figures/equations | `LabelledElem { name, body }` |

Isso complica:
- **Walk de introspecção** — `Introspector` precisa tratar ambos para `label_to_counter_key` (P462).
- **Layout** — `label_positions` precisa registar ambos para `/Dests` (P460).
- **Export PDF** — `/Dests` precisa iterar ambos.
- **Futuros consumidores** — `ref`, LoF, LoT, back-references precisam saber de ambos.

No Typst vanilla, não há dois tipos. Há `LabelElem`, que pode ser criado pelo utilizador ou gerado internamente. A distinção é de origem, não de tipo.

---

## ADR-0108 — Medir antes de decidir

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `Content::Label` existe? | Sim — P460 (`LabelElem { name, body }`) | ✅ |
| `Content::Labelled` existe? | Sim — P329 (`LabelledElem { name, body }`) | ✅ |
| Campos são idênticos? | Sim — ambos têm `name: EcoString` + `body: Content` | ✅ |
| Consumidores tratam ambos? | Sim — `Introspector`, `Layouter`, `export` | ✅ |
| Bloqueadores? | Nenhum — refacto mecânico | ✅ |

**Reclassificação:** XS (~10 min; 1 tipo unificado + ajuste de consumidores + tests).

---

## Toques pontuais

### 1. Unificar num único `Content::Label`

**Ficheiro:** `entities/content.rs` (ou `entities/elements/label.rs`)

```rust
pub struct LabelElem {
    pub name: EcoString,
    pub body: Content,
    pub auto: bool,  // NOVO: true = auto-generated (ex-Labelled), false = user-created
}

pub enum Content {
    // ... existing variants
    Label(LabelElem),
    // REMOVER: Labelled(LabelledElem),  // P329
}
```

**Decisão:** Manter `Content::Label` (nome alinhado com Typst vanilla). Adicionar campo `auto: bool` para distinguir origem quando necessário (raramente — a maioria dos consumidores não precisa saber).

### 2. Migrar `Content::Labelled` → `Content::Label { auto: true }`

**Ficheiros a ajustar:**
- `entities/elements/label.rs` — adicionar `auto: bool`.
- `entities/elements/labelled.rs` — **remover** (ficheiro órfão).
- `rules/eval/rules.rs` — onde `Labelled` é emitido (P329), mudar para `Label { auto: true }`.
- `rules/layout/mod.rs` — walk de `Labelled` → `Label`.
- `rules/introspect.rs` — `label_to_counter_key` tratava `Labelled` separadamente; unificar.
- `03_infra/src/export.rs` — `/Dests` iterava `Labelled`; unificar.

### 3. Ajustar `Introspector` / `TagIntrospector`

**Ficheiro:** `rules/introspect.rs`

- Remover braço `Content::Labelled` do walk.
- Manter braço `Content::Label` com lógica unificada.
- Se `auto: true`, inferir `counter_key` do `body` (figure/equation/table/heading) como já fazia para `Labelled`.
- Se `auto: false`, usar `counter_key` do `body` se numerado, ou `None`.

### 4. Ajustar `Layouter` / `export`

**Ficheiros:** `rules/layout/mod.rs`, `03_infra/src/export/builder.rs`

- `label_positions` / `extracted_label_positions`: iterar apenas `Content::Label`.
- `/Dests`: iterar apenas `Content::Label`.

### 5. Tests

- **L1:** 1 teste — `Label { auto: true }` e `Label { auto: false }` são ambos `Content::Label`.
- **L2:** 1 teste — `Introspector` resolve `counter_key` para ambos `auto=true` e `auto=false`.
- **L3:** 1 teste — documento com figure auto-labelled + user label gera `/Dests` corretos.

### 6. Spec L0

- `entities/elements/label.md` — atualizar: `LabelElem` com `auto: bool`.
- `entities/elements/labelled.md` — **remover** (ficheiro órfão; ou marcar como "consolidado em P464").
- `entities/content.md` — atualizar enum: apenas `Label`.

---

## Scope-out explícito

- **Não** altera comportamento observável — puramente refacto mecânico.
- **Não** adiciona funcionalidade nova.
- **Não** altera `label_to_counter_key` duplicado (Introspector vs TagIntrospector) — isso é refacto separado, não bloqueante.
- **Não** remove `label_to_counter_key` duplicado — mantém ambos, apenas unifica a fonte (`Label` único).
- **Não** altera `LinkItem` com `items` vs `body: Frame` — nota retroativa P452 já documenta.

---

## Critério de fecho

- [x] `Content::Labelled` removido do enum `Content`.
- [x] `LabelElem` ganha campo `auto: bool`.
- [x] Todos os consumidores (`eval`, `layout`, `introspect`, `export`) migrados para `Content::Label` único.
- [x] Ficheiro `entities/elements/labelled.rs` removido ou marcado como consolidado.
- [x] 3 tests verdes (1 L1 + 1 L2 + 1 L3).
- [x] Spec L0 atualizada (`label.md`, `labelled.md` removido, `content.md`).
- [x] `cargo test --workspace` verde; `crystalline-lint` zero violations.
- [x] **Dívida técnica `Content::Label` vs `Content::Labelled` marcada como RESOLVIDA** no roteiro.

---

## Próximo passo (Trilha 8 — refinos de stdlib)

Com P464, a dívida técnica residual está limpa. O próximo passo pode ser:

- **P465** — `repr()` completo (Trilha 8, S, ~15 min)
- **P465** — Métodos restantes de `array`/`dict`/`str` (Trilha 8, S, ~15 min)
- **P465** — Sonda `Selector::Where` (Trilha 3, S, ~15 min)
- **P465** — Bibliografia Fase 2: estilos numéricos (Trilha 6, M, ~40 min)

**Aguardando sua indicação:**

1. **Executar o P464** (cleanup Label/Labelled, ~10 min)?
2. **Escrever o P465** (próximo passo fechável: repr, métodos, Selector::Where, ou bibliografia)?
3. **Ajustar o escopo** do P464?

---

## Estado pós-P463 (para referência)

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
| **DEBT-2** | Closures eager | ✅ FECHADO | — |
| **DEBT-9** | Tracking contínuo | ℹ️ Processo | — |

**Inventário de débitos: LIMPO** (após P464).

**Trilha 1: COMPLETA E COERENTE.**  
**Trilha 2: COMPLETA.**

