# Passo P190G — Categoria Labels & TOC (1ª aplicação ADR-0071)

Sexto passo de implementação P190 (após P190A, B,
C, D, E, F + ramo paralelo P191A-C). Magnitude **M**
— 1ª aplicação directa do mecanismo P191/ADR-0071
em série P190.

P191 série fechada confirmou empiricamente:
- Walk fn aceita `&mut TagIntrospector`.
- 2 helpers walk-readers migrados
  (`compute_heading_auto_toc`, `compute_labelled`).
- `from_tags::from_tags` eliminado.
- Pattern ADR-0069 stylesheet preservado.
- ADR-0071 ACEITE.

P190G retoma série P190 com **mecanismo arquitectural
desbloqueado**. 3 defers acumulados agora resolvíveis
nesta categoria:
- `numbering_active` (P190E) — via
  `intr.is_numbering_active_at`.
- `flat` (P190F) — via `intr.flat_counter_at` (se
  aplicável a Labels).
- `hierarchical` (P190F) — via
  `intr.formatted_counter_at` (se aplicável).

Trabalho concreto (categoria 6 Labels & TOC):
1. **Eliminar campo `resolved_labels`** —
   `HashMap<Label, String>` — caminho Introspector
   activo desde P193B (`intr.resolved_label_for`);
   walk arm Labelled muta legacy como write paralelo
   M5; eliminar mutação após confirmar nenhum walk
   reader.
2. **Eliminar campo `headings_for_toc`** —
   `Vec<(Label, Content, usize)>` — caminho
   Introspector activo desde P200B
   (`intr.headings_for_toc()`); walk arm Heading
   muta legacy.
3. **Eliminar/migrar campo `auto_label_counter`**
   — `usize` — walk-internal (incrementado por walk
   arm Heading; lido por
   `compute_heading_for_toc` walk-internal e
   `compute_heading_auto_toc` migrado P191B).
   - **Decisão obrigatória**: pode ser local var em
     walk fn ou continuar como state field até
     P190I (eliminação struct).

Trabalho derivado (cleanup defer):
- **Resolver defer `numbering_active`** se walk arm
  Heading + walk arm Equation gate já não lêem
  legacy (verificar empíricamente após P191).

Após P190G:
- `CounterStateLegacy`: 10 → **7-8 fields** (3
  campos eliminados; +1 se `auto_label_counter`
  preservado).
- 2 walk arm mutações eliminadas
  (`resolved_labels.insert`,
  `headings_for_toc.push`).
- Layouter consumers migrados (se houver).
- Possível resolução defer `numbering_active`.
- Pattern "eliminação write paralelo M5": 6ª
  aplicação concreta.
- 1ª aplicação directa ADR-0071 mecanismo em P190.

**Pré-condição**: P191C concluído (ADR-0071
ACEITE). Tests workspace ~1.832 verdes; zero
violations. `CounterStateLegacy` 10 fields. Tracker
`p190-pause-resume-tracker.md` indica P190G.

**Restrições**:
- **Não** modificar walk fn signature (já tem
  `&mut TagIntrospector` desde P191B).
- **Não** modificar trait `Introspector`.
- **Não** modificar `TagIntrospector` struct
  fields.
- **Não** eliminar struct `CounterStateLegacy` —
  P190I.
- **Não** modificar 2 helpers walk-internal
  (`compute_figure`, `compute_heading_for_toc`)
  — P190H/I.
- **Não** modificar Layouter — P190I excepto se
  consumers desta categoria existirem.
- **Não** materializar lacunas residuais.
- API pública preservada.
- **Lembrete crítico**: 2 sub-passos restantes
  (P190H, P190I) + outros defers após P190G.

---

## Sub-passos

### .A Auditoria L0

#### Inventário 3 fields (categoria 6)

1. Confirmar fields em
   `01_core/src/entities/counter_state_legacy.rs`:
   - `pub resolved_labels: HashMap<Label, String>`.
   - `pub headings_for_toc: Vec<(Label, Content,
     usize)>`.
   - `pub auto_label_counter: usize`.
   - Type signatures exactos.

#### Inventário walk arm mutações

2. Identificar mutações em walk arms (per histórico
   M5):
   - **`state.resolved_labels.insert(...)`** — walk
     arm Labelled (P195D) ou outros.
   - **`state.headings_for_toc.push(...)`** — walk
     arm Heading (mutação 4 P196B/P200B).
   - **`state.auto_label_counter += 1`** — walk arm
     Heading.

3. Confirmar empíricamente cada mutação:
   - `grep -n "resolved_labels\." 01_core/src/rules/introspect.rs`.
   - `grep -n "headings_for_toc\." 01_core/src/rules/introspect.rs`.
   - `grep -n "auto_label_counter" 01_core/src/rules/introspect.rs`.

#### Inventário walk readers (DURANTE walk)

4. Identificar walk readers de `state.resolved_labels`:
   - **Esperado**: nenhum (helper `compute_labelled`
     migrado P191C lê via Introspector).
   - Verificar empíricamente via grep.

5. Identificar walk readers de `state.headings_for_toc`:
   - **Esperado**: nenhum (consumer outline.rs lê
     pós-walk via Introspector path desde P200B).

6. Identificar walk readers de `state.auto_label_counter`:
   - **Esperado**: walk arm Heading + helpers
     `compute_heading_for_toc` (walk-internal,
     P200B) + `compute_heading_auto_toc` (migrado
     P191B).
   - **Cláusula gate substancial**: se
     `compute_heading_auto_toc` migrado lê
     `auto_label_counter` via parameter, walk arm
     Heading passa-o explicitamente — `state` field
     pode ser eliminado.
   - Verificar signature P191B
     `compute_heading_auto_toc`.

#### Inventário Layouter consumers

7. Identificar consumers Layouter:
   - `grep -rn "self.counter.resolved_labels\|self.counter.headings_for_toc\|self.counter.auto_label_counter"
     01_core/src/`.
   - Esperado: nenhum (Layouter consumer outline
     migrado P200B).

8. Identificar Layouter assignments duais (per
   padrão P190B-F):
   - `grep -rn "resolved_labels\s*=\|headings_for_toc\s*=\|auto_label_counter\s*="
     01_core/src/rules/layout/mod.rs`.

#### Inventário cleanup defers

9. Confirmar estado defer `numbering_active`:
   - Walk arm Heading lê
     `state.is_numbering_active("heading")`?
     - Se **não** (helper P191B migrado para
       Introspector): defer resolvível.
     - Se **sim**: continua deferido para P190H ou
       I.
   - Walk arm Equation lê
     `state.is_numbering_active("equation")`?
     - **Esperado não** (P191B migrou gate).

10. Confirmar estado defer `lang`:
    - Walk arm Labelled (caller `compute_labelled`)
      passa `state.lang` via parameter desde P191C.
    - Field `state.lang` ainda lido durante walk?
      - Se **não** (apenas walk arm Labelled lê para
        passar a helper): defer resolvível em P190H
        (categoria Figures) ou P190I.

#### Tests dependentes

11. Identificar tests:
    - Tests sentinela mutação legacy
      `walk_arm_*_resolved_labels_via_legacy` ou
      similar.
    - Tests Layouter outline (preservar — semântica
      observable).
    - Tests `auto_label_counter` directos.

#### L0 alvos

12. Identificar L0s:
    - `entities/counter_state_legacy.md` (defer
      P190I para eliminação struct).
    - `rules/introspect.md` (walk arms purificados;
      mutações eliminadas).
    - Possivelmente outros.

Output: tabela com item + estado verificado.

**Critério de saída**:
- 3 fields localizados.
- Mutações walk arm catalogadas.
- Walk readers confirmados (esperado: zero ou
  apenas parameter passing).
- Layouter consumers identificados.
- Cleanup defers `numbering_active` + `lang`
  estado avaliado.
- Tests dependentes listados.

### .B Eliminar mutação `state.resolved_labels.insert`

Per `.A.2` e `.A.4`:

1. Localizar mutação em walk arm (provavelmente
   Labelled — P195D).

2. **Cláusula gate trivial**: confirmar que arm
   Labelled emite Tag::Labelled que popula
   `intr.resolved_labels` via populate_intr (per
   P191B). Sem isto, eliminação quebra paridade.

3. Eliminar `state.resolved_labels.insert(...)`.

4. Comentário inline P190G substitui ou actualiza
   comentário inline P195D sobre write paralelo M5.

5. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- Mutação eliminada.
- Walk arm puro para `resolved_labels`.

### .C Eliminar mutação `state.headings_for_toc.push`

Per `.A.2` e `.A.5`:

1. Localizar mutação em walk arm Heading (mutação
   4 — P196B/P200B).

2. **Cláusula gate trivial**: confirmar que walk
   arm Heading emite Tag::HeadingForToc
   pós-recursão (P200B) que popula
   `intr.headings_for_toc` via populate_intr.

3. Eliminar `state.headings_for_toc.push(...)`.

4. Comentário inline P190G actualiza comentário
   P200B sobre write paralelo M5.

5. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- Mutação eliminada.
- Walk arm Heading puro para `headings_for_toc`.

### .D Decisão `auto_label_counter` (eliminar ou local var)

Per `.A.6`:

**Decisão obrigatória**: 3 opções:

- **Opção α** (eliminar field; usar local var em
  walk fn):
  - Walk fn ganha local var `let mut
    auto_label_counter: usize = 0;`.
  - Incrementação inline em walk arm Heading.
  - Helpers `compute_heading_for_toc` e
    `compute_heading_auto_toc` recebem counter via
    parameter.
  - Field eliminado de `CounterStateLegacy`.
  - Magnitude: marginal — refactor incremental.

- **Opção β** (preservar field até P190I):
  - Field continua em `CounterStateLegacy`.
  - Mutação preservada como write paralelo
    desnecessário (não há reader externo).
  - Eliminação completa em P190I.
  - Magnitude: zero (defer).

- **Opção γ** (mover para `LayouterRuntimeState`):
  - Walk não tem acesso. Improvável.

Sugestão preliminar: **Opção α** — incremental;
field tornar-se local var preserva semântica e
elimina campo do struct.

Output: opção materializada.

### .E Eliminar/migrar `auto_label_counter` per `.D`

**Per Opção α**:

1. Em walk fn:
   - Adicionar local var `let mut auto_label_counter:
     usize = 0;`.

2. Em walk arm Heading:
   - `state.auto_label_counter += 1` →
     `auto_label_counter += 1`.

3. Em chamadas a helpers:
   - `compute_heading_for_toc(state, ...)` →
     adicionar parameter `auto_label_counter`.
   - `compute_heading_auto_toc(intr, location,
     auto_label_counter)` — already accepts (per
     P191B signature).

4. Adaptar 2 helpers para receber parameter
   `auto_label_counter`:
   - `compute_heading_for_toc` — adicionar
     parameter (walk-internal; P200B helper).
   - `compute_heading_auto_toc` — confirmar
     signature já aceita (per P191B).

5. Confirmar `cargo check --workspace` passa.

**Per Opção β**: nenhum trabalho neste sub-passo;
defer P190I.

**Critério de saída**:
- Per `.D` decisão materializada.

### .F Eliminar 3 fields (per `.D`)

Per `.A.1`:

1. Em
   `01_core/src/entities/counter_state_legacy.rs`:
   - Eliminar `pub resolved_labels`.
   - Eliminar `pub headings_for_toc`.
   - **Per Opção α `.D`**: eliminar
     `pub auto_label_counter`.
   - **Per Opção β `.D`**: preservar
     `auto_label_counter` (eliminar em P190I).

2. Adaptar `Default` impl ou `new()` constructor.

3. Confirmar `cargo check --workspace` passa.

4. **`CounterStateLegacy`: 10 → 7 fields** (Opção
   α) ou **8 fields** (Opção β).

**Critério de saída**:
- Fields eliminados per decisão.

### .G Eliminar Layouter assignments duais

Per `.A.8`:

1. Eliminar linhas de assignment para fields
   eliminados.

2. Comentário inline P190G actualiza.

3. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- Assignments eliminados.

### .H Resolver cleanup defer `numbering_active` (se aplicável)

Per `.A.9`:

**Caso 1**: walk arm Heading + walk arm Equation
gate já não lêem `state.is_numbering_active`
(P191B migrou Equation gate; P191B/C podem ter
migrado Heading via `compute_heading_auto_toc`):
- Eliminar mutações walk arm SetHeadingNumbering
  + SetEquationNumbering (write paralelo
  desnecessário).
- Eliminar field `numbering_active`.
- **`CounterStateLegacy`: 7 → 6 fields** (Opção α)
  ou **7** (Opção β).

**Caso 2**: ainda há walk reader:
- Defer `numbering_active` continua para P190H ou
  P190I.

**Decisão obrigatória empírica**.

Output: caso materializado.

### .I Adaptar tests

1. Identificar tests afectados (per `.A.11`):
   - Tests sentinela mutação legacy redundantes.
   - Tests `auto_label_counter` directos.
   - Tests Layouter outline (preservar).

2. Adaptação:
   - Tests redundantes — remover.
   - Tests adaptáveis — substituir.

3. Tests workspace verdes (Δ esperado: marginal
   negativo).

**Critério de saída**:
- Tests adaptados.

### .J Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs
   P191C baseline (~1.832): **marginal**.
3. `crystalline-lint .` zero violations.
4. `CounterStateLegacy.resolved_labels` **NÃO
   existe**.
5. `CounterStateLegacy.headings_for_toc` **NÃO
   existe**.
6. **Per Opção α `.D`**:
   - `CounterStateLegacy.auto_label_counter` **NÃO
     existe**.
   - Walk fn tem local var.
   - Helpers recebem parameter.
7. **Per Opção β `.D`**:
   - Field `auto_label_counter` preservado.
8. **Per Caso 1 `.H`**:
   - `CounterStateLegacy.numbering_active` **NÃO
     existe**.
   - Mutações walk arm SetHeadingNumbering +
     SetEquationNumbering eliminadas.
   - Walk arms ficam puros para esta categoria.
9. **Per Caso 2 `.H`**: defer continua.
10. `CounterStateLegacy`: 6 (Opção α + Caso 1), 7
    (Opção α + Caso 2 ou Opção β + Caso 1), ou 8
    (Opção β + Caso 2) fields.
11. Walk arm Labelled mutação `resolved_labels`
    eliminada.
12. Walk arm Heading mutação `headings_for_toc`
    eliminada.
13. Layouter assignments duais eliminados.
14. Comentários inline P190G presentes.
15. Trait `Introspector` **NÃO modificado**.
16. `TagIntrospector` fields **NÃO modificados**.
17. 2 helpers walk-internal (`compute_figure`,
    `compute_heading_for_toc`) — P190G pode tê-los
    adaptado para receber parameter
    `auto_label_counter` per `.E`. Verificar.
18. ADR-0070 PROPOSTO **NÃO transitada** (ACEITE
    em P190I).
19. Snapshot tests verdes.
20. Linter passa final.

### .K Encerramento

Escrever
`00_nucleo/materialization/typst-passo-190g-relatorio.md`
com:

- Resumo: categoria 6 (Labels & TOC) eliminada;
  3 fields eliminados (Opção α) ou 2 fields
  eliminados (Opção β); possível resolução defer
  `numbering_active`; 1ª aplicação directa
  ADR-0071 mecanismo em P190.
- Confirmação `.J` (20 verificações).
- Δ tests vs baseline P191C.
- Hashes finais.
- Decisões de execução notáveis:
  - Opção α vs β em `.D` (`auto_label_counter`).
  - Caso 1 vs Caso 2 em `.H` (defer
    `numbering_active`).
  - Padrão "1ª aplicação directa ADR-0071 em P190"
    estabelecido.
- Estado actual:
  - P190 série: A ✅ B ✅ C ✅ D ✅ E ✅ F ⚠️
    G ✅ | H-I pendentes.
  - **Categoria 6 (Labels & TOC) fechada**.
  - 95 passos executados.
- Pendências cumulativas: 2 categorias restantes
  + P190I + (eventualmente) defers remanescentes
  (`flat`, `hierarchical`, `lang`, possivelmente
  `numbering_active`).
- Próximo passo: P190H — categoria 7 (Figures).
  Magnitude M.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate
   substancial inesperado.
2. Mutação `resolved_labels.insert` eliminada
   (`.B`).
3. Mutação `headings_for_toc.push` eliminada
   (`.C`).
4. Decisão `.D` materializada
   (`auto_label_counter`).
5. `auto_label_counter` eliminado/migrado per `.D`
   (`.E`).
6. 2-3 fields eliminados (`.F`).
7. Layouter assignments eliminados (`.G`).
8. Decisão `.H` materializada (`numbering_active`
   defer).
9. Tests adaptados (`.I`).
10. Verificações `.J` passam (20/20).
11. Output observable em produção inalterado.
12. **1ª aplicação directa ADR-0071 mecanismo em
    P190 estabelecida**.
13. Relatório `.K` escrito.

---

## O que pode sair errado

- **Walk readers de `resolved_labels` ou
  `headings_for_toc` em sítios não previstos**:
  cláusula gate substancial — investigar
  empíricamente.
- **Layouter consumers ainda existem** (não
  esperados): cláusula gate trivial — migrar.
- **Helpers `compute_heading_for_toc` ainda lê
  `state.auto_label_counter` directamente**
  (impossível adicionar parameter em walk-internal
  helper): cláusula gate substancial — refactor
  signature ou Opção β.
- **Mutação `resolved_labels` em sítio não
  previsto** (e.g., outro walk arm): cláusula gate
  trivial.
- **Caso 2 em `.H`** (walk reader
  `numbering_active` ainda existe): defer continua;
  trabalho mais reduzido em P190G mas previsto.
- **Tests sentinela em quantidade significativa
  regridem**: padrão pragmático auditor #1.
- **Snapshot tests divergem**: improvável
  (caminho Introspector activo desde P193B/P200B);
  investigar.
- **Linter divergência V13/V14**: cláusula gate
  trivial.

---

## Notas operacionais

- **Tamanho**: M. ~30-50 LOC produção (mutações
  eliminadas + auto_label_counter refactor + fields
  eliminados + assignments) + ~20 LOC tests
  adaptados + ~20 LOC L0 (defer P190I).
- **Sem dependências externas novas**.
- **Sem ADR nova**.
- **1ª aplicação directa ADR-0071 mecanismo em
  P190** — pré-condição arquitectural P191
  exercitada empíricamente.
- **Pattern "eliminação write paralelo M5"**: 6ª
  aplicação concreta.
- **Cláusula gate trivial**: aplicável a forma
  exacta de signatures, recálculo de hashes,
  adaptação tests.
- **Cláusulas gate substancial possíveis**: walk
  readers em sítios não previstos; helpers
  walk-internal com signatures rígidas.
- **Próximo passo P190H**: categoria 7 (Figures).
  Magnitude M. Trabalho concreto:
  - Eliminar campos `figure_numbers`,
    `figure_label_numbers`, `local_figure_counters`.
  - Resolver defer `lang` se walk arm Labelled (que
    passa `state.lang` ao helper) for último
    consumer.
  - 2ª aplicação directa ADR-0071 mecanismo.
- **F1 progresso**: 10 → 6-8 fields ortogonais
  (depende decisões).
- **F3 progresso**: Layouter ainda 20 fields;
  inalterado em P190G.
- **Lembrete crítico**: 2 sub-passos restantes
  P190 série (P190H + P190I). Após P190I, ADR-0070
  ACEITE + M6 fechado + F1 fecha + F3 parcialmente
  fecha.
