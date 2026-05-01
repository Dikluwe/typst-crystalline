# Passo P168 — Migrar `figure-ref` em `layout_ref` (M5 sub-passo 2)

Primeira migração real de consumer para `Introspector`.
Subset apenas: a arm de `Content::Ref` para figuras em
`01_core/src/rules/layout/references.rs::layout_ref`. Único
caso Parcial viável identificado em P167 (relatório
inventário-consumers-counter-state-legacy.md).

**Após P168, M5 fica em pausa.** P169 inicia M9 (features
Introspection vanilla) — todas as 11 features antes de M5
retomar para os outros consumers (decisão pós-P167).

**Pré-condição**: P167 concluído. Inventário de consumers
disponível; figure-ref escolhido como primeiro a migrar.

**Restrições**:
- Não migrar outros consumers (todos Bloqueados ou
  parcialmente Parciais com lacunas críticas — P169+).
- Não eliminar `CounterStateLegacy` (M6, depois de M9).
- Não modificar walk emission (estabelecido em P162).
- Output observable não muda; snapshot tests passam
  inalterados.
- API pública preservada.

---

## Sub-passos

### .A Inventário e decisão sobre filtro figura-numerada-captioned

Reverificar (não confiar em P167):

1. **Localizar consumer**:
   - `01_core/src/rules/layout/references.rs` — função
     `layout_ref`. Identificar:
     - Linhas que lêem `state.figure_label_numbers` ou
       campo equivalente em `CounterStateLegacy`.
     - Lógica do filtro figura-numerada-captioned (apenas
       figuras com `numbering` + `caption` aparecem em
       `figure_label_numbers`).
   - Confirmar localização exacta (`references.rs:35` per
     P167; verificar se ainda é a linha actual).

2. **Inventário do filtro**:
   - Como é que `CounterStateLegacy` decide quais figuras
     entram em `figure_label_numbers`? Ler walk arm para
     `Content::Figure` em `rules/introspect.rs`.
   - Registar predicado exacto (provavelmente
     `figure.numbering.is_some() && figure.caption.is_some()`
     ou similar).

3. **Mapeamento equivalente em `TagIntrospector`**:
   - `state.figure_label_numbers` é `HashMap<Label, usize>`
     (provavelmente).
   - Equivalente em `Introspector`: precisa devolver
     `usize` (número da figura) por label, **filtrado**
     para apenas figuras numeradas+captioned.

4. **Decisão sobre filtro: A vs C**:

   **Caminho A**: `from_tags` filtra. Em
   `rules/introspect/from_tags.rs`, ao processar
   `ElementPayload::Figure`, verificar se figura é "numbered+captioned"
   antes de adicionar a `kind_index[Figure]` ou a um
   sub-mapa novo `numbered_figures: HashMap<Label, Location>`.
   - Problema: `ElementPayload::Figure` actualmente não
     carrega informação sobre numbering/caption. Filtro
     teria que ser feito sobre `figure_kind` ou outro proxy.
     Pode não ser viável directamente.

   **Caminho C**: campo novo em `ElementPayload::Figure`.
   - Adicionar `is_counted: bool` (ou `numbered_with_caption: bool`)
     a `ElementPayload::Figure`.
   - `extract_payload` em `rules/introspect/extract_payload.rs`
     decide o valor lendo `figure.numbering` e
     `figure.caption`.
   - `from_tags` consulta o campo para decidir indexação.
   - Toca `ElementPayload` enum (1 variant), `extract_payload`
     (1 arm), `from_tags` (1 arm), L0s correspondentes.

   **Tabela de decisão**:

   | Critério | Caminho A | Caminho C |
   |----------|-----------|-----------|
   | Modifica `ElementPayload` | Não | Sim (1 field novo em variant) |
   | Modifica `extract_payload` | Não | Sim (1 arm) |
   | Modifica `from_tags` | Sim (lógica filtro) | Sim (consulta field) |
   | Type-safety | Inferida | Explícita no payload |
   | Custo de adicionar features futuras (e.g. captioned-only) | Alto (lógica espalhada) | Baixo (campo claro) |
   | Aderência ao desenho payload type-safe | Inferior | Superior |

   **Sugestão**: caminho C é mais alinhado com o desenho
   ("payload por kind"). Decisão fica para `.A` com base em:
   - Se `figure.numbering` e `figure.caption` são acessíveis
     em `extract_payload`, **C é viável**.
   - Se algum dos dois é runtime-resolvido (depende de
     contexto que `extract_payload` não tem), **A pode ser
     única opção**.

5. **Construção do `Layouter` com `introspector`**:
   - `Layouter::new` em `01_core/src/rules/layout/mod.rs:144`
     (ou linha actual). Identificar assinatura.
   - `pub fn layout()` em `mod.rs:1325` (ou linha actual).
     Identificar onde `introspect()` ou
     `introspect_with_introspector()` são chamados.
   - Decidir como passar `introspector` para `Layouter`:
     - Field `introspector: TagIntrospector` em `Layouter`,
       inicializado em `Layouter::new` ou em `layout()`.
     - Argumento de função (`layout(content, ..., introspector)`).
   - Cláusula gate trivial: escolher caminho mais idiomático
     no cristalino.

Output: notas internas + decisões registadas:
- Caminho A vs C escolhido com justificação.
- Mecanismo de passagem `introspector` ao `Layouter`.
- Fields exactos lidos por `layout_ref` para figure-ref.

**Critério de saída e gate de decisão**:
- Se caminho C é viável (campos acessíveis): escolher C
  e prosseguir.
- Se caminho A é a única opção: escolher A; documentar
  porque C não foi viável.
- Se ambos são viáveis: gate trivial, escolher C por
  alinhamento com desenho.
- Se descobrir que figure-ref tem dependências adicionais
  não identificadas em P167: **gate substancial**, parar
  e reabrir.

### .B Modificações de tipo (apenas se caminho C)

Se caminho A foi escolhido em `.A`, saltar este sub-passo
e ir para `.C`.

1. Update L0 `00_nucleo/prompts/entities/element_payload.md`:
   - Adicionar campo `is_counted: bool` (ou nome decidido
     em `.A`) ao variant `Figure`.
   - Documentar semântica: `true` se figura tem `numbering`
     **e** `caption`; `false` caso contrário.

2. Update L1 `01_core/src/entities/element_payload.rs`:
   - Adicionar campo `is_counted: bool` ao variant `Figure`.
   - Tests co-localizados: construir variant com
     `is_counted: true` e `is_counted: false`, verificar
     igualdade.

3. Update L0 `00_nucleo/prompts/rules/introspect/extract_payload.md`:
   - Documentar que arm `Content::Figure` agora calcula
     `is_counted = figure.numbering.is_some() && figure.caption.is_some()`
     (ou predicado real confirmado em `.A`).

4. Update L1 `01_core/src/rules/introspect/extract_payload.rs`:
   - Modificar arm `Content::Figure` para calcular e
     popular `is_counted`.
   - Tests co-localizados: figure com numbering+caption →
     `is_counted: true`; sem numbering → `false`; sem
     caption → `false`.

**Critério de saída**:
- `cargo check` passa.
- `cargo test` — tests novos passam.
- Linter passa.

### .C Modificar `from_tags` para indexar figuras numeradas

1. Em `01_core/src/rules/introspect/from_tags.rs`:
   - Caminho A: adicionar lógica de filtro (predicado real
     decidido em `.A`).
   - Caminho C: consultar campo `is_counted` para decidir
     indexação.
   - Adicionar sub-mapa novo no `TagIntrospector`:
     `figure_label_numbers: HashMap<Label, usize>` (ou
     método derivado dos sub-stores existentes).

2. Update L0 `00_nucleo/prompts/rules/introspect/from_tags.md`:
   - Documentar lógica nova de indexação de figuras
     numeradas.

3. Update `Introspector` trait em
   `01_core/src/entities/introspector.rs`:
   - Adicionar método `figure_number_for_label(&self, label: &Label) -> Option<usize>`
     (ou nome consistente com convenção).
   - Update L0 correspondente.

4. Update L1 `TagIntrospector` impl: implementar método.

5. Tests co-localizados:
   - Figura com numbering+caption + label → método retorna
     `Some(N)`.
   - Figura sem numbering → método retorna `None`.
   - Figura sem caption → método retorna `None`.

**Critério de saída**:
- `cargo check` passa.
- Tests novos passam.
- Linter passa.

### .D Migrar `layout_ref` (figure-ref)

1. Identificar mecanismo de passagem `introspector` ao
   `Layouter` decidido em `.A`. Implementar.

2. Em `01_core/src/rules/layout/references.rs`:
   - Substituir leitura de `state.figure_label_numbers.get(&label)`
     por `self.introspector.figure_number_for_label(&label)`.
   - Manter `CounterStateLegacy` como input (não remover
     ainda — outros consumers ainda dependem).

3. Update L0 correspondente a `references.rs` (se existir):
   - Documentar que figure-ref agora consulta `Introspector`
     em vez de `CounterStateLegacy`.

**Critério de saída**:
- `cargo check` passa.
- `cargo test` — todos os tests passam (snapshot tests
  ADR-0033 inclusive).
- Linter passa.

### .E Tests de migração

1. Tests específicos do `layout_ref` para figure-ref:
   - Documento com 1 figure numbered+captioned + 1 ref
     → ref renderiza número correcto via `Introspector`.
   - Documento com figure sem numbering + ref → ref
     renderiza forma correcta de "sem número" (mesmo
     comportamento que antes da migração).
   - Documento com figure sem caption + ref → idem.

2. Test de paridade pre/post-migração:
   - Construir documento com figures numbered+captioned;
     comparar output antes (via `state.figure_label_numbers`)
     e depois (via `introspector.figure_number_for_label`).
     **Mesmo output esperado**.

**Critério de saída**:
- 4 tests novos passam.
- Tests existentes continuam a passar.
- Linter passa.

### .F Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace` — todos os tests passam.
   Δ vs baseline P167 (1593).
3. `crystalline-lint`: zero violations.
4. Caminho escolhido em `.A` (A ou C) implementado em `.B`
   (se C) ou directamente em `.C` (se A).
5. `Layouter` tem acesso a `TagIntrospector` (via field ou
   argumento, conforme decidido em `.A`).
6. `layout_ref` para figure-ref consulta
   `introspector.figure_number_for_label` em vez de
   `state.figure_label_numbers`.
7. `CounterStateLegacy` **continua a popular**
   `figure_label_numbers` (outros consumers ainda
   dependem). Não remover ainda.
8. L0s actualizados:
   - `element_payload.md` (se caminho C).
   - `extract_payload.md` (se caminho C).
   - `from_tags.md`.
   - `introspector.md` (método novo).
9. Snapshot tests de paridade ADR-0033 passam inalterados.
10. Linter passa em verificação final.

### .G Encerramento

Escrever
`00_nucleo/materialization/typst-passo-168-relatorio.md` com:

- Resumo: figure-ref migrado para `Introspector`. Caminho
  A/C escolhido em `.A` com justificação.
- Confirmação de cada verificação .F.
- Hashes finais de L0s modificados (preenchidos pelo
  linter).
- Decisões registadas em `.A`:
  - Caminho A vs C com justificação.
  - Mecanismo de passagem `introspector` ao `Layouter`.
- Δ tests vs baseline P167.
- **Estado de M5**: 1 consumer migrado (figure-ref).
  Restantes consumers (Layouter central, layout_outline,
  counter_helpers, layout_equation) **bloqueados até M9
  estar concluído**. M5 fica em pausa.
- Pendências cumulativas + nova actualização (lacuna #1
  resolvida via filtro figura-numerada-captioned se
  caminho C; lacuna #2/#3 ainda pendentes).
- Estado pós-passo: P168 concluído. **P169 começa M9**
  (primeira feature Introspection — decisão em P169 `.A`).

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu inventário com escolha A/C e mecanismo
   de passagem `introspector` ao `Layouter`.
2. Modificações de tipo aplicadas (se caminho C).
3. `from_tags` indexa figuras numeradas+captioned.
4. `Introspector` trait tem método novo
   `figure_number_for_label`.
5. `Layouter` tem acesso a `TagIntrospector`.
6. `layout_ref` para figure-ref consulta `Introspector`.
7. Tests de migração passam; paridade pre/post-migração
   confirmada.
8. Verificações `.F` 1-10 passam.
9. Relatório `.G` escrito.
10. Output observable não muda.
11. M5 fica em pausa após este passo. P169 começa M9.

---

## O que pode sair errado

- **Caminho C não viável (`.A` revela `figure.numbering` ou
  `figure.caption` não acessíveis em `extract_payload`)**:
  fallback para A. Documentar porque C não foi viável.
- **Caminho A revela lógica complexa de filtro**: pode
  precisar de payload mais rico de qualquer forma. Se for
  o caso, voltar para C com expansão de `extract_payload`.
- **`Layouter::new` tem assinatura complicada**: adicionar
  field novo pode tocar muitos call-sites. Cláusula gate
  trivial: escolher mecanismo de menor disruption (field
  com default vs argumento explícito).
- **`Introspector` trait precisa método novo (`figure_number_for_label`)**:
  é razoável, mas significa que P165 não cobriu todos os
  métodos necessários. Documentar como expansão.
- **Tests de paridade falham**: pode revelar divergência
  subtil entre `figure_label_numbers` legacy e
  `Introspector` novo. Investigar — pode ser bug em
  `from_tags` ou em `extract_payload`. Corrigir antes
  de prosseguir.
- **`CounterStateLegacy.figure_label_numbers` continua a
  ser populado mas agora redundante** (apenas lido por
  ninguém após migração): aceitável em M5 incompleto;
  campo será eliminado em M6 quando todos os consumers
  migrarem.
- **Linter detecta divergência L0↔L1 ao expandir
  `ElementPayload`**: ajustar conforme erro.

---

## Notas operacionais

- **Tamanho**: M. Caminho C: 4 L0s + 4 L1s tocados +
  migração + tests. Caminho A: 2 L0s + 2 L1s tocados.
  C é maior mas mais alinhado com desenho.
- **Pré-condição P169 (M9)**: M5 em pausa após P168. P169
  começa M9 com primeira feature Introspection (decisão
  em P169 `.A`).
- **Cláusula gate trivial** aplicável a decisões locais
  em `.A` (mecanismo de passagem ao `Layouter`, nome do
  método novo no `Introspector`).
- **`CounterStateLegacy.figure_label_numbers`**: continua
  a ser populado pelo walk em paralelo. Não eliminar.
  Outros consumers (provavelmente nenhum directo, mas
  validar) podem depender. Eliminação só em M6.
- **Resolução de lacuna #1 (figure.kind None vs "image")**:
  caminho C resolve indirectamente — `is_counted` filtra
  no momento da extracção, lacuna deixa de afectar este
  consumer. Lacunas #2 e #3 permanecem. Documentar.
