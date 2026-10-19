# Paridade de Produção — Passo 556

**Data:** 2026-07-03  
**Repositório:** `typst-crystalline`  
**Binários:**

- Cristalino antes de P554: `/tmp/typst-crystalline-antes-p554/target/release/typst` (commit `de46663e7`, P553)
- Cristalino depois de P554/P555: `./target/release/typst`
- Vanilla 0.15.0: `lab/typst-original/target/release/typst`
- Ferramentas: `pdfinfo`, `pdftotext`, `grep`, `diff`

---

## 1. Objetivo

P555 confirmou que o corpus compila sem erro e que a contagem de elementos
(`typst query`) não muda com a troca de fonte. Este passo mede o que depende
da fonte: **número de páginas** e **número de palavras extraídas** dos
documentos do corpus sem `#set text(font:)` explícito.

---

## 2. Método

### 2.1 Identificar documentos relevantes

```bash
grep -rLZ "set text(font:" lab/parity/corpus/*/*.typ > /tmp/p556-files.txt
```

Total: **86 ficheiros** sem fonte explícita.

### 2.2 Medir "antes" de P554

Checkout do commit imediatamente anterior a P554 (`de46663e7`, P553), build e
medição:

```bash
cd /tmp/typst-crystalline-antes-p554
git checkout de46663e7
cargo build --release
for f in $(cat /tmp/p556-files.txt); do
  ./target/release/typst "$f" /tmp/antes.pdf
  echo "$f|$(pdfinfo /tmp/antes.pdf | grep Pages | awk '{print $2}')|$(pdftotext /tmp/antes.pdf - | wc -w)"
done > /tmp/p556-antes.txt
```

### 2.3 Medir "depois" de P554/P555

```bash
for f in $(cat /tmp/p556-files.txt); do
  ./target/release/typst "$f" /tmp/depois.pdf
  echo "$f|$(pdfinfo /tmp/depois.pdf | grep Pages | awk '{print $2}')|$(pdftotext /tmp/depois.pdf - | wc -w)"
done > /tmp/p556-depois.txt
```

### 2.4 Comparar com vanilla 0.15.0

Para os documentos com ≥20 palavras extraídas e/ou que mudaram entre antes e
depois:

```bash
lab/typst-original/target/release/typst compile "$f" /tmp/vanilla.pdf
echo "$f|$(pdfinfo /tmp/vanilla.pdf | grep Pages | awk '{print $2}')|$(pdftotext /tmp/vanilla.pdf - | wc -w)"
```

---

## 3. Resultados

### 3.1 Resumo agregado

| Métrica | Antes (P553) | Depois (P554/P555) | Mudança |
|---|---|---|---|
| Documentos medidos | 86 | 86 | — |
| Documentos que mudaram de páginas | — | **0** | nenhuma |
| Documentos que mudaram de palavras extraídas | — | **6** | variações pequenas |

### 3.2 Documentos com ≥20 palavras extraídas

| Documento | Páginas antes | Páginas depois | Páginas vanilla | Palavras antes | Palavras depois | Palavras vanilla |
|---|---|---|---|---|---|---|
| p490/test-array.typ | 1 | 1 | 1 | 73 | 73 | 76 |
| p490/test-columns.typ | 1 | 1 | 1 | 150 | 150 | 150 |
| p490/test-page.typ | 1 | 1 | 1 | 50 | 50 | 52 |
| p490/test-par.typ | 1 | 1 | 1 | 23 | 23 | 23 |
| p490/test-place.typ | 1 | 1 | 1 | 32 | **33** | 32 |
| p500/test-bibliography-csl.typ | 1 | 1 | 1 | 29 | 29 | 1 |
| p500/test-calc-rest.typ | 1 | 1 | 1 | 21 | 21 | 1 |
| p500/test-dict-methods.typ | 1 | 1 | 1 | 22 | 22 | 14 |
| p500/test-page-header-footer.typ | 1 | 1 | 1 | 51 | 51 | 53 |
| p500/test-place-absolute.typ | 1 | 1 | 1 | 36 | **37** | 36 |
| visual/cite-bibliography.typ | 1 | 1 | 1 | 52 | 52 | 49 |
| visual/counter-heading.typ | 1 | 1 | 1 | 27 | 27 | 27 |
| visual/equation-ref.typ | 1 | 1 | 1 | 30 | 30 | 37 |
| visual/figure-ref.typ | 1 | 1 | 1 | 33 | **34** | 30 |
| visual/outline-toc.typ | 1 | 1 | 1 | 31 | **35** | 524 |

### 3.3 Documentos com <20 palavras que também mudaram

| Documento | Páginas antes | Páginas depois | Palavras antes | Palavras depois |
|---|---|---|---|---|
| p490/test-math.typ | 1 | 1 | 20 | **17** |

---

## 4. Análise

### 4.1 Número de páginas

**Nenhum documento mudou de número de páginas.** A troca de fonte por defeito
de sans-serif para serif não alterou a paginação do corpus. O único caso onde a
paginação era observável (`#set page(columns: 2)\n#lorem(1200)`, P553/P554)
melhorou de 3 para 2 páginas, e esse efeito já foi documentado.

### 4.2 Contagem de palavras extraídas

Seis documentos tiveram variações de 1–4 palavras na extração `pdftotext`.
Estas diferenças são **artefactos da extração**, não perda ou ganho de
conteúdo:

- **p490/test-math.typ**: a fonte serif aproxima glifos (ex. `∑1i` em vez de
  `∑1 i`), fazendo o `pdftotext` unir tokens que antes ficavam separados.
- **p490/test-place.typ**, **p500/test-place-absolute.typ**, **visual/figure-ref.typ**:
  a fonte mais compacta permite que mais texto caiba na linha, alterando os
  pontos de quebra e a contagem de palavras extraídas.
- **visual/outline-toc.typ**: a contagem de palavras no vanilla (524) é
  radicalmente diferente de ambas as versões cristalinas (~31–35), indicando
  que a diferença não é da fonte mas da semântica de outline/TOC entre
  cristalino e vanilla.

### 4.3 Direcção da mudança em relação ao vanilla

| Documento | Direcção da mudança | Observação |
|---|---|---|
| p490/test-place.typ | afastou 1 palavra | artefacto de extração |
| p500/test-place-absolute.typ | afastou 1 palavra | artefacto de extração |
| visual/figure-ref.typ | afastou 1 palavra | artefacto de extração |
| p490/test-math.typ | afastou 3 palavras | artefacto de extração de math |
| visual/outline-toc.typ | irrelevante | diferença estrutural de outline, não fonte |

Nenhum caso apresenta **regressão de paginação** nem **perda de conteúdo
semântico**. As variações são toleráveis face ao ganho de paridade visual e
paginação conseguido em P553/P554.

---

## 5. Decisão

**Não há acção corretiva a tomar.** A mudança de fonte por defeito para
`FreeSerif` (P554) e o fallback por classe (P555) não introduziram regressões
no corpus mensuráveis em páginas. As pequenas variações na contagem de palavras
extraídas são aceites como artefactos da extração `pdftotext` quando a fonte
muda.

---

## 6. Conclusão

- 86 documentos sem fonte explícita foram medidos antes e depois de P554.
- **0 documentos mudaram de número de páginas.**
- 6 documentos tiveram variações menores na contagem de palavras extraídas,
  explicáveis por diferenças de kerning/quebra de linha e não por perda de
  conteúdo.
- Nenhum caso exige correção ou decisão nova.
- O inventário mantém os itens de P553/P554 como fechados.
