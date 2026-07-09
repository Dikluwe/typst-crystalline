# Relatório de Paridade — P658

**Passo:** 658  
**Data:** 2026-07-09  
**Foco:** Estender a cache de resultados de shaping (P657) a `shaped_width` (P591).  
**Dependências:** P657 (cache em `shape_document`), P591 (`shaped_width`).  
**Hash do commit com o relatório:** `aee92bc31`

---

## 1. Sonda

### 1.1 Documento árabe repetido

```bash
python3 -c "
for i in range(500):
    print('الكتاب على الطاولة يوم جميل')
" > /tmp/p658-arabe-repetido.typ
sed -i '1i #set text(lang: \"ar\", dir: rtl, font: \"DejaVu Sans\", size: 20pt)' /tmp/p658-arabe-repetido.typ
./target/release/typst /tmp/p658-arabe-repetido.typ /tmp/p658-baseline.pdf --timings-json /tmp/p658-baseline-timings.json
```

Resultado baseline:

```json
{
    "layout_ms": 5.086795,
    "shape_ms": 283.027917,
    "total_ms": 294.258994
}
```

A fase `layout_ms` é pequena (≈5 ms) porque o documento tem apenas 500 linhas de texto puro.

### 1.2 Cache existente em `shaped_width`

`03_infra/src/font_metrics.rs:352` já contém `cached_shaped_width`, uma cache própria para `advance_shaped`:

- Chave: `ShapedWidthKey { text, size_bits, font_hash, bold, italic, weight, dir, lang }`.
- Desactivada quando `style.tracking.is_some()`.
- Guarda `Option<Pt>` (largura), não os glifos.

`shaped_width` (a função pública) é chamada apenas a partir de `cached_shaped_width` em `font_metrics.rs`.

### 1.3 Partilha com a cache de P657

A `ShapeCache` de P657 é criada localmente em `shape_document` (`03_infra/src/shaper.rs:43`) e passada pelas funções internas `shape_page` / `shape_item` / `try_shape`. `shaped_width` é chamado durante o **layout**, que acontece **antes** de `shape_document`. Portanto:

- `shaped_width` não pode reutilizar a cache de P657 (ainda não existe quando o layout corre).
- `shape_document` poderia reutilizar resultados de `shaped_width`, mas isso exigiria partilhar a cache de `FallbackFontMetrics` com `ShapeCache` — refactor da interface entre layout e shaper.

### 1.4 Taxa de repetição medida em `shaped_width`

Instrumentação temporária (não commitada) em `cached_shaped_width`:

```text
[P658 instrumentação] shaped_width cache hits=19886 misses=10 total=19896 hit_ratio=99.95%
```

A cache existente de `shaped_width` já atinge **99,95%** de hit ratio no documento árabe repetido.

---

## 2. Decisão

Não foi implementada extensão da cache de P657 a `shaped_width`.

Razões:

1. `shaped_width` **já tem cache própria** (P591), com hit ratio de 99,95% no caso de teste.
2. A cache de P657 só existe durante `shape_document`, que corre **depois** do layout; portanto `shaped_width` não pode consultá-la directamente.
3. Partilhar a cache entre layout e `shape_document` exigiria refactor significativo da interface (`FallbackFontMetrics` ↔ `ShapeCache`) para um ganho marginal, dado que ambas as caches já são eficazes independentemente.
4. O documento de teste não mostrou `layout_ms` dominante — a fase demorou ~5 ms, indicando que a cache existente já está a fazer o seu trabalho.

A "falha silenciosa" potencial (shaping repetido em `shaped_width`) já está coberta pela cache de P591. A não-partilha entre as duas caches pode causar trabalho duplicado no limite, mas ambos os caminhos estão cacheados internamente.

### Nota sobre a chave de `shaped_width`

A chave `ShapedWidthKey` **não inclui variações de eixo OpenType** (`axis_vars`). Para documentos com variation fonts e eixos diferentes, a cache de `shaped_width` pode devolver valores incorrectos. Isto é uma limitação da cache existente, não do scope deste passo; uma correcção futura deveria alinhar a chave com a de P657 (texto + face + direção + variações + tracking).

---

## 3. Validação

```bash
cargo test --workspace
```

Resultado: todos os crates passaram.

```bash
crystalline-lint .
```

Resultado: `✓ No violations found`.

---

## 4. Conclusão

- A sonda confirmou que `shaped_width` já está eficazmente cacheado (99,95% hit ratio).
- A extensão directa da cache de P657 não é viável devido à ordem do pipeline (layout antes de `shape_document`).
- Não houve alterações de código neste passo; o trabalho foi puramente de medição e decisão documentada.
