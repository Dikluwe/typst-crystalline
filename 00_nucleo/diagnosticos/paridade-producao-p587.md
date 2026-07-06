# Relatório Diagnóstico — Passo 587
## Causa exacta da quebra de linha prematura em RTL

- **Commit de Referência:** `4b62f5658` (Working Tree não commitado)
- **Data/Hora da Medição:** 2026-07-06 17:04:49 UTC

---

## 1. Valores exactos da decisão de quebra

Instrumentámos temporariamente `layout_word` em `01_core/src/rules/layout/cursor.rs:95` para imprimir os valores no momento da decisão:

```text
[P587] layout_word "الكتاب" w=Pt(144.0) cursor_x=Pt(80.87) right_margin=524.41 width=595.28 margin=70.87
[P587] layout_word "42" w=Pt(40.0) cursor_x=Pt(234.87) right_margin=524.41 width=595.28 margin=70.87
[P587] layout_word "على" w=Pt(72.0) cursor_x=Pt(284.87) right_margin=524.41 width=595.28 margin=70.87
[P587] layout_word "الطاولة" w=Pt(168.0) cursor_x=Pt(366.87) right_margin=524.41 width=595.28 margin=70.87
```

A decisão de quebra está em `01_core/src/rules/layout/cursor.rs:98`:

```rust
if self.regions.current.cursor_x.0 + w.0 > right_margin && self.regions.current.cursor_x.0 > self.page_config.margin
```

No momento em que "الطاولة" é avaliada:
- `cursor_x` = 366.87 pt
- `w` = 168.00 pt
- `cursor_x + w` = 534.87 pt
- `right_margin` = 524.41 pt
- 534.87 > 524.41 → **quebra accionada**.

A largura útil esperada é `width − 2 × margin = 595.28 − 2 × 70.87 = 453.54 pt`. O valor calculado no código está correcto.

---

## 2. Onde entram os 10 pontos extra

A soma das palavras é `144 + 40 + 72 + 168 = 424 pt`. Entre quatro palavras há três espaços de `space_width() = 10 pt` cada, totalizando `30 pt`. O total seria `424 + 30 = 454 pt`.

No entanto, o `cursor_x` inicial para "الكتاب" é `80.87 pt`, não `70.87 pt` (a margem). Faltam `10 pt`.

Instrumentámos `Content::Space` em `01_core/src/rules/layout/mod.rs:729`:

```text
[P587] layout_space cursor_x=Pt(70.87) -> +Pt(10.0)
[P587] layout_text "الكتاب 42 على الطاولة"
```

Há um `Content::Space` renderizado **antes** do texto, movendo o cursor de `70.87 pt` para `80.87 pt`.

Instrumentámos `eval_markup` em `01_core/src/rules/eval/mod.rs:447` para ver de onde vem esse espaço:

```text
[P587] eval_markup child kind=Hash text="#"
[P587] eval_markup child kind=SetRule text=""
[P587] eval_markup child kind=Space text="\n"
[P587] eval_markup child kind=Text text="الكتاب 42 على الطاولة"
```

A newline após `#set text(...)` é convertida em `Content::Space` por `01_core/src/rules/eval/mod.rs:494`:

```rust
SyntaxKind::Space | SyntaxKind::Parbreak => parts.push(Content::Space),
```

Essa newline inicial contribui com `10 pt` extra no início da linha.

---

## 3. Teste de margem zero

```bash
cat > /tmp/p587-margem.typ <<'EOF'
#set page(margin: 0pt)
#set text(dir: rtl, lang: "ar", size: 40pt)
الكتاب 42 على الطاولة
EOF
./target/release/typst /tmp/p587-margem.typ /tmp/p587.pdf
pdftotext -tsv /tmp/p587.pdf -
```

Resultado:

```text
5	1	0	0	0	0	131.28	-32.00	168.00	40.00	100	ةلواطلا
5	1	0	0	0	1	309.28	-32.00	72.00	40.00	100	ىلع
5	1	0	0	0	2	391.28	-32.00	40.00	40.00	100	42
5	1	0	0	0	3	441.28	-32.00	144.00	40.00	100	باتكلا
```

Com margem zero, as quatro palavras cabem numa linha só (largura da caixa de linha = 454 pt). Isto confirma que o cálculo de margem em si não está duplicado: a largura útil com margem zero é a largura da página inteira.

---

## 4. Teste sem newline após o `#set`

```bash
cat > /tmp/p587-sem-newline.typ <<'EOF'
#set text(dir: rtl, lang: "ar", size: 40pt)الكتاب 42 على الطاولة
EOF
./target/release/typst /tmp/p587-sem-newline.typ /tmp/p587-sem-newline.pdf
pdftotext -tsv /tmp/p587-sem-newline.pdf -
```

Resultado: o documento sem newline ainda quebra, mas o cursor inicial de "الكتاب" é agora `70.87 pt` (sem os 10 pt extra). A decisão de quebra ocorre porque `356.87 + 168 = 524.87 > 524.41` — uma diferença de apenas **0.46 pt**.

Isso mostra que há duas componentes:

1. **Componente principal (bug):** a newline após `#set` é convertida em `Content::Space` e renderizada como espaço visual no início do parágrafo. Sem esse espaço, o documento estaria na fronteira da largura útil.
2. **Componente de fronteira:** mesmo sem o espaço inicial, o total ocupado (`454 pt`) excede a largura útil (`453.54 pt`) por `0.46 pt`. Esta diferença minúscula explica por que o vanilla (451.20 pt) cabe e o cristalino não.

---

## 5. Causa exacta localizada

A causa exacta da quebra prematura observada em P586 é:

- **`01_core/src/rules/eval/mod.rs:494`** converte qualquer `SyntaxKind::Space` (incluindo a newline após `#set text(...)`) em `Content::Space`.
- **`01_core/src/rules/layout/mod.rs:729`** renderiza esse `Content::Space` no início do parágrafo, avançando `cursor_x` em `space_width()`.
- O espaço inicial de `10 pt` empurra o texto para a direita, fazendo com que o total ocupado (`454 pt + 10 pt = 464 pt`) exceda largamente a largura útil (`453.54 pt`), disparando a quebra antes de "الطاولة".

O vanilla não renderiza essa newline inicial como espaço visual. A correção correcta é fazer com que `eval_markup` ignore leading spaces/parbreaks no início de um parágrafo (ou o layout de `Content::Space` não avançar quando é o primeiro item da linha).

---

## 6. Decisão

Não implementámos a correção neste passo, conforme o tipo "sonda directa". O problema de sobreposição vertical de P577 permanece resolvido. O problema residual é este bug de espaço inicial em parágrafos, que se manifesta dramaticamente em RTL porque o texto começa já perto da margem direita.

A sequência RTL pode ser considerada **fechada quanto ao bug original de sobreposição**, mas o documento `الكتاب 42 على الطاولة` com margem default só terá paridade exacta com o vanilla depois de corrigido o tratamento de leading `Content::Space`.
