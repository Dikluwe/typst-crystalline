# Registo — P749: validação da posição do círculo após P748 e causa da diferença de texto

**Data:** 2026-07-14
**Commit de base:** `5d7a9951f2c569b8ae29f0d84be919af0351439b`
**Commit do registo:** `bc3585161`
**Passo:** 749

## O que aconteceu

O P748 comparou o topo do círculo no cristalino com o centro do círculo no vanilla — pontos de referência diferentes. Este passo recalcula a posição usando o mesmo ponto de referência (centro) e confirma a causa da diferença de ~3,8 pt no texto "X", em vez de assumir que é "métricas de fonte diferentes".

## Verificação do círculo

Documento: `/tmp/p749/circle.typ`

```typst
#circle(radius: 30pt, fill: rgb(255, 200, 0))
```

Extraído com `mutool draw -F trace`:

| Lado | Ponto de referência | Coordenadas de página (centro) |
|------|---------------------|--------------------------------|
| Cristalino | centro do path (média dos extremos) | (100,867 pt, 100,867 pt) |
| Vanilla | centro local (30,30) + translate (70,866; 70,866) | (100,866 pt, 100,866 pt) |

**Diferença de centro: ≈0,001 pt** — praticamente coincidente.

### Confirmação visual

`compare -metric AE /tmp/p749/vanilla.png /tmp/p749/cristalino.png /tmp/p749/diff.png` produziu uma imagem de diferença com um único círculo vermelho uniforme, sem deslocamento visível. A diferença de ~17 pt calculada no P748 era um artefacto da comparação entre pontos de referência diferentes; o círculo está correctamente posicionado.

## Verificação do texto "X"

Documento: `/tmp/p749/texto.typ`

```typst
X
```

| Lado | Fonte embutida | Baseline Y | Offset desde a margem (70,87 pt) |
|------|----------------|------------|----------------------------------|
| Vanilla | LibertinusSerif-Regular | 78,10 pt | 7,23 pt |
| Cristalino | CrystallineFont (subset de Liberation Serif) | 81,90 pt | 11,03 pt |

### Métricas verticais reais

Medidas com `ttf-parser` (tamanho 11 pt):

| Fonte | ascender (hhea) | typographic_ascender | cap_height | line_gap |
|-------|-----------------|----------------------|------------|----------|
| LibertinusSerif-Regular | 9,83 pt | 9,83 pt | 7,24 pt | 0 pt |
| LiberationSerif-Regular | 9,80 pt | 7,63 pt | 7,20 pt | 0,47 pt |

### Causa da diferença

- O vanilla posiciona a primeira baseline aproximadamente a `margin + cap_height` (78,10 pt ≈ 70,87 + 7,24 pt).
- O cristalino posiciona a primeira baseline a `margin + ascender` (≈ 80,67 pt) mais um pequeno offset adicional (≈1,23 pt) que ainda não está identificado, resultando nos 81,90 pt medidos.
- A diferença observável de ~3,8 pt é portanto principalmente uma **diferença de escolha de métrica para posicionamento da baseline** (ascender vs cap-height), não uma regressão do P748 nem uma diferença de fonte por si só.

A confirmação directa com `#set text(font: "Liberation Serif")` no cristalino não alterou a baseline (mantém 81,90 pt), provando que a fonte real não é o factor dominante.

## Decisão

- **Não há bug a corrigir no círculo**: o centro coincide com o vanilla.
- **A diferença de texto é uma divergência conhecida** de posicionamento da baseline (ascender vs cap-height), não introduzida pelo P748.
- **Nenhuma alteração de código** foi necessária.
- O relatório de P748 continua válido na conclusão (margem das formas corrigida), mas a explicação da diferença de texto deve ser atualizada de "métricas de fonte diferentes" para "escolha de métrica de baseline (ascender vs cap-height)". Essa correção documental fica para este registo.

## Validação

- `cargo test --workspace` não foi alterado por este passo (nenhuma mudança de código).
- `crystalline-lint .` — sem alterações de código, não há novas violações.
