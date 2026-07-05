# Relatório de Verificação — P563: Custo e Distorção da Reordenação Bidi

**Passo:** 563  
**Data de execução:** 2026-07-04  
**Foco:** Medir o custo real da passagem de reordenação bidi (P562) e quantificar a distorção de espaçamento em texto misto, antes de aceitar o fecho de P562 como definitivo.

---

## Parte 1 — Custo real da passagem de reordenação bidi

### Documento de teste

```typst
#lorem(1200)
```

Texto latino puro, 1200 palavras, sem qualquer RTL.

### Metodologia

Medições a frio, 3 execuções seguidas, com o binário `typst` em release:

```bash
for i in 1 2 3; do
    time ./target/release/typst /tmp/p563-latin-large.typ /tmp/p563.pdf
done
```

A comparação foi feita entre:

- **Com P562:** commit `70c39dabd` (*P562: implementa reordenação visual RTL de linhas*).
- **Sem P562:** commit `fcd8ab06d` (*P561: sonda RTL confirma layout bidi não implementado no cristalino*), imediatamente anterior.

Para medir sem P562, fiz checkout temporário de `fcd8ab06d`, rebuild release, medi, e voltei ao HEAD.

### Resultados

| Execução | Com P562 (real) | Sem P562 (real) |
|----------|-----------------|-----------------|
| 1        | 0.290 s         | 0.291 s         |
| 2        | 0.286 s         | 0.288 s         |
| 3        | 0.285 s         | 0.288 s         |
| **Média**| **0.287 s**     | **0.289 s**     |

**Diferença média:** −0.002 s (com P562 ligeiramente mais rápido, dentro da variação de medição).

### Conclusão da Parte 1

O custo da passagem de reordenação bidi é **desprezável** para documentos LTR. A afirmação de que a passagem "sai logo" para texto puramente LTR está confirmada numericamente.

---

## Parte 2 — Distorção de espaçamento em texto misto

### Documento de teste

```typst
#set text(lang: "ar", size: 40pt)
الكتاب 42 على الطاولة
```

### Metodologia

Compilei o documento com o cristalino (com P562) e com o vanilla, e extraí as posições x e y de cada palavra via `pdftotext -tsv`:

```bash
./target/release/typst /tmp/p563-mixed.typ /tmp/p563-mixed.pdf
lab/typst-original/target/release/typst compile /tmp/p563-mixed.typ /tmp/p563-mixed-vanilla.pdf
pdftotext -tsv /tmp/p563-mixed.pdf /tmp/p563-mixed.tsv
pdftotext -tsv /tmp/p563-mixed-vanilla.pdf /tmp/p563-mixed-vanilla.tsv
```

O texto árabe aparece invertido nos TSVs (leitura visual RTL), por isso relativo à ordem visual da esquerda para a direita.

### Resultados

#### Vanilla (referência)

Todas as palavras na **mesma linha** (baseline y ≈ 65.19 pt):

| Palavra (visual) | x (pt) | largura (pt) |
|------------------|--------|--------------|
| الطاولة          | 73.21  | 168.00       |
| على              | 251.21 | 72.00        |
| 42               | 333.21 | 37.20        |
| الكتاب           | 380.41 | 144.00       |

#### Cristalino (com P562)

| Palavra (visual) | x (pt) | y (pt)  | largura (pt) | linha |
|------------------|--------|---------|--------------|-------|
| الطاولة          | 70.87  | 66.03   | 168.00       | 2     |
| على              | 80.87  | 49.90   | 72.00        | 1     |
| 42               | 234.87 | 49.90   | 40.00        | 1     |
| الكتاب           | 284.87 | 49.90   | 144.00       | 1     |

### Análise das diferenças

- **الطاولة**: esperado x=73.21, y=65.19 (linha 1); real x=70.87, y=66.03 (linha 2).  
  Diferença horizontal: 2.34 pt. Diferença vertical: 0.84 pt (mudança de linha).
- **على**: esperado x=251.21; real x=80.87.  
  Diferença horizontal: **170.34 pt**.
- **42**: esperado x=333.21; real x=234.87.  
  Diferença horizontal: **98.34 pt**.
- **الكتاب**: esperado x=380.41; real x=284.87.  
  Diferença horizontal: **95.54 pt**.

### Problema identificado

A inversão simples de P562 preserva as posições x dos items originais. Como `الطاولة` é mais larga do que `على` (a palavra que ocupava a primeira posição x no layout LTR), colocar `الطاولة` em x=70.87 faz com que a palavra estoure a linha e seja movida para a linha seguinte pelo processamento posterior do PDF/export.

Este não é um problema de "espaçamento ligeiramente distorcido" — é uma **quebra de linha incorrecta** que torna o documento ilegível na disposição pretendida.

---

## Decisão

P562 **não está fechado**.

- **Parte 1 (desempenho):** aprovado. Custo desprezável em documentos LTR.
- **Parte 2 (distorção):** reprovado. A distorção de espaçamento em texto misto excede largamente o limiar de 1 pt e resulta em quebra de linha incorrecta.

### Acção recomendada

Reabrir a camada de "ordem visual das palavras na linha" com um passo de correcção próprio. A correcção deve recalcular as posições x dos items reordenados com base nas larguras reais das palavras, em vez de preservar as posições x impostas pelo layout LTR original.

---

## Estado da sequência de RTL

| Camada | Passo | Estado |
|--------|-------|--------|
| Shaping (formas das letras) | P484, P521 | Fechado |
| Fonte embutida no PDF | P560 | Fechado |
| Ordem visual das palavras na linha | P562 | **Reaberto — distorção mensurável** |
