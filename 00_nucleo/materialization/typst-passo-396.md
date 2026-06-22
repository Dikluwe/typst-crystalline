# Passo 396 — Materialização: `tiling(...)` (M)

**Tipo**: Materialização (L1 — stdlib constructor + consumer layout; zero tipo novo; zero I/O).
**Data**: 2026-06-22.
**Padrão**: diagnóstico-primeiro (sonda 389 + P395); medir-antes-de-decidir (ADR-0108).
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0107 (paridade linguagem), ADR-0017 (portão aberto P395), ADR-0054 (graded scope-out — pattern fill PDF real).
**Sonda fonte**: `typst-sonda-ausentes-ordem-passo-389.md` §2D — `tiling()`, M, depende `Value::Tiling` (P395 fechado).

> **Nota de numeração.** Um passo só. Não numerar à frente.

---

## 1. Contexto

P395 modelou `Value::Tiling` (tipo L1) + `Paint::Tiling` (layout integration). Este passo materializa o **constructor user-facing** e o **consumer layout** que o torna funcional.

No vanilla:
```typ
#tiling(image("pat.png"), size: 50pt, relative: "self")
#rect(fill: tiling(red))  // Color como body simplificado
```

A paridade linguagem (ADR-0107) é: `tiling(body)` retorna um valor de preenchimento (paint) que pode ser usado em `fill`, `stroke`, e outros consumers de `Paint`.

---

## 2. Decisão de engenharia

### 2.1 — `native_tiling` stdlib

```rust
// signature paralela a native_rgb / native_image
native_tiling(
    body: Value,           // Image | Color | Str (path) — Str resolve via ImageSource
    size: Option<Size2D>,   // (width, height) — None ↔ auto
    relative: Option<Str>,  // "self" | "parent" — default "self"
    spacing: Option<Size2D>, // gap — None ↔ zero
) -> Value::Tiling(Arc<Tiling>)
```

**Decisão ADR-0107 (língua vs mecânica)**: o vanilla aceita `tiling(image)` e `tiling(color)` diretamente. No cristalino:
- `Value::Image(ImageSource)` → `TilingBody::Image(...)`
- `Value::Color(Color)` → `TilingBody::Color(...)`
- `Value::Str(path)` → resolve via `ImageSource::from_path` (reusa P72-74) — **graded**, pode ser scope-out se ImageSource não tiver path resolution pública em L1. Documentar no L0.
- `Value::Tiling(Tiling)` → identity (reusa pattern `native_rgb`).

**Decisão ADR-0054 (graded)**: `TilingBody::Gradient` é placeholder desde P395. `native_tiling` rejeita `Gradient` com erro claro: `"gradient em tiling não suportado — scope-out ADR-0054"`. Não panica; não silently fallback.

### 2.2 — Consumer layout (Paint::Tiling)

P395 integrou `Paint::Tiling` em `entities/paint.rs` com `to_color()` fallback. Este passo **ativa o consumer**:

- `layout/mod.rs` — shapes/boxes/blocks que consomem `Paint` já usam `paint.to_color()` ou match em `Paint`. Se o match atual é `Paint::Color(c)` apenas, expandir para:
  ```rust
  match paint {
      Paint::Color(c) => ...,
      Paint::Tiling(t) => {
          // ADR-0054 graded: pattern fill real é scope-out
          // Fallback: emite Color via t.to_color() (já implementado P395)
          // Consumer real ativado: bounds calculados com t.size / t.spacing
          let fallback = t.to_color().unwrap_or(Color::Rgb(Rgb::new(0,0,0)));
          // ... emite Shape com fallback color ...
      }
  }
  ```

**Decisão**: o consumer não precisa de novo código de render — `Paint::Tiling` já tem `to_color()` fallback. O que este passo adiciona é **validação de argumentos** no stdlib + **testes E2E** que provam que `tiling(...)` pode ser passado a `fill:` e produz output (mesmo que fallback color).

### 2.3 — `Size2D` parsing

`size` e `spacing` aceitam:
- `Value::Length(l)` → `Size2D::uniform(l)`
- `Value::Array([w, h])` → `Size2D::new(w, h)` (ambos Length)
- `Value::None` → `None` (auto / zero)

Helper privado `extract_size2d(value, fn_name, field_name)` — paralelo a `extract_sides_lengths` (P156L) e `extract_stroke` (P227).

---

## 3. FASE A — L0 (redação; checkpoint obrigatório)

### A.1 — Prompt L0 `tiling-stdlib.md`

Novo em `00_nucleo/prompts/rules/stdlib/tiling-stdlib.md` (ou extensão de `visualize.md` se existir; confirmar path):

- **Paridade**: `tiling(body)` ≡ constructor de pattern fill vanilla; retorna `Value::Tiling`.
- **Substrato**: stdlib helper que constrói `Tiling` via `Tiling::new` (P395).
- **Sem tipo novo**: reutiliza `Value::Tiling` (P395), `TilingBody`, `Paint::Tiling`.
- **Argumentos**:
  - `body` (obrigatório): `Image` | `Color` | `Str` (path) | `Tiling` (identity).
  - `size` (opcional): `Length` | `Array[Length, Length]` — `None` ↔ auto.
  - `relative` (opcional): `"self"` | `"parent"` — default `"self"`.
  - `spacing` (opcional): `Length` | `Array[Length, Length]` — `None` ↔ zero.
- **Erros**:
  - `body` não é Image/Color/Str/Tiling → tipo inválido.
  - `body` é `Gradient` (via TilingBody) → `"gradient em tiling não suportado — scope-out ADR-0054"`.
  - `relative` inválido → `"self" ou "parent"`.
  - `size`/`spacing` tipos inválidos → formato esperado.
- **Consumer**: `Paint::Tiling` em shapes/boxes/blocks via `to_color()` fallback (P395). Pattern fill real — scope-out ADR-0054.
- **Teste**: `rect(fill: tiling(red))` parse+eval → layout aceita → emite shape com fallback color.

### A.2 — CHECKPOINT

Parar. Apresentar `tiling-stdlib.md` ao dono. **Só prosseguir para Fase B quando confirmar que guardou e computou hash.**

---

## 4. FASE B — Código (após confirmação humana)

### B.1 — Implementar `native_tiling`

Em `rules/stdlib/visualize.rs` (ou módulo apropriado; confirmar path existente):

```rust
pub fn native_tiling(args: &Args) -> SourceResult<Value> {
    let body = args.expect::<Value>("body")?;
    let size = args.find::<Value>("size")
        .map(|v| extract_size2d(v, "tiling", "size"))
        .transpose()?;
    let relative = args.find::<Str>("relative")
        .map(|s| parse_tiling_relative(&s))
        .transpose()?
        .unwrap_or(TilingRelative::Itself);  // "self"
    let spacing = args.find::<Value>("spacing")
        .map(|v| extract_size2d(v, "tiling", "spacing"))
        .transpose()?;

    let tiling_body = match body {
        Value::Image(img) => TilingBody::Image(img.clone()),
        Value::Color(c) => TilingBody::Color(*c),
        Value::Str(path) => {
            // Graded: ImageSource from path — reusa P72-74 se disponível
            // Se não disponível em L1, retornar erro claro
            let img = ImageSource::from_path(&path)
                .map_err(|e| eco_format!("imagem não encontrada: {e}"))?;
            TilingBody::Image(img)
        }
        Value::Tiling(t) => return Ok(Value::Tiling(t.clone())),
        _ => bail!("body deve ser imagem, cor, caminho ou tiling"),
    };

    // Guard against Gradient placeholder (P395)
    if matches!(tiling_body, TilingBody::Gradient) {
        bail!("gradient em tiling não suportado — scope-out ADR-0054");
    }

    let tiling = Tiling::new(tiling_body)
        .with_size(size)
        .with_relative(relative)
        .with_spacing(spacing);

    Ok(Value::Tiling(Arc::new(tiling)))
}
```

**Helper `extract_size2d`**:
```rust
fn extract_size2d(value: &Value, fn_name: &str, field: &str) -> SourceResult<Size2D> {
    match value {
        Value::Length(l) => Ok(Size2D::uniform(*l)),
        Value::Array(arr) if arr.len() == 2 => {
            let w = arr[0].cast::<Length>()?;
            let h = arr[1].cast::<Length>()?;
            Ok(Size2D::new(w, h))
        }
        _ => bail!("{fn_name}: {field} deve ser length ou array de 2 lengths"),
    }
}
```

**Helper `parse_tiling_relative`**:
```rust
fn parse_tiling_relative(s: &str) -> SourceResult<TilingRelative> {
    match s {
        "self" => Ok(TilingRelative::Itself),
        "parent" => Ok(TilingRelative::Parent),
        _ => bail!("relative deve ser 'self' ou 'parent'"),
    }
}
```

### B.2 — Registar em `make_stdlib`

```rust
scope.define("tiling", Func::native(native_tiling));
```

### B.3 — Consumer layout (ativação)

Verificar todos os consumers de `Paint` em `layout/mod.rs` (e submódulos):

- `Shape::Rect` / `Shape::Ellipse` / etc. — `fill: Option<Paint>` já usam `paint.to_color()` ou match. Confirmar que `Paint::Tiling` é tratado (fallback via `to_color()` já existe P395).
- Se houver match exhaustivo em `Paint`, adicionar braço `Paint::Tiling(t)` se ainda não existir (deve existir desde P395, mas confirmar).
- **Não adicionar pattern fill real** — scope-out ADR-0054. O fallback Color é suficiente para paridade estrutural.

### B.4 — Testes

1. **Unit stdlib** (6-8 tests em `stdlib/mod.rs` ou `stdlib/visualize.rs`):
   - `tiling_color_body` — `tiling(red)` retorna `Value::Tiling` com `TilingBody::Color`.
   - `tiling_image_body` — `tiling(img)` retorna `Value::Tiling` com `TilingBody::Image`.
   - `tiling_str_path` — `tiling("path.png")` resolve imagem (se ImageSource suportar) ou erro claro.
   - `tiling_identity` — `tiling(tiling(red))` retorna identity.
   - `tiling_size_length` — `tiling(red, size: 50pt)` → `Size2D::uniform(50pt)`.
   - `tiling_size_array` — `tiling(red, size: (50pt, 30pt))` → `Size2D::new(50pt, 30pt)`.
   - `tiling_relative_parent` — `tiling(red, relative: "parent")` → `Parent`.
   - `tiling_invalid_body` — `tiling(123)` → erro tipo.

2. **Unit consumer layout** (2-3 tests em `layout/tests.rs`):
   - `rect_fill_tiling_color` — `#rect(fill: tiling(red))` → layout emite Shape com fill Color (fallback).
   - `box_fill_tiling_image` — `#box(fill: tiling(img))` → layout aceita, fallback Color se image não renderizável.
   - `block_fill_tiling` — `#block(fill: tiling(blue))` → paridade morfológica com `fill: blue`.

3. **E2E eval→layout** (2-3 tests em `integration_tests.rs` ou equivalente):
   - `tiling_como_argumento_fill` — parse + eval + layout pipeline completo.
   - `tiling_size_spacing` — argumentos complexos propagam.
   - `tiling_repr` — `repr(tiling(red))` retorna string esperada.

### B.5 — Linhagem

- `@prompt` aponta para `tiling-stdlib.md` + `tiling.md` (P395).
- `@prompt-hash` via `--fix-hashes`.
- Referência cruzada: P395 (tipo), ADR-0017 (portão aberto), ADR-0054 (graded).

---

## 5. O que NÃO fazer (scope-out)

- **Não** implementar pattern fill real em PDF — scope-out ADR-0054 (futuro XL).
- **Não** implementar `Gradient` como body de `tiling` — placeholder P395, rejeitar com erro.
- **Não** implementar `relative: "parent"` semantic real — armazenar, mas consumer usa igual (adiado até multi-region layout real).
- **Não** tocar em `export.rs` — fallback já existe P395.
- **Não** adicionar tipo novo — reusa P395.
- **Não** fazer `document`/`title`/`asset` — é P397.
- **Não** fazer `Value::Bytes`/`Decimal`/`Duration`/`Version` — são S, fluem depois.

---

## 6. Critérios de aceitação

1. `tiling(red)` compila e retorna `Value::Tiling` morfologicamente válido.
2. `#rect(fill: tiling(red))` parseia, evalua, e layout aceita (fallback Color).
3. Zero tipo novo; zero I/O; zero variant novo em Content/Value.
4. Testes verdes (≥12 unit + 3-5 integration); lint zero; hashes propagados.
5. Inventário 148: `tiling()` transita `ausente` → `implementado` (ou `implementado⁺` se graded scope-out pattern fill).
6. L0 salvo e hashado antes do código (protocolo de nucleação).
7. `Gradient` em body rejeitado com mensagem clara (paridade ADR-0054).

---

## 7. O que pode sair errado

- **`ImageSource::from_path` não existe em L1.** Mitigação: scope-out `Str` body com erro claro; documentar como graded. Teste `tiling_str_path` adapta para expectativa de erro.
- **`Size2D` não existe — é `Size` ou `(Length, Length)` em L1.** Mitigação: usar tipo existente; se for tuple, adaptar `extract_size2d`.
- **Consumers de `Paint` usam `unwrap` em `Paint::Color` assumindo só existe Color.** Mitigação: grep todos os `match paint` / `paint.to_color()` / `if let Paint::Color` no layout; adicionar braço Tiling se necessário.
- **Tentação de já fazer pattern fill PDF.** Mitigação: ADR-0054 é XL; fallback Color é suficiente para M.

---

## 8. Referências

- `typst-sonda-ausentes-ordem-passo-389.md` §2D — confirmação de `tiling()` como M, depende P395.
- P395 — `Value::Tiling` tipo modelado (portão ADR-0017 aberto).
- P72-74 — `Image`/`ImageSource` baseline (reuso para body Str).
- P102 — `Color` como fill primitivo (paradigma fallback).
- P156L — `extract_sides_lengths` helper (template para `extract_size2d`).
- P227 — `extract_stroke` helper (template para parsing composto).
- ADR-0017 — enum fechado, portão aberto.
- ADR-0054 — graded scope-out (pattern fill PDF real).
- ADR-0107 — paridade linguagem vs mecânica.

---

## 9. Nota sobre o Tekt

Este passo é o **consumo do portão ADR-0017** — P395 abriu o tipo, P396 ativa o constructor. A separação é intencional: P395 foi M puro de infraestrutura (enum fechado, match exhaustivo), P396 é M de funcionalidade (stdlib + consumer). Juntos fecham uma feature vanilla completa em dois passos bem-delimitados.

Registar o tempo de ciclo P395→P396 como baseline de **passos acoplados de tipo+consumer** — se o ciclo for curto, o portão ADR-0017 funciona; se for longo, o tipo foi modelado antes do consumer estar claro.
