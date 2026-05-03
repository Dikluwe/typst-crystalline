# Passo P184B — Refinar `from_tags` arm Figure (chave `figure:{kind}`)

Primeiro passo de implementação P184 (após P184A diagnóstico).
Magnitude **S**.

Refina arm `Figure` em `from_tags.rs:71-95` para popular
`CounterRegistry` com chave `figure:{kind}` (per cláusula 1
P184A: `figure:{kind}` quando `Some`, `figure:image` quando
`None`). Mantém população paralela na chave global `"figure"`
durante janela compat (cláusula 5 P184A: simetria com P181/P182,
ainda que P184A §3.6 confirme legacy é dead code factual).

L0 `from_tags.md` actualizado para documentar convenção
promovida de `element_payload.rs:52` (que documentava
`figure:{kind}` mas nunca implementava).

Após P184B:
- `CounterRegistry` recebe entries para `figure:image`,
  `figure:table`, etc. quando arm Figure processa tag.
- Chave global `"figure"` continua populada (path legacy
  paralelo, dead code mas mantido por simetria até M6).
- `kind_index[ElementKind::Figure]` continua intocado
  (consumers figure-ref P168 não regridem).
- `figure_label_numbers` continua intocado (mesmo).
- Trait `Introspector` ainda **sem** `figure_number_at_index`
  (P184C adiciona).

**Pré-condição**: P184A concluído. Tests workspace 1.756
verdes; zero violations. 6 cláusulas P184A fixadas.

**Restrições**:
- **Não** adicionar método trait — P184C.
- **Não** migrar consumer C3 — P184D.
- **Não** modificar walk arm Figure em `introspect.rs` — fica
  legacy populado paralelo até M6.
- **Não** modificar `extract_payload` arm Figure
  (`extract_payload.rs:27-34`) — payload já carrega `kind`.
- **Não** modificar consumers existentes de `kind_index[Figure]`
  ou `figure_label_numbers`.
- API pública preservada.
- Output observable em produção inalterado — nova chave
  `figure:{kind}` populada em paralelo, sem consumer ainda.

---

## Sub-passos

### .A Auditoria L0

1. Confirmar arm `Figure` actual:
   - `01_core/src/rules/introspect/from_tags.rs:71-95` (per
     P184A §3.1).
   - Localizar `match payload { ... ElementPayload::Figure { .. } => ... }`.
   - Confirmar uso actual de `..` pattern que ignora `kind`
     (P184A §3.1 verificou).
   - Confirmar chamada actual a `apply_at` ou similar com
     chave global `"figure"`.

2. Confirmar payload `ElementPayload::Figure`:
   - `01_core/src/entities/element_payload.rs:52` (per
     P184A §3.2).
   - Confirmar `kind: Option<String>` é campo do payload.
   - Confirmar comentário documentando convenção
     `figure:{kind}` (a promover para implementação).

3. Confirmar `extract_payload.rs:27-34` arm Figure:
   - Já produz payload com `kind: kind.clone()`
     (P184A §3.3).
   - **Não modificar** — já correcto.

4. Confirmar `CounterRegistry::apply_at`:
   - `01_core/src/entities/counter_registry.rs` (ou similar).
   - Assinatura: `apply_at(&mut self, key: String, update:
     ..., location: Location)` ou similar.
   - Confirmar que aceita `String` como chave (não `&str`
     fixo).

5. Confirmar tests existentes:
   - `from_tags.rs:339-396` (per P184A §3) — tests sobre
     `figure_label_numbers` e `kind_index[Figure]`.
   - Devem continuar a passar sem alteração (nova chave
     `figure:{kind}` é adição, não substituição).

6. Confirmar L0 actual `from_tags.md`:
   - Localizar entrada actual sobre arm Figure.
   - Identificar onde adicionar documentação de convenção
     `figure:{kind}`.

Output: tabela com item + estado confirmado / linha actual /
observação.

**Critério de saída e gate de decisão**:
- Se `apply_at` exige chave `&str` em vez de `String`:
  cláusula gate trivial — adaptar com `.as_str()` ou
  similar.
- Se `kind: Option<String>` ausente do payload (improvável
  per P184A): cláusula gate substancial — recuar e
  investigar.
- Senão prosseguir.

### .B Actualizar L0 `from_tags.md`

1. Adicionar/actualizar entrada para arm Figure:
   - Documentar convenção: chave `figure:{kind}` quando
     `kind: Some(_)`; `figure:image` quando `kind: None`
     (default per P184A cláusula 1).
   - Justificar default `"image"` (referência a
     `introspect.rs:391` e `mod.rs:431` que usam mesmo
     default per P184A §3).
   - Documentar que chave global `"figure"` continua
     populada em paralelo durante janela compat M6.
   - Cross-reference: `element_payload.rs:52` que
     documentava convenção mas não implementava.

2. Hash em branco aguarda recálculo manual após
   confirmação humana.

**Critério de saída**:
- L0 contém entrada actualizada.
- Texto coerente com convenção P182A
  (`<feature>:<sub-feature>`).
- Default kind documentado literalmente.

### .C Refinar arm Figure em `from_tags.rs`

1. Em `from_tags.rs:71-95`:
   - Modificar destructure do payload para capturar `kind`
     (em vez de ignorar via `..`).
   - Adicionar chamada `apply_at` com chave `figure:{kind}`
     usando `kind.as_deref().unwrap_or("image")` para
     default.
   - **Manter** chamada `apply_at` existente com chave
     global `"figure"` (paralelo durante M6).
   - **Manter** `kind_index[ElementKind::Figure]` push.
   - **Manter** `figure_label_numbers` populate condicional
     em `is_counted`.

2. Forma exacta fica para Claude Code conforme convenção do
   projecto. P184A §14 sugeriu forma; usar como referência
   se aplicável.

3. Confirmar cabeçalho de linhagem `@prompt-hash` actualiza
   após edit do L0.

**Critério de saída**:
- `cargo check --workspace` passa.
- `cargo build --workspace` passa.
- Linter passa.

### .D Verificação de regressão

1. `cargo test --workspace --lib` — esperado: **1.756
   inalterado**. Nenhum test novo adicionado em P184B; arm
   refinado adiciona população, não substitui — tests
   existentes sobre `figure_label_numbers` e
   `kind_index[Figure]` continuam a passar.

2. Tests específicos do arm Figure em `from_tags.rs:339-396`
   passam:
   - Asserção sobre `figure_label_numbers` mantém-se.
   - Asserção sobre `kind_index[Figure]` mantém-se.
   - Não há asserção sobre `counters` registry para chave
     `"figure"` global ou `"figure:image"` específica
     (ainda não — P184C adiciona).

3. Snapshot tests ADR-0033 verdes.

**Critério de saída**:
- Δ vs P184A baseline (1.756): **0**.
- Tests existentes não regridem.
- Linter passa.

### .E Encerramento

Escrever
`00_nucleo/materialization/typst-passo-184b-relatorio.md`
com:

- Resumo: arm Figure refinado para popular `CounterRegistry`
  com chave `figure:{kind}`; chave global `"figure"`
  continua paralela; sem método trait novo (P184C).
- Confirmação `.D` (regressão zero).
- Hashes finais de L0 modificado (`from_tags.md`).
- Decisões de execução notáveis (se houver).
- Estado actual:
  - P184 série: A ✅ B ✅ | C-F pendentes.
  - M9: 11/11 (inalterado).
  - C3 desbloqueio em curso: eixo 2 parcialmente atendido
    (dados em sub-store presentes para nova chave); eixo
    1 já era OK; falta método trait + consumer migrado.
  - 39 passos executados.
- Pendências cumulativas: inalteradas.
- Próximo passo: P184C (método trait `figure_number_at_index`
  + impl + 5 tests unit).

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate substancial.
2. L0 `from_tags.md` actualizado com convenção `figure:{kind}`.
3. Arm Figure refinado em `from_tags.rs:71-95`.
4. Chave global `"figure"` continua populada paralelamente.
5. Tests existentes não regridem (Δ 0).
6. Verificações `.D` passam.
7. Relatório `.E` escrito.
8. Output observable em produção inalterado.

---

## O que pode sair errado

- **`apply_at` exige `&str`**: cláusula gate trivial —
  adaptar.
- **Tests existentes regridem inesperadamente**: indica que
  alguma asserção dependia da ausência de chave
  `figure:{kind}` — improvável mas possível. Investigar
  test específico antes de prosseguir.
- **`kind: Option<String>`** tem variantes que P184A não
  inventariou (e.g. `Option<&'static str>` ou enum em vez
  de `String`): cláusula gate trivial — adaptar
  `unwrap_or("image")`.
- **Linter divergência V13/V14**: cláusula gate trivial —
  `--fix-hashes`.
- **`@prompt-hash` desactualizado**: recálculo manual.

---

## Notas operacionais

- **Tamanho**: S puro. ~5 LOC arm + edits L0 (~10 linhas).
- **Sem dependências externas novas**.
- **Sem método trait novo** (P184C).
- **Sem consumer migrado** (P184D).
- **Sem tests novos** em P184B — adições estruturais sem
  asserção. Tests do método trait virão em P184C.
- **Pré-condição P184C**: este passo concluído.
- **Padrão replicado**: P181C/P182C/P178 (refinar arm
  existente para emitir mais informação).
- **Cláusula gate trivial**: aplicável a assinatura de
  `apply_at`, forma de `kind`, recálculo de hashes.
- **Cláusula gate substancial**: aplicável apenas se
  `kind` ausente do payload (improvável per P184A).
- **Convenção promovida**: `element_payload.rs:52`
  documentava `figure:{kind}` mas nunca implementava.
  P184B implementa o que estava documentado. Documentação
  fica em sintonia com código.
- **Dead code paralelo registado honestamente**: chave
  global `"figure"` é mantida por simetria com walk legacy
  e padrão P181/P182, **não** por preservação observable.
  Cleanup vem em M6.
