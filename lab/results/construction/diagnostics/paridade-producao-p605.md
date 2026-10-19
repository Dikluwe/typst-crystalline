# Paridade de Produção — P605

**Data do relatório:** 2026-07-07
**Passo:** 605
**Foco:** Expor `bookmarked`/`outlined` como argumento de `heading()` para controlar a presença em bookmarks PDF.

---

## Resumo executivo

P604 descobriu que `HeadingElem::outlined` existe internamente, mas a função `heading()` do avaliador não aceitava argumentos nomeados para o controlar. Este passo expõe o campo aos utilizadores:

- `heading(outlined: false)` — exclui o heading do índice do documento (`#outline()`) e da árvore de bookmarks PDF (`/Outlines`).
- `heading(bookmarked: false)` — reconhecido como sinónimo de `outlined: false`; exclui o heading de `/Outlines`.

O vanilla 0.15.0 distingue `outlined` (índice do documento) de `bookmarked` (bookmarks PDF), com `bookmarked: auto` por defeito a seguir `outlined`. O cristalino partilha a mesma fonte de dados (`headings_for_toc`) para o índice e para os bookmarks, pelo que usa uma única flag interna. Ambos os argumentos são aceites e produzem o efeito esperado de exclusão de `/Outlines`.

---

## Proveniência

- **Hash base:** `8d7b04bcf6ec851956b318403acff0ce565aad21`
- **Data/hora:** 2026-07-07T23:40-03:00 (referência de sessão)
- **Binários usados:**
  - Cristalino: `./target/release/typst` (reconstruído em release)
  - Vanilla 0.15.0: `lab/typst-original/target/release/typst compile`
- **Ferramentas auxiliares:** `mutool show`

---

## Sonda

### Comportamento do vanilla 0.15.0

```typst
= Secção normal
#heading(bookmarked: false)[Secção sem bookmark]
= Outra secção normal
```

**`mutool show ... outline`:**

```text
|   "Secção normal"         #page=1&zoom=100,70.86614,60.86615
|   "Outra secção normal"   #page=1&zoom=100,70.86614,120.33215
```

O heading intermédio não aparece. O mesmo acontece com `#heading(outlined: false)[...]`.

### Documentação do vanilla

`lab/typst-original/crates/typst-library/src/model/heading.rs`:

- `outlined: bool` — controla se aparece no `@outline` do documento. Se `true`, também aparece como bookmark PDF por defeito.
- `bookmarked: Smart<bool>` — controla explicitamente a presença no bookmark PDF do PDF exportado.

Para P605, o observável que interessa é: com `bookmarked: false` ou `outlined: false`, o heading não entra em `/Outlines`.

---

## Implementação

Ficheiros alterados:

- `01_core/src/entities/content.rs`:
  - Adicionado `Content::heading_with_outlined(level, body, outlined)`.
  - Adicionado `Content::heading_numbered_with_pattern_and_outlined(level, body, pattern, outlined)`.

- `01_core/src/engine/stdlib/structural.rs`:
  - `native_heading` passou a aceitar argumentos nomeados `outlined: bool` e `bookmarked: bool`.
  - Suporta as formas vanilla:
    - `heading(1, [Body])`
    - `heading([Body])` / `heading[Body]` (level default 1)
    - `heading(outlined: false)[Body]`
    - `heading(bookmarked: false)[Body]`
  - `outlined` tem prioridade sobre `bookmarked` quando ambos são fornecidos.

- `01_core/src/engine/introspect.rs`:
  - Walk arm `Content::Heading` só emite `Tag::HeadingForToc` quando `h.outlined == true`.
  - `materialize_time` preserva o campo `outlined` ao reconstruir headings.

- Prompts L0 actualizados:
  - `00_nucleo/prompts/engine/stdlib/structural.md` — documenta `outlined` e `bookmarked`.
  - `00_nucleo/prompts/engine/introspect.md` — documenta o gate `outlined` no walk.

- `@prompt-hash` actualizados via `crystalline-lint --fix-hashes .`:
  - `01_core/src/engine/stdlib/structural.rs` → `e6d3a5b5`
  - `01_core/src/engine/introspect.rs` → `42b9ffbd`

### Limitação conhecida

O cristalino não distingue `outlined` de `bookmarked` ao nível da semântica interna: ambos os argumentos afectam a mesma flag `HeadingElem::outlined`. Não é possível, nesta implementação, ter um heading no índice do documento mas ausente dos bookmarks PDF (ou vice-versa). Esta separação fica como melhoria futura.

---

## Validação

### Comparação directa com o vanilla

Documento de teste:

```typst
= Secção normal
#heading(bookmarked: false)[Secção sem bookmark]
= Outra secção normal
```

**Cristalino:**

```text
|   "Secção normal"         #page=1&zoom=nan,70.87,81.900028
|   "Outra secção normal"   #page=1&zoom=nan,70.87,114.150028
```

**Vanilla 0.15.0:**

```text
|   "Secção normal"         #page=1&zoom=100,70.86614,60.86615
|   "Outra secção normal"   #page=1&zoom=100,70.86614,120.33215
```

Ambos excluem o heading intermédio.

### Testes automatizados

Novos testes unitários em `01_core/src/engine/stdlib/structural.rs`:

- `native_heading_outlined_false_marca_campo`
- `native_heading_bookmarked_false_usa_outlined`
- `native_heading_outlined_prioridade_sobre_bookmarked`
- `native_heading_rejeita_outlined_nao_bool`

Novos testes de integração em `03_infra/src/integration_tests.rs`:

- `p605_heading_outlined_false_exclui_de_bookmarks`
- `p605_heading_bookmarked_false_exclui_de_bookmarks`

### Suite completa

- `cargo test --workspace` → 0 falhas.
- `crystalline-lint .` → `✓ No violations found`.

---

## Actualização das listas de disparidades

- `00_nucleo/diagnosticos/estado-disparidades-vanilla-p593.md`:
  - Adicionada entrada "Parâmetro `bookmarked`/`outlined` de `heading()`" à secção "Corrigido ao longo desta conversa".

- `00_nucleo/diagnosticos/inventario-decisoes-pendentes.md`:
  - Secção 2.3 (PDF — estrutura e metadados): adicionada linha `bookmarked`/`outlined` em `heading()` como **Fechado em P605**.

---

## Critérios de fecho do passo

- [x] Nome e comportamento do parâmetro confirmados no vanilla.
- [x] `heading()` reconhece `outlined` e `bookmarked`.
- [x] Heading com `outlined: false` / `bookmarked: false` não aparece em `/Outlines`.
- [x] Headings sem o parâmetro continuam em `/Outlines`.
- [x] Sem regressão nos testes de bookmarks anteriores (P602/603/604).
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
- [x] Listas de disparidades actualizadas.
- [x] Relatório escrito com proveniência.

---

## Ligações

- `00_nucleo/materialization/typst-passo-605.md` — passo que originou a implementação.
- `00_nucleo/prompts/engine/stdlib/structural.md` — Prompt L0 actualizado.
- `00_nucleo/prompts/engine/introspect.md` — Prompt L0 actualizado.
- `01_core/src/engine/stdlib/structural.rs:152` — `native_heading` com `outlined`/`bookmarked`.
- `01_core/src/engine/introspect.rs:1077` — walk arm `Content::Heading` com gate `outlined`.
- `01_core/src/entities/elements/heading.rs:23` — `HeadingElem::outlined`.
