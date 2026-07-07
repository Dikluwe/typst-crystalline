# Relatório Diagnóstico — Passo 590
## Remedir o documento de referência RTL com fonte neutra

- **Commit de Referência:** `b0eb192e4` (Working Tree não commitado)
- **Data/Hora da Medição:** 2026-07-07 02:32:27 UTC
- **ADR Base:** `00_nucleo/adr/adr-paridade-defeitos-testes.md`

---

## 1. Objetivo

Aplicar a fonte neutra (`DejaVu Sans`) ao documento de referência RTL usado desde P563 até P588, e comparar cristalino vs vanilla número a número, para responder se o algoritmo de RTL está finalmente correcto agora que o ruído de fonte foi eliminado.

---

## 2. Documento de teste

```typst
#set text(dir: rtl, lang: "ar", size: 40pt, font: "DejaVu Sans")
الكتاب 42 على الطاولة
```

Renderizado em ambos os lados com o mesmo ficheiro `.typ`, mudando apenas o compilador.

---

## 3. Resultados

### 3.1 Vanilla (com shaping)

```text
level	page_num	par_num	block_num	line_num	word_num	left	top	width	height	conf	text
1	1	0	0	0	0	0.000000	0.000000	595.275600	841.889800	-1	###PAGE###
3	1	0	0	0	0	136.362580	70.866176	388.046871	39.999999	-1	###FLOW###
4	1	0	0	0	0	136.362580	70.866176	388.046871	39.999999	-1	###LINE###
5	1	0	0	0	0	136.36	70.87	121.25	40.00	100	ةلواطلا
5	1	0	0	0	1	270.33	70.87	70.45	40.00	100	ىلع
5	1	0	0	0	2	353.49	70.87	50.90	40.00	100	42
5	1	0	0	0	3	417.10	70.87	107.30	40.00	100	باتكلا
```

- **Uma só linha.**
- Ordem visual RTL correcta: `الطاولة` → `على` → `42` → `الكتاب`.
- Total ocupado na linha (com espaços): ≈ `388.05 pt`.

### 3.2 Cristalino (com fonte neutra)

```text
level	page_num	par_num	block_num	line_num	word_num	left	top	width	height	conf	text
1	1	0	0	0	0	0.000000	0.000000	595.280000	841.890000	-1	###PAGE###
3	1	0	0	0	0	205.347000	49.903000	270.011000	98.640000	-1	###FLOW###
4	1	0	0	0	0	205.347000	49.903000	254.098000	40.000000	-1	###LINE###
5	1	0	0	0	0	205.35	49.90	70.44	40.00	100	ىلع
5	1	0	0	0	1	288.51	49.90	50.88	40.00	100	42
5	1	0	0	0	2	352.13	49.90	107.32	40.00	100	باتكلا
4	1	0	0	1	0	354.078000	108.543000	121.280000	40.000000	-1	###LINE###
5	1	0	0	1	0	354.08	108.54	121.28	40.00	100	ةلواطلا
```

- **Duas linhas.**
- A quebra prematura **persiste** mesmo com a mesma fonte.

---

## 4. Análise

### 4.1 Larguras shaped no PDF (ambos os lados)

| Palavra | Cristalino (pt) | Vanilla (pt) | Diferença |
|---|---|---|---|
| `الكتاب` | 107.32 | 107.30 | 0.02 |
| `42` | 50.88 | 50.90 | 0.02 |
| `على` | 70.44 | 70.45 | 0.01 |
| `الطاولة` | 121.28 | 121.25 | 0.03 |

As larguras **finais** das palavras batem quase exactamente. O problema não é a fonte.

### 4.2 Larguras não-shaped medidas no cristalino

Usando `fontTools` para somar advances individuais de cada caractere em `DejaVu Sans` (upem = 2048, size = 40 pt):

| Palavra | Largura não-shaped (pt) | Largura shaped no PDF (pt) | Diferença |
|---|---|---|---|
| `الكتاب` | 159.57 | 107.32 | 52.25 |
| `42` | 50.90 | 50.88 | 0.02 |
| `على` | 84.24 | 70.44 | 13.80 |
| `الطاولة` | 157.62 | 121.28 | 36.34 |

- Soma não-shaped das 4 palavras: **452.32 pt**.
- 3 espaços (` ` advance = 12.71 pt cada): **38.14 pt**.
- **Total não-shaped: 490.47 pt**.
- Largura útil da página A4: `595.28 − 2 × 70.87 = 453.54 pt`.

O total não-shaped **excede** a largura útil em `36.93 pt`. Por isso o cristalino quebra a linha.

O total **shaped** seria aproximadamente `388.05 pt` (do vanilla), cabendo confortavelmente.

### 4.3 Conclusão da causa

O cristalino mede a largura de cada palavra árabe **caractere a caractere**, sem aplicar shaping árabe (ligações contextuais). O render final dos itens de texto parece usar larguras shaped, mas a **decisão de quebra de linha** usa larguras não-shaped. Como as palavras árabes são significativamente mais largas antes do shaping, o cristalino decide quebrar prematuramente.

Esta é uma **diferença de algoritmo**, não de fonte. A fonte neutra removeu o ruído de `Liberation Serif` vs `Libertinus Serif`, mas revelou um bug de shaping subjacente.

---

## 5. Resposta às perguntas do passo

1. **As quatro palavras ficam na mesma linha, nos dois lados?**
   - Vanilla: sim.
   - Cristalino: **não** — quebra após `الكتاب 42 على`.

2. **A ordem visual está correcta?**
   - Vanilla: sim.
   - Cristalino: sim, dentro de cada linha. Mas a quebra coloca `الطاولة` sozinha na segunda linha.

3. **As posições batem certo dentro de 1 ponto?**
   - Não aplicável: o cristalino tem uma linha a mais, logo as posições são estruturalmente diferentes.

4. **A diferença é de algoritmo ou de outra coisa?**
   - É de **algoritmo**: o cristalino não aplica shaping árabe na medição de largura para decisão de quebra de linha.

---

## 6. Decisão

A sequência RTL **não está fechada**. Depois de eliminar o ruído de fonte, resta um bug real de shaping/medição de largura em texto árabe. A diferença residual de P588 (0.46 pt) era fonte; a diferença agora observada é algoritmo.

Próximo passo necessário: fazer com que o cristalino use larguras **shaped** (ou equivalentes) ao decidir quebras de linha para scripts que requerem shaping (árabe, e potencialmente outros scripts ligados/contextuais).

---

## 7. Tabela de estado da sequência RTL

| Passo | Estado | Nota |
|---|---|---|
| P560–P565 | Aberto | Medições podem ter ruído de fonte + shaping. |
| P566–P569 | Aberto | Mesmo documento de referência; ruído de shaping confirmado. |
| P570–P578 | Aberto | Documentos sem fonte neutra; precisam de re-teste. |
| P586–P588 | Aberto | P588 corrigiu espaço inicial, mas a quebra prematura persiste por shaping. |
| **P590** | **Aberto** | Fonte neutra confirmada; bug de shaping identificado. |

---

## 8. Validação

```bash
cargo build --workspace
cargo test --workspace
crystalline-lint .
```

Resultados:

- `cargo build --workspace`: sucesso.
- `cargo test --workspace`: sucesso (`typst-core` 3572 passados, `typst-infra` 597 passados, etc.).
- `crystalline-lint .`: `✓ No violations found`.

Nenhuma alteração de código foi feita neste passo — apenas medição e diagnóstico.
