# Diagnóstico — Location-aware Layouter (Passo 185A)

**Data**: 2026-05-03
**Passo**: P185A — diagnóstico-primeiro / L0-puro
**Escopo**: desenhar mecanismo de propagação `Location` ao Layouter
no ponto de consulta de contadores, pré-condição para desbloquear
C1 (heading prefix) e C2 (equation counter).
**Postura**: zero código tocado em L1–L4; zero testes modificados;
zero L0 modificado. Decisões + plano executável + ADR PROPOSTO.

---

## §1 Validação do estado actual

Inspecção empírica em 2026-05-03:

| Item | Estado confirmado | Linha actual / observação |
|------|-------------------|---------------------------|
| 1 | Trait `Introspector` location-aware methods | `formatted_counter_at(key, location)` em `introspector.rs:91` (P177); `state_value(key, location)` em `:76` (P171). Outros métodos `*_at` ausentes — ver §2. |
| 2 | Layouter uso de `Location` | `grep -rn "Location" 01_core/src/rules/layout/`: zero hits em ficheiros de produção (apenas em `tests.rs` para testes Introspector). Layouter actualmente **não conhece** Location no ponto da consulta. |
| 3 | `Locator` API | `entities/locator.rs`: struct simples com `counter: u64` interno; `Locator::new()`, `next() -> Location` (auto-incremento), `Default`. **Não-Clone** intencional (linha 22-23). **Determinismo provado** em test `duas_instancias_paralelas_produzem_sequencias_iguais` (linha 67-72): dois `Locator::new()` paralelos produzem sequências idênticas. |
| 4 | Walk de introspect uso de Locator | `introspect.rs:329-330`: `if let Some(payload) = do_extract_payload(content) { let loc = locator.next(); ... }`. Locator avança **exactamente** quando `is_locatable(content) == true` (invariante explícito em `locatable.rs:11`). |
| 5 | `is_locatable` predicate | Match exaustivo em `locatable.rs:20-50`. Locatable: Heading, Figure, Cite, Metadata, State, StateUpdate, Outline, Bibliography, SetHeadingNumbering. **`Equation` NÃO é locatable** — pré-requisito P186 para C2. |
| 6 | C1 consumer site | `mod.rs:310`: `self.counter.format_hierarchical("heading")` dentro de `Content::Heading` arm. Quando este arm executa, o Layouter está a processar uma heading — Location pode ser conhecida. |
| 7 | C2 consumer site | `equation.rs:97`: `self.counter.get_flat("equation")` dentro de `layout_equation` para `Content::Equation { block: true }`. Equation **não locatable** actualmente — Layouter não pode receber Location dela até P186. |
| 8 | Vanilla typst Locator | `lab/typst-original/.../introspection/locator.rs`: struct `Locator<'a>` com `LocatorLink` para measurement mode + `Locator::split` para sub-walks. **Owned** (passa por valor), não-Clone, com lifetime. Comentário linha 22: "all layouters receive an owned `Locator`" — confirma vanilla usa M2-like (parâmetro propagado). |
| 9 | Walks separados | Walk de introspect (`introspect.rs::walk`) e walk de layout (`layout/mod.rs::layout_content`) são walks **independentes** que iteram o mesmo `Content` em DFS. |

**Conclusão §1**: cristalino tem 4 ingredientes que tornam M3 (Locator
dedicado Layouter) trivialmente correcto:

1. `Locator` determinístico com garantia de igualdade entre instâncias
   paralelas (testada).
2. `is_locatable` predicate puro `&Content → bool`.
3. Walks de introspect e layout iteram a mesma estrutura DFS.
4. Locator avança exactamente em `is_locatable(content) == true`
   (invariante na ligação extract_payload ↔ is_locatable).

Sincronização por construção é alcançável sem comunicação cross-walk.

---

## §2 Inventário — Layouter, Locator, Introspector methods, vanilla

### §2.1 Layouter actual

`Layouter<M, S>` em `mod.rs:80-110` tem fields existentes:
- `font_size_pt`, `style`, `chain`, `page_config`
- `pages`, `current_items`, `current_line`
- `cursor_x`, `cursor_y`, `line_start_x`
- `counter: CounterStateLegacy`
- `introspector: TagIntrospector` (P181B)
- `figure_progress: HashMap<String, usize>`
- cell-related fields (`is_height_unconstrained`, `cell_*`)

State mutável é o padrão. Adicionar `locator: Locator` +
`current_location: Option<Location>` é extensão natural.

### §2.2 `Locator` actual

```rust
pub struct Locator { counter: u64 }

impl Locator {
    pub fn new() -> Self;
    pub fn next(&mut self) -> Location;   // auto-incrementa
}
```

Não-Clone intencional. Determinismo provado por test.

### §2.3 Trait `Introspector` — métodos location-aware

| Método | Existe? | Suporta C1? | Suporta C2? |
|--------|---------|-------------|-------------|
| `formatted_counter(key)` (P170) | ✅ | n/a (snapshot final) | n/a (snapshot final) |
| `formatted_counter_at(key, loc)` (P177) | ✅ | **✅** — heading prefix string | n/a (string, não usize) |
| `is_numbering_active(key)` (P182B) | ✅ | n/a (final) | n/a (final) |
| `is_numbering_active_at(key, loc)` | ❌ | **falta** | **falta** |
| `figure_number_at_index(kind, idx)` (P184C) | ✅ | n/a | n/a (figure, não equation) |
| `flat_counter(key)` | ❌ | n/a | n/a |
| `flat_counter_at(key, loc)` | ❌ | n/a | **falta** — equation counter retorna `usize` |
| `state_value(key, loc)` (P171) | ✅ | indirecto | indirecto |
| `state_final_value(key)` (P171) | ✅ | n/a (final) | n/a (final) |

**Métodos em falta para P185**:
- `is_numbering_active_at(key, location) -> bool` — replica
  `is_numbering_active` mas usa `state.value_at(key, location)` em
  vez de `state.final_value(key)`. Resolve P182E §5.2 (re-update
  correctness).
- `flat_counter_at(key, location) -> Option<usize>` — replica
  semântica de `state.get_flat` legacy mas via
  `CounterRegistry::value_at`. Necessário para C2 quando P186 promover
  Equation a locatable.

P185B materializa estes métodos.

### §2.4 Vanilla typst — solução equivalente

Forma vanilla: Locator passa via assinatura de cada layout function.
Sub-walks usam `Locator::split` para gerar Locators independentes
sincronizados com o pai. Measurement mode usa `LocatorLink` para
"reusar" Locations sem avançar o counter.

Cristalino não tem `LocatorLink` nem `split` — mas também não tem as
features (footnote relayout, measurement-mode pervasivo) que o
exigem. Para o escopo M5/M6 cristalino, M3 (Locator dedicado
determinístico) é suficiente.

---

## §3 Decisões cláusula 1–6

### Cláusula 1 — Mecanismo de propagação

**O1 (inputs)**: §1 + §2. `Locator` determinístico (provado);
`is_locatable` puro; walks DFS independentes; vanilla usa M2 mas
forçado pela sua estrutura de Locator com link.

**O2 (alternativas)**:
- **M1** — walk sincronizado com Locator partilhado.
- **M2** — parâmetro propagado em todas as layout functions.
- **M3** — Locator dedicado do Layouter (cursor próprio).

**O3 (critério)**:
- Coerência P163 walk puro: M2/M3 OK; M1 OK também (Locator
  partilhado é leitura).
- Coerência ADR-0036 atomização: M3 isola; M2 cascata; M1 acopla.
- Coerência ADR-0067 attribute-grammar futuro: M3 alinha (location
  é attribute herdado top-down via cursor field).
- Custo: M3 ~30 LOC; M2 ~150 LOC; M1 trivial mas adiciona
  acoplamento.
- Reversibilidade: M3 reverte trivialmente removendo 2 fields; M2
  cascata-reversa; M1 desfaz acoplamento.

**O4 (magnitude da decisão)**: substancial.

**O5 (reversibilidade)**: M3 mais reversível.

**Decisão**: **M3** — Locator dedicado do Layouter com `current_location`
field. Sincronização por construção via determinismo + `is_locatable`
gating.

ADR-0068 PROPOSTO documenta a decisão (cf. §6).

### Cláusula 2 — Trait methods location-aware necessários

**O1 (inputs)**: §2.3 inventário.

**O2 (alternativas)**:
- α: adicionar todos os métodos `*_at` esperados em P185B
  (`is_numbering_active_at`, `flat_counter_at`).
- β: adicionar apenas `is_numbering_active_at` (para P187 C1) e
  deferir `flat_counter_at` para P186/P188 quando Equation for
  locatable.

**O3 (critério)**: padrão P181F/P184C adiciona métodos quando há
consumer pendente. C1 (P187) precisa de `is_numbering_active_at` +
`formatted_counter_at` (já existe). C2 (P188) precisa de
`flat_counter_at` mas só após P186. Adicionar `flat_counter_at` em
P185B é trabalho preparatório — barato (replica `formatted_counter_at`)
mas sem consumer imediato.

**O4 (magnitude)**: trivial em ambas.

**O5 (reversibilidade)**: trivial.

**Decisão**: **Opção α**. Adicionar ambos em P185B (`is_numbering_active_at`
+ `flat_counter_at`). Marginal cost (~10 LOC + 4 tests cada);
permite P186/P187/P188 prosseguir sem dependência cross-passo.

### Cláusula 3 — Compatibilidade com `Locator`

**O1 (inputs)**: `entities/locator.rs` simples; não-Clone; determinístico.

**O2 (alternativas)**:
- A: Locator partilhado entre walks.
- B: Locator separado por walk com sincronização por construção.
- C: Pre-compute Locations no walk de introspect.

**O3 (critério)**: M3 escolhido em cláusula 1 implica B
(Layouter tem Locator próprio). A e C requerem comunicação
cross-componente desnecessária.

**O4 (magnitude)**: trivial.

**O5 (reversibilidade)**: trivial.

**Decisão**: **Opção B**. Layouter tem `locator: Locator` field;
`Layouter::new` inicializa via `Locator::new()`. Sincronização
guaranteed by `Locator` determinism + `is_locatable` invariant.

### Cláusula 4 — Forma de migração C1 + C2

**O1 (inputs)**: padrão P184D substitution-with-fallback.

**O2 (alternativas)**:
- A: substitution-with-fallback (replica P184D).
- B: substituição directa.

**O3 (critério)**: padrão estabelecido. Fallback durante janela compat
M6 é defensivo. C1 + C2 ficam para P187/P188 — esta cláusula apenas
fixa a forma para esses passos.

**O4 (magnitude)**: trivial.

**O5 (reversibilidade)**: trivial.

**Decisão**: **Opção A**. P187 e P188 usam substitution-with-fallback
quando consumirem `current_location`.

### Cláusula 5 — Compat walk puro

**O1 (inputs)**: P163 invariante walk puro cobre walk de introspect.

**O2 (alternativas)**:
- M1 (walk sincronizado): Locator partilhado é leitura — OK.
- M2 (parâmetro propagado): Location é argumento — OK.
- M3 (cursor próprio): cursor é state mutável dentro do Layouter
  (não no walk de introspect) — OK; Layouter já é stateful por
  design.

**O3 (critério)**: M3 escolhido em cláusula 1. P163 cobre walk de
introspect, não walk de layout. Layouter já tem state mutável
(cursor_x, cursor_y, current_line). Locator + current_location é
extensão natural, não violação.

**O4 (magnitude)**: trivial (confirmação).

**O5 (reversibilidade)**: n/a.

**Decisão**: **Confirmado compatível**. P163 preservado.

### Cláusula 6 — Critério de fecho de P185

**O1 (inputs)**: padrão P181F/P182B/P184F.

**O2 (alternativas)**:
- 1: diagnóstico (P185A) + infra (P185B trait methods + P185C
  Layouter integration) + tests (P185D) + relatório (P185E).
- 2: Opção 1 + migração C1+C2.

**O3 (critério)**: Opção 2 misturaria escopo P185 com
P186/P187/P188. P185 fecha **infra location-aware** — migração
de C1 + C2 fica para passos dedicados conforme plano P184F §13
(P186 Equation locatable; P187 migrar C1; P188 migrar C2).

**O4 (magnitude)**: P185A S documental + P185B S + P185C M +
P185D S + P185E S documental = **S-M agregado**.

**O5 (reversibilidade)**: P185 série reversível antes de P187+P188
consumirem.

**Decisão**: **Opção 1**. P185 fecha quando:
1. Trait methods `is_numbering_active_at` + `flat_counter_at`
   declarados + impl + tests (P185B).
2. Layouter tem `locator` + `current_location` integrados; gating
   em `layout_content` funcional (P185C).
3. Tests E2E confirmam mecanismo via consultas sintéticas em
   pipeline real (P185D).
4. Relatório consolidado escrito (P185E).
5. C1+C2 não migrados em P185 — ficam para P187/P188.

---

## §4 Plano de sub-passos P185B+ sem condicionais

| Sub-passo | Escopo | Magnitude | Depende |
|-----------|--------|-----------|---------|
| `.B` | Adicionar `is_numbering_active_at` + `flat_counter_at` ao trait `Introspector` + impl em `TagIntrospector` (delegação a `StateRegistry::value_at` + `CounterRegistry::value_at`) + 8-10 tests unit (4-5 cada) + L0 `entities/introspector.md` | S | — |
| `.C` | Layouter ganha fields `locator: Locator` + `current_location: Option<Location>`; `Layouter::new` inicializa `Locator::new()`; gating em `layout_content` com `is_locatable` (~15 LOC); `prev_loc` save/restore para scoping léxico; L0 `rules/layout.md` actualizado | M | `.B` |
| `.D` | Tests E2E confirmam `current_location` reflecte correctamente Location esperada para nós locatable em pipeline real (sem migrar C1/C2). 3-5 tests: heading produz Location esperada; sequence isola; nested headings preservam scoping | S | `.C` |
| `.E` | Relatório consolidado P185 (9 secções padrão P181J/P182F/P184consolidado); transição ADR-0068 PROPOSTO → ACEITE se validação passa | S | `.D` |

Sequência fixa B → C → D → E. Sem cláusulas condicionais.

---

## §5 Magnitude consolidada

P185 série agregada: **S-M**.

- P185A: S documental (~6h efectivas estimadas para auditoria +
  decisões + 3 outputs).
- P185B: S (~30 LOC trait + impl + ~50 LOC tests).
- P185C: M (~30 LOC Layouter + L0 update + cascata mínima).
- P185D: S (~50 LOC tests E2E).
- P185E: S documental.

Custo agregado P185B–E: ~150 LOC + ~120 LOC tests ≈ M. Diagnóstico
+ implementação + tests + relatório = **S-M**.

**Q2 honestidade**: implementação P185C é **M genuíno** — primeira
introdução de Locator no Layouter. Risco de descoberta de edge case
(ex.: layout salta nó que walk visitou) que pode escalar a M-L se
materializar. Estimativa M assumindo sincronização-por-construção
mantém-se válida; ADR-0068 critério de validação detecta divergência
em P185C.

---

## §6 ADR avaliação

**ADR-0068 PROPOSTO** criada (cf.
`00_nucleo/adr/typst-adr-0068-location-aware-layouter.md`).

Status `PROPOSTO` até P185C+D validarem. Critério de transição
`ACEITE` definido na ADR §"Critério de validação":
1. Materialização P185C com forma documentada.
2. Tests E2E P185D confirmam `current_location` correctness.
3. Magnitude real dentro de ±50% da estimada (M).

Critério `REJEITADA` se sincronização-por-construção falhar (ex.:
edge case em layout que diverge do walk de introspect).

Diferente de P181A/P182A/P183A/P184A — P185A criou ADR porque a
decisão arquitectural é substancial (escolha entre 3 mecanismos
com perfis diferentes, sem caminho óbvio).

---

## §7 DEBT avaliação

P185A não abre DEBT. P185 série é trabalho pendente identificado
em P182E §5.2 + P184F §8 — não é DEBT-cleanup mas execução
planeada.

DEBT M4-residual mantém cobertura **C1 + C2** registada em
P184F (m1-lacunas-captura.md anexo). P187 e P188 fechá-los após
P185 fornecer infra.

---

## §8 Próximo sub-passo — P185B

Escopo concreto:

1. Em `01_core/src/entities/introspector.rs`:
   - Adicionar ao trait `Introspector`:
     ```rust
     fn is_numbering_active_at(&self, key: &str, location: Location) -> bool;
     fn flat_counter_at(&self, key: &str, location: Location) -> Option<usize>;
     ```
   - Impl em `TagIntrospector`:
     - `is_numbering_active_at`: delega a `state.value_at(key, location)` + match `Value::Bool(true)`. Replica padrão `is_numbering_active` (P182B) mas usa `value_at` em vez de `final_value`.
     - `flat_counter_at`: delega a `counters.value_at(key, location)?.last().copied()`. Replica padrão `figure_number_at_index` (P184C) mas via Location em vez de idx.

2. L0 `entities/introspector.md` ganha entradas para os 2 métodos
   novos + entrada nova no histórico.

3. 8-10 tests unitários (4-5 cada):
   - `is_numbering_active_at`: vazio → false; before update → false;
     after update → true; re-update → reflecte Location consultada.
   - `flat_counter_at`: vazio → None; populate via apply_at → Some(N);
     idx-by-Location distinto isola.

4. Critério de fecho P185B: `cargo test --workspace --lib` passa
   com Δ baseline +8-10; `crystalline-lint .` zero violations
   (após `--fix-hashes`).

Pré-condição P185B: P185A concluído (este passo); ADR-0068
PROPOSTO documenta mecanismo M3.
