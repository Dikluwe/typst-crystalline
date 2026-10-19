# Passo P184C — `Introspector::figure_number_at_index` trait method

Segundo passo de implementação P184 (após P184A diagnóstico,
P184B refinamento arm Figure).
Magnitude **S**.

Adiciona método `figure_number_at_index(&self, kind: &str, idx:
usize) -> Option<usize>` ao trait `Introspector`. Impl em
`TagIntrospector` delega ao `CounterRegistry`. Se necessário,
adiciona helper `value_at_index` ao `CounterRegistry` (avaliado
em `.A.4`). 5 tests unitários cobrem casos típicos.

Após P184C:
- Trait `Introspector` ganha `figure_number_at_index`.
- `TagIntrospector` impl funciona contra `CounterRegistry`
  populado via P184B.
- Helper `value_at_index` em `CounterRegistry` se necessário
  (acesso por posição na história do counter por chave).
- Tests unitários cobrem populate + lookup.
- Layouter continua a ler de `state.counter.figure_numbers`
  legacy (até P184D).

**Pré-condição**: P184B concluído. Tests workspace 1.756
verdes; zero violations. Arm Figure popula
`CounterRegistry` com chave `figure:{kind}` em paralelo a
chave global `"figure"`.

**Restrições**:
- **Não** migrar consumer C3 — P184D.
- **Não** modificar arm Figure em `from_tags.rs` (P184B
  fechou).
- **Não** modificar walk arm Figure em `introspect.rs`.
- **Não** modificar `kind_index` ou `figure_label_numbers`.
- API pública preservada (adição de método trait não
  quebra callers que não chamem o método).
- Output observable em produção inalterado — método novo
  não tem consumer ainda.

---

## Sub-passos

### .A Auditoria L0

1. Confirmar trait `Introspector` actual:
   - `01_core/src/entities/introspector.rs` (per padrão).
   - Localizar definição do trait.
   - Identificar localização sugerida para inserir
     `figure_number_at_index` (ordem alfabética? ou
     agrupar com outros métodos figure-related?).
   - Confirmar que P181F (`bib_*_for_key`) e P182B
     (`is_numbering_active`) estão listados — ponto de
     ancoragem.

2. Confirmar L0 actual `entities/introspector.md`:
   - Localizar lista de métodos do trait.
   - Verificar onde adicionar entrada nova.

3. Confirmar `CounterRegistry`:
   - `01_core/src/entities/counter_registry.rs` (per
     P184A inventário).
   - Métodos existentes (`apply_at`, `value_at`, `format`,
     `formatted_at`, etc.).
   - Estrutura interna: `HashMap<String, Vec<(Location,
     Vec<usize>)>>` ou similar — confirmar empiricamente.
   - Verificar se já existe método `value_at_index` ou
     equivalente que retorne valor por posição na
     história (não por Location).

4. **Decisão pendente — helper `value_at_index`**:
   - Se `CounterRegistry` já expõe forma de obter "n-ésimo
     valor da chave X": **não adicionar helper** — impl
     do trait method delega directamente.
   - Se `CounterRegistry` só expõe acesso por Location:
     adicionar helper `value_at_index(&self, key: &str,
     idx: usize) -> Option<usize>` (ou similar) que
     percorre a `history` interna pela posição.
   - Decisão final em `.A.4` baseada em inspecção
     empírica.

5. Confirmar `TagIntrospector` impl block:
   - `impl Introspector for TagIntrospector { ... }` em
     `introspector.rs` (ou ficheiro de impl).
   - Localizar onde adicionar método novo.
   - Confirmar field name do `CounterRegistry` em
     `TagIntrospector` (`counters` ou outro).

6. Confirmar tests existentes em `mod tests`:
   - Padrão de helpers de construção (`TagIntrospector::empty()`?
     Helper directo para popular?).
   - Replicar padrão em `.D` tests.

Output: tabela com item + estado confirmado / linha
actual / observação. Decisão sobre helper registada.

**Critério de saída e gate de decisão**:
- Se `CounterRegistry` não tem método de acesso por idx:
  cláusula gate trivial — adicionar helper em `.C` antes
  do trait method.
- Se assinatura de `value_at` ou similar diverge do
  esperado: cláusula gate trivial — adaptar.
- Senão prosseguir.

### .B Actualizar L0 `entities/introspector.md`

1. Adicionar entrada para método novo:
   - Nome: `figure_number_at_index`.
   - Assinatura: `fn figure_number_at_index(&self, kind:
     &str, idx: usize) -> Option<usize>`.
   - Propósito: consulta o número da figure de kind X na
     posição idx (0-indexed) entre as figures desse kind
     processadas durante walk.
   - Default: `None` quando kind ausente em
     `CounterRegistry` ou idx fora de range.
   - Implementação: delega a `CounterRegistry`
     (eventualmente via helper `value_at_index` per `.A.4`).
   - Posição na lista: agrupar com outros métodos
     figure-related (se existirem) ou após
     `is_numbering_active` (P182B).

2. Hash em branco aguarda recálculo manual após
   confirmação humana.

**Critério de saída**:
- L0 contém entrada nova.
- Coerente com convenção dos métodos existentes.

### .C Adicionar helper em `CounterRegistry` (se necessário per `.A.4`)

Apenas se `.A.4` confirmar que helper é necessário.

1. Em `01_core/src/entities/counter_registry.rs`:
   - Adicionar método público (ou `pub(crate)`):
     `value_at_index(&self, key: &str, idx: usize) ->
     Option<usize>`.
   - Implementação: lookup da `history` para `key`;
     retorna o valor na posição `idx` (acesso linear ou
     directo conforme estrutura interna).
   - Se a estrutura interna for `HashMap<String,
     Vec<(Location, Vec<usize>)>>`, retornar
     `entries[idx].1.last().copied()` ou similar — forma
     exacta fica para Claude Code conforme convenção.

2. Tests unitários do helper (2-3) co-localizados em
   `mod tests` de `counter_registry.rs`:
   - Vazio devolve `None`.
   - Após `apply_at` retorna valor correcto.
   - Idx fora de range devolve `None`.

3. Cabeçalho `@prompt-hash` actualiza após edit do L0
   (se houver L0 dedicado a `counter_registry.md`).

**Critério de saída**:
- `cargo check --workspace` passa.
- 2-3 tests novos do helper passam.
- Tests existentes não regridem.
- Linter passa.

### .D Adicionar método ao trait + impl

1. Em `01_core/src/entities/introspector.rs`:
   - Adicionar declaração ao trait `Introspector`:
     ```
     fn figure_number_at_index(&self, kind: &str, idx: usize) -> Option<usize>;
     ```
   - Posicionar conforme `.A.1`.

2. Em `impl Introspector for TagIntrospector`:
   - Adicionar método.
   - Delegar ao `CounterRegistry`:
     - Construir chave `format!("figure:{}", kind)`.
     - Chamar `value_at_index` (helper de `.C`) ou
       método existente equivalente.
   - Forma exacta fica para Claude Code.

3. Documentação inline do método: 1-3 linhas
   explicando propósito + comportamento default.

4. Confirmar cabeçalho `@prompt-hash` actualiza após edit
   do L0.

**Critério de saída**:
- `cargo check --workspace` passa.
- Linter passa.

### .E Tests unitários

5 tests obrigatórios (padrão P181F / P182B):

1. **Vazio devolve `None`** — `TagIntrospector::empty()`
   + chamada `figure_number_at_index("image", 0)` →
   `None`.

2. **Após populate retorna `Some(N)`** — populate
   `CounterRegistry` directamente para chave
   `"figure:image"` com valor (idx 0); chamada retorna
   `Some(N)`.

3. **Kinds distintos isolados** — populate
   `figure:image` e `figure:table` separadamente;
   chamada para `image` retorna valor correcto, para
   `table` outro valor (key isolation).

4. **Idx fora de range devolve `None`** — populate com
   1 entry; `figure_number_at_index("image", 5)` →
   `None`.

5. **Default kind** — populate via mecanismo equivalente
   ao do arm Figure (com `kind: None` no payload, que
   per P184B mapeia para `"image"`); leitura via
   `figure_number_at_index("image", 0)` retorna valor.

Tests co-localizados em `mod tests` dentro de
`introspector.rs` (ou ficheiro de impl). Helpers de
construção replicam padrão dos tests existentes para
`bib_*_for_key` (P181F) ou `is_numbering_active`
(P182B).

**Critério de saída**:
- 5 tests novos passam.
- Tests existentes não regridem (1.756 + 5 + eventuais
  de `.C` helper = 1.761 a 1.764 esperado).

### .F Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P184B
   baseline (1.756): +5 (sem helper) ou +7 a +8 (com
   helper).
3. `crystalline-lint .` zero violations.
4. `figure_number_at_index` accessível via trait
   `Introspector`.
5. `TagIntrospector` impl delega correctamente.
6. Documento sem figures produz `figure_number_at_index(*,
   *)` = `None`.
7. Documento com figures produzidas via P184B arm
   produz valor correcto.
8. Walk **NÃO modificado**.
9. Layouter **NÃO modificado** (esperado em P184D).
10. Snapshot tests ADR-0033 verdes.
11. Linter passa final.

### .G Encerramento

Escrever
`00_nucleo/materialization/typst-passo-184c-relatorio.md`
com:

- Resumo: trait method materializado; impl delega a
  `CounterRegistry` (com ou sem helper conforme `.A.4`);
  5 tests unitários cobrem casos típicos.
- Confirmação `.F` (11 verificações).
- Δ tests vs baseline P184B (esperado +5 a +8).
- Hashes finais de L0s modificados (`introspector.md` +
  eventualmente `counter_registry.md`).
- Decisões de execução notáveis (em particular sobre
  helper).
- Estado actual:
  - P184 série: A ✅ B ✅ C ✅ | D-F pendentes.
  - C3 desbloqueio: eixo 2 atendido (dados em sub-store
    + método de acesso); eixo 1 já era OK; falta consumer
    migrado (P184D).
  - 40 passos executados.
- Próximo passo: P184D (migrar consumer C3 em
  `mod.rs:435–439` com substitution-with-fallback).

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate substancial;
   decisão sobre helper registada.
2. L0 `entities/introspector.md` actualizado.
3. (Se `.C` accionado) Helper `value_at_index` adicionado
   ao `CounterRegistry`.
4. Método `figure_number_at_index` declarado no trait.
5. Método implementado em `TagIntrospector`.
6. 5 tests unitários novos passam.
7. (Se `.C` accionado) 2-3 tests do helper passam.
8. Tests existentes não regridem.
9. Verificações `.F` passam.
10. Relatório `.G` escrito.
11. Output observable em produção inalterado.

---

## O que pode sair errado

- **`CounterRegistry` não expõe acesso por idx**: cláusula
  gate trivial — adicionar helper em `.C`. Esperado caso
  comum.
- **Estrutura interna do `CounterRegistry` é diferente do
  esperado**: cláusula gate trivial — adaptar
  implementação do helper.
- **`value_at_index` colide com método existente**:
  improvável; renomear se necessário (`get_at_index`,
  `nth_value`, etc.).
- **Trait method exige `&mut self`**: improvável; método é
  read-only.
- **Tests existentes regridem**: improvável dado que o
  método novo é adição. Se acontecer, investigar.
- **Helper retorna valor diferente do esperado**: pode
  haver diferença entre "n-ésima entry no Vec interno"
  vs "n-ésimo valor único acumulado". Documentar
  decisão e adaptar tests.
- **Linter divergência V13/V14 por adição de método sem
  L0**: cláusula gate trivial — actualizar L0 antes do
  código.

---

## Notas operacionais

- **Tamanho**: S puro. ~80-200 LOC dependendo de
  necessidade de helper:
  - Sem helper: ~10 LOC trait + impl + 5 tests (~50 LOC).
  - Com helper: ~10 LOC trait + impl + helper (~10 LOC)
    + 5 tests (~50 LOC) + 2-3 tests helper (~30 LOC).
- **Sem dependências externas novas**.
- **Pré-condição P184D**: este passo concluído.
- **Padrão replicado**: P181F (`bib_*_for_key`) + P182B
  (`is_numbering_active`).
- **Cláusula gate trivial**: aplicável a estrutura
  interna do `CounterRegistry`, nome do helper, formato
  de tests.
- **Sem cláusula gate substancial esperada**.
- **Decisão sobre helper é interna a P184C** — não
  arrasta P184D nem P184E.
