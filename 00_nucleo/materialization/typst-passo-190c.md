# Passo P190C — Categoria Page tracking + LayouterRuntimeState

Segundo passo de implementação P190 (após P190A
diagnóstico, P190B Bibliography). Magnitude **M
marginalmente maior que P190B** — primeira aplicação
da decisão "4 fields Layouter-runtime → struct
dedicada".

P190B confirmou empiricamente:
- Pattern "eliminação write paralelo M5" funciona —
  0 cláusulas gate disparadas.
- Layouter assignments duais — 2 contextos por
  categoria.
- Tests sentinela legacy redundantes removíveis ou
  adaptáveis.
- Δ tests pode ser negativo marginal.

P190C inicia categoria **Layouter-runtime** — campos
**não derivados de Content pre-pass**. Per P190A §3
achado crítico: 4 campos não cabem em Introspector;
movem para struct dedicada `LayouterRuntimeState` (ou
similar nome).

P190C trata 2 dos 4 campos Layouter-runtime:
- `label_pages: HashMap<Label, usize>` — mapeia label
  para número de página (escrita por
  `references.rs`).
- `known_page_numbers: HashMap<...>` — referência de
  páginas durante layout (lida por outline).

Trabalho concreto:
1. Criar struct `LayouterRuntimeState` em
   `01_core/src/entities/layouter_runtime_state.rs`
   (novo ficheiro).
2. Mover `label_pages` + `known_page_numbers` para
   essa struct.
3. Adicionar field
   `Layouter<M, S>::runtime: LayouterRuntimeState`
   (Layouter passa de 19 → 20 fields).
4. Migrar consumers Layouter de `self.counter.label_pages`
   para `self.runtime.label_pages`.
5. Migrar consumers Layouter de
   `self.counter.known_page_numbers` para
   `self.runtime.known_page_numbers`.
6. Eliminar fields `label_pages` + `known_page_numbers`
   de `CounterStateLegacy`.
7. Eliminar Layouter assignments duais
   correspondentes.
8. Adaptar tests dependentes (padrão pragmático
   auditor #1).

Após P190C:
- `CounterStateLegacy`: 14 → **12 fields**.
- `Layouter<M, S>`: 19 → **20 fields** (delta +1
  porque adiciona `runtime` field; mas
  conceptualmente reduz acoplamento).
- Struct nova `LayouterRuntimeState`: 2 fields.
- F3 progresso: Layouter ainda tem campo `counter`;
  redução total acontece em P190I.
- Pattern "eliminação write paralelo M5": 2ª
  aplicação concreta.
- **Padrão "Layouter-runtime → struct dedicada"**:
  1ª aplicação — replicada em P190D para outros 2
  fields Layouter-runtime (`is_readonly`, `lang`).

**Pré-condição**: P190B concluído. Tests workspace
1.867 verdes; zero violations. `CounterStateLegacy`
14 fields. ADR-0070 PROPOSTO ainda não transitada.

**Restrições**:
- **Não** modificar walk arm Heading, Figure, ou
  outros (estes não tocam `label_pages`/`known_page_numbers`).
- **Não** modificar `from_tags` (não popula estes
  fields).
- **Não** modificar trait `Introspector` (estes
  campos não cabem em Introspector — Layouter-runtime
  pre-pass).
- **Não** modificar `TagIntrospector`.
- **Não** eliminar struct `CounterStateLegacy` —
  P190I.
- **Não** modificar 4 helpers `compute_*`.
- **Não** materializar lacunas residuais.
- API pública preservada (eliminação interna).

---

## Sub-passos

### .A Auditoria L0

#### Inventário dos 2 fields

1. Confirmar fields em
   `01_core/src/entities/counter_state_legacy.rs`:
   - `pub label_pages: HashMap<Label, usize>` (ou
     similar).
   - `pub known_page_numbers: HashMap<...>` (ou
     similar).
   - Type signatures exactos.

2. Identificar walk arms ou `from_tags` arms que
   mutam estes fields:
   - **Esperado**: nenhum. São Layouter-runtime
     populated durante layout.
   - Confirmar empiricamente via `grep -n
     "label_pages\|known_page_numbers"
     01_core/src/rules/introspect/`.

#### Inventário Layouter consumers

3. Identificar consumers Layouter:
   - `grep -rn "self.counter.label_pages\|self.counter.known_page_numbers"
     01_core/src/`.
   - Identificar contexto (qual função; qual arm
     do `layout_content` match).

4. Identificar mutações Layouter próprias (Layouter
   muta os fields):
   - `grep -rn "counter.label_pages.insert\|counter.known_page_numbers.insert"
     01_core/src/`.
   - **Esperado**: pelo menos 1 mutação por field
     (escrita durante layout).

5. Identificar Layouter assignments duais (per P190B
   pattern):
   - `grep -rn "label_pages\s*=\|known_page_numbers\s*="
     01_core/src/rules/layout/mod.rs`.
   - Esperado: 4 linhas (2 fields × 2 contextos).

#### Tests dependentes

6. Identificar tests:
   - Tests que verificam `state.label_pages`
     directamente.
   - Tests que verificam `state.known_page_numbers`
     directamente.

#### Decisão de nomenclatura

7. **Decisão obrigatória**: nome da struct nova:
   - **Opção α**: `LayouterRuntimeState`.
   - **Opção β**: `LayoutState`.
   - **Opção γ**: `LayouterState`.
   - **Opção δ**: outro.

   Sugestão preliminar: **α** (`LayouterRuntimeState`)
   — clareza explícita sobre origem (Layouter-runtime,
   não derivada de Content).

#### L0 alvos

8. Identificar L0s:
   - `entities/counter_state_legacy.md` (fields
     eliminados — provavelmente diferido para P190I).
   - `entities/layouter_runtime_state.md` (novo).
   - `rules/layout/mod.md` (consumers migrados).

Output: tabela com item + estado verificado.

**Critério de saída**:
- 2 fields localizados.
- Layouter consumers + mutações + assignments
  identificados.
- Tests dependentes listados.
- Nome da struct nova decidido.

### .B Criar struct `LayouterRuntimeState`

1. Criar ficheiro
   `01_core/src/entities/layouter_runtime_state.rs`:
   ```
   //! P190C — Struct dedicada a state Layouter-runtime
   //! que NÃO é derivado de Content pre-pass.
   //! Diferente de TagIntrospector (state derivada
   //! de walk + from_tags), este struct é populated
   //! durante layout pelo Layouter.

   use std::collections::HashMap;
   use crate::entities::label::Label;

   #[derive(Debug, Default, Clone)]
   pub struct LayouterRuntimeState {
       /// Mapeia label para número de página.
       /// Populated por references.rs durante layout.
       pub label_pages: HashMap<Label, usize>,

       /// Page numbers conhecidos durante layout
       /// (referência usada por outline).
       /// Type signature exacto a confirmar em .A.
       pub known_page_numbers: HashMap<...>,
   }
   ```
   - Forma exacta dos types depende de `.A.1`.

2. Adicionar export em
   `01_core/src/entities/mod.rs`:
   ```
   pub use layouter_runtime_state::LayouterRuntimeState;
   ```

3. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- Struct nova criada.
- Export funcional.
- `cargo check --workspace` passa.

### .C Adicionar field `runtime` ao Layouter

1. Em
   `01_core/src/rules/layout/mod.rs` (ou onde
   Layouter está definido):
   - Adicionar field:
     ```
     pub runtime: LayouterRuntimeState,
     ```
   - Inicializar em `new()` ou construtor:
     ```
     runtime: LayouterRuntimeState::default(),
     ```

2. **Layouter passa de 19 → 20 fields** (mas reduz
   acoplamento conceptual — F3 mantém-se até P190I).

3. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- Field adicionado.
- Layouter inicialização adaptada.
- `cargo check --workspace` passa.

### .D Migrar consumers Layouter (`label_pages`)

1. Per `.A.3` e `.A.4`:
   - Substituir `self.counter.label_pages.X` por
     `self.runtime.label_pages.X` em todos os sítios.
   - Inclui mutações Layouter próprias
     (`self.counter.label_pages.insert(...)` →
     `self.runtime.label_pages.insert(...)`).

2. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- Consumers `label_pages` migrados.
- `cargo check --workspace` passa.

### .E Migrar consumers Layouter (`known_page_numbers`)

1. Análogo a `.D`:
   - Substituir `self.counter.known_page_numbers.X`
     por `self.runtime.known_page_numbers.X`.

2. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- Consumers `known_page_numbers` migrados.

### .F Eliminar fields de `CounterStateLegacy`

1. Em
   `01_core/src/entities/counter_state_legacy.rs`:
   - Eliminar field `pub label_pages`.
   - Eliminar field `pub known_page_numbers`.
   - Adaptar `Default` impl ou `new()` constructor.

2. Confirmar `cargo check --workspace` passa.

3. **`CounterStateLegacy`: 14 → 12 fields**.

**Critério de saída**:
- 2 fields eliminados.
- `cargo check --workspace` passa.

### .G Eliminar Layouter assignments duais

1. Per `.A.5`:
   - Eliminar 4 linhas de assignment (provavelmente
     em `mod.rs:1496-1498` e `mod.rs:1524-1526`
     — 2 fields × 2 contextos).
   - Comentário inline P190C substitui ou
     actualiza comentário inline P190B.

2. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- 4 linhas eliminadas.
- Comentários inline actualizados.

### .H Adaptar tests

1. Identificar tests afectados (per `.A.6`):
   - Tests que verificavam `state.label_pages` /
     `state.known_page_numbers` directamente.
   - Tests Layouter que dependiam de
     `layouter.counter.label_pages` (agora
     `layouter.runtime.label_pages`).

2. Adaptação:
   - Tests redundantes — remover.
   - Tests adaptáveis — substituir
     `layouter.counter.X` por `layouter.runtime.X`.

3. Tests workspace verdes (Δ esperado: 0 ou marginal
   negativo).

**Critério de saída**:
- Tests adaptados.
- Tests workspace verdes.

### .I Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P190B
   baseline (1.867): **0 ou marginal negativo**.
3. `crystalline-lint .` zero violations (após
   `--fix-hashes`).
4. **Struct nova `LayouterRuntimeState`** existe.
5. Layouter tem field `runtime: LayouterRuntimeState`.
6. `CounterStateLegacy.label_pages` **NÃO existe**.
7. `CounterStateLegacy.known_page_numbers` **NÃO
   existe**.
8. `CounterStateLegacy`: 12 fields (era 14).
9. Layouter consumers `label_pages` migrados.
10. Layouter consumers `known_page_numbers` migrados.
11. Layouter assignments duais eliminados (4 linhas).
12. Comentários inline P190C presentes.
13. Trait `Introspector` **NÃO modificado**.
14. `TagIntrospector` **NÃO modificado**.
15. Walk arms **NÃO modificados** (estes campos não
    são tocados por walk).
16. ADR-0070 PROPOSTO **NÃO transitada** (ACEITE em
    P190I).
17. Snapshot tests verdes.
18. Linter passa final.

### .J Encerramento

Escrever
`00_nucleo/materialization/typst-passo-190c-relatorio.md`
com:

- Resumo: categoria 2 (Page tracking) eliminada;
  primeira aplicação "Layouter-runtime → struct
  dedicada"; struct `LayouterRuntimeState` criada;
  `CounterStateLegacy` reduzido a 12 fields.
- Confirmação `.I` (18 verificações).
- Δ tests vs baseline P190B.
- Hashes finais.
- Decisões de execução notáveis:
  - Nome da struct nova (per `.A.7`).
  - Type signatures exactos (per `.A.1`).
  - Padrão arquitectural "Layouter-runtime"
    estabelecido.
- Estado actual:
  - P190 série: A ✅ B ✅ C ✅ | D-I pendentes.
  - **Categoria 2 (Page tracking) fechada**.
  - **Padrão Layouter-runtime estabelecido** —
    replicado em P190D para `is_readonly` + `lang`.
  - 87 passos executados.
- Pendências cumulativas: 6 categorias restantes
  (3, 4, 5, 6, 7) + P190I.
- Próximo passo: P190D — categoria 3 (Document
  metadata: `is_readonly` + `lang` + `has_outline`).

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate
   substancial.
2. Struct `LayouterRuntimeState` criada (`.B`).
3. Field `runtime` adicionado ao Layouter (`.C`).
4. Consumers `label_pages` migrados (`.D`).
5. Consumers `known_page_numbers` migrados (`.E`).
6. Fields eliminados de `CounterStateLegacy` (`.F`).
7. Layouter assignments duais eliminados (`.G`).
8. Tests adaptados (`.H`).
9. Verificações `.I` passam (18/18).
10. `CounterStateLegacy`: 12 fields.
11. **Padrão "Layouter-runtime → struct dedicada"
    estabelecido**.
12. Output observable em produção inalterado.
13. Relatório `.J` escrito.

---

## O que pode sair errado

- **Type signature `known_page_numbers` complexo**
  (envolvendo refs ou lifetimes): cláusula gate
  trivial — copiar exacto.
- **Layouter consumers em sítios não previstos**
  (mais que 2-3 ocorrências esperadas): cláusula
  gate trivial — migrar todos.
- **Layouter mutações de `label_pages` em
  references.rs** (não em mod.rs): cláusula gate
  trivial — migrar.
- **Tests Layouter regridem por mudança de path
  (`.counter.X` → `.runtime.X`)**: cláusula gate
  trivial — adaptar.
- **`Layouter::new()` exige novo parâmetro de
  construção para `runtime`**: improvável (pode usar
  `Default`); cláusula gate trivial.
- **Snapshot tests divergem** apesar de não tocar
  semântica: improvável — investigar se acontecer.
- **`Layouter<M, S>::clone()` ou similar precisa
  de adaptação**: cláusula gate trivial.
- **Linter divergência V13/V14**: cláusula gate
  trivial.

---

## Notas operacionais

- **Tamanho**: M (marginalmente maior que P190B
  porque cria struct nova + adiciona field Layouter).
  ~50 LOC produção (struct + Layouter field +
  consumer migrations + field eliminations) + ~20
  LOC tests adaptados + ~20 LOC L0.
- **Sem dependências externas novas**.
- **Sem ADR nova** (ADR-0070 PROPOSTO já criada em
  P190A; ACEITE em P190I).
- **Padrão arquitectural estabelecido**:
  "Layouter-runtime → struct dedicada" — replicado
  em P190D (`is_readonly` + `lang`) e
  potencialmente P190G/H se necessário.
- **Cláusula gate trivial**: aplicável a forma
  exacta de types, recálculo de hashes, adaptação
  tests.
- **Cláusula gate substancial**: não esperada
  (campos Layouter-runtime sem dependências
  cruzadas com walk arms).
- **Próximo passo P190D**: categoria 3 (Document
  metadata). Aplicação do padrão
  "Layouter-runtime → struct dedicada" para
  `is_readonly` + `lang` (`has_outline` é caso
  diferente — já tem `kind_index[Outline]` per P178;
  pode eliminar directamente sem mover para
  LayouterRuntimeState).
- **F1 progresso**: 14 → 12 fields ortogonais.
- **F3 progresso**: Layouter ainda tem `counter`
  field; mas conceptualmente F3 melhora — Layouter
  agora tem 2 categorias claras de state
  (Introspector-derived via `counter` em redução; e
  Layouter-runtime via `runtime`). F3 fecha em P190I.
