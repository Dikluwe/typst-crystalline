# Passo 138 — Relatório (Fase B.2: consumer `leading`)

**Data**: 2026-04-24
**Precondição**: Passo 137 encerrado; 1092 total tests; zero
violations; 54 ADRs activas; 12 DEBTs abertos (DEBT-52 com 6
gaps).
**Natureza**: passo S em L1. **Segundo consumer com efeito
visível** da série. Eixo vertical. Exporter inalterado.
**ADR**: **não tocada**.

---

## Sumário

Consumer `leading` implementado em **`flush_line`** (cursor.rs):
baseline inter-linha soma `leading_pt` ao `line_height` default.

**Opção soma** escolhida sobre "substituir": `line_height =
default + user_leading`. Preserva regressão (leading 0 = sem
set) mas diverge vanilla subtil (que faz `size_core +
user_leading` substituindo o default).

**Obstáculo descoberto empiricamente**: `self.style` em
`flush_line` pode já ter sido restaurado (`self.style =
prev_style` após Content::Text). Solução: **peek no último
FrameItem::Text em `current_line`** em vez de `self.style`.

**864 L1 (+3) + 24 L2 + 186 L3 + 21 L4** + 6 ignorados =
**1095 total** (+3 vs 1092). Zero violations.

**DEBT-52 gap 3 resolvido**. 5 gaps restantes.

---

## 138.A — Inventário confirmatório

### Findings

| Item | Resultado |
|------|-----------|
| A.1 vanilla | `pub leading: Length` em `ParElem` (model/par.rs:210). Default 0.65em. |
| A.2 cristalino line_height | `metrics.vertical_metrics(size) -> (ascender, line_height)`; `line_height = size * 1.2` (metrics.rs:88) |
| A.3 helper `Length::resolve_pt` | existe (reaproveitar de 137) |
| A.4 pipeline | `flush_line` em cursor.rs incrementa `cursor_y += line_height` quando `current_line` não-vazio |
| A.5 harness teste | `text_items_with_pos` retorna só x; criar `text_items_with_xy` |
| A.6 casos edge | primeira linha não aplica; documento 1-linha irrelevante; leading 0 = regressão |
| A.7 base tests | L1: 861, Total: 1092 |

### Descoberta fundamental: **cristalino não tem `Content::Parbreak`**

Text flows sem break implícito. `flush_line` só dispara por:
- Right-margin wrap (word_width exceeds space).
- Heading (`=` no início de linha).
- Raw block, List, Link, Shape/Image (linha 287, 333, 351, 433...).

**Implicação nos tests**: `"linha1\n\nlinha2"` NÃO produz duas
linhas distintas em cristalino. Teste precisa de heading ou
wrap explícito.

---

## 138.B — Fórmula escolhida: **opt soma**

```
line_height_final = metrics.vertical_metrics(size).1 + leading_pt
```

- `leading_pt` = `leading.resolve_pt(font_size_pt)`.
- `leading = None` → `line_height_final = default` (regressão).
- `leading = 0pt` → `line_height_final = default` (regressão).
- `leading = 20pt` → `line_height_final = default + 20pt`.

### Divergência vanilla

Vanilla `line_height = size_core + user_leading` (substitui).
Cristalino soma ao default `size × 1.2`. Para `size=11pt,
leading=20pt`:
- Cristalino: `11 × 1.2 + 20 = 13.2 + 20 = 33.2pt`.
- Vanilla (aproximado): `11 + 20 = 31pt`.

Diferença de ~2pt (0.2 × size) — observacional **direcção
correcta**, não pixel-perfect. Documentado em ADR-0054 perfil
"observacional graded".

---

## 138.C — Consumer em `flush_line`

### Problema descoberto empiricamente

Primeira tentativa: `self.style.leading.map(...)`. Falhou
porque `self.style` já foi restaurado ao outer scope em
Content::Text (`self.style = prev_style`). `flush_line`
dispara depois (em Heading), quando self.style tem leading=None.

### Solução: peek no current_line

```rust
let line_leading_pt = self.current_line
    .iter()
    .rev()
    .find_map(|item| match item {
        FrameItem::Text { style, .. } =>
            style.leading.map(|l| l.resolve_pt(self.font_size_pt.val())),
        _ => None,
    })
    .unwrap_or(0.0);

// ...
if had_items {
    let (_, line_height) = self.metrics.vertical_metrics(self.font_size_pt);
    self.cursor_y += line_height + Pt(line_leading_pt);
}
```

Peek no último Text item da linha a flush — o seu
`style.leading` é o valor efectivo. Não há dependência de
`self.style` — o estado está na `current_line` directamente.

---

## 138.D — Tests L1 novos (3)

Helper novo `text_items_with_xy` que retorna `(text, style, x,
y)`. Complementa o `text_items_with_pos` do 137 (só x).

### Testes

1. **`layout_leading_afecta_posicao_linha_seguinte_passo_138`**:
   input `"= Título\nlinha2"` com e sem `leading: 20pt`.
   Asserta `y_com > y_sem` para "linha2".
2. **`layout_leading_nao_afecta_documento_uma_linha_passo_138`**:
   1 linha — leading sem efeito. Primeiro item y idêntico.
3. **`layout_leading_zero_preserva_comportamento_base_passo_138`**:
   leading 0pt == sem set. Todos os items y iguais.

### Descobertas no teste 1

Tentativa inicial com `"linha1\n\nlinha2"` falhou: sem
`Parbreak` em cristalino, texto flui sem break. Substituído
por `"= Título\nlinha2"` que força `flush_line` via heading.

---

## 138.E — Validação manual

### Output PDF

```bash
$ typst sem.typ (heading + 3 linhas)
Td positions: 70.9 761.4  /  78.1 747.0  / 131.5 747.0  / 184.9 747.0

$ typst com.typ (#set par(leading: 20pt) + mesmo conteúdo)
Td positions: 70.9 761.4  /  78.1 727.0  / 131.5 727.0  / 184.9 727.0
```

**Diferença exacta de 20pt** no y após heading:
- `sem.pdf`: y = 747.0 (Δ da heading = 14.4pt = default line_height
  para size 11 × 1.2 × escala de heading).
- `com.pdf`: y = 727.0 (Δ da heading = 34.4pt = default + 20pt
  de leading).

**Paridade numérica exacta com fórmula**: consumer funciona
precisamente.

PDF sizes idênticos (1030 bytes) — Td coordinates mudaram mas
número de operators igual.

---

## 138.F — Canary e regressão

- Canary `eval_set_text_hyphenate_canary_passo_132b`: **passa**.
- Todos os tests existentes (861 pré-138): **passam**.
- Tracking (137) continua funcional: testes de 137 passam.

---

## 138.G — DEBT-52 actualizado

```markdown
- [x] Gap 1 (Fase A). Passo 136.
- [x] Gap 2 (Fase B.1 tracking). Passo 137.
- [x] Gap 3 (Fase B.2 leading). **Passo 138** — este.
- [ ] Gap 4 (Fase B.3): weight faux-bold.
- [ ] Gap 5-7 (Fase C).
- [ ] Gap 8 (opcional).
```

5 gaps restantes.

---

## 138.H — Verificação

### Cargo tests

```
test result: ok. 864 passed ...       (L1 +3 vs 861)
test result: ok. 186 passed, 6 ign    (L3 inalterado)
test result: ok. 24 passed            (L2 inalterado)
test result: ok. 21 passed            (L4 inalterado)
```

### `crystalline-lint .`

```
✓ No violations found
```

### Manual

- `typst sem.typ` → PDF com line spacing default.
- `typst com.typ` (`#set par(leading: 20pt)`) → PDF com linhas
  +20pt entre baselines. **Diferença visível**.
- Td positions numericamente verificáveis (grep no PDF).

---

## Ficheiros tocados

| Ficheiro | Mudança |
|----------|---------|
| `01_core/src/rules/layout/cursor.rs` | `flush_line` peek current_line, soma leading ao line_height |
| `01_core/src/rules/layout/tests.rs` | +3 tests integration + helper `text_items_with_xy` |
| `00_nucleo/DEBT.md` | Gap 3 DEBT-52 marcado resolvido |

**Exporter inalterado**. Frame carrega positions finais; PDF
reflecte automaticamente.

### Números finais

| Métrica | Antes (137) | Depois |
|---------|------:|-------:|
| L1 tests | 861 | **864** (+3) |
| L2 tests | 24 | 24 |
| L3 tests | 186 | 186 |
| L4 tests | 21 | 21 |
| **Total** | **1092** | **1095** (+3) |
| Violations | 0 | 0 |
| ADRs activas | 54 | 54 |
| DEBTs abertos | 12 | 12 (DEBT-52 gap 3 ✓) |

---

## Mudança observável

**Segundo efeito visível** da série 126-138. Utilizador com
`#set par(leading: Xpt)` vê:
- **Linhas mais espaçadas** verticalmente (ou menos — valor
  pode ser negativo?).
- **Cada linha após flush_line** ganha X pt extra de advance
  vertical.
- **Primeira linha** não afectada (leading é *entre* linhas).

**Tracking + leading combinados**: um input `#set text(tracking:
2pt)\n#set par(leading: 20pt)` tem efeito em ambos os eixos
sem conflito. Cada consumer é independente.

---

## Trajectória de `leading` — caso de estudo

Esta propriedade teve **5 passos** dedicados:

1. **128** (130 days ago-ish): capturada em `text` com
   divergência temporal aceite.
2. **131B**: Lang materialization (não afectou leading).
3. **133**: target `par` activado como known (infra).
4. **134**: `leading` migrado de `text` para `par`
   (contexto canónico, paridade ADR-0033).
5. **136**: propagação de `StyleDelta.leading` para
   `TextStyle.leading` (Fase A infra).
6. **138**: consumer activo — **efeito visível no PDF**.

**ADR-0033 lido literal** produziu esta sequência: cada passo
necessário, acumulação razoável. Outros DEBT-1 properties
beneficiam do mesmo rigor: weight teve trajectória análoga
(126 captura u16, 129 captura simbólico, 136 propagação,
139 consumer).

---

## Lições

1. **Obstáculo descoberto empiricamente paga-se**: expectativa
   inicial (ler `self.style.leading`) falhou porque o
   `prev_style` restore ocorre antes de `flush_line`. Solução
   (peek current_line) só surgiu após ver o test failure.
   **Test-driven descobre quirks** da estrutura actual.

2. **Ausência de `Content::Parbreak` é constraint real**:
   cristalino não tem paragraph break explícito. Tests precisam
   de headings ou wrap para forçar `flush_line`. Documentar
   como propriedade do code base (candidato a ADR futuro se
   alguma vez for alterado).

3. **Opção soma vs substituir**: escolha deliberada de
   divergência. Substituir exigiria extrair `size_core sem
   default leading` da `metrics.vertical_metrics`, que não é
   trivial (o default leading está baked-in no 1.2 factor).
   Soma é observacionalmente "direcção certa" — ADR-0054
   perfil graded aceita.

4. **Exporter inalterado é arquitectura limpa**: vs. 137 (onde
   PDF Tc precisou ser emitido), 138 é pure-layouter. Baseline
   advance numérico já é consumido pelo Td operator
   automaticamente. Frame carries final positions — separation
   of concerns validada.

5. **Paridade numérica exacta**: sem.y=747.0, com.y=727.0, Δ=20pt
   igual ao `leading: 20pt` input. Fórmula `resolve_pt` funciona
   precisamente; não há rounding loss.

6. **`text_items_with_xy` complementa `text_items_with_pos`**:
   two-helper pattern expande eixos de teste. Quando 139 e
   140+ precisarem de outras dimensões (estilo embebido, font
   ref), pode haver um `text_items_with_full` genérico. Evitar
   premature abstraction — adicionar quando o tercer consumer
   precisar.

---

## Estado pós-Passo 138

### DEBT-52 progresso

- ✓ Gap 1 (Fase A). Passo 136.
- ✓ Gap 2 (Fase B.1 tracking). Passo 137.
- ✓ Gap 3 (Fase B.2 leading). **Passo 138 — este**.
- ⏳ Gap 4 (Fase B.3 weight faux-bold). **Próximo**.
- ⏳ Gap 5-7 (Fase C).
- ⏳ Gap 8 (opcional, font dict).

### Próximo passo sugerido: **Passo 139** (Fase B.3)

Consumer `weight` faux-bold.

**Escopo**: aplicar `weight` como PDF stroke (faux-bold) ou
selecção de variante (F1/F2/F3 map). Requer decisão sobre
strategy — PDF `2 Tr` (stroke) como quick win sem embedding
real de fontes, ou mapear weight ranges para F2 (bold)
existente.

**Estimativa**: S. Similar infraestrutura ao 137 (exporter
change).

---

## Candidato futuro registado

- **Paridade exacta vanilla** para leading: refactor
  `metrics.vertical_metrics` para retornar `(size_core,
  default_line_gap)` separadamente. Permite opção subst
  vanilla. Passo dedicado.
- **`Content::Parbreak`**: adicionar explícito para suportar
  paragraph breaks — permite testes `"linha1\n\nlinha2"`
  como esperado. Refactor médio.
- **Golden PDF tests**: regressão binária quando infra chegar.
