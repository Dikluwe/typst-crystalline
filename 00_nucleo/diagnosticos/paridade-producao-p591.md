# Relatório Diagnóstico — Passo 591
## Usar largura com forma de escrita aplicada na decisão de quebra de linha

- **Commit de Referência:** `6137c498b` (Working Tree não commitado)
- **Data/Hora da Medição:** 2026-07-07 03:35:33 UTC
- **ADR Base:** `00_nucleo/adr/adr-paridade-defeitos-testes.md`

---

## 1. Objetivo

P590 confirmou que o cristalino decide onde quebrar a linha somando larguras de letras árabes isoladas, mas desenha o texto com letras ligadas. Uma palavra medida em 159.57 pt saía desenhada com 107.32 pt. Este passo corrige a decisão de quebra para usar a largura real com forma de escrita aplicada.

---

## 2. Sonda

### 2.1 O mecanismo de shaping já consegue devolver só a largura

O shaper (`03_infra/src/shaper.rs`) já usa `rustybuzz::shape` e obtém `glyph_positions()` com `x_advance`. Não é necessário gerar o resultado final duas vezes — basta somar os `x_advance` dos glyphs.

### 2.2 Outros scripts — hebraico

Teste com texto hebraico (`lang: "he"`, `dir: rtl`, `DejaVu Sans`):

```text
Hebraico: 5 palavras, vanilla e cristalino produzem 2 linhas idênticas.
```

O hebraico não liga letras, logo o problema não aparece. A correção é específica de scripts com formas contextuais obrigatórias (árabe, síriaco, etc.).

### 2.3 Custo antes da otimização

Medição inicial (sem cache):

| Documento | Baseline (P590) | P591 sem cache | Regressão |
|---|---|---|---|
| Latin 2000 palavras | 489.0 ms | 494.9 ms | +1.2% |
| Árabe 2400 palavras | 713.7 ms | 3066 ms | +330% |

A primeira implementação chamava o shaper para cada palavra árabe do zero — inaceitável.

---

## 3. Implementação

### 3.1 Interface `FontMetrics::advance_shaped`

Ficheiro: `01_core/src/rules/layout/metrics.rs`

```rust
pub trait FontMetrics: Send + Sync {
    fn advance(&self, text: &str, size: Pt, style: &TextStyle) -> Pt;
    // ...
    fn advance_shaped(&self, text: &str, size: Pt, style: &TextStyle) -> Option<Pt> {
        None
    }
}

pub fn needs_shaped_width(text: &str) -> bool {
    text.chars().any(|c| {
        matches!(
            c.script(),
            Script::Arabic | Script::Syriac | Script::Mongolian | Script::Nko | Script::Mandaic
        )
    })
}
```

### 3.2 `layout_word` usa `advance_shaped` quando disponível

Ficheiro: `01_core/src/rules/layout/cursor.rs:96`

```rust
let w = self.metrics
    .advance_shaped(word, self.style.size, &self.style)
    .unwrap_or_else(|| self.word_width(word));
```

### 3.3 `shaped_width` no shaper

Ficheiro: `03_infra/src/shaper.rs`

Função pública `shaped_width(world, text, style)` que replica a resolução de fontes do shaper mas retorna apenas `Option<Pt>`.

### 3.4 `FallbackFontMetrics::advance_shaped` com cache

Ficheiro: `03_infra/src/font_metrics.rs`

- Detecta scripts contextuais via `needs_shaped_width`.
- Chama `shaped_width`.
- Cache por `(texto, size, font_hash, bold, italic, weight, dir, lang)`.
- Bypass do cache quando `tracking` está activo.

### 3.5 Reordenação bidi e `fix_line_positions` também usam shaped width

Ficheiros:
- `03_infra/src/layout_bidi.rs` — helper `text_width_for_bidi` substitui `metrics.advance` em 5 locais.
- `03_infra/src/shaper.rs` — `estimate_width` em `fix_line_positions` tenta `advance_shaped` primeiro.

---

## 4. Resultados

### 4.1 Documento de referência RTL — agora numa linha

```typst
#set text(dir: rtl, lang: "ar", size: 40pt, font: "DejaVu Sans")
الكتاب 42 على الطاولة
```

Cristalino (P591):

```text
left=123.65 width=121.28  text=ةلواطلا
left=257.61 width=70.44   text=ىلع
left=340.78 width=50.88   text=42
left=404.39 width=107.32  text=باتكلا
```

Vanilla (P590):

```text
left=136.36 width=121.25  text=ةلواطلا
left=270.33 width=70.45   text=ىلع
left=353.49 width=50.90   text=42
left=417.10 width=107.30  text=باتكلا
```

- As 4 palavras cabem numa linha.
- Larguras individuais batem dentro de 0.03 pt.
- Posições `left` diferem em ~12.71 pt (um `space_width`) — diferença de alinhamento/espaçamento residual, não de quebra.

### 4.2 Performance após cache

| Documento | Baseline (P590) | P591 com cache | Delta |
|---|---|---|---|
| Latin 2000 palavras | 489.0 ms | 495.2 ms | +1.3% |
| Árabe 2400 palavras | 713.7 ms | 700.8 ms | −1.8% |

Sem regressão mensurável. A cache elimina o custo de shaping repetido.

---

## 5. Decisão

- A sequência RTL está **fechada quanto à quebra prematura causada por falta de shaping**.
- A diferença residual de ~12.71 pt no posicionamento horizontal é aceitável para este passo; não afecta a quebra de linha nem a legibilidade.
- A correção é segura para latim (não activa shaping) e não introduz regressão de desempenho.

---

## 6. Validação

```bash
cargo build --workspace --release
cargo test --workspace
crystalline-lint .
```

Resultados:

- `cargo build --workspace --release`: sucesso.
- `cargo test --workspace`: sucesso (`typst-core` 3572 passados, `typst-infra` 598 passados, 5 ignorados, restantes suites limpas).
- `crystalline-lint .`: `✓ No violations found`.

### Teste de regressão adicionado

`03_infra/src/font_metrics.rs::p591_advance_shaped_arabico_reduz_largura`:
- Confirma que `"الكتاب"` fica significativamente mais estreito com shaping.
- Confirma que `"42"` não sofre shaping contextual.

---

## 7. Tabela de estado da sequência RTL

| Passo | Estado | Nota |
|---|---|---|
| P560–P588 | Fechado | Causas anteriores corrigidas (font_size_pt, espaço inicial). |
| P590 | Fechado | Identificada causa de shaping não aplicado na medição. |
| **P591** | **Fechado** | Decisão de quebra usa largura shaped; documento de referência cabe numa linha; sem regressão de performance. |
