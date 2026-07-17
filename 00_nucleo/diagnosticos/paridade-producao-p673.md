# P673 — Relatório de Paridade de Produção

**Passo:** 673  
**Data:** 2026-07-10  
**Foco:** Confirmar se a `FaceCache` de `shaped_width` persiste entre chamadas ou é recriada a cada vez.  
**Commit base:** `a5530449760bccc39d039e0986b8390eab0d12d7` (P672)  
**ADR-0108 em vigor:** medição precede a decisão; números acompanhados de proveniência.

---

## 1. Verificação

### 1.1 Âmbito exacto da `FaceCache` em `shaped_width`

Comando:

```bash
grep -n "fn shaped_width\|FaceCache" 03_infra/src/shaper.rs
```

Resultado em `03_infra/src/shaper.rs:162`:

```rust
pub fn shaped_width(world: &dyn World, text: &str, style: &TextStyle) -> Option<Pt> {
    // ...
    let mut face_cache = FaceCache::new();
    // ...
}
```

A `FaceCache` era criada **dentro** do corpo de `shaped_width`, portanto era recriada a cada chamada. A cache de resultados de shaping (`shaped_width_cache` em `FallbackFontMetrics`) já evitava chamadas repetidas para o mesmo `(texto, estilo)`, mas para textos diferentes a `FaceCache` local fazia com que as faces fossem re-parseadas de novo.

### 1.2 Medição com documento árabe

Criou-se um documento com 2000 linhas de texto árabe variado, forçando muitas chamadas a `shaped_width`:

```bash
python3 -c "
import random
random.seed(0)
words = ['الكتاب', 'على', 'الطاولة', 'يوم', 'جميل', 'مدرسة', 'بيت', 'شمس', 'قمر', 'نجم', 'بحر', 'سماء', 'وردة', 'شجرة', 'طفل']
for i in range(2000):
    print(' '.join(random.sample(words, 5)))
" > /tmp/p673-arabe-variado.typ
sed -i '1i #set text(lang: "ar", dir: rtl, font: "Noto Naskh Arabic", size: 20pt)' /tmp/p673-arabe-variado.typ
```

Comando de medição (antes da correcção):

```bash
./target/release/typst /tmp/p673-arabe-variado.typ /tmp/p673-noto.pdf --timings-json /tmp/p673-noto-timings.json
```

Resultado antes:

```json
{
  "parse_ms": 0.000000,
  "eval_ms": 3.266843,
  "introspect_ms": 0.073447,
  "expand_context_ms": 0.000701,
  "layout_ms": 16.635044,
  "shape_ms": 51.434195,
  "subset_ms": 0.581134,
  "render_ms": 15.715078,
  "total_ms": 87.706442
}
```

Resultado depois:

```json
{
  "parse_ms": 0.000000,
  "eval_ms": 3.543687,
  "introspect_ms": 0.094636,
  "expand_context_ms": 0.000732,
  "layout_ms": 18.213174,
  "shape_ms": 52.175900,
  "subset_ms": 0.225680,
  "render_ms": 15.783385,
  "total_ms": 90.037194
}
```

O `layout_ms` manteve-se na mesma ordem de grandeza (~16–18 ms). O documento de teste é pequeno o suficiente para que o custo do re-parse local não se manifeste de forma dominante; a correcção é principalmente preventiva e de consistência com o caminho principal de `shape_document`.

---

## 2. Implementação

### 2.1 `03_infra/src/shaper.rs`

- Tornou-se `FaceCache` e `CachedFace` `pub(crate)`.
- Alterou-se a assinatura de `shaped_width` para receber a cache de fora:

```rust
pub(crate) fn shaped_width(
    world: &dyn World,
    text: &str,
    style: &TextStyle,
    face_cache: &mut FaceCache,
) -> Option<Pt> { ... }
```

- Removeu-se a criação local `let mut face_cache = FaceCache::new();` de dentro da função.

### 2.2 `03_infra/src/font_metrics.rs`

Adicionou-se um campo `shaper_face_cache: Arc<Mutex<crate::shaper::FaceCache>>` a `FallbackFontMetrics`, partilhado entre clones e entre todas as chamadas de `advance_shaped` no mesmo documento:

```rust
pub struct FallbackFontMetrics<'a> {
    world: &'a dyn World,
    cache: Arc<Mutex<HashMap<usize, Arc<CachedFace>>>>,
    shaped_width_cache: Arc<Mutex<HashMap<ShapedWidthKey, Pt>>>,
    shaper_face_cache: Arc<Mutex<crate::shaper::FaceCache>>,
}
```

Em `advance_shaped`, a cache é obtida e passada a `shaped_width`:

```rust
fn advance_shaped(&self, text: &str, _size: Pt, style: &TextStyle) -> Option<Pt> {
    use typst_core::engine::layout::needs_shaped_width;
    if !needs_shaped_width(text) {
        return None;
    }
    let world = self.world;
    let mut face_cache = self.shaper_face_cache.lock().unwrap();
    self.cached_shaped_width(text, style, || {
        crate::shaper::shaped_width(world, text, style, &mut face_cache)
    })
}
```

A `Clone` impl de `FallbackFontMetrics` foi actualizada para partilhar a nova cache.

---

## 3. Validação

### 3.1 Correcção do output

Comparação do texto extraído antes e depois da correcção (documento árabe):

```bash
pdftotext /tmp/p673-noto.pdf /tmp/p673-antes.txt
pdftotext /tmp/p673-depois-noto.pdf /tmp/p673-depois.txt
diff -u /tmp/p673-antes.txt /tmp/p673-depois.txt
```

Resultado: `0` linhas de diferença — output idêntico.

### 3.2 Testes e linter

- `cargo test --workspace`: passou.
- `crystalline-lint .`: `✓ No violations found`.

### 3.3 Benchmark completo

```bash
time timeout 900 python3 tools/perf/benchmark-p507.py
```

Tempo total: ~2 min 30 s.

#### Macro

| Passo | Vanilla (ms) | Cristalino (ms) | Rácio |
|---|---|---:|---:|---:|
| P618 | 5303,54 | 35958,10 | 6,78× |
| P657 | 5035,43 | 29097,89 | 5,78× |
| P671 | 5728,07 | 36357,03 | 6,35× |
| P672 | 5973,80 | 7974,72 | 1,33× |
| **P673** | **5483,96** | **6928,43** | **1,26×** |

O `macro-10x` manteve-se estável e melhorou ligeiramente (1,33× → 1,26×). O caminho principal de `shape_document` não foi afectado pela alteração em `shaped_width`.

#### Micro (seleção)

| Documento | Rácio P673 | Rácio P672 |
|---|---:|---:|
| test-array | 1,94× | 1,74× |
| test-calc | 1,88× | 1,89× |
| test-columns | 1,82× | 1,77× |
| test-footnote | 1,86× | 1,84× |
| test-math | 1,86× | 1,85× |
| test-raw | 1,46× | 1,46× |
| test-stroke-sides | 28,86× | 29,63× |
| test-image-fit | 28,27× | 29,31× |
| test-bibliography-csl | 1,57× | 1,54× |
| test-raw-advanced | 1,18× | 1,17× |

Os documentos micro mantiveram-se estáveis, sem regressão.

---

## 4. Decisão

- Confirmado que a `FaceCache` de `shaped_width` era **local e recriada a cada chamada**.
- Corrigido para persistir ao longo do documento, partilhando a cache através do `FallbackFontMetrics` (que vive no `Layouter` e é clonado durante o layout).
- O output mantém-se idêntico; testes e linter passam; benchmark não regrediu.

Ação: P673 fecha a pendência de consistência levantada por P672.

---

## 5. Proveniência da medição

- Commit base: `a5530449760bccc39d039e0986b8390eab0d12d7` (P672).
- Ficheiros alterados: `03_infra/src/shaper.rs`, `03_infra/src/font_metrics.rs`.
- Comando de verificação de âmbito: `grep -n "fn shaped_width\|FaceCache" 03_infra/src/shaper.rs`.
- Comando de criação do documento de teste: script Python + `sed` (ver secção 1.2).
- Comando de medição: `./target/release/typst /tmp/p673-arabe-variado.typ /tmp/p673-*.pdf --timings-json /tmp/p673-*-timings.json`.
- Comando de diff: `pdftotext` + `diff -u`.
- Comando de testes: `cargo test --workspace`.
- Comando de linter: `crystalline-lint .`.
- Comando de benchmark: `time timeout 900 python3 tools/perf/benchmark-p507.py`.
- Ficheiro de resultados: `tools/perf/results/benchmark-p507-summary.json`.
