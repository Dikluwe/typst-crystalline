# Passo P181H — walk arm `Bibliography` puro + `layout()` legacy migra

Sétimo passo de materialização P181 (após P181B–P181G).
Magnitude **S** com subtileza arquitectural em `.A`.

Restaura invariante walk puro (P163) — walk arm `Content::Bibliography`
deixa de mutar directamente `state.bib_*`. Tag emitida pelo topo
via `extract_payload` (P181D); `BibStore` populado por `from_tags`
arm (P181E).

**Decisão associada**: `layout()` legacy migra de
`layout_with_introspector(_, _, TagIntrospector::empty())` para
`introspect_with_introspector` + `layout_with_introspector`.
Sem esta mudança, cite-arm em path legacy retorna `None` →
regressão silenciosa.

Após P181H:
- Walk arm `Content::Bibliography` puro (apenas desce em `title`).
- `state.bib_*` legacy continua a existir mas **vazios** em
  produção (M6 elimina os fields).
- Path legacy `layout()` funciona via `Introspector` populado.
- Janela compat encerrada para bib state.

**Pré-condição**: P181G concluído. Cite-arm consome via
Introspector. Fallback defensivo a state legacy preservado.

**Restrições**:
- API pública preservada — `layout()` mantém signature.
- Output observable não muda — paridade verificada.
- Cite-arm fallback a state legacy mantido (M6 elimina;
  durante janela compat permanece como segurança extra).
- Field `state.bib_entries` + `bib_numbers` **continuam a
  existir** (eliminação em M6). Apenas deixam de ser
  populados em produção.

---

## Sub-passos

### .A Auditoria L0 + decisão `layout()` legacy

1. Confirmar walk arm actual:
   - `01_core/src/rules/introspect.rs:567-573`.
   - Forma actual:
     ```rust
     Content::Bibliography { entries, title } => {
         for entry in entries {
             let next_num = state.bib_numbers.len() as u32 + 1;
             state.bib_numbers.entry(entry.key.clone()).or_insert(next_num);
         }
         state.bib_entries.extend(entries.iter().cloned());
         if let Some(t) = title { walk(t, state, locator, tags, None); }
     }
     ```
   - Identificar variáveis exactas (`state`, `locator`,
     `tags`, etc.).

2. Confirmar `layout()` legacy actual:
   - `01_core/src/rules/layout/mod.rs:1380-1400` (forma
     aproximada — verificar).
   - Identificar como state é obtido:
     - Provavelmente: `let state = introspect(content);`
       seguido de `layout_with_introspector(content, state, TagIntrospector::empty())`.
   - Decisão `layout()` migração:
     - **Opção A** (sugerida): substituir por
       `introspect_with_introspector` (P166):
       ```rust
       pub fn layout(content: &Content) -> Frame {
           let (state, intr) = introspect_with_introspector(content);
           layout_with_introspector(content, state, intr)
       }
       ```
     - **Opção B**: manter `introspect()` mas adicionar
       construção paralela de Introspector. Trabalho dobrado.
     - Sugestão **A** — aproveita entry point P166.

3. Confirmar `introspect_with_introspector` API:
   - `rules/introspect.rs` ou `entities/`.
   - Signature: `pub fn introspect_with_introspector(content: &Content) -> (CounterStateLegacy, TagIntrospector)`.
   - Confirmar nome exacto e tipo de retorno.

4. Custo computacional:
   - `introspect_with_introspector` faz walk **uma vez**
     e produz ambos (state + introspector) em paralelo
     (P166).
   - Logo `layout()` antes fazia: walk (em introspect) +
     walk (em layout_with_introspector).
   - `layout()` depois faz: walk (em introspect_with_introspector)
     + walk (em layout_with_introspector).
   - **Custo idêntico** — P181H não introduz overhead.
     Confirmar.

5. Confirmar copy-sites mantidos:
   - `mod.rs:1385-1388` e `mod.rs:1413-1416`.
   - Em path Opção A `layout()` migrado, copy-site
     `state→Layouter` continua a copiar `state.bib_*`
     mas **vazios** (porque walk arm já não popula).
     Cite-arm fallback a `self.counter.bib_*` retorna
     `None`/empty → não contribui. Aceitável.
   - Em path `layout_with_introspector` chamado externamente:
     mesmo comportamento. Introspector populado serve.

6. Identificar tests existentes que cobrem path
   `layout()` legacy:
   - `cargo test --lib layout` ou similar.
   - Confirmar quais testam render de Bibliography +
     Cite via `layout()` directo. Estes devem continuar
     a passar pós-migração.

Output: notas internas; sem ficheiro novo.

**Critério de saída**:
- Auditoria limpa.
- Opção A confirmada.
- Custo computacional confirmado idêntico.

### .B Tests primeiro (devem falhar parcialmente)

Em `01_core/src/rules/introspect.rs::tests`:

```rust
#[test]
fn walk_arm_bibliography_puro_nao_muta_state_p181h() {
    let entry = make_bib_entry("a");
    let content = Content::Bibliography {
        entries: vec![entry.clone()],
        title: None,
    };
    let mut state = CounterStateLegacy::empty();
    let mut locator = Locator::root();
    let mut tags = Vec::new();
    walk(&content, &mut state, &mut locator, &mut tags, None);

    // Walk não popula state legacy
    assert!(state.bib_entries.is_empty(),
        "walk arm puro: state.bib_entries deve permanecer vazio");
    assert!(state.bib_numbers.is_empty(),
        "walk arm puro: state.bib_numbers deve permanecer vazio");

    // Mas Tag foi emitida (extract_payload do topo)
    assert!(tags.iter().any(|t| matches!(
        t,
        Tag::Start(_, info) if matches!(
            info.payload,
            ElementPayload::Bibliography { .. }
        )
    )), "Tag::Start Bibliography deve estar presente");
}
```

Em `01_core/src/rules/layout/tests.rs`:

```rust
#[test]
fn layout_legacy_renderiza_cite_via_introspector_apos_p181h() {
    // Construir Content com Bibliography + Cite Normal
    let content = make_doc_with_cite();

    // layout() legacy deve agora popular Introspector
    // internamente e cite-arm consome correctamente.
    let frame = layout(&content);

    let text = plain_text(&frame);
    // Cite Normal renderiza "[1]" em vez de "[key]".
    assert!(text.contains("[1]"));
}

#[test]
fn paridade_layout_vs_layout_with_introspector_apos_p181h() {
    let content = make_doc_with_cite();

    // Path 1: layout() legacy.
    let frame1 = layout(&content);

    // Path 2: layout_with_introspector explícito.
    let (state, intr) = introspect_with_introspector(&content);
    let frame2 = layout_with_introspector(&content, state, intr);

    assert_eq!(plain_text(&frame1), plain_text(&frame2));
}
```

Confirmar:
- Test `walk_arm_bibliography_puro_nao_muta_state` falha
  (walk ainda muta state).
- Tests `layout_legacy_renderiza_cite_via_introspector`
  + paridade falham (layout legacy não popula
  introspector ainda).

**Critério de saída**:
- Tests escritos.
- Falham conforme esperado.

### .C Update L0 `introspect.md`

Documentar walk arm puro:
- `Content::Bibliography` arm não muta `state.bib_*`
  directamente.
- Restaura invariante walk puro P163 (violada por P159C/F
  para bib).
- Tag emitida via `extract_payload` (mecanismo geral
  P162).
- `BibStore` populado por `from_tags` arm (P181E).

**Critério de saída**:
- L0 actualizado.

### .D Update L0 `layout.md`

Documentar `layout()` legacy migração:
- Antes: `layout_with_introspector(_, _, empty)`.
- Depois: `introspect_with_introspector + layout_with_introspector`.
- Custo computacional inalterado.
- Path legacy preserva funcionalidade bib via
  Introspector populado.

**Critério de saída**:
- L0 actualizado.

### .E Humano calcula `@prompt-hash`

Marco humano. Após `.C` e `.D`:
- `crystalline-lint --fix-hashes`.
- L1 linhagens em `introspect.rs` e `layout/mod.rs`
  actualizadas.

**Critério de saída**:
- L0 hashes preenchidos.
- L1 `@prompt-hash` correspondentes.

### .F Implementar walk arm puro

Em `01_core/src/rules/introspect.rs:567-573`:

```rust
// ANTES (P159C/F):
Content::Bibliography { entries, title } => {
    for entry in entries {
        let next_num = state.bib_numbers.len() as u32 + 1;
        state.bib_numbers.entry(entry.key.clone()).or_insert(next_num);
    }
    state.bib_entries.extend(entries.iter().cloned());
    if let Some(t) = title { walk(t, state, locator, tags, None); }
}

// DEPOIS (P181H):
Content::Bibliography { title, .. } => {
    // Tag emitida pelo topo via extract_payload (P181D).
    // BibStore populado por from_tags arm (P181E).
    // Walk puro — sem mutação directa de state.bib_*.
    if let Some(t) = title {
        walk(t, state, locator, tags, None);
    }
}
```

**Critério de saída**:
- `cargo check` passa.
- Test `walk_arm_bibliography_puro_nao_muta_state` passa.
- Tests existentes que dependem de state.bib_* via path
  `layout()` legacy podem falhar — esperado e corrigido
  em `.G`.

### .G Implementar `layout()` migração

Em `01_core/src/rules/layout/mod.rs::layout` (linha
~1380):

```rust
// ANTES:
pub fn layout(content: &Content) -> Frame {
    let state = introspect(content);
    layout_with_introspector(content, state, TagIntrospector::empty())
}

// DEPOIS:
pub fn layout(content: &Content) -> Frame {
    let (state, intr) = introspect_with_introspector(content);
    layout_with_introspector(content, state, intr)
}
```

Adaptar nomes exactos. Confirmar compilação.

**Critério de saída**:
- `cargo check` passa.
- Tests `.B` `layout_legacy_renderiza_cite_via_introspector`
  e paridade passam.
- Tests existentes de cite via `layout()` continuam a
  passar.
- Linter passa.

### .H Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` — todos os tests
   passam. Δ vs P181G baseline (1731). Estimativa: +3
   (3 tests `.B`).
3. `crystalline-lint .`: zero violations.
4. L0 `introspect.md` actualizado com hash.
5. L0 `layout.md` actualizado com hash.
6. L1 `introspect.rs` linhagem actualizada.
7. L1 `layout/mod.rs` linhagem actualizada.
8. Walk arm `Content::Bibliography` puro (não muta
   `state.bib_*`).
9. `state.bib_entries` e `bib_numbers` permanecem
   vazios após walk em produção.
10. Tag `Tag::Start(loc, ElementInfo { payload:
    Bibliography {..}, .. })` emitida via
    `extract_payload`.
11. `from_tags` continua a popular `BibStore` via arm
    (P181E).
12. `layout()` legacy migrado para
    `introspect_with_introspector` + `layout_with_introspector`.
13. Cite-arm em path `layout()` legacy renderiza
    correctamente via Introspector.
14. Paridade `layout()` vs `layout_with_introspector`
    confirmada.
15. Snapshot tests ADR-0033 verdes.
16. Linter passa final.

### .I Encerramento

Escrever
`00_nucleo/materialization/typst-passo-181h-relatorio.md`
com:

- Resumo: walk arm puro restaura P163; `layout()` legacy
  migrado para `introspect_with_introspector`. Janela
  compat encerrada para bib state.
- Confirmação de cada verificação `.H`.
- Hashes finais de L0s modificados.
- Decisões registadas em `.A`:
  - Opção A confirmada para `layout()` legacy.
  - Custo computacional verificado idêntico.
- Δ tests vs baseline P181G (esperado +3).
- **Estado de P181**: A-H concluídos; I-J pendentes.
- **Invariante walk puro P163 restaurada para
  Bibliography**.
- **Janela compat encerrada para bib state**: state
  legacy fields existem mas vazios; eliminados em M6.
- Pendências cumulativas inalteradas.
- Estado pós-passo: P181H concluído. P181I desbloqueado
  (tests E2E + lacuna #6 fechada).

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria + decisão Opção A registada.
2. Tests escritos primeiro (`.B`); falharam.
3. L0 `introspect.md` actualizado.
4. L0 `layout.md` actualizado.
5. Hashes calculados (`.E`).
6. Walk arm puro implementado (`.F`).
7. `layout()` legacy migrado (`.G`).
8. Verificações `.H` 1-16 passam.
9. Relatório `.I` escrito.
10. Output observable não muda.
11. Invariante P163 restaurada.

---

## O que pode sair errado

- **`introspect_with_introspector` API diferente do
  esperado**: confirmar signature exacta. Adaptar.
- **Custo computacional cresce inesperadamente**: walk
  duplo provavelmente já existia; verificar. Se cresce,
  documentar e aceitar — bib feature é raramente usada,
  custo aceitável.
- **`state.bib_*` é lido em outros sítios além de
  cite-arm**: P180 inventário reportou 1 consumer
  (cite-arm). Re-verificar. Se outros, migrar
  também ou recuar.
- **Tests existentes que populam `state.bib_*`
  manualmente em testes**: estes podem falhar pós-walk
  puro. Adaptar tests para usar `introspect_with_introspector`
  ou popular ambos directamente.
- **Snapshot tests detectam mudança subtil**: tag agora
  emitida + BibStore populado pode mudar ordem de
  algum output. Investigar; output observable deveria
  ser idêntico.
- **`title` walk descida tem comportamento diferente**:
  walk arm legacy fazia descida em title; preservar.
  Confirmar.
- **`layout_with_introspector` externamente chamado
  com `TagIntrospector::empty()` deliberadamente**:
  algum test ou caller pode passar empty para
  testar fallback. Migração `.G` afecta apenas
  `layout()` legacy. Outros callers preservados.
- **Linter divergência**: ajustar.

---

## Notas operacionais

- **Tamanho**: S. Walk arm simplifica (5 linhas removidas);
  `layout()` legacy alinha (1 linha mudada). 3 tests
  novos.
- **Pré-condição P181I**: walk puro; `layout()` migrado.
  P181I valida E2E que tudo funciona em conjunto e
  fecha lacuna #6.
- **Cláusula gate trivial**: aplicável a custo
  computacional, naming exacto.
- **Gate substancial possível**: se outros consumers
  de `state.bib_*` aparecerem além do cite-arm, recuar
  (decisão Opção B = aceitar regressão é gate
  substancial).
- **Invariante P163 restaurada**: walk puro era
  invariante violada por P159C/F para bib state.
  P181H restaura. Próximas adições futuras de Content
  variants podem replicar padrão (walk puro + tag +
  sub-store).
- **Janela compat encerrada para bib state**: durante
  P181D-G havia path duplo (state legacy + BibStore).
  P181H simplifica — state legacy ainda existe como
  fields (M6 elimina) mas vazios em produção.
- **`layout()` legacy mantém funcionalidade total**:
  cite renderiza correctamente via Introspector
  populado em `introspect_with_introspector`.
  Backward compat 100%.
- **M5 progresso indirecto**: P181H não migra novo
  consumer mas remove dependência walk-mutation que
  bloqueava M6.
