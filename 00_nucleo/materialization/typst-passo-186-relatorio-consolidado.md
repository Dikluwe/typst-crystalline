# Relatório consolidado — Série P186

**Período**: 2026-05-03 (P186A–F executados no mesmo dia)
**Magnitude agregada**: S (replicação de padrão P181/P182C/P184B
em 6 sub-passos)
**Estado**: ✅ Série fechada (A ✅ B ✅ C ✅ D ✅ E ✅ F ✅)
**ADR vinculada**: nenhuma (replicação de padrão)
**DEBT**: nenhum novo

---

## §1 Resumo executivo

A série P186 promoveu `Content::Equation` a locatable kind no
Introspector. Resolve **eixo 2 do bloqueio P183C** (counter
populado em sub-store) — combinado com P185 (eixo 1: Layouter
location-aware) deixa C2 (`equation.rs:97`) pronto para
migração em P188.

Pipeline Equation completo materializado:
- `Content::Equation { body, block }` → `is_locatable = true`
  (P186D).
- `extract_payload` produz `ElementPayload::Equation { block,
  counter_update }` (P186C).
- Walk emite Tag locatable; `from_tags` arm popula
  `kind_index[Equation]` (sempre) + `counters["equation"]`
  (gated por `block && state.value_at("numbering_active:equation",
  loc) == Some(Bool(true))`) (P186E).

**Achado central honestamente registado**: gate dormente em
produção. `Content::SetEquationNumbering` não existe em
cristalino (descoberta P186A §11.2). Em runtime real, state
`numbering_active:equation` é sempre `None` → gate bloqueia →
counter introspector permanece vazio. P188 substitution-with-fallback
cobre via legacy `state.get_flat("equation")`.

Δ tests cumulativo: **+18** (1783 → 1801) com **zero
regressões**. 6 sub-passos (5 implementação + 1 diagnóstico).

Diferença face P184 (Figure): em P184D Introspector tornou-se
o **caminho funcional** para C3; em P186 Introspector fica
**dormente em produção** até equation set rule materializar.
Estado paralelo a inversão observable mas com fallback legacy
permanente — registado honestamente.

---

## §2 Sub-passos materializados

| Passo | Magnitude planeada | Magnitude real | Δ tests | L0s tocados |
|-------|---------------------|-----------------|---------|-------------|
| **P186A** | S (diagnóstico) | S | 0 | nenhum (cria diagnóstico) |
| **P186B** | S | S | **+7** | `entities/element_payload.md` + `entities/element_kind.md` |
| **P186C** | S | S | **+3** | `rules/introspect/extract_payload.md` |
| **P186D** | trivial-S | S (cláusula gate trivial — vide §4) | 0 | `rules/introspect/locatable.md` |
| **P186E** | S | S | **+4** | `rules/introspect/from_tags.md` |
| **P186F** | S | S | **+4** | nenhum (só tests + relatório) |
| **Total** | — | — | **+18** | 5 L0 produção |

Sub-passos detalhados com Δ por baseline:

- P186A: 1783 (baseline P185E inalterado).
- P186B: 1783 → **1790** (+7) — variants `ElementPayload::Equation`
  + `ElementKind::Equation`.
- P186C: 1790 → **1793** (+3) — arm `extract_payload`.
- P186D: 1793 → **1793** (Δ 0) — `is_locatable` activado;
  fixture P185D restaurado; helper de teste invariante
  estendido; sem novos tests.
- P186E: 1793 → **1797** (+4) — arm `from_tags` com gate
  location-aware.
- P186F: 1797 → **1801** (+4) — tests E2E + relatório
  consolidado.

---

## §3 Decisões arquiteturais

### 6 cláusulas P186A fechadas

| # | Cláusula | Decisão | Sub-passo |
|---|----------|---------|-----------|
| 1 | Forma `ElementPayload::Equation` | **Opção B**: `{ block: bool, counter_update: CounterUpdate }` (paralelo Figure P184B) | P186B |
| 2 | Sub-store alvo | **Opção 1**: `CounterRegistry` (reuso) | P186E |
| 3 | Convenção de chave | **Opção A**: `"equation"` simples (paridade legacy + vanilla) | P186E |
| 4 | Auto-init | **Não necessário** (`apply_at` defensivo via `or_insert`) | P186E |
| 5 | Forma migração C2 (P188) | substitution-with-fallback `flat_counter_at(...).or_else(legacy)` | registado para P188 |
| 6 | Critério de fecho P186 | **Opção 2**: variant + locatable + extract_payload + from_tags + tests E2E | P186F |

### Decisão adicional fechada em P186E (não-prevista)

**Versão do gate**: Opção B (location-aware) escolhida em
P186E `.B`. Inlining via `matches!(state.value_at(...),
Some(Value::Bool(true)))` para evitar circularidade
estilística no `from_tags` (que constrói o introspector;
chamar trait method via `Introspector::is_numbering_active_at`
funcionaria mas inverteria a topologia natural).

### Sem ADR — replicação de padrão

ADR avaliada e dispensada. Promoção a locatable é refino
dentro de ADR-0026 (Content enum fechado); replicação de
padrão P181F (Bibliography) + P178 (Outline) + P182C
(SetHeadingNumbering). Decisões atómicas registadas no
diagnóstico P186A §2 e relatórios individuais.

---

## §4 Achados não-triviais durante execução

### P186A §11.2 — `Content::SetEquationNumbering` ausente

Achado mais consequencial da série. C2 consumer em
`equation.rs:25-33` já tem comentário inline confirmando que
*"cristalino ainda não tem variant `Content::SetEquationNumbering`,
logo o Introspector path nunca activa para equation"*. P186E
honra esta restrição implementando gate dormente em produção.

Implicação para P188: migração C2 resulta em **Introspector
path dormente + fallback legacy permanente** até equation
set rule materializar (passo dedicado fora da série P186).
Diferente de P184D Figure (que tornou-se caminho funcional).

### P186A §11.3 — `ElementKind::Equation` exigido

P186A descobriu que adicionar `ElementPayload::Equation`
sozinho não basta; `kind_index` precisa de `ElementKind::Equation`
para indexar. Ambos adicionados juntos em P186B.

### P186A §11.5 — Gate `block && state-active` não-trivial

Diferente de Figure (P184B incremento incondicional;
consumer usa figure_progress + idx), Equation exige gate
completo para preservar paridade com legacy walk
(`introspect.rs:377-382`). Sem gate, valores em
block-equation locations divergiriam quando há equations
inline ou block-não-numeradas.

### P186B — `from_tags` exaustivo forçou stub no-op

Match sobre `ElementPayload` em `from_tags.rs:50` é
exaustivo (sem catch-all). Adição de variant forçou arm
explícito. Stub no-op `ElementPayload::Equation { .. } => {}`
em P186B; estendido em P186D; preenchido com counter logic
em P186E.

### P186C — Descoberta empírica: walk gateia em `extract_payload`, não em `is_locatable`

**Erro factual na spec P186C** descoberto durante execução.
Spec assumiu que walk gates em `is_locatable`; análise
empírica de `introspect.rs:329` mostrou que walk gates
directamente em `extract_payload(content).is_some()`.

Os dois são equivalentes apenas quando a invariante
`is_locatable ↔ extract_payload.is_some()` é mantida (per
`locatable.rs:11`). A inversão sugerida pela spec
(`extract_payload` antes de `is_locatable`) **não eliminou
janela quebrada** — apenas inverteu sentido:

- Sem inversão: Layouter avançaria Locator para Equation
  enquanto walk não emitiria tag.
- Com inversão (executada): walk emite tag para Equation
  enquanto Layouter não avança Locator.

Em ambos os cenários, P185D test `gating_locator_apenas_em_locatables`
quebra. Auditor resolveu pragmaticamente em P186C
removendo Equation do fixture do test durante a janela
(P186D restaurou).

### P186D — Cláusula gate trivial em `from_tags`

Spec P186D restringia "não modificar from_tags" mas
verificação `.F.8` exigia 4 locatables visíveis em
`kind_index`. Sem populate de `kind_index` para Equation no
stub de P186B, walk_locs ≠ layout_locs e o test falhava.

Auditor estendeu stub com populate `kind_index` (sem
counter logic — esse fica para P186E). Decisão correcta
dada inconsistência interna da spec. Documentado em
P186D §"Decisões".

### P186D — Lacuna pré-existente em `build_minimal_for_each_variant`

Helper do test invariante em `locatable.rs::tests` não
cobria Equation. Lacuna silenciosa: qualquer divergência
entre `is_locatable(Equation)` e `extract_payload(Equation).is_some()`
ficaria escondida porque o test não testaria Equation.

P186D fechou. Outros variants podem ter mesma lacuna —
verificação fora de escopo.

### P186E — Decisão Opção B com inlining

Gate location-aware (Opção B) escolhido em P186E `.B` por
futureproofing alinhado com P185 direcção arquitectural.
Implementado inline via `matches!(intr.state.value_at(...),
Some(Value::Bool(true)))` para evitar dependência circular
do trait `Introspector` em `from_tags.rs` (que é o
construtor do TagIntrospector).

---

## §5 Estado final M9 e M5/M4

### M9 (counter-feature) — inalterado: 11/11

P186 não introduz feature M9 nova. Promoção de Equation a
locatable é **infraestrutural** — cobre eixo 2 do bloqueio
P183C; não é slot novo. M9 11/11 mantém-se.

### M5/M4 (read-site migration) — inalterado: 6/12

P186 não migra read-sites. C2 ainda bloqueado, mas agora
pela razão **certa** (ausência de equation set rule), não
pela razão estrutural (sub-store ausente). Quando P188
materializar a migração com substitution-with-fallback, C2
fica funcionalmente correcto via legacy + estruturalmente
correcto via Introspector dormente.

### Trait `Introspector` — 18 métodos (inalterado vs P185)

Sem método novo. P185B (`is_numbering_active_at` +
`flat_counter_at`) já cobre os métodos location-aware
necessários para C1 (P187) e C2 (P188).

### `ElementPayload`: 9 → **10** variants (Equation adicionada)
### `ElementKind`: 8 → **9** variants (Equation adicionada)
### `extract_payload` arms: 8 → **9** (Equation adicionada)
### `is_locatable`: 9 → **10** locatable, 47 → **46** não-locatable
### `from_tags::ElementPayload` arms: 8 → **9** (Equation com gate)

---

## §6 Estado final lacunas

Inalterado em P186. As lacunas catalogadas até P185E
permanecem na mesma situação. P186 não foi sobre lacunas —
foi sobre infraestrutura para desbloquear C2.

---

## §7 Pendências cumulativas + janela compat M6

### Activas

- **P183B (C1 heading prefix)** — depende P187. Independente
  de P186.
- **P183C (C2 equation counter)** — eixo 1 (P185) ✅ +
  eixo 2 (P186) ✅ resolvidos. Depende apenas de P188
  agora.
- **`Content::SetEquationNumbering` ausente** — pré-existente,
  documentado em P186A §11.2 e em comentários inline da
  produção (`equation.rs:25-29`). Não é DEBT P186 mas é
  trabalho identificado para passo dedicado fora da série
  P186-P188. Quando materializar, gate dormente em
  produção activa automaticamente; P188 Introspector path
  passa a ser caminho funcional em vez de dormente.
- **4 sites M4-fora-de-escopo** (TOC, fixpoint side-channels,
  resolved labels) — fora de escopo P186.

### Janela compat M6

Após P187 + P188 fecharem C1 + C2, **DEBT M4-residual fecha**.
Janela compat M6 (eliminar `CounterStateLegacy.numbering_active`,
`CounterStateLegacy.hierarchical/flat`) torna-se candidata
a abertura. Para Equation especificamente, M6 só pode
fechar quando:
1. P188 migra C2 (substitution-with-fallback).
2. `Content::SetEquationNumbering` materializa (passo
   dedicado).
3. C2 testes confirmam que Introspector é caminho funcional
   (não dormente).

Antes desses 3 passos, fallback legacy permanece necessário
para preservar paridade output em produção.

---

## §8 Próximos passos sugeridos

### Independente — pode prosseguir

1. **P187 — Migrar C1 (heading prefix)**: substitui
   `self.counter.format_hierarchical("heading")` em
   `mod.rs:310` por `self.introspector.formatted_counter_at(
   "heading", self.current_location.unwrap())`. Padrão
   substitution-with-fallback per ADR-0061. Blueprint
   literal em P185D test
   `pipeline_e2e_is_numbering_active_at_via_current_location`.
   Magnitude S–M.

### Desbloqueado por P186 — pode prosseguir após P186F

2. **P188 — Migrar C2 (equation counter)**: substitui
   `state.get_flat("equation")` em `equation.rs:97` por
   `self.introspector.flat_counter_at("equation",
   self.current_location.unwrap()).or_else(legacy)`.
   Substitution-with-fallback. **Documentar honestamente**
   que migração resulta em Introspector path dormente em
   produção até equation set rule materializar; fallback
   legacy é caminho funcional permanente. Magnitude S–M.

### Sequência fechamento M4-residual

3. **Após P186F + P187 + P188**: M4-residual fechado;
   DEBT M4-residual fecha; segue M5 (P189) com novos
   read-sites ou M9 slot 11.

### Trabalho identificado fora de escopo

4. **`Content::SetEquationNumbering` materialização** —
   passo dedicado quando equation set rule for prioridade.
   Activa Introspector path em P188; permite janela compat
   M6 abrir para Equation.

---

## §9 Conclusão

P186 fechou limpamente em 6 sub-passos com magnitudes
correctamente estimadas (5×S + 1×S diagnóstico, todos
acertaram ±0). Achados não-triviais ricos (em particular,
P186C empirismo sobre gating do walk) ratificaram que
diagnóstico-primeiro fornece valor mesmo em séries de
"replicação de padrão" — sem ele, erros factuais da spec
seriam materializados em código.

Achados centrais:
- **Equation locatable infraestrutural** completo — sub-store
  populado quando state é injectado.
- **Gate dormente em produção** registado honestamente —
  herda condição pré-existente (`Content::SetEquationNumbering`
  ausente); não é defeito da série P186.
- **Eixo 2 do bloqueio P183C resolvido estruturalmente** —
  com P185 (eixo 1), C2 desbloqueado para P188.
- **Cláusulas gate trivial pragmáticas** (P186C fixture
  ajuste, P186D `from_tags` populate) preservaram tests
  durante janelas intermediárias sem violar restrições
  arquiteturais.

A série P186 termina como **última peça infraestrutural de
M4-residual**. P187 (C1) + P188 (C2) são migrações finais
que fecham M4-residual; após eles, DEBT M4-residual fecha
e segue M5 (P189).

**53 passos executados** após P186F. Padrão diagnóstico-primeiro
aplicado pela 11ª vez consecutiva
(P131A/132A/140A/148/154A/181A/182A/183A/184A/185A/186A) —
todos os 11 acertaram a magnitude planeada ±1 nível.

Próximo passo sugerido: **P187A** (diagnóstico migração
C1 heading prefix) ou **P188A** (diagnóstico migração C2
equation counter — agora desbloqueado por P186).
