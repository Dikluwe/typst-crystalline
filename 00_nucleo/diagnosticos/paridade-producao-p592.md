# Relatório Diagnóstico — Passo 592
## A diferença residual de ~12,71 pt em RTL era um espaço final mal contado

- **Commit de Referência:** `7e8e8963810b8eb0b1ebf747ebc73371f2a194c4` (Working Tree não commitado)
- **Data/Hora da Medição:** 2026-07-07 11:41:59 UTC
- **ADR Base:** `00_nucleo/adr/adr-paridade-defeitos-testes.md`, ADR-0108

---

## 1. Objetivo

P591 mediu uma diferença residual de ~12,71 pt no posicionamento horizontal do documento RTL de referência e classificou-a como "aceitável". 12,71 pt é quase exactamente a largura de um espaço em DejaVu Sans a 40 pt — a mesma coincidência numérica que, em P587, levou à descoberta de um espaço a mais no início do parágrafo. Este passo investiga se a causa é a mesma ou outra, antes de aceitar a diferença.

---

## 2. Sonda

### 2.1 Largura exacta do espaço em DejaVu Sans a 40 pt

```bash
python3 -c "
from fontTools.ttLib import TTFont
f = TTFont('/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf')
upem = f['head'].unitsPerEm
space_gid = f.getBestCmap()[ord(' ')]
space_width = f['hmtx'][space_gid][0]
print(f'upem={upem}, space_width_units={space_width}, at 40pt={space_width/upem*40:.4f}pt')
"
```

Resultado:

```text
upem=2048, space_width_units=651, at 40pt=12.7148pt
```

O valor 12,7148 pt coincide com a diferença residual medida em P591 (dentro de 0,001 pt).

### 2.2 O documento de 3 palavras sem número no meio

```typst
#set text(dir: rtl, lang: "ar", size: 40pt, font: "DejaVu Sans")
الكتاب على الطاولة
```

Cristalino **antes** da correção:

```text
left=187.26  width=121.28  text=ةلواطلا
left=321.23  width=70.44   text=ىلع
left=404.39  width=107.32  text=باتكلا
```

Vanilla:

```text
left=199.98  width=121.25  text=ةلواطلا
left=333.94  width=70.45   text=ىلع
left=417.10  width=107.30  text=باتكلا
```

A diferença de ~12,71 pt aparece em **todas** as palavras, mesmo sem o número "42" no meio. Confirma-se que o problema é sistemático, não específico do documento de P591.

### 2.3 Origem do espaço extra

Traçando a sequência de `Content` que chega ao layout:

```text
[P592 DEBUG layout_content] content=styled(sequence([space, text("الكتاب على الطاولة"), space]), ...)
[P592 DEBUG layout_content] content=sequence([space, text("..."), space])
[P592 DEBUG layout_content] content=space  cursor_x=70.87
[P592 DEBUG layout_space] SKIPPED current_line_len=0
[P592 DEBUG layout_content] content=text("الكتاب على الطاولة")  cursor_x=70.87
[P592 DEBUG layout_content] content=space  cursor_x=395.30359375
[P592 DEBUG layout_space] current_line_len=3 sw=12.71484375 cursor_x_after=408.0184375
```

A sequência de markup produzida por `eval_markup` (`01_core/src/engine/eval/mod.rs:494`) contém um `Content::Space` inicial e outro final. O inicial é correctamente ignorado por P588 (`current_line.is_empty()`). O **final** avança o cursor em `space_width()` sem emitir nenhum item visual.

Em parágrafos LTR esse avanço final é invisível (o conteúdo fica na mesma posição, com espaço em branco à direita). Em RTL, `align_current_line_rtl` usava `cursor_x` como limite direito da linha, pelo que o espaço final deslocava toda a linha 12,71 pt para a esquerda. `reorder_bidi_line` propagava o erro ao recalcular as posições com base no left edge do último item, não no seu right edge.

---

## 3. Implementação

### 3.1 `align_current_line_rtl` alinha pelo conteúdo real

Ficheiro: `01_core/src/engine/layout/cursor.rs:158`

Em vez de `cursor_x` (que pode incluir `Content::Space` final), calcula `content_right` como o right edge máximo dos `FrameItem` na linha e alinha por esse valor.

### 3.2 `item_width` helper para right edge real

Ficheiro: `01_core/src/engine/layout/helpers.rs:32`

Novo helper que devolve a largura horizontal de um `FrameItem`. Para texto usa `advance_shaped` quando disponível (paridade com `layout_word` e `text_width_for_bidi`), para não reintroduzir divergência em scripts contextuais.

### 3.3 `reorder_bidi_line` usa content_right no cálculo do gap

Ficheiro: `03_infra/src/layout_bidi.rs:126`

A fórmula anterior usava `x_max = left edge do último item visual`, o que deixava de fora a largura desse item. A nova fórmula usa `content_right = max(item_x + item_width)`, garantindo que o último item termina na margem direita e que o `gap` distribuído entre palavras é correcto.

---

## 4. Resultados

### 4.1 Documento de 3 palavras

Cristalino **após** a correção:

```text
left=199.98  width=121.28  text=ةلواطلا
left=333.94  width=70.44   text=ىلع
left=417.11  width=107.32  text=باتكلا
```

Vanilla:

```text
left=199.98  width=121.25  text=ةلواطلا
left=333.94  width=70.45   text=ىلع
left=417.10  width=107.30  text=باتكلا
```

Diferença máxima: 0,01 pt (arredondamento).

### 4.2 Documento de referência de P591

```typst
#set text(dir: rtl, lang: "ar", size: 40pt, font: "DejaVu Sans")
الكتاب 42 على الطاولة
```

Cristalino **após** a correção:

```text
left=136.36  width=121.28  text=ةلواطلا
left=270.33  width=70.44   text=ىلع
left=353.49  width=50.88   text=42
left=417.11  width=107.32  text=باتكلا
```

Vanilla:

```text
left=136.36  width=121.25  text=ةلواطلا
left=270.33  width=70.45   text=ىلع
left=353.49  width=50.90   text=42
left=417.10  width=107.30  text=باتكلا
```

Diferença máxima: 0,02 pt em larguras (shaping vs. vanilla) e 0,01 pt em posições.

---

## 5. Decisão

- A diferença residual de ~12,71 pt **era** causada por um `Content::Space` final (newline após o texto) a ser contado no alinhamento RTL.
- A causa foi corrigida, não registada como resíduo aceitável.
- A sequência RTL está agora alinhada com o vanilla nos dois documentos de referência.

---

## 6. Validação

```bash
cargo build --workspace --release
cargo test --workspace
crystalline-lint .
```

Resultados:

- `cargo build --workspace --release`: sucesso.
- `cargo test --workspace`: sucesso.
- `crystalline-lint .`: `✓ No violations found`.

---

## 7. Tabela de estado da sequência RTL

| Passo | Estado | Nota |
|---|---|---|
| P560–P588 | Fechado | Causas anteriores corrigidas (font_size_pt, espaço inicial). |
| P590 | Fechado | Largura shaped aplicada à decisão de quebra. |
| P591 | Fechado | Documento de referência cabe numa linha; diferença residual marcada para investigação. |
| **P592** | **Fechado** | Diferença residual de ~12,71 pt corrigida: espaço final em RTL. |
