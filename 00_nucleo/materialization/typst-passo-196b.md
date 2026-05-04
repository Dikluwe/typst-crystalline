# Passo P196B — Walk arm Heading auto-toc + helper

Primeiro passo de implementação P196 (após P196A
diagnóstico). Magnitude **M genuína** — segunda
aplicação concreta do pattern ADR-0069 (após P195D).

Modifica walk arm `Content::Heading` em
`01_core/src/rules/introspect.rs:347-379` para:

1. **Manter 4 mutações legacy** (write paralelo durante
   janela compat M5):
   - `state.step_hierarchical("heading", *level)`.
   - `state.auto_label_counter += 1`.
   - `state.resolved_labels.insert(auto_label, text)`.
   - `state.headings_for_toc.push((auto_label,
     frozen_body, level))` — **E2-residuo** (lacuna #3).

2. **Computar `(auto_label, resolved_text)`** via helper
   privado `compute_heading_auto_toc(state, level) ->
   (Option<Label>, Option<String>)`.

3. **Emitir Tag manualmente após recursão** (pattern
   ADR-0069):
   - `if let Some(loc) = emitted_loc` — reuso directo
     de Location alocada para Heading (per P196A §11.1
     — Heading locatable simplifica face a P195D).
   - `Tag::Start(loc, ElementInfo::new(
     ElementPayload::Labelled { label: auto_label,
     resolved_text, figure_number: None }))` +
     `Tag::End(loc, 0)`.

Helper `compute_heading_auto_toc` análogo a
`compute_labelled` (P195D). Replica lógica actual
sem mutação.

Após P196B:
- Walk arm Heading emite Tag auto-toc pós-recursão em
  produção.
- `from_tags` arm Labelled (P195C) processa Tag → popula
  `intr.resolved_labels[auto-toc-N]`.
- Consumer C4 (P194B) começa a receber `Some(text)`
  do Introspector para **auto-toc labels**.
- **Caminho Introspector universal** para resolved
  labels: explicit (P195D) + auto-toc (P196B) +
  figure-ref (P168).
- **E2 fecha 3 das 4 mutações estruturalmente**.
  Mutação 4 (`headings_for_toc.push`) **continua
  activa** como E2-residuo até passo dedicado abrir
  sub-store `headings_for_toc` (lacuna #3).
- Mutações 1+2 (`step_hierarchical`, `auto_label_counter`)
  continuam — write paralelo necessário (counter state
  write-only durante walk).

**Pré-condição**: P196A concluído. Tests workspace
1.838 verdes; zero violations. 7 cláusulas P196A
fechadas. Decisão Opção 1 (reuso `ElementPayload::Labelled`)
fixada.

**Restrições**:
- **Manter 4 mutações legacy** durante janela compat M5.
- **Não** activar `is_locatable` ou `extract_payload`
  arms para Heading auto-toc — pattern post-recursion
  per ADR-0069.
- **Não** adicionar variant nova a `ElementPayload` —
  reuso `Labelled` per P196A cláusula 1.
- **Não** modificar trait `Introspector` (P185B fechou).
- **Não** modificar `TagIntrospector` (P193B fechou).
- **Não** modificar consumer C4 (P194B fechou).
- **Não** modificar `from_tags` arm Labelled (P195C
  cobre).
- **Não** abrir sub-store `headings_for_toc` — passo
  dedicado paralelo.
- **Não** migrar walk arm Figure — P197.
- **Não** modificar arms walk Heading P170 (locatable
  emit) — `emitted_loc` continua funcional.
- API pública preservada.
- Output observable em produção **inalterado** —
  mutação legacy paralela fornece valores idênticos.

---

## Sub-passos

### .A Auditoria L0

1. Confirmar walk arm Heading actual em
   `01_core/src/rules/introspect.rs:347-379`:
   - Per P196A §2.1, site real é 347-379. Re-verificar
     empiricamente.
   - Localizar 4 mutações.
   - Identificar variável `emitted_loc` em scope.

2. Confirmar `emitted_loc: Option<Location>`:
   - Walk top alocou para Heading (locatable per P164).
   - Acessível via `emitted_loc` ou similar.
   - Tipo é `Option<Location>` ou `Option<&Location>`?
     Confirmar.

3. Confirmar lógica de geração `auto_label`:
   - `Label(format!("auto-toc-{}", state.auto_label_counter))`
     per P196A §2.7.
   - Re-verificar empiricamente.

4. Confirmar lógica de geração `resolved_text`:
   - `state.format_hierarchical("heading")` mapped para
     `format!("Secção {}", n)` per P196A §2.8.
   - `unwrap_or_default()` produz `""` quando
     `format_hierarchical` retorna `None`.
   - Re-verificar.

5. Confirmar mutação 4 (`headings_for_toc.push`):
   - Per P196A §2.6, push faz
     `(auto_label, frozen_body, level)`.
   - Re-verificar tipo de tuple.

6. Confirmar L0 `rules/introspect.md`:
   - Localizar entrada walk arm Heading existente
     (provavelmente da promoção locatable P164).
   - Identificar onde adicionar nota P196B + secção
     Excepções M5 (E2-residuo).

7. Confirmar tests existentes:
   - Sentinela E2 P189B.
   - Identificar quais devem manter-se inalterados.

8. Confirmar `from_tags` arm Labelled (P195C):
   - Já popula `intr.resolved_labels` quando
     `Some(text)`.
   - Acceita `auto-toc-N` como key directamente —
     sem necessidade de adaptar.

Output: tabela com item + estado + linhas exactas para
edits.

**Critério de saída**:
- Walk arm Heading localizado.
- `emitted_loc` confirmado em scope.
- 4 mutações legacy localizadas.
- Lógica `auto_label` + `resolved_text` confirmada.

### .B Criar helper privado `compute_heading_auto_toc`

1. Em `01_core/src/rules/introspect.rs`:
   - Adicionar função privada (sem `pub`):
     ```
     fn compute_heading_auto_toc(
         state: &CounterStateLegacy,
         level: u8,
     ) -> (Option<Label>, Option<String>) {
         if !state.is_numbering_active("heading") {
             return (None, None);
         }
         let auto_label = Label(
             format!("auto-toc-{}", state.auto_label_counter)
         );
         let resolved = state.format_hierarchical("heading")
             .map(|n| format!("Secção {}", n));
         (Some(auto_label), resolved)
     }
     ```
   - Forma exacta replica lógica actual do walk arm
     (per `.A.3` + `.A.4`).
   - Retorno `(None, None)` quando numbering inactivo
     (caso edge: produção sem auto-toc).

2. Visibilidade: privada (sem `pub`) — uso interno
   apenas.

3. Confirmar `@prompt-hash` actualiza após edit do L0
   em `.D`.

**Critério de saída**:
- Helper criado em `introspect.rs`.
- `cargo check --workspace` passa.
- Sem regressão (helper não invocado ainda).

### .C Modificar walk arm Heading

1. Em `01_core/src/rules/introspect.rs:347-379` (per
   `.A.1`):
   - **Antes da recursão** do body: nada muda
     (mutações 1+2 podem permanecer onde estão; ou
     reordenar conforme convenção).
   - **Após `walk(body, ...)`** (mudança nova):
     ```
     // Computar auto-toc payload sem mutação
     // (helper privado, pattern ADR-0069).
     let (auto_label_opt, resolved_text) =
         compute_heading_auto_toc(state, *level);

     // Mutação legacy preservada (write paralelo
     // durante janela compat M5; remover em M6
     // per ADR-0069 §7).
     if let Some(auto_label) = &auto_label_opt {
         state.auto_label_counter += 1; // pode ficar
                                         // antes/depois
                                         // conforme
                                         // semântica
                                         // legacy
         if let Some(text) = &resolved_text {
             state.resolved_labels.insert(
                 auto_label.clone(),
                 text.clone(),
             );
         }
         // E2-residuo: headings_for_toc continua
         // activa até passo dedicado abrir sub-store
         // (lacuna #3).
         state.headings_for_toc.push((
             auto_label.clone(),
             frozen_body.clone(),
             *level,
         ));
     }

     // Tag emit pós-recursão (pattern ADR-0069).
     // Reuso directo de Location alocada para Heading
     // (Heading é locatable; mais simples que P195D
     // não-locatable).
     if let (Some(loc), Some(auto_label)) =
         (emitted_loc, auto_label_opt)
     {
         let payload = ElementPayload::Labelled {
             label: auto_label,
             resolved_text,
             figure_number: None,
         };
         tags.push(Tag::Start(
             loc,
             ElementInfo::new(payload),
         ));
         tags.push(Tag::End(loc, 0));
     }
     ```
   - Mutação 1 (`step_hierarchical`) permanece intacta
     (write paralelo necessário per P196A §11.4).
   - Forma exacta da expressão fica para Claude Code
     conforme convenção do projecto.

2. **Comentário inline obrigatório E2-residuo** (per
   P196A §11.3 + cláusula 4):
   ```
   // E2-residuo: state.headings_for_toc.push continua
   // activa porque sub-store `intr.headings_for_toc`
   // não existe (lacuna #3). Fecha em passo dedicado
   // abrir sub-store. Vide P196 consolidado §"E2
   // residuo".
   ```

3. **Comentário inline pattern ADR-0069**:
   ```
   // P196B — walk arm Heading emite Tag auto-toc
   // pós-recursão (pattern ADR-0069 post-recursion-tag-emission).
   // Mutação legacy preservada como write paralelo
   // durante janela compat M5; removida em M6.
   ```

4. Confirmar `@prompt-hash` actualiza após edit do L0
   em `.D`.

**Critério de saída**:
- Walk arm Heading emite Tag auto-toc pós-recursão.
- 4 mutações legacy preservadas.
- Helper invocado.
- 2 comentários inline presentes (pattern + E2-residuo).
- `cargo check --workspace` passa.

### .D Actualizar L0 `rules/introspect.md`

1. Adicionar entrada para walk arm Heading P196B
   modificado:
   - Helper `compute_heading_auto_toc` introduzido.
   - Walk arm emite Tag auto-toc pós-recursão (pattern
     ADR-0069).
   - **3 das 4 mutações** estruturalmente migram.
   - Mutação `headings_for_toc.push` permanece como
     **E2-residuo**.
   - Cross-references: ADR-0069, P195D (precedente
     directo), P196A §11.1 (`emitted_loc` simplifica),
     P189B §5 E2.

2. **Secção "Excepções M5" actualizada**:
   - E2 → **E2-residuo** (1 mutação restante de 4).
   - Pré-requisito para fechar: sub-store
     `intr.headings_for_toc` (lacuna #3).
   - Cross-reference a passo dedicado.

3. Hash em branco aguarda recálculo manual em `.F`.

**Critério de saída**:
- L0 reflecte mudança ao walk arm.
- Secção Excepções M5 actualizada.
- Cross-references presentes.

### .E Tests E2E

5 tests obrigatórios per P196A §13:

#### Test 1 — `heading_auto_toc_walk_emite_tag_e_popula_introspector`

1. Documento com Heading numerado:
   ```
   Content::Sequence(vec![
       Content::SetHeadingNumbering { active: true },
       Content::Heading {
           level: 1,
           body: Box::new(Content::Text("Intro".into())),
       },
       Content::Ref(Label("auto-toc-1".into())),
   ])
   ```

2. Pipeline: walk + from_tags → TagIntrospector
   populated.

3. Asserções:
   - `intr.resolved_labels.get(&Label("auto-toc-1".into()))`
     retorna `Some("Secção 1")` (ou texto equivalente).
   - **Caminho Introspector activa para auto-toc**.

#### Test 2 — `heading_auto_toc_paridade_legacy_vs_introspector`

1. Mesmo documento.
2. Asserções de paridade:
   - `state.resolved_labels.get(&Label("auto-toc-1".into()))`
     == `intr.resolved_labels.get(&Label("auto-toc-1".into()))`.
   - Confirma write paralelo legacy + Introspector.
3. Layout completo:
   - Consumer C4 (P194B) recebe `Some(text)` do
     Introspector path para auto-toc label.
   - `or_else` fallback legacy não chamado mas
     continua funcional como backup.

#### Test 3 — `heading_auto_toc_numbering_inactivo_emite_string_vazia`

1. Documento com Heading **sem** SetHeadingNumbering:
   ```
   Content::Sequence(vec![
       Content::Heading {
           level: 1,
           body: Box::new(Content::Text("Title".into())),
       },
   ])
   ```

2. Asserções:
   - `intr.resolved_labels.get(&Label("auto-toc-1".into()))`
     retorna `None` (helper retorna `(None, None)`).
   - Tag não emitida (per condição `if let
     (Some(loc), Some(auto_label))`).
   - Walk continua sem panic.
   - Confirma caso edge — sem auto-toc quando
     numbering inactivo.

#### Test 4 — `walk_e2_residuo_headings_for_toc_via_legacy` (sentinela)

1. Documento com Heading numerado.
2. Pipeline: walk completo.
3. Asserções:
   - `state.headings_for_toc.len() == 1` (mutação
     legacy preservada).
   - Confirma E2-residuo activo até passo dedicado.

#### Test 5 — `consumer_c4_recebe_some_para_auto_toc_label`

1. Documento com Heading + Ref a `auto-toc-N`.
2. Layout completo com TagIntrospector real (não
   inject directo).
3. Asserções:
   - `plain_text` contém "Secção 1" (ou texto
     equivalente).
   - **Confirma activação Introspector path em
     produção real para auto-toc**.
   - Inversão observable parcial completa para resolved
     labels (auto-toc + explicit + figure-ref).

Tests co-localizados em submódulo `p196b_walk_heading`
em `tests.rs`.

**Critério de saída**:
- 5 tests passam.
- Tests existentes não regridem.

### .F Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P196A
   baseline (1.838): **+5**.
3. `crystalline-lint .` zero violations (após
   `--fix-hashes`).
4. Tests `p196b_walk_heading::*` passam isoladamente.
5. Tests existentes (incluindo sentinela E2 P189B)
   **não regridem** — mutação legacy preservada.
6. Walk arm Heading emite Tag auto-toc pós-recursão em
   produção.
7. Helper `compute_heading_auto_toc` invocado pelo
   walk arm.
8. **4 mutações legacy preservadas** —
   `step_hierarchical`, `auto_label_counter`,
   `resolved_labels.insert`, `headings_for_toc.push`
   continuam.
9. `from_tags` arm Labelled (P195C) processa Tags
   auto-toc em produção real → popula
   `intr.resolved_labels[auto-toc-N]`.
10. **Comentário inline E2-residuo** presente em walk
    arm.
11. **Comentário inline pattern ADR-0069** presente.
12. L0 secção "Excepções M5" actualizada (E2 →
    E2-residuo).
13. `is_locatable(Content::Heading)` continua `true`
    (P164 intocado).
14. Trait `Introspector` NÃO modificado.
15. `TagIntrospector` NÃO modificado.
16. Consumer C4 (P194B) NÃO modificado — apenas começa
    a receber `Some(text)` para auto-toc labels.
17. Snapshot tests verdes.
18. Linter passa final.

### .G Encerramento

Escrever
`00_nucleo/materialization/typst-passo-196b-relatorio.md`
com:

- Resumo: walk arm emite Tag auto-toc pós-recursão;
  helper introduzido; E2 fecha 3/4 mutações;
  E2-residuo declarado.
- Confirmação `.F` (18 verificações).
- Δ tests vs baseline P196A (esperado +5).
- Hashes finais L0 (`rules/introspect.md`).
- Decisões de execução notáveis (se houver).
- Estado actual:
  - P196 série: A ✅ B ✅ | C pendente.
  - **E2 fecha 3 das 4 mutações estruturalmente**.
  - **E2-residuo declarado** (`headings_for_toc.push`
    activa até lacuna #3 fechar).
  - **Caminho Introspector universal** para resolved
    labels (auto-toc + explicit + figure-ref).
  - 71 passos executados.
- Pendências cumulativas: 4 excepções activas (E1, E3,
  E5, E6) + 1 residuo (E2-residuo).
- Próximo passo: P196C — relatório consolidado P196 +
  actualização nota DEBT M5-residual.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate
   substancial.
2. Helper `compute_heading_auto_toc` criado (`.B`).
3. Walk arm Heading emite Tag auto-toc pós-recursão
   (`.C`).
4. 4 mutações legacy preservadas paralelamente.
5. 2 comentários inline presentes (pattern + E2-residuo).
6. L0 `rules/introspect.md` actualizado (`.D`).
7. 5 tests E2E passam (`.E`).
8. Verificações `.F` passam (18/18).
9. Tests existentes não regridem (paridade observable
   preservada).
10. Output observable em produção inalterado (Introspector
    activa para auto-toc mas legacy fornece valores
    idênticos via write paralelo).
11. Relatório `.G` escrito.

---

## O que pode sair errado

- **Site walk arm Heading mudou** entre P196A e P196B
  (improvável): cláusula gate trivial — ajustar.
- **`emitted_loc` não está em scope do arm**: cláusula
  gate substancial — adaptar acesso (provavelmente via
  parâmetro ou variável local).
- **Helper retorna ordem diferente** de `(label, text)`
  esperado pelo walk arm: cláusula gate trivial —
  ajustar destructuring.
- **`auto_label_counter` semântica ordering legacy
  divergente** (incrementar antes ou depois de gerar
  label?): cláusula gate trivial — preservar ordem
  empírica do legacy.
- **Test 3 (numbering inactivo) falha** porque legacy
  ainda popula `state.headings_for_toc`: investigar —
  pode ser que mutação 4 acontece independente de
  numbering active. Cláusula gate substancial.
- **Test 4 (sentinela E2-residuo) falha** porque
  mutação 4 não é executada como esperado: cláusula
  gate substancial — re-verificar `.C`.
- **Tests existentes regridem por mudança de ordem
  Tags**: indica que Tag auto-toc emitida em momento
  errado. Cláusula gate substancial.
- **`frozen_body` clone falha** (não tem `Clone`):
  improvável — auditar `Content` derives.
- **Snapshot tests divergem**: investigar (output
  preservado por construção).
- **Linter divergência V13/V14**: cláusula gate
  trivial.

---

## Notas operacionais

- **Tamanho**: M genuíno. ~80 LOC walk arm + ~40 LOC
  helper + ~150 LOC tests + edits L0.
- **Sem dependências externas novas**.
- **Sem ADR; sem DEBT formal**.
- **Padrão materializado**: ADR-0069 segunda aplicação
  concreta. Variante operacional: locatable target
  (Heading) → reuso directo de `emitted_loc`. P195D
  variante: não-locatable target (Labelled) →
  snapshot+find_map.
- **Cláusula gate trivial**: aplicável a forma exacta
  do helper, ordem de mutações, recálculo de hashes.
- **Cláusula gate substancial**: aplicável a:
  - `emitted_loc` não acessível (esperado em scope).
  - Mutação 4 falha sentinela.
  - Tests E2 P189B regridem.
  - Ordem Tags causa timing issues.
  - Helper diverge da semântica legacy.
- **Inversão observable parcial completa após P196B**:
  caminho Introspector universal para resolved labels
  (auto-toc + explicit + figure-ref).
- **E2 fecha 3/4 estruturalmente**: 1 mutação residual
  (`headings_for_toc.push`) fica activa até passo
  dedicado abrir sub-store.
- **Próximo passo P196C**: relatório consolidado P196
  (9 secções) + actualização DEBT M5-residual.
  Magnitude S puro.
- **Padrão pragmático auditor disponível** durante
  execução se necessário (3 padrões M4-residual).
