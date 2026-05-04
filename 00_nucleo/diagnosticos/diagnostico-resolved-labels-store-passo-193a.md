# Diagnóstico — Sub-store `resolved_labels` (Passo P193a)

**Data**: 2026-05-04
**Magnitude**: S (diagnóstico)
**ADR vinculada**: nenhuma (replicação P181B BibStore).
**Pré-condição**: M5 incremental fechado em P189B; tests
workspace 1.815; zero violations.

---

## §1 Validação do estado actual + análise dos 2 eixos

### §1.1 — Campo legacy `state.resolved_labels`

`counter_state_legacy.rs:37`:
```rust
pub resolved_labels: HashMap<Label, String>,
```

Tipo: **`HashMap<Label, String>`** — chave é `Label`
(newtype sobre `String` per `entities/label.rs:12`).

### §1.2 — Arms que populam (E2/E4 P189B)

Em `introspect.rs` walk fn:
- `Content::Heading` arm: `state.resolved_labels.insert(auto_label, resolved_text)`
  (auto-toc).
- `Content::Labelled` arm:
  `state.resolved_labels.insert(label, text)` (explicit
  labels).

Per P189B, ambos arms são **excepções E2/E4** — continuam
a mutar legacy directamente porque consumer C4 ainda lê
de `state.resolved_labels`.

### §1.3 — Consumer C4

`layout/references.rs:53` em `layout_ref`:
```rust
let display_text = match layouter.counter.resolved_labels.get(target) {
    Some(text) => text.clone(),
    None       => format!("@{}", target.0),
};
```

Lê `layouter.counter.resolved_labels.get(&Label)` durante
layout; fallback `@{key}` se não encontrado.

Copy-sites adicionais:
- `mod.rs:1481` (short-circuit path):
  `l.counter.resolved_labels = initial_state.resolved_labels;`
- `mod.rs:1512` (fixpoint loop):
  `l.counter.resolved_labels = initial_state.resolved_labels.clone();`

Layouter recebe `resolved_labels` por copy do `initial_state`
externo.

### §1.4 — Vanilla typst

`lab/typst-original/.../introspection/locator.rs` usa
`Locator::resolve() -> Resolved` (struct rico com path,
key, etc.). Cristalino simplifica para `HashMap<Label,
String>` por design — sem nuance vanilla que afecte forma
do sub-store.

### §1.5 — Trait `Introspector` actual

18 métodos per P185-consolidado. Onde adicionar
`resolved_label_for`: imediatamente após métodos
location-aware (P185B), seguindo ordem cronológica.

### §1.6 — `TagIntrospector` struct actual

Sub-stores existentes: `LabelRegistry`, `CounterRegistry`,
`MetadataStore`, `StateRegistry`, `BibStore`,
`figure_label_numbers`, `kind_index`. Field novo
`resolved_labels: ResolvedLabelStore` paralelo a
`bib_store: BibStore`.

### §1.7 — Tests existentes

- `layout_resolved_labels_nao_interfere_entre_documentos`
  (`tests.rs:908-948`).
- E2E em P189B `walk_excepcao_e2/e4` confirmam populate
  legacy.

P193 **não** modifica esses tests. P193 adiciona tests
unit do sub-store novo.

### §1.8 — Análise dos 2 eixos

| Eixo | Análise | Conclusão |
|------|---------|-----------|
| **Eixo 1** (semântica temporal) | Consumer C4 lê durante layout (após walk completo). Resolução label→text é **determinística** após walk: cada label tem texto fixo. | **Snapshot final** — sem necessidade location-aware. |
| **Eixo 2** (existência dados) | Walks Heading/Labelled populam em produção (E2/E4). Sub-store novo será populated por arm `from_tags` em P195 (write paralelo durante janela compat). | **Sim** — dados disponíveis quando arm de populate existir. |

**Decisão (cláusula 5 abaixo)**: sub-store sem variante
`*_at`. API simples `resolved_label_for(&Label) -> Option<&str>`.

---

## §2 Decisões cláusulas 1–8

### §2.1 — Cláusula 1: forma estrutural

**Decisão fixada**: **Opção α** — struct dedicado mínimo,
replicando padrão BibStore (P181B):

```rust
#[derive(Debug, Clone, Default)]
pub struct ResolvedLabelStore {
    labels: HashMap<Label, String>,
}
```

Métodos:
- `pub fn empty() -> Self`.
- `pub(crate) fn insert(&mut self, label: Label, text: String)`.
- `pub fn get(&self, label: &Label) -> Option<&str>` (ou
  via método trait — vide §2.4).

**O1**: §1.1 confirma tipo legacy.
**O2**: Opção β (com indexação por Location) rejeitada —
Eixo 1 = snapshot final, sem benefício. Opção γ (sub-mapa
em StateRegistry) rejeitada — semântica diferente
(state value-at vs label resolution determinístico).
**O3**: padrão BibStore P181B.
**O4 — Magnitude**: trivial. ~30 LOC struct + métodos.
**O5 — Reversibilidade**: ALTA.

### §2.2 — Cláusula 2: localização no `TagIntrospector`

**Decisão fixada**: **Opção 1** — field directo paralelo
a `bib_store: BibStore`:

```rust
pub struct TagIntrospector {
    // existing fields
    pub bib_store: BibStore,
    // NEW (P193B)
    pub resolved_labels: ResolvedLabelStore,
}
```

**O3**: paralelismo arquitectural; cada sub-store é
estrutura própria. Opção 2 (aninhar em StateRegistry)
rejeitada — semântica diferente.

### §2.3 — Cláusula 3: populate em `from_tags`

**Decisão fixada**: **Opção A** — sem arm de populate em
P193. Sub-store fica vazio em produção até P195 adicionar
arm.

P193 abre **infra**; populate vem em P195 (migrar walk
arm Labelled para emitir Tag).

Tests unit em P193B populam manualmente via
`intr.resolved_labels.insert(...)` para validar API.

**O3**: Opção C (bridge legacy → sub-store em from_tags)
rejeitada — duplica estado durante janela compat. Opção A
é mais limpa.

### §2.4 — Cláusula 4: API trait

**Decisão fixada**: **Opção α** — referência:

```rust
fn resolved_label_for(&self, label: &Label) -> Option<&str>;
```

Implementação em `TagIntrospector`:
```rust
fn resolved_label_for(&self, label: &Label) -> Option<&str> {
    self.resolved_labels.get(label)
}
```

**O3**: padrão `bib_entry_for_key` (P181F) — método
dedicado retornando referência. Evita clone desnecessário.
Opção β (`Option<String>` clone) e Opção γ (expor registry
inteiro) rejeitadas.

### §2.5 — Cláusula 5: location-awareness

**Decisão fixada**: **sem variante `*_at`**.

Per §1.8 análise: eixo 1 = snapshot final. Resolução
label→text é determinística e write-once durante walk.

Diferente de P185B (`is_numbering_active_at`,
`flat_counter_at`) que precisaram de variante location-aware
porque consumer Layouter lê **durante** layout walk com
heading/equation re-update. Aqui consumer Layouter lê
após walk completo.

### §2.6 — Cláusula 6: compat com legacy

**Decisão fixada**: independência total durante janela
compat. Walks (E2/E4) continuam a mutar
`state.resolved_labels`; consumer C4 continua a ler de
legacy.

Plano de transição:
1. **P193B** (este passo) — sub-store + API trait. Vazio
   em produção.
2. **P195** — walk arm Labelled migra; emite Tag;
   `from_tags` arm popula sub-store. Walks Heading
   continuam a popular legacy também (write paralelo).
3. **P196** — walk arm Heading migra; legacy mutation
   removida; sub-store é único populator.
4. **P194** — consumer C4 migra para
   `intr.resolved_label_for(label).or_else(|| state.resolved_labels.get(label))`
   (substitution-with-fallback per padrão).
5. **M6** — `state.resolved_labels` removido; fallback
   removido.

### §2.7 — Cláusula 7: tipo de chave

**Decisão fixada**: **`Label`** (replica legacy).

Per §1.1, `state.resolved_labels: HashMap<Label, String>`.
Sub-store novo idêntico.

### §2.8 — Cláusula 8: critério de fecho de P193

**Decisão fixada**: P193 fecha quando:
1. Struct `ResolvedLabelStore` adicionada em
   `01_core/src/entities/resolved_label_store.rs` (ou
   módulo similar).
2. Field `resolved_labels: ResolvedLabelStore` em
   `TagIntrospector`.
3. Método trait
   `resolved_label_for(&self, label: &Label) -> Option<&str>`.
4. Tests unit:
   - `empty_store_returns_none`.
   - `insert_then_get`.
   - `multiple_labels_isolated`.
   - `trait_method_delegates`.
5. L0s actualizados:
   - Novo: `entities/resolved_label_store.md`.
   - Modificado: `entities/introspector.md` (field +
     método).

**Não exige**:
- Walk arms migrados (P195+).
- Consumer C4 migrado (P194).
- Populate em produção real.

---

## §3 Plano de sub-passos

**Sub-passo único agregado P193B**:

| Sub-passo | Escopo | Magnitude |
|-----------|--------|-----------|
| `.B` | Adicionar `ResolvedLabelStore` struct + L0 + field em `TagIntrospector` + método trait + 4 tests unit + actualização nota DEBT M5-residual + relatório consolidado P193 | S–M |

Total agregado: ~50 LOC produção (struct + impl) + ~30
LOC tests + edits L0 ≈ S puro.

---

## §4 Magnitude consolidada

**P193 série = S puro** (1×S agregado).

Idêntico em magnitude a P187/P188 (sub-passo único
agregado). Replicação literal de padrão P181B com
adaptação a estrutura mais simples (HashMap único vs
BibStore com 2 sub-mapas).

---

## §5 ADR avaliação

**Sem ADR criada.** Justificação:
- Padrão P181B BibStore replicado.
- Padrão P181F método trait replicado.
- Sem semântica nova; sem decisão arquitectural disruptiva.
- §1.4 confirmou que vanilla simplifica para resolução
  via traits — cristalino mantém HashMap simples por
  design (decisão pré-existente).

---

## §6 DEBT avaliação (M5-residual progresso)

### Cenário B (continuação)

**Sem DEBT formal aberto**. Nota actualizada per P189 §8:

> Antes de P193: 6 excepções (E1–E6) bloqueadas por **4
> pré-requisitos**:
> 1. Sub-store `resolved_labels` (não existe).
> 2. C4 migration.
> 3. Sub-store `headings_for_toc` (lacuna #3).
> 4. `Content::SetEquationNumbering` materialização.
>
> Após P193B: **3 pré-requisitos restantes**:
> 1. ~~Sub-store `resolved_labels`~~ ✅ aberto em P193B.
> 2. C4 migration — P194.
> 3. Sub-store `headings_for_toc` — passo dedicado.
> 4. `Content::SetEquationNumbering` — passo independente.
>
> Cadeia E2–E6 desbloqueia incrementalmente após P194 +
> P195 + P196 + P197 + P198. E1 fecha após
> SetEquationNumbering.

P193 marca **avanço da sequência §9 do P189
consolidado**: passo 1 de 7 fechado.

---

## §7 Relação com P189 §9 sequência

P193 = **passo 1 da sequência de 7 passos** identificada em
P189 §9 para fechar M5 universalmente:

```
P193 ✅ (sub-store resolved_labels — este passo)
  ↓
P194 (C4 migration — consumer Ref-arm)
  ↓
P195 (migrar walk arm Labelled) — E2+E4 fecham
  ↓
P196 (migrar walk arm Heading) — E2 residual
  ↓
P197 (migrar walk arm Figure) — E3 fecha
  ↓
P198 (migrar walks SetHeadingNumbering + CounterUpdate)
       — E5+E6 fecham
  ↓
M5 universal fecha (excepto E1)
  ↓
Passo independente (SetEquationNumbering) — E1 fecha
  ↓
M5 universal completamente fechado
  ↓
M6 (eliminar CounterStateLegacy)
```

P193 desbloqueia P194 (que depende de
`resolved_label_for` para fallback path) e indirectamente
P195 (que precisa de sub-store onde escrever).

---

## §8 Próximo sub-passo

**P193B** — abrir `ResolvedLabelStore`:

- Criar `01_core/src/entities/resolved_label_store.rs`:
  - Struct + métodos `empty`, `insert`, `get`, etc.
  - Tests unit inline.
- Criar L0 `00_nucleo/prompts/entities/resolved_label_store.md`.
- Editar `01_core/src/entities/introspector.rs`:
  - Adicionar field `resolved_labels: ResolvedLabelStore`
    em `TagIntrospector`.
  - Adicionar método trait
    `resolved_label_for(&self, label: &Label) -> Option<&str>`.
  - Implementação que delega a `self.resolved_labels.get(label)`.
- Editar L0 `00_nucleo/prompts/entities/introspector.md`:
  - Field novo + método novo + Histórico.
- Tests unit cobrindo populate manual + lookup.
- Actualizar nota DEBT M5-residual no relatório
  consolidado P193.

Magnitude: S puro. Sem cláusulas condicionais.
