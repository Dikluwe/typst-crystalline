# Diagnóstico — `numbering_active` (Passo 182A)

**Data**: 2026-05-02
**Passo**: P182A — diagnóstico-primeiro / L0-puro
**Escopo**: lacuna #4 (`is_numbering_active` / `numbering_active`) conforme `m1-lacunas-captura.md`.
**Postura**: zero código tocado em L1–L4; zero testes modificados; produzir decisões + plano executável.

---

## §1 Validação do estado actual

Inspecção empírica em 2026-05-02. Linhas confirmadas:

| Item | Ficheiro | Linha | Forma actual | Estado |
|------|----------|-------|--------------|--------|
| 1 | `01_core/src/entities/counter_state_legacy.rs` | 34 | `pub numbering_active: HashMap<String, bool>` | confirmado |
| 2 | `01_core/src/entities/counter_state_legacy.rs` | 101–103 | `pub fn is_numbering_active(&self, key: &str) -> bool { self.numbering_active.get(key).copied().unwrap_or(false) }` | confirmado |
| 3 | `01_core/src/entities/content.rs` | 176 | `SetHeadingNumbering { active: bool }` (apenas booleano — *não* `{ key, value }`) | confirmado com desvio face ao texto da lacuna |
| 4 | `01_core/src/rules/eval/rules.rs` | 227 | `Content::SetHeadingNumbering { active }` produzido por `#set heading(numbering: …)` em eval | confirmado |
| 5 | `01_core/src/rules/introspect.rs` | 455–457 | walk arm: `state.numbering_active.insert("heading".to_string(), *active)` | confirmado (write canonical) |
| 6 | `01_core/src/rules/layout/counters.rs` | 11–13 | `layout_set_heading_numbering` faz mesmo insert no Layouter local | confirmado (write paralelo no layout walk) |
| 7 | `01_core/src/rules/layout/mod.rs` | 313–315 | arm `Content::SetHeadingNumbering` delega a `counters::layout_set_heading_numbering` | confirmado |
| 8 | `01_core/src/rules/layout/mod.rs` | 301 | consumer Layouter heading-arm: `if self.counter.is_numbering_active("heading")` antes de gerar prefixo | confirmado |
| 9 | `01_core/src/rules/layout/equation.rs` | 24 | consumer Layouter equation-arm: `block && self.counter.is_numbering_active("equation")` antes de `step_flat` | confirmado (consumer adicional não previsto no texto da lacuna) |
| 10 | `01_core/src/rules/introspect.rs` | 360 | leitura intra-walk para `resolved_text` de heading auto-label: `if state.is_numbering_active("heading")` | confirmado (leitura interna ao próprio walk — não passa por Introspector) |
| 11 | `01_core/src/rules/introspect.rs` | 378 | leitura intra-walk para decidir `step_flat("equation")`: `if *block && state.is_numbering_active("equation")` | confirmado (leitura interna ao próprio walk) |
| 12 | `01_core/src/rules/layout/mod.rs` | 1414, 1442 | `l.counter.numbering_active = initial_state.numbering_active(.clone())` — copia de introspect-walk para layouter | confirmado |

**Desvios face ao texto da lacuna em `m1-lacunas-captura.md`**:

- Texto da lacuna sugere variant `Content::SetHeadingNumbering { key, value }`. Forma real é `{ active: bool }` — apenas booleano `heading` hardcoded. Implicação: o variant não é genérico por chave; só serve numeração de heading. Equation usa o mesmo `numbering_active["equation"]` mas sem variant equivalente — popular acontece apenas em testes (cf. `tests.rs:899`) ou (em produção) implicitamente quando código futuro o setar.
- Texto menciona apenas Layouter heading-arm como consumer. Inventário revela 2 consumers Layouter (heading-arm + equation-arm) e 2 leituras intra-walk (resolved_text, equation step). Total: 4 leituras de `is_numbering_active`. Apenas as 2 do Layouter são candidatas a migração — as 2 intra-walk consomem o mesmo `state` que o walk constrói e não podem consultar o Introspector que ainda não existe.

---

## §2 Inventário vanilla

Comando: `grep -rn "numbering_active\|is_numbering_active" lab/typst-original/crates/`.
Resultado: **zero matches**. Vanilla não tem o conceito `numbering_active`.

Mecanismo equivalente em vanilla:

- `HeadingElem` (`lab/typst-original/crates/typst-library/src/model/heading.rs:134`): `pub numbering: Option<Numbering>` como **campo do próprio elemento**, resolvido via StyleChain (`self.numbering.get_ref(styles)`). `Some(_)` → numeração activa; `None` → inactiva.
- `EquationElem` (`lab/typst-original/crates/typst-library/src/math/equation.rs:67`): idêntico — `pub numbering: Option<Numbering>`.
- `#set heading(numbering: "1.1")` aplica via StyleChain push hierárquico — não há flag boolean separado, não há HashMap global.
- Leitura: `heading.rs:251–252` resolve `self.numbering.get_ref(styles).as_ref().zip(self.location())`; `heading.rs:300–306` `Count::update` retorna `CounterUpdate::Step` apenas se `numbering.is_some()`.
- Default: `numbering: Option<Numbering>` default `None` → numeração **OFF**.
- Granularidade: por elemento individual via StyleChain hierárquica (escopo léxico). Cristalino diverge: HashMap `<String, bool>` global por chave.

Consequência: lacuna #4 não é "feature implementada em vanilla mas ausente em cristalino". É uma **divergência arquitectural** — cristalino inventou `numbering_active` boolean por falta de StyleChain location-aware. P182 não materializa StyleChain; mantém o boolean cristalino e migra-o para o Introspector.

---

## §3 Decisões cláusula 1–6

### Cláusula 1 — Mecanismo

**O1 — Inputs verificáveis**
- Inventário cristalino §1 (12 itens, write canonical em `introspect.rs:455–456`).
- Inventário vanilla §2 (zero matches; vanilla usa StyleChain, não state global).
- Padrão P171 já implementado: `ElementPayload::StateUpdate { key, update: StateUpdate::Set(Value) }` é processado por `from_tags` (`from_tags.rs:154–166`) populando `state: StateRegistry`. Trait `Introspector::state_value(key, location)` + `state_final_value(key)` (`introspector.rs:76–80`) já existem.
- `Content::State`/`Content::StateUpdate` já têm extracção em `extract_payload.rs:44–53`.

**O2 — Alternativas consideradas**
- **M1** — reusar `StateRegistry` P171: `extract_payload` arm `Content::SetHeadingNumbering` produz `ElementPayload::StateUpdate { key: "numbering_active:heading", update: StateUpdate::Set(Value::Bool(active)) }`; `from_tags` arm `StateUpdate` já cobre Set; trait expõe leitura via `state_final_value`.
- **M2** — sub-store dedicado `NumberingFlagStore` paralelo a `BibStore` (P181B). Replica padrão mas inventa estrutura para um booleano que cabe no registry existente.
- **M3** — campo document-level config em `TagIntrospector`. Não tem precedente em cristalino; vanilla também não usa; força mecanismo novo.

**O3 — Critério de escolha**
M1 reusa P171/P173 sem extensão de enum (sem novo `ElementPayload` variant), sem novo sub-store, sem novo método trait obrigatório (a leitura cabe em `state_final_value`). Replica padrão P175/P176/P177/P181F mas com custo zero adicional na infra. M2 é overkill para um booleano. M3 não tem justificação.

**Decisão**: **M1 — reusar StateRegistry P171 com chave canónica `numbering_active:heading`** (e `numbering_active:equation` se/quando equation set rule existir).

**O4 — Magnitude**: **trivial**. Sem ADR, sem novo payload, sem novo sub-store, sem novo trait method obrigatório. Adição localizada em 1 arm de `extract_payload`.

**O5 — Reversibilidade**: alta. Se o padrão A2 (cláusula 5) ganhar ADR futura para outro caminho, a migração é local; o sub-store P171 permanece útil para outros states.

---

### Cláusula 2 — Default value

**O1 — Inputs**: `counter_state_legacy.rs:101–103` retorna `unwrap_or(false)`. Vanilla `Option<Numbering>` default `None`. Tests confirmam (`tests.rs:706` "Por defeito, numbering_active está vazio — não deve aparecer 1.").

**O2 — Alternativas**: ON / OFF.

**O3 — Critério**: paridade com cristalino actual + paridade com vanilla. Ambos OFF.

**Decisão**: **OFF**. Helper `Introspector::is_numbering_active(key)` retorna `false` quando `state_final_value(key)` for `None` ou diferente de `Value::Bool(true)`.

**O4 — Magnitude**: trivial.

**O5 — Reversibilidade**: alta — caller pode passar default explícito num overload futuro.

---

### Cláusula 3 — Lista de consumers

**O1 — Inputs**: `grep -rn "numbering_active\|is_numbering_active" 01_core/src/rules/` (8 matches além de tests).

**O2 — Alternativas**: 1 consumer (apenas heading-arm) vs N consumers reais.

**O3 — Critério**: distinguir consumer Layouter (candidato à migração via Introspector) de leitura intra-walk (consome o `state` local que o walk constrói).

**Decisão**: **2 consumers Layouter** candidatos à migração:

| # | Ficheiro | Linha | Função / arm | Forma actual |
|---|----------|-------|--------------|--------------|
| 1 | `01_core/src/rules/layout/mod.rs` | 301 | `layout_content` arm `Content::Heading` | `if self.counter.is_numbering_active("heading")` |
| 2 | `01_core/src/rules/layout/equation.rs` | 24 | `Layouter::layout_equation` | `block && self.counter.is_numbering_active("equation")` |

**Não-consumers** (leituras intra-walk; `state` local do próprio walk; não migráveis para Introspector porque o Introspector ainda não existe quando estas linhas correm):

- `01_core/src/rules/introspect.rs:360` — `resolved_text` para heading auto-label.
- `01_core/src/rules/introspect.rs:378` — gating de `step_flat("equation")`.

Estas continuam a consultar `state.is_numbering_active(…)` directo.

**O4 — Magnitude**: trivial (2 consumers, ambos triviais).

**O5 — Reversibilidade**: alta — substitution-with-fallback (P168) preserva via fallback `self.counter.is_numbering_active(…)` durante janela compat.

---

### Cláusula 4 — Localização exacta

Tabela cláusula 3 § é localização exacta. Resumo:

| Consumer | Caminho | Linha actual | Forma migrada (proposta P182B+) |
|----------|---------|--------------|---------------------------------|
| Heading prefix | `01_core/src/rules/layout/mod.rs` | 301 | `if self.introspector.is_numbering_active("heading") || self.counter.is_numbering_active("heading")` (substitution-with-fallback) |
| Equation auto-numeração | `01_core/src/rules/layout/equation.rs` | 24 | `let is_numbered = block && (self.introspector.is_numbering_active("equation") \|\| self.counter.is_numbering_active("equation"));` |

**O4 — Magnitude**: trivial.

**O5 — Reversibilidade**: alta.

---

### Cláusula 5 — Forma da API

**O1 — Inputs**: trait `Introspector` em `01_core/src/entities/introspector.rs`. Precedentes: `state_value(key, location)` / `state_final_value(key)` (P171, location-aware e final), `bib_entry_for_key(key)` / `bib_number_for_key(key)` (P181F, lookup global por chave sem location).

**O2 — Alternativas**:
- **A1** — `state_value`/`state_final_value` directo. Caller faz `matches!(v, Value::Bool(true))` + `unwrap_or(false)` em cada call-site.
- **A2** — helper `is_numbering_active(&self, key: &str) -> bool`. Encapsula matching + default OFF.

**O3 — Critério**: P181F precedente directo — `bib_*_for_key` adicionados como helpers ao trait apesar de `state_*` ter cobertura genérica. Razão: encapsular tipos concretos (BibEntry, u32) e simplificar call-site. Mesma lógica aqui — encapsular `Value::Bool` matching e default OFF; 2 call-sites Layouter ficam triviais.

A2 sem location é coerente com a semântica cristalino actual (HashMap global por chave, "última escrita ganha"). Quando StyleChain location-aware for materializada (futuro hipotético), pode-se adicionar `is_numbering_active_at(key, location)` em sobrecarga; A2 sem location não bloqueia esse caminho.

**Decisão**: **A2 — `fn is_numbering_active(&self, key: &str) -> bool`** no trait `Introspector`. Impl em `TagIntrospector` delega a `self.state.final_value(key)` + match `Value::Bool(true)` + default `false`.

**O4 — Magnitude**: trivial. ~4 linhas no trait, ~6 na impl, ~4 testes unitários.

**O5 — Reversibilidade**: alta — método é adição pura ao trait (breaking apenas para implementações externas, que não existem em L1).

---

### Cláusula 6 — Critério de fecho da lacuna #4

**O1 — Inputs**: precedente P181A (lacuna #6, Opção 3): "infraestrutura pronta + consumer migrado; fields legacy permanecem até M6".

**O2 — Alternativas**:
- Opção 1 — infra pronta apenas, mesmo sem consumer migrado.
- Opção 2 — infra + consumer + remoção do legacy (M6 implícito).
- Opção 3 — infra + consumer migrado; legacy permanece até M6 dedicado.

**O3 — Critério**: simetria com lacuna #6 (P181, Opção 3). Mecanismo escolhido (M1) é o mesmo padrão P171/P181 — não há razão para divergir do critério de fecho.

**Decisão**: **Opção 3**. Lacuna #4 é considerada fechada quando, literalmente:

1. `Introspector::is_numbering_active(key) -> bool` existe no trait + impl `TagIntrospector` delega a StateRegistry P171.
2. `extract_payload` arm `Content::SetHeadingNumbering` produz `ElementPayload::StateUpdate { key: "numbering_active:heading", update: StateUpdate::Set(Value::Bool(active)) }`; `from_tags` arm `StateUpdate` (já existente) popula `state: StateRegistry`.
3. Layouter consumers em `mod.rs:301` e `equation.rs:24` consultam `self.introspector.is_numbering_active(…)` (substitution-with-fallback `||` `self.counter.is_numbering_active(…)`).

Pendências M6: campo legacy `CounterStateLegacy.numbering_active` continua a existir; walk arm `introspect.rs:455–456` continua a popular o state local; `layout/counters.rs:11–13` continua write paralelo no layout walk; `mod.rs:1414, 1442` continuam copy-sites; leituras intra-walk (`introspect.rs:360, 378`) continuam a consultar `state.is_numbering_active(…)`. M6 elimina todos.

**O4 — Magnitude**: trivial.

**O5 — Reversibilidade**: alta — Opção 3 mantém legacy paralelo, qualquer ajuste pós-fecho é local.

---

## §4 Plano de sub-passos sem condicionais (P182B+)

| Sub-passo | Escopo | Magnitude | Depende |
|-----------|--------|-----------|---------|
| `P182B` | Trait method `is_numbering_active(&self, key: &str) -> bool` adicionado ao trait `Introspector` em `01_core/src/entities/introspector.rs`; impl em `TagIntrospector` delega a `self.state.final_value(key)` + match `Value::Bool(true)` (default `false`); tests unitários em `mod tests` (vazio devolve false; populado responde true após apply Set Bool(true); diferentes keys isoladas) | S | — |
| `P182C` | `extract_payload` arm `Content::SetHeadingNumbering` em `01_core/src/rules/introspect/extract_payload.rs` produz `ElementPayload::StateUpdate { key: "numbering_active:heading", update: StateUpdate::Set(Value::Bool(active)) }`; walk arm em `introspect.rs:455–456` continua a popular `state.numbering_active` legacy (paralelo durante janela compat M6) | S | `P182B` |
| `P182D` | Layouter consumer `01_core/src/rules/layout/mod.rs:301` migra para `self.introspector.is_numbering_active("heading") \|\| self.counter.is_numbering_active("heading")` (substitution-with-fallback P168); consumer `01_core/src/rules/layout/equation.rs:24` migra simetricamente para `"equation"` | S | `P182B`, `P182C` |
| `P182E` | Tests E2E confirmam paridade — pipeline completo (`#set heading(numbering: "1.1")` + `Heading` + Layouter) produz output equivalente via Introspector e via state legacy; teste paralelo para equation block | S | `P182D` |
| `P182F` | Lacuna #4 marcada fechada em `m1-lacunas-captura.md`; tabela §Resumo actualizada de "Adiar" para "✅ **Resolvida em P182**"; relatório consolidado P182X | S | `P182E` |

Sem cláusulas condicionais. Sem gates "FULL vs INVENTORY_ONLY". Direcção fixada: M1 + A2 + Opção 3.

---

## §5 Magnitude consolidada

**S-M** (5 sub-passos S; total agregado equivalente a P181F + P181G + P181I sem precisar P181B/D/E porque infra P171 já cobre).

Estimativa P181J §5 ("magnitude S-M para lacuna #4 com mecanismo equivalente ao P181") **mantém-se válida**. M1 reusando StateRegistry implica que não há sub-store novo (P181B análogo), não há novo `ElementPayload` variant (P181C análogo), não há promoção a locatable (P181D análogo) — apenas P181F (trait + impl) + P181G (consumer migration) + P181I (tests E2E).

---

## §6 ADR — avaliação

P182 não cria ADR. Justificação literal:

- Cláusula 1 escolhe M1 que reusa StateRegistry P171 + `ElementPayload::StateUpdate` existente. Sem nova arquitectura.
- Cláusula 5 escolhe A2 que replica precedente P181F (`bib_*_for_key`). Padrão estabelecido.
- Sem extensão de enum, sem novo sub-store, sem mecanismo novo, sem invariante revogada.

Caso edge a registar: a chave canónica `numbering_active:heading` (formato `<feature>:<key>`) é convenção — não está documentada como ADR. Se P182B+ encontrar mais features state com keys hierárquicas, pode emergir necessidade de ADR de naming. Não é o caso de P182A.

---

## §7 DEBT — avaliação

P182A não abre DEBT.

Pendências M6 (já registadas em DEBTs existentes ou no critério Opção 3):
- Field `CounterStateLegacy.numbering_active` legacy continua até M6.
- Write canonical `introspect.rs:455–456` legacy continua até M6.
- Write paralelo `layout/counters.rs:11–13` continua até M6.
- Copy-sites `mod.rs:1414, 1442` continuam até M6.
- Leituras intra-walk `introspect.rs:360, 378` continuam até M6 (ou até StyleChain location-aware).

Estas pendências já estão implícitas no critério "Opção 3 fecha #4". M6 dedicado é F1 da `auditoria-fresh-projecto.md`. Não há DEBT novo a abrir.

Caso edge: se P182C revelar interacção entre `numbering_active:heading` e fixpoint loop em `mod.rs:1435–1446` (5 iterações com clone do state inicial), pode emergir invariante a documentar. P182C resolve no momento.

---

## §8 Próximo sub-passo

**P182B** — implementação literal:

1. Adicionar método `fn is_numbering_active(&self, key: &str) -> bool;` ao trait `Introspector` em `01_core/src/entities/introspector.rs:104` (após `bib_number_for_key`).
2. Implementar em `impl Introspector for TagIntrospector` (após linha 212):
   ```rust
   fn is_numbering_active(&self, key: &str) -> bool {
       matches!(self.state.final_value(key), Some(Value::Bool(true)))
   }
   ```
3. Tests em `mod tests` (após `bib_number_for_key_em_introspector_vazio_devolve_none`):
   - vazio devolve false para qualquer key;
   - após `state.apply_set("numbering_active:heading", Value::Bool(true))`, `is_numbering_active("numbering_active:heading")` retorna true;
   - diferentes keys isoladas;
   - `Value::Bool(false)` retorna false;
   - non-Bool value retorna false.
4. `cargo test --workspace --lib` mantém 1.738 verdes (+5 novos do trait method).
5. `crystalline-lint .` zero violations.

P182B não toca consumers Layouter (esse é P182D). Não toca `extract_payload` (esse é P182C). É adição pura ao trait + impl.
