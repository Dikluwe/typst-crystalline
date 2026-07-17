# Relatório de Paridade — P537b — `#set page(columns:)` → mecanismo de colunas

**Passo:** 537b  
**Data:** 2026-07-03  
**Foco:** Fechar a ligação entre `#set page(columns: N)` e o consumer `Content::Columns` já corrigido em P537.  
**Prompt L0:** `00_nucleo/prompts/passo-537b-set-page-columns.md` (hash `fe242b7f`)

---

## 1. Resumo executivo

P537 implementou colunas reais (com `colbreak()` e notas de rodapé por coluna) para a forma-função `#columns(2)[...]`. A sintaxe `#set page(columns: 2)`, contudo, continuava a ser ignorada: o eval descartava o argumento `columns` e o documento renderizava como uma coluna só.

Este passo adiciona `columns: Option<usize>` a `Content::SetPage` e a `PageConfig`, faz o eval reconhecer o argumento, e introduz uma transformação AST pura (`Content::wrap_page_columns`) que, sempre que encontra `Content::SetPage { columns: Some(n) }` numa `Sequence`, envolve os nós subsequentes num `Content::Columns { count: n, body: ... }`. O consumer `columns::layout` existente reaproveita-se sem alterações.

Resultado: `#set page(columns: 2)` produz duas colunas reais na mesma página, `colbreak()` separa as colunas, e as notas de rodapé aparecem no fundo da coluna correcta.

---

## 2. Sonda (medir antes de decidir)

| Pergunta | Medição | Resposta |
|---|---|---|
| `#set page(columns:)` é reconhecido no eval? | `01_core/src/engine/eval/rules.rs:743-779` | Não. O arm `target == "page"` lê apenas `width/height/margin/numbering`; `columns` é silenciosamente ignorado. |
| `Content::Columns` pode envolver o documento inteiro? | `01_core/src/engine/layout/columns.rs:59-161` | Sim. O layout de `Columns` divide o `body` pelos `Content::Colbreak` e renderiza os segmentos lado a lado na mesma página. |
| Ponto exacto de ligação? | `01_core/src/engine/eval/mod.rs:385-391` | Transformação AST pós-eval: o conteúdo subsequente a `SetPage { columns: Some(n) }` é envolvido num `Content::Columns`. |

---

## 3. Implementação

### 3.1 Ficheiros alterados

| Ficheiro | Alteração |
|---|---|
| `01_core/src/entities/content.rs` | Adicionado `columns: Option<usize>` a `Content::SetPage`; actualizada igualdade estrutural; adicionado método `Content::wrap_page_columns()`. |
| `01_core/src/entities/layout_types.rs` | Adicionado `columns: Option<usize>` a `PageConfig` (default `None`). |
| `01_core/src/engine/eval/rules.rs` | No arm `target == "page"`, lê e valida `columns: N` (`N >= 1`; `0` ou negativo dá erro). |
| `01_core/src/engine/eval/mod.rs` | Aplica `wrap_page_columns()` ao `rendered_content` após a passagem de show-rules. |
| `01_core/src/engine/layout/mod.rs` | Actualizado match de `Content::SetPage` para passar `columns` a `set_page::layout`. |
| `01_core/src/engine/layout/set_page.rs` | Propaga `columns` para `page_config.columns` quando muda. |
| `01_core/src/engine/layout/tests.rs` | Testes de regressão e novos testes P537b. |

### 3.2 Transformação AST

A função `Content::wrap_page_columns()`:

- Percorre a árvore via `map_content`.
- Só reescreve `Sequence`s que contenham `SetPage { columns: Some(_) }` — documentos sem esta set-rule são preservados byte-identicalmente (evita regressões em snapshots P307b).
- Para cada `SetPage { columns: Some(n) }`, agrupa os nós seguintes até encontrar outro `SetPage` ou `Pagebreak`.
- Substitui o grupo por um único `Content::Columns { count: n, body: Sequence(grupo) }`, mantendo o `SetPage` original imediatamente antes.

### 3.3 Scope-outs conscientes

- Paginação real com mudança de colunas a meio do documento: `#set page(columns:)` no topo do documento é o caso coberto; mudanças posteriores seguem a semântica actual de `SetPage` (só força nova página quando outros campos mudam).
- `columns: none` limpa o campo (suportado).
- `gutter` em `#set page(gutter: ...)` não faz parte deste passo.

---

## 4. Testes

### 4.1 Novos testes

| Teste | Ficheiro | O que verifica |
|---|---|---|
| `p537b_set_page_columns_produz_colunas_reais` | `01_core/src/engine/layout/tests.rs` | `#set page(columns: 2)` + `colbreak()` coloca texto em duas colunas distintas (x distinto). |
| `p537b_set_page_columns_footnotes_por_coluna` | `01_core/src/engine/layout/tests.rs` | Notas A e B aparecem em colunas diferentes e no fundo da página. |
| `p537b_set_page_columns_zero_erro` | `01_core/src/engine/layout/tests.rs` | `#set page(columns: 0)` produz erro no eval. |

### 4.2 Regressões verificadas

- `p220_colbreak_dentro_columns_separa_colunas_reais` — continua a passar.
- `p537_footnotes_columns_colbreak_posicionam_por_coluna` — continua a passar.
- `p537_footnote_uma_coluna_sem_regressao` — continua a passar.
- Snapshots P307b — sem alterações.

### 4.3 Resultado

```text
cargo test --workspace
  test result: ok. 3560 passed; 0 failed; 0 ignored
  test result: ok. 568 passed; 0 failed; 6 ignored
  ...
  (todas as suites passaram)

crystalline-lint .
  ✓ No violations found
```

---

## 5. Validação do critério de fecho do passo

- [x] Sonda completa antes de código.
- [x] `#set page(columns: 2)` produz colunas reais.
- [x] Notas de rodapé na coluna correcta, também por esta via.
- [x] Teste de P537 (`#columns(2)[...]`) sem regressão.
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p537b.md`.

---

## 6. Próximo passo

P538 — confirmação de fecho dos seis itens de alto impacto, agora com `#set page(columns:)` também coberto.
