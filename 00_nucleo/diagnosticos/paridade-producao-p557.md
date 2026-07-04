# Paridade de Produção — Passo 557

**Data:** 2026-07-03  
**Repositório:** `typst-crystalline`  
**Binários:**

- Cristalino: `./target/release/typst`
- Vanilla 0.15.0: `lab/typst-original/target/release/typst`
- Ferramentas: `pdfinfo`, `pdftotext`

---

## 1. Objetivo

Investigar a disparidade de palavras extraídas por `pdftotext` no documento
`lab/parity/corpus/visual/outline-toc.typ`, reportada no Passo 556:

| Versão | Palavras extraídas |
|---|---|
| Cristalino (P555/P556) | 31–35 |
| Vanilla 0.15.0 | 524 |

O objectivo é determinar se a diferença representa perda de conteúdo semântico
no cristalino ou um artefacto de formatação do outline gerado pelo vanilla.

---

## 2. Método

### 2.1 Documento sob análise

```typst
#outline()

= Introdução
Conteúdo da secção 1.

= Métodos
Conteúdo da secção 2.

= Resultados
Conteúdo da secção 3.

= Discussão
Conteúdo da secção 4.

= Conclusão
Conteúdo da secção 5.
```

### 2.2 Renderização e extração

```bash
./target/release/typst lab/parity/corpus/visual/outline-toc.typ /tmp/p557-cristalino.pdf
lab/typst-original/target/release/typst compile lab/parity/corpus/visual/outline-toc.typ /tmp/p557-vanilla.pdf

pdftotext /tmp/p557-cristalino.pdf /tmp/p557-cristalino.txt
pdftotext /tmp/p557-vanilla.pdf /tmp/p557-vanilla.txt

wc -w /tmp/p557-cristalino.txt /tmp/p557-vanilla.txt
pdfinfo /tmp/p557-cristalino.pdf | grep Pages
pdfinfo /tmp/p557-vanilla.pdf | grep Pages
```

---

## 3. Resultados

### 3.1 Páginas

Ambos os PDFs têm **1 página**.

### 3.2 Contagem de palavras

| Versão | Palavras extraídas |
|---|---|
| Cristalino | 35 |
| Vanilla | 524 |

### 3.3 Texto extraído — Cristalino

```text
Índicé
Introdução
Métodos
Résultãdos
Discussão
Conclusão

Introduç o
Contéudo dã sécção 1.
M todos
Contéudo dã sécção 2.
Résultãdos
Contéudo dã sécção 3.
Discuss o
Contéudo dã sécção 4.
Conclus o
Contéudo dã sécção 5.
```

### 3.4 Texto extraído — Vanilla

```text
Contents
Introdução . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . ⁠1
Métodos . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . ⁠1
Resultados . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . ⁠1
Discussão . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . ⁠1
Conclusão . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . ⁠1

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

### 3.5 Contagem normalizada

Removendo as sequências de leaders/dots do vanilla, a contagem desce para
aproximadamente **40 palavras** (ainda inclui os números de página e o título
"Contents"). O cristalino mantém 35 palavras.

A diferença residual de ~5 palavras explica-se por:

- Título "Contents" (vanilla) vs "Índicé" (cristalino) — 1 palavra cada.
- Acentos partidos no cristalino, que fazem o `pdftotext` dividir uma palavra
  em dois tokens (por exemplo, "Introduç o" em vez de "Introdução").

---

## 4. Análise

### 4.1 Causa principal da disparidade

A diferença de 524 vs 35 palavras não é perda de conteúdo semântico. É causada
pelos **leaders/dots e números de página** que o vanilla gera automaticamente no
`#outline()`, enquanto o cristalino gera um índice minimal sem esses elementos.

O cristalino implementa o layout de outline em
`01_core/src/rules/layout/outline.rs` (P457) e emite uma linha por heading com:

- título default "Índice";
- indentação por nível;
- prefixo numérico (se existir);
- corpo do heading;
- número da página (quando disponível em `runtime.known_page_numbers`).

Não está implementado:

- **fill/leader** entre o corpo do heading e o número de página;
- alinhamento do número de página à margem direita.

Esta lacuna já tinha sido identificada no Passo 539 (`investigacao-p539.md`,
item 14) como **"Sem leaders/dots" / Bug grande**.

### 4.2 Conteúdo semântico

O conteúdo real do documento está presente em ambas as versões:

| Elemento | Cristalino | Vanilla |
|---|---|---|
| Título do outline | "Índicé" | "Contents" |
| 5 entradas de outline | Introdução, Métodos, Résultãdos, Discussão, Conclusão | Introdução, Métodos, Resultados, Discussão, Conclusão |
| 5 headings do corpo | presentes (acentos partidos) | presentes |
| 5 frases de corpo | presentes (acentos partidos) | presentes |

Não há secções omitidas nem texto perdido.

### 4.3 Acentos partidos no cristalino

A extração do cristalino mostra palavras como:

- `Introduç o` (deveria ser `Introdução`)
- `M todos` (deveria ser `Métodos`)
- `Discuss o` / `Conclus o`
- `Résultãdos` no outline (deveria ser `Resultados`)
- `Contéudo dã sécção` (deveria ser `Conteúdo da secção`)

Este sintoma aponta para um problema de **ToUnicode / subsetting / mapeamento de
 glifos** no export PDF do cristalino. É observável em outros documentos do
corpus (P539, ocorrências 10 e 15–19) e está fora do escopo deste passo, mas
vale como nota adicional: a contagem de palavras extraídas no cristalino é
inflacionada artificialmente por palavras partidas, o que reduz ainda mais a
relevância da métrica bruta `wc -w`.

---

## 5. Decisão

**Não há acção corretiva imediata a tomar neste passo.** A disparidade 524 vs
35 palavras é um artefacto da formatação do outline, não uma regressão de
conteúdo.

- O item **"Sem leaders/dots"** do inventário de paridade (P539) permanece
  **aberto** até que o layout de outline implemente `fill`/`leader` e alinhamento
  do número de página.
- A contagem bruta de palavras extraídas (`pdftotext | wc -w`) **não deve ser
  usada como critério de paridade** para documentos com outline até que este gap
  seja fechado.
- O problema de acentos partidos no cristalino deve ser tratado separadamente,
  provavelmente na trilha de subsetting/ToUnicode.

---

## 6. Conclusão

- O documento `visual/outline-toc.typ` compila sem erros em ambas as versões e
  produz 1 página.
- A diferença de 524 vs 35 palavras extraídas é explicada pelos **leaders/dots e
  números de página** gerados pelo vanilla, que o cristalino ainda não
  implementa.
- O conteúdo semântico (5 headings + 5 frases) está intacto no cristalino.
- A lacuna de leaders/dots já está registada no inventário de paridade (P539).
- Nenhuma alteração de código foi necessária neste passo.

---

## 7. Anexos

### 7.1 Comandos de reprodução

```bash
./target/release/typst lab/parity/corpus/visual/outline-toc.typ /tmp/p557-cristalino.pdf
lab/typst-original/target/release/typst compile lab/parity/corpus/visual/outline-toc.typ /tmp/p557-vanilla.pdf
pdftotext /tmp/p557-cristalino.pdf /tmp/p557-cristalino.txt
pdftotext /tmp/p557-vanilla.pdf /tmp/p557-vanilla.txt
wc -w /tmp/p557-cristalino.txt /tmp/p557-vanilla.txt
```

### 7.2 Referências de código

- Layout do outline: `01_core/src/rules/layout/outline.rs`
- Definição de `OutlineElem`: `01_core/src/entities/elements/outline.rs`
- Inventário original: `00_nucleo/diagnosticos/investigacao-p539.md`
