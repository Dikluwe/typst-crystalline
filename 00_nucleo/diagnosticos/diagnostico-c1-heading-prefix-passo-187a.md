# Diagnóstico — C1 heading prefix migration (Passo P187A)

**Data**: 2026-05-03
**Magnitude**: S (diagnóstico)
**ADR vinculada**: nenhuma (replicação de padrão P184D).
**Pré-condição**: P186 série fechada; ADR-0068 ACEITE; P185
infraestrutura completa.

---

## §1 Validação do estado actual

### §1.1 — Site C1 actual (mod.rs:345)

```rust
Content::Heading { level, body } => {
    self.counter.step_hierarchical("heading", *level as usize);
    // ...
    use crate::entities::introspector::Introspector;
    let numbering_on = self.introspector
        .is_numbering_active("numbering_active:heading")
        || self.counter.is_numbering_active("heading");
    if numbering_on {
        if let Some(num_str) = self.counter.format_hierarchical("heading") {
            let prefix = Content::text(format!("{}. ", num_str));
            self.layout_content(&prefix);
        }
    }
    // ...
}
```

C1 = **`self.counter.format_hierarchical("heading")`** em
`mod.rs:345`. Linha referenciada na spec (310) está
desactualizada — P185C/P186 introduziram código antes desta
linha. Site real é **mod.rs:345** após inflação.

### §1.2 — Acesso a `self.introspector` e `self.current_location`

- `self.introspector` já consultado no site (linhas 341-343
  via P182D para `is_numbering_active`).
- `self.current_location: Option<Location>` é
  `pub(super)` em `mod.rs:131-141` (P185C).
- Ambos acessíveis no site C1.

### §1.3 — `formatted_counter_at` API (P177)

`introspector.rs:91`:

```rust
fn formatted_counter_at(&self, key: &str, location: Location) -> Option<String>;
```

Mesma forma de retorno que `format_hierarchical` legacy —
substituição directa por shape.

### §1.4 — `current_location` populated antes do site C1

`Content::Heading` é locatable (per `is_locatable.rs:23`).
Gating `advance_locator_if_locatable` em `layout_content`
(P185C, `mod.rs:236-240`) **precede** o match arm. Logo no
site C1 (linha 345, dentro de arm Heading), `current_location`
está `Some(loc_da_heading)`.

Empiricamente confirmado por test P185D `.E`
(`pipeline_e2e_is_numbering_active_at_via_current_location`)
que valida exactamente este pattern.

### §1.5 — `format_hierarchical` legacy

`counter_state_legacy.rs:126`:

```rust
pub fn format_hierarchical(&self, key: &str) -> Option<String> {
    let counter = self.hierarchical.get(key)?;
    if counter.is_empty() {
        None
    } else {
        Some(counter.iter().map(|n| n.to_string()).collect::<Vec<_>>().join("."))
    }
}
```

Retorna `"1.2.3"` ou `None`. Mesma shape que `formatted_counter_at`.

### §1.6 — Walk arm Heading + from_tags arm Heading

Walk de introspect (`introspect.rs:280` aproximadamente):
chama `state.step_hierarchical("heading", *level)` e emite
Tag.

`from_tags` arm Heading (`from_tags.rs:51-71`, P170): chama
`intr.counters.apply_hierarchical_at("heading", *depth as
usize, *loc)` — aplica step E regista snapshot na location.

Logo `intr.counters.value_at("heading", loc_da_heading)`
retorna o snapshot post-step para essa heading.
`formatted_counter_at` formata-o como string `"1.2.3"`.

### §1.7 — Paridade observable

Para sequência `H1, H2, H1`:
- Walk loc(0): step → counter `[1]` → snapshot at loc(0).
- Walk loc(1): step → counter `[1, 1]` → snapshot at loc(1).
- Walk loc(2): step → counter `[2]` → snapshot at loc(2).

Layouter no site C1 com `current_location = Some(loc(0))`
chama `formatted_counter_at("heading", loc(0))` →
`Some("1")`. Em loc(1) → `Some("1.1")`. Em loc(2) →
`Some("2")`.

Legacy `format_hierarchical("heading")` no Layouter:
- Site H1: post-step → `[1]` → `Some("1")`.
- Site H2: post-step → `[1, 1]` → `Some("1.1")`.
- Site H1 (segunda): post-step → `[2]` → `Some("2")`.

**Paridade observable** entre os dois paths confirmada por
construção. Diferente de P183B onde `formatted_counter`
(snapshot final) retornava sempre `"2"` para todos os sites.

---

## §2 Decisões cláusulas 1–6

### §2.1 — Cláusula 1: forma da expressão

**Decisão fixada**: combinação de Opção B (`and_then` para
`Option<Location>`) + Opção A (`or_else` legacy):

```rust
self.current_location
    .and_then(|loc| self.introspector.formatted_counter_at("heading", loc))
    .or_else(|| self.counter.format_hierarchical("heading"))
```

**O1**: §1.2-1.4 (current_location é Option; populated no
site C1).

**O2**:
- Opção A inline directo `unwrap()`: rejeitada — frágil;
  panic potencial se invariante violada.
- **Opção B `and_then`**: aceite — defensiva, sem panic.
- Opção C match explícito: rejeitada — verbose para 1 linha.

**O3**: P184D Figure padrão usou `.or_else()` para fallback
legacy. Replica.

**O4 — Magnitude**: trivial. ~3 LOC mudança.

**O5 — Reversibilidade**: ALTA (substitution-with-fallback
por construção).

### §2.2 — Cláusula 2: tratamento `None` do Introspector

**Decisão fixada**: **Opção A** — `or_else` para legacy
`format_hierarchical`. Replica P184D.

`formatted_counter_at` retorna `None` quando:
- Chave inexistente em `CounterRegistry`.
- Location anterior à primeira escrita.

Em produção real (com `Content::SetHeadingNumbering` +
heading processado), Introspector retorna `Some` para
headings. Fallback é defensivo, raramente disparado.

### §2.3 — Cláusula 3: tratamento `None` do `current_location`

**Decisão fixada**: **Opção B** — `and_then`.

Per §1.4, `current_location` é `Some` no site C1 (Heading
arm é processado após gating). Mas defensiva via `and_then`
preserva integridade caso ordem mude no futuro.

Combinação cláusula 1 + 2 + 3 produz a expressão:

```rust
self.current_location
    .and_then(|loc| self.introspector.formatted_counter_at("heading", loc))
    .or_else(|| self.counter.format_hierarchical("heading"))
```

### §2.4 — Cláusula 4: P183B aprendizado validado não-aplicável

P183B falhou porque `formatted_counter("heading")`
(snapshot-final) retornava `"2"` para todos os sites na
sequência `H1, H2, H1` — pré-emptava o fallback em
sequências com re-update.

`formatted_counter_at("heading", current_location)` é
**location-aware**. Para o mesmo cenário:
- H1 (loc=0): retorna `"1"` (correcto).
- H2 (loc=1): retorna `"1.1"` (correcto).
- H1 segunda (loc=2): retorna `"2"` (correcto).

Em todos os sites, Introspector path retorna o valor que
coincide com legacy walk-during. Fallback nunca é
necessário em produção real. P183B aprendizado confirmado
**não-aplicável** após P185.

Test P185D `.E`
(`pipeline_e2e_is_numbering_active_at_via_current_location`)
empiricamente demonstra equivalência via
`is_numbering_active_at` location-aware. Mesmo padrão se
aplica a `formatted_counter_at`.

### §2.5 — Cláusula 5: forma de migração

**Decisão fixada**: substitution-with-fallback per P184D
padrão. Replica literal:

```rust
// Antes (legacy):
let prefix = self.counter.format_hierarchical("heading");

// Depois (Introspector primeiro):
let prefix = self.current_location
    .and_then(|loc| self.introspector.formatted_counter_at("heading", loc))
    .or_else(|| self.counter.format_hierarchical("heading"));
```

### §2.6 — Cláusula 6: critério de fecho de P187

**Decisão fixada**: **Opção 3** — infra pronta + consumer
migrado + tests E2E paridade.

P187 fecha quando:
1. Consumer C1 migrado em `mod.rs:345`.
2. Tests E2E confirmam paridade observable (output Layouter
   idêntico legacy vs Introspector path).
3. P182D/P182E tests existentes não regridem.
4. DEBT M4-residual nota actualizada (cenário B): cobre
   apenas C2 após P187.

---

## §3 Plano de sub-passos

Sub-passo único agregado.

| Sub-passo | Escopo | Magnitude | Depende |
|-----------|--------|-----------|---------|
| **P187B** | Migrar consumer C1 (`mod.rs:345`) com substitution-with-fallback + L0 layout actualizado + tests E2E paridade + actualização nota DEBT M4-residual + relatório consolidado P187 | S | — |

Sequência simples B sem condicionais. Granularidade
inferior à P186 (5 sub-passos) porque migração é única
linha + tests directos.

Total agregado P187B: ~5 LOC produção + ~80 LOC tests +
edits L0 ≈ S puro.

---

## §4 Magnitude consolidada

P187 série: **S puro** (1×S agregado).

Diferente de P186 (S agregado por replicação 6 sub-passos
para infraestrutura). P187 é migração de consumer único
— infra já existe (P185).

---

## §5 ADR avaliação

**Sem ADR criada.** Justificação:
- Substitution-with-fallback é padrão P184D — não decisão
  arquitectural nova.
- `formatted_counter_at` já existe (P177) — sem nova
  primitiva.
- `current_location` já existe (P185C) — sem nova infra.
- Não há decisão arquitectural nova.

---

## §6 DEBT avaliação (actualização M4-residual)

### Cenário identificado: **B**

`grep` em `00_nucleo/` por "DEBT-M4-residual" / "P183F" não
retornou DEBT formal aberto. Apenas notas preventivas em
relatórios (P184F/P185-consolidado/P186-consolidado).

**Acção em P187B**: actualizar nota preventiva no relatório
P187 indicando que após migração:
- DEBT M4-residual cobre apenas **C2** (era C1 + C2).
- C2 fica para P188 (com Introspector path dormente em
  produção).
- Quando P183F formalmente abrir DEBT (se o fizer), incluir
  apenas C2.

---

## §7 Relação com P183B falha

P183B falhou com **gate substancial** porque:

1. `self.counter.step_hierarchical("heading", level)` em
   `mod.rs:328` mutava counter legacy durante walk.
2. Site C1 lia legacy via `format_hierarchical("heading")`
   — retornava valor walk-during (correcto por construção).
3. P183B substituiu por
   `formatted_counter("heading")` (Introspector trait
   method P170, snapshot final pós-walk).
4. **Falha**: `Some("2")` retornado para H1 (loc=0) em
   sequência `H1, H2, H1` — pré-emptava fallback. Output
   visível: H1 ficava com prefixo "2." em vez de "1.".

P185 desbloqueou via:
- `formatted_counter_at(key, location)` location-aware
  (P177 já existia mas não era usado em C1).
- `current_location: Option<Location>` no Layouter (P185C).
- Sincronização Locator empiricamente validada (P185D).

P187 finaliza a migração que P183B tentou. **Aprendizado
P183B**: substituir `formatted_counter` por
`formatted_counter_at` corrige o problema. Resto da
estrutura substitution-with-fallback é preservado.

P183B não estava errado em todos os aspectos — estava
errado apenas na primitiva. Infraestrutura para corrigir
demorou P185 inteiro (4 sub-passos).

---

## §8 Próximo sub-passo

**P187B** — migração consumer C1:

- Editar `01_core/src/rules/layout/mod.rs:345`:
  ```rust
  if let Some(num_str) = self.current_location
      .and_then(|loc| self.introspector.formatted_counter_at("heading", loc))
      .or_else(|| self.counter.format_hierarchical("heading"))
  {
      let prefix = Content::text(format!("{}. ", num_str));
      self.layout_content(&prefix);
  }
  ```
- Actualizar L0 `00_nucleo/prompts/rules/layout.md`:
  secção sobre heading prefix; padrão P184D substitution-with-fallback.
- Tests E2E paridade:
  - `c1_heading_prefix_via_introspector_path` (state
    activo via SetHeadingNumbering; Introspector retorna
    snapshot correcto).
  - `c1_heading_prefix_via_fallback_legacy` (Introspector
    vazio; fallback dispara).
  - `c1_heading_prefix_paridade_legacy_vs_migrated`
    (output paridade observable).
  - `c1_heading_prefix_re_update_correctness` (sequência
    H1, H2, H1 — valida que P183B aprendizado é
    correctamente abordado por P185).
- Actualizar relatório P187 com nota DEBT M4-residual
  cenário B (cobre apenas C2 após P187).

Magnitude: S puro. Sem cláusulas condicionais.
