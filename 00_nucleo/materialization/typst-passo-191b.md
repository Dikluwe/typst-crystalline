# Passo P191B — Implementação Opção A + 1 helper validation

Primeiro passo de implementação P191 (após P191A
diagnóstico). Magnitude **M+ genuína** — redesign
arquitectural do walk pipeline.

P191A confirmou empiricamente:
- **Opção A escolhida**: walk fn ganha `intr: &mut
  TagIntrospector` parameter.
- **`from_tags::from_tags` eliminado** em P191B —
  walk arms popularem `intr` directamente.
- **12 ElementPayload variants** migram from_tags
  arms para walk arms.
- **2 helpers walk-readers migráveis**:
  `compute_labelled`, `compute_heading_auto_toc`.
- **2 helpers walk-internal**: `compute_figure`,
  `compute_heading_for_toc` (inalterados).
- **Estratégia incremental**: P191B migra 1 helper
  como prova de conceito (`compute_heading_auto_toc`);
  P191C migra `compute_labelled` + cleanup.

P191B é trabalho **arquitectural genuíno**:
- Walk fn signature change (afecta ~20 recursive
  call sites).
- Eliminação de from_tags (12 arms migram).
- 1 helper migrado para Introspector path
  location-aware.
- Walk arm Equation gate migrado.

Trabalho concreto:
1. Adicionar parameter `intr: &mut TagIntrospector`
   ao walk fn.
2. Actualizar ~20 recursive walk call sites.
3. Migrar 12 from_tags arms para walk arms
   (popularem `intr` directamente onde Tag seria
   emitida ou pós-recursão).
4. Eliminar `from_tags::from_tags` ou reduzir a
   no-op.
5. Simplificar `introspect_with_introspector`:
   walk → return (sem etapa from_tags).
6. Migrar `compute_heading_auto_toc` para signature
   `<I: Introspector>(intr: &I, location: Location,
   ...)`.
7. Migrar walk arm Equation gate para
   `intr.is_numbering_active_at(...)` location-aware.
8. Adicionar tests novos sentinela mecanismo.
9. Adaptar tests existentes conforme necessário.

Após P191B:
- Walk fn aceita `&mut TagIntrospector`.
- `from_tags::from_tags` eliminado ou no-op.
- 12 walk arms popularem `intr` directamente.
- 1 helper migrado (`compute_heading_auto_toc`).
- Walk arm Equation gate migrado.
- Pattern ADR-0069 stylesheet preservado (5
  variantes operacionais funcionais).
- Tests workspace verdes (Δ marginal).

**Pré-condição**: P191A concluído. Tests workspace
1.855 verdes; zero violations. ADR-0071 PROPOSTO
criada. Lembrete formal P190 série pausada
documentado.

**Restrições**:
- **Não** migrar `compute_labelled` — defer P191C.
- **Não** modificar trait `Introspector` (Opção A
  preserva trait).
- **Não** modificar `TagIntrospector` struct
  fields (apenas mutar via novo parameter).
- **Não** eliminar `CounterStateLegacy` — P190I
  após retomar P190 série.
- **Não** eliminar campos `CounterStateLegacy`
  walk-readable — P190G/H/I após P191 fechar.
- **Não** modificar Layouter — P190G+.
- **Não** materializar lacunas residuais.
- API pública preservada.
- **Lembrete crítico**: P190 série em pausa —
  retomar P190G após P191C fechar.

---

## Sub-passos

### .A Auditoria L0

#### Inventário walk fn signature actual

1. Confirmar walk fn signature em
   `01_core/src/rules/introspect.rs`:
   ```
   fn walk(
       content:           &Content,
       state:             &mut CounterStateLegacy,
       locator:           &mut Locator,
       tags:              &mut Vec<Tag>,
       label_from_parent: Option<&Label>,
   );
   ```

2. Identificar **todas as recursive walk call
   sites** dentro de walk arms:
   - `grep -n "walk(" 01_core/src/rules/introspect.rs`.
   - Esperado: ~20 chamadas (per P191A §9).
   - Cada chamada precisará de novo parameter
     `intr`.

3. Confirmar `introspect_with_introspector` (ou
   função análoga):
   - Localização exacta.
   - Pipeline actual: walk → from_tags → return.

#### Inventário from_tags arms

4. Confirmar `from_tags::from_tags` em
   `01_core/src/rules/introspect/from_tags.rs`:
   - Match exhaustivo sobre 12 ElementPayload
     variants (per P190A §6).
   - Cada arm: que sub-store popula em
     `TagIntrospector`.
   - Lista exacta empírica.

5. Mapeamento walk arm ↔ from_tags arm:
   - Cada walk arm que emite Tag::Start com
     ElementPayload tem from_tags arm
     correspondente.
   - P191B move logic from_tags → walk arm.

#### Inventário helper a migrar

6. Confirmar `compute_heading_auto_toc` em
   `introspect.rs`:
   - Localização exacta.
   - Reads de state legacy:
     `state.is_numbering_active("heading")` +
     `state.format_hierarchical("heading")`.
   - Caller único: walk arm Heading.

7. Confirmar API location-aware Introspector:
   - `is_numbering_active_at(key, location) ->
     bool` (P185B).
   - `formatted_counter_at(key, location) ->
     Option<String>` (P185B).
   - Confirmar empíricamente que retornam valores
     correctos durante walk (i.e., `intr` é
     populated incrementalmente para que queries
     sejam válidas no momento da chamada).

#### Inventário walk arm Equation gate

8. Confirmar walk arm Equation gate em
   `introspect.rs:579` (per P190E §3 referência):
   - `if !state.is_numbering_active("equation")
     { return; }`.
   - Migrar para `intr.is_numbering_active_at(...)`.

#### Inventário tests dependentes

9. Identificar tests:
   - Tests `introspect_with_introspector_*` que
     verificam pipeline.
   - Tests `from_tags_*` que verificam from_tags
     directamente.
   - **Decisão**: tests `from_tags_*` ficam
     redundantes após eliminação. Adaptar ou remover.

#### L0 alvos

10. Identificar L0s:
    - `rules/introspect.md` (signature change +
      walk arms popularem intr).
    - `rules/introspect/from_tags.md` (eliminado
      ou no-op).

Output: tabela com item + estado verificado.

**Critério de saída**:
- Walk fn signature confirmada.
- ~20 recursive call sites localizados.
- 12 from_tags arms catalogados.
- Mapeamento walk arm ↔ from_tags arm completo.
- `compute_heading_auto_toc` localizado.
- API location-aware confirmada utilizável durante
  walk.
- Walk arm Equation gate localizado.
- Tests dependentes listados.

### .B Walk fn signature change

1. Em `01_core/src/rules/introspect.rs`:
   - Adicionar parameter `intr: &mut TagIntrospector`
     ao walk fn:
     ```
     fn walk(
         content:           &Content,
         state:             &mut CounterStateLegacy,
         locator:           &mut Locator,
         tags:              &mut Vec<Tag>,
         intr:              &mut TagIntrospector,
         label_from_parent: Option<&Label>,
     );
     ```

2. Confirmar `cargo check --workspace` falha
   (esperado — recursive call sites não actualizados).

**Critério de saída**:
- Signature actualizada.
- Compilação falha (esperado — `.C` actualiza
  callers).

### .C Actualizar recursive walk call sites

Per `.A.2`:

1. Para cada chamada `walk(content, state, locator,
   tags, label)` em walk arms:
   - Substituir por `walk(content, state, locator,
     tags, intr, label)`.

2. Esperado: ~20 chamadas actualizadas.

3. Confirmar `cargo check --workspace` passa para
   recursive calls (mas pode falhar para callers
   externos).

**Critério de saída**:
- Recursive call sites actualizados.

### .D Actualizar `introspect_with_introspector`

Per `.A.3`:

1. Em `introspect_with_introspector`:
   - Inicializar `let mut intr =
     TagIntrospector::default();` antes de walk.
   - Passar `&mut intr` para walk.
   - Após walk: `intr` está populated (sem necessidade
     de from_tags).
   - **Decisão crítica**: `from_tags::from_tags`
     ainda chamado?
     - **Opção α**: eliminar chamada — `intr` já
       populated.
     - **Opção β**: chamar como no-op para validação
       (sanity check); eliminar em P191C.
   - Sugestão: **α** — eliminação directa.

2. Função simplifica:
   ```
   pub fn introspect_with_introspector(
       content: &Content,
   ) -> (CounterStateLegacy, TagIntrospector) {
       let mut state = CounterStateLegacy::default();
       let mut locator = Locator::default();
       let mut tags = Vec::new();
       let mut intr = TagIntrospector::default();
       walk(content, &mut state, &mut locator,
            &mut tags, &mut intr, None);
       (state, intr)
   }
   ```

3. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- Pipeline simplificado.
- from_tags eliminado da pipeline principal.

### .E Migrar 12 from_tags arms para walk arms

Per `.A.4` e `.A.5`:

Para cada from_tags arm, mover logic para walk arm
correspondente. Walk arm popula `intr` directamente
no momento exacto onde from_tags processaria a Tag.

Esquema geral:
- Walk arm que emite `Tag::Start(loc,
  ElementInfo::new(payload))` com ElementPayload X:
  - Antes: from_tags processa após walk completo.
  - Depois: walk arm popula `intr.X.push(...)`
    directamente no momento da emissão.

**Iterar pelos 12 ElementPayload variants** (per
`.A.4`):
1. Heading.
2. Figure.
3. Labelled.
4. Equation.
5. Outline.
6. Bibliography.
7. CounterUpdate.
8. SetHeadingNumbering.
9. SetEquationNumbering.
10. HeadingForToc (P200B).
11. (outro - per P190A §6).
12. (outro).

**Cláusula gate substancial**: ordem de population
importa. Per P191A §4 cláusula 6:
> Sequencial natural: walk arm SetX populate
> `intr.state` ANTES de walk arm Equation query
> gate.

Confirmar que ordem walk natural respeita
dependências.

**Critério de saída**:
- 12 walk arms popularem `intr` directamente.
- `cargo check --workspace` passa.
- Tests existentes podem regredir temporariamente
  (esperado — adaptação `.I`).

### .F Eliminar `from_tags::from_tags`

Per `.D` Opção α:

1. Em `01_core/src/rules/introspect/from_tags.rs`:
   - **Opção α**: eliminar função
     `from_tags::from_tags` completamente.
   - **Opção β**: reduzir a no-op
     (`pub fn from_tags(_tags: &[Tag], ...) ->
     TagIntrospector { TagIntrospector::default()
     }`) para evitar quebrar imports externos
     temporários.
   - Sugestão: **α** se nenhum caller externo;
     **β** se caller externo.

2. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- from_tags eliminado ou no-op.

### .G Migrar `compute_heading_auto_toc`

Per `.A.6` e `.A.7`:

1. Mudar signature:
   ```
   fn compute_heading_auto_toc<I: Introspector>(
       intr:     &I,
       location: Location,
       counter:  usize,
   ) -> (Label, String) {
       // Substituir reads:
       // state.is_numbering_active("heading") →
       //   intr.is_numbering_active_at(
       //     "numbering_active:heading", location)
       // state.format_hierarchical("heading") →
       //   intr.formatted_counter_at("heading",
       //     location).unwrap_or_default()
       ...
   }
   ```
   - Forma exacta depende de empírico.

2. Adaptar caller (walk arm Heading):
   - Walk arm Heading agora passa `intr` + Location
     ao helper.

3. **Cláusula gate substancial**: helper precisa
   de Location empíricamente disponível no walk arm
   Heading. Per P196B + P200B, `emitted_loc` é
   variável existente. Confirmar.

4. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- `compute_heading_auto_toc` migrado.
- Caller adaptado.

### .H Migrar walk arm Equation gate

Per `.A.8`:

1. Em `introspect.rs:579` (linha aproximada):
   - Substituir:
     ```
     if !state.is_numbering_active("equation") {
         return;
     }
     ```
   - Por:
     ```
     if !intr.is_numbering_active_at(
            "numbering_active:equation",
            emitted_loc,
        ) {
         return;
     }
     ```

2. **Cláusula gate substancial**: walk arm Equation
   tem `emitted_loc` ou Location equivalente
   disponível no scope?
   - Se sim: prosseguir.
   - Se não: derivar Location via `locator`
     manualmente.

3. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- Walk arm Equation gate migrado.

### .I Adaptar tests

1. Identificar tests afectados (per `.A.9`):
   - Tests `from_tags_*` redundantes — remover.
   - Tests `introspect_with_introspector_*` —
     adaptar se necessário.
   - Tests sentinela `compute_heading_auto_toc` —
     adaptar para nova signature.

2. **Adicionar 1-2 tests novos sentinela**
   mecanismo:
   - Test "walk popula intr directamente" — verifica
     que `TagIntrospector` retornado tem sub-stores
     populated correctamente sem chamar from_tags.
   - Test "compute_heading_auto_toc lê via
     Introspector path" — paridade com
     comportamento legacy.

3. Tests workspace verdes (Δ esperado: marginal —
   tests from_tags removidos compensados por tests
   novos sentinela).

**Critério de saída**:
- Tests adaptados.
- Tests novos sentinela passam.

### .J Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs
   P191A baseline (1.855): **marginal**.
3. `crystalline-lint .` zero violations (após
   `--fix-hashes`).
4. Walk fn signature aceita `&mut TagIntrospector`.
5. ~20 recursive call sites actualizados.
6. `introspect_with_introspector` simplificado
   (sem from_tags na pipeline).
7. 12 walk arms popularem `intr` directamente.
8. `from_tags::from_tags` eliminado ou no-op.
9. `compute_heading_auto_toc` migrado para
   Introspector path location-aware.
10. Walk arm Equation gate migrado.
11. `compute_labelled` **NÃO modificado** (defer
    P191C).
12. `compute_figure` **NÃO modificado** (walk-internal).
13. `compute_heading_for_toc` **NÃO modificado**
    (walk-internal).
14. Trait `Introspector` **NÃO modificado**.
15. `TagIntrospector` fields **NÃO modificados**
    (apenas mutados via novo parameter).
16. `CounterStateLegacy` **NÃO modificado** em P191B
    (defer P190G/H/I).
17. Layouter **NÃO modificado**.
18. Pattern ADR-0069 stylesheet preservado (5
    variantes operacionais).
19. ADR-0071 PROPOSTO **NÃO transitada** (ACEITE
    em P191C).
20. Comentários inline P191B presentes.
21. Snapshot tests verdes.
22. Linter passa final.

### .K Encerramento

Escrever
`00_nucleo/materialization/typst-passo-191b-relatorio.md`
com:

- Resumo: Opção A implementada; from_tags eliminado;
  1 helper migrado; walk arm Equation gate migrado;
  prova de conceito validada.
- Confirmação `.J` (22 verificações).
- Δ tests vs baseline P191A.
- Hashes finais.
- Decisões de execução notáveis:
  - Eliminação from_tags (Opção α vs β em `.D` e
    `.F`).
  - Forma exacta de signature `compute_heading_auto_toc`.
  - Cláusulas gate substanciais resolvidas (Location
    em walk arm Heading + Equation).
  - Ordem de population walk arms vs queries
    (cláusula 6 P191A).
- Estado actual:
  - P191 série: A ✅ B ✅ | C pendente.
  - **Mecanismo Opção A validado**.
  - 92 passos executados.
  - **LEMBRETE: P190 série em pausa**.
- Pendências cumulativas: P191C (`compute_labelled`
  + cleanup + ADR ACEITE).
- Próximo passo: P191C — migrar `compute_labelled`
  + cleanup + ADR-0071 ACEITE + lembrete formal de
  retomar P190G.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate
   substancial inesperado.
2. Walk fn signature actualizada (`.B`).
3. Recursive call sites actualizados (`.C`).
4. `introspect_with_introspector` simplificado
   (`.D`).
5. 12 walk arms popularem `intr` directamente
   (`.E`).
6. `from_tags::from_tags` eliminado ou no-op (`.F`).
7. `compute_heading_auto_toc` migrado (`.G`).
8. Walk arm Equation gate migrado (`.H`).
9. Tests adaptados + sentinelas mecanismo (`.I`).
10. Verificações `.J` passam (22/22).
11. Pattern ADR-0069 stylesheet preservado.
12. Output observable em produção inalterado.
13. Relatório `.K` escrito.

---

## O que pode sair errado

- **Recursive call sites em quantidade não prevista
  (>20)**: cláusula gate trivial — actualizar todos.
- **Ordem de walk arm population vs queries
  diverge**: cláusula gate substancial — investigar
  cada caso.
- **Walk arm Equation `emitted_loc` não disponível
  no scope**: cláusula gate substancial — derivar
  Location manualmente ou refactor walk arm
  estrutura.
- **Walk arm Heading sem Location no scope para
  passar a `compute_heading_auto_toc`**: cláusula
  gate substancial.
- **`is_numbering_active_at` retorna valor stale
  durante walk** (porque `intr.state` ainda não
  populated quando query é feita): cláusula gate
  substancial — confirmar ordem natural walk.
- **`TagIntrospector::default()` não é construtor
  válido**: cláusula gate trivial — confirmar API.
- **Tests `from_tags_*` em quantidade significativa
  regridem**: padrão pragmático auditor #1.
- **Snapshot tests divergem**: investigar — pode
  indicar divergência de output observable.
- **Pattern ADR-0069 cenário α por construção
  P199B quebra**: cláusula gate substancial —
  investigar.
- **Linter divergência V13/V14**: cláusula gate
  trivial.

---

## Notas operacionais

- **Tamanho**: M+ genuíno. ~150-200 LOC produção
  (signature change + recursive call sites + 12
  walk arms migration + helper migration + gate
  migration + comentários) + ~50 LOC tests
  adaptados + sentinelas + ~30 LOC L0.
- **Sem dependências externas novas**.
- **Sem ADR nova** (ADR-0071 PROPOSTO já criada).
- **Pattern ADR-0069 stylesheet preservado** — 5
  variantes operacionais funcionais por construção.
- **Cláusula gate trivial**: aplicável a forma
  exacta de signatures, recálculo de hashes,
  adaptação tests.
- **Cláusulas gate substancial possíveis**:
  - Location não disponível em walk arm scope.
  - Ordem de walk arm population vs queries
    diverge.
  - `is_numbering_active_at` retorna stale durante
    walk.
  - Pattern ADR-0069 cenário α quebra.
- **Próximo passo P191C**: `compute_labelled`
  migration + cleanup + ADR-0071 ACEITE + lembrete
  formal P190 série retomar. Magnitude S-M.
- **LEMBRETE FORMAL**: P190 série em pausa após
  P190F. Retomar P190G após P191C fechar.
  Categorias 6, 7, P190I + 4 defers acumulados
  pendentes.
- **Validação prova de conceito**: P191B testa o
  mecanismo Opção A com 1 helper. Se mecanismo
  funciona, P191C migra `compute_labelled` (mais
  complexo) com confiança elevada.
