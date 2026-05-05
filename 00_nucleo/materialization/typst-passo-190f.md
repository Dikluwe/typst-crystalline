# Passo P190F — Categoria Counters core + 2 helpers + cleanup defers

Quinto passo de implementação P190 (após P190A, B,
C, D, E). Magnitude **M+** — categoria mais complexa
da série porque combina:
- **Migration de 2 helpers ADR-0069** para
  Introspector path location-aware
  (`compute_labelled`, `compute_heading_auto_toc`).
- **Resolução de 2 defers acumulados** (`lang`
  P190D, `numbering_active` P190E).
- **Eliminação de campos counters core** (`flat`,
  `hierarchical`).
- **Walk arm Equation gate** finalmente migrado.

P190E confirmou empiricamente:
- Caso 1 obrigatório — walk readers preservavam
  write paralelo M5.
- Pattern "location-aware migration" estabelecido
  como pre-condição para eliminação completa.
- 26ª aplicação diagnóstico-primeiro consecutiva
  sem disparar gate substancial.

P190F resolve as últimas dependências do walk pre-pass
sobre `CounterStateLegacy` antes de Layouter migration
final (P190G+H+I).

Trabalho concreto:
1. **Migrar `compute_labelled`** (P195D helper):
   - Eliminar leituras de `state.flat`,
     `state.hierarchical`, `state.figure_numbers`,
     `state.lang`.
   - Substituir por queries Introspector
     location-aware (`flat_counter_at`,
     `formatted_counter_at`, etc.).
   - Pode exigir mudança de signature (passar
     `intr` + `location` em vez de `state`).
2. **Migrar `compute_heading_auto_toc`** (P196B
   helper):
   - Eliminar `state.is_numbering_active("heading")`.
   - Eliminar `state.format_hierarchical(...)`.
   - Substituir por queries Introspector
     location-aware.
3. **Migrar walk arm Equation gate**:
   - Eliminar `state.is_numbering_active("equation")`.
   - Substituir por
     `intr.is_numbering_active_at(...)` location-aware.
4. **Eliminar mutações walk arm**:
   - SetHeadingNumbering: write paralelo desnecessário.
   - SetEquationNumbering: idem.
5. **Eliminar campos `CounterStateLegacy`**:
   - `numbering_active` (resolve defer P190E).
   - `lang` (resolve defer P190D).
   - `flat` (counters core).
   - `hierarchical` (counters core).
6. **Eliminar Layouter assignments duais**
   correspondentes.
7. **Adaptar tests** dependentes.

Após P190F:
- `CounterStateLegacy`: 10 → **6 fields**.
- 2 helpers migrados (não eliminados).
- 2 defers resolvidos.
- Walk arms SetHeadingNumbering, SetEquationNumbering,
  Equation **puros** para esta categoria.
- Pattern "eliminação write paralelo M5": 5ª
  aplicação.
- Padrão "helper migration para Introspector
  location-aware": 1ª aplicação concreta.

**Pré-condição**: P190E concluído. Tests workspace
1.855 verdes; zero violations. `CounterStateLegacy`
10 fields. Defers `lang` + `numbering_active`
documentados.

**Restrições**:
- **Não** eliminar 2 helpers walk-internal
  (`compute_figure`, `compute_heading_for_toc`) —
  P190G/P190H/P190I.
- **Não** modificar trait `Introspector`
  (location-aware methods activos desde P185B).
- **Não** modificar `TagIntrospector`.
- **Não** eliminar struct `CounterStateLegacy` —
  P190I.
- **Não** materializar lacunas residuais.
- API pública preservada.
- **Não** quebrar paridade observable em re-update
  scenarios — usar location-aware obrigatório (per
  P190E achado).

---

## Sub-passos

### .A Auditoria L0 + estratégia

#### Inventário helpers actuais

1. Confirmar `compute_labelled` em
   `01_core/src/rules/introspect.rs`:
   - Localização: linha aproximada (per P195D).
   - Signature: `fn compute_labelled(state: &CounterStateLegacy, ...)`.
   - Reads de state legacy: identificar todos
     (`state.flat`, `state.hierarchical`,
     `state.figure_numbers`, `state.lang`, etc.).
   - Caller: walk arm Labelled (P195D).
   - Number of locations onde é chamado.

2. Confirmar `compute_heading_auto_toc` em
   `introspect.rs`:
   - Localização: linha aproximada (per P196B).
   - Signature.
   - Reads de state legacy.
   - Caller: walk arm Heading.

3. Confirmar walk arm Equation gate em
   `introspect.rs:579` (per P190E §3 referência):
   - Linha exacta da gate `if !state.is_numbering_active("equation")`.

#### Inventário API Introspector location-aware

4. Confirmar trait methods location-aware
   disponíveis (per P185B):
   - `is_numbering_active_at(key, location)`.
   - `flat_counter_at(key, location)`.
   - `formatted_counter_at(key, location)`.
   - `hierarchical_counter_at(key, location)` (se
     existe; verificar empiricamente).
   - Outros relevantes.

5. **Decisão obrigatória**: API location-aware
   suficiente para substituir todas as reads de
   helpers + walk arm gate?
   - Se sim: prosseguir.
   - Se não: identificar gaps; possivelmente
     adicionar trait method (improvável — P185B
     foi exhaustivo).

#### Inventário walk arm mutações

6. Confirmar walk arm SetHeadingNumbering (P198B):
   - Mutação preservada.
   - Eliminar após helpers migrarem.

7. Confirmar walk arm SetEquationNumbering (P199B):
   - Mutação preservada.
   - Eliminar após walk arm Equation gate migrar.

8. Confirmar walk arms outros que ainda mutam
   `state.flat` ou `state.hierarchical`:
   - Heading: `step_hierarchical("heading", level)`.
   - Equation: `step_flat("equation")` (se ainda
     activo).
   - Outros.
   - **Decisão crítica**: estas mutações são
     necessárias para helpers que ainda lêem state?
     Após helpers migrarem, mutações tornam-se
     desnecessárias.

#### Inventário Layouter consumers `flat`/`hierarchical`

9. `grep -rn "self.counter.flat\|self.counter.hierarchical\|self.counter.format_hierarchical\|self.counter.get_flat"
   01_core/src/`.
   - Identificar consumers.
   - Esperado: alguns consumers (per P190A §6: 10
     ocorrências `self.counter.X`).

#### Inventário Layouter assignments duais

10. `grep -rn "numbering_active\s*=\|flat\s*=\|hierarchical\s*=\|lang\s*="
    01_core/src/rules/layout/mod.rs`.

#### Tests dependentes

11. Identificar tests:
    - Tests de helpers `compute_labelled`,
      `compute_heading_auto_toc`.
    - Tests sentinela legacy.
    - Tests Layouter via `.counter.flat` etc.

#### Estratégia de execução

12. **Decisão obrigatória**: ordem dos sub-passos:
    - **Opção α** (helpers primeiro): migrar
      helpers `.B` + `.C`; depois walk arm gate
      `.D`; depois mutações walk `.E`; finalmente
      campos + assignments.
    - **Opção β** (campos primeiro): improvável
      por causa de dependências.
    
    Sugestão: **α** — helpers primeiro.

#### L0 alvos

13. Identificar L0s:
    - `entities/counter_state_legacy.md` (defer
      para P190I).
    - `rules/introspect.md` (helpers migrados +
      walk arm Equation purificado + walk arms
      SetHeading/SetEquation purificados).
    - `rules/layout/*.md` se aplicável.

Output: tabela com item + estado verificado.

**Critério de saída**:
- 2 helpers localizados com reads empíricos.
- API location-aware confirmada suficiente.
- Mutações walk arm catalogadas.
- Layouter consumers identificados.
- Tests dependentes listados.
- Estratégia α confirmada.

### .B Migrar `compute_labelled` para Introspector path

Per `.A.1`:

1. Mudar signature de
   `fn compute_labelled(state: &CounterStateLegacy,
   ...)` para `fn compute_labelled<I: Introspector>(intr:
   &I, location: Location, ...)` (forma exacta
   depende de empírico).

2. Substituir reads:
   - `state.flat.get(key)` →
     `intr.flat_counter_at(key, location)`.
   - `state.hierarchical.get(key)` →
     `intr.formatted_counter_at(key, location)` ou
     similar.
   - `state.figure_numbers.get(key)` →
     `intr.figure_number_at_index(...)` per P185B.
   - `state.lang` → ?
     - **Cláusula gate substancial**: `lang` não
       tem cobertura natural em Introspector.
     - **Opção α**: helper passa a receber `lang`
       como parâmetro adicional.
     - **Opção β**: mover `lang` para
       `LayouterRuntimeState` mas como Layouter não
       chama walk arm, exige refactor mais profundo.
     - **Opção γ**: walk arm passa `lang` como
       parâmetro a `compute_labelled`.
     - Sugestão: **Opção γ** — walk arm Labelled
       já tem acesso a `state.lang` (até ser
       eliminado em `.G`); passa-o como parâmetro
       ao helper.

3. Adaptar caller (walk arm Labelled em
   `introspect.rs`).

4. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- `compute_labelled` migrado.
- Signature actualizada.
- Caller adaptado.
- `cargo check` passa.

### .C Migrar `compute_heading_auto_toc` para Introspector path

Per `.A.2`:

1. Mudar signature análogo a `.B`.

2. Substituir reads:
   - `state.is_numbering_active("heading")` →
     `intr.is_numbering_active_at("numbering_active:heading",
     location)`.
   - `state.format_hierarchical("heading")` →
     `intr.formatted_counter_at("heading", location)`
     ou similar.

3. Adaptar caller (walk arm Heading).

4. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- `compute_heading_auto_toc` migrado.

### .D Migrar walk arm Equation gate

Per `.A.3`:

1. Em `introspect.rs:579` (linha aproximada):
   - Substituir `if !state.is_numbering_active("equation")
     { return; }` por
     `if !intr.is_numbering_active_at("numbering_active:equation",
     emitted_loc) { return; }` ou similar.
   - Forma exacta depende de Location disponível
     no contexto walk.

2. **Cláusula gate substancial**: walk arm tem
   acesso a Introspector?
   - Se sim: prosseguir.
   - Se não: refactor mais profundo necessário —
     walk fn signature change.

3. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- Walk arm Equation gate migrado.

### .E Eliminar mutações walk arm SetHeadingNumbering + SetEquationNumbering

Per `.A.6` e `.A.7`:

1. Em walk arm SetHeadingNumbering:
   - Eliminar `state.numbering_active.insert("heading",
     active)`.
   - Walk arm fica puro (apenas emite Tags).

2. Em walk arm SetEquationNumbering:
   - Eliminar `state.numbering_active.insert("equation",
     active)`.
   - Walk arm fica puro.

3. Comentário inline P190F substitui ou actualiza.

4. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- 2 mutações eliminadas.
- Walk arms puros para esta categoria.

### .F Eliminar mutações walk arm `state.step_hierarchical` / `state.step_flat`

Per `.A.8`:

1. **Decisão crítica**: eliminar mutações
   `state.step_hierarchical`/`step_flat` em walk
   arms (Heading, Equation, etc.)?
   - Após `.B`/`.C`/`.D`, helpers já não lêem
     state durante walk; mutações são
     desnecessárias.
   - **Opção α** (eliminar agora): walk arms ficam
     puros para counters core.
   - **Opção β** (defer para P190G/H): helpers
     `compute_figure`, `compute_heading_for_toc`
     ainda podem ler state durante walk. Defer.

2. Verificar empiricamente:
   - `compute_figure` (P197B) lê
     `state.figure_numbers`?
   - `compute_heading_for_toc` (P200B) lê
     `state.flat`?

3. Se ambos não lêem state: Opção α (eliminar
   agora).

4. Se algum lê: Opção β (defer; preservar
   mutações).

5. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- Mutações eliminadas ou deferidas per decisão.

### .G Eliminar 4 campos de `CounterStateLegacy`

Per `.A.10`:

1. Em
   `01_core/src/entities/counter_state_legacy.rs`:
   - Eliminar `pub numbering_active`.
   - Eliminar `pub lang` (resolve defer P190D).
   - Eliminar `pub flat` (per `.F` Opção α).
   - Eliminar `pub hierarchical` (per `.F` Opção α).

2. **Decisão crítica**: se `.F` decidiu Opção β
   (defer mutações), eliminar apenas
   `numbering_active` + `lang`; defer `flat` +
   `hierarchical` para P190G/H/I.

3. Adaptar `Default` impl, métodos `new()`, etc.

4. Confirmar `cargo check --workspace` passa.

5. **`CounterStateLegacy`: 10 → 6 fields** (Opção
   α completa) ou **8 fields** (defer).

**Critério de saída**:
- Campos eliminados per decisão.

### .H Eliminar Layouter assignments duais

Per `.A.10`:

1. Eliminar linhas de assignment correspondentes.

2. Comentário inline P190F.

3. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- Assignments eliminados.

### .I Adaptar tests

1. Identificar tests afectados (per `.A.11`).

2. Tests sentinela legacy redundantes — remover.
   Esperado: -5 a -10 tests.

3. Tests Layouter — adaptar.

4. Tests workspace verdes (Δ esperado: marginal
   negativo).

**Critério de saída**:
- Tests adaptados.

### .J Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs
   P190E baseline (1.855): **marginal negativo**.
3. `crystalline-lint .` zero violations (após
   `--fix-hashes`).
4. `compute_labelled` migrado (signature
   Introspector + location).
5. `compute_heading_auto_toc` migrado.
6. Walk arm Equation gate migrado.
7. Walk arm SetHeadingNumbering puro (mutação
   eliminada).
8. Walk arm SetEquationNumbering puro (mutação
   eliminada).
9. Per `.F` Opção α: walk arms Heading + Equation
   sem `step_hierarchical`/`step_flat`. Per `.F`
   Opção β: mutações preservadas.
10. `CounterStateLegacy.numbering_active` **NÃO
    existe**.
11. `CounterStateLegacy.lang` **NÃO existe**.
12. Per `.F` Opção α: `CounterStateLegacy.flat` +
    `.hierarchical` **NÃO existem**.
13. `CounterStateLegacy`: 6 fields (Opção α) ou 8
    fields (Opção β).
14. Layouter assignments duais eliminados.
15. Comentários inline P190F presentes.
16. Trait `Introspector` **NÃO modificado**.
17. `TagIntrospector` **NÃO modificado**.
18. **2 defers resolvidos** (`lang` P190D;
    `numbering_active` P190E).
19. Helpers `compute_figure` + `compute_heading_for_toc`
    **NÃO modificados** (P190G/H/I).
20. ADR-0070 PROPOSTO **NÃO transitada**.
21. Snapshot tests verdes.
22. Linter passa final.

### .K Encerramento

Escrever
`00_nucleo/materialization/typst-passo-190f-relatorio.md`
com:

- Resumo: categoria 5 (Counters core) parcial ou
  completa per `.F`; 2 helpers migrados; 2 defers
  resolvidos.
- Confirmação `.J` (22 verificações).
- Δ tests vs baseline P190E.
- Hashes finais.
- Decisões de execução notáveis:
  - Opção α vs β em `.F`.
  - Opção γ (`lang` como parâmetro) em `.B`.
  - Padrão "helper migration para Introspector
    location-aware" estabelecido.
- Estado actual:
  - P190 série: A ✅ B ✅ C ✅ D ✅ E ✅ F ✅ |
    G-I pendentes.
  - **Categoria 5 fechada** (parcial ou completa).
  - **2 defers resolvidos**.
  - 90 passos executados.
- Pendências cumulativas: 3 categorias restantes
  (6, 7) + P190I.
- Próximo passo: P190G — categoria 6 (Labels &
  TOC). Magnitude M.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate
   substancial inesperada.
2. `compute_labelled` migrado (`.B`).
3. `compute_heading_auto_toc` migrado (`.C`).
4. Walk arm Equation gate migrado (`.D`).
5. Walk arm SetHeadingNumbering + SetEquationNumbering
   purificadas (`.E`).
6. Decisão `.F` (Opção α/β) materializada.
7. 4 campos eliminados per `.F` decisão (`.G`).
8. Layouter assignments eliminados (`.H`).
9. Tests adaptados (`.I`).
10. Verificações `.J` passam (22/22).
11. **2 defers resolvidos** (`lang` + `numbering_active`).
12. Padrão "helper migration para Introspector
    location-aware" estabelecido.
13. Output observable em produção inalterado.
14. Relatório `.K` escrito.

---

## O que pode sair errado

- **Helpers `compute_*` lêem campos sem cobertura
  Introspector location-aware**: cláusula gate
  substancial — investigar API gaps em `.A.4`.
- **Walk fn signature exige mudança profunda**
  para passar `intr` + `location`: cláusula gate
  substancial — refactor maior.
- **`compute_figure` ou `compute_heading_for_toc`
  leem `state.flat` ou `state.hierarchical`**:
  cláusula gate trivial — Opção β em `.F` (defer).
- **`lang` parameter passing torna walk fn
  signature complexa**: cláusula gate trivial —
  acceitar Opção γ.
- **Tests Layouter regridem por mudança de
  re-update semantics**: cláusula gate substancial
  — investigar.
- **Snapshot tests divergem**: improvável com
  location-aware migration; investigar se acontecer.
- **Re-update scenarios revelam bugs**: cláusula
  gate substancial — confirmar location-aware
  migration correcta per P190E achado.
- **API location-aware gaps**: cláusula gate
  substancial — adicionar trait method (improvável).
- **Walk arm Equation gate sem acesso a
  Introspector**: cláusula gate substancial —
  refactor walk fn signature.
- **Linter divergência V13/V14**: cláusula gate
  trivial.

---

## Notas operacionais

- **Tamanho**: M+. ~80-120 LOC produção (helper
  migrations + walk arm gate migration + mutações
  eliminação + 4 fields eliminação + assignments)
  + ~30 LOC tests adaptados + ~20 LOC L0 (defer
  P190I).
- **Sem dependências externas novas**.
- **Sem ADR nova**.
- **Padrões aplicados**:
  - "Eliminação write paralelo M5" (5ª aplicação).
  - "Eliminação directa via Introspector"
    (resolução defer `numbering_active`).
  - "Helper migration para Introspector
    location-aware" (1ª aplicação concreta).
- **Cláusula gate trivial**: aplicável a forma
  exacta de signatures, recálculo de hashes,
  adaptação tests.
- **Cláusulas gate substancial possíveis**: API
  gaps; walk fn signature change; re-update
  semantics; helpers walk-internal lêem state.
- **Próximo passo P190G**: categoria 6 (Labels &
  TOC). Magnitude M. Trabalho concreto:
  - Eliminar campo `auto_label_counter`
    (walk-internal — substituível por
    `intr.resolved_labels.len()` ou similar).
  - Eliminar campo `resolved_labels` (caminho
    Introspector activo desde P193B).
  - Eliminar campo `headings_for_toc` (caminho
    Introspector activo desde P200B).
  - 3 campos eliminados de uma vez.
- **F1 progresso**: 10 → 6 (Opção α) ou 8 (Opção β)
  fields ortogonais.
- **F3 progresso**: Layouter ainda 20 fields;
  inalterado em P190F.
