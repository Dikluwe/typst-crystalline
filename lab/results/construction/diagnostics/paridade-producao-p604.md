# Paridade de Produção — P604

**Data do relatório:** 2026-07-07
**Passo:** 604
**Foco:** Verificar se o vanilla 0.15.0 gera bookmarks PDF a partir de labels manuais, e implementar o equivalente se existir.

---

## Resumo executivo

A premissa do passo era que o vanilla 0.15.0 permitia criar bookmarks PDF a partir de pontos marcados com `#label(...)`, e que o cristalino estava em falta por só gerar bookmarks a partir de headings. A sonda directa refuta essa premissa:

- `#label("ponto-manual")` sozinho **não** aparece na árvore `/Outlines` do vanilla.
- `#outline(target: label("..."))` produz erro no vanilla: `cannot outline text`.
- O vanilla só gera bookmarks PDF a partir de `HeadingElem` com `bookmarked: true` (por defeito).

Portanto, o comportamento do cristalino — bookmarks só a partir de headings — coincide com o vanilla 0.15.0. Não há implementação a fazer para este passo. A disparidade documentada era um mal-entendido. As listas de disparidades foram actualizadas para reflectir que esta restrição é paridade, não scope-out funcional.

**Achado incidental:** o cristalino não expõe ainda a propriedade `bookmarked`/`outlined` do heading na sintaxe do utilizador. O campo interno `outlined` existe (`HeadingElem::outlined`), mas a função `heading()` do avaliador não aceita argumentos nomeados. Isso é uma disparidade real, mas não é o foco de P604.

---

## Proveniência

- **Hash base:** `677f2ba022e0d9e9cde23d34369e49863e31f2df`
- **Data/hora:** 2026-07-07T23:05-03:00 (referência de sessão)
- **Binários usados:**
  - Cristalino: `./target/release/typst`
  - Vanilla 0.15.0: `lab/typst-original/target/release/typst compile`
- **Ferramentas auxiliares:** `mutool show`, `grep`

---

## Sonda

### Documento com label manual

```typst
= Secção com heading

Texto normal. #label("ponto-manual") Aqui está um ponto marcado manualmente, sem ser um heading.

Mais texto.
```

**Vanilla 0.15.0 — `mutool show ... outline`:**

```text
|   "Secção com heading"  #page=1&zoom=100,70.86614,60.86615
```

**Cristalino — `mutool show ... outline`:**

```text
|   "Secção com heading"  #page=1&zoom=nan,70.87,81.900028
```

Em ambos, apenas o heading aparece. O label manual não gera bookmark.

### `#outline(target: label(...))`

```typst
#outline(target: label("ponto-manual"))

= Secção com heading
Texto normal. #label("ponto-manual") Aqui está um ponto marcado.
Mais texto.
```

**Vanilla 0.15.0:**

```text
error: cannot outline text
```

O vanilla rejeita a ideia de fazer outline de um label isolado.

### Busca por `bookmark` no código-fonte do vanilla 0.15.0

```bash
grep -rn "bookmark" lab/typst-original/crates/typst-library/src/ lab/typst-original/crates/typst-pdf/src/
```

Resultados limitados a:

- `heading.rs` — campo `bookmarked` do `HeadingElem`.
- `outline.rs` — comentários sobre headings bookmarked.
- `outline.rs` (pdf) — leitura do campo `bookmarked`.

**Não existe** `BookmarkElem`, `#bookmark`, ou qualquer outro mecanismo de bookmark manual no vanilla 0.15.0.

---

## Decisão

Não se implementa suporte a bookmarks manuais a partir de labels. O vanilla 0.15.0 não o suporta, pelo que não é uma disparidade. O comportamento "bookmarks só a partir de headings" passa a ser considerado paridade.

O achado incidental sobre a sintaxe `heading(bookmarked: false)`/`heading(outlined: false)` fica registado para um passo futuro, se for prioritário.

---

## Actualização das listas de disparidades

- `00_nucleo/diagnosticos/inventario-decisoes-pendentes.md`:
  - `Bookmarks só de headings`: estado alterado de **Scope-out** para **Fechado em P604**.

- `00_nucleo/diagnosticos/estado-disparidades-vanilla-p593.md`:
  - `Bookmarks só a partir de headings` removido da secção "Scope-out deliberado".
  - Adicionado à secção "Corrigido ao longo desta conversa" com a razão de P604.

---

## Validação

- `cargo test --workspace` → 0 falhas.
- `crystalline-lint .` → `✓ No violations found`.

Não houve alterações de código; os testes existentes de P602/603 confirmam que os bookmarks de headings continuam a funcionar.

---

## Critérios de fecho do passo

- [x] Mecanismo exacto do vanilla confirmado por medição directa.
- [x] Conclusão: não há implementação a fazer; comportamento coincide com vanilla.
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
- [x] Listas de disparidades actualizadas.
- [x] Relatório escrito com proveniência.

---

## Ligações

- `00_nucleo/materialization/typst-passo-604.md` — passo que originou esta sonda.
- `00_nucleo/diagnosticos/inventario-decisoes-pendentes.md` — lista de decisões actualizada.
- `00_nucleo/diagnosticos/estado-disparidades-vanilla-p593.md` — estado das disparidades actualizado.
- `01_core/src/entities/elements/heading.rs:23` — `HeadingElem` com campo `outlined` (achado incidental).
