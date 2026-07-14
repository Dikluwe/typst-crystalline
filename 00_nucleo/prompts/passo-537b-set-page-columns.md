# Prompt L0 — P537b — `#set page(columns:)` → mecanismo de colunas
Hash do Código: 6ada155e

**Camada**: L1  
**Ficheiros alvo**: `01_core/src/entities/content.rs`, `01_core/src/entities/layout_types.rs`, `01_core/src/rules/eval/rules.rs`, `01_core/src/rules/eval/mod.rs`, `01_core/src/rules/layout/set_page.rs`, `01_core/src/rules/layout/mod.rs`, `01_core/src/rules/layout/tests.rs`  
**Origem**: Passo 537b — fecha a ligação entre `#set page(columns: N)` e o consumer `Content::Columns` já corrigido em P537.  
**ADRs**: ADR-0107 (paridade linguagem vs mecânica), ADR-0108 (medir antes de decidir), ADR-0109 (atomização forma B), ADR-0054 (perfil graded / scope-outs).

---

## 1. Contexto

P537 implementou colunas reais (com `colbreak()` e notas de rodapé por coluna) para a forma-função explícita `#columns(2)[...]`. A sintaxe `#set page(columns: 2)`, no entanto, continua ignorada: o eval descarta o argumento `columns` e o layout renderiza o documento como uma coluna só.

Este L0 autoriza a ligação: `#set page(columns: N)` deve produzir o mesmo observable de `#columns(N)[...]` — N colunas reais na página, `colbreak()` separando colunas, e notas de rodapé no fundo da coluna correcta.

---

## 2. Medições (file:line) que sustentam a decisão

- `01_core/src/rules/eval/rules.rs:743-779` — `target == "page"` lê `width`/`height`/`margin`/`numbering`, mas **não lê `columns`**. O argumento é silenciosamente ignorado.
- `01_core/src/rules/layout/mod.rs:858-860` — `Content::SetPage` delega a `set_page::layout`, que só altera `page_config.width/height/margin/numbering`.
- `01_core/src/entities/layout_types.rs:433-449` — `PageConfig` não tem campo `columns`.
- `01_core/src/rules/layout/columns.rs:59-161` — `columns::layout` já sabe renderizar N colunas reais lado a lado, dividindo o `body` pelos `Content::Colbreak`.
- `01_core/src/rules/eval/mod.rs:407-566` — `eval_markup` constrói uma `Sequence` de `Content`; `#set page(...)` produz um `Content::SetPage` seguido do conteúdo subsequente como irmãos na `Sequence`.

**Decisão**: reaproveitar `Content::Columns` existente. Não recriar mecânica de colunas. A diferença entre `#columns(2)[body]` e `#set page(columns: 2)` é apenas **quem fornece o body**: na forma-função o body é explícito; na set-rule o body é o resto do documento (até próxima fronteira de página/colunas).

**P552 — correcção**: as notas de rodapé de `#columns(2)[body]` e `#set page(columns: 2)` **não** têm o mesmo observable. O vanilla coloca as notas de `#columns()` empilhadas no final do contentor (coluna esquerda), enquanto `#set page(columns:)` coloca cada nota no fundo da coluna onde é referenciada. Para preservar esta distinção, o `ColumnsElem` sintético produzido por `wrap_page_columns` deve ser marcado com `page_columns: true` (ver L0 `rules/columns` P552).

---

## 3. Alterações autorizadas

### 3.1 `Content::SetPage` — adicionar `columns`

Em `01_core/src/entities/content.rs`:

```rust
SetPage {
    width:     Option<f64>,
    height:    Option<f64>,
    margin:    Option<f64>,
    numbering: Option<EcoString>,
    /// **P537b** — número de colunas definido por `#set page(columns: N)`.
    columns:   Option<usize>,
},
```

- `columns: None` ↔ propriedade ausente (comportamento actual de uma coluna).
- `columns: Some(0)` não deve ser produzido (validado no eval).

### 3.2 `PageConfig` — adicionar `columns`

Em `01_core/src/entities/layout_types.rs`:

```rust
pub struct PageConfig {
    pub width:     f64,
    pub height:    f64,
    pub margin:    f64,
    pub numbering: Option<EcoString>,
    /// **P537b** — colunas activas para páginas desta configuração.
    pub columns:   Option<usize>,
}
```

Default: `columns: None`.

### 3.3 Eval — reconhecer `columns`

Em `01_core/src/rules/eval/rules.rs`, no arm `target == "page"`:

- Adicionar `let mut columns: Option<usize> = None;`.
- No loop dos named args, tratar `"columns"`:
  - `Value::Int(n)` com `n >= 1` → `columns = Some(n as usize)`.
  - `Value::Int(n)` com `n < 1` → erro `"columns must be at least 1"`.
  - `Value::None` → `columns = None`.
  - Outros tipos → ignorar (preservar comportamento de herança/ignorar).
- Incluir `columns` no `Content::SetPage { ... }` retornado.

### 3.4 Layout de `SetPage` — propagar `columns` para `PageConfig`

Em `01_core/src/rules/layout/set_page.rs`:

- Adicionar parâmetro `columns: &Option<usize>`.
- Se `columns != &new_config.columns`, actualizar `new_config.columns` e marcar `changed = true`.
- **Não forçar nova página só por mudança de `columns`** nesta fase (scope-out: paginação real por set-page columns é diferida; o teste de P537b usa `#set page(columns: 2)` no topo do documento, onde a página já está vazia).
- **P750** — quando `SetPage` muda a configuração de página e reinicia o cursor,
  o `cursor_y` deve ser posicionado a `margem + cap_height(style.size)`, usando
  `FontMetrics::cap_height`, não o ascender. Isto aplica a regra geral de
  inicialização do cursor do L0 `rules/layout.md` também ao ponto onde SetPage
  reconfigura a região.

### 3.5 Transformação AST pós-eval — envolver o body implícito

Adicionar helper puro `wrap_page_columns(content: Content) -> Content` (pode viver em `01_core/src/entities/content.rs` como método de `Content`, ou como função privada em `01_core/src/rules/eval/mod.rs`).

**Semântica**:

- Percorrer recursivamente o `Content`.
- Quando encontrar `Content::Sequence(parts)` (qualquer nível), procurar elementos `Content::SetPage { columns: Some(n), .. }`.
- Para cada ocorrência, agrupar os elementos subsequentes na mesma `Sequence` até:
  - encontrar outro `Content::SetPage` (com ou sem `columns`);
  - encontrar `Content::Pagebreak`;
  - chegar ao fim da `Sequence`.
- Substituir esse grupo de elementos por um único `Content::Columns { count: n, body: Content::Sequence(grupo) }`.
- O `Content::SetPage` original permanece imediatamente antes do `Columns`, para que `set_page::layout` actualize `page_config` antes de `columns::layout` consumir.
- Recursar no `body` do `Columns` produzido (para tratar nested set-pages) e nas outras variantes container normalmente.

**Porquê transformação AST e não estado do Layouter?**
- O consumer `columns::layout` precisa de um `body` único para dividir pelos `colbreak`. O `SetPage` não tem body — o body é o resto do escopo. Transformar a AST é a ponte de menor blast radius entre a set-rule e o consumer existente.
- A transformação é pura (sem I/O), portanto permanece em L1.

**Limitação consciente (paridade linguagem, não mecânica)**:
- A transformação envolve o resto do escopo léxico, não "cada página". Isso é suficiente para o caso de uso do topo do documento e preserva o observable de duas colunas reais. Paginação real com mudança de colunas a meio do documento é scope-out per ADR-0054 graded.

### 3.6 Aplicar a transformação

Em `01_core/src/rules/eval/mod.rs`, `eval_with_full_error`:

- Após `run_pass(true)` e antes de `module.set_content(rendered_content)`, aplicar `wrap_page_columns` ao `rendered_content`.
- A transformação deve ser aplicada **também** ao `original_content` (introspection content) para manter alinhamento entre as duas passagens? Não — `original_content` é usado pelo `TagIntrospector` para elementos locatable; `SetPage` não é locatable, e `Columns` não altera a localização de headings/labels no observable. Aplicar apenas em `rendered_content` é suficiente. Se surgirem discrepâncias, igualar na revisão.

---

## 4. Testes

### 4.1 Teste E2E de eval + layout

Adicionar em `01_core/src/rules/layout/tests.rs`:

```rust
#[test]
fn p537b_set_page_columns_produz_colunas_reais() {
    let source = Source::new(
        FileId::new(None, std::path::Path::new("/test.typ")),
        r#"#set page(columns: 2)
A
#colbreak()
B"#.to_string(),
    );
    let world = crate::rules::eval::tests::NullWorld;
    let module = eval_for_test(&world, &source).unwrap();
    let content = module.content().expect("módulo deve ter content");
    let doc = layout(content);
    // Verificar que A e B têm x distinto (duas colunas).
    // ...
}
```

Alternativa construir `Content` directamente se o teste E2E for pesado.

### 4.2 Teste de footnotes por coluna via `#set page(columns:)`

Construir:

```
Content::Sequence([
    Content::SetPage { ..., columns: Some(2) },
    Content::text("Hello "),
    Content::footnote(Content::text("Nota A")),
    Content::text(" world."),
    Content::colbreak(false),
    Content::text("Goodbye "),
    Content::footnote(Content::text("Nota B")),
    Content::text(" moon."),
])
```

Aplicar `wrap_page_columns` (ou invocar via eval) e verificar que as notas A e B têm x distinto e y próximo do fundo (mesmo critério de `p537_footnotes_columns_colbreak_posicionam_por_coluna`).

### 4.3 Regressão `#columns(2)[...]`

- Confirmar que `p537_footnotes_columns_colbreak_posicionam_por_coluna` continua a passar sem alteração.
- Confirmar que `p220_colbreak_dentro_columns_separa_colunas_reais` continua a passar.

### 4.4 Teste de `columns` inválido

- `#set page(columns: 0)` → erro `"columns must be at least 1"`.
- `#set page(columns: -1)` → erro.

---

## 5. Validação final

- `cargo test --workspace` limpo.
- `crystalline-lint .` limpo.
- Header de linhagem `@prompt` / `@prompt-hash` actualizado nos ficheiros tocados.
- Relatório em `00_nucleo/diagnosticos/paridade-producao-p537b.md`.

---

## 6. Scope-outs

- Paginação real com `#set page(columns:)` a meio do documento (nova página só quando outros campos de `SetPage` mudam).
- `columns: none` explicitamente já é suportado (limpa o campo).
- Gutters via `#set page(gutter: ...)` não fazem parte deste passo.
