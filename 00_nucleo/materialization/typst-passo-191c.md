# Passo P191C — Encerramento série P191 + ADR-0071 ACEITE + LEMBRETE P190 RETOMAR

Segundo e último passo de implementação P191 (após
P191A diagnóstico, P191B prova de conceito).
Magnitude **S-M** — passo combinado:
- Migrar último helper walk-reader
  (`compute_labelled`) usando mecanismo já validado.
- Encerramento série com relatório consolidado.
- ADR-0071 PROPOSTO → ACEITE.
- **Lembrete formal CRÍTICO**: P190 série em pausa
  — retomar P190G após P191C fechar.

P191B confirmou empíricamente:
- Mecanismo Opção A funcional.
- 25 recursive call sites actualizados.
- `from_tags::from_tags` eliminado (969 LOC); 273
  LOC preservados (`apply_state_funcs` para Func
  eval em fixpoint).
- `compute_heading_auto_toc` migrado com signature
  genérica `<I: Introspector>`.
- Walk arm Equation gate migrado.
- 5 cláusulas gate substanciais resolvidas
  empíricamente.
- LOC líquido -699 (refactor simplifica).
- 28ª aplicação diagnóstico-primeiro consecutiva.

P191C aplica o mecanismo já provado para
`compute_labelled` (mais complexo — 4 arms vs 1 do
helper anterior).

Trabalho concreto:
1. Migrar `compute_labelled` para signature
   genérica `<I: Introspector>(intr: &I, target:
   &Content, location: Location) -> (Option<String>,
   Option<usize>)` (forma exacta per empírico).
2. Adaptar caller (walk arm Labelled — pattern
   P195D não-locatable com snapshot+find_map).
3. Auditoria empírica final.
4. Relatório consolidado P191 com 9 secções padrão.
5. ADR-0071 transitada PROPOSTO → ACEITE.
6. Lembrete formal CRÍTICO em ficheiro dedicado.

Após P191C:
- Mecanismo Opção A completo (2 helpers
  walk-readers migrados).
- 2 helpers walk-internal preservados
  (`compute_figure`, `compute_heading_for_toc`).
- Pattern ADR-0069 stylesheet preservado.
- ADR-0071 ACEITE.
- Pré-condição arquitectural cumprida para
  retomar P190G.
- **Tracker P190 retomar formalizado**.

**Pré-condição**: P191B concluído. Tests workspace
1.832 verdes; zero violations. Mecanismo Opção A
validado. ADR-0071 PROPOSTO ainda activa.

**Restrições**:
- **Não** modificar trait `Introspector`.
- **Não** modificar `TagIntrospector` struct
  fields.
- **Não** eliminar `CounterStateLegacy` — P190I
  após retomar P190 série.
- **Não** eliminar campos `CounterStateLegacy`
  walk-readable — P190G/H/I.
- **Não** modificar Layouter — P190G+.
- **Não** materializar lacunas residuais.
- API pública preservada.
- **LEMBRETE CRÍTICO**: após P191C, P190 série
  retoma. Não esquecer.

---

## Sub-passos

### .A Auditoria L0 + estado pós-P191B

#### Estado consolidado pós-P191B

1. Tests workspace 1.832 verdes.
2. Linter zero violations.
3. Mecanismo Opção A validado:
   - Walk fn signature aceita `&mut TagIntrospector`.
   - 25 recursive call sites actualizados.
   - `from_tags::from_tags` eliminado.
   - 12 walk arms popularem `intr` directamente.
   - `compute_heading_auto_toc` migrado.
   - Walk arm Equation gate migrado.

#### Inventário `compute_labelled`

4. Localizar `compute_labelled` em
   `01_core/src/rules/introspect.rs`:
   - Signature actual: `fn compute_labelled(state:
     &CounterStateLegacy, target: &Content, ...) ->
     (Option<String>, Option<usize>)` (forma
     aproximada).
   - **4 arms** internos (per P191B §14):
     - Equation arm: lê `state.flat`,
       `state.numbering_active`.
     - Figure arm: lê `state.figure_numbers`,
       `state.lang`.
     - Heading arm.
     - Outros.
   - Reads de state legacy: identificar
     empíricamente cada arm.

5. Confirmar caller único (walk arm Labelled — P195D
   variante não-locatable).

6. Confirmar pattern P195D variante (não-locatable +
   snapshot+find_map):
   - Walk arm Labelled emite Tag pós-recursão.
   - Helper chamado com snapshot do estado.
   - Após migração: helper recebe `intr` + Location
     do target.

#### API location-aware Introspector requerida

7. Confirmar trait methods location-aware:
   - `flat_counter_at(key, location)`.
   - `formatted_counter_at(key, location)`.
   - `figure_number_at_index(...)` per P185B.
   - `is_numbering_active_at(key, location)`.
   - **Cláusula gate substancial**: API location-aware
     suficiente para 4 arms? Especialmente arm
     Figure que lê `state.lang`.

#### Cláusula `lang`

8. **Cláusula gate substancial específica**: arm
   Figure de `compute_labelled` lê `state.lang`
   (per P190D defer).
   - **Opção α**: passar `lang` como parâmetro
     adicional ao helper.
   - **Opção β**: walk arm Labelled tem acesso a
     `state.lang` no scope; passa explicitamente.
   - **Opção γ**: sub-store dedicado em
     TagIntrospector para `lang` (over-engineering).
   - Sugestão: **Opção β** — incremental;
     `state.lang` ainda existe em CounterStateLegacy
     durante P191; eliminado quando walk arm tem
     outra fonte.

#### Snapshot Location no walk arm Labelled

9. **Cláusula gate substancial**: walk arm Labelled
   tem Location do target disponível?
   - Per pattern P195D (não-locatable):
     snapshot+find_map sobre tags emitidas
     anteriormente.
   - Após migração walk → walk com `intr`: snapshot
     pode ser `intr.tags_for_location()` ou similar.
   - Ou: walk arm Labelled emite Tag::Labelled
     primeiro com Location, depois processa target;
     Location do Labelled passa-se ao helper.

#### Tests dependentes

10. Identificar tests:
    - Tests `compute_labelled` directos.
    - Tests E2E walk arm Labelled.
    - Tests sentinela P191B preservadas.

#### L0 alvos

11. Identificar L0s:
    - `rules/introspect.md` (helper migration +
      walk arm Labelled).
    - Possivelmente novos tests sentinela em L0.

Output: tabela com item + estado verificado.

**Critério de saída**:
- `compute_labelled` localizado com 4 arms
  catalogados.
- API location-aware confirmada utilizável.
- Cláusula `lang` resolvida (Opção α/β/γ).
- Snapshot Location resolvido empíricamente.
- Tests dependentes listados.

### .B Migrar `compute_labelled`

Per `.A.4`-`.A.9`:

1. Mudar signature para genérica:
   ```
   fn compute_labelled<I: Introspector>(
       intr:     &I,
       location: Location,
       target:   &Content,
       lang:     Option<&str>,  // per .A.8 Opção β
   ) -> (Option<String>, Option<usize>)
   ```
   - Forma exacta depende de empírico.

2. Substituir reads em 4 arms:
   - **Equation arm**:
     - `state.flat.get("equation")` →
       `intr.flat_counter_at("equation", location)`.
     - `state.is_numbering_active("equation")` →
       `intr.is_numbering_active_at("numbering_active:equation",
       location)`.
   - **Figure arm**:
     - `state.figure_numbers.get(...)` →
       `intr.figure_number_at_index(...)` per P185B.
     - `state.lang` → `lang` parameter.
   - **Heading arm**:
     - Reads de state → queries Introspector
       location-aware.
   - **Outros arms**: análogo.

3. Adaptar caller (walk arm Labelled — pattern
   P195D não-locatable):
   - Passar `intr` + Location do target + `lang`
     ao helper.
   - Snapshot+find_map preservado (per pattern
     P195D).

4. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- `compute_labelled` migrado.
- 4 arms substituídos por queries Introspector.
- Caller adaptado.
- `cargo check --workspace` passa.

### .C Adaptar tests

1. Identificar tests afectados (per `.A.10`):
   - Tests `compute_labelled` directos — adaptar
     para nova signature.
   - Tests E2E walk arm Labelled — preservar
     (semântica observável inalterada).

2. **Adicionar 1 test sentinela** (se aplicável):
   - "compute_labelled lê via Introspector path
     location-aware" — paridade com legacy.

3. Tests workspace verdes (Δ esperado: marginal —
   alguns tests adaptados).

**Critério de saída**:
- Tests adaptados.

### .D Auditoria empírica final

Confirmar empíricamente estado pós-P191C:

1. Tests workspace passam.
2. Linter zero violations.
3. Mecanismo Opção A completo:
   - Walk fn aceita `&mut TagIntrospector`.
   - `from_tags::from_tags` eliminado.
   - 12 walk arms popularem `intr` directamente.
   - **2 helpers walk-readers migrados**:
     `compute_heading_auto_toc` (P191B) +
     `compute_labelled` (P191C).
   - Walk arm Equation gate migrado.
4. 2 helpers walk-internal preservados.
5. Pattern ADR-0069 stylesheet preservado.

Output: tabela com item + estado verificado.

### .E Relatório consolidado P191

Criar
`00_nucleo/materialization/typst-passo-191-relatorio-consolidado.md`
com 9 secções padrão (replica P181J / P184F /
P195-P200):

- **§1 Resumo executivo**: barreira arquitectural
  P190F resolvida via Opção A; mecanismo walk
  pipeline com Introspector accessible
  implementado e validado; 2 helpers migrados;
  walk arm Equation gate migrado; from_tags
  eliminado; ADR-0071 ACEITE; **pré-condição
  cumprida para retomar P190G**.

- **§2 Sub-passos materializados**:

  | Passo | Magnitude planeada | Magnitude real | Δ tests | L0s |
  |---|---|---|---|---|
  | P191A | S-M | S-M | 0 | 0 |
  | P191B | M+ | M+ | -2 | 0 |
  | P191C | S-M | S-M | marginal | 1 (introspect.md) |
  | **Total** | M+ a L | M+ | **~Δ marginal** | 1 L0 |

- **§3 Decisões arquitecturais**: 9 cláusulas
  P191A fechadas; 4 decisões empíricas P191B
  (Opção α, signature genérica, cláusulas gate);
  3 decisões empíricas P191C (Opção β para `lang`,
  Snapshot Location).

- **§4 Achados não-triviais**:
  - P191B `apply_state_funcs` preservada (Func
    eval em fixpoint) — eliminação from_tags não
    é total.
  - P191B Locations monotónicas garantem ordering
    Sets vs Equation gate.
  - P191B caso edge (Set após Func intermediário)
    não exercitado; documentado como limitação
    aceite.
  - P191B LOC líquido -699 (refactor simplifica
    globalmente).
  - P191C `lang` resolvido via parameter passing
    (Opção β) — incremental.

- **§5 Estado activo vs preservado**:
  - **Activado em P191**:
    - Walk fn aceita `&mut TagIntrospector`.
    - 12 walk arms popularem `intr` directamente.
    - 2 helpers walk-readers usam Introspector
      path location-aware.
    - Walk arm Equation gate via location-aware.
  - **Preservado**:
    - 2 helpers walk-internal (`compute_figure`,
      `compute_heading_for_toc`).
    - Pattern ADR-0069 stylesheet (5 variantes).
    - `apply_state_funcs` para Func eval em
      fixpoint.
    - `CounterStateLegacy` 10 fields (4 com walk
      readers; defer P190G/H/I).

- **§6 Estado final M9, M5, M6**:
  - M9: 11/11 (inalterado).
  - **M5: COMPLETO** (inalterado).
  - **M6: barreira resolvida** — pré-condição
    arquitectural cumprida.
  - `Content` enum: 13 variants (inalterado).
  - `ElementPayload`: 13 variants (inalterado).
  - `ElementKind`: 10 (inalterado).
  - Trait `Introspector`: 20 métodos (inalterado).
  - `TagIntrospector`: 9 sub-stores (inalterado).
  - `CounterStateLegacy`: 10 fields (inalterado em
    P191; defer P190G/H/I).
  - `LayouterRuntimeState`: 3 fields (inalterado).

- **§7 Estado final lacunas**:
  - Lacunas residuais (#1, #1b, #2): inalteradas.
  - Não impactam M6 retomar.

- **§8 Pendências cumulativas**:
  - **P190 série em pausa após P190F** — retomar
    P190G após P191C fechar.
  - 4 defers acumulados a resolver:
    - `lang` (P190D) — possivelmente resolvido em
      P190G se walk arm Heading deixar de ler.
    - `numbering_active` (P190E) — agora
      resolvível via `intr.is_numbering_active_at`.
    - `flat` (P190F).
    - `hierarchical` (P190F).
  - DEBT M6 documentação fecha por execução em
    P190G/H/I.
  - F1 fecha após P190I.
  - F3 parcialmente fecha após P190I.

- **§9 Próximos passos**:
  
  **Imediato — retomar P190 série**:
  - **P190G** — Categoria 6 (Labels & TOC).
    Magnitude M.
  - **P190H** — Categoria 7 (Figures). Magnitude
    M.
  - **P190I** — Walk arms purification + Layouter
    final + struct elim + L0 + ADR-0070 ACEITE.
    Magnitude M+.
  
  **Após M6 fechar**:
  - M7 (loop fixpoint).
  - M8 (memoização comemo).
  - Lacunas residuais (#1, #1b, #2) — passos
    dedicados.

**Critério de saída**:
- Relatório consolidado existe.
- 9 secções presentes.
- §8 dedicada a pendências P190 + 4 defers.

### .F ADR-0071 ACEITE

1. Editar
   `00_nucleo/adr/typst-adr-0071-walk-pipeline-redesign.md`:
   - Estado: PROPOSTO → **ACEITE**.
   - Validação empírica registada:
     - P191B: prova de conceito com 1 helper.
     - P191C: 2º helper migrado;
       `compute_labelled` mais complexo validado.
     - 25 recursive call sites mecânicos.
     - 12 ElementPayload variants populated via
       helper centralizado.
     - 5 cláusulas gate substanciais resolvidas.
     - Pattern ADR-0069 preservado.
     - LOC líquido -699 (P191B).
     - Tests verdes Δ marginal.

2. Cross-references actualizadas:
   - P191A (PROPOSTO).
   - P191B (validação parcial).
   - P191C (validação completa + ACEITE).

**Critério de saída**:
- ADR-0071 ACEITE.

### .G Lembrete formal P190 retomar

Actualizar
`00_nucleo/p190-pause-resume-tracker.md`:

1. Estado: P191 série fechada; **P190 série pronta
   para retomar**.

2. Próximo sub-passo: **P190G** — Categoria 6
   (Labels & TOC).

3. Snapshot pós-P191:
   - `CounterStateLegacy`: 10 fields.
   - `LayouterRuntimeState`: 3 fields.
   - Tests workspace: ~1.832 (verificar empírico).
   - 94 passos executados.
   - Defers acumulados: 4.

4. Trabalho restante M6:
   - **P190G** Labels & TOC (M).
   - **P190H** Figures (M).
   - **P190I** Walk arms purification + Layouter
     final + struct elim + ADR-0070 ACEITE (M+).
   - Cleanup 4 defers acumulados.

5. Após M6 fechar:
   - F1 fecha.
   - F3 parcialmente fecha.
   - Desbloqueia M7 + M8.

**Critério de saída**:
- Tracker actualizado.

### .H Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs
   P191B baseline (1.832): **marginal**.
3. `crystalline-lint .` zero violations.
4. `compute_labelled` migrado para Introspector
   path location-aware.
5. Caller walk arm Labelled adaptado.
6. Mecanismo Opção A completo (2 helpers
   migrados).
7. Pattern ADR-0069 stylesheet preservado (5
   variantes).
8. **ADR-0071 ACEITE** (transitada per `.F`).
9. **Tracker P190 retomar actualizado** per `.G`.
10. Relatório consolidado P191 escrito (9 secções).
11. `compute_figure`, `compute_heading_for_toc`
    **NÃO modificados**.
12. Trait `Introspector` **NÃO modificado**.
13. `TagIntrospector` fields **NÃO modificados**.
14. `CounterStateLegacy` **NÃO modificado** em
    P191C.
15. Layouter **NÃO modificado**.
16. Snapshot tests verdes.
17. Linter passa final.

### .I Encerramento

P191C é o passo final da série P191. Após `.H`
concluído, série está fechada.

Estado projectado pós-P191C:

- **P191 série**: A ✅ B ✅ C ✅ — fechada.
- **Mecanismo Opção A completo** — 2 helpers
  walk-readers migrados + walk arm Equation gate
  migrado.
- **ADR-0071 ACEITE** registada.
- **Pré-condição arquitectural cumprida** —
  P190G/H/I podem retomar com confiança elevada.
- **Tracker P190 actualizado** — sinaliza retomar.
- **Pattern ADR-0069 preservado** (5 variantes).
- **94 passos executados** (P191B=93 + P191C=94).
- **Tests workspace**: ~1.832 (verificar empírico).
- **CounterStateLegacy**: 10 fields (inalterado em
  P191; defer P190G/H/I).
- **LayouterRuntimeState**: 3 fields (inalterado).
- **Padrão diagnóstico-primeiro**: 29ª aplicação
  consecutiva.

**Próximo passo**: **P190G** — categoria 6 (Labels
& TOC). Retomar P190 série conforme tracker.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate
   substancial inesperado.
2. `compute_labelled` migrado (`.B`).
3. Tests adaptados (`.C`).
4. Auditoria empírica final (`.D`).
5. Relatório consolidado P191 (9 secções) escrito
   (`.E`).
6. ADR-0071 ACEITE (`.F`).
7. Tracker P190 retomar actualizado (`.G`).
8. Verificações `.H` passam (17/17).
9. Tests workspace verdes (Δ marginal).
10. Mecanismo Opção A completo.
11. Pattern ADR-0069 stylesheet preservado.
12. **Pré-condição arquitectural cumprida para
    retomar P190G**.

---

## O que pode sair errado

- **`compute_labelled` arm Figure não tem `lang`
  acessível mesmo com Opção β**: cláusula gate
  substancial — Opção α (parameter via walk arm) ou
  refactor mais profundo.
- **Snapshot+find_map P195D pattern incompatível
  com novo signature**: cláusula gate substancial
  — investigar.
- **API location-aware insuficiente para arm
  Figure (figure_number_at_index complexo)**:
  cláusula gate substancial.
- **Tests E2E walk arm Labelled regridem**:
  cláusula gate trivial.
- **Snapshot tests divergem**: improvável.
- **ADR-0071 transição requer mais validação**:
  improvável após 2 helpers + walk gate validados.
- **Tracker P190 retomar não captura todos defers**:
  cláusula gate trivial.
- **Linter divergência V13/V14**: cláusula gate
  trivial.

---

## Notas operacionais

- **Tamanho**: S-M. ~50 LOC produção (helper
  migration + caller adaptation) + ~20 LOC tests
  adaptados + ~20 LOC L0 + ~250 LOC relatório
  consolidado.
- **Sem dependências externas novas**.
- **ADR-0071 ACEITE**.
- **Mecanismo Opção A completo** — pré-condição
  arquitectural cumprida.
- **Cláusula gate trivial**: aplicável a forma
  exacta de signatures, recálculo de hashes,
  adaptação tests.
- **Cláusulas gate substancial possíveis**: 3
  declaradas (lang acessibilidade, snapshot+find_map
  compatibilidade, API gaps).
- **Próximo passo**: **P190G** — retomar P190 série.
  Categoria 6 (Labels & TOC). Magnitude M.
- **LEMBRETE FORMAL CRÍTICO**: após P191C fechar,
  P190G é o próximo passo. Tracker
  `p190-pause-resume-tracker.md` documenta
  trabalho restante.
- **Marco arquitectural**: P191 é primeiro ramo
  paralelo na série P190 cumprida com sucesso.
  Padrão "diagnóstico-primeiro → ADR-PROPOSTO →
  validação → ADR-ACEITE" replicado em ramo curto
  (3 sub-passos). Análoga a ADR-0068 P185A em
  estrutura mas executada em escala menor.
