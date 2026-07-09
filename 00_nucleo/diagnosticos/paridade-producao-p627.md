# Relatório — Passo 627 (P627)

**Data:** 2026-07-09  
**Commit de implementação:** `7fc0412dd`  
**Foco:** Direcção de colunas por página em documentos que mudam de LTR para RTL só com `#pagebreak()`.

---

## Resumo executivo

P626 corrigiu a direcção de preenchimento de colunas para RTL, mas deixou uma limitação disclosed: documentos bilingues com um único `#set page(columns:)` no topo e mudança de `text.dir` apenas por `#pagebreak()` ficavam com a direcção da primeira página em todas as páginas seguintes.

P627 resolve isto de duas formas coordenadas:

1. **`wrap_page_columns` (`01_core/src/entities/content.rs`)** — parte o body de um `Content::Columns` sintético nos `Content::Pagebreak` aninhados dentro de `Styled`/`Sequence`, criando um `ColumnsElem` independente por secção de página. Os `Pagebreak` originais são preservados como separadores.
2. **`body_dir` (`01_core/src/rules/layout/columns.rs`)** — passa a procurar a direcção mais interna (`text.dir`) sobre conteúdo visível, ignorando `Styled` vazios (`Space`, `Parbreak`, `Pagebreak`) e preferindo a direcção do conteúdo real. Sem esta mudança, os segmentos resultantes da divisão ainda herdam a direcção do `Styled` exterior.

---

## Sonda

### Comandos

```bash
cat > /tmp/p627-bilingue.typ <<'EOF'
#set page(columns: 2)
#set text(lang: "en", dir: ltr, size: 16pt)
#lorem(150)

#pagebreak()

#set text(lang: "ar", dir: rtl, size: 16pt)
#lorem(150)
EOF
./target/release/typst /tmp/p627-bilingue.typ /tmp/p627-cristalino.pdf
lab/typst-original/target/release/typst compile /tmp/p627-bilingue.typ /tmp/p627-vanilla.pdf
```

### Resultados

| Página | Cristalino antes | Cristalino depois | Vanilla 0.15.0 |
|--------|------------------|-------------------|----------------|
| 1 (LTR) | `x = 70.87` (esquerda) | `x = 70.87` (esquerda) | `x = 70.87` (esquerda) |
| 2 (RTL) | `x = 107.98` (esquerda — **bug**) | `x = 346.66` (direita) | `x = 343.37` (direita) |

### Ponto de agrupamento

`wrap_page_columns` em `01_core/src/entities/content.rs:2232` agrupava tudo o que segue um `SetPage { columns: Some(_) }` até ao próximo `SetPage`/`Pagebreak` ao nível da sequência exterior. Como o eval coloca o conteúdo subsequente dentro de um `Styled` que envolve uma `Sequence` com o `Pagebreak` no seu interior, o agrupador não via a quebra de página e criava um único `ColumnsElem` para ambas as secções.

---

## Implementação

### Ficheiros alterados

- `00_nucleo/prompts/rules/columns.md`:
  - Actualizada secção 4.3 para referir `01_core/src/entities/content.rs`.
  - Adicionada secção 4.3.1 (P627) autorizando a divisão por `Pagebreak`.
  - Adicionada secção 5.2 com medições e decisão de P627.
  - Actualizados critérios de verificação.
  - Hash actualizado para `b9d094f3`.

- `01_core/src/entities/content.rs`:
  - Adicionado `page_column_segments` — divide recursivamente `Sequence`/`Styled` nos `Pagebreak` de topo, preservando os `Pagebreak` e reenvolvendo segmentos com os estilos exteriores.
  - Alterado `wrap_page_columns` para emitir múltiplos `ColumnsElem` quando o body contém `Pagebreak` aninhados.

- `01_core/src/rules/layout/columns.rs`:
  - Alterado `body_dir` para procurar a direcção mais interna sobre conteúdo visível.
  - Adicionado `has_visible_content` para distinguir conteúdo real de separadores estruturais.

- `01_core/src/rules/layout/tests.rs`:
  - Adicionado `p627_bilingue_muda_direcao_com_pagebreak` — verifica que página 1 LTR começa à esquerda e página 2 RTL começa à direita.

---

## Validação

### Testes automatizados

```bash
cargo test --workspace
```

Resultado: **todos passam**.

```bash
crystalline-lint .
```

Resultado: **No violations found**.

### Casos visuais

| Caso | Página 1 | Página 2 | Página 3 |
|------|----------|----------|----------|
| Bilingue LTR → `#pagebreak()` → RTL | `x = 70.87` (esquerda) | `x = 346.66` (direita) | — |
| Três secções LTR → RTL → LTR | `x = 70.87` (esquerda) | `x = 368.88` (direita) | `x = 70.87` (esquerda) |
| LTR único | `x = 70.87` (esquerda) | — | — |
| RTL único | `x = 346.66` (direita) | — | — |

### Regressões verificadas

- Teste `p626_set_page_columns_rtl_preenche_direita_primeiro` continua a passar.
- Documentos LTR/RTL de uma só direcção mantêm o comportamento de P626.

---

## Decisões e notas

- **Por que dividir em `wrap_page_columns` e não em `columns::layout`?** A separação em `wrap_page_columns` é mais limpa arquitecturalmente: cada página passa a ser o seu próprio `ColumnsElem`, com o seu próprio body e direcção. Fazer a mudança de direcção no meio de `layout_flow` exigiria estado adicional no `Layouter` e conhecimento do body restante.
- **Por que `body_dir` procura a direcção mais interna?** Após a divisão, o segmento RTL pode ficar embrulhado num `Styled` exterior com `text.dir: ltr` (porque o `#set text(dir: ltr)` no topo do documento criou esse wrapper). Procurar a direcção mais interna sobre conteúdo visível evita que um `parbreak` vazio ou um wrapper exterior decida a direcção.
- **Limitação conhecida:** se dentro do mesmo `ColumnsElem` houver um `Pagebreak` dentro de um contentor mais profundo (ex.: dentro de um `#box` ou `#grid`), a divisão ainda não o aplica. O caso coberto é o documento bilingue típico: `#set text(dir: …)` ao nível do fluxo principal, separado por `#pagebreak()`.

---

## Critérios de fecho do passo

- [x] Sonda completa, sintoma confirmado com números.
- [x] Correcção aplicada, direcção decidida por página, não por grupo inteiro.
- [x] Testado com múltiplas transições de direcção no mesmo documento.
- [x] Testes de P626 sem regressão.
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p627.md`, com hash do commit.
