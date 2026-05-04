# Passo P193B — Abrir `ResolvedLabelStore`

Único passo de implementação P193 (após P193A diagnóstico).
Magnitude **S** agregada — passo único combinando struct
nova, L0, integração no `TagIntrospector`, método trait,
tests unit, e relatório consolidado.

**Passo 1 da sequência §9 do P189 consolidado** —
infraestrutura para fechar 5 das 6 excepções M5 (E2-E6).

Cria `01_core/src/entities/resolved_label_store.rs` com
struct paralela a `BibStore` (P181B):

```rust
pub struct ResolvedLabelStore {
    labels: HashMap<Label, String>,
}
```

Adiciona field `pub resolved_labels: ResolvedLabelStore`
ao `TagIntrospector` (paralelo a `bib_store`,
`kind_index`, etc.) e método trait
`resolved_label_for(&self, label: &Label) -> Option<&str>`
que delega a `resolved_labels.get(label)`.

Após P193B:
- Sub-store `ResolvedLabelStore` existe estruturalmente.
- Trait `Introspector` tem método novo (19 métodos).
- `TagIntrospector` tem 7 → **8** sub-stores.
- **Sub-store fica vazio em produção** até P195 adicionar
  arm de populate em `from_tags`.
- Walks (E2/E4) continuam a mutar
  `state.resolved_labels` legacy directamente.
- Consumer C4 (`references.rs:53`) continua a ler legacy.
- DEBT M5-residual: 4 → 3 pré-requisitos pendentes.

**Pré-condição**: P193A concluído. Tests workspace 1.815
verdes; zero violations. 8 cláusulas P193A fechadas.

**Restrições**:
- **Não** modificar walk arms — P195+.
- **Não** modificar consumer C4 — P194.
- **Não** modificar `from_tags` — P195.
- **Não** popular sub-store via Tag — P195.
- **Não** abrir DEBT M5-residual formal (Cenário B
  continua).
- API pública preservada — adição de field e método é
  retrocompatível.
- Output observable em produção **inalterado** —
  sub-store fica vazio; ninguém o consulta ainda.

---

## Sub-passos

### .A Auditoria L0

1. Confirmar `state.resolved_labels` actual:
   - `01_core/src/entities/counter_state_legacy.rs:37`
     (per P193A §2.1).
   - Tipo: `HashMap<Label, String>`.
   - Re-verificar empiricamente.

2. Confirmar `Label` newtype:
   - `01_core/src/entities/label.rs:12` (per P193A §2.2).
   - `pub struct Label(pub String)`.
   - Confirmar `Eq`, `Hash`, `Clone`, `Debug` derives
     (necessários para `HashMap` key).

3. Confirmar `TagIntrospector` actual:
   - `01_core/src/entities/introspector.rs` ou similar.
   - Localizar struct + sub-stores existentes.
   - Identificar onde adicionar `resolved_labels` field
     (paralelo a `bib_store`).

4. Confirmar trait `Introspector` actual:
   - 18 métodos existentes per P185-consolidado.
   - Localizar onde adicionar `resolved_label_for`
     (ordem cronológica per P185B convenção).

5. Confirmar `BibStore` como template:
   - `01_core/src/entities/bib_store.rs` (per P181B).
   - Struct, métodos `empty`, `insert(pub(crate))`,
     `get`, `len`, `is_empty`.
   - Replicar shape para `ResolvedLabelStore`.

6. Confirmar L0 `entities/bib_store.md` como template:
   - Para criar L0 `entities/resolved_label_store.md`
     paralelamente.

7. Confirmar `Layouter` copia `state.resolved_labels`:
   - `mod.rs:1481, 1512` (per P193A §2.5).
   - **Não tocar** — copy-sites continuam funcionais
     durante janela compat. P194 substitui.

Output: tabela com item + estado + linhas exactas para
edits.

**Critério de saída**:
- Tipo de `state.resolved_labels` confirmado.
- `Label` derives confirmados.
- `TagIntrospector` localizado.
- Template BibStore localizado.
- Copy-sites identificados (não tocar).

### .B Criar struct `ResolvedLabelStore`

1. Criar `01_core/src/entities/resolved_label_store.rs`:
   - Struct `ResolvedLabelStore { labels: HashMap<Label, String> }`.
   - Visibilidade do field `labels`: **`pub(crate)`** ou
     **privado** (decisão depende do template BibStore;
     replicar literalmente).
   - Métodos:
     - `pub fn empty() -> Self`.
     - `pub(crate) fn insert(&mut self, label: Label,
       resolved: String)`.
     - `pub fn get(&self, label: &Label) -> Option<&str>`.
     - `pub fn len(&self) -> usize`.
     - `pub fn is_empty(&self) -> bool`.
   - Derives: `Debug, Default, Clone` (replicar
     BibStore).

2. Adicionar à árvore de módulos:
   - `01_core/src/entities/mod.rs` ou similar.
   - `pub mod resolved_label_store;` ou similar.
   - `pub use resolved_label_store::ResolvedLabelStore;`
     se convenção do projecto exigir re-export.

3. Confirmar `@prompt-hash` adicionado.

**Critério de saída**:
- Ficheiro criado.
- Struct declarável e construível.
- `cargo check --workspace` passa.

### .C Criar L0 `entities/resolved_label_store.md`

1. Criar `00_nucleo/prompts/entities/resolved_label_store.md`
   replicando shape de `entities/bib_store.md`:
   - Secções padrão L0 (14 secções típicas).
   - Documentação de struct + métodos.
   - Cross-reference a P193A diagnóstico.
   - Nota explícita: **sub-store é vazio em produção
     durante janela compat M5; populate via Tag activa em
     P195**.

2. Hash em branco aguarda recálculo manual em `.G`.

**Critério de saída**:
- L0 existe com shape paralela a BibStore L0.

### .D Adicionar field a `TagIntrospector`

1. Em `01_core/src/entities/introspector.rs` (ou similar):
   - Adicionar field `pub resolved_labels: ResolvedLabelStore`
     ao struct `TagIntrospector`.
   - Posicionar conforme convenção (paralelo a
     `bib_store`).

2. Actualizar construtor `TagIntrospector::empty()` (ou
   equivalente) para inicializar
   `resolved_labels: ResolvedLabelStore::empty()`.

3. Confirmar `@prompt-hash` actualiza após edit do L0
   (em `.F`).

**Critério de saída**:
- Field adicionado.
- Construtor inicializa correctamente.
- `cargo check --workspace` passa.

### .E Adicionar método trait `resolved_label_for`

1. Em trait `Introspector` (em
   `01_core/src/entities/introspector.rs` ou similar):
   - Adicionar método
     ```
     fn resolved_label_for(&self, label: &Label) -> Option<&str>;
     ```
   - Posicionar após `figure_number_at_index` (P184C) ou
     conforme convenção empírica em `.A.4`.

2. Implementação para `TagIntrospector`:
   - `self.resolved_labels.get(label)`.

3. Confirmar `@prompt-hash` actualiza após edit do L0.

**Critério de saída**:
- Método trait declarado.
- Implementação delegada.
- Trait passa a ter 19 métodos.
- `cargo check --workspace` passa.

### .F Actualizar L0 `entities/introspector.md`

1. Editar L0 `00_nucleo/prompts/entities/introspector.md`:
   - Adicionar entrada para field `resolved_labels` em
     `TagIntrospector`.
   - Adicionar entrada para método trait
     `resolved_label_for`.
   - Adicionar entrada em Histórico de Revisões
     mencionando P193B.

2. Hash em branco aguarda recálculo manual em `.G`.

**Critério de saída**:
- L0 reflecte struct e trait actualizados.

### .G Tests unit

4 tests obrigatórios (per P193A §13.5):

#### Test 1 — `empty_store_returns_none`

1. `let store = ResolvedLabelStore::empty()`.
2. Assert `store.get(&Label("foo".into())) == None`.
3. Assert `store.is_empty() == true`.
4. Assert `store.len() == 0`.

#### Test 2 — `insert_then_get`

1. `let mut store = ResolvedLabelStore::empty()`.
2. `store.insert(Label("intro".into()), "Capítulo 1".into())`.
3. Assert `store.get(&Label("intro".into())) == Some("Capítulo 1")`.
4. Assert `store.is_empty() == false`.
5. Assert `store.len() == 1`.

#### Test 3 — `multiple_labels_isolated`

1. Setup: 3 inserts com labels distintos.
2. Asserções:
   - Cada label retorna o seu valor único.
   - Lookup de label não inserida retorna `None`.
   - `len() == 3`.

#### Test 4 — `trait_method_delegates`

1. `let mut intr = TagIntrospector::empty()`.
2. `intr.resolved_labels.insert(Label("ref1".into()),
   "Secção 1".into())`.
3. Assert
   `<TagIntrospector as Introspector>::resolved_label_for(&intr,
   &Label("ref1".into())) == Some("Secção 1")`.
4. Assert `intr.resolved_label_for(&Label("nope".into())) == None`.

Tests co-localizados em `mod tests` de
`resolved_label_store.rs` (tests 1-3) + tests do trait
em `introspector.rs:mod tests` (test 4).

**Critério de saída**:
- 4 tests passam.
- Tests existentes não regridem.

### .H Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P193A
   baseline (1.815): **+4**.
3. `crystalline-lint .` zero violations (após
   `--fix-hashes`).
4. Tests `resolved_label_store::*` e
   `introspector::tests::trait_method_delegates`
   passam isoladamente.
5. Struct `ResolvedLabelStore` declarável e construível.
6. Field `resolved_labels` em `TagIntrospector`
   inicializa via `empty()`.
7. Método `resolved_label_for` no trait `Introspector`
   delega correctamente.
8. Sub-store **vazio em produção** — `from_tags` arms
   intactos (P195 adiciona populate).
9. Walks legacy (E2/E4) **NÃO modificados** —
   continuam a popular `state.resolved_labels`.
10. Consumer C4 (`references.rs:53`) **NÃO modificado**.
11. Copy-sites Layouter (`mod.rs:1481, 1512`) **NÃO
    modificados**.
12. Tests existentes não regridem.
13. Snapshot tests ADR-0033 verdes.
14. Linter passa final.

### .I Actualizar nota DEBT M5-residual

P193 avança 1 dos 4 pré-requisitos para fechar M5
universalmente.

1. **Não editar** relatórios anteriores (preservação
   histórica per P187B/P188B/P189B).

2. Adicionar nota nova no relatório consolidado P193
   (`.J`) que actualiza estado:
   - **Antes P193**: 4 pré-requisitos pendentes
     (sub-store `resolved_labels`, C4 migration,
     sub-store `headings_for_toc`, `SetEquationNumbering`).
   - **Após P193B**: **3 pré-requisitos** restantes.
     `resolved_labels` aberto.
   - Cadeia E2-E6 fica num passo mais perto de
     desbloqueio.

**Critério de saída**:
- Nota actualizada no relatório consolidado P193.

### .J Escrever relatório consolidado P193

1. Criar
   `00_nucleo/materialization/typst-passo-193-relatorio-consolidado.md`
   com 9 secções (padrão P181J / P184F / P185 / P186 /
   P187 / P188 / P189):

   - §1 Resumo executivo + sub-store aberto +
     1º passo da sequência §9 P189.
   - §2 Sub-passos materializados (tabela métricas A–J
     dentro de P193B único).
   - §3 Decisões arquitecturais (8 cláusulas P193A
     fechadas).
   - §4 Achados não-triviais durante execução:
     - P193A §11.1 — decisão pré-existente preservada
       (HashMap vs Locator/Resolved vanilla).
     - P193A §11.2 — auto-toc + explicit unificados em
       mesmo HashMap.
     - P193A §11.3 — consumer C4 simples (fallback
       trivial em P194).
     - P193A §11.4 — sem variante location-aware.
     - P193A §11.5 — sub-store vazio em produção
       (janela compat).
   - §5 Estado final M9 (inalterado 11/11) e M5
     (1 arm migrado + 6 excepções; 3 pré-requisitos
     restantes em vez de 4).
   - §6 Estado final lacunas (#3 inalterada; ainda activa).
   - §7 Pendências cumulativas + nota DEBT M5-residual
     (4 → 3 pré-requisitos).
   - §8 Próximos passos sugeridos:
     - **P194 (C4 migration)** — agora desbloqueado;
       blueprint trivial.
     - P195 (migrar walk Labelled) — ainda depende de
       trabalho em sequência.
     - Restantes passos da sequência P189 §9.
   - §9 Conclusão.

2. Sem L0 novo (L0s `resolved_label_store.md` +
   `introspector.md` editados são parte de P193B `.C`
   e `.F`).

**Critério de saída**:
- Relatório consolidado existe.
- 9 secções presentes.
- Nota DEBT M5-residual actualizada.

### .K Encerramento

P193B é o passo único de implementação. Após `.J`
concluído, série P193 está fechada.

Estado projectado pós-P193B:

- **P193 série**: A ✅ B ✅. Fechada.
- **`TagIntrospector` sub-stores**: 7 → **8**.
- **Trait `Introspector`**: 18 → **19 métodos**.
- **M5 progresso**: 1 arm migrado + 6 excepções
  (inalterado); **1 dos 4 pré-requisitos avançado**.
- **DEBT M5-residual**: 4 → 3 pré-requisitos pendentes.
- **M9**: 11/11 (inalterado).
- **M5/M4 progresso (read-sites)**: 8/12 (inalterado —
  P194 mexe).
- **Tests workspace**: 1.815 → **1.819** (+4).
- **62 passos executados** (per P193A §12: P193A = 61 +
  P193B = 62).
- **Padrão diagnóstico-primeiro**: 15ª aplicação
  consecutiva (15/15 acertaram a magnitude planeada
  ±1 nível).
- **Próximo: P194 (C4 migration)** — desbloqueado.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate substancial.
2. Struct `ResolvedLabelStore` criada (`.B`).
3. L0 `entities/resolved_label_store.md` criada (`.C`).
4. Field `resolved_labels` em `TagIntrospector` (`.D`).
5. Método trait `resolved_label_for` (`.E`).
6. L0 `entities/introspector.md` actualizada (`.F`).
7. 4 tests unit passam (`.G`).
8. Verificações `.H` passam (14/14).
9. Nota DEBT M5-residual actualizada (`.I`).
10. Relatório consolidado P193 (9 secções) escrito (`.J`).
11. Output observable em produção **inalterado** —
    sub-store vazio.
12. Walks legacy E2/E4 **NÃO modificados**.

---

## O que pode sair errado

- **`Label` não tem `Hash` ou `Eq` derivados**: cláusula
  gate trivial — adicionar derives ou usar `String` como
  chave.
- **Convenção de naming difere** (`ResolvedLabels` vs
  `ResolvedLabelStore` vs `LabelRegistry`): cláusula
  gate trivial — usar nome consistente com BibStore
  (provável `ResolvedLabelStore`).
- **`TagIntrospector::empty()` não existe** (construtor
  diferente): cláusula gate trivial — adaptar.
- **Re-export em `entities/mod.rs` exigido**: cláusula
  gate trivial — adicionar.
- **L0 `entities/bib_store.md` tem shape diferente do
  esperado**: cláusula gate trivial — replicar shape
  empírica.
- **Trait `Introspector` definido em ficheiro diferente
  de `TagIntrospector`**: cláusula gate trivial —
  ajustar.
- **Test 4 (trait method delegates) falha por
  ambiguidade no método call**: cláusula gate trivial —
  usar fully-qualified path.
- **Snapshot tests divergem**: improvável (output
  preservado por construção). Se acontecer, investigar.
- **Linter divergência V13/V14**: cláusula gate trivial.

---

## Notas operacionais

- **Tamanho**: S agregado. ~50 LOC produção (struct +
  field + método) + ~50 LOC tests + ~80 LOC L0 nova +
  edits L0 introspector + relatório consolidado.
- **Sem dependências externas novas**.
- **Sem ADR; sem DEBT formal**.
- **Padrão replicado**: P181B BibStore literal + P181F
  método trait literal.
- **Cláusula gate trivial**: aplicável a derives,
  naming, re-exports, paths.
- **Sem cláusula gate substancial esperada**.
- **Janela compat documentada honestamente**: sub-store
  vazio em produção até P195. Não é regressão; é design
  intencional. Comentário inline em
  `resolved_label_store.rs` + nota em L0 + secção em
  consolidado §5.
- **Próximo passo P194 desbloqueado**: consumer C4
  migration via substitution-with-fallback. Blueprint
  P184D/P187B/P188B aplicável directamente. Magnitude S
  esperada.
- **Cadeia P195+ ainda bloqueada**: walks Labelled/Heading
  precisam de Tag emitida + arm em `from_tags`. P193 não
  faz isto — P195 faz.
