# Passo P200C — Encerramento série P200 + Marco M5 universal

Segundo e último passo de implementação P200 (após
P200A diagnóstico, P200B trabalho híbrido).
Magnitude **S puro** — passo de validação documental e
**registo formal do marco M5 universal completo**.

P200C **não modifica código produção**. Foca em:

1. **Auditoria empírica final** — confirmar P200B
   integra coerentemente com estado anterior.
2. **Relatório consolidado P200** com 9 secções padrão
   (replica P181J / P184F / P185 / P186 / P187 / P188 /
   P189 / P193 / P194 / P195 / P196 / P197 / P198 /
   P199).
3. **Actualizar nota DEBT M5-residual** — **0
   excepções + 0 residuos + 0 pré-requisitos**.
4. **Marco arquitectural — M5 universal completo**:
   - Primeira vez desde declaração em P189B.
   - 7 séries materializadas (P189B, P193, P194, P195,
     P196, P197, P198, P199, P200).
   - 6 excepções fechadas + 1 residuo fechado + 2
     pré-requisitos paralelos materializados.
   - Todos walk arms M5 fechados estruturalmente.
5. **Documentação de transição para M6**: DEBT M6
   formalizada (write paralelo M5 ainda activo;
   cleanup em P190A reescrita do zero).

Após P200C:
- Série P200 fechada (3 sub-passos A-C).
- E2-residuo + lacuna #3 fechadas.
- **M5 universal completo registado formalmente em
  L0 e em relatório consolidado**.
- DEBT M5-residual: **0 excepções + 0 residuos + 0
  pré-requisitos**.
- DEBT M6 formalizada (write paralelo legacy + 4
  helpers `compute_*` + Layouter assignments).
- Pattern ADR-0069: 5 variantes operacionais
  consolidadas; 7 aplicações concretas; 4 helpers
  privados.

**Pré-condição**: P200B concluído. Tests workspace
1.869 verdes; zero violations. Sub-store
`headings_for_toc` aberto; Tag emit pós-recursão
funcional; consumer outline migrado.

**Restrições**:
- **Não** modificar código produção — passo documental.
- **Não** modificar tests existentes.
- **Não** abrir DEBT formal — M6 documentação não
  exige DEBT formal (Cenário B continua para M5;
  M6 será passo dedicado).
- **Não** materializar P190A — aguarda decisão
  estratégica do utilizador após P200C.
- **Não** transitar ADRs.

---

## Sub-passos

### .A Auditoria empírica final

Confirmar empiricamente estado pós-P200B:

1. Tests workspace 1.869 verdes.
2. Linter zero violations.
3. Confirmar field `headings_for_toc` em
   `TagIntrospector` (9º sub-store).
4. Confirmar trait method `headings_for_toc()` em
   `Introspector` trait (20º método).
5. Confirmar variant `ElementPayload::HeadingForToc`
   (13ª variant).
6. Confirmar `ElementKind::HeadingForToc` **NÃO
   adicionada** (justificação documentada per P200B
   §7).
7. Confirmar helper `compute_heading_for_toc` (4º na
   família ADR-0069 stylesheet).
8. Confirmar walk arm Heading emite **3 Tags
   pós-recursão**:
   - Heading (walk top, pré-recursão).
   - Labelled auto-toc (P196B, pós-recursão).
   - HeadingForToc (P200B, pós-recursão).
   - 6 tags por Heading folha (3 Start + 3 End).
9. Confirmar mutação 4 legacy preservada
   (`state.headings_for_toc.push(...)` continua).
10. Confirmar `from_tags` arm `HeadingForToc`
    funcional.
11. Confirmar consumer `outline.rs:24` migrado
    (substitution-with-fallback).
12. Confirmar Layouter assignments
    (`mod.rs:1490, 1521`) **NÃO modificados** (write
    paralelo M5 obrigatório).
13. Confirmar `compute_heading_auto_toc` (P196B)
    **NÃO modificado** (sub-stores diferentes).
14. Confirmar 5 tests novos P200B + 4 tests P196B
    adaptados passam.
15. **Confirmar M5 universal completo
    empiricamente**:
    - E1 ✅ fechada (P199B).
    - E2 ✅ fechada (P196B 3/4 + P200B 1/4 = 4/4).
    - E3 ✅ fechada (P197B).
    - E4 ✅ fechada (P195D).
    - E5 ✅ fechada (P198B).
    - E6 ✅ fechada (P198C).
    - E2-residuo ✅ fechada (P200B).
    - Lacuna #3 ✅ fechada (P200B).
    - **0 excepções activas + 0 residuos + 0
      pré-requisitos**.

Output: tabela com item + estado verificado.

**Critério de saída**:
- 15 verificações empíricas passam.
- Tests 1.869 inalterados.
- Auditoria sem disparar gate substancial.
- M5 universal completo confirmado empiricamente.

### .B Escrever relatório consolidado P200

Criar
`00_nucleo/materialization/typst-passo-200-relatorio-consolidado.md`
com 9 secções padrão + secção 10 dedicada ao marco:

- **§1 Resumo executivo**: trabalho híbrido (3
  categorias combinadas); E2-residuo + lacuna #3
  fechadas; **marco M5 universal completo pela
  primeira vez desde P189B**; desbloqueia M6 (P190A
  reescrita do zero — magnitude L).

- **§2 Sub-passos materializados**:

  | Passo | Magnitude planeada | Magnitude real | Δ tests | L0s |
  |---|---|---|---|---|
  | P200A | S | S | 0 | 0 |
  | P200B | M+ | M+ | +5 | 1 (`introspect.md`) |
  | P200C | S | S | 0 | 0 |
  | **Total** | M+ agregado | M+ | **+5** | 1 L0 |

- **§3 Decisões arquitecturais**: 9 cláusulas P200A
  fechadas + 4 decisões de execução notáveis em P200B
  (helper sempre retorna Some, ElementKind NÃO
  criada, frozen_body clonado, apenas 4 tests P196B
  regridem).

- **§4 Achados não-triviais durante execução**:
  - P200A §3 — `state.headings_for_toc` type signature
    `usize` (não `u8`); auditor empírico corrigiu.
  - P200A §3 — Layouter assignments
    `mod.rs:1490, 1521` descobertos (write paralelo
    M5 dual).
  - P200B §7 — helper sempre retorna `Some`
    (mutação 4 legacy é incondicional, não gated por
    numbering active).
  - P200B §7 — `ElementKind::HeadingForToc` NÃO
    criada — Tag derivada não precisa de
    ElementKind. Precedente P196B (Labelled auto-toc
    também sem ElementKind).
  - P200B §7 — `frozen_body` clonado para reuso
    (custo O(1) via Arc).
  - P200B §7 — apenas 4 tests P196B regridem (não 5).
  - P200B — sem nova variante operacional ADR-0069
    (trabalho híbrido = combinação de 3 padrões
    testados).

- **§5 Estado activo vs preservado** (replica padrão
  P195/P196/P197/P198/P199):
  - **Activado em P200B**: caminho Introspector para
    `headings_for_toc`; consumer outline.rs:24 first
    branch activa; substitution-with-fallback fornece
    backup raramente disparado.
  - **Mutação legacy preservada** (write paralelo M5):
    - Walk arm Heading mutação 4
      (`state.headings_for_toc.push`).
    - Layouter assignments
      (`mod.rs:1490, 1521`).
  - **Cleanup orgânico em M6** (P190A reescrita do
    zero) quando Layouter migrar para Introspector
    path completo.

- **§6 Estado final M9 e M5**:
  - M9: 11/11 (inalterado).
  - **M5: COMPLETO** — todos walk arms fechados
    estruturalmente (Outline P189B + Bibliography
    P181H + Labelled P195D + Heading P196B+P200B +
    Figure P197B + SetHeadingNumbering P198B +
    CounterUpdate P198C + SetEquationNumbering
    P199B).
  - `Content` enum: 1 variant nova
    (`SetEquationNumbering` em P199B).
  - `ElementPayload`: 12 → **13 variants**
    (`HeadingForToc` em P200B).
  - `ElementKind`: 10 (inalterado em P200).
  - Trait `Introspector`: 19 → **20 métodos**.
  - `TagIntrospector` sub-stores: 8 → **9**.

- **§7 Estado final lacunas**:

  | # | Lacuna | Pré-P200 | Pós-P200 |
  |---|---|---|---|
  | #1 | Figure kind=None ↔ Introspector | activa | activa (ortogonal a M5) |
  | #1b | from_tags arm Figure sem gate `is_counted` | activa | activa (ortogonal) |
  | #2 | reservada | — | — |
  | #3 | `headings_for_toc` sub-store ausente | activa | **fechada (P200B)** |
  | #4 | reservada | — | — |
  | #5 | `formatted_counter` Introspector | resolvida (P170) | resolvida |

  Lacunas #1 + #1b ortogonais a M5; permanecem
  activas. Não bloqueiam M6.

- **§8 Pendências cumulativas + DEBT M5-residual + M6 documentação**:

  Cenário B continua para M5 (sem DEBT formal aberto;
  fechado naturalmente após P200B).

  Nota actualizada (vide `.C`).

  **DEBT M6 documentação** (não-formal, registo
  arquivístico):
  - Write paralelo M5 activo em todos walk arms
    fechados estruturalmente.
  - 4 helpers privados `compute_*` (P195D, P196B,
    P197B, P200B) leem state legacy.
  - Layouter assignments
    (`mod.rs:1490, 1521`) dependem de
    `state.headings_for_toc`.
  - `CounterStateLegacy` struct ainda existe;
    eliminação em M6.
  - P190A reescrita do zero — magnitude L
    cross-modular esperada.

- **§9 Próximos passos sugeridos**:

  **M5 universal fechado**. Opções:

  - **Pausa estratégica** — boa altura para reflectir
    antes de M6 (refactor maior cross-modular).
  - **Iniciar P190A reescrita do zero** — M6
    eliminação `CounterStateLegacy`. Magnitude L.
    Trabalho previsto:
    - Eliminar mutações legacy em walk arms.
    - Migrar/eliminar 4 helpers `compute_*`.
    - Migrar Layouter assignments.
    - Eliminar struct `CounterStateLegacy`.
    - Adaptar consumers que ainda dependem de legacy.
  - **Lacunas residuais** (#1, #1b) — passo dedicado
    paralelo se desejado; ortogonais a M6.

- **§10 Marco arquitectural — M5 universal completo**
  (secção dedicada):

  Histórico:
  - P189B (declaração): 6 excepções + bibliografia
    em desenvolvimento.
  - P193 (sub-store ResolvedLabelStore): pré-requisito
    P194/P195.
  - P194 (consumer C4 migrado): pré-requisito P195+.
  - P195 (E4 fechada — Labelled): primeira aplicação
    ADR-0069.
  - P196 (E2 → E2-residuo — Heading auto-toc): 2ª
    aplicação ADR-0069.
  - P197 (E3 fechada — Figure): 1ª aplicação cenário α.
  - P198 (E5 + E6 fechadas — SetHeadingNumbering +
    CounterUpdate): cenário β-promote 1ª aplicação;
    sequência §9 P189 cumprida.
  - P199 (E1 fechada — SetEquationNumbering): cenário
    α por construção 1ª aplicação; **0 excepções
    activas pela primeira vez**.
  - **P200 (E2-residuo + lacuna #3 fechadas): trabalho
    híbrido; M5 universal COMPLETO**.

  **9 séries materializadas** entre declaração P189B e
  fecho P200B.

  **5 variantes operacionais ADR-0069 consolidadas**
  como catálogo arquitectural completo:
  - P195D variante (não-locatable).
  - P196B variante (locatable + body).
  - Cenário α (P197B Figure, P198B SetHeadingNumbering).
  - Cenário α por construção (P199B SetEquationNumbering).
  - Cenário β-promote (P198C CounterUpdate).

  **7 aplicações concretas ADR-0069 stylesheet**:
  P195D + P196B + P197B + P198B + P198C + P199B +
  P200B.

  **4 helpers privados família ADR-0069**:
  `compute_labelled` + `compute_heading_auto_toc` +
  `compute_figure` + `compute_heading_for_toc`.

  **Ferramentas arquitecturais activadas**:
  - Sub-stores `TagIntrospector`: 8 → 9.
  - Variants `ElementPayload`: 12 → 13.
  - Métodos trait `Introspector`: 19 → 20.
  - Variants `Content`: + 1 (SetEquationNumbering).

  **Padrão diagnóstico-primeiro confirmado em escala**:
  - 22 aplicações consecutivas sem falhar magnitude
    planeada ±1 nível.
  - 0 cláusulas substanciais disparadas em séries
    P195-P200.
  - Reservas mantidas explícitas (Reserva 1 =
    SetEquationNumbering) endereçadas no momento
    apropriado (P199B após >12 séries).

  Após M5 universal completo: desbloqueia M6 (P190A
  reescrita do zero — eliminação `CounterStateLegacy`;
  magnitude L cross-modular).

**Critério de saída**:
- Relatório consolidado existe.
- 9 secções padrão + §10 dedicada ao marco.
- §5 dedicada a estado activo vs preservado.
- §10 documenta histórico M5 + marco.

### .C Actualizar nota DEBT M5-residual

P200 fecha **E2-residuo + lacuna #3 simultaneamente**.
**M5 universal completo**.

1. **Não editar** relatórios anteriores (preservação
   histórica).

2. Adicionar nota nova no relatório consolidado P200
   `.B`:

   > **Antes P200**: 0 excepções activas + 1 residuo
   > (E2-residuo); 1 pré-requisito paralelo restante
   > (lacuna #3).
   >
   > **Após P200B**: **0 excepções activas + 0
   > residuos + 0 pré-requisitos**.
   >
   > **MARCO ARQUITECTURAL — M5 universal completo
   > pela primeira vez desde declaração em P189B**:
   > - Todos walk arms fechados estruturalmente.
   > - 6 excepções fechadas + 1 residuo fechado + 2
   >   pré-requisitos paralelos materializados.
   > - Pattern ADR-0069 com 5 variantes operacionais
   >   consolidadas; 7 aplicações concretas; 4
   >   helpers privados.
   > - Trait `Introspector`: 19 → 20 métodos.
   > - `ElementPayload`: 12 → 13 variants.
   > - `TagIntrospector` sub-stores: 8 → 9.
   >
   > **DEBT M6 documentação** (não-formal — Cenário B
   > continua para M5; M6 será passo dedicado):
   > - Write paralelo M5 activo em todos walk arms
   >   fechados estruturalmente.
   > - 4 helpers privados `compute_*` leem state
   >   legacy durante walk.
   > - Layouter assignments (`mod.rs:1490, 1521`)
   >   dependem de `state.headings_for_toc`.
   > - `CounterStateLegacy` struct ainda existe.
   > - Cleanup orgânico em **M6 (P190A reescrita do
   >   zero)** — magnitude L cross-modular esperada.

**Critério de saída**:
- Nota actualizada no relatório consolidado P200.

### .D Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. **Δ vs P200B
   baseline (1.869): 0** (sem código produção
   tocado).
3. `crystalline-lint .` zero violations.
4. Relatório consolidado P200 existe com 9 secções +
   §10.
5. §5 do consolidado dedicada a "Estado activo vs
   preservado".
6. §10 do consolidado dedicada ao **marco M5
   universal completo**.
7. Nota DEBT M5-residual actualizada (0 + 0 + 0).
8. **DEBT M6 documentação** registada.
9. Sem L0 modificada (passo puramente documental).
10. Sem ADR modificada.
11. Snapshot tests verdes.
12. Linter passa final.

### .E Encerramento

P200C é o passo final da série P200. Após `.D`
concluído, série está fechada.

Estado projectado pós-P200C:

- **P200 série**: A ✅ B ✅ C ✅. Fechada.
- **E2-residuo + lacuna #3 fechadas** (P200B —
  trabalho híbrido).
- **M5 universal completo registado formalmente em
  L0 + relatório consolidado**.
- **Excepções activas**: **0**.
- **Residuos**: **0**.
- **Pré-requisitos restantes**: **0**.
- **`Content` enum**: + 1 variant
  (`SetEquationNumbering` P199B).
- **`ElementPayload`**: 13 variants
  (`HeadingForToc` P200B).
- **`ElementKind`**: 10 (inalterado em P200).
- **Trait `Introspector`**: 20 métodos
  (`headings_for_toc()` P200B).
- **`TagIntrospector`**: 9 sub-stores
  (`headings_for_toc` P200B).
- **Tests workspace**: 1.869 (inalterado em P200C —
  passo documental).
- **Padrão diagnóstico-primeiro**: 22ª aplicação
  consecutiva (P200A na lista).
- **Pattern ADR-0069**: 5 variantes operacionais
  consolidadas; 7 aplicações concretas; 4 helpers
  privados.
- **9 séries materializadas** entre declaração P189B
  e fecho M5 universal.
- **DEBT M6**: write paralelo M5 ainda activo;
  cleanup em P190A reescrita do zero; magnitude L
  cross-modular.

**Próximas decisões estratégicas para o utilizador**:

1. **Pausa estratégica** — boa altura para reflectir
   antes de M6.
2. **Iniciar P190A reescrita do zero** — M6
   eliminação `CounterStateLegacy`.
3. **Lacunas residuais** (#1, #1b) — passo dedicado
   paralelo se desejado; ortogonais a M6.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` confirmou estado pós-P200B empiricamente
   (15/15).
2. Relatório consolidado P200 (9 secções + §10
   marco) escrito (`.B`).
3. Nota DEBT M5-residual actualizada (`.C`).
4. **Marco M5 universal completo** registado
   formalmente.
5. DEBT M6 documentação registada.
6. Verificações `.D` passam (12/12).
7. Tests workspace 1.869 inalterados (passo
   documental).
8. Linter zero violations.
9. Sem código produção tocado.
10. Sem ADR modificada.

---

## O que pode sair errado

- **Auditoria `.A` revela divergência inesperada**
  (improvável — P200B fechou limpo): cláusula gate
  substancial.
- **Linter divergência** após edits L0: cláusula
  gate trivial.
- **Snapshot tests divergem** apesar de não tocar
  código: improvável.
- **Contagem cumulativa diverge** entre relatórios:
  aceitar correcção em P200C como ponto de verdade.

---

## Notas operacionais

- **Tamanho**: S puro. ~280 LOC relatório consolidado
  + nota DEBT (mais que passos consolidados anteriores
  porque inclui §10 marco).
- **Sem dependências externas novas**.
- **Sem código produção tocado**.
- **Sem ADR; sem DEBT formal aberto**.
- **DEBT M6 documentação** registada (informacional —
  não DEBT formal).
- **Padrão replicado**: encerramento série P186 / P187
  / P188 / P189 / P193 / P194 / P195 / P196 / P197 /
  P198 / P199 (relatório consolidado 9 secções
  padrão).
- **§10 secção dedicada** ao marco M5 universal —
  primeiro encerramento com secção dedicada (não
  replicado de séries anteriores; necessário porque
  M5 universal completo é evento singular).
- **Cláusula gate trivial**: aplicável a localização
  de ficheiros, formato L0, recálculo de hashes.
- **Sem cláusula gate substancial esperada**.
- **Estado intermédio seguro**: passo puramente
  documental.
- **Marco arquitectural significativo**:
  - M5 universal completo pela primeira vez desde
    P189B.
  - 9 séries materializadas (P189B, P193, P194,
    P195, P196, P197, P198, P199, P200).
  - 5 variantes operacionais ADR-0069 consolidadas
    como catálogo arquitectural completo.
  - Padrão diagnóstico-primeiro confirmado em
    escala (22 aplicações sem falhar).
- **Após P200C**: aguarda decisão estratégica do
  utilizador. P190A reescrita do zero é trabalho
  arquivístico maior e merece reflexão antes de
  iniciar.
