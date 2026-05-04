# Passo P197C — Encerramento série P197

Segundo e último passo de implementação P197 (após
P197A diagnóstico, P197B refactor walk arm Figure).
Magnitude **S puro** — passo de validação documental e
encerramento.

P197C **não modifica código produção**. Foca em:

1. **Auditoria empírica final** — confirmar P197B
   integra coerentemente com estado anterior.
2. **Relatório consolidado P197** com 9 secções padrão
   (replica P181J / P184F / P185 / P186 / P187 / P188 /
   P189 / P193 / P194 / P195 / P196).
3. **Actualizar nota DEBT M5-residual** (Cenário B
   continuação — E3 fecha estruturalmente).
4. **Corrigir contagem de passos** identificada em
   P197B §9 (75 cumulativos, não 73).

Após P197C:
- Série P197 fechada (3 sub-passos A-C).
- E3 fechada estruturalmente (declaração formal em L0
  feita em P197B).
- DEBT M5-residual: **3 excepções activas + 1
  residuo** (E1, E2-residuo, E5, E6); 2 pré-requisitos
  restantes inalterados.
- Pattern ADR-0069 com **3 variantes operacionais
  consolidadas** (P195D / P196B / P197B).

**Pré-condição**: P197B concluído. Tests workspace
1.848 verdes; zero violations. Walk arm Figure usa
helper `compute_figure`; mutação legacy preservada;
E3 declarada fechada estruturalmente em L0.

**Restrições**:
- **Não** modificar código produção — passo documental.
- **Não** modificar tests existentes.
- **Não** abrir DEBT formal — Cenário B continua.
- **Não** materializar passos futuros (P198, etc.).
- **Não** transitar ADRs — ADR-0069 já ACEITE.
- **Não** abrir sub-store `headings_for_toc` — passo
  dedicado paralelo.

---

## Sub-passos

### .A Auditoria empírica final

Confirmar empiricamente estado pós-P197B:

1. Tests workspace 1.848 verdes.
2. Linter zero violations.
3. Confirmar walk arm Figure
   (`introspect.rs:Content::Figure` — linha real per
   P197B §4):
   - Helper `compute_figure` invocado.
   - Mutação legacy preservada
     (`state.local_figure_counters.entry...`,
     `state.figure_numbers.entry...push`).
   - Comentário inline P197B presente.

4. Confirmar `from_tags` arm Figure (P184B) **NÃO
   modificado**.

5. Confirmar variant `ElementPayload::Figure` **NÃO
   modificado**.

6. Confirmar consumer C3 (P184D) inalterado em
   `references.rs:484` (ou linha real).

7. Confirmar `compute_labelled` P195D Figure arm **NÃO
   modificado** — continua a ler
   `state.figure_numbers.last()` durante walk.

8. Confirmar 5 tests sentinela cenário α passam:
   - `figure_walk_caminho_introspector_ja_activo`.
   - `figure_walk_helper_compute_figure_invocado`.
   - `figure_paridade_legacy_vs_introspector_inalterada`.
   - `figure_numbering_inactivo_helper_retorna_none`
     (ajustado per P197B §8).
   - `figure_compute_labelled_p195d_continua_funcional`.

9. Confirmar paridade observable em pipeline real:
   - Pipeline test com Figure numerada + Ref:
     - `state.figure_numbers["image"]` populated via
       walk (write paralelo).
     - `intr.figure_number_at_index("image", 0)`
       retorna `Some(1)` (caminho Introspector activo
       desde P184).
     - Consumer C3 recebe `Some(n)` do Introspector
       path.
   - Pipeline test com Figure dentro de Labelled:
     - `compute_labelled` P195D Figure arm produz
       valores correctos (lê legacy).
     - `intr.resolved_labels[label]` populated via
       Tag P195D.
     - `intr.figure_label_numbers[label]` populated.

10. Confirmar L0 `rules/introspect.md`:
    - Tabela "Excepções M5" actualizada — E3
      "fechada estruturalmente em P197B (cenário α)".
    - Lista "Ordem inversa à mutação" — passo 6
      marcado ✅ (P197B).
    - Secção "Walk arm Figure migrado (P197B, cenário
      α)" presente.
    - Hash `b9f78ff9` confirmado.

11. Confirmar mutação legacy preservada via grep:
    - `grep -n "figure_numbers\|local_figure_counters"
      01_core/src/rules/introspect.rs` retorna
      ocorrências esperadas no walk arm Figure +
      qualquer leitura em `compute_labelled` P195D.

12. Confirmar contagem cumulativa de passos:
    - Per P197B §9: cálculo identificou 75 passos
      cumulativos após P197B (não 73).
    - Re-verificar: P196A=71, P196B=72, P196C=73,
      P197A=74, P197B=75, P197C=76.
    - Corrigir contagem em §"Encerramento" deste
      passo + relatório consolidado.

Output: tabela com item + estado verificado.

**Critério de saída**:
- 12 verificações empíricas passam.
- Tests 1.848 inalterados.
- Auditoria sem disparar gate substancial.
- Contagem corrigida.

### .B Escrever relatório consolidado P197

Criar
`00_nucleo/materialization/typst-passo-197-relatorio-consolidado.md`
com 9 secções padrão (replica P181J / P184F / P185 /
P186 / P187 / P188 / P189 / P193 / P194 / P195 / P196):

- **§1 Resumo executivo**: refactor walk arm Figure
  via helper `compute_figure`; cenário α confirmado
  (caminho Introspector já activo desde P184); E3
  fechada estruturalmente; pattern ADR-0069 com 3
  variantes operacionais consolidadas.
- **§2 Sub-passos materializados**: tabela métricas
  A-C (magnitudes planeadas vs reais, Δ tests, L0s
  tocados).
- **§3 Decisões arquiteturais**: 7 cláusulas P197A
  fechadas (cenário α em todas) + decisão de helper
  retornar `Option<usize>`.
- **§4 Achados não-triviais durante execução**:
  - P197A §5 — cenário α confirmado empiricamente
    (variant existe, sub-store via CounterRegistry,
    consumer C3 P184D activo).
  - P197A §6 — cláusula gate substancial cadeia E2-E3
    resolvida sem disparar gate (mutação legacy
    preservada).
  - P197B §8 — 2 ajustes pós-execução (test 4
    `is_counted` divergência ortogonal; EcoString vs
    String).
  - P197B §9 — contagem cumulativa de passos corrigida
    para 75 (era reportada 73).
- **§5 Estado estilístico vs activo** (variante de §5
  P195/P194/P188 — adaptada porque P197 não introduz
  estado dormente novo):
  - **Activo desde P184**: caminho Introspector para
    figure numbering activo. Consumer C3 (P184D)
    recebe `Some(n)` via `figure_number_at_index`;
    fallback legacy raramente disparado.
  - **Activo desde P195D**: `intr.resolved_labels` +
    `intr.figure_label_numbers` populated para Figure
    labels via Tag pós-recursão (Labelled wrapper).
  - **Refactor estilístico em P197B**: helper
    `compute_figure` extraído para consistência com
    pattern ADR-0069. Sem mudança semântica.
  - **Mutação legacy preservada**: cleanup orgânico
    em M6 quando `compute_labelled` P195D Figure arm
    migrar para CounterRegistry.
- **§6 Estado final M9 e M5**:
  - M9: 11/11 (inalterado).
  - M5 progresso:
    - 1 arm migrado completamente (Outline P189B).
    - 2 arms migrados parcialmente estruturalmente
      (Labelled P195D; Heading P196B 3/4 mutações).
    - 1 arm declarado fechado estruturalmente (Figure
      P197B — cenário α; refactor estilístico).
    - **3 excepções activas + 1 residuo**: E1,
      E2-residuo, E5, E6.
  - `ElementPayload`: 11 variants (inalterado).
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
  - 3 excepções activas + 1 residuo.
  - 2 pré-requisitos restantes
    (`headings_for_toc`, `SetEquationNumbering`).
- **§9 Próximos passos sugeridos**:
  - **P198 — walks SetHeadingNumbering + CounterUpdate**
    (E5+E6 fecham): magnitude **a determinar
    empiricamente** em P198A. Pode ser cenário α
    (declaração formal) ou aplicação concreta de
    pattern ADR-0069 — auditor decide. Per P197A §11
    achado e P197B §12, **3 variantes operacionais
    consolidadas** disponíveis.
  - SetEquationNumbering (paralelo, fora série): E1
    fecha.
  - **Passo dedicado abrir sub-store
    `intr.headings_for_toc`**: fecha E2-residuo
    (lacuna #3).
  - Após sequência completa: M5 universal fecha;
    segue M6 (P200/P190 — eliminação
    `CounterStateLegacy`).

**Critério de saída**:
- Relatório consolidado existe.
- 9 secções presentes.
- §5 dedicada a estilístico vs activo.
- Contagem cumulativa corrigida.

### .C Actualizar nota DEBT M5-residual

P197 fecha **E3 estruturalmente completa** (vs E2 que
ficou com residuo).

1. **Não editar** relatórios anteriores (preservação
   histórica).

2. Adicionar nota nova no relatório consolidado P197
   `.B`:

   > **Antes P197**: 4 excepções activas + 1 residuo
   > (E1, E2-residuo, E3, E5, E6); 2 pré-requisitos
   > M5-residual restantes.
   >
   > **Após P197B**: **3 excepções activas + 1
   > residuo**:
   > - E1 — Reserva 1
   >   (`Content::SetEquationNumbering` ausente).
   > - **E2-residuo** — `headings_for_toc.push`
   >   (lacuna #3 bloqueia fechamento total).
   > - E5 — SetHeadingNumbering walk arm.
   > - E6 — CounterUpdate walk arm.
   >
   > **2 pré-requisitos restantes** (inalterado vs
   > P196):
   > - Sub-store `intr.headings_for_toc` (lacuna
   >   #3). **Fecha E2-residuo**.
   > - `Content::SetEquationNumbering`. **Fecha E1**.
   >
   > **E3 fechada estruturalmente** (cenário α —
   > caminho Introspector activo desde P184; refactor
   > estilístico em P197B). Diferente de E2 (que ficou
   > com residuo) e de E4 (que fechou via Tag
   > pós-recursão P195D).
   >
   > Mutação legacy preservada como write paralelo M5
   > (`compute_labelled` P195D Figure arm depende);
   > cleanup orgânico em M6.

**Critério de saída**:
- Nota actualizada no relatório consolidado P197.

### .D Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. **Δ vs P197B
   baseline (1.848): 0** (sem código produção
   tocado).
3. `crystalline-lint .` zero violations.
4. Relatório consolidado P197 existe com 9 secções.
5. §5 do consolidado dedicada a "Estado estilístico
   vs activo".
6. §9 do consolidado regista próximos passos
   (P198/SetEquationNumbering/passo dedicado
   `headings_for_toc`).
7. Nota DEBT M5-residual actualizada (3 excepções
   activas + 1 residuo; 2 pré-requisitos restantes).
8. Contagem cumulativa de passos corrigida (76 após
   P197C).
9. Sem L0 modificada (passo puramente documental).
10. Sem ADR modificada.
11. Snapshot tests verdes.
12. Linter passa final.

### .E Encerramento

P197C é o passo final da série P197. Após `.D`
concluído, série está fechada.

Estado projectado pós-P197C:

- **P197 série**: A ✅ B ✅ C ✅. Fechada.
- **E3**: fechada estruturalmente (declaração formal
  em L0 P197B; refactor estilístico — cenário α).
- **Excepções activas**: E1, E2-residuo, E5, E6 (3 + 1
  residuo; era 4 + 1 em P196C; E3 fechada).
- **DEBT M5-residual**: 2 pré-requisitos restantes
  (inalterado em P197 — E3 era excepção, não
  pré-requisito).
- **`ElementPayload`**: 11 variants (inalterado).
- **`ElementKind`**: 9 (inalterado).
- **Trait `Introspector`**: 19 métodos (inalterado).
- **`TagIntrospector`**: 8 sub-stores (inalterado).
- **Tests workspace**: 1.848 (inalterado em P197C —
  passo documental).
- **76 passos executados** (P197B = 75 + P197C = 76;
  contagem corrigida per P197B §9).
- **Padrão diagnóstico-primeiro**: 19ª aplicação
  consecutiva (P197A na lista).
- **Pattern ADR-0069**: **3 variantes operacionais
  consolidadas**:
  - P195D variante (não-locatable): snapshot+find_map.
  - P196B variante (locatable): `emitted_loc` directo.
  - P197B variante (cenário α): refactor estilístico
    sem Tag pós-recursão.
- **Próximo passo**: **P198A** — diagnóstico walks
  SetHeadingNumbering + CounterUpdate. Magnitude S
  esperada para diagnóstico; auditor decide
  empiricamente que variante operacional aplica a cada
  arm (3 disponíveis).

---

## Critério de conclusão

Todas em conjunto:

1. `.A` confirmou estado pós-P197B empiricamente
   (12/12).
2. Relatório consolidado P197 (9 secções) escrito
   (`.B`).
3. Nota DEBT M5-residual actualizada no consolidado
   (`.C`).
4. Contagem cumulativa corrigida (76).
5. Verificações `.D` passam (12/12).
6. Tests workspace 1.848 inalterados (passo
   documental).
7. Linter zero violations.
8. Sem código produção tocado.
9. Sem ADR modificada.

---

## O que pode sair errado

- **Auditoria `.A` revela divergência inesperada**
  (improvável — P197B fechou limpo): cláusula gate
  substancial.
- **Linter divergência** após edits L0 (relatório
  consolidado): cláusula gate trivial — `--fix-hashes`.
- **Snapshot tests divergem** apesar de não tocar
  código: improvável; investigar se acontecer.
- **Contagem cumulativa diverge** do esperado em
  outras referências (notas dispersas em relatórios
  anteriores): aceitar correcção em P197C como ponto
  de verdade; relatórios anteriores não são editados
  (preservação histórica).

---

## Notas operacionais

- **Tamanho**: S puro. ~250 LOC relatório consolidado +
  nota DEBT.
- **Sem dependências externas novas**.
- **Sem código produção tocado**.
- **Sem ADR; sem DEBT formal**.
- **Padrão replicado**:
  - Encerramento série P186 / P187 / P188 / P189 /
    P193 / P194 / P195 / P196 (relatório consolidado
    9 secções padrão).
- **Cláusula gate trivial**: aplicável a localização
  de ficheiros, formato L0, recálculo de hashes,
  contagem.
- **Sem cláusula gate substancial esperada**.
- **Estado intermédio seguro**: passo puramente
  documental.
- **3 variantes operacionais ADR-0069 consolidadas
  para P198 decidir**:
  - P195D (não-locatable): snapshot+find_map.
  - P196B (locatable): `emitted_loc` directo.
  - P197B (cenário α): declaração formal sem Tag
    pós-recursão.
- **Próximo passo P198A**: diagnóstico walks
  SetHeadingNumbering + CounterUpdate. Cada arm pode
  estar em variante diferente — auditor decide
  empiricamente. Magnitude esperada para diagnóstico:
  S; implementação P198B+ depende de qual variante
  aplica a cada arm.
