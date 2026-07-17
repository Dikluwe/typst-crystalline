# Prompt L0 — `stdlib/transforms` — módulo `transforms`
Hash do Código: 68c52ac8

**Camada**: L1
**Ficheiro alvo**: `01_core/src/engine/stdlib/transforms.rs`
**Origem**: Passo 96.5 (extraído de `stdlib.rs` conforme ADR-0037), com marcos
P78 (`move`/`rotate`/`scale`) e P156F (`skew`).
**ADRs**: ADR-0037 (coesão por domínio), ADR-0054 (perfil graded), ADR-0061
(layout roadmap).
**Convenções partilhadas**: ver `00_nucleo/prompts/engine/stdlib/_comum.md`.

---

## Módulo `transforms` — funções nativas de transformação

Este módulo implementa transformações geométricas 2D aplicadas a content:
translação, rotação, escala e distorção (skew). Todas produzem
`Content::Transform { matrix, body }` com uma `TransformMatrix` adequada.

Todas as funções partilham a assinatura padrão de `native_*`:

```rust
fn native_X(
    ctx: &mut EvalContext,
    args: &Args,
    world: &dyn World,
    current_file: FileId,
) -> SourceResult<Value>
```

O corpo (`body`) é sempre o primeiro argumento posicional do tipo `Content`.

---

### `native_move(dx?, dy?, body)`

**Assinatura**: `move(dx: Length?, dy: Length?, body: Content) -> Content`

**Argumentos**:
- `dx`, `dy`: deslocamento em pt (`Length`, `Float`, `Int`). Default `0.0`.
- `body`: primeiro argumento posicional `Content` obrigatório.
- Não aceita outros argumentos nomeados.

**Semântica**: Cria `Content::Transform { matrix: translate(dx, dy), body }`.

**Paridade vanilla**: Equivalente a `#move(dx: 1cm, dy: 2cm, [body])`.

**Limitações / scope-outs**:
- `origin` scope-out (paridade vanilla).

**Testes canónicos**:
```
move([x]) -> Transform translate(0,0)
move(dx: 5pt, [x]) -> Transform translate(5,0)
move(dx: 1cm, dy: 2cm, [x]) -> Transform translate(1cm,2cm)
move(dx: -3pt, [x]) -> Transform translate(-3,0)
move() -> Err "exige um corpo de conteúdo"
```

---

### `native_rotate(angle, body)`

**Assinatura**: `rotate(angle: Angle | Float, body: Content) -> Content`

**Argumentos**:
- `angle`: `Value::Angle` (graus → radianos) ou `Value::Float` (radianos
  directos). Pode vir como named `angle:` ou como primeiro argumento
  posicional compatível.
- `body`: primeiro argumento posicional `Content` obrigatório.

**Semântica**: Cria `Content::Transform { matrix: rotate(angle_rad), body }`.

**Paridade vanilla**: Equivalente a `#rotate(45deg, [body])` / `#rotate(1rad, [body])`.

**Limitações / scope-outs**:
- `origin` scope-out (paridade vanilla).

**Testes canónicos**:
```
rotate(45deg, [x]) -> Transform rotate(pi/4)
rotate(0rad, [x]) -> Transform rotate(0)
rotate([x]) -> Err "exige um corpo" (ou angle default 0 se body presente)
rotate(45deg) -> Err "exige um corpo de conteúdo"
```

---

### `native_scale(x?, y?, body)`

**Assinatura**: `scale(x: Float | Int?, y: Float | Int?, body: Content) -> Content`

**Argumentos**:
- `x`: factor de escala horizontal. Default `1.0`.
- `y`: factor de escala vertical. Default igual a `x` (escala uniforme).
- `body`: primeiro argumento posicional `Content` obrigatório.

**Semântica**: Cria `Content::Transform { matrix: scale(sx, sy), body }`.

**Paridade vanilla**: Equivalente a `#scale(2, [body])` / `#scale(x: 2, y: 0.5, [body])`.

**Limitações / scope-outs**:
- `origin` scope-out (paridade vanilla).

**Testes canónicos**:
```
scale(2, [x]) -> Transform scale(2,2)
scale(x: 2, y: 0.5, [x]) -> Transform scale(2,0.5)
scale([x]) -> Transform scale(1,1)
scale(2) -> Err "exige um corpo de conteúdo"
```

---

### `native_skew(ax?, ay?, body)`

**Assinatura**: `skew(ax: Angle | Float?, ay: Angle | Float?, body: Content) -> Content`

**Argumentos**:
- `ax`: ângulo de distorção horizontal (default `0`). Aceita `Angle` ou
  `Float` (radianos).
- `ay`: ângulo de distorção vertical (default `0`).
- `body`: primeiro argumento posicional `Content` obrigatório.

**Semântica**: Cria `Content::Transform { matrix: skew(ax_rad, ay_rad), body }`.

**Paridade vanilla**: Equivalente a `#skew(30deg, [body])` / `#skew(ax: 10deg, ay: 5deg, [body])`.

**Limitações / scope-outs**:
- `origin` scope-out (paridade vanilla).
- Ângulos com `|a| >= π/2 - 1e-3` rad rejeitados porque `tan` diverge.

**Testes canónicos**:
```
skew(30deg, [x]) -> Transform skew(pi/6, 0)
skew(ax: 10deg, ay: 5deg, [x]) -> Transform skew(10°, 5°)
skew(90deg, [x]) -> Err "ângulo demasiado próximo de ±π/2"
skew([x]) -> Transform skew(0,0)
skew(10deg) -> Err "exige um corpo de conteúdo"
```
