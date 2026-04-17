# Passo 65 — Convergência de Layout (Algoritmo Fixpoint) e DEBT-17

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/rules/layout/mod.rs` — Onde a função orquestradora `layout()` reside.
- `03_infra/src/integration_tests.rs` — Onde a lógica de 3 passagens foi colocada no Passo 63.
- `01_core/src/entities/layout_types.rs` — Onde `PagedDocument` e `extracted_label_pages` estão definidos.

Pré-condição: `cargo test` — 651 L1 + 123 L3 + 50 parity, zero violations.
DEBT-16 encerrado.

---

## Contexto

No Passo 63, o cálculo das páginas na TOC usou 3 passagens fixas. O problema
é que inserir os números de página na TOC pode aumentar a altura da TOC — o
que empurra os títulos para a página seguinte. Nesse caso, os números da
Passagem 2 ficam errados na Passagem 3.

O algoritmo fixpoint resolve isto: o layout corre em ciclo até que o mapa de
páginas de uma iteração seja igual ao da iteração anterior (convergência). Um
limite de iterações (5) evita ciclos infinitos em documentos patológicos.

**Nota arquitectural:** a lógica de convergência pertence a L1, não a L3.
Qualquer consumidor da biblioteca deve receber o documento já convergido — sem
precisar de orquestrar passagens manualmente. O `compile_to_pdf` em L3 voltará
a ser linear.

---

## Diagnósticos obrigatórios antes de codificar

```bash
# 1. Localizar onde a orquestração de passagens está actualmente
grep -n "layout(\|Passagem\|draft\|final_state" \
  03_infra/src/integration_tests.rs | head -20

# 2. Confirmar assinatura actual de layout()
grep -n "^pub fn layout" 01_core/src/rules/layout/mod.rs | head -5

# 3. Ver o que layout() faz actualmente com initial_state
grep -n "initial_state\|resolved_labels\|headings_for_toc\|label_pages" \
  01_core/src/rules/layout/mod.rs | head -20

# 4. Confirmar que extracted_label_pages existe em PagedDocument
grep -n "extracted_label_pages" \
  01_core/src/entities/layout_types.rs | head -5
```

Reportar o output completo antes de continuar. A resposta à questão 3 é
crítica: confirmar exactamente quais campos do `CounterState` são copiados
para o Layouter actualmente — o fixpoint vai copiar os mesmos campos mais
`label_pages`, e limpar `hierarchical` e `flat` em cada iteração.

---

## Tarefa 1 — Motor de Convergência em `layout()` (L1)

Em `01_core/src/rules/layout/mod.rs`, substituir a função `layout()` actual
pelo ciclo de fixpoint:

```rust
/// Layout com convergência de fixpoint.
///
/// Se não houver títulos catalogados para a TOC (`headings_for_toc` vazio),
/// corre uma única passagem — o fixpoint de páginas só serve a TOC.
/// Caso contrário, itera até convergência (máximo 5 vezes).
pub fn layout(content: &Content, initial_state: CounterState) -> PagedDocument {
    // ── Short-circuit: sem TOC, não há necessidade de fixpoint ──────────
    // A condição correcta é a presença do nó Outline, não a existência de
    // títulos. Um documento com 50 capítulos mas sem #outline() tem
    // headings_for_toc com 50 entradas — mas nunca vai ler label_pages,
    // por isso o fixpoint seria uma passagem desperdiçada.
    if !initial_state.has_outline {
        let mut l = Layouter::new(/* métricas actuais */);
        l.counter.resolved_labels  = initial_state.resolved_labels;
        l.counter.headings_for_toc = initial_state.headings_for_toc;
        l.counter.numbering_active = initial_state.numbering_active;
        // label_pages começa vazio — sem páginas a resolver
        l.layout_content(content);
        return l.finish();
    }

    // ── Fixpoint: documentos com TOC ─────────────────────────────────────
    const MAX_ITERATIONS: usize = 5;

    // Mapa de páginas da iteração anterior — apenas para leitura pelo outline.rs.
    // NÃO é o mesmo campo onde o references.rs escreve durante o layout.
    // Separação leitura/escrita: o Layouter lê de `known_page_numbers` e
    // escreve em `label_pages` (que começa vazio em cada iteração via Layouter::new()).
    let mut known_page_numbers: HashMap<Label, usize> = HashMap::new();
    let mut final_doc: Option<PagedDocument> = None;

    for _ in 0..MAX_ITERATIONS {
        let mut l = Layouter::new(/* métricas actuais */);

        // Estado base da introspecção — copiado em cada iteração.
        l.counter.resolved_labels  = initial_state.resolved_labels.clone();
        l.counter.headings_for_toc = initial_state.headings_for_toc.clone();
        l.counter.numbering_active = initial_state.numbering_active.clone();

        // Injectar os números de página da iteração anterior para leitura.
        // O outline.rs lê deste campo ao construir as linhas da TOC.
        // O label_pages (onde references.rs escreve) começa vazio via Layouter::new().
        l.counter.known_page_numbers = known_page_numbers.clone();

        l.layout_content(content);
        let doc = l.finish();

        // Convergência: mapa de páginas gerado == mapa da iteração anterior?
        if doc.extracted_label_pages == known_page_numbers {
            return doc;
        }

        // Actualizar para a próxima iteração.
        known_page_numbers = doc.extracted_label_pages.clone();
        final_doc = Some(doc);
    }

    // Limite atingido sem convergência (DEBT-17: caso patológico).
    // Retornar o documento da última iteração — melhor esforço sem warnings
    // em L1 (log:: não é permitido sem ADR; erros ficam em SourceDiagnostic).
    final_doc.expect("layout: deve produzir pelo menos um documento")
}
```

**Campo `known_page_numbers` e `has_outline` em `CounterState`:** adicionar junto com `label_pages`:

```rust
pub struct CounterState {
    // ... campos existentes ...
    pub label_pages:         HashMap<Label, usize>,  // escrita pelo references.rs
    pub known_page_numbers:  HashMap<Label, usize>,  // leitura pelo outline.rs
    /// Verdadeiro se o documento contém pelo menos um nó Content::Outline.
    /// Determina se o fixpoint de páginas é necessário.
    /// Populado pela introspecção — não pela contagem de títulos.
    pub has_outline: bool,
}
```

Em `outline.rs`, substituir a leitura de `label_pages` por `known_page_numbers`:

```rust
let page_num = layouter.counter.known_page_numbers.get(&label)
    .map(|p| format!("  {}", p))
    .unwrap_or_default();
```

Em `introspect.rs`, activar `has_outline` no braço terminal de `Content::Outline`:

```rust
Content::Outline => {
    state.has_outline = true;
    // Outline não altera contadores — apenas sinaliza que o fixpoint é necessário.
},
```

Em `references.rs`, o código actual (`label_pages.insert(...)`) mantém-se inalterado — escreve no campo correcto.

Em `PagedDocument::finish()`, copiar de `label_pages` para `extracted_label_pages` como antes. `known_page_numbers` e `has_outline` não são exportados.

---

## Tarefa 2 — Limpeza em L3

Em `03_infra/src/integration_tests.rs` (ou onde o orquestrador principal
reside), remover a lógica manual de Passagem 2 e Passagem 3. O pipeline
volta a ser linear:

```rust
// ANTES (Passo 63 — 3 passagens manuais em L3):
let intro_state = introspect(&content);
let draft_doc   = layout(&content, intro_state.clone());
let mut final_state = intro_state;
final_state.label_pages = draft_doc.extracted_label_pages;
let final_doc = layout(&content, final_state);
export_pdf(&final_doc)

// DEPOIS (Passo 65 — convergência interna a L1):
let intro_state = introspect(&content);
let doc = layout(&content, intro_state);  // fixpoint acontece aqui
export_pdf(&doc)
```

Correr `cargo test` depois desta limpeza para confirmar que os testes L3
continuam a passar com o pipeline simplificado.

---

## Tarefa 3 — Testes

### Testes L1 — Convergência

```rust
#[test]
fn layout_converge_sem_ciclo_infinito() {
    use crate::rules::introspect::introspect;
    use crate::rules::layout::layout;

    let content = Content::Sequence(vec![
        Content::SetHeadingNumbering { active: true },
        Content::Outline,
        Content::heading(1, Content::text("Capítulo 1")),
        Content::heading(2, Content::text("Secção 1.1")),
    ]);

    let state = introspect(&content);
    // Se o fixpoint tiver defeito, isto entra em loop até MAX_ITERATIONS
    // e retorna o último documento — não deve panic.
    let doc = layout(&content, state);

    let text = doc.plain_text();
    assert!(text.contains("Capítulo 1"), "título deve aparecer: {:?}", text);
    assert!(text.contains("Índice")    || text.contains("ndice"),
        "TOC deve aparecer: {:?}", text);
}

#[test]
fn layout_documento_sem_toc_usa_curto_circuito() {
    use crate::rules::introspect::introspect;
    use crate::rules::layout::layout;

    // Documento COM títulos mas SEM #outline(). O vetor headings_for_toc
    // terá entradas, mas has_outline é false — o short-circuit evita o loop.
    // Prova que a condição correcta é has_outline, não headings_for_toc.is_empty().
    let content = Content::Sequence(vec![
        Content::SetHeadingNumbering { active: true },
        Content::heading(1, Content::text("Introdução")),
        Content::heading(2, Content::text("Motivação")),
        Content::text("Texto sem índice."),
    ]);

    let state = introspect(&content);
    // has_outline deve ser false porque não há Content::Outline no documento
    assert!(!state.has_outline, "sem Outline no documento, has_outline deve ser false");

    let doc = layout(&content, state);
    assert!(!doc.pages.is_empty(), "documento deve ter páginas");
}

#[test]
fn layout_com_labels_produz_extracted_label_pages() {
    use crate::entities::label::Label;
    use crate::rules::introspect::introspect;
    use crate::rules::layout::layout;

    let content = Content::Sequence(vec![
        Content::Labelled {
            label:  Label("sec1".to_string()),
            target: Box::new(Content::heading(1, Content::text("Secção"))),
        },
    ]);

    let state = introspect(&content);
    let doc = layout(&content, state);

    assert!(
        doc.extracted_label_pages.contains_key(&Label("sec1".to_string())),
        "extracted_label_pages deve conter a label após convergência"
    );
}
```

### Testes L3 — Pipeline simplificado

```rust
#[test]
fn pipeline_toc_paginada_pipeline_linear() {
    // Confirmar que o pipeline L3 é agora linear (sem passagens manuais)
    // e que a TOC contém números de página.
    let (world, _dir) = world_from_str(
        "#set heading(numbering: \"1.\")\n\
         #outline()\n\
         = Introdução\n\
         = Conclusão"
    );
    let pdf = compile_to_pdf(&world);
    assert!(!pdf.is_empty(), "PDF com TOC paginada não deve estar vazio");
}

#[test]
fn pipeline_sem_toc_nao_regrediu() {
    // Regressão: documentos sem TOC não devem ser afectados pelo fixpoint.
    let (world, _dir) = world_from_str(
        "= Introdução\n\
         Texto simples sem índice."
    );
    let pdf = compile_to_pdf(&world);
    assert!(!pdf.is_empty());
}
```

---

## Verificação final

```bash
cargo clippy --fix --allow-dirty --allow-no-vcs
cargo test
crystalline-lint --fix-hashes .
crystalline-lint .
```

Critérios de conclusão:
- [ ] Lógica de 3 passagens removida de L3.
- [ ] `CounterState` tem `has_outline: bool` (default `false`).
- [ ] `introspect.rs` activa `has_outline = true` no braço `Content::Outline`.
- [ ] `layout()` em L1 verifica `!initial_state.has_outline` para o short-circuit
  (não `headings_for_toc.is_empty()` — documentos com títulos mas sem Outline
  são igualmente elegíveis ao short-circuit).
- [ ] `layout()` em L1 tem o ciclo `for` com `MAX_ITERATIONS = 5` para
  documentos com TOC.
- [ ] `CounterState` tem dois campos distintos: `label_pages` (escrita pelo
  `references.rs`) e `known_page_numbers` (leitura pelo `outline.rs`).
- [ ] Em cada iteração, `label_pages` começa vazio via `Layouter::new()` —
  nunca é copiado do estado anterior.
- [ ] Em cada iteração, `known_page_numbers` recebe o `extracted_label_pages`
  da iteração anterior.
- [ ] O ciclo compara `doc.extracted_label_pages == known_page_numbers`.
- [ ] Sem `log::` em L1 — limite atingido retorna documento de melhor esforço.
- [ ] Testes L3 do Passo 63 (TOC paginada) continuam a passar.
- [ ] DEBT-17 marcado como **encerrado** em `01_core/DEBT.md`.
- [ ] Zero violações no linter e no clippy.

---

## Ao terminar, reportar

**Do diagnóstico:**
- Se a lógica de 3 passagens estava em `compile_to_pdf` ou noutra função em L3.
- Quais campos o `layout()` actual copiava do `initial_state` — confirmar que
  o fixpoint copia os mesmos mais `label_pages`.

**Da implementação:**
- Quantas iterações o corpus de teste precisou para convergir (1, 2, ou mais).
- Se o documento sem TOC convergiu na iteração 0 (mapa vazio == mapa vazio).
- Se `log::` estava disponível ou foi omitido.
- Número total de testes e zero violations confirmados.

**Go/No-Go para o Passo 66:**
- **GO — DEBT-18 (contexto temporal na TOC):** com DEBT-17 encerrado, Passo 66
  aborda títulos com `CounterDisplay` na TOC que mostram valores incorrectos.
- **GO — expansão da stdlib:** com o desacoplamento completo (DEBT-16) e a
  convergência estável (DEBT-17), Passo 66 pode adicionar funções nativas com
  named args sem risco de acoplamento.
- **NO-GO — fixpoint não convergiu no corpus de teste:** se os testes revelarem
  que `MAX_ITERATIONS = 5` é insuficiente; Passo 66 analisa o documento
  patológico e decide se aumentar o limite ou implementar detecção de ciclo.
