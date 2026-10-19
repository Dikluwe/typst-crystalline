# Passo P198D — Encerramento série P198

Terceiro e último passo de implementação P198 (após
P198A diagnóstico, P198B declaração formal cenário α
para E5, P198C promote cenário β-promote para E6).
Magnitude **S puro** — passo de validação documental e
encerramento.

P198D **não modifica código produção**. Foca em:

1. **Auditoria empírica final** — confirmar P198B + P198C
   integram coerentemente.
2. **Relatório consolidado P198** com 9 secções padrão
   (replica P181J / P184F / P185 / P186 / P187 / P188 /
   P189 / P193 / P194 / P195 / P196 / P197).
3. **Actualizar nota DEBT M5-residual** (Cenário B
   continuação — E5 + E6 fecham estruturalmente).
4. **Marco arquitectural**: sequência §9 P189 cumprida
   na totalidade. M5 universal a 2 pré-requisitos
   paralelos do fecho.

Após P198D:
- Série P198 fechada (4 sub-passos A-D).
- E5 e E6 fechadas estruturalmente.
- DEBT M5-residual: 2 excepções + 1 residuo →
  **1 excepção + 1 residuo** (E1, E2-residuo); 2
  pré-requisitos restantes inalterados.
- **Sequência §9 P189 cumprida**: P193 → P194 → P195 →
  P196 → P197 → P198 todos materializados.
- **M5 universal a 2 pré-requisitos paralelos do fecho**:
  - E1 ↔ `Content::SetEquationNumbering` materialização.
  - E2-residuo ↔ sub-store `intr.headings_for_toc`
    (lacuna #3).
- Pattern ADR-0069 com **4 variantes operacionais**:
  P195D + P196B + cenário α + cenário β-promote.
- **5 aplicações concretas ADR-0069 stylesheet**: P195D,
  P196B, P197B, P198B, P198C.

**Pré-condição**: P198C concluído. Tests workspace 1.859
verdes; zero violations. Walk arm CounterUpdate
promovido a locatable; variant +
`ElementKind::CounterUpdate` adicionados; arms
`extract_payload` + `from_tags` funcionais.

**Restrições**:
- **Não** modificar código produção — passo documental.
- **Não** modificar tests existentes.
- **Não** abrir DEBT formal — Cenário B continua.
- **Não** materializar passos paralelos (`SetEquationNumbering`,
  passo dedicado `headings_for_toc`).
- **Não** transitar ADRs — ADR-0069 já ACEITE.
- **Não** materializar P190/P200 (M6) — aguarda M5
  universal fechar.

---

## Sub-passos

### .A Auditoria empírica final

Confirmar empiricamente estado pós-P198C:

1. Tests workspace 1.859 verdes.
2. Linter zero violations.
3. Confirmar P198B (cenário α — E5):
   - Walk arm SetHeadingNumbering com comentário
     inline P198B.
   - Mutação legacy `state.numbering_active.insert("heading", *active)`
     preservada.
   - `extract_payload` arm SetHeadingNumbering retorna
     `Some(StateUpdate)` (P182C inalterado).
   - `from_tags` arm StateUpdate (P171) funcional.

4. Confirmar P198C (cenário β-promote — E6):
   - Variant `ElementPayload::CounterUpdate` presente
     (12 variants total).
   - `ElementKind::CounterUpdate` presente (10
     variants total).
   - `is_locatable(Content::CounterUpdate) = true`.
   - `extract_payload(Content::CounterUpdate)` retorna
     `Some(ElementPayload::CounterUpdate { ... })`.
   - `from_tags` arm CounterUpdate funcional (3
     caminhos: Step+heading, Step+other, Update).
   - Walk arm CounterUpdate com comentário inline
     P198C.
   - 3 mutações legacy preservadas
     (`step_hierarchical`, `step_flat`, `update_flat`).

5. Confirmar 5 + 6 = 11 tests sentinela passam:
   - P198B: 5 tests `set_heading_numbering_*`.
   - P198C: 6 tests `counter_update_*`.

6. Confirmar paridade observable em pipeline real:
   - Pipeline test com SetHeadingNumbering + Heading +
     Ref(auto-toc-N):
     - `state.numbering_active.get("heading")` retorna
       `Some(&true)`.
     - `intr.is_numbering_active(...)` retorna idêntico.
     - Consumer C4 recebe `Some("Secção 1")` para
       auto-toc.
   - Pipeline test com CounterUpdate + Equation +
     Labelled:
     - `state.get_flat("equation")` retorna valor
       correcto.
     - `intr.flat_counter_at("equation", final_loc)`
       retorna idêntico.
     - `compute_labelled` Equation arm produz "Equação
       (N)" via legacy.

7. Confirmar L0 `rules/introspect.md`:
   - Tabela "Excepções M5" actualizada — E5 e E6
     "fechadas estruturalmente".
   - Lista "Ordem inversa à mutação" — passos 1-8
     marcados ✅.
   - Secções novas presentes: "Walk arm
     SetHeadingNumbering migrado (P198B, cenário α)"
     + "Walk arm CounterUpdate migrado (P198C,
     β-promote)".
   - Hash actualizado.

8. Confirmar L0s `entities/element_payload.md` +
   `entities/element_kind.md`:
   - Entradas para `CounterUpdate` adicionadas.
   - Hashes actualizados.

9. Confirmar mutações legacy preservadas via grep:
   - SetHeadingNumbering: `state.numbering_active.insert("heading"`.
   - CounterUpdate: `state.step_hierarchical\|step_flat\|update_flat`.

10. Confirmar tests sentinela P189B (E5, E6) passam
    sem regressão.

11. Confirmar contagem cumulativa:
    - P197C=76 (per P197 §11 corrigido).
    - P198A=77, P198B=77 (per P198B §8), P198C=78
      (per P198C §8), P198D=79.
    - **NOTA**: P198B §8 reportou "77 passos" mas se
      P198A=77, P198B deveria ser 78. Re-verificar
      empiricamente; corrigir se divergência.

12. **Marco arquitectural**: confirmar que sequência §9
    P189 está totalmente cumprida:
    - P193 ✅ sub-store ResolvedLabelStore.
    - P194 ✅ consumer C4 migrado.
    - P195 ✅ walk arm Labelled (E4 fechada).
    - P196 ✅ walk arm Heading auto-toc (E2 → E2-residuo).
    - P197 ✅ walk arm Figure (E3 fechada).
    - P198 ✅ walks SetHeadingNumbering + CounterUpdate
      (E5+E6 fechadas).
    - **Resta 2 pré-requisitos paralelos** (E1, E2-residuo).

Output: tabela com item + estado verificado.

**Critério de saída**:
- 12 verificações empíricas passam.
- Tests 1.859 inalterados.
- Auditoria sem disparar gate substancial.
- Contagem cumulativa coerente.

### .B Escrever relatório consolidado P198

Criar
`00_nucleo/materialization/typst-passo-198-relatorio-consolidado.md`
com 9 secções padrão (replica P181J / P184F / P185 /
P186 / P187 / P188 / P189 / P193 / P194 / P195 / P196 /
P197):

- **§1 Resumo executivo**: walks SetHeadingNumbering +
  CounterUpdate fecham E5+E6; 2 variantes operacionais
  diferentes (cenário α + cenário β-promote 1ª
  aplicação); pattern ADR-0069 com 4 variantes
  consolidadas + 5 aplicações concretas; **sequência §9
  P189 cumprida**; M5 universal a 2 pré-requisitos
  paralelos.

- **§2 Sub-passos materializados**: tabela métricas A-D
  (magnitudes planeadas vs reais, Δ tests, L0s tocados).

  Esperado:

  | Passo | Magnitude planeada | Magnitude real | Δ tests | L0s |
  |---|---|---|---|---|
  | P198A | S | S | 0 | 0 |
  | P198B | S | S | +5 | 1 (`introspect.md`) |
  | P198C | M | M | +6 | 3 (`introspect.md`, `element_payload.md`, `element_kind.md`) |
  | P198D | S | S | 0 | 0 |
  | **Total** | M- agregado | M- | **+11** | 3 L0s |

- **§3 Decisões arquiteturais**: 9 cláusulas P198A
  fechadas + decisão `ElementKind::CounterUpdate`
  adicionada (per P198C §7) + decisão sem helpers
  estilísticos (P198B + P198C).

- **§4 Achados não-triviais durante execução**:
  - P198A §5 — estados divergentes confirmados (E5
    cenário α; E6 cenário β-promote).
  - P198B §7 — sem helper extraído (mutação trivial).
  - P198C §3 — reuso enum `CounterUpdate` existente
    (P161 rename); namespacing Rust resolve colisão
    nominal.
  - P198C §4 — paridade exacta walk legacy ↔ from_tags
    (3 caminhos).
  - P198C §7 — `ElementKind::CounterUpdate` adicionada
    per convenção cristalino (todo locatable tem
    ElementKind).
  - P198C §7 — import `CounterUpdate` em `from_tags.rs`
    necessário (cláusula gate trivial).

- **§5 Estado activo vs preservado** (variante de §5
  P195/P196/P197 — adaptada porque P198 introduz nova
  variante):
  - **Activo desde P182C (E5)**: caminho Introspector
    para SetHeadingNumbering — StateRegistry populated
    via Tag::StateUpdate.
  - **Activado em P198C (E6)**: caminho Introspector
    para CounterUpdate — CounterRegistry populated via
    Tag::CounterUpdate (variante β-promote 1ª
    aplicação).
  - **Mutação legacy preservada** (write paralelo M5):
    - SetHeadingNumbering: 1 mutação preservada
      (`compute_heading_auto_toc` P196B + walk arm
      Equation lêem).
    - CounterUpdate: 3 mutações preservadas
      (`compute_*` helpers P195D/P196B/P197B lêem).
  - **Cleanup orgânico em M6** (P190/P200) quando
    `compute_*` helpers migrarem para sub-stores
    Introspector ou eliminarem-se com
    `CounterStateLegacy`.

- **§6 Estado final M9 e M5**:
  - M9: 11/11 (inalterado).
  - M5 progresso:
    - 1 arm migrado completamente (Outline P189B).
    - 2 arms migrados parcialmente estruturalmente
      (Labelled P195D; Heading P196B 3/4 mutações).
    - 1 arm declarado fechado estruturalmente (Figure
      P197B — cenário α).
    - 2 arms novos fechados estruturalmente em P198 (E5
      cenário α; E6 cenário β-promote).
    - **1 excepção activa + 1 residuo**: E1, E2-residuo.
  - `ElementPayload`: 11 → **12 variants** (CounterUpdate).
  - `ElementKind`: 9 → **10 variants** (CounterUpdate).
  - Trait `Introspector`: 19 métodos (inalterado).
  - `TagIntrospector` sub-stores: 8 (inalterado).

- **§7 Estado final lacunas**:
  - Lacuna #3 (`headings_for_toc`): activa.
  - Outras: inalteradas.

- **§8 Pendências cumulativas + DEBT M5-residual**:
  - Cenário B continua (sem DEBT formal aberto).
  - Nota actualizada (vide `.C`).
  - 1 excepção activa + 1 residuo.
  - 2 pré-requisitos paralelos restantes — fora série
    §9 P189.

- **§9 Próximos passos sugeridos**:

  **Pré-requisitos paralelos** (M5 universal fecha
  após ambos):
  - **`Content::SetEquationNumbering` materialização**
    (passo independente, fora série §9): fecha **E1**.
    Magnitude esperada: M (variant + arms).
  - **Sub-store `intr.headings_for_toc`** (passo
    independente, lacuna #3): fecha **E2-residuo**.
    Magnitude esperada: M (sub-store + arm + consumer
    outline).

  **Após M5 universal fechar**:
  - **P190 / P200 — M6 eliminação `CounterStateLegacy`**
    (passo agregado): cleanup do write paralelo M5;
    remoção do struct + dependências; consumer migrações
    finais (`compute_*` helpers eliminados ou migrados).
    Magnitude esperada: L (refactor maior).

  **Decisão estratégica para o utilizador**:
  - Ordem dos pré-requisitos paralelos (E1 vs
    E2-residuo): E1 pode ser executada
    independentemente; E2-residuo idem.
  - Ambos fecharem em qualquer ordem antes de P190/P200.

**Critério de saída**:
- Relatório consolidado existe.
- 9 secções presentes.
- §5 dedicada a estado activo vs preservado.

### .C Actualizar nota DEBT M5-residual

P198 fecha **E5 e E6 estruturalmente** (variantes
diferentes). Sequência §9 P189 cumprida.

1. **Não editar** relatórios anteriores (preservação
   histórica).

2. Adicionar nota nova no relatório consolidado P198
   `.B`:

   > **Antes P198**: 3 excepções activas + 1 residuo
   > (E1, E2-residuo, E5, E6); 2 pré-requisitos
   > M5-residual restantes.
   >
   > **Após P198C**: **1 excepção activa + 1 residuo**:
   > - E1 — Reserva 1
   >   (`Content::SetEquationNumbering` ausente).
   > - **E2-residuo** — `headings_for_toc.push`
   >   (lacuna #3 bloqueia fechamento total).
   >
   > **2 pré-requisitos restantes** (inalterado vs
   > P195/P196/P197):
   > - Sub-store `intr.headings_for_toc` (lacuna
   >   #3). **Fecha E2-residuo**.
   > - `Content::SetEquationNumbering`. **Fecha E1**.
   >
   > **E5 fechada estruturalmente** (P198B — cenário
   > α; caminho Introspector activo desde P182C).
   > **E6 fechada estruturalmente** (P198C — cenário
   > β-promote 1ª aplicação).
   >
   > **Sequência §9 P189 cumprida na totalidade**: P193 →
   > P194 → P195 → P196 → P197 → P198. M5 universal
   > a 2 pré-requisitos paralelos do fecho — ambos
   > fora série §9.
   >
   > Mutações legacy preservadas como write paralelo M5
   > em todas as excepções fechadas; cleanup orgânico
   > em M6 (P190/P200) quando `compute_*` helpers
   > migrarem ou `CounterStateLegacy` for eliminado.

**Critério de saída**:
- Nota actualizada no relatório consolidado P198.

### .D Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. **Δ vs P198C
   baseline (1.859): 0** (sem código produção
   tocado).
3. `crystalline-lint .` zero violations.
4. Relatório consolidado P198 existe com 9 secções.
5. §5 do consolidado dedicada a "Estado activo vs
   preservado".
6. §9 do consolidado regista pré-requisitos paralelos
   (`SetEquationNumbering`, `headings_for_toc`) +
   P190/P200.
7. Nota DEBT M5-residual actualizada (1 excepção
   activa + 1 residuo; 2 pré-requisitos paralelos
   restantes).
8. **Marco arquitectural** registado: sequência §9
   P189 cumprida.
9. Contagem cumulativa coerente (79 após P198D).
10. Sem L0 modificada (passo puramente documental).
11. Sem ADR modificada.
12. Snapshot tests verdes.
13. Linter passa final.

### .E Encerramento

P198D é o passo final da série P198. Após `.D`
concluído, série está fechada.

Estado projectado pós-P198D:

- **P198 série**: A ✅ B ✅ C ✅ D ✅. Fechada.
- **E5**: fechada estruturalmente (P198B — cenário α).
- **E6**: fechada estruturalmente (P198C — cenário
  β-promote 1ª aplicação).
- **Excepções activas**: E1, E2-residuo (1 + 1
  residuo; era 3 + 1 em P197C; E5+E6 fechadas).
- **DEBT M5-residual**: 2 pré-requisitos restantes
  (inalterado em P198 — pré-requisitos destrancam
  E1 + E2-residuo; ambos fora série §9).
- **`ElementPayload`**: 12 variants (era 11; +1
  CounterUpdate).
- **`ElementKind`**: 10 variants (era 9; +1
  CounterUpdate).
- **Trait `Introspector`**: 19 métodos (inalterado).
- **`TagIntrospector`**: 8 sub-stores (inalterado).
- **Tests workspace**: 1.859 (inalterado em P198D —
  passo documental).
- **79 passos executados** (P198C = 78 + P198D = 79).
- **Padrão diagnóstico-primeiro**: 20ª aplicação
  consecutiva (P198A na lista).
- **Pattern ADR-0069**: **4 variantes operacionais
  consolidadas**:
  - P195D variante (não-locatable): snapshot+find_map.
  - P196B variante (locatable + body): `emitted_loc`
    directo.
  - Cenário α (P197B Figure, P198B SetHeadingNumbering):
    refactor estilístico ou declaração formal.
  - Cenário β-promote (P198C CounterUpdate): promote
    completo.
- **5 aplicações ADR-0069 stylesheet**: P195D + P196B +
  P197B + P198B + P198C.
- **Sequência §9 P189 cumprida**: 6 passos
  materializados (P193 → P198).
- **M5 universal a 2 pré-requisitos paralelos do fecho**.

**Próximos passos** (ordem livre):

1. **`Content::SetEquationNumbering` materialização**
   — fecha E1.
2. **Sub-store `intr.headings_for_toc`** — fecha
   E2-residuo (lacuna #3).
3. (Após ambos) **P190/P200 — M6 eliminação
   `CounterStateLegacy`**.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` confirmou estado pós-P198C empiricamente
   (12/12).
2. Relatório consolidado P198 (9 secções) escrito
   (`.B`).
3. Nota DEBT M5-residual actualizada no consolidado
   (`.C`).
4. Marco arquitectural registado (sequência §9 P189
   cumprida).
5. Contagem cumulativa corrigida (79 após P198D).
6. Verificações `.D` passam (13/13).
7. Tests workspace 1.859 inalterados (passo
   documental).
8. Linter zero violations.
9. Sem código produção tocado.
10. Sem ADR modificada.

---

## O que pode sair errado

- **Auditoria `.A` revela divergência inesperada**
  (improvável — P198C fechou limpo): cláusula gate
  substancial.
- **Linter divergência** após edits L0 (relatório
  consolidado): cláusula gate trivial — `--fix-hashes`.
- **Snapshot tests divergem** apesar de não tocar
  código: improvável.
- **Contagem cumulativa diverge** entre relatórios
  (per P198B §8 reportou 77 mas P198A=77 implica
  P198B=78): aceitar correcção em P198D como ponto
  de verdade; relatórios anteriores não editados.

---

## Notas operacionais

- **Tamanho**: S puro. ~250 LOC relatório consolidado +
  nota DEBT.
- **Sem dependências externas novas**.
- **Sem código produção tocado**.
- **Sem ADR; sem DEBT formal**.
- **Padrão replicado**:
  - Encerramento série P186 / P187 / P188 / P189 /
    P193 / P194 / P195 / P196 / P197 (relatório
    consolidado 9 secções padrão).
- **Cláusula gate trivial**: aplicável a localização
  de ficheiros, formato L0, recálculo de hashes,
  contagem.
- **Sem cláusula gate substancial esperada**.
- **Estado intermédio seguro**: passo puramente
  documental.
- **Marco arquitectural**: sequência §9 P189 cumprida.
  M5 universal a 2 pré-requisitos paralelos do fecho.
- **4 variantes operacionais ADR-0069 consolidadas**
  para futura referência:
  - P195D (não-locatable): snapshot+find_map.
  - P196B (locatable + body): `emitted_loc` directo.
  - Cenário α (P197B, P198B): refactor estilístico /
    declaração formal.
  - Cenário β-promote (P198C): promote completo
    (variant + locatable + 2 arms).
- **Próximos passos** (ordem livre fora série §9):
  - `SetEquationNumbering` materialização (fecha E1).
  - Sub-store `headings_for_toc` (fecha E2-residuo).
  - P190/P200 (M6 eliminação `CounterStateLegacy`).
