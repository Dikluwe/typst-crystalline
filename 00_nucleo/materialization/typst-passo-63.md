# Passo 63 — TOC Paginada e Congelamento de AST (DEBT-12 e DEBT-13)

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/rules/layout/outline.rs` — Geração da TOC actual (sem páginas).
- `01_core/src/rules/layout/references.rs` — Braço `Labelled` actual.
- `01_core/src/entities/counter_state.rs` — Onde o mapa de páginas será adicionado.
- `01_core/src/rules/layout/mod.rs` — Orquestrador do layout físico.

Pré-condição: `cargo test` — 631 L1 + 121 L3 + 50 parity, zero violations.
DEBT-10 encerrado. DEBT-16 registado (acoplamento eval/stdlib).

---

## Contexto

Este passo endereça dois DEBTs em simultâneo com uma abordagem coordenada:

**DEBT-12 (números de página na TOC):** a TOC actual lista títulos com a sua
numeração lógica, mas não mostra em que página cada secção começa. Resolver
isto requer uma terceira passagem de layout — o layouter físico corre duas
vezes, e só na segunda é que a TOC tem acesso ao mapa de páginas gerado na
primeira.

**DEBT-13 (efeitos colaterais duplicados):** a TOC clona o `Content` do
título para preservar formatação. Se o título contiver `CounterUpdate`, o
contador avança duas vezes. Resolver isto requer um modo read-only no
`CounterState` que bloqueia mutações durante a renderização da TOC.

As duas correcções são independentes mas complementares: DEBT-13 deve ser
resolvido antes de DEBT-12, porque a segunda passagem de layout percorre a
TOC novamente — com read-only, a segunda passagem é segura.

**Aviso de fixpoint:** se os números de página na TOC (Passagem 3) aumentarem
a altura da TOC ao ponto de empurrar títulos para a página seguinte, os
números ficariam errados. O Typst original resolve isto com um algoritmo de
fixpoint (iteração até convergência). Neste passo, 3 passagens fixas são
suficientes para fechar o DEBT-12 — o caso de degradação é registado como
DEBT-17 e endereçado em passos futuros.

---

## Diagnósticos obrigatórios antes de codificar

```bash
# 1. Confirmar se o Layouter tem acesso ao número da página actual
grep -n "current_page\|page_number\|pages\.len" \
  01_core/src/rules/layout/mod.rs | head -10

# 2. Verificar estrutura de PagedDocument — tem acesso às páginas individuais?
grep -n "struct PagedDocument\|pub pages" \
  01_core/src/entities/layout_types.rs | head -5

# 3. Ver como layout() retorna o CounterState ao orquestrador
#    (necessário para extrair label_pages após a Passagem 2)
grep -n "^pub fn layout\|counter\|initial_state" \
  01_core/src/rules/layout/mod.rs | head -15

# 4. Confirmar como o Layouter sabe em que página está actualmente
grep -n "self\.pages\|self\.current\|flush_line\|new_page" \
  01_core/src/rules/layout/mod.rs | head -20
```

Reportar o output completo antes de continuar. A resposta à questão 1 é
crítica: se o Layouter não expõe `current_page_number()`, a lógica de
registo em `references.rs` precisa de inferir a página a partir de
`self.pages.len()` no momento do encontro com a Label.

---

## Tarefa 1 — Registo de DEBT-16 e DEBT-17 (L0)

Registar em `00_nucleo/DEBT.md` antes de qualquer código:

```markdown
### DEBT-16 — Acoplamento do Avaliador à Stdlib (Passo 62)
A função `figure()` foi implementada como interceptador em `eval.rs` porque
`NativeFunc` não suporta argumentos nomeados (só aceita `&[Value]`). Cada
interceptador adicionado ao avaliador aumenta o acoplamento e degrada o ciclo
de avaliação. Resolução: refactorizar `NativeFunc` para suportar
`(args: &[Value], named: &IndexMap<EcoString, Value>)` e remover todos os
interceptadores do `eval.rs`.

### DEBT-17 — Fixpoint da TOC (Passo 63)
A orquestração de 3 passagens é suficiente para a maioria dos documentos, mas
não é correcta em geral: se os números de página na TOC (Passagem 3) aumentarem
a altura da TOC ao ponto de empurrar secções para a página seguinte, os números
ficarão errados. O Typst original resolve com iteração até convergência (fixpoint).
Resolução futura: substituir as 3 passagens fixas por um loop que corre até que
`label_pages` não mude entre iterações.

### DEBT-18 — Perda de Contexto Temporal em AST Clonado na TOC (Passo 63)
O modo `is_readonly` bloqueia mutações de contadores durante a renderização da
TOC, mas não resolve o problema das leituras (`CounterDisplay`). Exemplo: o
utilizador escreve `= Capítulo #counter("cap").display()`. Na página 5, o
contador vale 3 e o título renderiza "Capítulo 3". Na TOC (página 1), o
Layouter avalia o `CounterDisplay` com o contador ainda em 0, e lista
"Capítulo 0 .............. pág. 5".

Causa raiz: ao clonar o AST puro para a TOC, o nó é arrancado do seu contexto
temporal. O `is_readonly` impede que a TOC estrague o futuro, mas não permite
que a TOC "veja" o futuro.

Resolução futura: na Passagem 2 (draft), transformar os títulos em geometria
estática resolvida (texto + formatação, sem nós de estado dinâmico) em vez de
passar o AST cru para `headings_for_toc`. Isto exige que a introspecção
materialize o texto visual dos títulos durante a Passagem 1, em vez de clonar
o AST para avaliação posterior. Qualquer nó que leia estado dinâmico
(`CounterDisplay`, `counter().get()`) renderizará valores incorrectos na TOC
até esta arquitectura ser implementada.
```

---

## Tarefa 2 — Motor de Congelamento (DEBT-13)

### 2a — Flag `is_readonly` no `CounterState`

Em `01_core/src/entities/counter_state.rs`:

```rust
#[derive(Debug, Clone, Default)]
pub struct CounterState {
    pub hierarchical:      HashMap<String, Vec<usize>>,
    pub flat:              HashMap<String, usize>,
    pub numbering_active:  HashMap<String, bool>,
    pub resolved_labels:   HashMap<Label, String>,
    pub headings_for_toc:  Vec<(Label, Content, usize)>,
    pub auto_label_counter: usize,
    pub label_pages:       HashMap<Label, usize>,  // adicionado na Tarefa 3
    /// Modo read-only: bloqueia mutações de contadores.
    /// Activado durante a renderização de clones de AST na TOC para evitar
    /// que nós CounterUpdate disparem duas vezes (DEBT-13).
    pub is_readonly: bool,
}
```

Actualizar todos os métodos de mutação para respeitar a flag:

```rust
pub fn step_hierarchical(&mut self, key: &str, level: usize) {
    if self.is_readonly { return; }
    // ... lógica existente ...
}

pub fn step_flat(&mut self, key: &str) {
    if self.is_readonly { return; }
    // ... lógica existente ...
}

pub fn update_flat(&mut self, key: &str, value: usize) {
    if self.is_readonly { return; }
    // ... lógica existente ...
}
```

`is_readonly` não impede leituras — apenas `step_*` e `update_*`. Os métodos
`format_hierarchical`, `get_flat`, `is_numbering_active` e `resolved_labels`
continuam acessíveis normalmente.

### 2b — Activar read-only em `outline.rs`

Em `01_core/src/rules/layout/outline.rs`, envolver a renderização de cada
linha da TOC com activação e restauração da flag:

```rust
for (label, body_content, level) in entries {
    let indent = "  ".repeat(level.saturating_sub(1));

    layouter.counter.is_readonly = true;

    let line = Content::Sequence(vec![
        Content::text(indent),
        Content::Ref { target: label.clone() },
        Content::text(" "),
        body_content.clone(),
        // Número de página — string vazia na Passagem 2, número real na Passagem 3
        {
            let page_num = layouter.counter.label_pages.get(&label)
                .map(|p| format!("  {}", p))
                .unwrap_or_default();
            Content::text(page_num)
        },
        Content::Linebreak,
    ]);

    // is_readonly mantém-se activo durante layout_node — é exactamente durante
    // a execução do layout que CounterUpdate seria disparado. Desactivar antes
    // de layout_node tornaria a protecção ineficaz.
    layouter.layout_node(&line, layouter.style);
    layouter.counter.is_readonly = false;  // Restaurar DEPOIS do layout
}
```

---

## Tarefa 3 — Mapa de Páginas (DEBT-12)

### 3a — Campo `label_pages` no `CounterState`

Já incluído na Tarefa 2a. Nenhuma acção adicional em `counter_state.rs`.

### 3b — Método `current_page_number()` no Layouter

O Layouter precisa de saber em que página está quando processa uma Label.
Dependendo do output do diagnóstico, usar uma destas abordagens:

**Abordagem A** (se o Layouter tem `self.pages` como `Vec<Frame>`):

```rust
// Em layout/mod.rs ou layout/references.rs:
pub(super) fn current_page_number(&self) -> usize {
    // Páginas são 1-indexed: a página actual é a próxima a ser finalizada.
    self.pages.len() + 1
}
```

**Abordagem B** (se o Layouter usa um cursor de página explícito):
Usar o campo existente directamente.

Registar no relatório qual abordagem foi usada.

### 3c — Registo da página em `references.rs`

Em `01_core/src/rules/layout/references.rs`, no braço `Content::Labelled`,
registar a página depois de fazer o layout do target — o elemento só sabe em
que página aterrou após ser processado pelo motor geométrico:

```rust
pub fn layout_labelled<M: FontMetrics>(
    layouter: &mut Layouter<M>,
    target:   &Content,
    label:    &Label,
) {
    // Layout primeiro — o target pode forçar uma quebra de página.
    layouter.layout_node(target, layouter.style);

    // Registar a página DEPOIS do layout: o elemento já aterrou na sua página
    // final. Registar antes resultaria no número da página anterior quando
    // o target força uma quebra.
    let page = layouter.current_page_number();
    layouter.counter.label_pages.insert(label.clone(), page);
}
```

---

## Tarefa 4 — Orquestração em 3 Passagens (L3)

Em `03_infra/src/integration_tests.rs` (ou onde `compile_to_pdf` está
implementado), actualizar a orquestração:

```rust
pub fn compile_to_pdf(world: &SystemWorld) -> Vec<u8> {
    let source  = world.source(world.main()).unwrap();
    let module  = eval(world, &source).unwrap();
    let content = module.content().unwrap();

    // ── Passagem 1 — Introspecção ──────────────────────────────────────
    let intro_state = introspect(&content);

    // ── Passagem 2 — Layout Draft ──────────────────────────────────────
    // Gera as posições das páginas. A TOC não tem números de página ainda.
    let draft_state = intro_state.clone();
    let draft_doc   = layout(&content, draft_state);

    // Extrair o mapa de páginas gerado durante o layout draft.
    // layout() precisa de retornar o CounterState final — ver 4a.
    let label_pages = draft_doc.counter_state.label_pages.clone();

    // ── Passagem 3 — Layout Final ──────────────────────────────────────
    let mut final_state = intro_state;
    final_state.label_pages = label_pages;  // TOC terá números reais
    let final_doc = layout(&content, final_state);

    export_pdf(&final_doc)
}
```

### 4a — Expor as páginas das labels sem alterar a assinatura de `layout()`

Alterar a assinatura de `layout()` para retornar um par quebraria dezenas de
call sites em L1 e L3. A abordagem correcta é adicionar o mapa directamente
à estrutura de retorno existente.

Em `01_core/src/entities/layout_types.rs`, adicionar o campo ao `PagedDocument`:

```rust
pub struct PagedDocument {
    pub pages: Vec<Frame>,
    /// Mapa de labels para o número de página onde aterraram.
    /// Populado pela Passagem 2 (draft) e usado pela Passagem 3 (final).
    /// Vazio por defeito — só tem dados após `layout()` com labels no documento.
    pub extracted_label_pages: HashMap<Label, usize>,
}
```

No final de `layout()` em `layout/mod.rs`, antes de retornar, copiar o mapa:

```rust
pub fn layout(content: &Content, initial_state: CounterState) -> PagedDocument {
    let mut l = Layouter::new(/* métricas actuais */);
    l.counter.resolved_labels  = initial_state.resolved_labels;
    l.counter.headings_for_toc = initial_state.headings_for_toc;
    l.layout_content(content);
    let mut doc = l.finish();
    // Expor o mapa de páginas sem mudar a assinatura de layout().
    doc.extracted_label_pages = l.counter.label_pages;  // ou via finish()
    doc
}
```

No orquestrador em L3, aceder os dados via `draft_doc.extracted_label_pages`:

```rust
// Passagem 2 — draft
let draft_doc = layout(&content, intro_state.clone());

// Transferir o mapa de páginas para o estado final
let mut final_state = intro_state;
final_state.label_pages = draft_doc.extracted_label_pages;

// Passagem 3 — final
let final_doc = layout(&content, final_state);
```

---

## Tarefa 5 — Testes

### Testes L1 — Motor de congelamento

```rust
#[test]
fn counter_state_readonly_bloqueia_step_flat() {
    let mut state = CounterState::new();
    state.is_readonly = true;
    state.step_flat("equation");
    assert_eq!(state.get_flat("equation"), 0,
        "step_flat não deve avançar em modo read-only");
}

#[test]
fn counter_state_readonly_permite_leitura() {
    let mut state = CounterState::new();
    state.step_flat("equation");  // avança antes de activar read-only
    state.is_readonly = true;
    assert_eq!(state.get_flat("equation"), 1,
        "get_flat deve funcionar mesmo em modo read-only");
}

#[test]
fn counter_state_readonly_bloqueia_step_hierarchical() {
    let mut state = CounterState::new();
    state.is_readonly = true;
    state.step_hierarchical("heading", 1);
    assert_eq!(state.format_hierarchical("heading"), None,
        "step_hierarchical não deve avançar em modo read-only");
}
```

### Testes L1 — Mapa de páginas

```rust
#[test]
fn layout_regista_pagina_de_label() {
    use crate::entities::label::Label;
    use crate::rules::introspect::introspect;
    use crate::rules::layout::layout;

    let content = Content::Sequence(vec![
        Content::Labelled {
            label:  Label("sec1".to_string()),
            target: Box::new(Content::heading(1, Content::text("Introdução"))),
        },
    ]);

    let state = introspect(&content);
    let (_, final_state) = layout(&content, state);

    assert!(
        final_state.label_pages.contains_key(&Label("sec1".to_string())),
        "label_pages deve conter a label processada"
    );
}
```

### Testes L3 — TOC com números de página

```rust
#[test]
fn pipeline_toc_com_paginas_nao_causa_panico() {
    // A 3ª passagem não deve causar panic mesmo que a TOC seja maior
    // com os números de página (caso de degradação DEBT-17).
    let (world, _dir) = world_from_str(
        "#set heading(numbering: \"1.\")\n\
         #outline()\n\
         = Introdução\n\
         == Motivação\n\
         = Conclusão"
    );
    let pdf = compile_to_pdf(&world);
    assert!(!pdf.is_empty(), "PDF com TOC paginada não deve estar vazio");
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
- [ ] DEBT-16, DEBT-17 e DEBT-18 registados em `00_nucleo/DEBT.md` antes de qualquer código.
- [ ] `is_readonly: bool` adicionado ao `CounterState`.
- [ ] `step_hierarchical`, `step_flat`, `update_flat` respeitam `is_readonly`.
- [ ] `outline.rs` activa `is_readonly` antes de construir cada linha da TOC
  e restaura antes de `layout_node`.
- [ ] `label_pages: HashMap<Label, usize>` adicionado ao `CounterState`.
- [ ] `current_page_number()` implementado no Layouter (abordagem A ou B).
- [ ] `references.rs` regista a página de cada Label antes de layoutar o target.
- [ ] `extracted_label_pages: HashMap<Label, usize>` adicionado a `PagedDocument`
  (sem alterar a assinatura de `layout()`).
- [ ] `layout()` copia `l.counter.label_pages` para `doc.extracted_label_pages` antes de retornar.
- [ ] Orquestrador em L3 usa 3 passagens: introspecção → draft → final.
- [ ] DEBT-12 marcado como **encerrado** em `00_nucleo/DEBT.md`.
- [ ] DEBT-13 marcado como **encerrado** (mitigado) em `00_nucleo/DEBT.md`.
- [ ] Zero violações no linter e no clippy.

---

## Ao terminar, reportar

**Do diagnóstico:**
- Se o Layouter tinha `current_page_number()` ou foi necessário inferir de
  `self.pages.len()` (qual abordagem A ou B).
- Se `layout()` já retornava alguma forma de estado, ou se a mudança de
  assinatura para `(PagedDocument, CounterState)` quebrou muitos call sites.

**Da implementação:**
- Se o posicionamento do `is_readonly = false` antes de `layout_node` causou
  alguma duplicação de efeitos nos testes.
- Se a Passagem 2 (draft) gerou uma TOC de altura diferente da Passagem 3
  (final com páginas) — indicador de DEBT-17 activo.
- Número total de testes e zero violations confirmados.

**Go/No-Go para o Passo 64:**
- **GO — DEBT-16 (NativeFunc com named args):** com DEBT-12 e DEBT-13
  encerrados, Passo 64 refactoriza `NativeFunc` para suportar named args e
  remove os interceptadores de `eval.rs`, eliminando o acoplamento.
- **GO — DEBT-17 (fixpoint da TOC):** se os testes revelarem que 3 passagens
  são insuficientes para um corpus real, Passo 64 implementa o loop de
  convergência antes de avançar.
- **NO-GO — mudança de assinatura de `layout()` quebrou L3:** se os call
  sites em L3 são demasiados para actualizar atomicamente; Passo 64 resolve
  a interface antes de avançar.
