# Passo P199C — Encerramento série P199

Segundo e último passo de implementação P199 (após
P199A diagnóstico, P199B materialização
`Content::SetEquationNumbering`). Magnitude **S puro**
— passo de validação documental e encerramento.

P199C **não modifica código produção**. Foca em:

1. **Auditoria empírica final** — confirmar P199B
   integra coerentemente com estado anterior.
2. **Relatório consolidado P199** com 9 secções padrão
   (replica P181J / P184F / P185 / P186 / P187 / P188 /
   P189 / P193 / P194 / P195 / P196 / P197 / P198).
3. **Actualizar nota DEBT M5-residual** — **0
   excepções activas + 1 residuo** (E2-residuo).
4. **Marco arquitectural**: pela primeira vez desde
   P189B há **0 excepções activas**. Reserva 1
   materializada após mais de 12 séries de espera. M5
   universal a **1 passo paralelo do fecho**.

Após P199C:
- Série P199 fechada (3 sub-passos A-C).
- E1 fechada estruturalmente.
- DEBT M5-residual: 1 excepção + 1 residuo → **0
  excepções + 1 residuo** (E2-residuo); 1
  pré-requisito restante (`headings_for_toc` —
  fecha E2-residuo).
- Pattern ADR-0069 com **5 variantes operacionais
  consolidadas** (auditor escalou cenário α por
  construção de sub-variante para variante autónoma
  em P199B §2).
- **6 aplicações concretas ADR-0069 stylesheet**:
  P195D + P196B + P197B + P198B + P198C + P199B.

**Pré-condição**: P199B concluído. Tests workspace
1.864 verdes; zero violations. Variant
`SetEquationNumbering` materializada; E1 declarada
fechada estruturalmente em L0.

**Restrições**:
- **Não** modificar código produção — passo documental.
- **Não** modificar tests existentes.
- **Não** abrir DEBT formal — Cenário B continua.
- **Não** materializar passos paralelos (sub-store
  `headings_for_toc`).
- **Não** transitar ADRs — ADR-0069 já ACEITE.
- **Não** materializar P190A — aguarda M5 universal
  fechar.

---

## Sub-passos

### .A Auditoria empírica final

Confirmar empiricamente estado pós-P199B:

1. Tests workspace 1.864 verdes.
2. Linter zero violations.
3. Confirmar variant `Content::SetEquationNumbering`
   em `entities/content.rs`:
   - Variant presente após `SetHeadingNumbering`.
   - Comentário DEBT-10 presente.
   - Cross-reference P199B + Reserva 1 P189B presente.

4. Confirmar 4 match arms exhaustivos cobertos
   (per P199B §8):
   - `content.rs:980` (`plain_text`).
   - `content.rs:1200` (`eq`).
   - `content.rs:1483, 1694` (2 listas de
     terminais).
   - `introspect.rs:101` (terminais em
     `materialize_time`).

5. Confirmar arm `is_locatable(SetEquationNumbering)
   = true` em `locatable.rs`.

6. Confirmar arm `extract_payload(SetEquationNumbering)`
   retorna `Some(StateUpdate { key:
   "numbering_active:equation", update:
   Set(Bool(active)) })`.

7. Confirmar walk arm em `introspect.rs`:
   - Mutação legacy `state.numbering_active.insert("equation",
     *active)` preservada.
   - Comentário inline P199B presente.

8. Confirmar `from_tags` arm StateUpdate (P171) **NÃO
   modificado** — genérica processa
   `numbering_active:equation`.

9. Confirmar Layouter:
   - Helper `layout_set_equation_numbering` em
     `rules/layout/counters.rs` (paralelo a
     `layout_set_heading_numbering`).
   - Consumer arm `Content::SetEquationNumbering` em
     `rules/layout/mod.rs`.
   - `equation.rs:32-33` first branch
     (substitution-with-fallback) **activa em produção
     real**.

10. Confirmar 5 tests sentinela passam (per P199B §6):
    - `set_equation_numbering_extract_payload_emite_state_update`.
    - `set_equation_numbering_from_tags_popula_state_registry`.
    - `set_equation_numbering_paridade_legacy_vs_introspector`.
    - `walk_arm_equation_le_numbering_active_legacy_apos_set`.
    - `consumer_layouter_equation_activa_via_introspector`.

11. Confirmar paridade observable em pipeline real:
    - Pipeline test com SetEquationNumbering +
      Equation + Labelled + Ref:
      - `state.numbering_active["equation"] == true`.
      - `intr.is_numbering_active(...)` retorna
        idêntico.
      - Counter `state.flat["equation"]` avança para
        1.
      - `compute_labelled` Equation arm produz
        `Some("Equação 1")` ou similar.
      - Layouter Equation first branch retorna
        `Some(...)` via Introspector path.

12. Confirmar L0 `rules/introspect.md`:
    - Tabela "Excepções M5" actualizada — E1
      "fechada estruturalmente em P199B (cenário α
      por construção)".
    - Lista "Ordem inversa à mutação" — passo 9
      marcado ✅ (P199B).
    - Secção "Variant SetEquationNumbering
      materializada (P199B, cenário α por construção)"
      presente.
    - Hash `603170c8` confirmado.

13. Confirmar L0 `entities/content.md`:
    - Entrada para variant `SetEquationNumbering`
      presente.

14. Confirmar contagem cumulativa de passos:
    - Per P198D: 80 passos.
    - P199A=81 (per P199A §11 indicou 21ª aplicação
      diagnóstico-primeiro consecutiva).
    - P199B=82 (per P199B §9 documentou estado
      actual sem contagem explícita; esta inferência
      baseia-se em incremento +1 por sub-passo).
    - P199C=83.
    - **NOTA**: confirmar empiricamente — pode haver
      divergência similar à corrigida em P198D `.A.11`.

15. **Marco arquitectural**: confirmar **0 excepções
    activas** após P199B (primeira vez desde P189B):
    - E1 ✅ fechada (P199B).
    - E2 → E2-residuo (P196B parcial).
    - E3 ✅ fechada (P197B).
    - E4 ✅ fechada (P195D).
    - E5 ✅ fechada (P198B).
    - E6 ✅ fechada (P198C).
    - **E2-residuo** activa — único restante.

Output: tabela com item + estado verificado.

**Critério de saída**:
- 15 verificações empíricas passam.
- Tests 1.864 inalterados.
- Auditoria sem disparar gate substancial.
- Contagem cumulativa coerente.

### .B Escrever relatório consolidado P199

Criar
`00_nucleo/materialization/typst-passo-199-relatorio-consolidado.md`
com 9 secções padrão (replica P181J / P184F / P185 /
P186 / P187 / P188 / P189 / P193 / P194 / P195 / P196 /
P197 / P198):

- **§1 Resumo executivo**: variant
  `Content::SetEquationNumbering` materializada após
  >12 séries de espera (Reserva 1 P189B); cenário α
  por construção (5ª variante operacional ADR-0069
  consolidada como autónoma); E1 fechada
  estruturalmente; **marco — 0 excepções activas pela
  primeira vez desde P189B**; M5 universal a 1 passo
  paralelo do fecho.

- **§2 Sub-passos materializados**: tabela métricas
  A-C (magnitudes planeadas vs reais, Δ tests, L0s
  tocados).

  Esperado:

  | Passo | Magnitude planeada | Magnitude real | Δ tests | L0s |
  |---|---|---|---|---|
  | P199A | S | S | 0 | 0 |
  | P199B | M | M | +5 | 2 (`introspect.md`, `entities/content.md`) |
  | P199C | S | S | 0 | 0 |
  | **Total** | M agregado | M | **+5** | 2 L0s |

- **§3 Decisões arquitecturais**: 7 cláusulas P199A
  fechadas + decisão de adicionar helper Layouter
  fora escopo P199A (per P199B §8) + decisão de
  escalar cenário α por construção a variante autónoma
  (per P199B §2).

- **§4 Achados não-triviais durante execução**:
  - P199A §3 — Layouter
    `equation.rs:32-33` substitution-with-fallback
    antes adormecida confirmado empiricamente.
  - P199B §8 — 7 match arms exhaustivos induzidos
    (mais do que P198C que teve 4); auditor cobriu
    via `cargo check` warnings.
  - P199B §8 — helper Layouter
    `layout_set_equation_numbering` adicionado fora
    escopo P199A. Trabalho trivial (1 linha) mas
    não previsto.
  - P199B §2 — cenário α por construção escalado de
    sub-variante (P199A §5) para variante autónoma
    (5ª variante operacional ADR-0069). Evolução do
    pensamento durante execução.
  - P199B §8 — DEBT-10 introduzida no comentário do
    variant (alinhamento com vanilla StyleChain
    futuro).

- **§5 Estado activo vs preservado** (replica P198 §5
  pattern):
  - **Activado em P199B (E1)**:
    - Caminho Introspector para SetEquationNumbering:
      StateRegistry populated via Tag::StateUpdate
      (chave `numbering_active:equation`).
    - Counter Equation activado em CounterRegistry
      via gate em `from_tags::Equation` (P186E)
      antes dormente.
    - Layouter `equation.rs:32-33` first branch
      activa em produção real.
    - `compute_labelled` Equation arm retorna
      `Some("Equação (n)")` para Equation labels via
      legacy.
  - **Mutação legacy preservada** (write paralelo M5):
    - SetEquationNumbering: 1 mutação preservada
      (`state.numbering_active.insert("equation")`).
      Necessária porque walk arm Equation +
      `compute_labelled` Equation arm lêem
      `state.is_numbering_active("equation")` /
      `state.get_flat("equation")` durante walk.
    - Cleanup orgânico em M6 (P190A reescrita do
      zero).

- **§6 Estado final M9 e M5**:
  - M9: 11/11 (inalterado).
  - M5 progresso:
    - 1 arm migrado completamente (Outline P189B).
    - Bibliography migrado (P181H).
    - Labelled migrado estruturalmente (P195D).
    - Heading auto-toc migrado parcialmente (E2 →
      E2-residuo P196B).
    - Figure declarada estruturalmente fechada
      (P197B — cenário α).
    - SetHeadingNumbering declarada estruturalmente
      fechada (P198B — cenário α).
    - CounterUpdate fechada estruturalmente (P198C —
      cenário β-promote).
    - **SetEquationNumbering materializada (P199B —
      cenário α por construção)**.
    - **0 excepções activas + 1 residuo**:
      E2-residuo.
  - `Content` enum: + 1 variant (SetEquationNumbering).
  - `ElementPayload`: 12 variants (inalterado em P199
    — reuso `StateUpdate`).
  - `ElementKind`: 10 (inalterado em P199).
  - Trait `Introspector`: 19 métodos (inalterado).
  - `TagIntrospector` sub-stores: 8 (inalterado).

- **§7 Estado final lacunas**:
  - Lacuna #3 (`headings_for_toc`): activa.
    Bloqueia E2-residuo. **Único pré-requisito
    restante**.
  - Outras: inalteradas.

- **§8 Pendências cumulativas + DEBT M5-residual**:
  - Cenário B continua (sem DEBT formal aberto).
  - Nota actualizada (vide `.C`).
  - **0 excepções activas + 1 residuo**.
  - **1 pré-requisito restante**.
  - DEBT-10 (futuro StyleChain) registada no
    comentário do variant; auditor pode formalizar
    em `m1-lacunas-captura.md` se desejar.

- **§9 Próximos passos sugeridos**:

  **Pré-requisito restante** (M5 universal fecha
  após):
  - **Sub-store `intr.headings_for_toc`** (passo
    paralelo, lacuna #3): fecha **E2-residuo**.
    Magnitude esperada: M (sub-store + arm +
    consumer outline).

  **Após M5 universal fechar**:
  - **P190A reescrita do zero — M6 eliminação
    `CounterStateLegacy`**: cleanup do write
    paralelo M5; remoção do struct; migração/eliminação
    final dos `compute_*` helpers que leem legacy.
    Magnitude esperada: **L** (refactor maior
    cross-modular). P190A original
    (`typst-passo-185a-relatorio.md` renomeado)
    declarado obsoleto — escrever do zero baseado no
    estado real.

**Critério de saída**:
- Relatório consolidado existe.
- 9 secções presentes.
- §5 dedicada a estado activo vs preservado.

### .C Actualizar nota DEBT M5-residual

P199 fecha **E1 estruturalmente completa**.

1. **Não editar** relatórios anteriores (preservação
   histórica).

2. Adicionar nota nova no relatório consolidado P199
   `.B`:

   > **Antes P199**: 1 excepção activa + 1 residuo
   > (E1, E2-residuo); 2 pré-requisitos M5-residual
   > restantes.
   >
   > **Após P199B**: **0 excepções activas + 1
   > residuo**:
   > - **E2-residuo** — `headings_for_toc.push`
   >   (lacuna #3 bloqueia fechamento total).
   >
   > **1 pré-requisito restante**:
   > - Sub-store `intr.headings_for_toc` (lacuna
   >   #3). **Fecha E2-residuo**.
   >
   > **E1 fechada estruturalmente** (P199B — cenário
   > α por construção 1ª aplicação). Variant
   > `Content::SetEquationNumbering` materializada
   > após >12 séries de espera (Reserva 1 desde
   > P189B). Caminho Introspector activado
   > imediatamente em produção real (Layouter
   > `equation.rs:32-33` first branch
   > substitution-with-fallback antes adormecida).
   >
   > **Marco arquitectural**: pela primeira vez
   > desde P189B há **0 excepções M5 activas**. M5
   > universal a 1 passo paralelo do fecho.
   >
   > Mutação legacy preservada como write paralelo M5
   > (`state.numbering_active["equation"]` +
   > `state.flat["equation"]` lidos por walk arm
   > Equation + `compute_labelled` Equation arm);
   > cleanup orgânico em M6 (P190A reescrita do
   > zero).

**Critério de saída**:
- Nota actualizada no relatório consolidado P199.

### .D Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. **Δ vs P199B
   baseline (1.864): 0** (sem código produção
   tocado).
3. `crystalline-lint .` zero violations.
4. Relatório consolidado P199 existe com 9 secções.
5. §5 do consolidado dedicada a "Estado activo vs
   preservado".
6. §9 do consolidado regista pré-requisito restante
   (`headings_for_toc`) + P190A.
7. Nota DEBT M5-residual actualizada (0 excepções
   activas + 1 residuo; 1 pré-requisito restante).
8. **Marco arquitectural** registado: 0 excepções
   activas pela primeira vez desde P189B.
9. Contagem cumulativa coerente.
10. Sem L0 modificada (passo puramente documental).
11. Sem ADR modificada.
12. Snapshot tests verdes.
13. Linter passa final.

### .E Encerramento

P199C é o passo final da série P199. Após `.D`
concluído, série está fechada.

Estado projectado pós-P199C:

- **P199 série**: A ✅ B ✅ C ✅. Fechada.
- **E1**: fechada estruturalmente (P199B — cenário α
  por construção 1ª aplicação).
- **Excepções activas**: **0 + 1 residuo**
  (E2-residuo). Era 1 + 1 em P198D. **Marco — 0
  excepções activas pela primeira vez desde P189B**.
- **DEBT M5-residual**: 1 pré-requisito restante
  (`headings_for_toc`).
- **`Content` enum**: + 1 variant
  (`SetEquationNumbering`).
- **`ElementPayload`**: 12 variants (inalterado em
  P199).
- **`ElementKind`**: 10 (inalterado).
- **Trait `Introspector`**: 19 métodos (inalterado).
- **`TagIntrospector`**: 8 sub-stores (inalterado).
- **Tests workspace**: 1.864 (inalterado em P199C —
  passo documental).
- **Padrão diagnóstico-primeiro**: 21ª aplicação
  consecutiva (P199A na lista).
- **Pattern ADR-0069**: **5 variantes operacionais
  consolidadas** (auditor escalou cenário α por
  construção a variante autónoma):
  - P195D variante (não-locatable): snapshot+find_map.
  - P196B variante (locatable + body): `emitted_loc`
    directo.
  - Cenário α (P197B, P198B): refactor estilístico /
    declaração formal.
  - **Cenário α por construção (P199B)**: materializar
    variant — caminho activa imediatamente.
  - Cenário β-promote (P198C): promote completo.
- **6 aplicações ADR-0069 stylesheet**: P195D + P196B
  + P197B + P198B + P198C + **P199B**.
- **Próximo passo**: **P200A** — diagnóstico
  sub-store `intr.headings_for_toc`. Fecha
  E2-residuo. Magnitude esperada para diagnóstico:
  S; implementação P200B+ provável M (sub-store novo
  + arm `from_tags` + variant Tag possivelmente +
  consumer outline).

---

## Critério de conclusão

Todas em conjunto:

1. `.A` confirmou estado pós-P199B empiricamente
   (15/15).
2. Relatório consolidado P199 (9 secções) escrito
   (`.B`).
3. Nota DEBT M5-residual actualizada no consolidado
   (`.C`).
4. Marco arquitectural registado (0 excepções
   activas; M5 universal a 1 passo do fecho).
5. Verificações `.D` passam (13/13).
6. Tests workspace 1.864 inalterados (passo
   documental).
7. Linter zero violations.
8. Sem código produção tocado.
9. Sem ADR modificada.

---

## O que pode sair errado

- **Auditoria `.A` revela divergência inesperada**
  (improvável — P199B fechou limpo): cláusula gate
  substancial.
- **Linter divergência** após edits L0 (relatório
  consolidado): cláusula gate trivial.
- **Snapshot tests divergem** apesar de não tocar
  código: improvável.
- **Contagem cumulativa diverge** entre relatórios:
  aceitar correcção em P199C como ponto de verdade;
  relatórios anteriores não editados.

---

## Notas operacionais

- **Tamanho**: S puro. ~250 LOC relatório consolidado
  + nota DEBT.
- **Sem dependências externas novas**.
- **Sem código produção tocado**.
- **Sem ADR; sem DEBT formal**.
- **Padrão replicado**:
  - Encerramento série P186 / P187 / P188 / P189 /
    P193 / P194 / P195 / P196 / P197 / P198
    (relatório consolidado 9 secções padrão).
- **Cláusula gate trivial**: aplicável a localização
  de ficheiros, formato L0, recálculo de hashes.
- **Sem cláusula gate substancial esperada**.
- **Estado intermédio seguro**: passo puramente
  documental.
- **Marco arquitectural**:
  - 0 excepções M5 activas pela primeira vez desde
    P189B.
  - Reserva 1 (E1) materializada após mais de 12
    séries.
  - M5 universal a 1 passo paralelo do fecho.
- **5 variantes operacionais ADR-0069 consolidadas**
  como catálogo arquitectural completo.
- **Próximo passo P200A**: diagnóstico sub-store
  `headings_for_toc`. Trabalho concreto previsto:
  - Adicionar sub-store `headings_for_toc:
    Vec<(Label, Content, u8)>` em `TagIntrospector`.
  - Possível variant Tag nova
    (`ElementPayload::HeadingForToc` ou reuso de
    Labelled/StateUpdate).
  - Walk arm Heading: emitir Tag dedicada
    pós-recursão (cláusula 4 mutação E2 — última
    mutação legacy do walk arm Heading).
  - `from_tags` arm popula sub-store novo.
  - Migrar consumer `outline.rs:24` para ler do
    Introspector.
  - Eliminar mutação 4 legacy → E2-residuo fecha
    completamente.
  - Magnitude esperada: M (similar a P198C
    β-promote em complexidade — sub-store novo +
    arm + variant possivelmente).
- **Após P200 fechar**: M5 universal completo.
  Desbloqueia M6 (P190A reescrita do zero —
  eliminação `CounterStateLegacy`; magnitude L).
