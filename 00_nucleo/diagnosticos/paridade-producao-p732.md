# Relatório P732 — `polygon`: fallback de stroke default + coordenadas `Length`

**Data:** 2026-07-13
**Passo:** `00_nucleo/materialization/typst-passo-732.md`
**ADRs em vigor:** ADR-0107 (paridade é com a linguagem), ADR-0108 (medir antes de decidir).
**Commit:** A PREENCHER
**Proveniência das medições (regra de proveniência):** commit base `8cd3e319b429e3d1883394fdc2ee860682cc6eaf` ("P731: preenche hash do commit no relatório"), branch `Tekt`, working tree com as alterações deste passo (`git diff HEAD --stat`: `00_nucleo/prompts/rules/stdlib/shapes.md`, `01_core/src/rules/stdlib/shapes.rs`, `01_core/src/rules/stdlib/mod.rs`, `01_core/src/rules/eval/tests.rs`). Medições vanilla: `lab/typst-original/target/release/typst`; medições cristalino: `./target/release/typst` (release build de 2026-07-13T22:40Z). Renders: `mutool draw -r 150`; contagem/diff de pixels: `python3 /tmp/pngdiff.py`.

---

## Sonda — medições antes de decidir (ADR-0108)

O passo partia da inspecção de P727: `native_polygon` (`shapes.rs:348-351`) não aplicava o fallback `Smart::Auto` do vanilla, sem caso medido com pixels. A sonda mediu **dois defeitos**, não um:

### Defeito 1 — coordenadas `Length` rejeitadas (bloqueava o caso do próprio passo)

O caso do passo, `#polygon((0pt, 0pt), (50pt, 0pt), (25pt, 40pt))`:

| | Vanilla | Cristalino (antes) |
|---|---|---|
| `(0pt, 0pt)` lengths | exit 0, **898 px não-brancos** | **exit 1** — "polygon(): argumento 0 não é uma coordenada válida" |
| `(0, 0)` inteiros | **exit 1** — "expected relative length, found integer" | exit 0 |
| `(0.0, 0.0)` floats | **exit 1** — "expected relative length, found float" | exit 0 |
| `(50%, 0pt)` ratio | exit 0 | exit 1 ("coordenada inválida") |

Causa localizada: `extract_coordinate` (`01_core/src/rules/stdlib/shapes.rs:265-274`) só aceitava `cast_float()` (Int/Float). O domínio estava **invertido** face ao vanilla: o cristalino aceitava exactamente o que o vanilla rejeita e rejeitava o que o vanilla exige (`Rel<Length>`).

### Defeito 2 — página em branco sem fill nem stroke (foco do passo, confirmado com pixels)

Com o defeito 1 a bloquear o caminho de lengths, a confirmação de pixels fez-se pelo caminho numérico (compilava no cristalino):

| Caso | Vanilla | Cristalino (antes) |
|---|---|---|
| Triângulo sem fill nem stroke | 898 px não-brancos | **0 px — página em branco** (exit 0) |
| `fill: red` | 4494 px | não compilava (defeito 1) |
| `stroke: blue` | 898 px | não compilava (defeito 1) |

Mesmo mecanismo de `curve` pré-P727: o path chegava ao PDF sem operador de pintura.

### Critério de fecho da sonda

- [x] Bug confirmado com contagem de pixels (0 vs 898), não só inspecção de código — via caminho numérico, porque o caminho de lengths estava bloqueado pelo defeito 1.
- [x] Caso "com fill, sem stroke" confirmado (vanilla 4494 px; cristalino bloqueado pelo defeito 1).

## L0 (Prompt)

`00_nucleo/prompts/rules/stdlib/shapes.md` — secção `native_polygon` reescrita: assinatura aceita `Array[Length, Length]` (paridade vanilla) além da interface legada `Array[Float, Float]`; fallback determinístico documentado (paridade `Smart::Auto`, `lab/typst-original/crates/typst-layout/src/shapes.rs:336-339`); scope-outs registados com as medições da sonda (ratio/em sem dimensão de referência em eval — precedente P513; o cristalino mantém a aceitação de Int/Float por partilhar `coord_component` com a interface documentada de `curve` — divergência registada em `achados-adiados-cetz.md`); marcos P727/P732 no cabeçalho. `crystalline-lint --fix-hashes .` → header de `shapes.rs` sincronizado (`33eb1231`); `crystalline-lint .` → **0 violations**.

## Implementação

`01_core/src/rules/stdlib/shapes.rs`:

1. `coord_component(val) -> Option<f64>` — novo helper: `Value::Length(l)` → `l.abs.to_pt()`; resto → `cast_float()` (comportamento legado intacto). `extract_coordinate` passa a usá-lo nas duas componentes. Sem efeito sobre `curve` (superset — só adiciona aceitação de `Length`).
2. `native_polygon` — fallback idêntico ao de `native_curve` (P727): `parsed_stroke` separado; se `fill.is_none() && parsed_stroke.is_none()` → stroke preta 1pt; senão `parsed_stroke` (com fill sem stroke → `None`).
3. Doc comments actualizados.

## Validação

- Fail-first confirmado: `cargo test -p typst-core p732` antes da implementação → **0 passed / 5 failed**.
- Depois: **5 passed, 0 failed** — `p732_polygon_aceita_coordenadas_length`, `p732_polygon_sem_cores_tem_stroke_preta_1pt`, `p732_polygon_com_fill_nao_tem_stroke_fallback`, `p732_polygon_com_stroke_explicito_preserva` (unitários em `stdlib/mod.rs`, padrão espelhado dos testes P727) + `p732_polygon_length_compila_e2e` (`eval/tests.rs`).
- `cargo test --workspace` — **4727 passed, 0 failed** (4032 + 631 + 33 + 2 + 27 + 2; 8 ignored pré-existentes). Pré-P732: 4722; +5 = os novos testes do passo.
- `crystalline-lint .` — **0 violations**.

### Diff de pixels final (150 dpi, `mutool draw` + `pngdiff.py`)

| Caso (ficheiro exacto do passo) | Vanilla (px não-brancos) | Cristalino depois | Diff |
|---|---|---|---|
| `#polygon((0pt, 0pt), (50pt, 0pt), (25pt, 40pt))` | 898 | 897 | **0.0834%** — anti-aliasing |
| `#polygon(fill: red, ...)` | 4494 | 4494 | **0.1358%** — anti-aliasing |
| `#polygon(stroke: blue, ...)` | 898 | 897 | **0.0797%** — anti-aliasing |

Os três casos deixam de erro/página em branco para paridade visual (mesmo nível dos diffs de P727: 0.048%).

## Critério de fecho do passo

- [x] Sonda mínima completa, com contagem de pixels — e um segundo defeito medido (coordenadas Length) que bloqueava o caso do próprio passo.
- [x] Implementado e testado: `polygon` sem fill nem stroke → stroke preto 1pt (diff 0.0834%); com fill sem stroke → sem stroke (diff 0.1358%); stroke explícito sem regressão (diff 0.0797%).
- [x] Sem regressão em `cargo test --workspace` (4727 passed, 0 failed).
- [x] `crystalline-lint .` limpo.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p732.md`, com hash do commit.
- [x] Item marcado como fechado em `achados-adiados-cetz.md` (e novo achado registado: aceitação de Int/Float que o vanilla rejeita + ratio scope-out).
