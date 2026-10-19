# Relatório consolidado — Série P193

**Período**: 2026-05-04 (P193A diagnóstico + P193B implementação)
**Magnitude agregada**: S
**Estado**: ✅ Série fechada (A ✅ B ✅) — **passo 1 da
sequência §9 P189 consolidado**
**ADR vinculada**: nenhuma
**DEBT**: M5-residual avança 1 dos 4 pré-requisitos

---

## §1 Resumo executivo

Sub-store **`ResolvedLabelStore`** aberto em `TagIntrospector`
— infraestrutura para mapeamento Label → texto resolvido.
Replica padrão BibStore (P181B) com adaptação a estrutura
mais simples (`HashMap<Label, String>`).

Adições materializadas:
- Struct `ResolvedLabelStore` em `01_core/src/entities/resolved_label_store.rs`
  com 5 métodos (`empty`, `get`, `len`, `is_empty`,
  `insert(pub(crate))`).
- Field `pub resolved_labels: ResolvedLabelStore` em
  `TagIntrospector` (paralelo a `bib_store`).
- Método trait `Introspector::resolved_label_for(&Label) -> Option<&str>`.
- L0 nova `entities/resolved_label_store.md`.
- L0 actualizada `entities/introspector.md` (field +
  método + Histórico).

Δ tests cumulativo: **+6** (1815 → 1821) com **zero
regressões**.

**Sub-store vazio em produção** durante janela compat M5 —
walks legacy E2/E4 (P189B) continuam a popular
`state.resolved_labels`; consumer C4 lê de legacy. Activação
em P194 (consumer migration) + P195 (walk arm Labelled
migrado).

P193 é **passo 1 dos 7** identificados em P189 §9 para
fechar M5 universalmente. **3 dos 4 pré-requisitos**
restam.

---

## §2 Sub-passos materializados

| Passo | Magnitude planeada | Magnitude real | Δ tests | L0s tocados |
|-------|---------------------|-----------------|---------|-------------|
| **P193A** | S (diagnóstico) | S | 0 | nenhum |
| **P193B** | S (agregado) | S | **+6** | `entities/resolved_label_store.md` (nova) + `entities/introspector.md` |
| **Total** | — | — | **+6** | 2 L0s |

P193B agregou em sub-passo único:
- `.A` auditoria (template BibStore).
- `.B` criar struct `ResolvedLabelStore` + entities/mod.rs.
- `.C` criar L0 `entities/resolved_label_store.md`.
- `.D` adicionar field a `TagIntrospector`.
- `.E` adicionar método trait + impl.
- `.F` actualizar L0 introspector.
- `.G` 6 tests unit (4 ResolvedLabelStore + 2 trait).
- `.H` verificação estrutural (14/14).
- `.I` actualizar nota DEBT M5-residual.
- `.J` relatório consolidado P193 (este ficheiro).

---

## §3 Decisões arquiteturais

### 8 cláusulas P193A fechadas

| # | Cláusula | Decisão | Sub-passo |
|---|----------|---------|-----------|
| 1 | Forma estrutural | **Opção α** struct dedicado mínimo `ResolvedLabelStore { labels: HashMap<Label, String> }` | P193B `.B` |
| 2 | Localização | **Opção 1** field directo paralelo a `bib_store` | P193B `.D` |
| 3 | Populate em from_tags | **Opção A** sem arm em P193 (P195 adiciona) | P193B `.B` |
| 4 | API trait | **Opção α** `resolved_label_for(&Label) -> Option<&str>` | P193B `.E` |
| 5 | Location-awareness | **Sem variante `*_at`** (snapshot final) | P193B `.E` |
| 6 | Compat com legacy | Independência total durante janela compat | P193B `.B/.C/.F` |
| 7 | Tipo de chave | **`Label`** (replica legacy) | P193B `.B` |
| 8 | Critério fecho | Struct + field + método trait + tests + L0s | P193B `.J` |

### Sem ADR — replicação de padrão

Padrão P181B BibStore + P181F método trait replicados
literalmente. Decisões registadas em P193A §2.

---

## §4 Achados não-triviais durante execução

### P193A §11.1 — Decisão pré-existente preservada (HashMap vs Locator/Resolved vanilla)

Vanilla typst usa `Locator::resolve() -> Resolved` (struct
rico com path, key, content reference). Cristalino
simplificou desde P165 para `HashMap<Label, String>` em
`CounterStateLegacy.resolved_labels`. P193 mantém a
simplificação — `ResolvedLabelStore` é **materialização
da decisão pré-existente, não decisão nova**.

Sem ADR necessário.

### P193A §11.2 — Auto-toc + explicit unificados em mesmo HashMap

`state.resolved_labels` é populated por **2 caminhos**:
1. `Content::Heading` arm: `auto-toc-N` → `"Secção {prefix}"`.
2. `Content::Labelled` arm: label explícita → texto.

Ambos populam o **mesmo HashMap**. `ResolvedLabelStore`
preserva esta semântica — chave `Label` (incluindo
auto-toc-N) → valor `String`. Sem distinção arquitectural
entre auto-toc e explicit.

### P193A §11.3 — Consumer C4 simples (fallback trivial em P194)

`references.rs:53`:
```rust
let display_text = match layouter.counter.resolved_labels.get(target) {
    Some(text) => text.clone(),
    None       => format!("@{}", target.0),
};
```

Sem lógica condicional complexa. P194 migration:
substitution-with-fallback per padrão P184D/P187B/P188B —
trivial.

### P193A §11.4 — Sem variante `*_at` (snapshot final)

Análise dos 2 eixos confirmou: eixo 1 = snapshot final
(consumer lê durante layout, após walk completo). Diferente
de P185B (counter location-aware) que precisaram porque
counter muda durante walk.

`resolved_labels` é **write-once por label durante walk**.
API simples `Option<&str>` (sem Location parameter).

### P193A §11.5 — Sub-store vazio em produção (janela compat)

P193 abre **infra**. Não popula em produção. Walks E2/E4
continuam a popular legacy directamente.

Tests unit em P193B populam manualmente
(`intr.resolved_labels.insert(...)`) para validar API.
Populate via `from_tags` vem em P195.

Esta janela compat é **honestidade obrigatória** —
documentada em L0 `resolved_label_store.md` §"Estado em
P193B" + comentário inline na struct + relatório
consolidado §5.

---

## §5 Estado final M9 e M5

### M9 (counter-feature) — inalterado: 11/11

P193 não introduz feature M9 nova.

### M5 — **incremental**: 1 arm migrado + 6 excepções (inalterado vs P189B)

Cadeia de pré-requisitos para fechar excepções E2-E6:

**Antes de P193**: 4 pré-requisitos pendentes.
**Após P193B**: **3 pré-requisitos** restantes:
1. ~~Sub-store `resolved_labels`~~ ✅ aberto em P193B.
2. C4 migration — P194.
3. Sub-store `headings_for_toc` — passo dedicado.
4. `Content::SetEquationNumbering` — passo independente.

E1 (Equation) continua bloqueado por SetEquationNumbering
(independente de P193).

### Trait `Introspector` — 18 → **19 métodos**

Método novo: `resolved_label_for(&Label) -> Option<&str>`.

### `TagIntrospector` sub-stores — 7 → **8**

Sub-store novo: `resolved_labels: ResolvedLabelStore`
paralelo a `bib_store: BibStore`.

### M5/M4 (read-sites) — inalterado: 8/12

P193 não migra read-sites — abre infraestrutura.

### `entities/mod.rs` — `pub mod resolved_label_store;`

Adicionado.

---

## §6 Estado final lacunas

- **Lacuna #3** (`headings_for_toc` sub-store): activa
  ainda. Independente de P193 — passo dedicado fechará.
- Outras lacunas: inalteradas.

---

## §7 Pendências cumulativas + DEBT M5-residual

### DEBT M5-residual (Cenário B)

**Sem DEBT formal aberto**. Nota actualizada:

> Antes P193: 4 pré-requisitos pendentes para fechar
> cadeia E2-E6.
>
> **Após P193B: 3 pré-requisitos restantes** (1 dos 4
> avançado).
> 1. ~~Sub-store `resolved_labels`~~ ✅ P193B.
> 2. C4 migration — P194 (próximo).
> 3. Sub-store `headings_for_toc` — passo dedicado.
> 4. `Content::SetEquationNumbering` — passo independente.
>
> Cadeia E2-E6 fica num passo mais perto de desbloqueio.
> Próximo: P194 (C4 migration) com substitution-with-fallback
> trivial — blueprint P184D/P187B/P188B aplicável.

DEBT M5-residual continua em Cenário B.

---

## §8 Próximos passos sugeridos

### Sequência continua (per P189 §9)

1. **P194 — C4 migration**: consumer Ref-arm em
   `layout/references.rs:53` migra para
   `intr.resolved_label_for(target).or_else(|| layouter.counter.resolved_labels.get(target).map(String::as_str))`.
   Magnitude S esperada — replica P184D/P187B/P188B
   substitution-with-fallback.

2. **P195** — migrar walk arm `Labelled` para emitir
   `ElementPayload::Labelled` (a criar) ou usar mecanismo
   alternativo para popular sub-store via `from_tags`.
   Magnitude S–M (depende de decisão sobre payload).

3. **P196** — migrar walk arm `Heading` (auto-toc).

4. **P197** — migrar walk arm `Figure`.

5. **P198** — migrar walks `SetHeadingNumbering` +
   `CounterUpdate`.

6. (Independente) — `Content::SetEquationNumbering`
   materialização.

Após sequência: M5 universalmente fechado; segue M6
(eliminação `CounterStateLegacy`).

### Independente

- Sub-store `headings_for_toc` (lacuna #3) — passo
  dedicado quando convier.
- M9 slot 11 — feature counter nova.

---

## §9 Conclusão

P193 fechou em 2 sub-passos (A diagnóstico + B
implementação agregada) com magnitude correctamente
estimada (S em ambos). Replicação literal de padrão P181B
BibStore com estrutura mais simples (HashMap único vs
2 sub-mapas).

Achados centrais:
- **Replica P181B** literalmente — struct dedicada, field
  paralelo, método trait simples.
- **Sem variante location-aware** — análise dos 2 eixos
  (P193A §1.8) confirmou snapshot final.
- **Sub-store vazio em produção** — honestidade
  documental obrigatória registada em 4 pontos (L0 +
  comentário struct + comentário trait + relatório
  consolidado).
- **P193 desbloqueia P194** (C4 migration) imediatamente.
  Cadeia P195+ ainda exige migração walk arm.

P193 é **passo 1 dos 7** da sequência §9 P189
consolidado. Após sequência completa: M5 universal
fechado; segue M6.

**62 passos executados** após P193B. Padrão
diagnóstico-primeiro mantido — 15/15 acertaram a
magnitude planeada ±1 nível
(P131A/132A/140A/148/154A/181A/182A/183A/184A/185A/186A/
187A/188A/189A/193A).

Próximo passo sugerido: **P194A** (diagnóstico C4
migration) ou directamente **P194B** se decisão for que
P194 é trivial e dispensa diagnóstico-primeiro.
