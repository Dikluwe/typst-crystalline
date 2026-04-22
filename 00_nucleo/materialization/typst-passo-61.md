# Passo 61 — Tabela de Conteúdos (TOC) e Decomposição do Layouter

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/rules/introspect.rs` — Onde os títulos serão catalogados.
- `01_core/src/entities/counter_state.rs` — Receberá o vector de títulos.
- `01_core/src/rules/layout.rs` — Ficheiro a decompor em submódulos.

Pré-condição: `cargo test` — 619 L1 + 118 L3 + 50 parity, zero violations.
O sistema resolve referências forward e backward em duas passagens.

---

## Contexto

Este passo tem dois objectivos sequenciais com uma dependência estrita
entre eles: a decomposição (Tarefas 1–2) deve estar compilando antes de
adicionar a TOC (Tarefas 3–5), porque a TOC usa código que vai residir
nos submódulos criados na decomposição.

**Objectivo A — Pagar DEBT-11:** decompor `layout.rs` em submódulos
com responsabilidades claras. O linter exige um L0 por ficheiro, por isso
os prompts L0 são criados antes dos ficheiros de código.

**Objectivo B — Implementar `Content::Outline`:** a TOC usa a Passagem 1
(introspecção) para catalogar os títulos e a Passagem 2 (layout) para os
desenhar. A introspecção gera uma label automática por heading — necessária
porque o utilizador não escreve `<label>` em todos os títulos.

**Limitação intencional (DEBT-12):** a TOC lista numeração lógica das
secções mas não números de página. A paginação real só é conhecida após
o layout físico, o que exige uma terceira passagem ou retorno de dados do
layout para a introspecção — arquitetura para passos futuros.

---

## Diagnósticos obrigatórios antes de codificar

```bash
# 1. Tamanho actual de layout.rs — confirmar necessidade de divisão
wc -l 01_core/src/rules/layout.rs

# 2. Verificar se Content::Outline já existe
grep -n "Outline\|Toc\b" 01_core/src/entities/content.rs | head -5

# 3. Listar prompts de layout existentes em L0
ls -l 00_nucleo/prompts/rules/layout*.md

# 4. Confirmar grupos de braços no layout_content para guiar a extracção
grep -n "Content::" 01_core/src/rules/layout.rs | head -40
```

Reportar o output antes de continuar. A resposta à questão 4 determina
como agrupar os braços nos submódulos — a divisão deve seguir a estrutura
real do código, não uma estrutura idealizada.

---

## Tarefa 1 — Prompts L0 para Submódulos (L0)

Criar os prompts antes de tocar no código. O linter V7 dispara se o
ficheiro de código existir sem prompt associado.

### `00_nucleo/prompts/rules/layout_counters.md`

```markdown
# L0 — Layout: Contadores e Numeração

## Módulo
`01_core/src/rules/layout/counters.rs`

## Propósito
Encapsula os braços do Layouter que alteram ou exibem o estado de
contadores: `SetHeadingNumbering`, `CounterUpdate`, `CounterDisplay`.
Funções chamadas por `layout.rs` (orquestrador).

## Regras de negócio
- `SetHeadingNumbering { active }` → muta `self.counter.numbering_active`.
- `CounterUpdate { key, action }` → delega em `step_flat`/`update_flat`/
  `step_hierarchical`.
- `CounterDisplay { kind }` → lê o estado actual e gera `Content::text`.
- Nenhuma destas funções gera geometria de página directamente.

## Critérios de verificação
- `CounterUpdate(Update(5))` → `counter.get_flat("equation") == 5`.
- `CounterDisplay("heading")` → texto contém o número formatado.
```

### `00_nucleo/prompts/rules/layout_references.md`

Se este ficheiro já existir do Passo 59 com conteúdo diferente, expandir
a secção "Módulo" para reflectir a nova localização:

```markdown
# L0 — Layout: Referências e Labels

## Módulo
`01_core/src/rules/layout/references.rs`

## Propósito
Encapsula os braços `Ref` e `Labelled`. Consulta `resolved_labels`
injectado pela introspecção (Passagem 1). Não escreve em `resolved_labels`
— essa escrita foi removida no Passo 60 e pertence apenas a `introspect.rs`.

## Regras de negócio
- `Labelled { target, label: _ }` → layout transparente do target apenas.
- `Ref { target }` → consulta `self.counter.resolved_labels`; se encontrar,
  desenha o texto resolvido; se não, desenha `@nome` (nunca panic).

## Critérios de verificação
- `Ref` com label existente → texto resolvido no plain_text.
- `Ref` com label inexistente → `@nome` no plain_text, sem panic.
```

### `00_nucleo/prompts/rules/layout_outline.md`

```markdown
# L0 — Layout: Tabela de Conteúdos

## Módulo
`01_core/src/rules/layout/outline.rs`

## Propósito
Encapsula o braço `Content::Outline`. Lê `headings_for_toc` do
`CounterState` injectado e gera a sequência visual da TOC.

## Regras de negócio
- Não faz introspecção — apenas consome `headings_for_toc` já populado.
- Gera um `Content::Sequence` com um heading de nível 1 ("Índice") e
  uma linha por título, indentada pelo nível.
- Cada linha usa `Content::Ref` apontando para a label automática gerada
  pela introspecção — o texto resolvido já estará em `resolved_labels`.
- Não calcula números de página (DEBT-12).

## Critérios de verificação
- Documento com 3 headings → TOC tem 3 linhas após o "Índice".
- Heading de nível 2 → linha indentada (contém espaços de indentação).
- Ausência de headings → TOC exibe apenas o título "Índice".
```

```bash
git add 00_nucleo/prompts/rules/layout_counters.md \
        00_nucleo/prompts/rules/layout_references.md \
        00_nucleo/prompts/rules/layout_outline.md
crystalline-lint --fix-hashes .
```

---

## Tarefa 2 — Decomposição de `layout.rs` (L1)

**Estratégia:** criar os submódulos como ficheiros novos com os braços
relevantes, e substituir os braços no `layout.rs` por chamadas de delegação.
Não apagar código — mover. Correr `cargo test` após cada extracção antes
de passar à seguinte.

### Estrutura de destino

```
01_core/src/rules/layout/
  mod.rs          ← antigo layout.rs (orquestrador + Layouter struct)
  counters.rs     ← SetHeadingNumbering, CounterUpdate, CounterDisplay
  references.rs   ← Labelled, Ref
  outline.rs      ← Outline (Tarefa 5)
```

Se o linter não aceitar `rules/layout/mod.rs` como substituto de
`rules/layout.rs` (o path de import muda), verificar com:

```bash
# Confirmar se o linter aceita o caminho com mod.rs
grep -rn "use.*rules::layout" 01_core/src/ 03_infra/src/ | head -10
```

Se os imports usarem `rules::layout::layout` e o ficheiro passar a ser
`rules/layout/mod.rs`, os imports continuam a funcionar em Rust sem
alteração — `mod.rs` é transparente para o sistema de módulos.

### 2a — Extrair `counters.rs`

Antes de mover qualquer braço, renomear `layout.rs` para `layout/mod.rs`
e declarar os submódulos no topo do ficheiro imediatamente — antes de
qualquer outra alteração:

```rust
// Primeira coisa a escrever em layout/mod.rs após mover o ficheiro:
pub mod counters;
pub mod references;
pub mod outline;
```

O compilador vai reportar erros de ficheiro em falta. Criar os ficheiros
vazios (com apenas o header de linhagem) para que o compilador compile
antes de qualquer migração de código:

```bash
touch 01_core/src/rules/layout/counters.rs
touch 01_core/src/rules/layout/references.rs
touch 01_core/src/rules/layout/outline.rs
# Adicionar headers de linhagem a cada ficheiro vazio
cargo test  # deve compilar — ficheiros vazios não têm erros
```

Só após este `cargo test` passar é que se move o código braço a braço.

Criar `01_core/src/rules/layout/counters.rs` com header de linhagem
apontando para `layout_counters.md`. Mover os braços:
- `Content::SetHeadingNumbering { active }` → função `pub fn layout_set_heading_numbering`
- `Content::CounterUpdate { key, action }` → função `pub fn layout_counter_update`
- `Content::CounterDisplay { kind }` → função `pub fn layout_counter_display`

Cada função recebe `(&mut CounterState, ...)` e retorna o `Content` a
desenhar (ou `None` para os braços sem output visual).

No `layout/mod.rs`, substituir os braços por:

```rust
Content::SetHeadingNumbering { active } =>
    counters::layout_set_heading_numbering(&mut self.counter, *active),
Content::CounterUpdate { key, action } =>
    counters::layout_counter_update(&mut self.counter, key, action),
Content::CounterDisplay { kind } => {
    let text = counters::format_counter_display(&self.counter, kind);
    self.layout_node(&Content::text(text), self.style)
},
```

```bash
cargo test  # ← verificar antes de continuar
```

### 2b — Extrair `references.rs`

Criar `01_core/src/rules/layout/references.rs` com header de linhagem
apontando para `layout_references.md`. Mover os braços:
- `Content::Labelled { target, label: _ }` → função `pub fn layout_labelled`
- `Content::Ref { target }` → função `pub fn layout_ref`

```bash
cargo test  # ← verificar antes de continuar
```

---

## Tarefa 3 — Recolha de Títulos na Introspecção (L1)

### 3a — Expandir `CounterState`

Em `01_core/src/entities/counter_state.rs`, adicionar dois campos:

```rust
#[derive(Debug, Clone, Default)]
pub struct CounterState {
    pub hierarchical:      HashMap<String, Vec<usize>>,
    pub flat:              HashMap<String, usize>,
    pub numbering_active:  HashMap<String, bool>,
    pub resolved_labels:   HashMap<Label, String>,
    /// Títulos catalogados para a TOC.
    /// Tupla: (label automática, corpo do título como Content, nível).
    /// Guardar Content em vez de String preserva formatação (negrito,
    /// itálico, equações inline) na TOC — `plain_text()` destruiria isso.
    pub headings_for_toc:  Vec<(Label, Content, usize)>,
    /// Contador interno para gerar labels únicas para cada heading.
    /// Não representa numeração de secções — é apenas um gerador de IDs.
    pub auto_label_counter: usize,
}
```

### 3b — Braço `Heading` na introspecção

Em `introspect.rs`, expandir o braço `Content::Heading`. O corpo do título
é clonado directamente (sem `plain_text()`) para preservar formatação:

```rust
Content::Heading { level, body } => {
    state.step_hierarchical("heading", *level as usize);

    // Gerar label automática única para que a TOC possa referenciar este título.
    state.auto_label_counter += 1;
    let auto_label = Label(format!("auto-toc-{}", state.auto_label_counter));

    // Só registar prefixo se a numeração estiver activa.
    // Se a numeração estiver inactiva, inserir string vazia — o braço Ref
    // resolverá para "" (nada visível) em vez de usar o fallback "@auto-toc-N".
    // Sem esta inserção, títulos não-numerados imprimem "@auto-toc-N" no PDF.
    let resolved_text = if state.is_numbering_active("heading") {
        state.format_hierarchical("heading")
            .map(|prefix| format!("Secção {}", prefix))
            .unwrap_or_default()
    } else {
        String::new()  // Resolve para string vazia — sem fallback "@auto-toc-N"
    };
    state.resolved_labels.insert(auto_label.clone(), resolved_text);

    // Guardar para a TOC: clonar o body preserva formatação (negrito,
    // equações, etc.). plain_text() destruiria "= *Introdução* ao $E=mc^2$".
    state.headings_for_toc.push((auto_label, *body.clone(), *level as usize));

    walk(body, state);
},
```

### 3d — Adicionar `Content::Outline` ao `walk`

Em `introspect.rs`, adicionar `Content::Outline` ao braço terminal
(sem efeito nos contadores):

```rust
// No braço de terminais:
| Content::Outline => {},
```

---

## Tarefa 4 — `Content::Outline` (L1)

Em `01_core/src/entities/content.rs`, adicionar a variante:

```rust
/// Marcador para a Tabela de Conteúdos.
/// O layouter substitui este nó pela lista de títulos do documento.
Outline,
```

Actualizar `plain_text()` (retornar `""`) e `is_empty()` (retornar `false`).

**Actualizar `layout()` para alargar a ponte entre passagens.** O Passo 60
estabeleceu que `layout()` copia apenas `resolved_labels` do estado da
introspecção. Agora é necessário copiar também `headings_for_toc` —
sem esta passagem de bastão, o Layouter encontrará o vector vazio e
a TOC nunca será desenhada.

**Atenção:** a assinatura de `layout()` estabelecida no Passo 60 é
`layout(content: &Content, initial_state: CounterState)`. Não chamar
`introspect()` internamente — isso reverteria a separação de passagens
do Passo 60 e duplicaria o processamento. O orquestrador em L3 continua
a chamar a introspecção primeiro e a passar o resultado.

```rust
pub fn layout(content: &Content, initial_state: CounterState) -> PagedDocument {
    let mut l = Layouter::new(/* métricas actuais */);
    // Passagem de bastão: injectar os dois campos que a Passagem 2 consome.
    l.counter.resolved_labels  = initial_state.resolved_labels;
    l.counter.headings_for_toc = initial_state.headings_for_toc;
    // NÃO copiar hierarchical, flat, numbering_active — reconstruídos nó a nó.
    l.layout_content(content);
    l.finish()
}
```

Adicionar ao braço terminal do Layouter em `layout/mod.rs` enquanto
`outline.rs` ainda não tem implementação (placeholder):

```rust
Content::Outline => {
    // Implementação delegada em outline.rs — Tarefa 5.
    // Por agora, não gera output visual.
},
```

Correr `cargo test` para confirmar que todas as variantes de `Content`
continuam cobertas no `walk` e no match do Layouter.

---

## Tarefa 5 — Implementação de `outline.rs` (L1)

Criar `01_core/src/rules/layout/outline.rs` com header de linhagem
apontando para `layout_outline.md`.

Substituir o placeholder do Layouter:

```rust
// Em layout.rs (orquestrador):
Content::Outline => outline::layout_outline(self),
```

Em `outline.rs`:

```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout_outline.md
//! @prompt-hash <hash>
//! @layer L1
//! @updated 2026-04-12

use crate::entities::content::Content;
// Importar o tipo do Layouter — ajustar o path conforme a estrutura real

/// Gera a sequência visual da Tabela de Conteúdos.
/// Lê `headings_for_toc` do estado injectado pela introspecção.
/// Usa Content clonado (não String) para preservar formatação dos títulos.
pub fn layout_outline<M: FontMetrics>(layouter: &mut Layouter<M>) {
    // Clonar o vector antes do loop para evitar borrow duplo de `layouter`:
    // `layouter.counter` (borrow imutável) e `layouter.layout_node` (borrow mutável).
    let entries: Vec<_> = layouter.counter.headings_for_toc.clone();

    let mut seq = Vec::new();

    // Título da TOC
    seq.push(Content::heading(1, Content::text("Índice")));

    for (label, body_content, level) in entries {
        // Indentação proporcional ao nível.
        let indent = "  ".repeat(level.saturating_sub(1));

        // O Ref usa a label automática. Se a numeração estava activa,
        // resolved_labels contém "Secção X"; se não estava, contém "".
        // Em ambos os casos, o braço Ref não usa o fallback "@auto-toc-N".
        let line = Content::Sequence(vec![
            Content::text(indent),
            Content::Ref { target: label },
            Content::text(" "),
            body_content,  // Content clonado — preserva formatação original
                       // ATENÇÃO: se o título contiver CounterUpdate ou CounterDisplay,
                       // esses nós serão avaliados pelo Layouter novamente aqui,
                       // causando duplicação de efeitos (ex: contador avançado duas vezes).
                       // Registado como DEBT-13 — ver Tarefa 6.
            Content::Linebreak,
        ]);
        seq.push(line);
    }

    layouter.layout_node(&Content::Sequence(seq), layouter.style);
}
```

**Visibilidade `pub(super)` — obrigatória antes de criar os submódulos.**
Ao mover lógica para `counters.rs`, `references.rs`, e `outline.rs`, esses
ficheiros precisam de aceder a campos e métodos do `Layouter` que estão
actualmente privados ao módulo `layout`. Como os submódulos são filhos
directos de `layout/mod.rs`, `pub(super)` concede acesso apenas ao módulo
pai e seus irmãos — sem expor o `Layouter` ao resto da crate (`parser`,
`eval`, `math`), o que `pub(crate)` faria indevidamente.

Antes de extrair o primeiro braço, actualizar as declarações em `layout/mod.rs`:

```rust
// Em layout/mod.rs — promover visibilidade dos membros partilhados:
pub struct Layouter<M: FontMetrics> {
    pub(super) counter:      CounterState,
    pub(super) style:        TextStyle,
    pub(super) font_size_pt: Pt,
    // ... restantes campos com pub(super) ...
}

impl<M: FontMetrics> Layouter<M> {
    pub(super) fn layout_node(&mut self, content: &Content, style: TextStyle) {
        self.layout_content(content);
    }
    // ... outros métodos auxiliares com pub(super) ...
}
```

`pub(super)` é estritamente mais restrito que `pub(crate)`: só os módulos
dentro de `layout/` podem aceder. O motor de `eval`, o parser e o motor
matemático permanecem completamente cegos ao `Layouter`.

---

## Tarefa 6 — Testes

**Registo de DEBT-13 em `00_nucleo/DEBT.md` antes dos testes:**

```markdown
### DEBT-13 — Efeitos colaterais duplicados na TOC (Passo 61)
O `outline.rs` injeta clones do `Content` dos títulos na sequência da TOC.
Se um título contiver `CounterUpdate` ou `CounterDisplay` (ex: `= Capítulo
#counter("cap").step()`), esses nós são avaliados duas vezes pelo Layouter
— uma na TOC e outra no título real — causando avanço duplo de contadores.
Mitigação actual: os testes deste passo não usam contadores dentro de headings.
Resolução futura: mecanismo de "congelamento" de AST que neutraliza efeitos
colaterais em clones de renderização estática.
```

**Aviso para os testes:** não usar `CounterUpdate`, `CounterDisplay`, ou
chamadas a `counter(...)` dentro do `body` de um `Content::Heading` nos
testes deste passo — os resultados serão não-determinísticos enquanto
DEBT-13 não estiver resolvido.

### Testes L1 — Introspecção com TOC

```rust
#[test]
fn introspect_cataloga_headings_para_toc() {
    use crate::entities::label::Label;
    use crate::rules::introspect::introspect;

    let content = Content::Sequence(vec![
        Content::SetHeadingNumbering { active: true },
        Content::heading(1, Content::text("Introdução")),
        Content::heading(2, Content::text("Motivação")),
        Content::heading(1, Content::text("Conclusão")),
    ]);

    let state = introspect(&content);
    assert_eq!(state.headings_for_toc.len(), 3);

    let (_, title_0, level_0) = &state.headings_for_toc[0];
    assert_eq!(title_0, "Introdução");
    assert_eq!(*level_0, 1);

    let (_, _, level_1) = &state.headings_for_toc[1];
    assert_eq!(*level_1, 2);
}

#[test]
fn introspect_gera_labels_automaticas_unicas() {
    use crate::rules::introspect::introspect;

    let content = Content::Sequence(vec![
        Content::heading(1, Content::text("A")),
        Content::heading(1, Content::text("B")),
    ]);

    let state = introspect(&content);
    let label_a = &state.headings_for_toc[0].0;
    let label_b = &state.headings_for_toc[1].0;
    assert_ne!(label_a, label_b, "labels automáticas devem ser únicas");

    // As labels devem estar em resolved_labels
    assert!(state.resolved_labels.contains_key(label_a));
    assert!(state.resolved_labels.contains_key(label_b));
}
```

### Testes L1 — Layout da TOC

```rust
#[test]
fn layout_outline_gera_indice_com_titulos() {
    use crate::rules::introspect::introspect;
    use crate::rules::layout::layout;

    let content = Content::Sequence(vec![
        Content::SetHeadingNumbering { active: true },
        Content::Outline,
        Content::heading(1, Content::text("Introdução")),
        Content::heading(2, Content::text("Motivação")),
    ]);

    // Passagem 1 — o teste orquestra explicitamente como o orquestrador L3 faz.
    let state = introspect(&content);
    // Passagem 2 — layout recebe o estado pré-calculado.
    let doc = layout(&content, state);
    let text = doc.plain_text();

    assert!(text.contains("Índice"), "TOC deve ter título 'Índice'");
    assert!(text.contains("Introdução"), "TOC deve listar o título H1");
    assert!(text.contains("Motivação"), "TOC deve listar o título H2");
}

#[test]
fn layout_outline_sem_headings_gera_apenas_titulo() {
    use crate::rules::introspect::introspect;
    use crate::rules::layout::layout;

    let content = Content::Outline;
    let state = introspect(&content);
    let doc = layout(&content, state);
    let text = doc.plain_text();

    assert!(text.contains("Índice") || text.is_empty(),
        "TOC sem headings deve gerar apenas o título ou estar vazia");
}
```

### Testes L3 — Pipeline completo

```rust
#[test]
fn pipeline_outline_gera_pdf_sem_panico() {
    let (world, _dir) = world_from_str(
        "#set heading(numbering: \"1.\")\n\
         #outline()\n\
         = Introdução\n\
         == Motivação\n\
         = Conclusão"
    );
    let pdf = compile_to_pdf(&world);
    assert!(!pdf.is_empty(), "PDF com #outline() não deve estar vazio");
}
```

---

## Verificação final

```bash
# Após cada extracção de submódulo (não apenas no final):
cargo test

# No final:
cargo clippy --fix --allow-dirty --allow-no-vcs
cargo test
crystalline-lint --fix-hashes .
crystalline-lint .
```

Critérios de conclusão:
- [ ] Prompts L0 criados com `git add` antes de qualquer ficheiro de código.
- [ ] `layout/mod.rs` declara `pub mod counters; pub mod references; pub mod outline;`
  como primeira acção — antes de mover qualquer braço.
- [ ] Campos e métodos auxiliares do `Layouter` promovidos para `pub(super)`
  (não `pub(crate)`) antes de extrair o primeiro submódulo.
- [ ] Ficheiros vazios criados e compilando antes de mover código.
- [ ] `layout/counters.rs` criado e braços migrados; `cargo test` passa.
- [ ] `layout/references.rs` criado e braços migrados; `cargo test` passa.
- [ ] `CounterState::headings_for_toc` é `Vec<(Label, Content, usize)>`.
- [ ] `introspect.rs` regista sempre a auto-label em `resolved_labels`:
  prefixo numérico se numerado, string vazia se não numerado.
- [ ] `layout()` mantém assinatura com `CounterState` externo (Passo 60);
  copia `resolved_labels` E `headings_for_toc`; não chama `introspect()`.
- [ ] `Content::Outline` adicionado ao enum, ao `walk`, e ao match do Layouter.
- [ ] `layout/outline.rs` usa `Content` clonado (não String) para os títulos.
- [ ] `Content::Ref` na TOC nunca imprime `@auto-toc-N`.
- [ ] DEBT-12 registado (sem números de página na TOC).
- [ ] DEBT-13 registado (efeitos colaterais duplicados em clones de AST).
- [ ] Testes não usam `CounterUpdate` dentro de `Heading` (DEBT-13).
- [ ] Zero violações no linter e no clippy.

---

## Ao terminar, reportar

**Do diagnóstico:**
- Número de linhas de `layout.rs` antes da decomposição.
- Se `Content::Outline` já existia ou foi adicionado neste passo.
- Quais prompts L0 de layout já existiam e quais foram criados de raiz.

**Da implementação:**
- Se os campos do `Layouter` precisaram de `pub(crate)` ou se a visibilidade
  de módulo pai (`pub(super)`) foi suficiente para os submódulos.
- Se o clone de `headings_for_toc` em `outline.rs` foi necessário (borrow
  duplo) ou se a estrutura do Layouter permitiu outra abordagem.
- Se a migração de `rules/layout.rs` para `rules/layout/mod.rs` causou
  alterações nos imports em L3.
- Número total de testes e zero violations confirmados.

**Go/No-Go para o Passo 62:**
- **GO — `#figure()` e auto-numeração**: decomposição feita e TOC operacional;
  Passo 62 adiciona `Content::Figure { body, caption }` com contador automático,
  fechando o subitem de DEBT-10.
- **GO — números de página na TOC** (DEBT-12): se a arquitectura de retorno
  de dados do layout for prioritária, Passo 62 implementa a terceira passagem.
- **NO-GO — decomposição causou regressões**: se a extracção de submódulos
  quebrou testes em L3; Passo 62 estabiliza os imports antes de avançar.
