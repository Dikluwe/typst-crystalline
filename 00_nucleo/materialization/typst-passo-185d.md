# Passo P185D — Tests E2E sincronização Locator

Terceiro passo de implementação P185 (após P185A diagnóstico,
P185B trait methods, P185C Layouter integration).
Magnitude **S**.

Adiciona tests E2E que validam empiricamente a
sincronização-por-construção entre Locator do Layouter
(P185C) e Locator do walk de introspect. Pré-condição
necessária para ADR-0068 transitar PROPOSTO → ACEITE em
P185E.

Após P185D:
- Sincronização Locator confirmada em pipeline real.
- Sequência de `Location`s produzida pelo Layouter
  iguala sequência produzida pelo walk de introspect
  para o mesmo `Content`.
- ADR-0068 candidato a ACEITE em P185E.
- Layouter location-aware **validado** — pronto para
  consumer migration (C1 em P187, C2 em P188).

**Pré-condição**: P185C concluído. Tests workspace 1.779
verdes; zero violations. Layouter ganhou `locator`
(`Locator::new()`) + `current_location: Option<Location>`;
gating no topo de `layout_content` via helper
`advance_locator_if_locatable`.

**Restrições**:
- **Não** modificar código de produção em
  `01_core/src/rules/`, `01_core/src/entities/`,
  `02_shell/`, `03_infra/`, `04_wiring/`.
- **Não** modificar walk de introspect.
- **Não** modificar Layouter, trait `Introspector`,
  `Locator`.
- **Não** migrar consumers — P187/P188.
- API pública preservada — P185D é apenas tests.
- Output observable em produção inalterado.

---

## Sub-passos

### .A Auditoria de tests existentes + plano de instrumentação

1. Inventariar tests existentes que cobrem o caminho:
   - `grep -rn "current_location\|Locator" 01_core/src/rules/layout/`.
   - Tests de P185B em `mod tests` de `introspector.rs`
     cobrem `is_numbering_active_at` / `flat_counter_at`
     em isolation. P185D estende para pipeline.
   - Confirmar ausência de tests prévios de
     sincronização Locator ↔ Layouter (esperado).

2. Inventariar mecanismo de instrumentação:
   - **Como obter `Vec<Location>` do walk de introspect?**
     - Opção 1: ler tags emitidas (cada tag locatable
       carrega `Location`).
     - Opção 2: instrumentar `from_tags` para colectar
       Locations.
     - Confirmar via empírico em `.A` qual está
       disponível. Provável: opção 1.
   - **Como obter `Vec<Location>` do Layouter?**
     - O field `current_location` muda durante
       `layout_content`. Para colectar a sequência:
       - Opção A: **alterar struct para colectar history**
         (não — viola restrição de não-tocar produção).
       - Opção B: **wrapper de teste** que intercepta
         `layout_content` e regista `current_location`
         após cada gating.
       - Opção C: **expor método `Layouter::locations_visited()`**
         para tests (não — viola restrição).
     - Decisão: **Opção B** — função de teste local que
       envolve `layout_content` e captura snapshots.
   - Se Opção B for impraticável (ex.: arms internos
     que chamam `layout_content` recursivamente sem
     hook): cláusula gate substancial — recuar para
     P185C e expor um hook explícito (alteração mínima
     de produção).

3. Confirmar `is_locatable` cobertura:
   - Per P185A §3.5: Heading, Figure, Cite, Metadata,
     State, StateUpdate, Outline, Bibliography,
     SetHeadingNumbering. Equation excluído.
   - Tests devem usar variants cobertos para garantir
     comparação válida.

4. Confirmar acesso a `Location` para asserções:
   - `Location` precisa de `Eq` e `Debug` para
     `assert_eq!` em tests. Confirmar empiricamente.

Output: tabela com item + estado + decisão sobre
mecanismo de instrumentação (Opção A/B/C).

**Critério de saída e gate de decisão**:
- Se Opção B é viável: prosseguir.
- Se Opção B impraticável: cláusula gate substancial.
  Recuar e re-arquitectar P185C para expor hook mínimo.

### .B Test E2E sincronização (caso central)

1. Construir documento de teste com sequência conhecida
   de locatables:
   - Heading + Figure + Cite (3 locatables, todos
     cobertos por `is_locatable`).
   - Sequência mínima viável.

2. Pipeline:
   - **Path A** (walk de introspect): correr
     `walk(content) → tags`; extrair sequência de
     `Location`s das tags emitidas.
   - **Path B** (Layouter): correr layout instrumentado
     que captura `current_location` após cada chamada a
     `advance_locator_if_locatable`; produz
     `Vec<Location>`.

3. Asserções:
   - `path_a.len() == path_b.len()` (mesmo número de
     locatables visitados).
   - `path_a == path_b` (sequências iguais elemento por
     elemento).
   - Se desigual: o test falha e indica
     desincronização — bloqueador real, não trivial.

4. Nome sugerido:
   `sincronizacao_locator_layouter_iguala_walk_introspect`.

**Critério de saída**:
- Test passa.
- Sincronização confirmada empiricamente.

### .C Test E2E mistura locatables + não-locatables

1. Construir documento com mistura:
   - Heading (locatable) + Text (não-locatable) +
     Figure (locatable) + Equation (não-locatable per
     P185A §3.5) + Cite (locatable).

2. Pipeline:
   - Path A: walk produz Locations apenas para os 3
     locatables (Heading, Figure, Cite).
   - Path B: Layouter avança `current_location` apenas
     para os 3 locatables.

3. Asserções:
   - Sequências iguais.
   - Locator não avança em Text e Equation (confirmação
     empírica que gating respeita `is_locatable`).

4. Nome sugerido:
   `gating_locator_apenas_em_locatables`.

**Critério de saída**:
- Test passa.
- Comportamento de gating validado.

### .D Test E2E `current_location` antes do primeiro locatable

1. Construir documento com Text (não-locatable) seguido
   de Heading.

2. Pipeline:
   - Imediatamente após processar Text:
     `current_location` deve ser `None` (nenhum
     locatable processado ainda).
   - Após processar Heading:
     `current_location` deve ser `Some(...)`.

3. Asserções:
   - Confirma que `Option<Location>` resolve a
     ambiguidade que `Location::from_raw(0)` teria
     criado (decisão registada em P185C §"Tipo do
     field").

4. Nome sugerido:
   `current_location_none_antes_de_primeiro_locatable`.

**Critério de saída**:
- Test passa.
- Decisão de tipo `Option<Location>` validada
  empiricamente.

### .E (Opcional) Test E2E pipeline completo com Introspector populado

1. Construir documento típico que combine
   `SetHeadingNumbering(true) + 3 headings`.

2. Pipeline:
   - Walk + from_tags → `TagIntrospector` populado.
   - Layout `layout_with_introspector(content, &intr)`.
   - Para cada heading processado, validar que
     `intr.is_numbering_active_at("numbering_active:heading",
     layouter.current_location.unwrap())` retorna
     `true`.

3. Asserções:
   - Demonstra que infra P185 (trait methods + Layouter
     fields) funciona end-to-end. P187 vai usar
     literalmente este padrão para migrar C1.

4. Nome sugerido:
   `pipeline_e2e_is_numbering_active_at_via_current_location`.

**Critério de saída**:
- Test passa.
- Validação end-to-end da arquitectura M3.

### .F Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P185C
   baseline (1.779): +3 a +4 dependendo de cobertura
   `.E`.
3. `crystalline-lint .` zero violations.
4. Tests `p185d_*` passam isoladamente
   (`cargo test --workspace --lib p185d`).
5. Tests existentes não regridem.
6. Output observable em produção inalterado (P185D não
   toca produção).
7. Snapshot tests ADR-0033 verdes.
8. Linter passa final.

### .G Encerramento

Escrever
`00_nucleo/materialization/typst-passo-185d-relatorio.md`
com:

- Resumo: 3-4 tests E2E adicionados; sincronização Locator
  confirmada empiricamente; decisão de tipo
  `Option<Location>` validada; pipeline end-to-end
  funciona.
- Confirmação `.F` (8 verificações).
- Δ tests vs baseline P185C (esperado +3 a +4).
- Hashes finais L0 (esperado zero edits — apenas tests).
- Decisões de execução notáveis (mecanismo de
  instrumentação escolhido).
- Estado actual:
  - P185 série: A ✅ B ✅ C ✅ D ✅ | E pendente.
  - **Sincronização ADR-0068 validada** — candidata a
    ACEITE em P185E.
  - 47 passos executados.
- Pendências cumulativas: inalteradas.
- Próximo passo: P185E (encerramento série + transição
  ADR-0068 PROPOSTO → ACEITE + relatório consolidado).

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate substancial;
   mecanismo de instrumentação confirmado (Opção B).
2. Test sincronização caso central passa (`.B`).
3. Test gating em mistura locatables/não-locatables passa
   (`.C`).
4. Test `current_location` antes do primeiro locatable
   passa (`.D`).
5. (Opcional) Test pipeline end-to-end com Introspector
   populado passa (`.E`).
6. Tests existentes não regridem.
7. Verificações `.F` passam (8/8).
8. Relatório `.G` escrito.
9. Output observable em produção inalterado.

---

## O que pode sair errado

- **Mecanismo de instrumentação Opção B impraticável**:
  cláusula gate substancial. Hook para captura de
  `current_location` exigiria mudança de produção em
  P185C. Recuar P185D e fazer revisão arquitectural de
  P185C para expor hook mínimo (ex.: callback opcional
  ou método `pub(crate)` para tests).
- **Sequências A e B diferem**: cláusula gate
  substancial **crítica**. Indica que ADR-0068 mecanismo
  M3 tem falha de sincronização-por-construção. Antes de
  reverter, investigar:
  - Determinismo do `Locator` foi violado?
  - Gating em `layout_content` corresponde literalmente
    ao gating em walk de introspect?
  - Algum arm do Layouter chama `layout_content`
    recursivamente de forma que avança Locator extra?
  - Caso confirmado: ADR-0068 fica PROPOSTO; P185
    re-arquitecta. Possível recuar para mecanismo M2
    (parâmetro propagado) com custo M-L mais alto.
- **Walk de introspect emite tags em ordem diferente**
  da ordem de visita do `is_locatable`: cláusula gate
  substancial. Investigar invariante explícito em
  `locatable.rs:11`.
- **`Location` não implementa `Eq` ou `Debug`**:
  cláusula gate trivial — adicionar derives ou usar
  comparação manual.
- **Test `.D` (None antes do primeiro locatable) falha
  porque Layouter avança Locator antes de qualquer
  locatable**: indica bug em P185C — gating dispara em
  algum arm não-locatable. Investigar.
- **Tests existentes regridem**: improvável; P185D não
  toca produção. Se acontecer, investigar.
- **Snapshot tests divergem**: improvável pelo mesmo
  motivo. Se acontecer, indica que tests de produção
  estão a usar `current_location` indirectamente, o que
  não devia ser possível (P185C garante consumer
  ausente).

---

## Notas operacionais

- **Tamanho**: S puro. ~80-150 LOC tests + helpers.
- **Sem código de produção tocado** — tests apenas.
- **Sem dependências externas novas**.
- **Pré-condição P185E**: este passo concluído.
- **Padrão**: tests E2E de validação arquitectural.
  Diferente dos E2E de paridade observable (P182E,
  P184E) — aqui o objectivo é confirmar invariante
  estrutural, não output equivalente.
- **Cláusula gate trivial**: aplicável a derives,
  imports, formato de tests.
- **Cláusula gate substancial**: aplicável se
  instrumentação Opção B for impraticável OU se
  sincronização literalmente falhar. Caso 2 é o **risco
  central** — se acontecer, ADR-0068 não transita para
  ACEITE e P185 escala arquiteturalmente.
- **`.E` é opcional** mas recomendado — fornece
  validação end-to-end que P187 vai usar como blueprint
  literal. Default é incluir.
- **Importância de `.B`** (caso central): se este test
  passa, o invariante de sincronização-por-construção
  da ADR-0068 está empiricamente validado. Se falha,
  todo o trabalho M3 está em risco.
- **P185D é o gate de qualidade da série P185**.
  Diferente dos `*D/E` de outras séries (que confirmam
  trabalho funcional), aqui o objectivo é validar
  decisão arquitectural. Se passar, P185 fecha
  limpamente; se falhar, P185 entra em revisão.
