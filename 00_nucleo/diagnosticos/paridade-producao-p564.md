# Relatório de Verificação — P564: Correcção do Posicionamento Bidi

**Passo:** 564  
**Data de execução:** 2026-07-04  
**Foco:** Corrigir o recálculo das posições x na passagem de reordenação bidireccional (P562), usando as larguras reais dos items em vez de preservar as coordenadas do layout LTR.

---

## Resumo da correcção

A implementação de P562 invertia apenas os textos/estilos dos `FrameItem::Text` dentro de cada linha RTL, mantendo as posições x impostas pelo `Layouter` LTR. Em texto misto (`الكتاب 42 على الطاولة`), isso colocava `الطاولة` numa posição x demasiado estreita, fazendo-a saltar para a linha seguinte (ver `paridade-producao-p563.md`).

P564 altera `03_infra/src/layout_bidi.rs` para:

1. Receber `metrics: &dyn FontMetrics` (via `FallbackFontMetrics::new(world)` na pipeline).
2. Medir cada item com `FontMetrics::advance(text, size, style)`.
3. Inferir o gap original entre palavras a partir das posições x e das larguras reais.
4. Recalcular as posições x na nova ordem visual, preservando o início da linha e o espaçamento entre palavras.

A fórmula do gap foi corrigida para usar a posição x do último item (não o seu fim), evitando gaps negativos e sobreposição:

```text
gap = (x_max - x_min - sum(widths[0..n-1])) / (n - 1)
```

---

## Alterações de ficheiros

- `03_infra/src/layout_bidi.rs` — API alterada para `reorder_bidi_document(doc, metrics)`; lógica de recálculo de x; testes actualizados para `p564_*`.
- `03_infra/src/pipeline.rs` — chama `reorder_bidi_document` com `FallbackFontMetrics::new(world)`.
- `00_nucleo/prompts/infra/layout_bidi.md` — L0 actualizado para refletir a nova API e o recálculo de x (hash `f958068a`).

---

## Validação com documentos de teste

### 1. Texto árabe puro (`الكتاب على الطاولة`, 40 pt)

Todas as palavras na mesma linha; ordem visual correta; sem sobreposição.

#### Cristalino

| Palavra (visual) | x (pt) | largura (pt) |
|------------------|--------|--------------|
| الطاولة          | 80.87  | 168.00       |
| على              | 258.87 | 72.00        |
| الكتاب           | 340.87 | 144.00       |

#### Vanilla

| Palavra (visual) | x (pt) | largura (pt) |
|------------------|--------|--------------|
| الطاولة          | 120.41 | 168.00       |
| على              | 298.41 | 72.00        |
| الكتاب           | 380.41 | 144.00       |

**Análise:** a ordem visual e os espaçamentos relativos estão correctos. A diferença absoluta de x (≈ 40 pt) é esperada porque o cristalino ainda não implementa alinhamento RTL à direita (`dir: rtl`) — o parágrafo começa à esquerda. As larguras e a sequência visual coincidem com o vanilla.

---

### 2. Texto misto que cabe numa linha (`الكتاب 42 على الطاولة`, 20 pt)

Todas as palavras na mesma linha; o número 42 mantém-se LTR no meio.

#### Cristalino

| Palavra (visual) | x (pt) | largura (pt) |
|------------------|--------|--------------|
| الطاولة          | 75.87  | 84.00        |
| على              | 164.87 | 36.00        |
| 42               | 205.87 | 20.00        |
| الكتاب           | 230.87 | 72.00        |

#### Vanilla

| Palavra (visual) | x (pt) | largura (pt) |
|------------------|--------|--------------|
| الطاولة          | 298.81 | 84.00        |
| على              | 387.81 | 36.00        |
| 42               | 428.81 | 18.60        |
| الكتاب           | 452.41 | 72.00        |

**Análise:** a sequência visual está correcta (`الطاولة` → `على` → `42` → `الكتاب`). Não há quebra de linha nem sobreposição. A diferença de posição x absoluta deve-se, novamente, ao alinhamento de parágrafo (esquerdo no cristalino, direito no vanilla).

---

### 3. Texto misto de 40 pt — caso de referência de P563

Este documento **continua a quebrar linha** no cristalino.

#### Cristalino

| Palavra (visual) | x (pt) | y (pt) | linha |
|------------------|--------|--------|-------|
| على              | 80.87  | 49.90  | 1     |
| 42               | 162.87 | 49.90  | 1     |
| الكتاب           | 212.87 | 49.90  | 1     |
| الطاولة          | 70.87  | 66.03  | 2     |

#### Vanilla

| Palavra (visual) | x (pt) | y (pt) | linha |
|------------------|--------|--------|-------|
| الطاولة          | 73.21  | 65.19  | 1     |
| على              | 251.21 | 65.19  | 1     |
| 42               | 333.21 | 65.19  | 1     |
| الكتاب           | 380.41 | 65.19  | 1     |

**Análise:** P564 resolveu a sobreposição e a ordem visual dentro da primeira linha (`على 42 الكتاب`), mas `الطاولة` ainda é colocada numa segunda linha pelo `Layouter` LTR. O problema de fundo é que o layout LTR decide a quebra de linha antes da reordenação visual; quando o texto não cabe, a última palavra desce. Uma passagem posterior pura não pode, por definição, alterar a quebra de linha sem violar o próprio L0 de `layout_bidi`.

A diferença de alinhamento (esquerdo vs direito) também contribui: no vanilla o parágrafo RTL começa mais à direita, o que dá mais margem para o texto caber na mesma linha. No cristalino o parágrafo começa à esquerda e a largura total excede o espaço disponível.

---

## Custo real da passagem (documento LTR puro)

Medições a frio, 3 execuções seguidas, binário release:

| Execução | Tempo real |
|----------|------------|
| 1        | 0.281 s    |
| 2        | 0.278 s    |
| 3        | 0.275 s    |
| **Média**| **0.278 s**|

Comparando com P563 (`0.287 s`), o custo adicional de medir larguras reais é imperceptível — a diferença está dentro da variação de medição.

---

## Testes e linter

```bash
cargo test -p typst-infra --lib   # 585 passed, 0 failed
crystalline-lint .                # No violations found
```

---

## Decisão

**P564 fecha a corrupção de posições x dentro de cada linha RTL.**

- **Aprovado:** texto árabe puro e texto misto que cabe numa linha produzem ordem visual correcta, sem sobreposição, com custo desprezável em LTR.
- **Reprovado para o caso de 40 pt:** a quebra de linha incorrecta persiste porque o `Layouter` LTR não tem noção de direcção RTL. Corrigir isso exige um passo futuro que actue no layout/quebra de linha (mudar o `Layouter` ou adicionar um passo de reflow), não numa passagem posterior pura.

---

## Estado da sequência de RTL

| Camada | Passo | Estado |
|--------|-------|--------|
| Shaping (formas das letras) | P484, P521 | Fechado |
| Fonte embutida no PDF | P560 | Fechado |
| Ordem visual das palavras na linha | P562/P564 | **Parcial — posições x corrigidas; quebra de linha RTL ainda requer passo futuro** |
| Alinhamento/quebra de linha RTL (`dir: rtl`) | — | Scope-out futuro |
