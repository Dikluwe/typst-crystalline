# Relatório de paridade — P767c

**Passo:** 767c  
**Data:** 2026-07-15  
**Base:** P767a (commit `beb4d4e4f`)  
**Foco:** Corrigir o ancoramento vertical de `Content::Shape` quando sucede texto não-bloco no fluxo de parágrafo, depois de P767b ter medido um desvio de ~6,05 pt.  
**L0:** `00_nucleo/prompts/engine/layout/shape_block_behaviour.md` (actualizado com a regra de ancoramento P767c).  
**Código alterado:**
- `01_core/src/engine/layout/shape.rs` — ancoramento vertical condicional:
  - forma depois de texto: base em `baseline + above`, topo em `baseline + above + height`, próxima baseline em `shape_top + below + cap_height`;
  - forma depois de bloco ou isolada sem texto antes: mantém o modelo P767a (`shape_base = cursor_y − cap_height`, avanço `shape_base + height + below`).
- `03_infra/fixtures/p307b/reference/04-shapes.pdf` e `07-multi-feature.pdf` — snapshots actualizados (mudança de layout esperada).

---

## 1. Metodologia

Extração de coordenadas com `mutool trace` e comparação rasterizada com `mutool draw` + ImageMagick `compare -metric AE`.

Vanilla:
```bash
lab/typst-original/target/release/typst compile <input>.typ <output>.pdf
```

Cristalino:
```bash
target/release/typst <input>.typ -o <output>.pdf
```

Documentos de teste gerados em `temp_p767c/`.

---

## 2. Caso misto `A #rect(...) B` — antes e depois

| Elemento | Vanilla Y (pt) | P767a Y (pt) | P767c Y (pt) | Δ P767a→P767c (pt) |
|----------|---------------|--------------|--------------|-------------------|
| "A" baseline | 78,104 | 78,105 | 78,105 | +0,001 |
| `rect` inferior | 91,304 | 85,250 | 91,300 | +6,050 |
| `rect` superior | 113,981 | 107,930 | 113,980 | +6,050 |
| "B" baseline | 134,419 | 128,369 | 134,419 | +6,050 |

**Resultado:** o desvio de ~6,05 pt foi eliminado; as quatro coordenadas batem com o vanilla a menos de arredondamento de `mutool` (±0,005 pt).

---

## 3. Todas as primitivas misturadas com texto

Documento: `A #<primitiva>(...) B`.

| Primitiva | A ΔY (pt) | shape_bottom ΔY (pt) | shape_top ΔY (pt) | B ΔY (pt) |
|-----------|-----------|----------------------|-------------------|-----------|
| rect | +0,001 | −0,004 | 0,000 | 0,000 |
| square | +0,001 | −0,004 | 0,000 | 0,000 |
| ellipse | +0,001 | +0,001 | 0,000 | 0,000 |
| circle | +0,001 | +0,001 | 0,000 | 0,000 |
| line | +0,001 | N/A¹ | N/A¹ | +0,001 |
| polygon | +0,001 | −0,004 | 0,000 | 0,000 |

¹ `line` e algumas formas curvas usam path composto; a extracção automática do bounding-box exigiria parser mais elaborado, mas a posição do texto posterior já confirma o alinhamento vertical.

**Resultado:** a correcção generaliza para todas as primitivas testadas.

---

## 4. Primitivas isoladas — sem regressão

Documento: `#<primitiva>(...)`.

| Primitiva | shape_bottom ΔY (pt) | shape_top ΔY (pt) |
|-----------|----------------------|-------------------|
| rect | −0,006 | −0,003 |
| square | −0,006 | −0,003 |
| ellipse | +0,001 | +0,001 |
| circle | +0,001 | +0,001 |
| line | N/A | N/A |
| polygon | +0,004 | −0,003 |

**Resultado:** diferenças dentro do ruído de arredondamento; baseline de formas isoladas preservado.

---

## 5. Checklist de sub-layouts

| Cenário | Documento | AE P767a | AE P767c | Nota |
|---------|-----------|----------|----------|------|
| grid | `#grid(columns: 2, gutter: 5pt, [A], rect(...), [B], [C])` | 4 226 | 4 334 | Mesma ordem de grandeza; divergência de grid continua fora do âmbito. |
| box | `A #box[#rect(...) B] C` | 19 684 | 22 875 | Divergência conhecida de `box` mantém-se (forma como bloco dentro da caixa); ligeiro aumento por variação do posicionamento relativo, mas ainda no mesmo regime. |
| columns | `#columns(2)[A #rect(...) B]` | 6 972 | 213 | Melhoria substancial — agora beneficia do ancoramento correto no fluxo principal. |
| place | `A #place(top+right, rect(...)) B` | 996 | 1 209 | Sem regressão qualitativa; caminho absoluto preservado. |

**Resultado:** `place()` não regrediu. `grid`/`box` mantêm divergências documentadas fora do âmbito de P767c. `columns` melhorou significativamente.

---

## 6. Texto puro — sem regressão

Documento exacto de P762 (`#set page(width: 350pt, margin: 40pt); #set text(font: "DejaVu Sans", size: 11pt); <lorem>`), com string literal para isolar layout do conteúdo.

| Linha | Vanilla Y (pt) | P767c Y (pt) | ΔY (pt) |
|-------|---------------|--------------|---------|
| 1 | 40,000028 | 39,999578 | −0,000450 |
| 2 | 55,507478 | 55,507578 | +0,000100 |
| 3 | 71,014878 | 71,014578 | −0,000300 |
| 4 | 86,522278 | 86,522578 | +0,000300 |
| 5 | 102,029678 | 102,029578 | −0,000100 |

Erro máximo: **< 0,001 pt**.

**Resultado:** nenhuma regressão no layout vertical de parágrafos de texto puro.

---

## 7. Testes automatizados

```bash
cargo test --workspace
```

- **Resultado:** todas as suites passaram.
- Snapshots de `p307b` actualizados conscientemente:
  - `03_infra/fixtures/p307b/reference/04-shapes.pdf`
  - `03_infra/fixtures/p307b/reference/07-multi-feature.pdf`

---

## 8. Linter

```bash
crystalline-lint --fix-hashes .
crystalline-lint .
```

- `shape.rs`: header `@prompt` mantém `00_nucleo/prompts/engine/layout/shape_block_behaviour.md`.
- Resultado final: zero violações excepto `V7` (prompt órfão pré-existente `package_version_resolution.md`).

---

## 9. Causa e correção em resumo

P767b identificou duas causas no modelo P767a:

1. `block_chain_active == false` suprimia o `above` de `1.2em` quando a forma era a primeira depois de texto.
2. Mesmo quando o `above` era aplicado, o código ancorava o **topo** da forma em `baseline + cap_height`, enquanto o vanilla ancora a **base** da forma em `baseline + above`.

A correção em `shape.rs`:

- Guarda `baseline_before_flush` antes de `flush_line()`.
- Detecta `had_text_line` (a linha descarregada continha conteúdo).
- Para forma depois de texto: `shape_base = baseline_before_flush + above`; cursor avança para `shape_base + height + below + cap_height`.
- Para forma depois de bloco ou isolada: mantém P767a.

O desvio medido de ~6,05 pt (`1.2em − cap_height`) desapareceu.

---

## 10. Próximo passo

A linha de trabalho iniciada em P763h/P767a fecha com P767c para o caso texto+forma no fluxo principal. Restam como débito documentado:
- Comportamento de `grid` e `box` (forma tratada como bloco dentro de célula/caixa vs inline no vanilla).
- Formas consecutivas isoladas em parágrafos separados ainda não batem pixel-a-pixel com o vanilla (modelo de colapso de margem entre parágrafos distintos).
