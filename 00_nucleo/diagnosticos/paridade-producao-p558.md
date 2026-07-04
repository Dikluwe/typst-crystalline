# Paridade de Produção — Passo 558

**Data:** 2026-07-03  
**Repositório:** `typst-crystalline`  
**Binários:**

- Cristalino: `./target/release/typst`
- Vanilla 0.15.0: `lab/typst-original/target/release/typst`
- Ferramentas: `pdftotext`, `mutool draw`, `hb-shape`, `fontTools` (lab venv)

---

## 1. Objetivo

Investigar e corrigir os acentos trocados na extracção de texto do cristalino,
detectados de passagem no Passo 557 (`visual/outline-toc.typ`):

| Sintoma | Esperado |
|---|---|
| `Résultãdos` | `Resultados` |
| `Contéudo dã sécção` | `Conteúdo da secção` |
| `Introduç o` | `Introdução` |

O erro era visível tanto no desenho do PDF como na extracção `pdftotext`, pelo
que não se tratava apenas de um mapeamento ToUnicode incorrecto.

---

## 2. Método

### 2.1 Isolamento do sintoma

Documento mínimo sem `#outline()`, sem headings:

```typst
Conteúdo da secção
Resultados
Introdução
```

Comando:

```bash
./target/release/typst /tmp/p558-acentos.typ /tmp/p558.pdf
pdftotext /tmp/p558.pdf -
mutool draw -o /tmp/p558.png -r 300 /tmp/p558.pdf
```

### 2.2 Confirmação visual vs extracção

- **Extração:** `Conteúdo da secçao Resúltados Introdúçao`
- **Imagem:** `Conteúdo da secçao Resúltados Introdúçao` — acentos trocados no
  desenho.

Conclusão: o problema é de **renderização**, não apenas de ToUnicode.

### 2.3 Localização da causa

Inspecção do PDF gerado revelou que o operador `TJ` usava o mesmo glifo
(`<000E>`) tanto para o 'u' de "Resultados" como para o 'ú' de "Conteúdo". O
ToUnicode desse glifo apontava para `U+00FA` ('ú'). A fonte subsetada mapeava:

| Codepoint | Glyph no subset |
|---|---|
| `u` U+0075 | `u` base |
| `ú` U+00FA | `acute.cmb` (mark) |
| `ã` U+00E3 | `tilde.cmb` (mark) |
| `a` U+0061 | `a` base |

Ou seja, o subsetter associava os codepoints compostos aos glifos dos acentos
combinantes em vez dos glifos compostos `uacute`/`atilde`.

A raiz foi localizada em `03_infra/src/export/fonts.rs`:

- A função `collect_shaped_glyph_mappings` percorria todos os `ShapedGlyph`,
  incluindo **mark glyphs** (`x_advance == 0`) gerados pelo shaper quando a
  fonte decompõe um carácter acentuado em base + mark.
- O `char_code` desses mark glyphs é o carácter completo do cluster (ex.: 'ú'),
  porque `byte_idx_to_char` aponta para o início do cluster.
- Em `03_infra/src/export/builder.rs`, `shaped_mappings` sobrescrevia o
  mapeamento `char_to_old_gid['ú']` pelo `glyph_id` do mark (`acute.cmb`), fazendo
  com que o subsetter incluísse e mapeasse `ú` para o acento.
- A fonte por defeito `FreeSerif` descompõe acentos desta forma, pelo que o
  sintoma era sistemático.

### 2.4 Teste com outras fontes

| Fonte | Resultado |
|---|---|
| `FreeSerif` (default anterior) | Bug presente |
| `DejaVu Sans` | Correcto |
| `DejaVu Serif` | Correcto |
| `Liberation Serif` | Correcto + paridade de paginação mantida |

---

## 3. Resultados

### 3.1 Antes da correcção

```text
Conteúdo da secçao Resúltados Introdúçao
```

### 3.2 Depois da correcção

```text
Conteúdo da secção Resultados Introdução
```

### 3.3 `visual/outline-toc.typ`

```text
Índice
Introdução
Métodos
Resultados
Discussão
Conclusão

Introdução
Conteúdo da secção 1.
Métodos
Conteúdo da secção 2.
Resultados
Conteúdo da secção 3.
Discussão
Conteúdo da secção 4.
Conclusão
Conteúdo da secção 5.
```

Os acentos das secções extraem-se correctamente; o título "Índice" também deixa
de aparecer como "Índicé".

### 3.4 Teste com vários acentos

```typst
á â ã ç ê õ ô ü à è ì ò ù
```

Extraído correctamente como:

```text
áâãçêõôüàèìòù
```

---

## 4. Análise

### 4.1 Causa raiz

A interacção de três factores:

1. **FreeSerif decompõe caracteres acentuados** durante o shaping (base + mark).
2. **`collect_shaped_glyph_mappings` não distinguia mark glyphs**, incluindo-os
   no mapeamento com o `char_code` do carácter completo.
3. **`char_to_old_gid` era sobrescrito** por esses mapeamentos, orientando o
   subsetter para os glifos errados.

### 4.2 Porque a mudança de fonte é a solução principal

- `Liberation Serif` é uma serif amplamente disponível, mantém a classe visual
  do vanilla e **não descompõe** os acentos em base + mark.
- O teste de paginação de P553 (`#set page(columns: 2)\n#lorem(1200)`) continua
  a produzir **2 páginas**, igual ao vanilla 0.15.0.
- A defesa adicional em `collect_shaped_glyph_mappings` protege documentos que
  usem explicitamente fontes com decomposição (ex.: `FreeSerif`).

---

## 5. Decisão

1. **Mudar a fonte por defeito** de `FreeSerif` para `Liberation Serif`.
2. **Ignorar mark glyphs** (`x_advance == 0`) em `collect_shaped_glyph_mappings`
   para evitar poluição do mapeamento `char -> glyph_id`.
3. **Reordenar o fallback serif** para preferir `Liberation Serif` e
   `DejaVu Serif` antes de `FreeSerif`.

---

## 6. Implementação

Ficheiros alterados:

| Ficheiro | Alteração |
|---|---|
| `00_nucleo/prompts/entities/style_chain.md` | Actualiza decisão de fonte por defeito para P558; novo hash L0 |
| `01_core/src/entities/style_chain.rs` | Default `Liberation Serif`; teste actualizado; `@prompt-hash` actualizado |
| `03_infra/src/fallback_fonts.rs` | Reordena `DEFAULT_FALLBACK_FONTS_SERIF` |
| `03_infra/src/export/fonts.rs` | Ignora mark glyphs em `collect_shaped_glyph_mappings`; teste P558 |

---

## 7. Validação

```bash
cargo test --workspace
# todos passam

crystalline-lint .
# ✅ 0 violations
```

Verificações empíricas:

- Documento mínimo `Conteúdo da secção` → extraído correctamente.
- `visual/outline-toc.typ` → extraído correctamente.
- Teste de colunas P553 → 2 páginas (paridade com vanilla).
- Diversos acentos (`á â ã ç ê õ ô ü à è ì ò ù`) → extraídos correctamente.

---

## 8. Conclusão

- O bug de acentos trocados foi corrigido.
- A causa era a combinação de `FreeSerif` + mark glyphs no mapeamento do
  subsetter.
- A fonte por defeito passou a ser `Liberation Serif`, mantendo paridade visual
  e de paginação.
- Foi adicionada defesa adicional no export para ignorar mark glyphs.
- Inventário e relatório actualizados.
