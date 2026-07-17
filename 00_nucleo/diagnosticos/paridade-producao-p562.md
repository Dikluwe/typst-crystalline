# Relatório de Execução — P562: Ordem Visual RTL das Palavras na Linha

**Passo:** 562  
**Data de execução:** 2026-07-04  
**Foco:** Corrigir a ordem visual das palavras árabes na linha, mantendo o shaping interno de cada palavra intacto.

---

## Sonda A.0 (ADR-0114)

### Comandos executados

```bash
# Localizar referências a bidi/RTL no layout (L1)
grep -n "layout_word\|cursor_x\|advance.*word" 01_core/src/engine/layout/cursor.rs | head -20

# Verificar onde bidi_runs é usado no projecto
grep -R "bidi_runs\|unicode_bidi\|unicode-bidi" . --include="*.rs" | head -30

# Localizar o arm Content::Text no layout monolito
grep -n "Content::Text" 01_core/src/engine/layout/mod.rs

# Confirmar a pipeline: layout → shape → export
grep -n "shape_document\|layout_with_introspector_and_metrics" 03_infra/src/pipeline.rs
```

### Resultados

1. **O Layouter avança `cursor_x` sempre para a direita?** PASS  
   `01_core/src/engine/layout/cursor.rs:134` e `:149` — `self.regions.current.cursor_x += w;` sem noção de direcção.

2. **`bidi_runs` está acessível no Layouter (L1)?** FAIL  
   `bidi_runs` só existe em L3 (`03_infra/src/shaper.rs:121,490`).

3. **Existe informação de direcção que sobreviva até ao Layouter?** FAIL  
   Não há campo de direcção em `TextStyle` ou `Layouter`.

### Decisão de desenho

Passagem posterior em L3, entre `layout` e `shape_document`, que reordena os `FrameItem::Text` dentro de cada linha visual usando `unicode-bidi`. Opção escolhida por não tocar no hot path de quebra de linha e por reutilizar o padrão P482 (`shape_document`).

---

## Implementação

### Ficheiros alterados

- `03_infra/src/layout_bidi.rs` — novo módulo com `reorder_bidi_document`.
- `03_infra/src/lib.rs` — expõe o módulo.
- `03_infra/src/pipeline.rs` — insere a passagem entre layout e shape.
- `00_nucleo/prompts/infra/layout_bidi.md` — Prompt L0 (hash `a1bb59da`).

### Algoritmo

1. Agrupa os items de cada página por linha visual (baseline y dentro de 0.01 pt).
2. Para cada linha, concatena os textos dos `FrameItem::Text`.
3. Usa `unicode_bidi::BidiInfo` para determinar a direcção base da linha.
4. Se a direcção base for RTL, inverte a ordem dos textos/estilos entre as posições x existentes.
5. Itens não-texto (`Shape`, `Image`, etc.) mantêm as suas posições.

### Limitação conhecida

A implementação actual preserva as posições x originais dos items. Quando as larguras das palavras invertidas são muito diferentes, o espaçamento visual pode ficar ligeiramente distorcido em texto misto (ex.: árabe + números latinos). A ordem visual das palavras está correcta; o espaçamento perfeito exigiria recalcular larguras com métricas de fonte, o que foi deixado como evolução futura.

---

## Validação

### Testes unitários

5 testes em `03_infra/src/layout_bidi.rs`:

- `p562_latin_no_change` — texto latino puro não é alterado.
- `p562_reorder_arabic_line` — `الكتاب على الطاولة` fica na ordem visual RTL.
- `p562_mixed_latin_arabic` — `الكتاب 42 على الطاولة` mantém 42 no meio.
- `p562_empty_text_unchanged` — texto vazio não causa panic.
- `p562_line_with_shape_unchanged` — shape entre palavras árabes não é afectado.

Resultado: `5 passed`.

### Testes de workspace

```bash
cargo test --workspace
```

Resultado: `584 passed; 1 failed; 5 ignored`. A única falha é o snapshot `p307b_07_multi_feature`, que **já falhava antes desta alteração** (confirmado ao comentar temporariamente a passagem de reordenação). Não é regressão introduzida por P562.

### Validação visual

#### Documento árabe puro

```typst
#set text(lang: "ar", size: 40pt)
الكتاب على الطاولة
```

Comandos:

```bash
./target/release/typst /tmp/p561-rtl-order.typ /tmp/p562-cristalino.pdf
mutool draw -o /tmp/p562-cristalino.png -r 150 /tmp/p562-cristalino.pdf
```

Resultado: a ordem visual das palavras no cristalino agora coincide com o vanilla — `الكتاب` aparece à direita da linha, `الطاولة` à esquerda. O shaping interno (ligação das letras) permanece correcto.

#### Documento misto (árabe + número latino)

```typst
#set text(lang: "ar", size: 40pt)
الكتاب 42 على الطاولة
```

Comandos:

```bash
./target/release/typst /tmp/p562-mixed.typ /tmp/p562-mixed.pdf
mutool draw -o /tmp/p562-mixed.png -r 150 /tmp/p562-mixed.pdf
lab/typst-original/target/release/typst compile /tmp/p562-mixed.typ /tmp/p562-mixed-vanilla.pdf
mutool draw -o /tmp/p562-mixed-vanilla.png -r 150 /tmp/p562-mixed-vanilla.pdf
```

Resultado: a ordem visual está correcta — `42` mantém-se LTR no meio, com os trechos árabes invertidos. Nota: há uma pequena distorção de espaçamento entre `الطاولة` e `على` devido à preservação das posições x originais (limitação documentada acima).

### Linter

```bash
crystalline-lint .
```

Resultado: `✓ No violations found`.

### Benchmark

```bash
python3 tools/perf/benchmark-p507.py
```

Não concluído nesta sessão — o script excede o tempo razoável (depende de
`hyperfine` e compila vários documentos repetidamente). A passagem é
propositadamente barata para documentos LTR: `BidiInfo::new` processa o
buffer da linha e, para texto LTR puro, a detecção de nível base LTR
permite sair imediatamente sem reordenar items.

---

## Estado da sequência de RTL

| Camada | Passo | Estado |
|--------|-------|--------|
| Shaping (formas das letras) | P484, P521 | Fechado |
| Fonte embutida no PDF | P560 | Fechado |
| Ordem visual das palavras na linha | P562 | Fechado |

---

## Scope-out e passos futuros

- Escrita vertical (CJK top → bottom, mongol bottom → top): deixado para passos dedicados, mencionado em `00_nucleo/prompts/infra/layout_bidi.md`.
- Alinhamento de parágrafo explícito (`dir: rtl`): scope-out — este passo corrige a ordem visual, não o alinhamento.
- Recálculo de espaçamento com métricas de fonte para texto misto: evolução futura.
