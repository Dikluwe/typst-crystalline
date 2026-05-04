# Passo P196C — Encerramento série P196

Segundo e último passo de implementação P196 (após
P196A diagnóstico, P196B walk arm Heading auto-toc).
Magnitude **S puro** — passo de validação documental e
encerramento.

P196C **não modifica código produção**. Foca em:

1. **Auditoria empírica final** — confirmar P196B
   integra coerentemente com estado anterior.
2. **Relatório consolidado P196** com 9 secções padrão
   (replica P181J / P184F / P185 / P186 / P187 / P188 /
   P189 / P193 / P194 / P195).
3. **Actualizar nota DEBT M5-residual** (Cenário B
   continuação — E2 fecha 3/4 mutações; E2-residuo
   declarado).

Após P196C:
- Série P196 fechada (3 sub-passos A-C).
- E2 fecha 3/4 mutações estruturalmente.
- E2-residuo declarado formalmente
  (`headings_for_toc.push`).
- Caminho Introspector universal para resolved labels
  registado (auto-toc + explicit + figure-ref).
- DEBT M5-residual: 5 excepções activas → 4 + 1
  residuo. 2 pré-requisitos restantes inalterados.
- Pattern ADR-0069 com 2 aplicações concretas
  documentadas.

**Pré-condição**: P196B concluído. Tests workspace
1.843 verdes; zero violations. Walk arm Heading emite
Tag auto-toc; mutação legacy preservada; E2-residuo
documentado em L0.

**Restrições**:
- **Não** modificar código produção — passo documental.
- **Não** modificar tests existentes.
- **Não** abrir DEBT formal — Cenário B continua.
- **Não** materializar passos futuros (P197, etc.).
- **Não** transitar ADRs — ADR-0069 já ACEITE.
- **Não** abrir sub-store `headings_for_toc` — passo
  dedicado paralelo.

---

## Sub-passos

### .A Auditoria empírica final

Confirmar empiricamente estado pós-P196B:

1. Tests workspace 1.843 verdes.
2. Linter zero violations.
3. Confirmar walk arm Heading
   (`introspect.rs:Content::Heading`):
   - Helper `compute_heading_auto_toc` invocado
     (linha ~441 per relatório P196B §9.6).
   - 4 mutações legacy preservadas (mutações 1-4 per
     P196B §2).
   - Tag pós-recursão emitida (linhas ~461-471 per
     P196B §9.7) com `Tag::Start` + `Tag::End(loc, 0)`.
   - Reuso `emitted_loc` directo (sem snapshot+find_map).

4. Confirmar `from_tags` arm Labelled (P195C):
   - Continua a popular `intr.resolved_labels` para
     chave `auto-toc-N` (variant `Labelled` cobre
     auto-toc semanticamente per P196A §11.2).

5. Confirmar consumer C4 (P194B) inalterado em
   `references.rs:53-67`.

6. Confirmar 2 comentários inline em walk arm:
   - Pattern ADR-0069 (auto-toc emit).
   - E2-residuo (`headings_for_toc.push`).

7. Confirmar L0 `rules/introspect.md`:
   - Tabela "Excepções M5" actualizada — E2 →
     **E2-residuo** com 1 mutação restante.
   - E4 marcada "Fechou estruturalmente em P195D".
   - Lista "Ordem inversa à mutação" — passos 1-4
     marcados ✅; passo 5 novo (sub-store
     `intr.headings_for_toc`).
   - Secção "Walk arm Heading migrado (P196B,
     ADR-0069)".

8. Confirmar 5 tests E2E novos passam:
   - `heading_auto_toc_walk_emite_tag_e_popula_introspector`.
   - `heading_auto_toc_paridade_legacy_vs_introspector`.
   - `heading_auto_toc_numbering_inactivo_emite_string_vazia`.
   - `walk_e2_residuo_headings_for_toc_via_legacy`.
   - `consumer_c4_recebe_some_para_auto_toc_label`.

9. Confirmar 5 tests existentes adaptados (per P196B
   §6):
   - `walk_emite_start_e_end_para_heading` (2 → 4 tags).
   - `walk_aninha_start_end_para_heading_contendo_figure`
     (4 → 6 tags).
   - `walk_emite_tags_em_paralelo_com_state` (6 → 10
     tags).
   - `bracketing_valido_em_sequencia_plana` (6 → 12
     tags).
   - `end_hash_distingue_conteudo` (`filter hash != 0`).

10. Confirmar paridade observable em pipeline real:
    - Pipeline test com Heading numerado + Ref
      `auto-toc-N`:
      - `state.resolved_labels.get(auto-toc-N)` retorna
        texto.
      - `intr.resolved_labels.get(auto-toc-N)` retorna
        texto idêntico (write paralelo).
      - Consumer C4 recebe `Some(text)` do Introspector
        path; fallback legacy não chamado mas continua
        funcional.
    - Pipeline test com Heading sem numbering:
      - Helper retorna `(label, "")` per decisão P196B
        §3 (paridade legacy: insert com texto vazio).
      - Tag emitida com `resolved_text: Some("")`.
      - Sub-store contém `auto-toc-N → ""`.

11. Confirmar 4 mutações legacy preservadas via grep:
    - `grep -n "step_hierarchical\|auto_label_counter\|resolved_labels.insert\|headings_for_toc.push" 01_core/src/rules/introspect.rs`
      retorna 4 ocorrências no walk arm Heading + 1
      ocorrência (`resolved_labels.insert`) no walk arm
      Labelled (P195D).

12. Confirmar tests sentinela E2 P189B passam (mutação
    4 preservada; sentinela E2-residuo dedicado em
    test 4 acima).

Output: tabela com item + estado verificado.

**Critério de saída**:
- 12 verificações empíricas passam.
- Tests 1.843 inalterados.
- Auditoria sem disparar gate substancial.

### .B Escrever relatório consolidado P196

Criar
`00_nucleo/materialization/typst-passo-196-relatorio-consolidado.md`
com 9 secções padrão (replica P181J / P184F / P185 /
P186 / P187 / P188 / P189 / P193 / P194 / P195):

- **§1 Resumo executivo**: walk arm Heading auto-toc
  migrado via pattern ADR-0069; 2ª aplicação concreta;
  E2 fecha 3/4 mutações estruturalmente; E2-residuo
  declarado; caminho Introspector universal para
  resolved labels.
- **§2 Sub-passos materializados**: tabela métricas A-C
  (magnitudes planeadas vs reais, Δ tests, L0s tocados).
- **§3 Decisões arquiteturais**: 7 cláusulas P196A
  fechadas + decisão de helper retornar `(Label,
  String)` concretos vs `Option` (per P196B §3).
- **§4 Achados não-triviais durante execução**:
  - P196A §11.1 — `emitted_loc` directo simplifica
    face a P195D snapshot+find_map.
  - P196A §11.2 — `ElementPayload::Labelled` cobre
    auto-toc semanticamente.
  - P196A §11.3 — lacuna #3 mantida; E2-residuo
    documentado.
  - P196A §11.4 — mutações 1+2 são write paralelo
    necessário (counter state).
  - P196A §11.5 — Tag auto-toc partilha Location com
    Tag Heading.
  - P196B §3 — helper retorna concrete `(Label,
    String)` em vez de `Option<…>` per paridade
    legacy (insert com texto vazio).
  - P196B §6 — 5 tests existentes adaptados (4 tags
    em vez de 2 por Heading — padrão pragmático
    auditor #1: ajustar fixture).
  - P196B §5 — sequência exacta de tags registada
    para Heading + Heading-com-Figure.
- **§5 Estado dormente vs activo** (secção dedicada
  paralela a §5 P195 / P194 / P188):
  - Activo (P196B activa em produção):
    - Auto-toc Heading → Introspector path (consumer
      C4 recebe `Some(text)` para `auto-toc-N`).
    - `intr.resolved_labels` populated **universalmente**
      (auto-toc + explicit + figure-ref).
    - Inversão observable parcial completa para
      resolved labels.
  - Continua legacy (E2-residuo activo):
    - `state.headings_for_toc.push((auto_label,
      frozen_body, level))` — sub-store ausente
      (lacuna #3).
    - Consumer outline.rs:24 lê directamente do
      legacy.
    - Fecha em passo dedicado abrir sub-store
      `intr.headings_for_toc`.
  - Mutação legacy preservada como write paralelo
    durante janela compat M5; cleanup orgânico em M6.
- **§6 Estado final M9 e M5**:
  - M9: 11/11 (inalterado).
  - M5 progresso:
    - 1 arm migrado completamente (Outline P189B).
    - 2 arms migrados parcialmente (Labelled P195D
      estruturalmente; Heading P196B 3/4 mutações
      estruturalmente).
    - **4 excepções activas + 1 residuo**: E1, E2-residuo,
      E3, E5, E6.
  - `ElementPayload`: 11 variants (inalterado vs
    P195B — Opção 1 reuso).
  - `ElementKind`: 9 (inalterado).
  - Trait `Introspector`: 19 métodos (inalterado).
  - `TagIntrospector` sub-stores: 8 (inalterado).
- **§7 Estado final lacunas**:
  - Lacuna #3 (`headings_for_toc`): activa ainda.
    Bloqueia E2-residuo. Passo dedicado paralelo.
  - Outras: inalteradas.
- **§8 Pendências cumulativas + DEBT M5-residual**:
  - Cenário B continua (sem DEBT formal aberto).
  - Nota actualizada (vide `.C`).
  - 4 excepções activas + 1 residuo.
  - 2 pré-requisitos restantes
    (`headings_for_toc`, `SetEquationNumbering`).
- **§9 Próximos passos sugeridos**:
  - **P197 — walk arm Figure** (E3 fecha): magnitude
    S–M (depende de auditoria empírica). Pattern
    ADR-0069 3ª aplicação. Figure é locatable —
    variante `emitted_loc` aplicável (similar a P196).
  - P198 — walks SetHeadingNumbering + CounterUpdate
    (E5+E6 fecham).
  - SetEquationNumbering (paralelo): E1 fecha.
  - **Passo dedicado abrir sub-store
    `intr.headings_for_toc`**: fecha E2-residuo
    (lacuna #3).
  - Após sequência completa: M5 universal fecha;
    segue M6 (P200 / P190 — eliminação
    `CounterStateLegacy`).

**Critério de saída**:
- Relatório consolidado existe.
- 9 secções presentes.
- §5 dedicada a dormente vs activo.

### .C Actualizar nota DEBT M5-residual

P196 fecha **3 das 4 mutações de E2 estruturalmente**.
Mutação 4 (`headings_for_toc.push`) declarada como
**E2-residuo** — fica activa até passo dedicado abrir
sub-store.

1. **Não editar** relatórios anteriores (preservação
   histórica).

2. Adicionar nota nova no relatório consolidado P196
   `.B`:

   > **Antes P196**: 5 excepções activas (E1, E2, E3,
   > E5, E6); 2 pré-requisitos M5-residual restantes.
   >
   > **Após P196B**: **4 excepções activas + 1
   > residuo**:
   > - E1 — Reserva 1
   >   (`Content::SetEquationNumbering` ausente).
   > - **E2-residuo** — `headings_for_toc.push` (lacuna
   >   #3 bloqueia fechamento total; sub-store
   >   ausente).
   > - E3 — Figure walk arm.
   > - E5 — SetHeadingNumbering walk arm.
   > - E6 — CounterUpdate walk arm.
   >
   > **2 pré-requisitos restantes** (inalterado vs
   > P195):
   > - Sub-store `intr.headings_for_toc` (lacuna #3).
   >   **Fecha E2-residuo**.
   > - `Content::SetEquationNumbering`. **Fecha E1**.
   >
   > **3 das 4 mutações de E2 estruturalmente
   > fechadas**. Caminho Introspector universal para
   > resolved labels (auto-toc + explicit +
   > figure-ref). Mutação legacy preservada como
   > fallback durante janela compat M5;
   > cleanup orgânico em M6.

**Critério de saída**:
- Nota actualizada no relatório consolidado P196.

### .D Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. **Δ vs P196B
   baseline (1.843): 0** (sem código produção tocado).
3. `crystalline-lint .` zero violations.
4. Relatório consolidado P196 existe com 9 secções.
5. §5 do consolidado dedicada a "Estado dormente vs
   activo".
6. §9 do consolidado regista próximos passos
   (P197/P198/SetEquationNumbering/passo dedicado
   `headings_for_toc`).
7. Nota DEBT M5-residual actualizada (4 excepções
   activas + 1 residuo; 2 pré-requisitos restantes).
8. Sem L0 modificada (passo puramente documental).
9. Sem ADR modificada.
10. Snapshot tests verdes.
11. Linter passa final.

### .E Encerramento

P196C é o passo final da série P196. Após `.D`
concluído, série está fechada.

Estado projectado pós-P196C:

- **P196 série**: A ✅ B ✅ C ✅. Fechada.
- **E2**: 3/4 mutações estruturalmente fechadas;
  **E2-residuo declarado** (`headings_for_toc.push`
  activa até lacuna #3 fechar).
- **Excepções activas**: E1, E2-residuo, E3, E5, E6 (4
  + 1 residuo; era 5 em P195E; E2 reduzida a residuo).
- **DEBT M5-residual**: 2 pré-requisitos restantes
  (inalterado em P196 — pré-requisitos destrancam
  excepções; E2-residuo precisa de pré-requisito
  `headings_for_toc` para fechar).
- **`ElementPayload`**: 11 variants (inalterado).
- **`ElementKind`**: 9 (inalterado).
- **Trait `Introspector`**: 19 métodos (inalterado).
- **`TagIntrospector`**: 8 sub-stores (inalterado).
- **Tests workspace**: 1.843 (inalterado em P196C —
  passo documental).
- **72 passos executados** (P196B = 71 + P196C = 72).
- **Padrão diagnóstico-primeiro**: 18ª aplicação
  consecutiva (P196A na lista).
- **Pattern ADR-0069**: 2 aplicações concretas
  documentadas (P195D + P196B).
- **Caminho Introspector universal para resolved
  labels**: auto-toc (P196B) + explicit (P195D) +
  figure-ref (P168). `or_else` fallback raramente
  disparado em produção.
- **Próximo passo**: **P197A** — diagnóstico walk arm
  Figure. Magnitude S esperada (3ª aplicação do pattern
  reduz incerteza). E3 fecha.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` confirmou estado pós-P196B empiricamente
   (12/12).
2. Relatório consolidado P196 (9 secções) escrito
   (`.B`).
3. Nota DEBT M5-residual actualizada no consolidado
   (`.C`).
4. Verificações `.D` passam (11/11).
5. Tests workspace 1.843 inalterados (passo
   documental).
6. Linter zero violations.
7. Sem código produção tocado.
8. Sem ADR modificada.

---

## O que pode sair errado

- **Auditoria `.A` revela divergência inesperada**
  (improvável — P196B fechou limpo): cláusula gate
  substancial. Investigar antes de escrever
  consolidado.
- **Linter divergência** após edits L0 (relatório
  consolidado): cláusula gate trivial — `--fix-hashes`.
- **Snapshot tests divergem** apesar de não tocar
  código: improvável; investigar se acontecer.

---

## Notas operacionais

- **Tamanho**: S puro. ~250 LOC relatório consolidado +
  nota DEBT.
- **Sem dependências externas novas**.
- **Sem código produção tocado**.
- **Sem ADR; sem DEBT formal**.
- **Padrão replicado**:
  - Encerramento série P186 / P187 / P188 / P189 /
    P193 / P194 / P195 (relatório consolidado 9
    secções padrão).
- **Cláusula gate trivial**: aplicável a localização
  de ficheiros, formato L0, recálculo de hashes.
- **Sem cláusula gate substancial esperada**.
- **Estado intermédio seguro**: passo puramente
  documental.
- **Pattern ADR-0069 com 2 aplicações documentadas**:
  P195D (não-locatable target → snapshot+find_map) +
  P196B (locatable target → reuso `emitted_loc`).
  Variantes operacionais consolidadas para P197/P198
  decidirem qual aplica empiricamente.
- **Próximo passo P197A**: diagnóstico walk arm Figure.
  Figure é locatable — variante `emitted_loc` provável.
  Pattern ADR-0069 3ª aplicação. Magnitude S esperada
  para diagnóstico; implementação P197B+ provável M
  agregado.
