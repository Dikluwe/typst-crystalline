# Relatório — typst-passo-855: validação geométrica de `rect(radius:)`

**Data de execução:** 2026-07-23  
**Commit base:** `dfe3c2282ec0d6bd97d5834f00214e7c7f5d2d49`  
**Estado do working tree:** alterações não commitadas de passos anteriores (`git status --short` mostra modificações de P850–P854 e ficheiros `typst-passo-851` a `typst-passo-856` em `materialization/` e `diagnosticos/`). Nenhuma alteração de código foi introduzida por este passo.

---

## Resumo executivo

A geometria do canto arredondado produzida pelo exportador cristalino **bate com a do vanilla Typst 0.15.0** nos quatro casos de teste. Não foi necessária nenhuma correção no exportador nem na construção de `Corners<Length>`. O único problema operacional encontrado foi um binário `target/release/typst` desatualizado (compilado antes da integração de `radius:`); após recompilar `typst-wiring` a partir do código-fonte actual, os PDFs gerados são geometricamente equivalentes.

---

## Ferramentas e binários

| Binário | Path | Versão |
|---|---|---|
| Cristalino | `./target/release/typst` | `typst 0.15.0 (23ff5b5c)` |
| Vanilla | `./lab/typst-original/target/release/typst` | `typst 0.15.0 (969087ec)` |
| Extracção PDF | `mutool` (mupdf) | `/usr/bin/mutool` |
| Comparação visual | `mutool draw` + ImageMagick `compare` | `compare` via `/usr/bin/compare` |

**Nota:** o binário cristalino pertence ao package `typst-wiring` (`04_wiring/Cargo.toml`). `cargo build --release -p typst-shell` não o actualiza. Foi necessário:

```bash
cargo build --release -p typst-wiring
```

---

## Passo 1 — Sonda geométrica

### Casos de teste

Ficheiros gerados em `temp/p855/`:

| Caso | Código Typst |
|---|---|
| 1 | `#rect(radius: 10pt, width: 100pt, height: 60pt, fill: red)` |
| 2 | `#rect(radius: (top-left: 15pt, top-right: 5pt, bottom-left: 5pt, bottom-right: 15pt), width: 100pt, height: 60pt, fill: red)` |
| 3 | `#rect(radius: 0pt, width: 100pt, height: 60pt, fill: red)` |
| 4 | `#rect(radius: 40pt, width: 60pt, height: 80pt, fill: red)` |

Comandos usados para compilar e descodificar:

```bash
cd temp/p855
../../target/release/typst caseN.typ caseN_crystalline.pdf
../../lab/typst-original/target/release/typst compile caseN.typ caseN_vanilla.pdf
mutool clean -d caseN_crystalline.pdf caseN_crystalline_decoded.pdf
mutool clean -d caseN_vanilla.pdf caseN_vanilla_decoded.pdf
```

### Caso 1 — raio uniforme 10 pt

**Cristalino** (content stream object 4):

```text
80.867 771.023 m
160.867 771.023 l
166.390 771.023 170.867 766.546 170.867 761.023 c
170.867 721.023 l
170.867 715.500 166.390 711.023 160.867 711.023 c
80.867 711.023 l
75.344 711.023 70.867 715.500 70.867 721.023 c
70.867 761.023 l
70.867 766.546 75.344 771.023 80.867 771.023 c
h
f
```

- Bounding box: `x = 70.867 .. 170.867` → largura **100 pt**; `y = 711.023 .. 771.023` → altura **60 pt**.
- Cada canto tem raio **10 pt**.
- Pontos de controlo a `10 × 0.55228475 ≈ 5.523 pt` do canto (factor κ do arco de círculo).

**Vanilla** (content stream object 7):

```text
q 1 0 0 -1 70.86614 771.0236 cm
0 10 m
0 4.4771523 4.4771523 0 10 0 c
90 0 l
95.52285 0 100 4.4771523 100 10 c
100 50 l
100 55.522846 95.52285 60 90 60 c
10 60 l
4.4771523 60 0 55.522846 0 50 c
h
f
Q
```

- O vanilla aplica uma `cm` que translada e inverte Y; no espaço local a largura é **100 pt**, a altura **60 pt** e o raio **10 pt**.
- Os pontos de controlo estão a `10 × (1 − 0.55228475) ≈ 4.477 pt` do canto, o que geometricamente define a mesma curva Bézier cúbica (a distância entre o ponto final e o segundo controlo é `10 × 0.55228475 ≈ 5.523 pt`).

**Conclusão:** curvas idênticas; o cristalino desenha em coordenadas absolutas, o vanilla em coordenadas locais com `cm`. Forma equivalente.

### Caso 2 — raios diferentes por canto

**Cristalino:** `tl = 15`, `tr = 5`, `br = 15`, `bl = 5`.

```text
85.867 771.023 m
165.867 771.023 l
168.628 771.023 170.867 768.785 170.867 766.023 c
170.867 726.023 l
170.867 717.739 164.151 711.023 155.867 711.023 c
75.867 711.023 l
73.105 711.023 70.867 713.262 70.867 716.023 c
70.867 756.023 l
70.867 764.308 77.582 771.023 85.867 771.023 c
h
f
```

- Top-right: `tr = 5` → pontos de controlo a `5 × κ ≈ 2.761 pt`.
- Bottom-right: `br = 15` → pontos de controlo a `15 × κ ≈ 8.284 pt`.
- Bottom-left: `bl = 5` → pontos de controlo a `5 × κ ≈ 2.761 pt`.
- Top-left: `tl = 15` → pontos de controlo a `15 × κ ≈ 8.284 pt`.

**Vanilla:** `tl = 15`, `tr = 5`, `br = 15`, `bl = 5`.

```text
0 15 m
0 6.7157288 6.7157288 0 15 0 c
95 0 l
97.76142 0 100 2.2385762 100 5 c
100 45 l
100 53.28427 93.28427 60 85 60 c
5 60 l
2.2385762 60 0 57.761425 0 55 c
h
f
```

**Conclusão:** cada canto tem o raio correcto e os pontos de controlo seguem o factor κ. Geometria equivalente.

### Caso 3 — raio 0 pt

**Cristalino:**

```text
70.87 711.02 100.00 60.00 re
f
```

Usa o operador PDF `re` (rectangle), confirmando que `radius: 0pt` produz `ShapeKind::Rect` sem custo de curva.

**Vanilla:**

```text
0 0 m
100 0 l
100 60 l
0 60 l
h
f
```

Desenha o rectângulo como path manual de 4 linhas.

**Conclusão:** ambos produzem um rectângulo reto. O cristalino é mais compacto (`re`); o vanilla usa path explícito. Semântica idêntica.

### Caso 4 — raio maior que metade do lado menor

**Cristalino:** `width = 60`, `height = 80`, `radius = 40`. O clamp limita o raio a `min(w, h) / 2 = 30 pt`.

```text
100.867 771.023 m
100.867 771.023 l
117.435 771.023 130.867 757.592 130.867 741.023 c
130.867 721.023 l
130.867 704.455 117.435 691.023 100.867 691.023 c
100.867 691.023 l
84.298 691.023 70.867 704.455 70.867 721.023 c
70.867 741.023 l
70.867 757.592 84.298 771.023 100.867 771.023 c
h
f
```

- Largura 60 pt, altura 80 pt, raio efectivo 30 pt em todos os cantos.
- As edges superior e inferior têm comprimento zero (`100.867 771.023 m` seguido de `100.867 771.023 l`), o que é esperado quando `2 × radius = width`.

**Vanilla:** raio efectivo 30 pt.

```text
0 30 m
0 13.4314575 13.4314575 0 30 0 c
30 0 l
46.568542 0 60 13.4314575 60 30 c
60 50 l
60 66.56854 46.568542 80 30 80 c
30 80 l
13.4314575 80 0 66.56854 0 50 c
h
f
```

**Conclusão:** ambos fazem clamp ao raio máximo (metade da menor dimensão) e desenham a mesma forma alongada com cantos totalmente arredondados.

### Comparação visual

Renderização a 150 dpi com `mutool draw` e comparação RMSE com ImageMagick:

| Caso | RMSE |
|---|---|
| 1 | `2.68168 (4.09e-05)` |
| 2 | `3.09163 (4.72e-05)` |
| 3 | `32.0063 (4.88e-04)` |
| 4 | `4.08149 (6.23e-05)` |

As diferenças são da ordem de 10⁻⁴ a 10⁻⁵, atribuíveis a anti-aliasing e ao uso do operador `re` vs path manual no caso 3. Nenhuma divergência geométrica detectável.

---

## Passo 2 — Correções

Nenhuma correção foi necessária. O exportador em `03_infra/src/export/stream.rs::emit_rounded_rect_ops` já implementa:

- Clamp do raio a metade da menor dimensão (`max_r = (w.min(h)) / 2.0`).
- Factor κ = `0.552_284_749_831` para aproximação do arco de círculo.
- Emissão no sentido horário com cantos na ordem `top-left → top-right → bottom-right → bottom-left`.
- Optimização `radius: 0pt` → `ShapeKind::Rect` (já garantido em `01_core/src/entities/content.rs::shape_with_radius`).

---

## Passo 3 — Validação final

### Testes por crate

Comandos corridos e resultados:

```bash
cargo test -p typst-core
```

```text
test result: ok. 4649 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out
doc-tests typst_core: ok. 0 passed; 0 failed; 3 ignored; 0 measured; 0 filtered out
```

```bash
cargo test -p typst-shell
```

```text
test result: ok. 36 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

```bash
cargo test -p typst-infra
```

```text
test result: ok. 714 passed; 0 failed; 5 ignored; 0 measured; 0 filtered out
```

```bash
cargo test -p typst-wiring
```

```text
test result: ok. 31 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
Running tests/crystalline_lint.rs: ok. 2 passed; 0 failed; 0 ignored
```

### Teste completo do workspace

```bash
cargo test --workspace
```

Resultado global: **todos os testes passaram** (resumos acima).

### Linter arquitetural

```bash
crystalline-lint .
```

```text
warning: Prompt órfão: '00_nucleo/prompts/infra/package_version_resolution.md' não é referenciado por nenhum arquivo em L1–L4. [V7]
```

Zero violações relacionadas com este passo. O warning V7 é preexistente e não diz respeito a `rect(radius:)`.

---

## Conclusão

A validação geométrica de `rect(radius:)` está **fechada**. O exportador cristalino produz arcos de canto equivalentes aos do vanilla Typst 0.15.0 para raios uniformes, raios por canto, raio zero e raio clampado. A suíte de testes do workspace passa na totalidade.
