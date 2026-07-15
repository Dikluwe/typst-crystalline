# Diagnóstico P763g — Checklist de sub-layouts e investigação do AE=2730 nativo `line`+`circle`

**Data da medição:** 2026-07-15T19:47:20-03:00  
**Commit base:** `d5791eb7a7d01e302e6f05ec334f5226653fedc9`  
**Working tree:** limpo (sem alterações de código)  
**Passo:** P763g  
**Objectivo:** Verificar `grid` e `columns` sob a correcção de P763f, e identificar a causa real do AE=2730 no documento nativo `#line` + `#circle`.

---

## Parte A — Checklist de sub-layouts

Todos os testes usam `mutool draw -r 300` antes de `compare -metric AE`.

### `grid` com `place` dentro de `block`+`align`

`/tmp/p763g-grid.typ`:

```typst
#set page(width: 8cm, height: 6cm)
#grid(
  columns: 2, gutter: 5pt,
  block(width: 3cm, height: 2cm, align(top, place(top+left, dx: 5pt, dy: 5pt, circle(radius: 10pt)))),
  block(width: 3cm, height: 2cm, align(top, place(top+left, dx: 5pt, dy: 5pt, circle(radius: 10pt)))),
)
```

| Variante | AE |
|----------|-----|
| grid com place | **4093** |
| grid sem place (`align(top, circle(...))`) | **2020** |

### `columns` com `place` dentro de `block`+`align`

`/tmp/p763g-columns.typ`:

```typst
#set page(width: 8cm, height: 6cm)
#columns(2)[
  #block(width: 3cm, height: 2cm, align(top, place(top+left, dx: 5pt, dy: 5pt, circle(radius: 10pt))))
  #colbreak()
  #block(width: 3cm, height: 2cm, align(top, place(top+left, dx: 5pt, dy: 5pt, circle(radius: 10pt))))
]
```

| Variante | AE |
|----------|-----|
| columns com place | **5418** |
| columns sem place | **5414** |

### Interpretação

A diferença introduzida pelo `place` dentro de `block`+`align` em `columns` é de apenas **4 pixels** (5418 vs 5414), irrelevante. Em `grid`, o `place` aumenta o AE de 2020 para 4093, mas a inspecção visual mostra que o `grid` cristalino já não renderiza a segunda célula (ou posiciona-a fora da página) mesmo sem `place`. A divergência principal é estrutural do próprio `grid`, não da correcção de `layout_place`.

**Conclusão:** a correcção de P763f não introduziu regressão específica de `place` em `grid`/`columns`. Os valores elevados são divergências pré-existentes desses sub-layouts.

---

## Parte B — Investigação do AE=2730 em `#line` + `#circle` nativo

### Medições isoladas

| Documento | AE |
|-----------|-----|
| `#line(start: (0pt,0pt), end:(56pt,28pt))` | **0** |
| `#circle(radius: 10pt)` | **402** |
| `#line(...)` + `#circle(...)` | **2730** |

### Coordenadas via `mutool trace`

Documento `/tmp/p763g-line-circle.typ`:

```typst
#set page(width: 8cm, height: 4cm)
#line(start: (0pt, 0pt), end: (56pt, 28pt))
#circle(radius: 10pt)
```

#### Vanilla

```text
stroke_path transform="1 0 0 1 13.498313 13.498314"
  moveto x="0" y="0"
  lineto x="56" y="28"

stroke_path transform="1 0 0 1 13.498313 54.69831"
  moveto x="0" y="10"
```

Coordenadas absolutas (sistema Y-down do layout):
- Linha: topo em `(13.50, 99.89)`, base em `(69.50, 71.89)`.
- Círculo: centro em `(13.50, 48.69)`.

#### Cristalino

```text
stroke_path transform="1 0 0 -1 0 113.38"
  moveto x="13.498" y="99.886"
  lineto x="69.498" y="71.886"

stroke_path transform="1 0 0 -1 0 113.38"
  moveto x="23.498" y="71.886"
```

Coordenadas absolutas (Y-down):
- Linha: topo em `(13.50, 13.49)`, base em `(69.50, 41.49)`.
- Círculo: centro em `(23.50, 41.49)`.

### Diferenças observadas

- **Horizontal:** círculo cristalino está deslocado **+10 pt** (raio) em relação ao vanilla.
- **Vertical:** círculo cristalino está deslocado **-7,2 pt** em relação ao centro vanilla; se considerarmos a posição relativa à base da linha, o cristalino coloca o círculo exactamente na base da linha (`y = 41,49`), enquanto o vanilla o coloca **23,2 pt abaixo** da base da linha.
- A linha em si coincide bit-exact (a parte o arredondamento a 0,01 pt).

### Causa identificada

A divergência não é um deslocamento próprio do `circle` isolado, nem um avanço de cursor simples. Reflete uma diferença de **modelo de layout**:

- No Typst vanilla, `line` e `circle` são elementos **inline**: participam no fluxo de texto, são posicionados relativamente à baseline, e o avanço vertical entre eles segue `top-edge + |bottom-edge| + leading`.
- No cristalino, `line` e `circle` são `ShapeKind::Line` / `ShapeKind::Ellipse` dentro de `ShapeElem`, tratados como **elementos de bloco**: posicionados em `cursor_x`/`cursor_y` e o cursor avança pela altura da própria forma (`cursor_y += resolved_h`), sem leading.

Isto explica simultaneamente:
1. Por que `line` isolado bate (não há interacção com outro elemento).
2. Por que `circle` isolado tem AE baixo (baseline) — não há avanço vertical para comparar.
3. Por que a combinação diverge: o vanilla aplica espaçamento de linha entre os dois elementos inline; o cristalino empilha-os como blocos.

### Tentativa de correcção e resultado

Testámos centrar `ShapeKind::Ellipse` horizontalmente em `cursor_x` (subtraindo `width/2` de `pos.x`). O resultado:

- `circle` isolado: AE 402 → **2713** (piorou).
- `line`+`circle`: AE 2730 → **2722** (melhoria marginal).
- `cetz` line+circle: AE 1022 → **1022** (inalterado).

A pioria no `circle` isolado confirma que a posição horizontal correcta depende do modelo de layout: quando `circle` é block-level, o canto esquerdo deve estar em `cursor_x`; quando é inline, o centro deve estar em `cursor_x`. Corrigir só um dos aspectos sem migrar o modelo todo não funciona.

### Decisão

Não foi implementada correcção no âmbito deste passo. O AE=2730 é uma consequência da diferença arquitetural **inline vs block-level** entre o vanilla e o cristalino para primitivas de desenho. Corrigir isso exigiria reescrever o layout de `Shape`/`Line`/`Circle` para produzir `InlineItem`s, o que está fora do escopo de P763g e da linha de trabalho `cetz`/download de pacotes.

---

## Validação

- `cargo test --workspace` — verde (4150 + 637 + 33 + 2 + 27 + 2 passed; 0 failed).
- `crystalline-lint .` — zero violações (apenas V7 esperado de `package_version_resolution.md`).

---

## Conclusão

- **Parte A:** `grid` e `columns` têm divergências estruturais próprias, mas a correcção de P763f não introduziu regressão específica de `place` dentro desses sub-layouts.
- **Parte B:** o AE=2730 de `#line` + `#circle` nativo foi identificado como diferença de modelo (inline vs block-level), não como um bug localizável de cursor ou bounding box. Não corrigido neste passo.
- A linha de trabalho `cetz`/download de pacotes pode ser fechada: o canvas do `cetz` foi corrigido em P763f, e o observável residual nativo é independente do pacote.
