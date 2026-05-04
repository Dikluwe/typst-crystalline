# Passo P197B — Refactor walk arm Figure (cenário α)

Primeiro passo de implementação P197 (após P197A
diagnóstico). Magnitude **S/M** — refactor mínimo (não
migração arquitectural).

P197A confirmou empiricamente **cenário α**: caminho
Introspector para figure numbering já está activo em
produção desde P184 (variant `ElementPayload::Figure`,
`from_tags` arm, sub-store via `CounterRegistry`,
consumer C3 P184D). P197B é refactor estilístico para
consistência com pattern ADR-0069 (helper privado) +
declaração formal em L0 que **E3 fecha estruturalmente**.

P197B **não emite Tag pós-recursão** — diferente de
P195D/P196B. Razão: walk top já emite Tag pré-recursão
via locatable (`is_locatable(Content::Figure) = true`).
Pattern ADR-0069 dispensa-se per cenário α.

Trabalho concreto:
1. Extrair helper `compute_figure(state, kind,
   is_counted) -> Option<usize>` análogo a
   `compute_labelled` (P195D) e
   `compute_heading_auto_toc` (P196B).
2. Refactor walk arm Figure para chamar helper.
3. Mutação legacy preservada (write paralelo M5 —
   `compute_labelled` P195D Figure arm depende).
4. 5 tests sentinela cobrir cenário α.
5. L0 actualizado: tabela Excepções M5 (E3 → fechada
   estruturalmente); secção "Walk arm Figure migrado
   P197B".

Após P197B:
- E3 declarada formalmente fechada **estruturalmente**.
- Caminho Introspector inalterado (já activo desde
  P184).
- Helper `compute_figure` integrado consistentemente
  com pattern ADR-0069 (mesmo shape que outros 2
  helpers).
- Mutação legacy preservada — `compute_labelled` P195D
  Figure arm continua funcional.
- DEBT M5-residual: 4 excepções + 1 residuo → **3
  excepções + 1 residuo** (E1, E2-residuo, E5, E6); 2
  pré-requisitos restantes inalterados.

**Pré-condição**: P197A concluído. Tests workspace
1.843 verdes; zero violations. 7 cláusulas P197A
fechadas. Cenário α confirmado.

**Restrições**:
- **Não** emitir Tag pós-recursão — pattern ADR-0069
  dispensado per cenário α (Tag pré-recursão via
  locatable já cobre).
- **Não** modificar variant `ElementPayload::Figure`
  (cenário α — variant cobre).
- **Não** modificar `from_tags` arm Figure (cenário α —
  arm já popula sub-stores).
- **Não** modificar trait `Introspector` (P185B
  fechou).
- **Não** modificar `TagIntrospector` (P193B fechou).
- **Não** modificar consumer C3 (P184D fechou) ou
  consumer C4 (P194B fechou).
- **Não** modificar `compute_labelled` (P195D) — Figure
  arm continua a ler legacy.
- **Manter mutação legacy** —
  `state.figure_numbers.push` + `state.local_figure_counters
  += 1` necessárias durante janela compat M5.
- **Não** migrar walks SetHeadingNumbering +
  CounterUpdate — P198.
- API pública preservada.
- Output observable em produção **inalterado** —
  caminho Introspector já activo; helper extraction é
  refactor puro.

---

## Sub-passos

### .A Auditoria L0

1. Confirmar walk arm Figure actual em
   `01_core/src/rules/introspect.rs:490-519` (per P197A
   §3):
   - Re-verificar empiricamente.
   - Localizar gate `numbering.is_some() &&
     caption.is_some()`.
   - Localizar 2 mutações.

2. Confirmar `Content::Figure` variant:
   - Campos: `kind`, `numbering`, `caption`, `body`,
     possivelmente outros.

3. Confirmar variant `ElementPayload::Figure`:
   - Cobre semântica per P197A §5.
   - Sem mudança esperada.

4. Confirmar `is_locatable(Content::Figure) = true`:
   - Walk top emite `Tag::Start` antes de entrar no
     match arm.
   - `emitted_loc` em scope mas **não usado em P197B**
     (cenário α dispensa Tag pós-recursão).

5. Confirmar `from_tags` arm Figure (existente):
   - Popula 4 sub-stores per P197A §5
     (`CounterRegistry`, `figure_label_numbers`,
     `kind_index`, possivelmente outros).
   - **Não modificar**.

6. Confirmar consumer C3 (P184D):
   - `mod.rs:484` consulta
     `intr.figure_number_at_index(...)`.
   - Substitution-with-fallback activo desde P184D.
   - **Não modificar**.

7. Confirmar `compute_labelled` (P195D) Figure arm:
   - Lê `state.figure_numbers.last()` durante walk
     (per P197A §6).
   - Mutação legacy `state.figure_numbers.push`
     **necessária** para `compute_labelled` continuar
     funcional.
   - **Não modificar**.

8. Confirmar L0 `rules/introspect.md`:
   - Tabela "Excepções M5" — actualizar entrada E3.
   - Secção nova "Walk arm Figure migrado (P197B,
     cenário α)" — análoga a "Walk arm Heading migrado
     (P196B, ADR-0069)".

9. Confirmar tests existentes:
   - Sentinela E3 P189B (per `.D` ponto 3 do P189B).
   - Identificar quais devem manter-se inalterados.

Output: tabela com item + estado + linhas exactas.

**Critério de saída**:
- Walk arm Figure localizado.
- Cenário α confirmado.
- Mutação legacy preservada estabelecida obrigatória.

### .B Criar helper privado `compute_figure`

1. Em `01_core/src/rules/introspect.rs`:
   - Adicionar função privada (sem `pub`) análoga a
     `compute_labelled` e `compute_heading_auto_toc`:
     ```
     fn compute_figure(
         state:      &CounterStateLegacy,
         kind:       &Option<String>,
         is_counted: bool,
     ) -> Option<usize> {
         if !is_counted {
             return None;
         }
         let kind_key = kind.clone().unwrap_or_else(
             || "figure".to_string(),
         );
         let counter = state.local_figure_counters
             .get(&kind_key)
             .copied()
             .unwrap_or(0);
         Some(counter + 1)
     }
     ```
   - Forma exacta replica lógica actual do walk arm
     (per `.A.1`).
   - Visibilidade: privada (sem `pub`).
   - Caso edge: `is_counted = false` retorna `None`.

2. Confirmar `@prompt-hash` actualiza após edit do L0
   em `.D`.

**Critério de saída**:
- Helper criado em `introspect.rs`.
- `cargo check --workspace` passa.
- Sem regressão (helper não invocado ainda).

### .C Refactor walk arm Figure para usar helper

1. Em `01_core/src/rules/introspect.rs:490-519` (per
   `.A.1`):
   - Antes do gate `numbering.is_some() &&
     caption.is_some()`: nada muda.
   - Dentro do gate, **substituir** lógica actual por
     chamada ao helper:
     ```
     // Helper extraído (P197B, consistência com
     // pattern ADR-0069 — sem Tag pós-recursão
     // porque caminho Introspector já activo desde
     // P184).
     let is_counted = numbering.is_some() &&
                       caption.is_some();
     if let Some(figure_number) =
         compute_figure(state, kind, is_counted)
     {
         // Mutação legacy preservada (write paralelo
         // M5 — compute_labelled P195D Figure arm
         // depende; cleanup em M6).
         let kind_key = kind.clone().unwrap_or_else(
             || "figure".to_string(),
         );
         *state.local_figure_counters
             .entry(kind_key.clone())
             .or_insert(0) += 1;
         state.figure_numbers
             .entry(kind_key)
             .or_default()
             .push(figure_number);
     }
     ```
   - Forma exacta fica para Claude Code conforme
     convenção do projecto.

2. **Comentário inline obrigatório**:
   ```
   // P197B — walk arm Figure refactor (cenário α).
   // Caminho Introspector já activo desde P184
   // (variant ElementPayload::Figure + from_tags arm
   // + sub-store CounterRegistry + consumer C3 P184D).
   // Mutação legacy preservada como write paralelo M5
   // porque compute_labelled P195D Figure arm
   // depende. Cleanup orgânico em M6.
   ```

3. Confirmar `@prompt-hash` actualiza após edit do L0.

**Critério de saída**:
- Walk arm Figure usa helper.
- Mutação legacy preservada.
- Comentário inline presente.
- `cargo check --workspace` passa.

### .D Actualizar L0 `rules/introspect.md`

1. Tabela "Excepções M5" — actualizar entrada E3:
   - Estado: **fechada estruturalmente** (P197B).
   - Mutação legacy preservada como write paralelo M5.
   - Cleanup em M6.

2. Adicionar secção "Walk arm Figure migrado (P197B,
   cenário α)":
   - Caminho Introspector já activo desde P184.
   - Helper `compute_figure` extraído (consistência
     com pattern ADR-0069 stylesheet).
   - Mutação legacy preservada para
     `compute_labelled` P195D Figure arm.
   - Cross-references: P184B (variant + from_tags
     arm), P184D (consumer C3 substitution-with-fallback),
     P195D (`compute_labelled` Figure arm), ADR-0069
     (pattern stylesheet).

3. Actualizar lista "Ordem inversa à mutação":
   - Passos 1-5 marcados ✅ (P193B, P194B, P195D,
     P196B, **P197B**).
   - Passo 6 novo (P198 — SetHeadingNumbering +
     CounterUpdate).

4. Hash em branco aguarda recálculo manual em `.F`.

**Critério de saída**:
- L0 reflecte refactor.
- Tabela Excepções M5 actualizada.
- Secção nova presente.

### .E Tests sentinela cenário α

5 tests obrigatórios per P197A §12.

#### Test 1 — `figure_walk_caminho_introspector_ja_activo`

1. Documento com Figure numerada:
   ```
   Content::Figure {
       kind: Some("figure".into()),
       numbering: Some(...),
       caption: Some(Box::new(Content::Text("Cap".into()))),
       body: Box::new(Content::Text("body".into())),
       label: None,
   }
   ```

2. Pipeline: walk + from_tags → TagIntrospector
   populated.

3. Asserções:
   - `intr.figure_number_at_index("figure", 0)` retorna
     `Some(1)` (consumer C3 P184D path activo).
   - `intr.kind_index[&ElementKind::Figure].len() ==
     1`.
   - **Confirma caminho Introspector já activo desde
     P184** (não dependente de P197B).

#### Test 2 — `figure_walk_helper_compute_figure_invocado`

1. Mesmo documento.
2. Asserção (via instrumentação ou black-box):
   - `state.figure_numbers["figure"]` contém valor
     correcto.
   - Helper extraído produz mesmo resultado que walk
     legacy pré-P197B.
   - **Confirma refactor sem regressão**.

#### Test 3 — `figure_paridade_legacy_vs_introspector_inalterada`

1. Pipeline com Figure + Ref(label).
2. Asserções:
   - `intr.figure_number_at_index(...)` consistente
     com `state.figure_numbers.last(...)`.
   - Consumer C3 (P184D) recebe `Some(n)` do
     Introspector path.
   - `or_else` fallback legacy não chamado mas
     funcional como backup.

#### Test 4 — `figure_numbering_inactivo_helper_retorna_none`

1. Documento com Figure **sem** caption ou numbering:
   ```
   Content::Figure {
       kind: Some("figure".into()),
       numbering: None, // ou caption: None
       ...
   }
   ```

2. Asserções:
   - Helper `compute_figure` retorna `None`.
   - `state.figure_numbers["figure"]` vazio (sem
     push).
   - `intr.figure_number_at_index(...)` retorna
     `None`.
   - Confirma caso edge — `is_counted = false`.

#### Test 5 — `figure_compute_labelled_p195d_continua_funcional`

1. Documento com Figure dentro de Labelled wrapper:
   ```
   Content::Labelled {
       label: Label("fig1".into()),
       target: Box::new(Content::Figure { ... }),
   }
   ```

2. Asserções:
   - `compute_labelled` P195D Figure arm produz
     `(Some("Figura 1"), Some(1))` (lê
     `state.figure_numbers.last()`).
   - `intr.resolved_labels.get(&Label("fig1".into()))`
     retorna `Some("Figura 1")` (P195D Tag).
   - `intr.figure_label_numbers.get(&Label("fig1".into()))`
     retorna `Some(1)`.
   - **Confirma cadeia E2-E3 funcional após refactor**.

Tests co-localizados em submódulo `p197b_walk_figure`
em `tests.rs` (ou similar).

**Critério de saída**:
- 5 tests passam.
- Tests existentes não regridem.

### .F Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P197A
   baseline (1.843): **+5**.
3. `crystalline-lint .` zero violations (após
   `--fix-hashes`).
4. Tests `p197b_walk_figure::*` passam isoladamente.
5. Tests existentes (incluindo sentinela E3 P189B,
   tests P184B, P168, P195D Figure arm)
   **não regridem** — mutação legacy preservada;
   refactor sem mudança semântica.
6. Walk arm Figure usa helper `compute_figure`.
7. **Mutação legacy preservada** —
   `state.figure_numbers.push` + `state.local_figure_counters
   += 1` continuam.
8. **Comentário inline** presente em walk arm.
9. **L0 secção nova "Walk arm Figure migrado (P197B,
   cenário α)"** presente.
10. **Tabela Excepções M5** com E3 marcada
    "fechada estruturalmente".
11. `compute_labelled` P195D Figure arm **NÃO
    modificado**.
12. Variant `ElementPayload::Figure` NÃO modificado.
13. `from_tags` arm Figure NÃO modificado.
14. Trait `Introspector` NÃO modificado.
15. Consumer C3 (P184D) NÃO modificado.
16. Consumer C4 (P194B) NÃO modificado.
17. Snapshot tests verdes.
18. Linter passa final.

### .G Encerramento

Escrever
`00_nucleo/materialization/typst-passo-197b-relatorio.md`
com:

- Resumo: refactor extracção helper; cenário α
  confirmado; E3 fechada estruturalmente; sem Tag
  pós-recursão (caminho Introspector já activo desde
  P184).
- Confirmação `.F` (18 verificações).
- Δ tests vs baseline P197A (esperado +5).
- Hashes finais L0 (`rules/introspect.md`).
- Decisões de execução notáveis.
- Estado actual:
  - P197 série: A ✅ B ✅ | C pendente.
  - **E3 fechada estruturalmente** (vs E2 que ficou
    com residuo).
  - 73 passos executados.
- Pendências cumulativas: 3 excepções activas + 1
  residuo (E1, E2-residuo, E5, E6).
- Próximo passo: P197C — relatório consolidado P197 +
  actualização nota DEBT M5-residual.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate
   substancial.
2. Helper `compute_figure` criado (`.B`).
3. Walk arm Figure usa helper (`.C`).
4. Mutação legacy preservada (write paralelo M5).
5. Comentário inline presente (`.C`).
6. L0 `rules/introspect.md` actualizado (`.D`).
7. 5 tests sentinela passam (`.E`).
8. Verificações `.F` passam (18/18).
9. Tests existentes não regridem.
10. `compute_labelled` P195D Figure arm continua
    funcional.
11. Output observable em produção inalterado
    (refactor estilístico).
12. Relatório `.G` escrito.

---

## O que pode sair errado

- **Site walk arm Figure mudou** entre P197A e P197B:
  cláusula gate trivial — ajustar referência.
- **Helper `compute_figure` produz valor diferente do
  walk legacy** (off-by-one ou similar): cláusula gate
  substancial — investigar lógica.
- **Test 5 (`compute_labelled` continua funcional)
  falha**: indica que mutação legacy não foi preservada
  como esperado. Cláusula gate substancial — re-verificar
  `.C`.
- **Tests P184B/P168 regridem**: improvável (refactor
  sem mudança semântica). Investigar se acontecer.
- **Tests sentinela E3 P189B regridem**: indica que
  refactor alterou comportamento. Cláusula gate
  substancial.
- **`numbering.is_some() && caption.is_some()` gate
  divergente** do que P197A inferiu: cláusula gate
  trivial — preservar gate empírico.
- **Snapshot tests divergem**: improvável (refactor
  puro). Investigar se acontecer.
- **Linter divergência V13/V14**: cláusula gate
  trivial — `--fix-hashes`.

---

## Notas operacionais

- **Tamanho**: S/M. ~30 LOC produção (helper + refactor)
  + ~80 LOC tests + ~50 LOC L0.
- **Sem dependências externas novas**.
- **Sem ADR; sem DEBT formal**.
- **Padrão materializado**: refactor estilístico para
  consistência com pattern ADR-0069 (mesmo shape de
  helper que `compute_labelled` e
  `compute_heading_auto_toc`). Sem aplicação concreta
  do pattern (Tag pós-recursão dispensada).
- **Cláusula gate trivial**: aplicável a forma exacta
  do helper, recálculo de hashes, gate empírico.
- **Cláusula gate substancial**: aplicável a:
  - Helper diverge da lógica legacy.
  - `compute_labelled` P195D quebra.
  - Tests sentinela E3 regridem.
- **Distinção de P195D/P196B**: P197B **não emite Tag
  pós-recursão**. Refactor estilístico para
  consistência. Caminho Introspector já activo desde
  P184. Cenário α dispensa pattern ADR-0069.
- **Próximo passo P197C**: relatório consolidado P197
  (9 secções) + actualização DEBT M5-residual.
  Magnitude S puro.
- **Implicação para passos futuros (P198)**: auditor
  P198A deve verificar empiricamente se cenário α
  aplica a SetHeadingNumbering + CounterUpdate (variants
  + sub-stores + consumers podem já estar activos).
  Não-trivial — cada arm pode estar em estado diferente.
