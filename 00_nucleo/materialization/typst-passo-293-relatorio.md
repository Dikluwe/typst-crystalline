# Relatório — Passo 293 (`P-curve-geometry`)

**Data**: 2026-05-19
**Spec**: `00_nucleo/materialization/typst-passo-293.md`
**Diagnóstico Fase A**: `00_nucleo/diagnosticos/diagnostico-curve-geometry-passo-293.md`
**Tipo**: ortogonal (1º pós-série cumulativa P288–P292)
**Baseline P292**: 2793 testes  →  **P293**: 2802 testes (Δ = +9)
**Hash `export.rs`**: `66cb8ac3` preservado bit-exact (10º passo consecutivo: P282→P293)
**ADRs meta novas**: 0

---

## §1 — Sumário executivo

P293 activa o variant `PathItem::CubicTo` (existente desde P277, inerte
até hoje) adicionando o constructor stdlib `native_curve`. Emit PDF do
operador `c` (cubic Bézier) já estava materializado em
`03_infra/src/export.rs` em três sítios (linhas 2375 / 2457 / 2629);
P293 fornece apenas o caminho de entrada.

**Resultado funcional**: pela primeira vez no projecto, `#curve(...)`
em Typst gera output PDF com curvas Bézier cúbicas reais.

**Resultado metodológico**: a Fase A inaugurou a secção **A.0.0
(clarificação de scope)** porque a spec partiu de referência
arquitectural ambígua ("ADR-0078 sub-fase b") que não cobre curvas. A
inspecção literal de `geometry.rs` + `shapes.rs` + `export.rs` +
vanilla descobriu a hipótese **H6** (activação posterior de variant
inerte) que não constava em H1–H5 da spec — refutação **significativa
genuína**, não estructuralmente forçada.

---

## §2 — Fase A (síntese)

| Secção | Veredicto |
|---|---|
| A.0.0 (inaugural) | H6 descoberta empiricamente; refutação genuína de H1–H5 |
| A.0 (ADR-0098) | ✅ vigente — emit consume via `FrameItem::Shape` capture |
| A.1 inventário | 5 stdlib shape funções pré-P293 / 0 constroem `CubicTo` |
| A.2 decisão | `native_curve` aceita variadic `(kind, ...)` tuples |
| A.3 integração | Zero novos variants em `Content` / `ShapeKind` / `PathItem` |
| A.4 emit | Hash `export.rs` preservado — emit já existe |
| A.5 bugs latentes | 5 cenários Bézier fronteira verificados; nenhum bug |
| A.5' anti-reflexão | N=3 cumulativo (P291+P292+P293); 4 elementos novos identificados |

Detalhe completo: `00_nucleo/diagnosticos/diagnostico-curve-geometry-passo-293.md`.

---

## §3 — Materialização

### §3.1 — `01_core/src/rules/stdlib/shapes.rs` (+~110 LOC)

Função `native_curve` adicionada após `native_polygon`:

```rust
pub fn native_curve(
    _ctx: &mut EvalContext,
    args: &Args,
    span: Span,
    file_id: FileId,
) -> SourceResult<Value> {
    let mut path_items: Vec<PathItem> = Vec::new();
    for (i, val) in args.items.iter().enumerate() {
        let arr = match val {
            Value::Array(a) if !a.is_empty() => a,
            _ => return Err(/* "argumento N: esperava array não-vazio" */),
        };
        let kind = match &arr[0] {
            Value::Str(s) => s.as_str(),
            _ => return Err(/* "argumento N: 1º elemento deve ser string" */),
        };
        match kind {
            "move"     => path_items.push(PathItem::MoveTo(point_from(&arr[1])?)),
            "line"     => path_items.push(PathItem::LineTo(point_from(&arr[1])?)),
            "cubic"    => path_items.push(PathItem::CubicTo(
                              point_from(&arr[1])?,
                              point_from(&arr[2])?,
                              point_from(&arr[3])?,
                          )),
            "close"    => path_items.push(PathItem::ClosePath),
            "quadratic" => return Err(/* "scope-out P293 ADR-0054 graded" */),
            other => return Err(/* "kind desconhecido: {other}" */),
        }
    }
    // bbox analítica via path_bbox (P277), construir Content::Shape com
    // ShapeKind::Path(path_items) + fill/stroke nomeados.
    Ok(Value::Content(Content::Shape { ... }))
}
```

Pontos arquitecturais:

- **Sintaxe cristalino** = tuples `("kind", ...)`; vanilla usa
  `curve.move(...)` (scope methods, fora do escopo P293).
- **`quadratic` é scope-out explícito** com erro informativo —
  `PathItem` não tem variant `QuadraticTo` (ADR-0054 graded). Frente
  pendente registada.
- **`path_bbox` (P277)** reutilizado sem alteração para AABB
  analítica das cúbicas.

### §3.2 — `01_core/src/rules/stdlib/mod.rs`

Linha 49: `native_curve` adicionado ao re-export do módulo `shapes`.

### §3.3 — `01_core/src/rules/eval/mod.rs`

`make_stdlib` regista `curve` após `polygon`:

```rust
scope.define("curve",   Value::Func(Func::native("curve",   native_curve)));
```

### §3.4 — Zero alterações em L3 / L2

- `03_infra/src/export.rs`: hash `66cb8ac3` preservado bit-exact.
- `02_shell/`: intacto.

---

## §4 — Testes

### §4.1 — `01_core/src/rules/stdlib/mod.rs` (+8 testes L1)

| Teste | Verifica |
|---|---|
| `p293_curve_move_line_basico` | Construção elementar move + line + close |
| **`p293_curve_cubic_activa_pathitem_cubicto`** | **Activação inaugural de `PathItem::CubicTo` via stdlib** |
| `p293_curve_cubic_preserva_control_points` | `c1` / `c2` / `end` preservados sem reordenação |
| `p293_curve_quadratic_scope_out_retorna_err` | `"quadratic"` retorna `Err` (ADR-0054 graded) |
| `p293_curve_kind_desconhecido_retorna_err` | Robustez de input |
| `p293_curve_vazia_retorna_err` | Robustez de input |
| `p293_curve_bbox_analitica_via_path_bbox_p277` | Reuso correcto do P277 |
| `p293_curve_named_fill_e_stroke` | Argumentos nomeados funcionam |

### §4.2 — `03_infra/src/export.rs` (+1 teste L3)

`p293_curve_cubic_emite_pdf_c_operator` — constrói
`Content::Shape { kind: ShapeKind::Path(vec![MoveTo, CubicTo, ...]) }`,
faz layout + export, e verifica que o output contém o operador PDF
`c` (cubic Bézier) e `m` (moveto).

---

## §5 — Validação

### §5.1 — `cargo test --workspace`

```
test result: ok. 2308 passed; 0 failed; 0 ignored
test result: ok.  447 passed; 0 failed; 6 ignored
test result: ok.   24 passed; 0 failed; 0 ignored
test result: ok.    2 passed; 0 failed; 0 ignored
test result: ok.   21 passed; 0 failed; 0 ignored
                  -----
                  2802 passed total
```

Baseline P292 = 2793; delta = +9 = 8 (L1) + 1 (L3). Esperado: ✓.

### §5.2 — `crystalline-lint .`

```
✓ No violations found
```

### §5.3 — Hash export.rs

```
//! @prompt-hash 66cb8ac3
```

Preservado bit-exact pelo **10º passo consecutivo** (P282, P285, P286,
P287, P288, P289, P290, P291, P292, P293). ADR-0098 honrada.

---

## §6 — Estado de hashes L0

| Ficheiro L0 | Antes P293 | Pós P293 |
|---|---|---|
| `style.md` | inalterado | inalterado (P293 ortogonal a Style) |
| `content.md` | inalterado | inalterado |
| `stdlib.md` | `21ade03a` | `21ade03a` (L0 não enumera funções individuais — política única para toda a stdlib; sem drift) |

Nota: `stdlib.md` documenta apenas a política da camada stdlib (regras
de I/O via `ctx.world`, padrão de erro, etc.), não cada `native_*`.
`native_polygon` já existente também não consta nominalmente.
`native_curve` herda o mesmo regime, portanto **sem alteração de
hash necessária**. `crystalline-lint --fix-hashes .` confirmou
"Nothing to fix".

---

## §7 — Padrões metodológicos

### §7.1 — §8.7' "A.0.0 template" (**inaugural N=1**)

P293 inaugura o template **A.0.0 (clarificação de scope empírica)**
para passos futuros cuja spec parta de referência arquitectural
ambígua. Sequência:

1. Inspeccionar literalmente os ficheiros mencionados na spec.
2. Verificar se as hipóteses H1–HN da spec cobrem o estado real.
3. Se não — registar a hipótese genuína (Hk+1) e prosseguir com
   scope clarificado empiricamente.
4. Documentar a refutação como "significativa genuína" (vs
   "estructuralmente forçada").

Padrão entra em **N=1 emergente**. Promoção a ADR aguarda N≥3.

### §7.2 — §8.6 "A.5' anti-reflexão" (N=3 cumulativo)

Cumulativo P291 + P292 + P293. P293 traz 4 elementos novos
identificados em A.5'.3 (A.0.0 em si, H6, ortogonalidade vs
cumulativo, refutação genuína). **Não promovido** em P293 — uma ADR
meta por passo (P273.17 §0) e §8.7' inaugural tem prioridade.

### §7.3 — §8.3 "refutação pragmática" (candidato N=6 refutado)

P293 forneceria N=6 (refutação genuína de spec). **Decisão de não
promover** documentada em A.4.2: o ganho metodológico já está
capturado no template §8.7' inaugural; promover §8.3 simultaneamente
violaria a regra "uma meta por passo". Anti-padrão de
over-formalização honrado.

### §7.4 — ADR-0098 (10º passo consecutivo)

Hash `export.rs` preservado. Invariante "feature graded via
`FrameItem::Shape` capture, sem alteração de emit" robusta sobre 10
features distintas.

### §7.5 — ADR-0099 (9ª reaplicação)

Padrão "activação posterior de feature graded" reaplicado a
`PathItem::CubicTo` (variant em enum diferente de `Style`). Confirma
que o padrão é **arquitectural**, não restrito a `Style`.

---

## §8 — Cobertura vanilla vs cristalino

`00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:

- **Linha 194** (`curve.cubic` / cubic Bézier): `implementado⁺` →
  `implementado` (degrau qualitativo: aproximação P277 → entrada
  stdlib funcional).
- **Nota ⁷⁹** (~110 LOC) adicionada com a história completa:
  inauguração de A.0.0, descoberta H6, 6.º paradigma consumer,
  scope-out `quadratic`.

---

## §9 — Frentes pendentes pós-P293

| Frente | Nota |
|---|---|
| `native_quadratic` / `PathItem::QuadraticTo` | scope-out explícito P293; passo dedicado futuro |
| `curve.move` / `curve.cubic` scope methods (sintaxe vanilla fiel) | Bloqueado por scope methods em stdlib — frente independente |
| `native_path` (parser SVG path string) | Frente vanilla separada; não bloqueia curve |
| `PathItem` em `Content::Shape` via parser literal SVG | Frente vanilla separada |

---

## §10 — Decisão sobre P294

P294 pode ser qualquer frente ortogonal. Candidatos rankeados por
A.0.0 inicial recomendada:

1. **Extensão P293**: `PathItem::QuadraticTo` + `native_quadratic` —
   replica padrão P293 (caminho de entrada + emit Bézier quadrático
   `v` ou `y`).
2. **Frente nova**: math-accent-cancel ou footnote-cluster (P292 §9.2
   ranking, ainda válidos).

Decisão fica para o operador humano. **P293 não dita P294**.

---

## §11 — Fecho

P293 fechado com:

- **+9 testes** (8 L1 + 1 L3) — todos verdes.
- **0 violations** no `crystalline-lint`.
- **Hash `export.rs` preservado** (10º passo consecutivo).
- **Padrão §8.7' (A.0.0 template) inaugurado** N=1.
- **0 ADRs meta novas** — anti-padrão over-formalização honrado.
- **6.º paradigma consumer arquitecturalmente distinto** registado
  em A.1.6 (consecutive desde P288).

**MARCO**: 1.º passo ortogonal pós-série cumulativa P288–P292.
PathItem::CubicTo deixa de ser inerte após estar no código por 16
passos (introduzido em P277).
