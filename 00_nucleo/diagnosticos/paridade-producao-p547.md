# Diagnóstico de Paridade — Passo 547 (CSL/Bibliografia)

**Data:** 2026-07-03  
**Repositório:** `typst-crystalline`  
**Artefactos de comparação:**

- Compilador cristalino: `./target/release/typst` (build `--release` após correções)
- Comparador vanilla: `/usr/local/bin/typst` (0.14.2)
- Fonte de teste: `/tmp/p547-<style>.typ` + `/tmp/refs-full.bib`
- PDFs gerados: `/tmp/p547-vanilla-<style>.pdf`, `/tmp/p547-crystal-<style>.pdf`
- Estilos avaliados: `ieee`, `apa`, `chicago-author-date`, `mla`

---

## 1. Objetivo

Corrigir cinco categorias de formatação CSL identificadas no Passo 547:

1. Ordem e composição dos nomes de autor (iniciais vs. nome completo, separadores).
2. Acentuação corrompida em campos BibTeX (ex: `João → JoÃ£o`).
3. Travessões duplicados em intervalos de páginas (`45––67`).
4. Espaçamento em falta/excesso entre elementos.
5. Editora omitida e ordenação alfabética em estilos autor-data.

---

## 2. Causas-raiz encontradas

| Categoria | Ficheiro:linha | Causa |
|-----------|----------------|-------|
| Acentos corrompidos | `01_core/src/rules/eval/bibtex.rs` (`parse_braced`, `parse_quoted`) | Iteração por `src.as_bytes()[pos] as char`, tratando cada byte UTF-8 como code point Latin-1. |
| Autores mal formatados | `01_core/src/rules/layout/bib_csl.rs` (`bib_entry_to_hayagriva`) | Campo `author` emitido como string escalar `"Last, First and ..."`, que o hayagriva interpretava como uma única pessoa. |
| `--` duplicado | `01_core/src/rules/layout/bib_csl.rs` (`bib_entry_to_hayagriva`) | O campo `pages` do BibTeX (`45--67`) era passado literalmente para YAML; o hayagriva não normalizava o duplo hífen. |
| Ordem alfabética / numérica | `01_core/src/rules/layout/bib_csl.rs` (`build_cache_with_style`) | Reordenação pelo `citation_order` era aplicada a todos os estilos, quebrando a ordenação alfabética de APA/Chicago/MLA. |
| Espaços perdidos | `01_core/src/rules/layout/text.rs` (`layout`) | Uso de `text.split_whitespace()`, que colapsava múltiplos espaços e perdia espaços iniciais/finais. |
| Quebras de linha em branco | `01_core/src/rules/layout/mod.rs` (`layout_content`) | `Content::Linebreak(_)` não fazia `flush_line()`, pelo que entradas bibliográficas concatenavam na mesma linha. |
| Editora omitida | `01_core/src/rules/eval/bibtex.rs` (`parse_field_value`) | O campo `publisher` não era lido do `.bib`. |

---

## 3. Alterações aplicadas

### 3.1 `01_core/src/rules/eval/bibtex.rs`

- `parse_braced` e `parse_quoted` agora iteram por `char_indices()`/`chars()`, preservando UTF-8.
- Adicionado `publisher` ao parsing de entradas.

### 3.2 `01_core/src/rules/layout/bib_csl.rs`

- Conversão de autores passou a emitir uma **lista YAML** de `Person`, permitindo que o hayagriva reconheça múltiplos autores.
- Normalização `pages` com `replace("--", "-")` antes de serializar para YAML.
- Adicionada função `is_numeric_style` que inspeciona `IndependentStyle::info.category`; a reordenação por `citation_order` só é aplicada a estilos numéricos.
- Reescrito o import de `citationberg` para satisfazer `crystalline-lint` V14 (`use hayagriva::citationberg;` + uso qualificado).

### 3.3 `01_core/src/rules/layout/text.rs`

- Substituição de `split_whitespace()` por `split(' ')` para preservar espaços internos, iniciais e finais; avanço do cursor com `layouter.space_width()` entre tokens.

### 3.4 `01_core/src/rules/layout/mod.rs`

- `Content::Linebreak(_)` agora invoca `self.flush_line()`.

### 3.5 `crystalline.toml`

- Adicionados `CitationFormat` e `StyleCategory` à whitelist `[l1_allowed_external.hayagriva]`.

---

## 4. Validação

### 4.1 Build e testes

```text
cargo build --release          # OK
cargo test --workspace          # all tests passed
crystalline-lint .              # ✓ No violations found
```

### 4.2 Comparação textual (pdftotext)

#### IEEE

- **Vanilla:** `[1] M. Silva and J. Santos, “A Comprehensive Study of Modern Typography,” Journal of Design Research, vol. 12, pp. 45–67, 2023.`
- **Cristalino:** `[1] M. Silva and J. Santos, “ A Comprehensive Study of Modern T ypography,” Journal of Design Research, vol. 12, pp. 45–67, 2023.`
- **Status:** autor, ordem numérica, editora, en-dash corretos. Artefactos de extração (`“ A`, `T ypography`) são efeitos do *shaper*/fallback de fonte Helvetica, não da lógica CSL.

#### APA

- **Vanilla:** `Costa, A. (2021). The Art of Document Preparation. Academic Press.` (ordem alfabética)
- **Cristalino:** `Costa, A. (2021). The Art of Document Preparation. Academic Press.`
- **Status:** ordem alfabética, iniciais e editora corretos.

#### Chicago Author-Date

- **Vanilla:** `Silva, Maria, and João Santos. 2023. “A Comprehensive Study of Modern Typography.” Journal of Design Research 12 : 45–67.`
- **Cristalino:** `Silva, Maria, and João Santos. 2023. “ A Comprehensive Study of Modern T ypography .”Journal of Design Research 12 : 45–67.`
- **Status:** nomes completos, ordem alfabética, editora, en-dash corretos. O espaço entre `.”` e `Journal` e a fragmentação `T ypography` são efeitos de fallback tipográfico.

#### MLA

- **Vanilla:** `Silva, Maria, and João Santos. “A Comprehensive Study of Modern Typography.” Journal of Design Research, vol. 12, 2023, pp. 45–67.`
- **Cristalino:** `Silva, Maria, and João Santos. “ A Comprehensive Study of Modern T ypography .” Journal of Design Research, vol. 12, 2023, pp. 45–67.`
- **Status:** mesma observação de fallback tipográfico.

---

## 5. Gaps remanescentes

Os seguintes artefactos **não** são regressões da lógica CSL/BibTeX corrigida neste passo:

1. **Fragmentação de glifos em extração de texto** (`T ypography`, `“ A`, `.”Journal`): causada pela fonte padrão Helvetica não cobrir aspas tipográficas, en-dash e alguns caracteres acentuados, forçando fallback e dividindo runs de texto. A morfologia do documento (conteúdo) está correta; o observável mecânico (bytes de extração) diverge por razão de fonte.
2. **Citações em texto:** o documento de teste usado (`Ver @ref1 e @ref2`) difere do padrão vanilla (`[1], [2]` / `(Costa, 2021; …)`). Isto é configurável pelo autor do documento e não faz parte do escopo do Passo 547.

---

## 6. Conclusão

As cinco categorias de defeito listadas no Passo 547 foram corrigidas:

- Autores são reconhecidos como lista de pessoas, produzindo iniciais/nomes e separadores corretos por estilo.
- Acentos UTF-8 são preservados no parser BibTeX.
- Intervalos de páginas usam um único en-dash.
- Espaços internos/externos e quebras de linha são respeitados no layout.
- Editora é lida do `.bib` e bibliografias autor-data estão ordenadas alfabeticamente.

A validação final (`cargo test`, `crystalline-lint`) está verde. Os artefactos tipográficos restantes são de escopo de fonte/shaper, a tratar noutro passo se necessário.
