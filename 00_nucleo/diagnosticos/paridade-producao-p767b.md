# Relatório de verificação — P767b

**Passo:** 767b  
**Data:** 2026-07-15  
**Base:** commit `beb4d4e4f` (P767a)  
**Foco:** Verificar se a correcção de P767a realmente resolveu o caso texto+forma e se não regrediu o baseline de texto puro de P745–P762.

---

## 1. Reconfirmação do baseline de texto puro

O documento exacto de P762 (`/tmp/p762-fixo.typ`) usa `#lorem(50)`. Não foi possível reproduzi-lo literalmente porque o cristalino usa um vocabulário `lorem` diferente do vanilla (sem pontuação) — uma divergência de **conteúdo** documentada em `01_core/src/rules/stdlib/text.rs:586`:

> "O texto exacto não precisa de coincidir com o vanilla; a paridade é semântica — exactamente `n` palavras de Lorem Ipsum."

Para isolar o **layout** do conteúdo, usou-se o mesmo texto como string literal:

```typst
#set page(width: 350pt, margin: 40pt)
#set text(font: "DejaVu Sans", size: 11pt)
Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
```

### Coordenadas Y das primeiras linhas (pdftotext -bbox)

| Linha | Vanilla Y (pt) | Cristalino Y (pt) | ΔY (pt) |
|-------|---------------|-------------------|---------|
| 1 | 40,000028 | 39,999578 | −0,000450 |
| 2 | 55,507478 | 55,507578 | +0,000100 |
| 3 | 71,014878 | 71,014578 | −0,000300 |

Erro máximo: **< 0,001 pt**.

### Comparação visual

- AE = 7 661 (ImageMagick, 300 ppp).
- A diferença pixel-a-pixel é devida a micro-variações de kerning/hinting, não a deslocamento vertical: as coordenadas Y das linhas batem a menos de 0,001 pt.

**Conclusão do Passo 0:** não há regressão real de texto puro introduzida por P767a. O layout vertical de parágrafos de texto mantém o baseline de P745–P762.

---

## 2. Coordenadas exactas do caso misto `A #rect(...) B`

Documento:

```typst
A #rect(width: 1cm, height: 0.8cm, fill: red) B
```

Extraído com `mutool trace` e convertido para o sistema de coordenadas do PDF (origem no canto inferior-esquerdo, Y para cima):

| Elemento | Vanilla Y (pt) | Cristalino Y (pt) | ΔY (pt) |
|----------|---------------|-------------------|---------|
| "A" baseline | 78,104 | 78,105 | +0,001 |
| `rect` inferior | 91,304 | 85,250 | −6,054 |
| `rect` superior | 113,981 | 107,930 | −6,051 |
| "B" baseline | 134,419 | 128,369 | −6,050 |

**Padrão da diferença:** "A" está no mesmo sítio nos dois; o `rect` e o "B" estão deslocados **para baixo** no cristalino em aproximadamente **6,05 pt**.

O valor 6,05 pt corresponde a:

```text
1.2em − cap_height ≈ 13,2 pt − 7,145 pt = 6,055 pt
```

onde `1.2em` é o `above`/`below` por defeito aplicado em P767a (`SHAPE_BLOCK_SPACING_EM`, `01_core/src/rules/layout/shape.rs:20`) e `cap_height` é a altura da maiúscula da fonte (medida a partir das coordenadas: 85,250 − 78,105 = 7,145 pt).

---

## 3. Causa identificada por leitura de código

A diferença não é "espaçamento de parágrafo de texto puro" — o texto puro não regrediu. A causa está no **ancoramento vertical do bloco de forma**.

Em `01_core/src/rules/layout/shape.rs:60-66`, o `above` de `1.2em` só é aplicado se `layouter.block_chain_active` estiver verdadeiro:

```rust
let gap = if layouter.block_chain_active {
    layouter.prev_block_below_pending.max(above_pt)
} else {
    0.0
};
```

Em `01_core/src/rules/layout/sequence.rs:55`, o estado `block_chain_active` é posto a `false` depois de conteúdo que não é bloco (como o texto "A"):

```rust
if !matches!(part, Content::Block { .. } | Content::Shape(_)) {
    layouter.block_chain_active = false;
    layouter.prev_block_below_pending = 0.0;
}
```

Por isso, quando a forma é a **primeira** numa sequência depois de texto, o `above` é suprimido. O código coloca o topo da forma ao nível do topo da linha (`cursor_y − cap_height`, `shape.rs:85`), ou seja, o rect começa a `baseline + cap_height` no PDF.

No vanilla, pelo contrário, o `rect` comporta-se como um bloco cuja base fica a `baseline + above` (a próxima baseline da linha), e depois estende-se para cima pela sua altura. O desvio entre os dois modelos é exactamente `above − cap_height`.

A estrutura de acessibilidade do PDF do vanilla confirma este modelo: "A" e o `rect` partilham o mesmo `<structure standard="P">`, e "B" inicia um novo parágrafo. O `rect` é um bloco inline dentro do parágrafo, posicionado na grelha de linhas, não um bloco de fluxo com colapso de margem.

---

## 4. Conclusão

- **Não há regressão de texto puro.** O layout vertical de parágrafos de texto mantém-se alinhado com o vanilla (< 0,001 pt).
- **A explicação de P767a sobre "espaçamento de parágrafo de texto puro" é incorrecta.** O AE alto no caso misto não vem do texto; vem do posicionamento vertical do próprio `rect`.
- **P767a não resolveu completamente o problema.** A forma ainda desvia ~6 pt do vanilla porque o modelo de bloco implementado usa um ancoramento diferente do `BlockElem::single_layouter` do vanilla.
- **Causa específica:** supressão do `above` spacing para a primeira forma após texto (`block_chain_active == false`) e alinhamento do topo da forma ao topo da linha (`baseline + cap_height`) em vez da base da forma à próxima baseline (`baseline + above`).

---

## 5. Próximo passo recomendado

P767c deve corrigir o ancoramento vertical de `Content::Shape` no fluxo principal, alinhando-o com a grelha de linhas do parágrafo (`baseline + above` como base do rect), e verificar as coordenadas antes/depois com a mesma disciplina de medição usada aqui.
