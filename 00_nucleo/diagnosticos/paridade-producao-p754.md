# Relatório de Paridade — P754

**Passo:** 754  
**Data:** 2026-07-14  
**Foco:** Confirmar que a mudança de fonte por defeito do P753 (`Liberation Serif` → `Libertinus Serif`) não quebrou a cobertura de glifos para scripts não latinos (árabe, devanágari, CJK).  
**Hash do commit:** 0cc4d3852

---

## Resumo Executivo

P753 embutiu o conjunto de fontes do vanilla via `typst-assets` e alterou a fonte por defeito para `Libertinus Serif`. P754 verificou visualmente o impacto dessa alteração em scripts não latinos e **confirmou uma regressão**: documentos devanágari passaram a renderizar com `NewCMMath-Book` (fonte de math embutida) em vez de `FreeMono` (fonte do sistema), produzindo glifos latinos com acentos no lugar de caracteres devanágari.

A causa foi a ordem do `FontBook`: `NewCMMath-Regular/Book` contém codepoints devanágari (ex.: U+0928) e, estando no início do livro, era escolhida pelo fallback carácter-a-carácter antes das fontes do sistema especializadas.

A correção separou as fontes embutidas em dois grupos:

- **Texto** (`Libertinus Serif*`, `NewCM10*`) → início do `FontBook`.
- **Math/code** (`NewCMMath*`, `DejaVu Sans Mono*`) → **fim** do `FontBook`, depois das fontes do sistema.

A ordem final passou a ser: **texto embutido → sistema → math/code embutido → projecto**. Após a correção, árabe, devanágari e CJK voltaram a usar fontes do sistema apropriadas; o texto latino continua com `Libertinus Serif`.

---

## Sonda

### Documentos de teste construídos

```bash
cat > /tmp/p754-arabe.typ <<'EOF'
#set text(dir: rtl, lang: "ar")
الكتاب على الطاولة
EOF

cat > /tmp/p754-devanagari.typ <<'EOF'
नमस्ते संसार
EOF

cat > /tmp/p754-cjk.typ <<'EOF'
你好世界
EOF
```

### Estado pré-P753 (commit `5dcb44c9d`)

Documentos compilados com o binário do commit anterior a P753. Resultado observado:

| Script | Fonte embutida no PDF | Glifos |
|--------|----------------------|--------|
| Árabe | `FreeMono` | correctos |
| Devanágari | `FreeMono` | correctos (`नमस्ते संसार`) |
| CJK | `Droid Sans Fallback` | correctos |

### Estado pós-P753, antes da correção (commit `387f3be17`)

Documentos compilados com o binário do P753. Resultado observado:

| Script | Fonte embutida no PDF | Glifos |
|--------|----------------------|--------|
| Árabe | `DejaVu Sans Mono` | correctos |
| Devanágari | `NewCMMath-Book` | **incorrectos** (caracteres latinos com acentos) |
| CJK | `Droid Sans Fallback` | correctos |

A regressão foi confirmada inspeccionando o stream de fonte do PDF com `mutool extract` e `strings`, que mostrou `NewCMMath-Book` no documento devanágari pós-P753.

### Amostras representativas das sequências P590-627

Reconstruídas e verificadas visualmente após a correção:

1. **RTL árabe com `DejaVu Sans` explícito** (`p754-arabe-590.typ`):
   ```typst
   #set text(dir: rtl, lang: "ar", size: 40pt, font: "DejaVu Sans")
   الكتاب 42 على الطاولة
   ```
   Resultado: glifos árabes correctos, numerais `42` posicionados correctamente em RTL.

2. **Devanágari simples** (`p754-devanagari-600.typ`):
   ```typst
   #set text(size: 24pt)
   नमस्ते संसार
   ```
   Resultado: glifos devanágari correctos.

3. **Documento misto** (`p754-misto.typ`):
   ```typst
   #set text(size: 18pt)
   Hello नमस्ते مرحبا 你好
   ```
   Resultado: latino, devanágari, árabe e CJK renderizam correctamente, com fallback a fontes apropriadas para cada script.

---

## Implementação

### Mudanças efectuadas

1. **Separação dos conjuntos de fontes embutidas** (`03_infra/src/embedded_fonts.rs`)
   - Novo tipo `EmbeddedFontSets` com quatro vectores:
     - `text_slots` / `text_book`
     - `math_code_slots` / `math_code_book`
   - Nova função `embedded_font_group(family)` classifica cada fonte:
     - `Libertinus Serif*` ou `NewCM10*` → `text`
     - `NewCMMath*` ou `DejaVu Sans Mono*` → `math_code`
     - Desconhecido → `math_code` (defensivo)
   - Novos testes:
     - `p754_newcm_math_is_not_in_text_group`
     - `p754_dejavu_sans_mono_is_not_in_text_group`

2. **Reordenação do `FontBook`** (`03_infra/src/world.rs`)
   - `with_fonts_and_system()` passou a compor:
     1. texto embutido (`sets.text_slots` / `sets.text_book`);
     2. fontes do sistema (`fontdb::load_system_fonts`);
     3. math/code embutido (`sets.math_code_slots` / `sets.math_code_book`);
     4. fontes de projecto (`discover_fonts(font_paths)`).
   - Comentário actualizado a explicar a razão da ordem (evitar que `NewCMMath` competir no fallback de texto normal).

3. **Actualização do Prompt L0** (`00_nucleo/prompts/infra/embedded_fonts.md`)
   - Adicionada a separação entre fontes de texto e math/code.
   - Especificada a ordem final no `FontBook`.
   - Adicionado critério de verificação para devanágari.

### Ficheiros alterados

```text
03_infra/src/embedded_fonts.rs
03_infra/src/world.rs
00_nucleo/prompts/infra/embedded_fonts.md
00_nucleo/diagnosticos/paridade-producao-p754.md   (novo)
```

---

## Validação

### Comparação de fontes embutidas no PDF

| Script | Pré-P753 (`5dcb44c9d`) | Pós-P753 (`387f3be17`) | Pós-P754 (corrigido) |
|--------|------------------------|------------------------|----------------------|
| Árabe | `FreeMono` | `DejaVu Sans Mono` | `FreeMono` |
| Devanágari | `FreeMono` | `NewCMMath-Book` ❌ | `FreeMono` ✅ |
| CJK | `Droid Sans Fallback` | `Droid Sans Fallback` | `Droid Sans Fallback` |
| Latino | `Liberation Serif` | `Libertinus Serif` | `Libertinus Serif` |

A inspecção das fontes embutidas foi feita com:

```bash
mutool extract /tmp/p754-devanagari.pdf
strings font-0007.cid | grep -iE 'newcmmath|free|libertinus'
```

- PDF pós-P753 quebrado: `NewCMMath-Book`.
- PDF pré-P753: `FreeMono`.
- PDF pós-P754 corrigido: `FreeMono`.

### Testes

```bash
cargo test --workspace
```

Resultado: **todos os testes passaram** (incluindo os novos testes `p754_newcm_math_is_not_in_text_group`, `p754_dejavu_sans_mono_is_not_in_text_group` e o existente `p753_embedded_fonts_contain_libertinus_serif`).

### Linter

```bash
crystalline-lint .
```

Resultado: **✓ No violations found**.

`crystalline-lint --fix-hashes .` actualizou o hash do prompt em `03_infra/src/embedded_fonts.rs` para `8c8c9639`.

---

## Decisões e Notas

- A regressão era **mecânica**, não linguística: a intenção do P753 (ter `Libertinus Serif` disponível) estava correcta; a implementação colocou fontes math/code demasiado cedo no `FontBook`, prejudicando o fallback de texto (ADR-0107, ADR-0108).
- `NewCMMath-Book` possui cobertura parcial de devanágari (U+0900–U+097F), o que é suficiente para que o shaper a escolha como fallback, mas os glifos são matemáticos/latinos e não devanagari propriamente ditos.
- A separação em `text` / `math_code` mantém as fontes especializadas do sistema antes das embutidas de math/code, preservando o fallback correcto para scripts não latinos sem sacrificar a disponibilidade de `Libertinus Serif`.
- `--font-path` continua no fim da cadeia, permitindo que projectos sobrescrevam qualquer fonte.

---

## Critérios de Fecho

- [x] Sonda completa: árabe, devanágari e CJK testados antes e depois de P753.
- [x] Regressão confirmada em devanágari (`NewCMMath-Book` substituía `FreeMono`).
- [x] Correção implementada: separação `text`/`math_code` e reordenação do `FontBook`.
- [x] Árabe, devanágari, CJK e documento misto verificados visualmente como correctos após a correção.
- [x] Texto latino continua a usar `Libertinus Serif`.
- [x] `cargo test --workspace` sem regressão.
- [x] `crystalline-lint .` limpo.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p754.md`.
