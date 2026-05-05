# Passo P190E — Categoria Numbering active (eliminação directa via Introspector)

Quarto passo de implementação P190 (após P190A, B,
C, D). Magnitude **M** — aplicação do padrão
"eliminação directa via Introspector" (P190B style).

P190D consolidou:
- Defer `lang` documentado (lido por
  `compute_labelled` walk fn).
- Guard `is_readonly` refactor para Layouter level.
- `CounterStateLegacy`: 10 fields.
- 25ª aplicação consecutiva diagnóstico-primeiro.

P190E elimina **`numbering_active: HashMap<String,
bool>`** — caminho Introspector activo desde P198B
+ P199B (StateRegistry com chaves
`numbering_active:heading` + `numbering_active:equation`).

Trabalho concreto:
1. Migrar Layouter consumers de
   `self.counter.is_numbering_active(key)` para
   `self.introspector.is_numbering_active_at(key,
   location)` ou similar.
2. Mutação walk arm `state.numbering_active.insert(...)`
   em SetHeadingNumbering (P198B) e
   SetEquationNumbering (P199B): **decidir
   empiricamente**:
   - Se walk arm Equation gate em
     `is_numbering_active("equation")` ainda lê
     legacy: preservar mutação como write paralelo
     M5; eliminar em P190F (helper migration).
   - Se já lê Introspector: eliminar mutação aqui.
3. Eliminar field `numbering_active` de
   `CounterStateLegacy`.
4. Eliminar Layouter assignments duais.
5. Adaptar tests dependentes.

Após P190E:
- `CounterStateLegacy`: 10 → **9 fields**.
- Layouter consumers `numbering_active` migrados
  para Introspector path.
- Pattern "eliminação write paralelo M5": 4ª
  aplicação concreta.
- Padrão "eliminação directa via Introspector": 3ª
  aplicação (após P190B Bibliography + P190D
  has_outline).

**Pré-condição**: P190D concluído. Tests workspace
1.862 verdes; zero violations. `CounterStateLegacy`
10 fields. `LayouterRuntimeState` 3 fields. `lang`
deferido (presente em CounterStateLegacy).

**Restrições**:
- **Não** modificar walk arms outros (Heading,
  Figure) — não tocam `numbering_active`.
- **Não** modificar walk arms SetHeadingNumbering
  ou SetEquationNumbering **se mutação é write
  paralelo necessário** (verificar `.A`).
- **Não** modificar `from_tags` arm StateUpdate
  (P171 genérica funcional).
- **Não** modificar trait `Introspector` —
  `is_numbering_active_at` activo desde P185B.
- **Não** modificar `TagIntrospector`.
- **Não** eliminar struct `CounterStateLegacy` —
  P190I.
- **Não** modificar 4 helpers `compute_*` —
  potenciais leitores de `state.is_numbering_active`
  ficam para P190F.
- **Não** materializar lacunas residuais.

---

## Sub-passos

### .A Auditoria L0

#### Inventário field e mutações

1. Confirmar field em
   `01_core/src/entities/counter_state_legacy.rs`:
   - `pub numbering_active: HashMap<String, bool>`.
   - Type signature exacto.

2. Identificar walk arms que mutam:
   - SetHeadingNumbering (P198B):
     `state.numbering_active.insert("heading",
     active)`.
   - SetEquationNumbering (P199B):
     `state.numbering_active.insert("equation",
     active)`.
   - **Decisão obrigatória `.A.2`**: estas mutações
     são write paralelo M5 obrigatório (lidas por
     algum consumer durante walk)?

3. Identificar leitores DURANTE WALK:
   - `grep -n "state.is_numbering_active\|state.numbering_active\." 01_core/src/rules/introspect.rs`.
   - Esperados (per achados anteriores):
     - Walk arm Equation gate em
       `state.is_numbering_active("equation")`
       (P199B comentário inline).
     - Possivelmente `compute_labelled` ou
       `compute_heading_auto_toc`.
   - Se há leitores walk: write paralelo é
     **obrigatório** preservar até helpers migrarem
     (P190F).

#### Inventário Layouter consumers

4. Identificar consumers Layouter:
   - `grep -rn "self.counter.is_numbering_active\|self.counter.numbering_active"
     01_core/src/`.
   - Ocorrências esperadas (per P190A §6):
     `equation.rs:33`, `mod.rs:343`.

5. Identificar API Introspector equivalente:
   - `is_numbering_active_at(key, location) -> bool`
     (P185B activo).
   - Confirmar assinatura empírica.

6. Decisão de location:
   - Cada consumer Layouter precisa de Location para
     query.
   - **Cláusula gate substancial**: se consumer
     não tem Location no contexto, migração é mais
     complexa. Investigar empiricamente.

7. Identificar Layouter assignments duais:
   - `grep -rn "numbering_active\s*=" 01_core/src/rules/layout/mod.rs`.

#### Tests dependentes

8. Identificar tests:
   - Tests que verificam `state.is_numbering_active`
     directamente.
   - Tests Layouter via `.counter.is_numbering_active`.

#### L0 alvos

9. Identificar L0s:
   - `entities/counter_state_legacy.md` (field
     eliminado — diferido para P190I).
   - Possivelmente `rules/layout/mod.md`.

Output: tabela com item + estado verificado.

**Critério de saída**:
- Field localizado.
- Mutações walk arm catalogadas.
- Leitores durante walk identificados.
- Layouter consumers + assignments identificados.
- Decisão `.A.2` materializada (preservar mutação
  ou eliminar agora).
- Decisão sobre Location nos consumers.

### .B Migrar consumer 1 (`equation.rs:33`)

1. Per `.A.4` e `.A.5`:
   - Substituir `self.counter.is_numbering_active("equation")`
     por
     `self.introspector.is_numbering_active_at("equation",
     location)` (forma exacta per `.A.5`).

2. Confirmar `cargo check --workspace` passa.

3. **Cláusula gate trivial**: se `equation.rs:33`
   não tem Location no contexto, usar
   `self.introspector.is_numbering_active(key)`
   sem location (snapshot final), ou injectar
   Location no contexto.

**Critério de saída**:
- Consumer 1 migrado.

### .C Migrar consumer 2 (`mod.rs:343`)

1. Análogo a `.B`:
   - Substituir consumer em `mod.rs:343`.

2. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- Consumer 2 migrado.

### .D Decisão sobre mutação walk arm

Per `.A.2` e `.A.3`:

**Caso 1**: leitores walk existem (write paralelo
obrigatório).
- Preservar mutações em SetHeadingNumbering +
  SetEquationNumbering.
- Adicionar comentário inline P190E documentando
  preservação.
- Eliminação completa diferida para P190F (helpers
  migration).

**Caso 2**: nenhum leitor walk lê
`state.numbering_active`.
- Eliminar mutações em SetHeadingNumbering +
  SetEquationNumbering.
- Walk arms ficam puros para esta categoria.

Output: decisão materializada.

### .E Eliminar field `numbering_active`

Per `.A.1`:

1. Em
   `01_core/src/entities/counter_state_legacy.rs`:
   - Eliminar field `pub numbering_active:
     HashMap<String, bool>`.
   - Adaptar `Default` impl.

2. **Decisão crítica**: se `.D` decidiu Caso 1
   (preservar mutações), **NÃO ELIMINAR FIELD
   AINDA** — mutações precisam do field. Defer
   eliminação para P190F.

3. **Decisão crítica**: se `.D` decidiu Caso 2,
   eliminar field aqui.

4. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- Field eliminado se Caso 2; deferido se Caso 1.
- `cargo check --workspace` passa.

### .F Eliminar Layouter assignments duais

Per `.A.7`:

1. Eliminar linhas de assignment correspondentes.

2. Comentário inline P190E.

3. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- Assignments eliminados.

### .G Adaptar tests

1. Identificar tests afectados (per `.A.8`):
   - Tests sentinela `state.is_numbering_active`
     redundantes.
   - Tests Layouter via `.counter.X`.

2. Adaptação:
   - Tests redundantes — remover.
   - Tests adaptáveis — substituir.

3. Tests workspace verdes (Δ esperado: 0 ou marginal
   negativo).

**Critério de saída**:
- Tests adaptados.

### .H Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P190D
   baseline (1.862): **0 ou marginal negativo**.
3. `crystalline-lint .` zero violations (após
   `--fix-hashes`).
4. Layouter consumers `numbering_active` migrados.
5. **Per Caso 1 (`.D`)**:
   - Field `numbering_active` ainda existe em
     `CounterStateLegacy` (defer P190F).
   - `CounterStateLegacy`: 10 fields (inalterado em
     P190E para esta categoria).
   - Mutações walk arm preservadas.
6. **Per Caso 2 (`.D`)**:
   - `CounterStateLegacy.numbering_active` **NÃO
     existe**.
   - `CounterStateLegacy`: 10 → 9 fields.
   - Walk arm SetHeadingNumbering puro.
   - Walk arm SetEquationNumbering puro.
7. Layouter assignments duais eliminados.
8. Comentários inline P190E presentes.
9. Trait `Introspector` **NÃO modificado**.
10. `TagIntrospector` **NÃO modificado**.
11. ADR-0070 PROPOSTO **NÃO transitada**.
12. Snapshot tests verdes.
13. Linter passa final.

### .I Encerramento

Escrever
`00_nucleo/materialization/typst-passo-190e-relatorio.md`
com:

- Resumo: categoria 4 (Numbering active) eliminada
  ou parcialmente eliminada per Caso (`.D`).
- Confirmação `.H` (13 verificações).
- Δ tests vs baseline P190D.
- Hashes finais.
- Decisões de execução notáveis:
  - Caso 1 vs Caso 2 (`.D`).
  - Localização de Location nos consumers
    Layouter (per `.A.6`).
- Estado actual:
  - P190 série: A ✅ B ✅ C ✅ D ✅ E ✅ | F-I
    pendentes.
  - **Categoria 4 (Numbering active)** fechada ou
    parcialmente fechada.
  - 89 passos executados.
- Pendências cumulativas: 4 categorias restantes
  + P190I + `lang` defer + (eventual) defer
  `numbering_active` se Caso 1.
- Próximo passo: P190F — categoria 5 (Counters core
  + 2 helpers migrados). Magnitude M+ —
  inclui migração de `compute_labelled` +
  `compute_heading_auto_toc` para Introspector path
  location-aware. Resolução defer `lang` +
  potencialmente defer `numbering_active`.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate
   substancial inesperada.
2. Consumer 1 migrado (`.B`).
3. Consumer 2 migrado (`.C`).
4. Decisão Caso 1/Caso 2 sobre mutação walk arm
   (`.D`).
5. Field eliminado per Caso 2 ou deferido per Caso
   1 (`.E`).
6. Layouter assignments eliminados (`.F`).
7. Tests adaptados (`.G`).
8. Verificações `.H` passam (13/13).
9. Output observable em produção inalterado.
10. Relatório `.I` escrito.

---

## O que pode sair errado

- **Layouter consumers não têm Location no
  contexto**: cláusula gate substancial — investigar
  como obter Location ou usar API sem location.
- **Walk arm Equation lê `state.is_numbering_active`
  obrigatoriamente**: cláusula gate substancial —
  Caso 1 obrigatório; eliminação parcial.
- **Helpers `compute_*` lêem `state.numbering_active`
  durante walk**: cláusula gate substancial —
  Caso 1 obrigatório.
- **API `is_numbering_active_at` tem assinatura
  diferente do esperado**: cláusula gate trivial —
  ajustar consumer migration.
- **Tests Layouter regridem por mudança de path**:
  cláusula gate trivial.
- **Snapshot tests divergem**: improvável (caminho
  Introspector estável desde P198B/P199B); investigar
  se acontecer.
- **Linter divergência V13/V14**: cláusula gate
  trivial.

---

## Notas operacionais

- **Tamanho**: M. ~25-50 LOC produção (depende de
  Caso 1 vs Caso 2) + ~20 LOC tests adaptados.
- **Sem dependências externas novas**.
- **Sem ADR nova**.
- **Pattern "eliminação directa via Introspector"**:
  3ª aplicação. Caminho Introspector estável desde
  P198B + P199B (>3 séries — confiança alta).
- **Cláusula gate trivial**: aplicável a forma
  exacta de assinatura, recálculo de hashes,
  adaptação tests.
- **Cláusula gate substancial possível**: leitores
  walk de `state.numbering_active` (Caso 1
  obrigatório).
- **Próximo passo P190F**: categoria 5 (Counters
  core + 2 helpers migrados). Magnitude M+.
  Trabalho concreto:
  - Migrar 2 helpers (`compute_labelled`,
    `compute_heading_auto_toc`) para Introspector
    path location-aware.
  - Eliminar 2 helpers walk-internal
    (`compute_figure`, `compute_heading_for_toc`)
    se possível ou diferir para P190I.
  - Eliminar campos counters core (`flat`,
    `hierarchical`).
  - Resolver `lang` defer (helper migration).
  - Possivelmente resolver `numbering_active` defer
    se Caso 1 em P190E.
- **F1 progresso**: 10 → 9 (Caso 2) ou 10 (Caso 1
  defer) fields ortogonais.
- **F3 progresso**: Layouter ainda 20 fields
  (inalterado em P190E).
