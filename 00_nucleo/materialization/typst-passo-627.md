---
# P627 — Direcção de colunas em documentos que mudam de LTR para RTL só com `#pagebreak()`

> **Passo:** 627
> **Data:** 2026-07-05
> **Foco:** P626 corrigiu a direcção de preenchimento de colunas para RTL, mas deixou disclosed uma limitação: se um documento tiver um único `#set page(columns:)` no topo, e depois mudar de LTR para RTL só com `#pagebreak()` (sem outro `#set page(columns:)`), a direcção de preenchimento fica fixada pela primeira direcção encontrada, não pela direcção real de cada página. Isto é plausível num documento bilingue. Este passo corrige, com a mesma disciplina de sonda-primeiro já estabelecida — não é para ser feito com pressa.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.** **ADR-0114 EM VIGOR** — esta área (`wrap_page_columns`, `columns.rs`) já teve mais do que uma correcção em cima de outra; sonda obrigatória antes de qualquer código, para não repetir o padrão desta sequência inteira.

---

## Contexto

`wrap_page_columns` (introduzido em P537b) agrupa o conteúdo depois de um `#set page(columns:)` num único `Content::Columns`, até encontrar outro `SetPage` ou `Pagebreak` que mude as propriedades relevantes. P626 lê `text.dir` do corpo desse grupo, uma vez, para decidir a direcção de preenchimento — mas se o grupo inteiro (várias páginas, unidas só por `#pagebreak()` simples) misturar LTR e RTL, só a primeira direcção encontrada é usada para todas as páginas do grupo.

---

## Sonda

### Confirmar o sintoma directamente

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
mutool trace /tmp/p627-cristalino.pdf | grep -A2 -m1 'fill_text'
```

Confirmar se a segunda página (RTL) preenche a coluna direita primeiro, como devia, ou a esquerda, herdando a direcção da primeira página.

### Confirmar o que o vanilla faz no mesmo documento

```bash
lab/typst-original/target/release/typst compile /tmp/p627-bilingue.typ /tmp/p627-vanilla.pdf
mutool trace /tmp/p627-vanilla.pdf | grep -A2 -m1 'fill_text'
```

### Confirmar onde `wrap_page_columns` decide os limites do agrupamento

```bash
grep -n "fn wrap_page_columns\|SetPage\|Pagebreak" 01_core/src/entities/content.rs | head -20
```

Confirmar se `wrap_page_columns` já tem alguma noção de "quebrar o grupo quando a direcção muda", ou se agrupa cegamente até ao próximo `SetPage`/`Pagebreak` de propriedades de página, sem olhar para `text.dir`.

### Critério de fecho da sonda

- [ ] Sintoma confirmado com o documento bilingue, com números/posições, não só a descrição.
- [ ] Comportamento do vanilla confirmado no mesmo documento.
- [ ] Ponto exacto de agrupamento em `wrap_page_columns` localizado, com `file:line`.

---

## Implementação

Depende da sonda. A direcção mais provável: `wrap_page_columns` (ou a leitura de `body_dir` já criada em P626) precisa de decidir a direcção por página, não por grupo inteiro — ou `body_dir` passa a ser recalculado a cada página dentro do grupo, em vez de uma vez só para o grupo inteiro.

### Critério de fecho da implementação

- [ ] Documento bilingue: cada página preenche as colunas na direcção certa para o seu próprio `text.dir`.
- [ ] Documentos de uma só direcção (o caso já corrigido em P626) sem regressão.
- [ ] Testado também com três ou mais mudanças de direcção no mesmo documento (LTR, RTL, LTR de novo), não só uma transição.

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

Repetir os testes de P626 (`p626_set_page_columns_rtl_preenche_direita_primeiro`) para confirmar que continuam a passar sem alteração.

---

## Critério de fecho do passo

- [ ] Sonda completa, sintoma confirmado com números.
- [ ] Correcção aplicada, direcção decidida por página, não por grupo inteiro.
- [ ] Testado com múltiplas transições de direcção no mesmo documento.
- [ ] Testes de P626 sem regressão.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p627.md`, com hash do commit.
