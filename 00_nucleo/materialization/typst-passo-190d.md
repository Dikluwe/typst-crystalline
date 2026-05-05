# Passo P190D — Categoria Document metadata (combinação 2 padrões)

Terceiro passo de implementação P190 (após P190A
diagnóstico, P190B Bibliography, P190C Page
tracking). Magnitude **M** — aplicação simultânea
de **2 padrões diferentes**:
- `is_readonly` + `lang` → padrão **Layouter-runtime
  → struct dedicada** (P190C).
- `has_outline` → padrão **eliminação directa via
  Introspector** (P190B).

P190C estabeleceu:
- Struct `LayouterRuntimeState` (2 fields:
  `label_pages` + `known_page_numbers`).
- Padrão para fields Layouter-runtime sem
  cobertura natural em Introspector.
- 0 cláusulas gate disparadas.

P190D expande `LayouterRuntimeState` para 4 fields
(adicionando `is_readonly` + `lang`) e elimina
`has_outline` via cobertura existente.

Trabalho concreto:
1. Expandir `LayouterRuntimeState`:
   - Adicionar `is_readonly: bool` (DEBT-13 per
     P190A §3 — campo Layouter-runtime).
   - Adicionar `lang: ...` (config field externo
     per P190A §3).
2. Migrar consumers Layouter de
   `self.counter.is_readonly` para
   `self.runtime.is_readonly`.
3. Migrar consumers Layouter de
   `self.counter.lang` para `self.runtime.lang`.
4. Migrar consumers `state.has_outline` para
   `intr.has_kind(ElementKind::Outline)` (ou similar
   per P178).
5. Eliminar 3 fields de `CounterStateLegacy`:
   `is_readonly` + `lang` + `has_outline`.
6. Adaptar tests dependentes.

Após P190D:
- `CounterStateLegacy`: 12 → **9 fields**.
- `LayouterRuntimeState`: 2 → **4 fields**.
- Pattern "eliminação write paralelo M5": 3ª
  aplicação concreta.
- Padrão "Layouter-runtime → struct dedicada": 2ª
  aplicação (atinge 4 fields esperados).

**Pré-condição**: P190C concluído. Tests workspace
1.867 verdes; zero violations. `CounterStateLegacy`
12 fields. `LayouterRuntimeState` existe com 2
fields.

**Restrições**:
- **Não** modificar walk arms (estes campos não
  são tocados por walk).
- **Não** modificar `from_tags` — `has_outline` já
  tem cobertura via `kind_index[Outline]` (P178).
- **Não** modificar trait `Introspector`.
- **Não** modificar `TagIntrospector`.
- **Não** eliminar struct `CounterStateLegacy` —
  P190I.
- **Não** modificar 4 helpers `compute_*`.
- **Não** materializar lacunas residuais.
- API pública preservada.

---

## Sub-passos

### .A Auditoria L0

#### Inventário 3 fields a eliminar

1. Confirmar fields em
   `01_core/src/entities/counter_state_legacy.rs`:
   - `pub is_readonly: bool` — type confirmar.
   - `pub lang: ...` — type confirmar (provável
     `String` ou `Option<String>`).
   - `pub has_outline: bool` — type confirmar.

2. Identificar walk arms ou `from_tags` arms que
   mutam estes fields:
   - **Esperado**: nenhum walk arm muta.
   - `has_outline` provavelmente lido em `Layouter`
     para gating fixpoint.
   - `is_readonly` — escrita via Layouter (DEBT-13).
   - `lang` — escrita via Layouter ou config externa.

#### Inventário consumers `is_readonly` + `lang`

3. Identificar consumers Layouter:
   - `grep -rn "self.counter.is_readonly\|self.counter.lang"
     01_core/src/`.
   - Identificar contexto.

4. Identificar mutações Layouter próprias destes
   fields:
   - `grep -rn "counter.is_readonly\s*=\|counter.lang\s*=\|counter.is_readonly.set"
     01_core/src/`.

5. Identificar Layouter assignments duais (per
   padrão P190B/P190C):
   - `grep -rn "is_readonly\s*=\|lang\s*="
     01_core/src/rules/layout/mod.rs`.

#### Inventário consumers `has_outline`

6. Identificar consumers de `state.has_outline`:
   - `grep -rn "has_outline" 01_core/src/`.
   - Esperado: `Layouter` consulta para decidir se
     fixpoint de páginas é necessário.

7. Confirmar API equivalente Introspector (P178):
   - `intr.has_kind(ElementKind::Outline)` ou
     similar.
   - `intr.kind_index.get(&ElementKind::Outline).map(|v| !v.is_empty()).unwrap_or(false)`
     ou método trait dedicado.
   - Confirmar nome exacto empiricamente.

#### Inventário walk arm Outline

8. Confirmar walk arm Outline em `introspect.rs`:
   - **Decisão obrigatória**: walk arm Outline ainda
     muta `state.has_outline = true`?
   - Se sim: mutação legacy preservada como write
     paralelo M5; eliminada em P190I.
   - Se não: caminho Introspector via P178 já
     completo.

#### Tests dependentes

9. Identificar tests:
   - Tests que verificam `state.is_readonly`,
     `state.lang`, `state.has_outline` directamente.
   - Tests Layouter dependentes via `.counter.X`.

#### L0 alvos

10. Identificar L0s:
    - `entities/counter_state_legacy.md` (fields
      eliminados — diferido para P190I).
    - `entities/layouter_runtime_state.md` (fields
      adicionados).

Output: tabela com item + estado verificado.

**Critério de saída**:
- 3 fields localizados.
- Layouter consumers + mutações + assignments
  identificados.
- Walk arm Outline state confirmado.
- API Introspector para `has_outline` confirmada.

### .B Expandir struct `LayouterRuntimeState`

1. Em
   `01_core/src/entities/layouter_runtime_state.rs`:
   - Adicionar 2 fields:
     ```
     /// Indica se Layouter está em modo read-only
     /// (DEBT-13 P190A §3).
     pub is_readonly: bool,

     /// Língua do documento (config field externo).
     /// Type confirmado em .A.1.
     pub lang: ...,
     ```
   - Forma exacta de `lang` depende de `.A.1`.

2. Adaptar `Default` impl se necessário.

3. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- `LayouterRuntimeState`: 2 → 4 fields.
- `cargo check --workspace` passa.

### .C Migrar consumers `is_readonly`

1. Per `.A.3` e `.A.4`:
   - Substituir `self.counter.is_readonly` por
     `self.runtime.is_readonly` em todos os sítios.
   - Inclui mutações Layouter próprias.

2. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- Consumers `is_readonly` migrados.

### .D Migrar consumers `lang`

1. Análogo a `.C`:
   - Substituir `self.counter.lang` por
     `self.runtime.lang`.

2. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- Consumers `lang` migrados.

### .E Migrar consumers `has_outline` (padrão diferente)

Per `.A.6` e `.A.7` — **padrão de eliminação directa
via Introspector** (P190B style):

1. Substituir `self.counter.has_outline` ou
   `state.has_outline` por chamada equivalente
   Introspector:
   - Forma exacta depende de `.A.7`.
   - Provável:
     `self.introspector.has_kind(ElementKind::Outline)`
     ou similar.

2. Mutação `state.has_outline = true` em walk arm
   Outline (per `.A.8`):
   - **Se ainda existe**: preservar como write
     paralelo M5; eliminar em P190I.
   - **Se já purificada**: confirmar caminho
     Introspector activo.

3. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- Consumers `has_outline` migrados para Introspector
  path.
- Walk arm Outline analisado per `.A.8`.

### .F Eliminar 3 fields de `CounterStateLegacy`

1. Em
   `01_core/src/entities/counter_state_legacy.rs`:
   - Eliminar field `pub is_readonly: bool`.
   - Eliminar field `pub lang: ...`.
   - Eliminar field `pub has_outline: bool`.
   - Adaptar `Default` impl ou `new()`.

2. Confirmar `cargo check --workspace` passa.

3. **`CounterStateLegacy`: 12 → 9 fields**.

**Critério de saída**:
- 3 fields eliminados.
- `cargo check --workspace` passa.

### .G Eliminar Layouter assignments duais

1. Per `.A.5`:
   - Eliminar linhas de assignment para
     `is_readonly`, `lang`, `has_outline`.
   - Comentário inline P190D substitui ou
     actualiza.

2. **Decisão sobre `has_outline` assignment**:
   - Se Layouter assignment `l.counter.has_outline
     = ...` existe: eliminar (caminho Introspector
     fornece).
   - Se `state.has_outline` é mutado pelo walk arm
     Outline: preservar mutação até P190I.

3. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- Assignments eliminados.
- Comentários inline actualizados.

### .H Adaptar tests

1. Identificar tests afectados:
   - Tests que verificavam `state.is_readonly`,
     `state.lang`, `state.has_outline` directamente.
   - Tests Layouter via `.counter.X`.

2. Adaptação:
   - Tests redundantes — remover ou adaptar.
   - Tests relevantes — substituir
     `layouter.counter.X` por `layouter.runtime.X`
     ou `intr.has_kind(...)`.

3. Tests workspace verdes (Δ esperado: 0 ou marginal
   negativo).

**Critério de saída**:
- Tests adaptados.

### .I Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P190C
   baseline (1.867): **0 ou marginal negativo**.
3. `crystalline-lint .` zero violations (após
   `--fix-hashes`).
4. **`LayouterRuntimeState`: 4 fields** (era 2).
5. `CounterStateLegacy.is_readonly` **NÃO existe**.
6. `CounterStateLegacy.lang` **NÃO existe**.
7. `CounterStateLegacy.has_outline` **NÃO existe**.
8. `CounterStateLegacy`: 9 fields (era 12).
9. Layouter consumers `is_readonly` migrados.
10. Layouter consumers `lang` migrados.
11. Consumers `has_outline` migrados para Introspector
    path.
12. Layouter assignments duais eliminados.
13. Comentários inline P190D presentes.
14. Trait `Introspector` **NÃO modificado**.
15. `TagIntrospector` **NÃO modificado**.
16. Walk arm Outline analisado per `.A.8` —
    mutação preservada como write paralelo (se
    existe) ou já pura.
17. ADR-0070 PROPOSTO **NÃO transitada**.
18. Snapshot tests verdes.
19. Linter passa final.

### .J Encerramento

Escrever
`00_nucleo/materialization/typst-passo-190d-relatorio.md`
com:

- Resumo: categoria 3 (Document metadata) eliminada;
  combinação de 2 padrões; `LayouterRuntimeState`
  4 fields; `CounterStateLegacy` 9 fields.
- Confirmação `.I` (19 verificações).
- Δ tests vs baseline P190C.
- Hashes finais.
- Decisões de execução notáveis:
  - Type signature `lang` (per `.A.1`).
  - Walk arm Outline state (per `.A.8`).
  - Padrão "eliminação directa via Introspector"
    aplicado a `has_outline`.
- Estado actual:
  - P190 série: A ✅ B ✅ C ✅ D ✅ | E-I pendentes.
  - **Categoria 3 (Document metadata) fechada**.
  - 88 passos executados.
- Pendências cumulativas: 5 categorias restantes
  + P190I.
- Próximo passo: P190E — categoria 4 (Numbering
  active). Magnitude M.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate
   substancial.
2. `LayouterRuntimeState` expandida (`.B`).
3. Consumers `is_readonly` migrados (`.C`).
4. Consumers `lang` migrados (`.D`).
5. Consumers `has_outline` migrados (`.E`).
6. 3 fields eliminados (`.F`).
7. Layouter assignments eliminados (`.G`).
8. Tests adaptados (`.H`).
9. Verificações `.I` passam (19/19).
10. `CounterStateLegacy`: 9 fields.
11. `LayouterRuntimeState`: 4 fields.
12. Output observable em produção inalterado.
13. Relatório `.J` escrito.

---

## O que pode sair errado

- **Type signature `lang` complexa** (envolvendo
  `Option`, `Lang` enum, etc.): cláusula gate
  trivial — copiar exacto.
- **`has_outline` ainda é populated por walk arm
  Outline com lógica não-trivial**: cláusula gate
  substancial — investigar empiricamente em `.A.8`.
- **API Introspector para `has_outline` não tem
  método dedicado**: cláusula gate trivial —
  usar `kind_index` directamente ou adicionar
  método trait.
- **Tests Layouter regridem por mudança path**:
  cláusula gate trivial.
- **`is_readonly` é mutado em sítios não previstos**
  (não Layouter): cláusula gate trivial — investigar.
- **Layouter `is_readonly` semantics são
  cross-cutting** (afecta múltiplos arms): cláusula
  gate trivial — replicar em todos os sítios.
- **Snapshot tests divergem**: improvável.
- **Linter divergência V13/V14**: cláusula gate
  trivial.

---

## Notas operacionais

- **Tamanho**: M (similar a P190C). ~40 LOC produção
  (struct expansion + consumer migrations + field
  eliminations) + ~10 LOC tests adaptados + ~20
  LOC L0.
- **Sem dependências externas novas**.
- **Sem ADR nova**.
- **2 padrões aplicados simultaneamente** —
  primeira ocorrência:
  - Layouter-runtime → struct dedicada (`is_readonly`,
    `lang`).
  - Eliminação directa via Introspector
    (`has_outline` per P178).
- **Cláusula gate trivial**: aplicável a forma
  exacta de types, recálculo de hashes, adaptação
  tests.
- **Cláusula gate substancial possível**: walk arm
  Outline state — se ainda tem mutação legacy não
  prevista.
- **Próximo passo P190E**: categoria 4 (Numbering
  active). Magnitude M. Trabalho concreto: campo
  `numbering_active` — caminho Introspector activo
  desde P198B + P199B (StateRegistry com chaves
  `numbering_active:heading` + `numbering_active:equation`).
  Eliminação directa via Introspector path (P190B
  pattern).
- **F1 progresso**: 12 → 9 fields ortogonais.
- **F3 progresso**: Layouter ainda 20 fields;
  delta 0 (struct `LayouterRuntimeState` apenas
  expandida; field `runtime` já existe).
