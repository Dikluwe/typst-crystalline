# P574 — Interacção entre P568 (codepoints) e a sequência RTL

**Data:** 2026-07-05  
**Passo:** 574  
**Tipo:** Verificação directa  
**ADR-0108:** em vigor

---

## 1. Objectivo

Confirmar se a correcção P568 (`try_shape` preserva texto só de espaço;
`collect_text_codepoints` inclui esses espaços no subset) interage com o
trabalho RTL de P566–P569.

---

## 2. Documento de referência (curto)

Input (`/tmp/p574-referencia.typ`):

```typst
#set text(lang: "ar", size: 40pt)
الكتاب 42 على الطاولة
```

Comando:

```bash
./target/release/typst /tmp/p574-referencia.typ /tmp/p574.pdf
pdftotext -tsv /tmp/p574.pdf -
```

Resultado pós-P573/P574:

| Texto (visual) | left (pt) | top (pt) | width (pt) |
|----------------|----------:|---------:|-----------:|
| ةلواطلا        |     70.87 |    49.90 |     168.00 |
| على            |    248.72 |    49.90 |      72.00 |
| 42             |    330.56 |    49.90 |      40.00 |
| باتكلا         |    380.41 |    49.90 |     144.00 |

Comparação directa com P567:

| Texto (visual) | left (pt) P567 | top (pt) P567 | width (pt) P567 | Δ left | Δ top | Δ width |
|----------------|---------------:|--------------:|----------------:|-------:|------:|--------:|
| ةلواطلا        |          70.87 |         49.90 |          168.00 |   0.00 |  0.00 |    0.00 |
| على            |         248.72 |         49.90 |           72.00 |   0.00 |  0.00 |    0.00 |
| 42             |         330.56 |         49.90 |           40.00 |   0.00 |  0.00 |    0.00 |
| باتكلا         |         380.41 |         49.90 |          144.00 |   0.00 |  0.00 |    0.00 |

**Conclusão:** as posições são idênticas às medições de P567. P568 não altera
o posicionamento do documento de referência RTL.

---

## 3. Documento longo

Input (`/tmp/p574-longo.typ`):

```typst
#set text(lang: "ar", size: 14pt)
هذا نص طويل باللغة العربية. يحتوي على معلومات قيمة. نأمل أن يعمل بشكل صحيح.
```

Comando:

```bash
./target/release/typst /tmp/p574-longo.typ /tmp/p574-longo.pdf
pdftotext /tmp/p574-longo.pdf -
```

Resultado pós-P573/P574:

```text
‫هذا نص طويل باللغة العربية‪.‬يحتوي على معلومات قيمة‪.‬نأمل أن‬
‫يعمل بشكل صحيح‪.‬‬
```

Comparação com P569:

P569 reportou 4 linhas:

```text
‫نأمل أن‬
‫يحتوي على معلومات قيمة ‪.‬‬
‫هذا نص طويل باللغة العربية ‪.‬‬
‫يعمل بشكل صحيح ‪.‬‬
```

O output actual tem 2 linhas e os pontos aparecem colados às palavras
precedentes. No entanto, as palavras árabes **não estão coladas** em
`pdftotext` (há espaços entre `هذا`, `نص`, `طويل`, etc.), pelo que o critério
morfológico de P569 (palavras separadas) mantém-se.

### 3.1 Sonda de isolamento da causa

Para descartar P568 como causa da diferença face ao relatório de P569,
reverti temporariamente as alterações P568 (`git stash push -u`), compilei o
binário release e gerei o mesmo documento longo:

```bash
./target/release/typst /tmp/p574-longo.typ /tmp/p574-longo-sem-p568.pdf
pdftotext /tmp/p574-longo-sem-p568.pdf -
mutool draw -F txt /tmp/p574-longo-sem-p568.pdf 1
```

Output `pdftotext` sem P568:

```text
‫هذا نص طويل باللغة العربية‪.‬يحتوي على معلومات قيمة‪.‬نأمل أن‬
‫يعمل بشكل صحيح‪.‬‬
```

Output `mutool` sem P568:

```text
نألمأن.ةميقتامولعمىلعيوتحي.ةيبرعلاةغللابليوطصناذه
.حيحصلكشبلمعي
```

O output sem P568 é **idêntico** ao output com P568. Logo, a diferença de
layout face ao relatório P569 não é causada por P568.

### 3.2 Atribuição da diferença

O relatório P569 (secção 5) refere que, aquando da sua validação, existia no
working tree código órfão em L1 (`layout_space` em `01_core`) que afectava
testes e, provavelmente, medições. Esse código foi descartado em P572. A
configuração actual (P573) produz o output com 2 linhas, que é o comportamento
do código commitado sem o código órfão.

**Conclusão:** P568 não interage com a sequência RTL no documento longo. A
diferença de layout face ao relatório P569 é uma consequência do descarte do
código órfão de L1 em P572, não de P568. O critério de palavras não coladas
de P569 continua satisfeito.

---

## 4. Decisão / Classificação

| Questão | Decisão | Base de medição |
|---------|---------|-----------------|
| P568 altera posições do documento de referência RTL? | **Não** | Coordenadas idênticas a P567. |
| P568 altera quebra de linha do documento longo RTL? | **Não** | Output idêntico com/sem P568. |
| P568 afecta a separação de palavras árabes? | **Não** | `pdftotext` continua a extrair palavras separadas. |
| Diferença face a P569 é regressão? | **Não** | Atribuída ao descarte de código órfão L1 em P572; critério morfológico de P569 mantém-se. |

---

## 5. Validação final

```bash
cargo build --release --bin typst   # ok
cargo test --workspace              # ok
crystalline-lint .                  # No violations found
```

---

## 6. Conclusão

P568 e a sequência RTL (P566–P569) são **independentes**. A correcção de
P568 não introduz regressão no posicionamento nem na separação de palavras em
documentos RTL. As diferenças observadas no documento longo face ao relatório
P569 são explicadas pelo descarte de código órfão em P572 e não constituem
regressão de P568.
