# Passo P181I — Tests E2E + lacuna #6 fechada

Oitavo passo de materialização P181 (após P181B–P181H).
Magnitude **S**.

Valida pipeline completo de bib state e fecha lacuna #6 em
`m1-lacunas-captura.md`. Tests verificam integração de
todos os componentes P181 em conjunto.

Após P181I:
- Lacuna #6 marcada como ✅ Resolvida em P181.
- 3 critérios de fecho (P181A §2.6 Opção 3) verificados.
- Tests E2E codificam invariantes pós-P181 — protegem
  contra regressão futura.

**Pré-condição**: P181H concluído. Pipeline completo:
- `BibStore` em `entities/` (P181B).
- `ElementKind::Bibliography` + `ElementPayload::Bibliography` (P181C).
- `is_locatable` + `extract_payload` arms (P181D).
- `from_tags` arm popula `BibStore` (P181E).
- Trait `Introspector::bib_*_for_key` (P181F).
- Cite-arm consome via Introspector (P181G).
- Walk puro + `layout()` legacy migrado (P181H).

**Restrições**:
- **Não** adicionar funcionalidade nova.
- **Não** modificar componentes P181B-H.
- API pública preservada.
- Tests E2E componente (Opção B) — sem dependência em
  parser/eval. Refino para E2E completo via parser fica
  para futuro.

---

## Sub-passos

### .A Auditoria + decisão E2E componente vs completo

1. Confirmar 3 critérios de fecho P181A §2.6 (Opção 3):
   - **Critério 1**: `01_core/src/entities/bib_store.rs`
     existe.
   - **Critério 2**: `Introspector::bib_entry_for_key` +
     `bib_number_for_key` no trait + impl `TagIntrospector`;
     `from_tags` arm popula `bib_store`.
   - **Critério 3**: Layouter cite-arm consulta via
     `self.introspector.bib_*_for_key(...)`.
   - Verificar literalmente que cada ficheiro contém o
     código esperado.

2. Decidir nível dos tests E2E:

   **Opção A** — completo via parser:
   - `parse(source) → eval → introspect → layout → render`.
   - Realismo total.
   - Dependente de parser/eval estáveis para bib syntax
     vanilla.

   **Opção B** — componente:
   - Construir `Content` directamente (sem parser).
   - Pipeline `Content → walk → from_tags → BibStore →
     layout_with_introspector → render`.
   - Replica padrão P162–P181G (todos usaram Opção B).
   - Sem dependência em parser.

   Sugestão: **B**. Consistência com passos anteriores.

3. Identificar localização dos tests:
   - `01_core/src/rules/layout/tests.rs::p181i_e2e_bib`
     ou módulo dedicado.
   - Ou `01_core/tests/` se preferir integration-style.
   - Decisão local em `.A` conforme convenção
     cristalina.

4. Confirmar localização de
   `m1-lacunas-captura.md`:
   - `00_nucleo/diagnosticos/m1-lacunas-captura.md`.
   - Localizar entrada lacuna #6 (linha 89 conforme
     P181A relatório).

Output: notas internas; sem ficheiro novo.

**Critério de saída**:
- Auditoria limpa.
- 3 critérios verificados literalmente.
- Opção B confirmada.
- Localização tests definida.

### .B Tests E2E (componente)

Em `01_core/src/rules/layout/tests.rs::p181i_e2e_bib` ou
módulo equivalente:

```rust
#[test]
fn e2e_p181i_pipeline_completo_bib_state() {
    // 1. Construir Content com Bibliography + Cite Normal
    let entry_a = make_bib_entry("intro");
    let entry_b = make_bib_entry("methods");
    let content = Content::Sequence {
        children: vec![
            Content::Bibliography {
                entries: vec![entry_a.clone(), entry_b.clone()],
                title: None,
            },
            Content::Cite {
                key: "intro".to_string(),
                supplement: None,
                form: CitationForm::Normal,
            },
            Content::Cite {
                key: "methods".to_string(),
                supplement: None,
                form: CitationForm::Normal,
            },
        ],
    };

    // 2. layout() legacy: walk puro + Introspector populado +
    //    cite-arm consome via Introspector.
    let frame = layout(&content, CounterStateLegacy::empty());

    // 3. Assertions:
    let text = plain_text(&frame);
    assert!(text.contains("[1]"), "cite intro deve renderizar [1]: {}", text);
    assert!(text.contains("[2]"), "cite methods deve renderizar [2]: {}", text);
}

#[test]
fn e2e_p181i_paridade_state_legacy_vazio_em_producao() {
    // Walk puro: state.bib_* permanece vazio durante walk.
    let entry = make_bib_entry("a");
    let content = Content::Bibliography {
        entries: vec![entry.clone()],
        title: None,
    };

    let mut state = CounterStateLegacy::empty();
    let mut locator = Locator::root();
    let mut tags = Vec::new();
    walk(&content, &mut state, &mut locator, &mut tags, None);

    // Walk arm puro garante state legacy vazio.
    assert!(state.bib_entries.is_empty());
    assert!(state.bib_numbers.is_empty());

    // BibStore populado via from_tags.
    let intr = from_tags(&tags);
    assert_eq!(intr.bib_store.len(), 1);
    assert_eq!(intr.bib_number_for_key("a"), Some(1));
}

#[test]
fn e2e_p181i_multi_bibliography_concat_replicado() {
    // Cláusula 2 P181A: extend semantics.
    let content = Content::Sequence {
        children: vec![
            Content::Bibliography {
                entries: vec![
                    make_bib_entry("a"),
                    make_bib_entry("b"),
                ],
                title: None,
            },
            Content::Bibliography {
                entries: vec![
                    make_bib_entry("c"),
                    make_bib_entry("d"),
                ],
                title: None,
            },
        ],
    };

    let (_, intr) = introspect_with_introspector(&content);

    assert_eq!(intr.bib_store.len(), 4);
    assert_eq!(intr.bib_number_for_key("a"), Some(1));
    assert_eq!(intr.bib_number_for_key("b"), Some(2));
    assert_eq!(intr.bib_number_for_key("c"), Some(3));
    assert_eq!(intr.bib_number_for_key("d"), Some(4));
}

#[test]
fn e2e_p181i_or_insert_preserva_primeiro_numero() {
    // Cláusula 3 P181A: or_insert não sobrescreve.
    let content = Content::Sequence {
        children: vec![
            Content::Bibliography {
                entries: vec![make_bib_entry("a")],
                title: None,
            },
            Content::Bibliography {
                entries: vec![
                    make_bib_entry("a"),  // duplicate
                    make_bib_entry("b"),
                ],
                title: None,
            },
        ],
    };

    let (_, intr) = introspect_with_introspector(&content);

    // "a" preserva número original; "b" é novo.
    assert_eq!(intr.bib_number_for_key("a"), Some(1));
    assert_eq!(intr.bib_number_for_key("b"), Some(2));
}
```

Confirmar que tests passam — pipeline completo funciona.

**Critério de saída**:
- Tests E2E escritos e passam.
- Linter passa.

### .C Update `m1-lacunas-captura.md` — fechar lacuna #6

1. Localizar entrada lacuna #6 (linha 89 aprox).

2. Actualizar de "Em curso (P181)" para
   "✅ Resolvida em P181":

   **Forma sugerida**:
   ```markdown
   #6: bib_entries / bib_numbers
   ✅ **Resolvida em P181** (`diagnostico-bib-store-passo-181a.md` +
   relatórios P181B-P181I).

   Mecanismo:
   - `BibStore` sub-store em `entities/bib_store.rs` (P181B).
   - `ElementKind::Bibliography` + `ElementPayload::Bibliography` (P181C).
   - Walk puro restaurado + `layout()` legacy migra para
     `introspect_with_introspector` (P181H).
   - Cite-arm via `Introspector::bib_*_for_key` (P181G).
   - Tests E2E codificam invariantes (P181I).

   Critérios P181A §2.6 (Opção 3) verificados:
   1. ✅ Sub-store existe.
   2. ✅ Trait expõe API; `from_tags` popula.
   3. ✅ Layouter cite-arm consome via Introspector.

   Pendências M6: campos legacy `bib_entries`/`bib_numbers`
   em `CounterStateLegacy` continuam a existir (vazios em
   produção); fallback cite-arm preservado como
   segurança extra. M6 elimina ambos.
   ```

3. Update tabela "Resumo" (se existe — P181A relatório
   §11 reportou linha 112):
   - Lacuna #6 muda para ✅ resolvida.

**Critério de saída**:
- Diagnóstico actualizado.
- Linter passa (não toca código).

### .D Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` — todos os tests
   passam. Δ vs P181H baseline (1733). Estimativa: +4
   (4 tests `.B`).
3. `crystalline-lint .`: zero violations.
4. Tests E2E P181I existem e cobrem:
   - Pipeline completo (Content → walk → from_tags →
     BibStore → cite-arm).
   - Walk puro (state legacy vazio em produção).
   - Multi-Bibliography concat (cláusula 2).
   - or_insert preserva número (cláusula 3).
5. `m1-lacunas-captura.md`:
   - Lacuna #6 marcada como ✅ resolvida.
   - 3 critérios P181A §2.6 documentados como verificados.
6. Walk **NÃO modificado**.
7. `from_tags` **NÃO modificado**.
8. `Introspector` trait **NÃO modificado**.
9. Layouter **NÃO modificado**.
10. Sub-store `BibStore` **NÃO modificado**.
11. Snapshot tests ADR-0033 verdes.
12. Linter passa final.

### .E Encerramento

Escrever
`00_nucleo/materialization/typst-passo-181i-relatorio.md`
com:

- Resumo: pipeline E2E validado; lacuna #6 fechada.
- Confirmação de cada verificação `.D`.
- Decisões registadas em `.A`:
  - Opção B (componente, sem parser).
  - Localização tests.
- Δ tests vs baseline P181H (esperado +4).
- **Estado de P181**: A-I concluídos; J pendente
  (relatório consolidado).
- **Lacuna #6**: ✅ Resolvida em P181.
- **M9 features**: 10/11 (Bibliography conta agora).
- Pendências cumulativas + actualização.
- Estado pós-passo: P181I concluído. P181J desbloqueado.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria + 3 critérios verificados.
2. Tests E2E escritos e passam (`.B`).
3. `m1-lacunas-captura.md` actualizado (`.C`).
4. Verificações `.D` 1-12 passam.
5. Relatório `.E` escrito.
6. Lacuna #6 fechada formalmente.
7. M9 atinge 10/11 features.

---

## O que pode sair errado

- **Helper `make_bib_entry` ausente em layout/tests.rs**:
  importar de from_tags ou criar localmente. Cláusula
  gate trivial.
- **`Content::Sequence` não existe**: usar
  `Content::Stack` ou outro container conforme
  cristalino. Adaptar.
- **`plain_text(&frame)` ausente**: helper de tests
  existente (P181G usou). Localizar e reusar.
- **`introspect_with_introspector` retorna tipo
  diferente do esperado**: confirmar tuple `(state,
  intr)`. Adaptar.
- **Tests numericamente sensíveis**: cite Normal pode
  renderizar `[1]` ou `1` ou `1.` conforme convenção.
  Verificar formato real e ajustar asserts.
- **Tabela "Resumo" em `m1-lacunas-captura.md` tem
  forma diferente**: adaptar conforme estrutura actual.
- **Linter divergência**: ajustar.

---

## Notas operacionais

- **Tamanho**: S. 4 tests E2E + 1 update de diagnóstico.
- **Pré-condição P181J**: lacuna #6 fechada; pipeline
  validado. P181J consolida todos os resultados P181
  num relatório único.
- **Cláusula gate trivial**: aplicável a forma de
  Content::Sequence/Stack, helpers de teste, formato
  exacto de [N].
- **Padrão E2E componente**: replicado de P162-P181G.
  Consistência mantida. E2E completo via parser fica
  para refino futuro (parser stability).
- **M9 atinge 10/11**: contagem de features inclui
  Bibliography agora (lacuna #6 fecha = feature
  bib materializada). Restante: lacuna #4
  (numbering_active) — infraestrutura pronta P171,
  consumer aguarda.
- **Janela compat encerrada**: P181I não toca código de
  produção; apenas valida e documenta. M6 elimina
  campos legacy quando lacuna #4 também fechar e M5
  saturar.
