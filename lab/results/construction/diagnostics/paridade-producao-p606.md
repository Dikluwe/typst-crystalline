# Paridade de Produção — P606

**Data do relatório:** 2026-07-08
**Passo:** 606
**Foco:** Separar `outlined` de `bookmarked` em `heading()` — `outlined` controla o índice do documento (`#outline()`); `bookmarked` controla a árvore `/Outlines` do PDF.

---

## Resumo executivo

P605 expôs os argumentos `outlined` e `bookmarked` em `heading()`, mas internamente ambos afectavam a mesma flag (`HeadingElem::outlined`). O vanilla 0.15.0 distingue as duas: `outlined` controla o índice impresso; `bookmarked` controla os bookmarks PDF, com `bookmarked: auto` a seguir `outlined` por defeito.

P606 fecha essa disparidade:

- `HeadingElem` passa a ter `outlined: bool` e `bookmarked: Option<bool>` (`None` = auto).
- `HeadingElem::is_bookmarked()` devolve `bookmarked.unwrap_or(self.outlined)`.
- O walk de introspecção emite `Tag::HeadingForToc` quando `outlined == true` e `Tag::HeadingForBookmarks` quando `is_bookmarked() == true`.
- `#outline()` consome `Introspector::headings_for_toc()`.
- A geração de `/Outlines` consome `Introspector::headings_for_bookmarks()`.

Resultado: os quatro casos possíveis (`outlined`/`bookmarked` true/false) reproduzem o comportamento do vanilla.

---

## Proveniência

- **Hash base:** `e01e39f68413f33565825ada79d9737061bb2bbe`
- **Data/hora:** 2026-07-08T00:20-03:00 (referência de sessão)
- **Binários usados:**
  - Cristalino: `./target/release/typst` (reconstruído em release)
  - Vanilla 0.15.0: `lab/typst-original/target/release/typst compile`
- **Ferramentas auxiliares:** `pdftotext`, `mutool show`

---

## Sonda

### Documento de teste

```typst
#outline()

= Caso A — outlined true, bookmarked true (por defeito)

#heading(outlined: false)[Caso B — só bookmarked]

#heading(bookmarked: false)[Caso C — só outlined]

#heading(outlined: false, bookmarked: false)[Caso D — nenhum dos dois]
```

### Comportamento do vanilla 0.15.0

**Índice impresso (`pdftotext`):**

```text
Contents
Caso A — outlined true, bookmarked true (por defeito) ........................................ 1
Caso C — só outlined ......................................................................... 1
```

**Bookmarks PDF (`mutool show ... outline`):**

```text
|   "Caso A — outlined true, bookmarked true (por defeito)"   #page=1&zoom=100,70.86614,120.47516
```

Interpretação confirmada:

| Caso | `outlined` | `bookmarked` | Índice impresso | Bookmarks PDF |
|------|------------|--------------|-----------------|---------------|
| A | default true | auto→true | sim | sim |
| B | false | auto→false | não | não |
| C | true | false | sim | não |
| D | false | false | não | não |

---

## Implementação

Ficheiros alterados:

- `01_core/src/entities/elements/heading.rs`:
  - `HeadingElem` ganha `outlined: bool` e `bookmarked: Option<bool>`.
  - `new_with_outlined_and_bookmarked(...)`.
  - `is_bookmarked()` → `bookmarked.unwrap_or(self.outlined)`.
  - `get_field`, `map_content`, `map_text` preservam `bookmarked`.

- `01_core/src/entities/content.rs`:
  - `heading_with_outlined_and_bookmarked(...)`.
  - `heading_numbered_with_pattern_outlined_bookmarked(...)`.

- `01_core/src/entities/element_payload.rs`:
  - Nova variante `ElementPayload::HeadingForBookmarks { label, number, body, level }`.

- `01_core/src/entities/introspector.rs`:
  - Nova sub-store `headings_for_bookmarks: Vec<...>`.
  - Novo método de trait `Introspector::headings_for_bookmarks()`.

- `01_core/src/engine/introspect.rs`:
  - `materialize_time` preserva `outlined` e `bookmarked`.
  - Walk arm `Content::Heading` emite `Tag::HeadingForToc` se `outlined`, e `Tag::HeadingForBookmarks` se `h.is_bookmarked()`.

- `01_core/src/engine/stdlib/structural.rs`:
  - `native_heading` parseia `outlined` e `bookmarked` separadamente.
  - Testes unitários P606 adicionados; testes P605 antigos removidos por serem sinónimo da nova semântica.

- `03_infra/src/pipeline.rs`:
  - `doc.extracted_headings = intr.headings_for_bookmarks().to_vec();`

- `03_infra/src/integration_tests.rs`:
  - `compile_to_pdf` usa `intr.headings_for_bookmarks()`.
  - Testes de integração P605/P606 actualizados.

- `03_infra/src/measurements.rs`:
  - `CountingIntrospector` implementa `headings_for_bookmarks()`.

- `01_core/src/engine/eval/repr.rs`:
  - Teste `repr_content_heading` actualizado com `bookmarked: None`.

Prompts L0 actualizados:

- `00_nucleo/prompts/engine/stdlib/structural.md` — `outlined`/`bookmarked` como flags separadas.
- `00_nucleo/prompts/engine/introspect.md` — separação `headings_for_toc` vs `headings_for_bookmarks`.

`@prompt-hash` actualizados via `crystalline-lint --fix-hashes .`:

- `01_core/src/engine/introspect.rs` → `4d0b61c1`
- `01_core/src/engine/stdlib/structural.rs` → `e3351b12`

---

## Validação

### Comparação directa com o vanilla

**Cristalino — índice impresso:**

```text
Índice
Caso A — outlined true, bookmarked true (por defeito)
Caso C — só outlined
```

**Cristalino — bookmarks PDF:**

```text
|   "Caso A — outlined true, bookmarked true (por defeito)"   #page=1&zoom=nan,70.87,81.900028
```

**Vanilla — índice impresso e bookmarks:** ver secção "Sonda". O cristalino reproduz exactamente os quatro casos: A no índice e bookmarks; B em nenhum; C só no índice; D em nenhum.

### Testes automatizados

Novos testes unitários em `01_core/src/engine/stdlib/structural.rs`:

- `native_heading_outlined_false_mantem_bookmarked_auto`
- `native_heading_bookmarked_false_mantem_outlined_true`
- `native_heading_outlined_false_bookmarked_true_separados`

Novos testes unitários em `01_core/src/engine/introspect.rs`:

- `p606_outlined_e_bookmarked_separados_nas_substores`
- `bracketing_valido_8_tags_por_heading_p606` (anteriormente 6 tags)

Novos testes de integração em `03_infra/src/integration_tests.rs`:

- `p606_heading_bookmarked_false_exclui_de_bookmarks_mas_mantem_toc`
- `p606_heading_outlined_false_bookmarked_true_aparece_em_bookmarks`

Testes de contagem de tags actualizados para reflectir a tag extra `HeadingForBookmarks`:

- `walk_emite_start_e_end_para_heading`: 6 → 8 tags
- `walk_aninha_start_end_para_heading_contendo_figure`: 8 → 10 tags
- `walk_emite_tags_em_paralelo_com_state`: 12 → 16 tags
- `bracketing_valido_em_sequencia_plana`: 18 → 24 tags

### Suite completa

- `cargo test --workspace` → 0 falhas.
- `crystalline-lint .` → `✓ No violations found`.

---

## Actualização das listas de disparidades

- `00_nucleo/diagnosticos/estado-disparidades-vanilla-p593.md`:
  - Actualizada entrada "Parâmetro `bookmarked`/`outlined` de `heading()`" para reflectir que a separação completa foi implementada em P606.

- `00_nucleo/diagnosticos/inventario-decisoes-pendentes.md`:
  - Secção 2.3 (PDF — estrutura e metadados): entrada `bookmarked`/`outlined` em `heading()` actualizada para **Fechado em P606**.

---

## Critérios de fecho do passo

- [x] Sonda completa, quatro casos confirmados no vanilla.
- [x] `outlined` e `bookmarked` implementados como campos separados, com fallback `bookmarked: auto` a seguir `outlined`.
- [x] Os quatro casos testados e a bater com o vanilla.
- [x] Sem regressão nos testes de P602 a P605.
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
- [x] Prompts L0 actualizados e hashes sincronizados.
- [x] Listas de disparidades actualizadas.
- [x] Relatório escrito com proveniência.

---

## Ligações

- `00_nucleo/materialization/typst-passo-606.md` — passo que originou a implementação.
- `00_nucleo/prompts/engine/stdlib/structural.md` — Prompt L0 actualizado.
- `00_nucleo/prompts/engine/introspect.md` — Prompt L0 actualizado.
- `01_core/src/entities/elements/heading.rs:23` — `HeadingElem` com `outlined` e `bookmarked`.
- `01_core/src/entities/elements/heading.rs:64` — `HeadingElem::is_bookmarked()`.
- `01_core/src/engine/stdlib/structural.rs:152` — `native_heading` com parse separado.
- `01_core/src/engine/introspect.rs:1194` — walk arm `Content::Heading` emite `HeadingForToc` / `HeadingForBookmarks`.
- `01_core/src/entities/introspector.rs:341` — sub-store `headings_for_bookmarks`.
- `03_infra/src/pipeline.rs:103` — `doc.extracted_headings` a partir de `headings_for_bookmarks()`.
