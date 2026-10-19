# Verificação do sinal do delta `TJ` — Passo 549

**Data:** 2026-07-03
**Repositório:** `typst-crystalline`
**Binários:**

- Cristalino: `./target/release/typst` (P548, hash `6a3e3a3a7`)
- Vanilla: `/usr/local/bin/typst` (0.14.2) e `lab/typst-original/target/release/typst` (0.15.0)
- Ferramentas: `mutool` 1.23.10

---

## 1. Objetivo

P548 trocou o sinal do delta no operador PDF `TJ` de `(x_advance - nominal)` para `(nominal - x_advance)`. Este passo verifica directamente os números do `TJ` contra o vanilla, sem confiar apenas no texto extraído por `pdftotext`.

---

## 2. Caso `AV` — kerning negativo conhecido

Documento:

```typst
#set text(font: "DejaVu Sans", size: 48pt)
AV
```

### 2.1 Operador `TJ`

**Cristalino (obj 4):**

```text
[ <0001> 64 <0002> 0 ] TJ
```

**Vanilla (obj 15):**

```text
[(
\000\001) 63.964844 (\000\002)] TJ
```

Ambos apresentam um **número positivo** entre o glifo "A" e o glifo "V".

### 2.2 Posições reais dos glifos

| Compilador | A x | V x | Distância A→V |
|---|---:|---:|---:|
| Cristalino | 86,128 | 115,888 | 29,760 |
| Vanilla | 0,000 | 29,761689 | 29,761689 |

Diferença: **0,002 pt** — imperceptível.

### 2.3 Interpretação

`AV` tem kerning negativo em DejaVu Sans: o avanço real (`x_advance`) é menor que a largura nominal declarada no `/W` do CIDFont. Logo `nominal - x_advance > 0`. No operador PDF `TJ`, o número é **subtraído** da coordenada horizontal antes de desenhar o próximo glifo. Um valor positivo move o cursor para a esquerda, aproximando o "V" do "A". O vanilla faz exactamente o mesmo. A fórmula de P548 reproduz o comportamento do vanilla.

---

## 3. Caso `Texto Type Toe Tyler Yellow Yesterday`

Documento:

```typst
#set text(font: "DejaVu Sans")
Texto Type Toe Tyler Yellow Yesterday
```

### 3.1 Operador `TJ` (excertos)

**Cristalino (obj 4):**

```text
[ <0001> 170 <0005> 18 <000D> 0 <000B> 0 <0007> 0 ] TJ
[ <0001> 156 <000E> 0 <0008> 0 <0005> 0 ] TJ
[ <0001> 170 <0007> 0 <0005> 0 ] TJ
[ <0001> 156 <000E> 0 <0006> 0 <0005> 0 <0009> 0 ] TJ
[ <0002> 133 <0005> 0 <0006> 0 <0006> 0 <0007> 0 <000C> 0 ] TJ
```

**Vanilla (obj 15):**

```text
[(
\000\002\000\r\000\004\000\002\000\n) 17.578125 (\000\016\000\017\000\007)] TJ
```

Ambos usam valores **positivos** nos deltas entre glifos com kerning negativo (ex.: `T`→`e`, `T`→`y`, `Y`→`e`).

### 3.2 Posições reais dos glifos (primeiras palavras)

| Par | Cristalino Δx | Vanilla Δx | Diferença |
|---|---:|---:|---:|
| T → e (`Texto`) | 4,850998 | 4,851860 | 0,000862 |
| T → y (`Type`) | 5,004990 | 5,007624 | 0,002634 |
| T → o (`Toe`) | 4,851000 | 4,851862 | 0,000862 |
| T → y (`Tyler`) | 5,005000 | 5,007620 | 0,002620 |
| Y → e (`Yellow`) | 5,257990 | 5,260070 | 0,002080 |

As diferenças são sub-miliponto e explicáveis por arredondamentos de `f64` na emissão do `TJ` (o cristalino arredonda para inteiro, o vanilla usa frações).

---

## 4. Conclusão

A fórmula de **P548 está correcta**:

```text
advance_tu = (nominal - x_advance) / upm * 1000
```

A fórmula anterior `(x_advance - nominal)` estava errada desde P520/P521. A troca de sinal em P548 não introduziu uma regressão; corrigiu um erro que tinha sido mascarado pela combinação de:

1. **Medição de largura sem kerning em P544** — reservava mais espaço do que o shaper usava.
2. **Delta TJ com sinal errado** — afastava os glifos no PDF.

Os dois erros compensavam-se parcialmente no texto extraído, mas produziam posicionamento real incorrecto. Quando P548 corrigiu a medição (kerning) e o delta (sinal), o posicionamento alinhou-se com o vanilla.

### Impacto em passos anteriores

| Passo | Dependência do sinal do TJ | Requer revisão? |
|---|---|---|
| P520/P521 | Introduziu a fórmula errada; teste `p520_emit_shaped_kerning_delta` esperava `-40` | **Já corrigido em P548** (teste alterado para `+40`, L0s actualizados) |
| P525 | Não toca no delta TJ | Não |
| P538e | Não toca no delta TJ | Não |
| P543 | Fallback global de fonte; não depende do sinal do TJ | Não |
| P544 | `FallbackFontMetrics` — o bug de medição sem kerning mascarava o sinal errado | Corrigido em P548 |
| P546 | Diagnóstico; não alterou o sinal | Não |

Não há código de produção a alterar neste passo. A fórmula de P548 é a correcta.

---

## 5. Ficheiros de verificação

- `/tmp/p549-av.typ`, `/tmp/p549-kern.typ`
- `/tmp/p549-cristalino.pdf`, `/tmp/p549-vanilla.pdf`
- `/tmp/p549-2.pdf`, `/tmp/p549-2-vanilla.pdf`
- `/tmp/p549-cristalino.png`, `/tmp/p549-vanilla.png`, `/tmp/p549-diff.png`

Estes ficheiros são temporários e não são commitados.
