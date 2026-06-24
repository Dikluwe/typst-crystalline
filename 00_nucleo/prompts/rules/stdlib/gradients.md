# Prompt L0 — `stdlib/gradients` — módulo `gradient`
Hash do Código: c03deb9d

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/stdlib/gradients.rs`
**Origem**: Passo 96.5 (extraído de `stdlib.rs` conforme ADR-0037), com marcos
P262 (`gradient.linear`), P264 (`gradient.radial`) e P267 (`gradient.conic`).
**ADRs**: ADR-0037 (coesão por domínio), ADR-0054 (perfil graded), ADR-0087
(Gradient Linear-only, posteriormente expandido), ADR-0091 (ColorSpace runtime).
**Convenções partilhadas**: ver `00_nucleo/prompts/rules/stdlib/_comum.md`.
**Entidade subjacente**: ver `00_nucleo/prompts/entities/gradient.md`.

---

## Módulo `gradient` — funções nativas de gradiente

O módulo regista três funções nativas sob a chave `"gradient"` no scope global,
via `make_gradient_module()`. Cada função constrói um `Value::Gradient(...)`
com o tipo de gradiente correspondente. O parsing de stops é partilhado.

### Formato de stops

Argumentos posicionais variádicos (`args.items`). Cada stop aceita:

- `Value::Color(c)` → `GradientStop::unspaced(c)` (offset automático).
- `Value::Array([Color, Ratio])` → `GradientStop::new(c, offset)`.
- `Value::Array([Color, Float/Int])` → o número é convertido para `Ratio`.

Pelo menos um stop é obrigatório; zero stops → erro.

### ColorSpace

O named argument `space` aceita `"oklab"`, `"oklch"`, `"srgb"`, `"luma"`,
`"linear-rgb"`, `"hsl"`, `"hsv"`, `"cmyk"`. Default `"oklab"` (paridade vanilla).
A interpolação usa o dispatcher `interpolate_in_space` definido em
`entities/gradient.rs`.

### `relative`

Named argument `relative` aceita `"self"`, `"parent"` ou `"auto"`. Default
`"auto"` (resolvido como `Self_`). Representação interna `Option<RelativeTo>`.

---

### `native_gradient_linear` (`gradient.linear`)

**Assinatura vanilla**:
```typc
gradient.linear(..stops, angle: angle, space: "oklab", relative: "auto")
```

**Argumentos**:
- `..stops`: stops posicionais variádicos (pelo menos 1).
- `angle`: `Angle` ou `Float` (radianos). Default `0deg`.
- `space`: string de color space (ver acima). Default `"oklab"`.
- `relative`: `"self" | "parent" | "auto"`. Default `"auto"`.

**Semântica**: Constrói `Value::Gradient(Gradient::Linear(Linear { stops, angle, space, relative }))`.

**Paridade vanilla**: Equivalente a `#gradient.linear(red, blue, angle: 45deg)`.

**Limitações / scope-outs**:
- PDF render não desenha gradiente real; `Paint::to_color()` faz fallback para a
  cor do primeiro stop (`first_stop_color()`).
- `tiling(gradient)` é rejeitado em `visualize.rs` (scope-out ADR-0054).
- `anti_alias` não exposto.

**Testes canônicos**:
```
gradient.linear(red) -> Value::Gradient(Linear)
gradient.linear(red, blue, angle: 45deg) -> Linear com angle = 45deg
gradient.linear() -> Err "pelo menos 1 stop requerido"
gradient.linear(red, angle: 90deg, space: "srgb") -> space = Srgb
gradient.linear(red, relative: "parent") -> relative = Some(Parent)
gradient.linear(red, foo: 1) -> Err "argumento nomeado inesperado"
```

---

### `native_gradient_radial` (`gradient.radial`)

**Assinatura vanilla**:
```typc
gradient.radial(..stops, center: (50%, 50%), radius: 50%,
                focal-center: center, focal-radius: 0%,
                space: "oklab", relative: "auto")
```

**Argumentos**:
- `..stops`: stops posicionais variádicos (pelo menos 1).
- `center`: array `[Ratio, Ratio]` ou `[Float/Int, Float/Int]`. Default `(50%, 50%)`.
- `radius`: `Ratio`, `Float` ou `Int`. Default `50%`. Deve estar em `[0, 1]`.
- `focal_center`: mesmo formato de `center`. Default igual a `center`.
- `focal_radius`: mesmo formato de `radius`. Default `0%`.
- `space` / `relative`: idem `linear`.

**Semântica**: Constrói `Value::Gradient(Gradient::Radial(Radial { stops, center, radius, focal_center, focal_radius, space, relative }))`.

**Validações**:
- `focal_radius > radius` → erro.
- Distância de `focal_center` a `center` deve ser `< radius - focal_radius`
  (focal circle dentro do outer circle); caso contrário → erro.

**Paridade vanilla**: Equivalente a `#gradient.radial(red, blue)`.

**Limitações / scope-outs**:
- PDF render fallback para cor do primeiro stop.
- `tiling(gradient)` rejeitado.

**Testes canônicos**:
```
gradient.radial(red) -> Value::Gradient(Radial)
gradient.radial(red, blue, radius: 30%) -> radius = 0.3
gradient.radial(red, focal-radius: 10%) -> focal_radius = 0.1, focal_center = center
gradient.radial(red, focal-radius: 60%) -> Err "focal_radius > radius"
gradient.radial(red, center: (0%, 0%), focal-center: (80%, 80%), radius: 50%) -> Err "focal circle fora"
gradient.radial() -> Err "pelo menos 1 stop requerido"
```

---

### `native_gradient_conic` (`gradient.conic`)

**Assinatura vanilla**:
```typc
gradient.conic(..stops, center: (50%, 50%), angle: 0deg,
               space: "oklab", relative: "auto")
```

**Argumentos**:
- `..stops`: stops posicionais variádicos (pelo menos 1).
- `center`: array `[Ratio, Ratio]`. Default `(50%, 50%)`.
- `angle`: `Angle` ou `Float` (radianos). Default `0deg`.
- `space` / `relative`: idem `linear`.

**Semântica**: Constrói `Value::Gradient(Gradient::Conic(Conic { stops, center, angle, space, relative }))`.

**Paridade vanilla**: Equivalente a `#gradient.conic(red, blue, angle: 90deg)`.

**Limitações / scope-outs**:
- Sem `focal_*` (não existem em ConicGradient vanilla).
- PDF render fallback para cor do primeiro stop.
- `tiling(gradient)` rejeitado.

**Testes canônicos**:
```
gradient.conic(red) -> Value::Gradient(Conic)
gradient.conic(red, blue, angle: 90deg) -> angle = 90deg
gradient.conic(red, center: (25%, 75%)) -> center = (0.25, 0.75)
gradient.conic() -> Err "pelo menos 1 stop requerido"
gradient.conic(red, focal-radius: 5%) -> Err "argumento nomeado inesperado"
```

---

## Nota sobre scope-out de render PDF

As funções nativas **existem** e produzem `Value::Gradient`. A entidade
`Gradient` (Linear/Radial/Conic) está implementada em `entities/gradient.rs`,
incluindo interpolação em vários espaços de cor e auto-spacing de stops.
`Value::Gradient` tem `repr` básico em `eval/repr.rs` e é aceite como `Paint`
em `shapes.rs` (`parse_paint`).

O scope-out real está no **consumidor final de renderização**: quando o PDF
writer precisa de uma cor sólida, `Paint::to_color()` usa
`Gradient::first_stop_color()` como fallback. Não há shading PDF (`/Sh`)
implementado. Esta limitação é intencional no perfil graded (ADR-0054) e deve
ser declarada honestamente em qualquer trabalho futuro que reactive gradientes
no output.
