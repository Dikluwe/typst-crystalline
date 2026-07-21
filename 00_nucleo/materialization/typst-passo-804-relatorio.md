# Relatório — typst-passo-804 (achado P798 #9): `visualize` — `#line(length: ...)` rejeitado, vanilla aceita

**Data:** 2026-07-21
**Executor:** Kimi Code (a pedido do utilizador, nesta conversa — prompt lido de `00_nucleo/materialization/typst-passo-804.md`)
**Proveniência das medições:** commit base `0661aef91c2ebc80d754d59e936c3a6350bfd543`. Sonda "antes" com working tree contendo P799–P803 (zonas não relacionadas). Validação "depois" com working tree não commitado: P799–P804. Hora da validação: 2026-07-21 ~16:50 -0300.
**Binários:** `./target/release/typst` (rebuild 16:50), `lab/typst-original/target/release/typst` (vanilla 0.15.0).

---

## Passo 1 — Sonda (antes)

Comandos: `./target/release/typst -o <out>.pdf <fonte>.typ 2>&1` / `lab/typst-original/target/release/typst compile <fonte>.typ <out>.pdf 2>&1`.

| Fonte | Cristalino (antes) | Vanilla |
|---|---|---|
| `#line(length: 3cm)` | `error: argumento nomeado inesperado em line(): 'length'` | compila (exit 0) |
| `#line(length: 3cm, angle: 30deg)` | idem | compila |
| `#line(length: 3cm, end: (1cm, 1cm))` | idem | compila — **`length` ignorado** |

Medição importante (Passo 1.2): o vanilla **não rejeita** `length`/`angle` combinados com `end` — a documentação no código diz "only respected if `end` is none" e a compilação confirma. Logo não há combinação inválida a validar (o Passo 2 do prompt previa essa possibilidade; foi refutada por medição e fica registado).

## Pontos exactos do código

**Vanilla** — `lab/typst-original/crates/typst-library/src/visualize/line.rs`:

```rust
pub struct LineElem {
    pub start: Axes<Rel<Length>>,
    pub end: Option<Axes<Rel<Length>>>,
    /// The line's length. This is only respected if `end` is `{none}`.
    #[default(Abs::pt(30.0).into())]
    pub length: Rel<Length>,
    /// ... only respected if `end` is `{none}`.
    pub angle: Angle,
    #[fold]
    pub stroke: Stroke,
}
```

Geometria (`crates/typst-layout/src/shapes.rs::layout_line`): sem `end`,
`delta = (cos(angle) · length, sin(angle) · length)`.

**Cristalino** — `01_core/src/engine/stdlib/shapes.rs::native_line` (antes): whitelist `["dx", "dy", "stroke", "start", "end"]`; `length`/`angle` rejeitados como "argumento nomeado inesperado" (scope-out de P739B).

## Passo 2 — Implementação

L0 actualizado primeiro: `00_nucleo/prompts/engine/stdlib/shapes.md`, secção `native_line` — assinatura com `length?`/`angle?`, semântica medida, scope-outs (`Ratio` em `length`, `start` ≠ 0). Hash corrigido (`shapes.rs` → `252a9a79`).

Diff resumido de `native_line`:
- Whitelist passa a incluir `"length"`, `"angle"`.
- Novo braço `None if contains length/angle`: `dx = cos(angle)·length`, `dy = sin(angle)·length`; defaults `30pt`/`0deg`; erro se combinado com `dx`/`dy` legado; `Ratio` → erro de scope-out explícito; tipo errado → `expected length/angle, found {type}`; `start` admitido só se `(0,0)` (mesmo scope-out de P739B).
- Braço `end`: `length`/`angle` presentes são **ignorados** (paridade medida).

## Passo 3 — Validação (depois)

### Geometria (`mutool trace`, deltas moveto→lineto)

| Fonte | Cristalino (depois) | Vanilla | |
|---|---|---|---|
| `#line(length: 3cm)` | (85.038, 0) | (85.039, 0) | ✓ |
| `#line(length: 3cm, angle: 30deg)` | (73.645, 42.519) | (73.646, 42.520) | ✓ |
| `#line(length: 3cm, end: (1cm, 1cm))` | (28.346, 28.346) — length ignorado | (28.346, 28.346) | ✓ |

(3cm = 85.04pt; cos30·85.04 = 73.65, sin30·85.04 = 42.52; 1cm = 28.346pt.)

### Testes novos (escritos primeiro; os 4 falharam antes)

`01_core/src/engine/stdlib/mod.rs`:
- `p804_line_length_sozinho` — `length: 3cm` → dx=85.04, dy=0.
- `p804_line_length_com_angle` — `length: 4cm, angle: 90deg` → dx≈0, dy=113.39.
- `p804_line_length_ignorado_com_end` — `length` + `end` → delta do `end`, length ignorado.
- `p804_line_length_nao_combinavel_com_dx` — `length` + `dx` → erro "não pode ser combinado".

### Suíte `typst-core`

- ANTES (fim de P803): **4324** passed + 1 ignored (total 4325).
- DEPOIS: `cargo test -p typst-core --lib` → **4328** passed; 0 failed; 1 ignored (total 4329 = +4 testes novos ✓).

Lint: `crystalline-lint .` → exit 0, zero violações.
