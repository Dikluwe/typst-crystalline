# Relatório P193a — Diagnóstico sub-store `resolved_labels`

**Data**: 2026-05-04
**Magnitude**: S (diagnóstico puro)
**Pré-condição**: M5 incremental fechado em P189B; tests
workspace 1.815 verdes; zero violations.

---

## §1 Escopo

P193A é o passo de diagnóstico-primeiro que precede a
abertura do sub-store `resolved_labels`. Replica registo
de P181A/P182A/P183A/P184A/P185A/P186A/P187A/P188A/P189A.

P193 é **passo 1 da sequência de 7 passos** identificada
em P189 §9 para fechar M5 universalmente.

P193A produziu 2 artefactos documentais e zero código:

- `00_nucleo/diagnosticos/diagnostico-resolved-labels-store-passo-193a.md` (8 secções).
- `00_nucleo/materialization/typst-passo-193a-relatorio.md` (este, 14 secções).

Sem ADR. Sem DEBT formal.

---

## §2 Inputs verificados empiricamente (8+ greps + reads)

| # | Input | Resultado |
|---|-------|-----------|
| 1 | `state.resolved_labels` tipo | `HashMap<Label, String>` (`counter_state_legacy.rs:37`) |
| 2 | `Label` definição | `pub struct Label(pub String)` (newtype; `entities/label.rs:12`) |
| 3 | Arms que populam | `Heading` arm (auto-toc) + `Labelled` arm (explicit) — ambos excepções E2/E4 P189B |
| 4 | Consumer C4 | `references.rs:53` `layouter.counter.resolved_labels.get(target)` em `layout_ref` |
| 5 | Copy-sites Layouter | `mod.rs:1481, 1512` (initial_state copiado) |
| 6 | Vanilla typst | `Locator::resolve() -> Resolved` rico; cristalino simplifica para HashMap (decisão pré-existente) |
| 7 | Tests existentes | `layout_resolved_labels_nao_interfere_entre_documentos` + sentinelas P189B E2/E4 |
| 8 | Análise dos 2 eixos | Eixo 1 = snapshot final; Eixo 2 = sim em produção (após P195) |

Conclusão central: sub-store **sem variante location-aware**
necessária. Diferente de P185B (`*_at` métodos) — `resolved_labels`
é write-once durante walk; consumer lê após walk completo.

---

## §3 Decisões cláusulas 1–8 (resumo)

| # | Cláusula | Decisão |
|---|----------|---------|
| 1 | Forma estrutural | **Opção α** struct dedicado mínimo `ResolvedLabelStore { labels: HashMap<Label, String> }` |
| 2 | Localização | **Opção 1** field directo paralelo a `bib_store: BibStore` |
| 3 | Populate em `from_tags` | **Opção A** sem arm em P193 (P195 adiciona) |
| 4 | API trait | **Opção α** `resolved_label_for(&self, label: &Label) -> Option<&str>` |
| 5 | Location-awareness | **Sem variante `*_at`** (snapshot final) |
| 6 | Compat com legacy | Independência total durante janela compat; transição em 5 passos |
| 7 | Tipo de chave | **`Label`** (replica legacy) |
| 8 | Critério fecho | Struct + field + método trait + tests unit + L0s |

---

## §4 Plano de sub-passos B (sem condicionais)

**Sub-passo único agregado**:

| Sub-passo | Escopo | Magnitude |
|-----------|--------|-----------|
| `.B` | Struct `ResolvedLabelStore` + L0 + field em `TagIntrospector` + método trait + 4 tests unit + nota DEBT + relatório consolidado P193 | S |

---

## §5 Magnitude agregada

**P193 série = S puro** (1×S agregado em sub-passo único).

Idêntico em magnitude a P187/P188 (sub-passo único após
diagnóstico). Replicação de padrão P181B BibStore com
adaptação a estrutura mais simples.

---

## §6 Dependências de outros passos

### §6.1 — Pré-requisitos (cumpridos)

- M5 incremental fechado em P189B (1 arm migrado + 6
  excepções).
- Trait `Introspector` 18 métodos (P185B).
- `TagIntrospector` com 7 sub-stores (P181/P184/P186).
- Padrão P181B BibStore como template.

### §6.2 — Dependentes (P193 desbloqueia)

- **P194** (C4 migration) — consumer Ref-arm pode adoptar
  `intr.resolved_label_for(label).or_else(legacy)`.
- **P195** (migrar walk Labelled) — `from_tags` arm pode
  popular `intr.resolved_labels`.
- Cadeia E2-E6 desbloqueada incrementalmente.

### §6.3 — Independente

- **`Content::SetEquationNumbering`** materialização —
  paralelo; fecha E1.
- Sub-store `headings_for_toc` (lacuna #3) — passo
  paralelo.

---

## §7 ADR avaliação

**Sem ADR criada.** Padrão P181B BibStore replicado.
Padrão P181F método trait replicado. Sem semântica nova;
sem decisão arquitectural disruptiva.

§1.4 (vanilla typst) confirmou que cristalino simplifica
para HashMap por design — sem nuance que afecte forma do
sub-store.

---

## §8 DEBT avaliação (M5-residual progresso)

### Cenário B (continuação)

**Sem DEBT formal aberto**. Nota actualizada:

- **Antes P193**: 4 pré-requisitos pendentes para fechar
  cadeia E2-E6.
- **Após P193B**: **3 pré-requisitos** restantes (1 dos
  4 avançado).
- Cadeia E2-E6 fica num passo mais perto de desbloqueio.

DEBT M5-residual continua em Cenário B (sem formal; notas
preventivas).

---

## §9 Restrições honradas

- **Zero código tocado**.
- **Zero testes** modificados.
- **Não modifica trait `Introspector`** — P193B.
- **Não modifica `TagIntrospector`** — P193B.
- **Não cria struct `ResolvedLabelStore`** — P193B.
- **Não modifica walk arms** — P195+.
- **Não modifica consumer C4** — P194.
- **Não modifica `from_tags`** — P195+.
- **Sem inflação retórica**.
- **Honestidade obrigatória**: sub-store fica vazio em
  produção até P195+ — registado em §1.8 + §2.3 do
  diagnóstico.
- **Sem cláusulas condicionais** nos sub-passos.

---

## §10 Verificações

- ✅ `cargo check --workspace` passa (sem código tocado).
- ✅ `cargo test --workspace` passa: **1.815** inalterado
  vs P189B.
- ✅ `crystalline-lint .` zero violations.
- ✅ Diagnóstico produzido (8 secções).
- ✅ Relatório produzido (este ficheiro).
- ✅ Sem ADR; sem DEBT formal.
- ✅ Análise dos 2 eixos aplicada.

---

## §11 Achados não-triviais

### §11.1 — Vanilla typst usa Locator/Resolved trait, cristalino HashMap

Cristalino simplificou desde o início para
`HashMap<Label, String>` em `state.resolved_labels`.
Vanilla tem estrutura mais rica com `Locator::resolve()`
retornando `Resolved` (path + key + content). Cristalino
não tem essa nuance e P193 mantém a simplificação —
sub-store dedicado replica `HashMap<Label, String>`
literal.

Sem ADR necessário porque decisão é **continuidade de
escolha pré-existente**, não decisão nova.

### §11.2 — Auto-toc em Heading + explicit em Labelled

`state.resolved_labels` é populated por **2 caminhos**:
- `Content::Heading` arm em walk: gera label
  `auto-toc-N` e mapeia para `"Secção {prefix}"`.
- `Content::Labelled` arm em walk: mapeia label explícita
  para texto baseado no target.

Ambos populam o **mesmo HashMap**. Sub-store novo
preserva esta semântica — chave `Label` (incluindo
auto-toc-N) → valor `String` (texto resolvido).

### §11.3 — Consumer C4 simples — fallback `@{key}`

Consumer C4 em `references.rs:53` é **simples** — match
`Some(text)` → render text; `None` → fallback literal
`@{key}`. Sem lógica condicional complexa.

P194 migration será trivial: substitution-with-fallback
per padrão P184D/P187B/P188B.

### §11.4 — Sem variante `*_at` necessária

Análise dos 2 eixos confirmou: eixo 1 = snapshot final
(consumer lê durante layout, após walk completo). Diferente
de P185B (counter location-aware) que precisaram porque
counter muda durante walk.

`resolved_labels` é **write-once por label durante walk**
(walk arms fazem `insert` mas não `update`). Consumer lê
após walk completo — snapshot final é suficiente.

### §11.5 — Sub-store fica vazio em produção até P195

P193 abre **infra**. Não popula em produção. Walks
continuam a popular legacy directamente (E2/E4).

Tests unit em P193B populam manualmente
(`intr.resolved_labels.insert(...)`) para validar API.
Populate via `from_tags` vem em P195.

Esta janela compat (sub-store vazio em produção) é
honestidade obrigatória per restrição P193A — registada
em §1.8 + §2.3 + §11.5 e em diagnóstico §6.

### §11.6 — P193 é primeiro passo da sequência §9 do P189 consolidado

P193 marca início da implementação do plano sequencial
identificado em P189 §9 para fechar M5 universalmente.
Sequência:

1. ✅ P193 (este passo) — sub-store resolved_labels.
2. P194 — C4 migration.
3. P195 — migrar walk arm Labelled.
4. P196 — migrar walk arm Heading.
5. P197 — migrar walk arm Figure.
6. P198 — migrar walks SetHeadingNumbering + CounterUpdate.
7. SetEquationNumbering (independente) — fecha E1.

Após esses 7 passos, M5 universalmente fechado; segue M6.

---

## §12 Snapshot pós-P193A

- **Tests workspace**: 1.815 (inalterado).
- **Trait `Introspector`**: 18 métodos.
- **`TagIntrospector` sub-stores**: 7 (LabelRegistry,
  CounterRegistry, MetadataStore, StateRegistry, BibStore,
  figure_label_numbers, kind_index).
- **M5 progresso**: 1 arm migrado + 6 excepções (P189B);
  P193 desbloqueia primeira pré-condição.
- **DEBT M5-residual**: 4 → 3 pré-requisitos pendentes.
- **61 passos executados** (P189B = 60 + P193A = 61).
- **Padrão diagnóstico-primeiro**: 15ª aplicação consecutiva
  (P193A na lista).

---

## §13 Próximo passo

**P193B** — abrir `ResolvedLabelStore`:

1. Criar `01_core/src/entities/resolved_label_store.rs`:
   - Struct mínima com `HashMap<Label, String>` interno.
   - Métodos `empty`, `insert(pub(crate))`, `get`, `len`,
     `is_empty`.
   - Tests unit inline.

2. Criar L0 `00_nucleo/prompts/entities/resolved_label_store.md`
   com 14 secções padrão.

3. Editar `01_core/src/entities/introspector.rs`:
   - Adicionar field `pub resolved_labels: ResolvedLabelStore`
     em `TagIntrospector`.
   - Adicionar método trait
     `fn resolved_label_for(&self, label: &Label) -> Option<&str>`.
   - Implementação delega a `self.resolved_labels.get(label)`.

4. Editar L0 `00_nucleo/prompts/entities/introspector.md`:
   - Field novo + método novo + entrada Histórico.

5. Tests unit (4):
   - `empty_store_returns_none`.
   - `insert_then_get`.
   - `multiple_labels_isolated`.
   - `trait_method_delegates`.

6. Actualizar nota DEBT M5-residual no relatório
   consolidado P193 (4 → 3 pré-requisitos).

7. Relatório consolidado P193 (9 secções padrão).

Magnitude: S puro. Sem cláusulas condicionais.

---

## §14 Conclusão

P193A fechou 8 cláusulas com decisão literal e plano em
sub-passo único. Magnitude S agregada confirmada. ADR
avaliada e dispensada (replicação P181B + padrão).
DEBT avaliado: cenário B continuação (3 dos 4
pré-requisitos restam após P193B).

Achados centrais:
- **Replica P181B BibStore** literalmente — struct
  dedicada, field paralelo, método trait simples.
- **Sem variante location-aware** — análise dos 2 eixos
  confirmou snapshot final.
- **Sub-store vazio em produção** até P195 — honestidade
  obrigatória registada em 4 secções do diagnóstico.
- **P193 desbloqueia P194** (C4 migration) e
  indirectamente P195 (walk Labelled migration).

P193 é **passo 1 dos 7** identificados em P189 §9.
Sequência de migrações continua incrementalmente. M5
universal fecha após sequência completar.

Padrão diagnóstico-primeiro mantido — 15/15 acertaram a
magnitude planeada ±1 nível
(P131A/132A/140A/148/154A/181A/182A/183A/184A/185A/186A/
187A/188A/189A/193A).

Próximo passo: **P193B** — abertura concreta do sub-store.
