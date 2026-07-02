---
# P535 — Bookmarks PDF (`/Outlines`)

> **Passo:** 535
> **Data:** 2026-07-02
> **Foco:** P531 Grupo 4.2 confirmou que o cristalino não gera a árvore `/Outlines` do catálogo PDF — o painel de marcadores/navegação de um leitor de PDF fica vazio. Isto é diferente do `outline()` do documento (índice visível no conteúdo da página, já implementado). Aqui trata-se da estrutura de navegação do próprio ficheiro PDF.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.** **ADR-0114 EM VIGOR** — sonda antes de qualquer edição.
> **Dependências:** P531 (Grupo 4.2). Não depende de P534 — área isolada, só exportação, não toca shaper nem layout.

---

## Contexto

Um leitor de PDF (Acrobat, Firefox, mupdf, etc.) tem um painel lateral de marcadores, construído a partir da árvore `/Outlines` no catálogo do ficheiro PDF. Isto é distinto do índice de conteúdo que aparece na própria página (`outline()`, já implementado no cristalino). Um documento pode ter os dois, ou só um.

P531 confirmou: `pdfinfo`/`mutool show ... outline` não mostra entradas no PDF gerado pelo cristalino; o vanilla mostra uma entrada por heading.

---

## Sonda

```bash
grep -rn "/Outlines\|PdfOutline\|bookmark\|OutlineItem" 03_infra/src/export/ --include="*.rs"
```

Perguntas, com `file:line`:

1. Existe algum código, mesmo que incompleto, para gerar a árvore de bookmarks?
2. O cristalino já tem, nalgum sítio, a lista de headings com a sua posição de página (necessária para construir a árvore — cada bookmark aponta para uma página/posição)? Isto provavelmente já existe, porque `outline()` (o índice de conteúdo) precisa da mesma informação.
3. Onde é que o catálogo do PDF é construído (`/Catalog`, `/Root`)? É aí que a entrada `/Outlines` tem de ser adicionada.

```bash
grep -rn "/Catalog\|/Root\|catalog" 03_infra/src/export/ --include="*.rs" | head -10
```

### Critério de fecho da sonda

- [ ] Confirmado se existe algum código parcial.
- [ ] Confirmado se a lista de headings com posição de página já está disponível nalguma estrutura reutilizável.
- [ ] Localizado o ponto onde o catálogo do PDF é escrito.

---

## Implementação

Estrutura esperada da árvore `/Outlines`, por nível de heading:

```
/Outlines (dicionário raiz da árvore)
  /First → primeiro bookmark de nível 1
  /Last  → último bookmark de nível 1
  /Count → número total de bookmarks visíveis

Cada bookmark:
  /Title (string, o texto do heading)
  /Parent → dicionário pai
  /Next / /Prev → irmãos ao mesmo nível
  /First / /Last → filhos (headings de nível inferior aninhados)
  /Dest ou /A → destino (página + posição)
```

Se a lista de headings com posição de página já existe (provável, reaproveitando o que `outline()` já usa), o trabalho aqui é sobretudo: construir a árvore de ligações `/Next`/`/Prev`/`/Parent`/`/First`/`/Last` a partir do nível de cada heading (`=`, `==`, `===`), e escrever isso como objectos PDF no export.

### Critério de fecho da implementação

- [ ] Árvore `/Outlines` construída a partir dos níveis de heading do documento.
- [ ] Cada bookmark aponta para a página e posição correcta.
- [ ] Aninhamento por nível de heading (heading de nível 2 fica filho do heading de nível 1 anterior).
- [ ] `/Catalog` referencia a árvore.

---

## Validação

```bash
cat > /tmp/test-bookmarks.typ <<'EOF'
= Primeira Secção
Texto.
== Subsecção
Mais texto.
= Segunda Secção
Texto.
EOF
./target/release/typst /tmp/test-bookmarks.typ /tmp/bookmarks.pdf
mutool show /tmp/bookmarks.pdf outline
```

Esperado: três entradas visíveis na árvore ("Primeira Secção" com "Subsecção" aninhada, e "Segunda Secção"), cada uma associada à página correcta.

Comparar com vanilla 0.15.0 para o mesmo documento.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda completa antes de código.
- [ ] `/Outlines` gerado e aninhado por nível de heading.
- [ ] Testado com pelo menos três níveis de aninhamento (headings de nível 1, 2, e 3).
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p535.md`.

---

## Próximo passo

P536 — Metadados XMP (`#set document(title:, author:)`). Também isolado à exportação.
