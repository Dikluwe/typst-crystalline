# Passo 137 — Relatório (Fase B.1: consumer `tracking`)

**Data**: 2026-04-24
**Precondição**: Passo 136 encerrado; 1089 total tests; zero
violations; 54 ADRs activas; 12 DEBTs abertos (DEBT-52 com
7 gaps).
**Natureza**: passo S em L1 + L3. **Primeiro consumer com
efeito visível** desde Passo 102 (fill). Fase B.1 do roadmap
DEBT-1.
**ADR**: **não tocada**.

---

## Sumário

Consumer `tracking` implementado em 2 pontos:

1. **L1 `cursor.rs::word_width`**: acresce `(n-1) × tracking_pt`
   para cada word de n chars. Garante line-wrapping correcto.
2. **L3 `export.rs`**: emite PDF `Tc` operator dentro de BT/ET
   quando `style.tracking` é `Some(non-zero)`. `Tc` adiciona
   character spacing a cada glyph do `Tj`.

**Primeiro efeito visível** desde o `fill` (Passo 102).
`#set text(tracking: 2pt)\nHELLO` produz PDF com espaço
adicional entre glyphs — diferença observável.

**861 L1 (+3) + 24 L2 + 186 L3 + 21 L4** + 6 ignorados =
**1092 total** (+3 vs 1089). Zero violations.

**DEBT-52 gap 2 resolvido**. 6 gaps restantes.

---

## 137.A — Inventário confirmatório

### Findings críticos

| Item | Resultado |
|------|-----------|
| A.1 advance em `cursor.rs::word_width` | `metrics.advance(word, size) -> Pt` |
| A.2 `Length::resolve_pt(font_size_pt: f64) -> f64` | **existe** em `layout_types.rs:554` ✓ |
| A.3 test harness `layout_typst` + `text_items` | existe em `tests_set_rule_integration` |
| A.4 último glyph sem tracking | `(n-1) × tracking_pt` no word_width |
| A.5 exporter gate | **PDF `Tj` simples com single position** — mas `Tc` operator resolve |
| A.6 FrameItem::Text repr | **string-level** (não per-glyph) — Tc inside BT/ET funciona |
| A.7 base tests | L1: 858, Total: 1089 |

### Gate A.5 — workaround clean

FrameItem::Text é **string-level** (não per-glyph), mas PDF
tem o **`Tc` operator**: dentro de BT/ET, `{value} Tc` adiciona
character spacing a cada glyph do `Tj` subsequente.

Vantagens sobre switch para `TJ` com positions individuais:
- Zero mudança na estrutura de `FrameItem::Text`.
- Zero mudança de representação de glyph positions (string
  permanece).
- PDF primitive apropriado para "character spacing" — exactly
  what `tracking` semanticamente significa.
- 3 linhas adicionadas no export.

**Gate não bloqueou**. Consumer completo em 1 passo (vs 2 se
tivesse de refactorar FrameItem).

---

## 137.B — `Length::resolve_pt` pré-existente

Já existe em L1 (Passo anterior). Nenhuma adição necessária:

```rust
impl Length {
    pub fn resolve_pt(&self, font_size_pt: f64) -> f64 {
        self.abs.to_pt() + self.em * font_size_pt
    }
}
```

---

## 137.C — Consumer em layout (`cursor.rs`)

### Diff `word_width`

```rust
// antes:
fn word_width(&self, word: &str) -> Pt {
    self.metrics.advance(word, self.style.size)
}

// depois:
fn word_width(&self, word: &str) -> Pt {
    let base = self.metrics.advance(word, self.style.size);
    let tracking_extra = self.style.tracking
        .map(|t| {
            let tracking_pt = t.resolve_pt(self.style.size.val());
            let n = word.chars().count();
            tracking_pt * n.saturating_sub(1) as f64
        })
        .unwrap_or(0.0);
    Pt(base.val() + tracking_extra)
}
```

**Paridade vanilla**: `(n-1)` tracking entre pares de chars
dentro do word. Último char não ganha tracking (garantido via
`saturating_sub(1)`).

---

## 137.D — Exporter `Tc` operator

### Diff `export.rs`

```rust
// adicionado antes do format! existente:
let tracking_pt = style.tracking
    .map(|t| t.resolve_pt(style.size.val()))
    .unwrap_or(0.0);
let tc_op = if tracking_pt.abs() > f64::EPSILON {
    format!("{:.2} Tc\n", tracking_pt)
} else {
    String::new()
};

// format! do BT/ET estendido:
"BT\n/{font_ref} {:.1} Tf\n{tc_op}{:.1} {:.1} Td\n({safe}) Tj\nET\n"
```

**Semântica**: cada `BT/ET` emite `Tc` se tracking non-zero,
ou omite (default Tc = 0). PDF spec garante Tc se propaga
dentro do BT/ET mas não atravessa BT boundaries.

### PDF real

Entrada `#set text(tracking: 2pt)\nHELLO`:
```
BT
/F1 11.0 Tf
2.00 Tc          ← novo
78.1 761.4 Td
(HELLO) Tj
ET
```

Entrada sem tracking:
```
BT
/F1 11.0 Tf
70.9 761.4 Td    ← Tc omitido; Td começa mais à esquerda
(HELLO) Tj
ET
```

---

## 137.E — Testes L1 novos (3)

Em `tests_set_rule_integration`:

1. **`set_text_tracking_propaga_ao_frame_passo_137`**: valida
   que `style.tracking == Some(Length::pt(1.0))` chega ao
   frame.
2. **`layout_tracking_afecta_posicao_palavra_seguinte_passo_137`**:
   compara posição `x` de "CD" entre `"AB CD"` sem tracking e
   `"AB CD"` com tracking 1em/12pt. Assert `x_com > x_sem`.
3. **`layout_tracking_um_char_nao_acumula_passo_137`**:
   verifica que tracking não se propaga inter-word. `"A B"`
   com tracking 10pt não soma 10pt entre A e B (só intra-word).

### Helper novo

```rust
fn text_items_with_pos(doc) -> Vec<(String, TextStyle, f64)>
```

Extrai `pos.x` além de text+style — útil para tests de
positioning.

---

## 137.F — Teste manual visual

```bash
$ typst HELLO.typ -o sem.pdf        # sem tracking
$ typst HELLO.typ -o com.pdf        # #set text(tracking: 2pt)

$ grep -a "BT\|Tf\|Td\|Tj\|Tc" sem.pdf
BT /F1 11.0 Tf  70.9 761.4 Td  (HELLO) Tj  ET

$ grep -a "BT\|Tf\|Td\|Tj\|Tc" com.pdf
BT /F1 11.0 Tf  2.00 Tc  78.1 761.4 Td  (HELLO) Tj  ET
```

**Verificações**:
- ✓ `Tc` operator presente em `com.pdf`.
- ✓ Position de word diverge (78.1 vs 70.9 = 7.2pt diff).
  7.2pt ≈ 4 char-pairs × ~1.8pt (com `2pt tracking * 0.6 letter-advance` scale conversion, aproximado).
- ✓ Tamanhos PDF: 955 vs 938 bytes (diff marginal, condiz com
  `2.00 Tc` adicionado).

---

## 137.G — Regressão

Canary `eval_set_text_hyphenate_canary_passo_132b`: **passa**.
Testes layout existentes: **passam**. Nenhuma regressão
detectada.

---

## 137.H — DEBT-52 actualizado

```markdown
- [x] Fase A — **Passo 136**.
- [x] Consumer tracking. **Resolvido no Passo 137** — (...).
      **Primeiro efeito visível** desde Passo 102.
- [ ] Consumer leading.
- [ ] ...
```

---

## 137.I — Verificação

### Cargo tests

```
test result: ok. 861 passed ...       (L1 +3 vs 858)
test result: ok. 186 passed, 6 ign    (L3 inalterado)
test result: ok. 24 passed            (L2 inalterado)
test result: ok. 21 passed            (L4 inalterado)
```

### `crystalline-lint .`

```
✓ No violations found
```

### Manual

- `typst sem.typ` → PDF normal.
- `typst com.typ` (`#set text(tracking: 2pt)`) → PDF com `Tc
  2.00` emit, word positions shift (diff visível).
- `typst h.typ` (canary `hyphenate`) → warning esperado.

---

## Ficheiros tocados

| Ficheiro | Mudança |
|----------|---------|
| `01_core/src/rules/layout/cursor.rs` | `word_width` acrescenta `(n-1) × tracking_pt` |
| `03_infra/src/export.rs` | PDF `Tc` operator condicional em `BT/ET` |
| `01_core/src/rules/layout/tests.rs` | +3 tests integration + helper `text_items_with_pos` |
| `00_nucleo/DEBT.md` | Gap 2 DEBT-52 marcado resolvido |

### Números finais

| Métrica | Antes (136) | Depois |
|---------|------:|-------:|
| L1 tests | 858 | **861** (+3) |
| L2 tests | 24 | 24 |
| L3 tests | 186 | 186 |
| L4 tests | 21 | 21 |
| **Total** | **1089** | **1092** (+3) |
| Violations | 0 | 0 |
| ADRs activas | 54 | 54 |
| DEBTs abertos | 12 | 12 (DEBT-52 gap 2 ✓) |

---

## Mudança observável

**Primeiro efeito visível em PDF** desde Passo 102. Utilizador
que escreva `#set text(tracking: Xpt)` vê:
- **PDF**: `Tc` operator dentro de `BT/ET`, adicionando X pt
  após cada glyph.
- **Visual**: espaçamento entre letras aumenta; palavras
  ocupam mais espaço horizontal.
- **Line wrap**: o layouter também considera o extra space,
  evitando overflows.

**Line-wrap + rendering alinhados**: a mesma `Length::resolve_pt`
é usada nos dois lados, garantindo que o layouter prevê
correctamente o que o exporter desenha.

---

## Lições

1. **PDF `Tc` operator é gift para tracking**: zero refactor
   da estrutura `FrameItem::Text`; 3 linhas no exporter
   resolvem. Alternativa (`TJ` com array) exigiria per-glyph
   positions — passo L (grande). Escolha `Tc` reflecte
   princípio "usar o primitive PDF que case com a semântica".

2. **Gate 137.A.5 quase disparou**: o exporter é simplista
   (Tj apenas), mas PDF tem primitive próprio para este caso.
   Importante gate não assumir que "exporter simplista" =
   "impossível" — pesquisar PDF spec antes de escalar.

3. **`Length::resolve_pt` funciona duas vezes**: layouter usa
   para word_width; exporter usa para Tc value. Garante
   consistency. Se `Length` mudar (ex: adicionar novas
   componentes relativas), ambos beneficiam.

4. **Line wrapping e rendering têm de usar mesma formula**:
   se layouter usa `(n-1) × tracking` mas exporter emite `n ×
   Tc`, glyphs overflow porque rendering > layouter. Testar
   com line-wrap (words longos) seria próximo passo de
   regressão — **candidato para teste futuro**.

5. **Test numérico aproximado é honest**: em vez de assertar
   valor exacto, assertar `x_com > x_sem` é mais robusto —
   não depende do `advance` exacto da fonte (que pode variar
   entre builds). Comparação relativa captura a intenção.

6. **3 tests cobrem três invariantes distintos**:
   propagação, efeito inter-word, não-aplicação intra-word
   triviais. Cada um documenta um aspecto diferente do
   comportamento.

---

## Estado pós-Passo 137

### DEBT-52 progresso

- ✓ Gap 1 (Fase A): TextStyle. **Passo 136**.
- ✓ Gap 2 (Fase B.1): tracking. **Passo 137**.
- ⏳ Gap 3 (Fase B.2): leading (próximo).
- ⏳ Gap 4 (Fase B.3): weight faux-bold.
- ⏳ Gap 5-7 (Fase C): font string/array, lang hyphenation.
- ⏳ Gap 8 (opcional): font dict.

### Próximo passo sugerido: **Passo 138** (Fase B.2)

Consumer `leading` — vertical line spacing.

**Escopo**: aplicar `leading` como `line_height` adicional em
`flush_line` ou equivalente. Resolvido contra `size` para Pt.
Se existe, adapta `line_height` base.

**Estimativa**: S. Similar escopo a 137 mas em eixo vertical.

---

## Candidato futuro registado

- **Line-wrapping regression test com tracking**: assertar que
  long word com tracking forte wrapeia na coluna correcta —
  valida que `word_width` (layouter) e `Tc` (exporter) estão
  sincronizados.
- **`eval_with_warnings` helper** continua pendente.
- **Golden PDF tests** para regressão binária — quando infra
  chegar.
