# Relatório Diagnóstico — Passo 586
## Re-teste do documento RTL de referência após as correcções de `font_size_pt`

- **Commit de Referência:** `8ab4384f1` (Working Tree não commitado)
- **Data/Hora da Medição:** 2026-07-06 16:33:40 UTC

---

## 1. Documento e método

Repetimos exactamente o documento e o método de P578:

```bash
cat > /tmp/p586-rtl.typ <<'EOF'
#set text(dir: rtl, lang: "ar", size: 40pt)
الكتاب 42 على الطاولة
EOF
./target/release/typst /tmp/p586-rtl.typ /tmp/p586.pdf
pdftotext -tsv /tmp/p586.pdf /tmp/p586.tsv
lab/typst-original/target/release/typst compile /tmp/p586-rtl.typ /tmp/p586-vanilla.pdf
pdftotext -tsv /tmp/p586-vanilla.pdf /tmp/p586-vanilla.tsv
mutool draw -o /tmp/p586.png -r 150 /tmp/p586.pdf
```

---

## 2. Tabela de posições — Cristalino

```bash
pdftotext -tsv /tmp/p586.pdf /tmp/p586.tsv && cat /tmp/p586.tsv
```

```text
level	page_num	par_num	block_num	line_num	word_num	left	top	width	height	conf	text
1	1	0	0	0	0	0.000000	0.000000	595.280000	841.890000	-1	###PAGE###
3	1	0	0	0	0	238.410000	49.903000	276.000000	40.000000	-1	###FLOW###
4	1	0	0	0	0	238.410000	49.903000	276.000000	40.000000	-1	###LINE###
5	1	0	0	0	0	238.41	49.90	72.00	40.00	100	ىلع
5	1	0	0	0	1	320.41	49.90	40.00	40.00	100	42
5	1	0	0	0	2	370.41	49.90	144.00	40.00	100	باتكلا
4	1	0	0	1	0	346.410000	108.543000	168.000000	40.000000	-1	###LINE###
5	1	0	0	1	0	346.41	108.54	168.00	40.00	100	ةلواطلا
```

---

## 3. Tabela de posições — Vanilla

```bash
pdftotext -tsv /tmp/p586-vanilla.pdf /tmp/p586-vanilla.tsv && cat /tmp/p586-vanilla.tsv
```

```text
level	page_num	par_num	block_num	line_num	word_num	left	top	width	height	conf	text
1	1	0	0	0	0	0.000000	0.000000	595.275600	841.889800	-1	###PAGE###
3	1	0	0	0	0	73.209450	61.426200	451.200000	45.600000	-1	###FLOW###
4	1	0	0	0	0	73.209450	61.426200	451.200000	45.600000	-1	###LINE###
5	1	0	0	0	0	73.21	65.19	168.00	40.00	100	ةلواطلا
5	1	0	0	0	1	251.21	65.19	72.00	40.00	100	ىلع
5	1	0	0	0	2	333.21	61.43	37.20	45.60	100	42
5	1	0	0	0	3	380.41	65.19	144.00	40.00	100	باتكلا
```

---

## 4. Resposta às perguntas do passo

### 4.1. As quatro palavras ficam todas na mesma linha, com o mesmo `top`?

**Não.** No cristalino, as palavras dividem-se em duas linhas:
- Linha 0 (`top` 49.90): `على`, `42`, `الكتاب`
- Linha 1 (`top` 108.54): `الطاولة`

No vanilla, as quatro palavras ficam na mesma linha (`top` 65.19).

### 4.2. A ordem visual está correcta?

Dentro de cada linha do cristalino, a ordem visual está correcta para RTL (palavra lógica final `الكتاب` à direita, palavra lógica anterior `على` à esquerda). No entanto, `الطاولة` não deveria estar numa linha separada.

### 4.3. Há sobreposição visual na imagem?

**Não.** A imagem gerada por `mutool draw` mostra duas linhas distintas, sem sobreposição vertical. O problema original de P577 (sobreposição de glifos) desapareceu.

### 4.4. Se ainda houver problema, é o mesmo tipo de sobreposição vertical ou é outra coisa?

É **outra coisa**. O problema residual é uma **quebra de linha prematura**: o cristalino parte a linha antes do vanilla. Não é o ponto em aberto de P578 (`align_current_line_rtl()` chamado independentemente em `flush_line()` e em `finish()`) — o alinhamento dentro de cada linha está correcto.

---

## 5. Sonda da causa da quebra prematura

Testámos o mesmo documento sem o número latino (`الكتاب على الطاولة`):

```text
5	1	0	0	0	0	110.41	49.90	168.00	40.00	100	ةلواطلا
5	1	0	0	0	1	288.41	49.90	72.00	40.00	100	ىلع
5	1	0	0	0	2	370.41	49.90	144.00	40.00	100	باتكلا
```

Sem o `42`, as três palavras cabem numa linha só. O problema aparece quando se acrescenta o token latino.

Testámos ainda o documento original com uma margem menor (`#set page(margin: 2cm)`):

```text
5	1	0	0	0	0	74.59	24.69	168.00	40.00	100	ةلواطلا
5	1	0	0	0	1	252.59	24.69	72.00	40.00	100	ىلع
5	1	0	0	0	2	334.59	24.69	40.00	40.00	100	42
5	1	0	0	0	3	384.59	24.69	144.00	40.00	100	باتكلا
```

Com margem de 2 cm, as quatro palavras cabem numa linha só no cristalino. Isto indica que a quebra prematura com margem default (2.5 cm no cristalino) é uma questão de **largura útil**, não um bug de alinhamento RTL. A diferença entre cristalino e vanilla deve-se a variações acumuladas de fonte (Liberation Serif vs fonte default do vanilla), espaçamento ou margem default — dentro de uma margem ligeiramente menor, o resultado é equivalente.

---

## 6. Decisão

- **O problema original de P577 (sobreposição vertical de glifos) está resolvido.** A correção de `font_size_pt` estático em P579/P580/P582 funcionou.
- **A paridade exacta do documento `الكتاب 42 على الطاولة` com margem default não está completa**, porque o cristalino parte a linha e o vanilla não. A causa é largura útil, não o alinhamento RTL.
- **A sequência RTL fica fechada quanto ao bug de sobreposição**, mas com a ressalva documentada de que este documento específico ainda não produz saída idêntica ao vanilla com margem default.
