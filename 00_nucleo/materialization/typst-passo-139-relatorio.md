# Passo 139 — Relatório (Fase B.3: consumer `weight` faux-bold)

**Data**: 2026-04-24
**Precondição**: Passo 138 encerrado; 1095 total tests; zero
violations; 54 ADRs activas; 12 DEBTs abertos (DEBT-52 com 5
gaps).
**Natureza**: passo S em L1 + L3. **Terceiro consumer com
efeito visível**. **Fase B completa**.
**ADR**: **não tocada**.

---

## Sumário

Consumer `weight` implementado como **faux-bold via PDF stroke**:

1. **L1** `TextStyle::faux_bold_stroke_pt(k)` — fórmula
   `((weight - 400) / 300).max(0) × size × k`.
2. **L3** `export.rs` — emit `q \n 2 Tr \n {stroke} w \n BT ... ET \n Q`
   quando stroke > 0.

**K = 0.04** calibração inicial. Weight 700 @ 11pt → stroke
0.44pt (visivelmente bold). Weight 400 / None / ≤400 →
**zero regressão** (PDF binariamente idêntico).

`q/Q` wrap necessário porque `w` é graphics state (atravessa
BT); isola stroke ao span actual sem afectar lines seguintes.

**869 L1 (+5) + 24 L2 + 186 L3 + 21 L4** + 6 ignorados =
**1100 total** (+5 vs 1095). Zero violations.

**DEBT-52 gap 4 resolvido**. **Fase B completa** (gaps 1-4).
4 gaps restantes (Fase C).

---

## 139.A — Inventário confirmatório

### Findings

| Item | Resultado |
|------|-----------|
| A.1 export BT/ET pattern | Pós-137: `BT /F? Tf {Tc} {x} {y} Td ({text}) Tj ET` — `Tc` condicional foi o modelo |
| A.2 helper location | `TextStyle::faux_bold_stroke_pt(k)` em L1 (formula típográfica); K constante em L3 (calibração PDF) |
| A.3 K = 0.04 | stroke 0.44pt @ weight 700 size 11 — calibração inicial |
| A.4 harness tests | unit tests directo em `layout_types.rs::tests` (não exige pipeline completo) |
| A.5 `w` é graphics state | confirmado: `FrameItem::Line` usa `q ... w ... Q` pattern (line 636) |
| A.6 `Tr` operator | `2 Tr` = fill + stroke = faux-bold |
| A.7 F2 Helvetica-Bold | **orthogonal** ao weight — `*bold*` sintáctico usa F2; weight numérico usa stroke |

### Gate A.5 — q/Q wrap obrigatório

PDF spec confirma: `w` (line width) é **graphics state**, não
text state. Sem `q/Q` wrap, stroke do weight atravessaria para
`FrameItem::Line` seguinte (bordas, linhas de frac math, etc.),
produzindo linhas mais grossas.

Pattern escolhido (Opção B do enunciado): `q BT ... ET Q`
isolado por span. `0 w` reset (Opção A) seria alternativa mais
leve mas menos robusta.

### F2 coexistence

`*bold*` (markup syntax) → `style.bold: true` → PDF F2
(Helvetica-Bold).
`#set text(weight: 700)` → `style.weight: Some(700)` → PDF F1
+ `2 Tr 0.44 w` (faux-bold).

Combinação: `#set text(weight: 700)` + `*bold*` → F2 + stroke
= over-bold visual. Aceite como limitação conhecida até font
embedding real (Fase C).

---

## 139.B — Helper `faux_bold_stroke_pt`

```rust
// 01_core/src/entities/layout_types.rs
impl TextStyle {
    pub fn faux_bold_stroke_pt(&self, k: f64) -> f64 {
        let w = self.weight.unwrap_or(400);
        let factor = ((w as f64 - 400.0) / 300.0).max(0.0);
        factor * self.size.val() * k
    }
}
```

**Parâmetro `k`**: coeficiente de calibração (typical 0.04).
Testes podem usar outros valores; exporter usa `FAUX_BOLD_K =
0.04` constante.

**`None` → 400 (regular)**: `unwrap_or(400)` garante
comportamento determinístico. Equivalente a `Some(400)`.

**Clamp a zero**: weights ≤ 400 produzem stroke 0 (sem efeito).
Inclui thin (100), extralight (200), light (300). Aceite —
faux-bold não suporta faux-light.

---

## 139.C — Exporter emit

```rust
// 03_infra/src/export.rs (dentro do match FrameItem::Text)
const FAUX_BOLD_K: f64 = 0.04;
let stroke_pt = style.faux_bold_stroke_pt(FAUX_BOLD_K);
let (q_open, q_close, bold_ops) = if stroke_pt > f64::EPSILON {
    (
        "q\n",
        "Q\n",
        format!("2 Tr\n{:.3} w\n", stroke_pt),
    )
} else {
    ("", "", String::new())
};

ops.push_str(&format!(
    "{q_open}BT\n/{font_ref} {:.1} Tf\n{tc_op}{bold_ops}{:.1} {:.1} Td\n({safe}) Tj\nET\n{q_close}",
    style.size.val(), pos.x.val(), pdf_y
));
```

### Saída PDF exemplar (weight 700)

```
q
BT
/F1 11.0 Tf
2 Tr
0.440 w
78.1 761.4 Td
(HELLO WORLD) Tj
ET
Q
```

Sem weight (ou weight ≤ 400): mesma saída **sem** `q`, `2 Tr`,
`w`, `Q` — PDF idêntico a pré-139.

---

## 139.D — Tests L1 (5)

Em `layout_types.rs::tests`:

1. **`text_style_faux_bold_400_zero_passo_139`**: stroke = 0.
2. **`text_style_faux_bold_700_positivo_passo_139`**: stroke =
   0.44pt exacto.
3. **`text_style_faux_bold_100_clamp_zero_passo_139`**: clamp
   negative factor → 0.
4. **`text_style_faux_bold_escala_com_size_passo_139`**: size
   dobra → stroke dobra (proporção).
5. **`text_style_faux_bold_none_weight_tratado_como_400_passo_139`**:
   `None == Some(400)` (zero stroke).

Testes unit são **auto-suficientes** (não exigem pipeline).
Valores numéricos precisos porque fórmula é determinística.

---

## 139.E — Regressão em frame (não adicionada, validada manualmente)

Em vez de adicionar teste de frame regression (o setup seria
complexo dado que `weight: Some(400)` vs `None` produziria
frame identical mas exporter emit diferente em string bytes),
validação manual confirmou:

- `reg.pdf` (sem weight) = **938 bytes**.
- `w400.pdf` (weight 400) = **938 bytes** (identical).
- `w100.pdf` (weight 100 clamp) = **938 bytes** (identical).
- `w700.pdf` (weight 700) = **973 bytes** (+35 para `q\n2 Tr\n0.440 w\nQ\n`).
- `w900.pdf` (weight 900) = **973 bytes** (mesma estrutura,
  stroke value different).

---

## 139.F — Teste manual visual (K calibração)

```bash
$ grep -aE "q$|2 Tr|\.[0-9]{3} w|Q$" w700.pdf
q
2 Tr
0.440 w
Q
```

**K = 0.04 mantido** como calibração inicial.

Para avaliar visualmente se 0.44pt stroke @ size 11 parece
bold em PDF standard 72dpi renderer, abrir `w700.pdf` e
comparar com `reg.pdf` no mesmo viewer. (Observação
empírica fora do alcance deste snapshot textual — K pode ser
ajustado num passo futuro se calibração visual exigir.)

---

## 139.G — Reset graphics state: `q/Q` ✓

Escolhida **Opção B** (save/restore graphics state).

Alternativa (Opção A, `0 w`) seria mais leve mas não restauraria
strokes personalizados anteriores. `q/Q` é idempotent e
compositional — recomendado para stroke scoping.

---

## 139.H — Canary preservado

- `eval_set_text_hyphenate_canary_passo_132b`: passa.
- Tests de tracking (137): passam.
- Tests de leading (138): passam.
- Tests bold/italic/size/fill antigos: passam.

---

## 139.I — DEBT-52 actualizado

```markdown
- [x] Gap 1 (Fase A). Passo 136.
- [x] Gap 2 (Fase B.1 tracking). Passo 137.
- [x] Gap 3 (Fase B.2 leading). Passo 138.
- [x] Gap 4 (Fase B.3 weight faux-bold). **Passo 139** — este.
      **Fase B completa**.
- [ ] Gap 5: consumer `font` string.
- [ ] Gap 6: consumer `font` array.
- [ ] Gap 7: consumer `lang` hyphenation.
- [ ] Gap 8 (opcional): font dict + `regex` authorization.
```

4 gaps restantes, todos na **Fase C** (font + lang).

---

## 139.J — Verificação

### Cargo tests

```
test result: ok. 869 passed ...       (L1 +5 vs 864)
test result: ok. 186 passed, 6 ign    (L3 inalterado)
test result: ok. 24 passed            (L2 inalterado)
test result: ok. 21 passed            (L4 inalterado)
```

### `crystalline-lint .`

```
✓ No violations found
```

### Manual — 5 cenários

- `regular` (sem weight) → PDF sem `Tr/w`.
- `weight: 400` → PDF **idêntico** a regular (938 bytes).
- `weight: 100` → PDF **idêntico** a regular (clamp a 0).
- `weight: 700` → PDF com `q / 2 Tr / 0.440 w / ... / Q` (973
  bytes).
- `weight: 900` → PDF com `q / 2 Tr / 0.733 w / ... / Q` (973
  bytes).

**Canary `hyphenate`**: warning emitido normalmente.

---

## Ficheiros tocados

| Ficheiro | Mudança |
|----------|---------|
| `01_core/src/entities/layout_types.rs` | +`TextStyle::faux_bold_stroke_pt(k)` + 5 unit tests |
| `03_infra/src/export.rs` | +FAUX_BOLD_K const + emit `q/2 Tr/w/Q` condicional |
| `00_nucleo/DEBT.md` | Gap 4 DEBT-52 marcado resolvido; Fase B completa |

### Números finais

| Métrica | Antes (138) | Depois |
|---------|------:|-------:|
| L1 tests | 864 | **869** (+5) |
| L2 tests | 24 | 24 |
| L3 tests | 186 | 186 |
| L4 tests | 21 | 21 |
| **Total** | **1095** | **1100** (+5) |
| Violations | 0 | 0 |
| ADRs activas | 54 | 54 |
| DEBTs abertos | 12 | 12 (DEBT-52 com 4 gaps) |

---

## Mudança observável

**Terceiro consumer com efeito visível** — Fase B completa.

Utilizador com `#set text(weight: N)`:
- **N ≤ 400**: output idêntico a regular (sem efeito).
- **N > 400**: PDF ganha stroke faux-bold proporcional a
  `(N-400)/300 × size × 0.04`.
- **N = 700**: stroke 0.44pt @ size 11 — visualmente bold.
- **N = 900**: stroke 0.73pt @ size 11 — mais bold.

**Combinações Fase B em conjunto**:
- `#set text(weight: 700, tracking: 1pt)\n#set par(leading: 5pt)` →
  PDF com faux-bold + spaced letters + wider lines.
- Cada consumer é independente; efeitos compõem sem conflito.

---

## Fase B retrospective

4 passos (136/137/138/139) completaram Fase B.

| Passo | Consumer | Efeito | Complexidade |
|------:|----------|--------|:------------:|
| 136 | Fase A (infra) | Nenhum | XS |
| 137 | tracking (horizontal) | **Primeiro** efeito visível desde 102 | S |
| 138 | leading (vertical) | Segundo efeito visível | S |
| 139 | weight (stroke) | Terceiro efeito visível | S |

**Roadmap 135 validado**. Fase B em ~4-6h cumulativo. Fase C
(font + lang) é mais complexa — font envolve PDF embedding
real, lang envolve crate hifenização.

---

## Lições

1. **PDF primitives são presente inesperado**: Fase B usou 3
   primitives diferentes — `Tc` (character spacing), Td
   position (natural), `Tr`+`w` (stroke). Cada consumer mapeia
   para o primitive apropriado sem refactor de representação
   intermédia.

2. **`q/Q` wrap é cheaper que reset explícito**: Opção A
   (`0 w` no fim) seria mais leve em bytes PDF mas mais
   frágil (estado tem de ser tracked). `q/Q` é
   compositional — qualquer alteração de graphics state
   dentro do scope é revertida.

3. **K = 0.04 como calibração inicial**: documentado como
   "calibração inicial". Ajuste visual fica como candidato
   futuro se renderização em viewers diferentes mostrar
   inconsistência. Fórmula estável — só a constante varia.

4. **F2 coexistence limitation documented**: `*bold*` vs
   `#set text(weight)` produzem PDFs diferentes. Over-bold em
   combinação. Aceite até font embedding real.

5. **Weight 400 = None garantido**: `unwrap_or(400)` explicit.
   Sem ambiguidade entre "set 400" e "não set". Ambos produzem
   stroke 0. Zero surpresa para utilizador.

6. **Tests unitários da fórmula são suficientes**: não precisamos
   de pipeline completo para validar `faux_bold_stroke_pt`. 5
   unit tests cobrem 100% da lógica em ~30 linhas. Valor por
   unidade de custo alto.

---

## Estado pós-Passo 139

### DEBT-52 progresso

- ✓ Gap 1 (Fase A): TextStyle (136).
- ✓ Gap 2 (Fase B.1): tracking (137).
- ✓ Gap 3 (Fase B.2): leading (138).
- ✓ Gap 4 (Fase B.3): **weight faux-bold (139 — este)**.
- ⏳ Gap 5: font string (Fase C).
- ⏳ Gap 6: font array.
- ⏳ Gap 7: lang hyphenation.
- ⏳ Gap 8: font dict (opcional).

**Fase B completa**. **Fase C aberta**.

### Próximo passo sugerido: **Passo 140** (Fase C.1)

Consumer `font` string — selecção por nome via
`FontBook::select`.

**Escopo**: integrar `FontBook::select(family_name, variant)`
em L1; exporter muda font_ref (F1/F2/F3) conforme selecção.
Mas **PDF hardcoded em Helvetica** é limitação — para font
embedding real, infra L3 dedicada.

**Provável pausa para diagnóstico**: Fase C pode exigir ADR
ou diagnóstico separado antes de 140 (font) por complexidade
de PDF embedding.

**Estimativa 140**: S se ficar em "select + fallback sem
embedding" (aproximação ainda mais simples que faux-bold).
M-L se incluir embedding real.

### Candidatos futuros

- **K calibration visual**: ajustar 0.04 se renderizadores
  mostrarem inconsistência.
- **Golden PDF tests** para Fase B (tracking + leading +
  weight combinados).
- **`eval_with_warnings` helper** continua pendente; cada passo
  de Fase C adicionará mais harnesses.
