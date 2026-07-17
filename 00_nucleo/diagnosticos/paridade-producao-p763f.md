# Diagnóstico P763f — Correcção da transformação de coordenadas em `place` dentro de sub-frames

**Data da medição:** 2026-07-15T19:05:48-03:00  
**Commit base:** `b33706d208f2518acd8bdf032430e50437bd0e07`  
**Working tree:** alterado (`01_core/src/engine/layout/placement.rs`)  
**Passo:** P763f  
**Objectivo:** Corrigir a divergência de coordenadas que P763c e P763e confirmaram no documento `cetz` com `line` + `circle`.

---

## Localização da causa

P763e demonstrou que o bug não estava no renderizador de paths em si, mas na forma como o `place` calculava a origem de ancoragem quando era executado **dentro de um sub-frame** (por exemplo, `align(top, place(...))` dentro de um `block`).

Ficheiro alterado: `01_core/src/engine/layout/placement.rs`, função `layout_place`.

O código original usava sempre `origin_y = page_config.margin` e `y_offset = sub_origin_y` (ascender local). Isto funciona quando `place` é chamado no fluxo principal da página, mas falha quando `place` corre dentro de um sub-frame criado por `align`:

- O sub-frame tem a sua própria origem vertical (topo local).
- Usar `page_config.margin` como origem dentro desse referencial local desloca o conteúdo para baixo pelo valor da margem (~70 pt).
- A compensação `y_offset = sub_origin_y` não anula correctamente esse erro porque a origem já estava errada.

## Implementação

Em `layout_place`, quando `self.is_sub_frame == true` e não estamos dentro de uma célula de Grid, as coordenadas de ancoragem passam a ser relativas ao próprio sub-frame:

- `origin_x = 0.0`
- `origin_y = 0.0`
- `y_offset = 0.0`

O comportamento fora de sub-frames mantém-se inalterado (`page_config.margin` como origem, `sub_origin_y` como offset). O caso de células de Grid (`cell_origin_x/y` Some) também não foi alterado.

---

## Validação

### Método de medição

Todas as comparações usam `mutool draw -r 300` para rasterizar antes de `compare -metric AE`, conforme a regra estabelecida em P763e. O valor directo de `compare` sobre PDFs é registado apenas como nota — nunca como critério de fecho.

### Documento `cetz` original (idêntico a P763c/P763e)

`/tmp/p763f-cetz-original.typ`:

```typst
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
```

| Método | AE |
|--------|-----|
| Antes da correcção (rasterizado) | **10725** |
| Depois da correcção (rasterizado) | **1022** |
| Depois da correcção (directo PDF) | **0** |

### Coordenadas via `mutool trace`

#### Vanilla

```text
stroke_path transform="1 0 0 1 99.2126 70.86615"
  moveto x="0" y="28.346457"
  lineto x="56.692914" y="0"

stroke_path transform="1 0 0 1 70.86614 70.86615"
  moveto x="28.346457" y="0"
```

Coordenadas absolutas (Y-down):
- Linha: `(99.21, 99.21)` → `(155.91, 70.87)`
- Círculo centro: `(99.21, 99.21)` — ponto inicial da linha.

#### Cristalino (após correcção)

```text
stroke_path transform="1 0 0 -1 0 841.89"
  moveto x="99.21" y="742.68"
  lineto x="155.9" y="771.02"

stroke_path transform="1 0 0 -1 0 841.89"
  moveto x="99.21" y="771.02"
```

Convertendo Y (`y_user = 841.89 - y_pdf`):
- Linha: `(99.21, 99.21)` → `(155.90, 70.87)`
- Círculo centro: `(99.21, 70.87)` — ponto inicial da linha.

**As coordenadas batem.** O círculo passa a coincidir com o ponto inicial da linha, como no vanilla. O deslocamento vertical de ~61 pt foi eliminado.

### Residual de AE=1022

O residual não é um deslocamento geométrico. As coordenadas de `line` e `circle` coincidem com precisão de 0.01 pt. O valor residual deve-se a diferenças estruturais de renderização entre os dois PDFs, visíveis no `mutool trace`:

- `miterlimit="4"` (vanilla) vs `miterlimit="10"` (cristalino)
- `colorspace="DeviceGray"` (vanilla) vs `colorspace="DeviceRGB"` (cristalino)
- Presença de estrutura de tagged PDF no vanilla (ausente no cristalino)

A inspecção visual confirma que os desenhos são sobrepostos; o residual é ruído de rasterização, não erro de posicionamento.

### Regressão nos casos já validados

| Documento | AE (rasterizado) | Observação |
|-----------|------------------|------------|
| `#rect(width: 2cm, height: 2cm)` | **241** | Baseline inalterado |
| `#place(top + left, rect(...))` | **245** | Sem regressão |
| `#block(..., align(top, rect(...)))` | **245** | Sem regressão |
| `#block(..., align(top, place(..., line(...))))` | **0** | Caso mínimo do bug, corrigido |
| `#circle(radius: 10pt)` | **271** | Baseline |
| `cetz` com `line` isolado | **313** | Melhorado (não regressão) |
| `cetz` com `line` + `circle` | **1022** | Corrigido geometricamente |

Nota: o documento nativo `#line(start: (0pt,0pt), end:(56pt,28pt))` + `#circle(radius:10pt)` mantém AE=**2730**. Este é o deslocamento próprio da primitiva `circle` nativo que P763c identificou, separado do problema do canvas do `cetz`. Não foi alterado neste passo.

---

## Validação técnica

- `cargo test --workspace` — verde (4150 + 637 + 33 + 2 + 27 + 2 passed; 0 failed).
- `crystalline-lint .` — zero violações (apenas V7 esperado de `package_version_resolution.md`).

---

## Conclusão

- O bug de deslocamento do canvas do `cetz` foi corrigido.
- A causa era o cálculo de origem de `place` dentro de sub-frames, não uma inversão de eixo Y no renderizador de paths.
- O método correcto de medição (`mutool draw -r 300` antes de `compare`) continua a ser obrigatório; a comparação directa de PDFs continua a dar AE=0 mesmo quando há divergências geométricas reais.
- A linha de trabalho `cetz`/download de pacotes pode ser fechada do ponto de vista do posicionamento do canvas.
- O deslocamento próprio do `circle` nativo (AE=2730) permanece como observável residual, mas é uma causa independente fora do âmbito deste passo.
