---

# P513 — Sonda A.0 e Materialização de Curve Elements

> **Passo:** 513
> **Data:** 2026-06-30
> **Foco:** Verificar empiricamente se os 5 elementos de curva existem no cristalino; se ausentes, materializar `curve.move`, `curve.line`, `curve.cubic`, `curve.quad`, `curve.close`. Não declarar conclusão — medir antes de decidir.
> **Tipo:** Sonda A.0 + Implementação M-size (se gate duro satisfeito).
> **Tamanho:** M (~45 min de sonda + implementação + validação).
> **ADR-0107 ACEITE** — paridade é linguagem, não mecânica.
> **ADR-0108 ACEITE** — medir antes de decidir; língua vs mecânica explícita.
> **ADR-0109 ACEITE** — atomização de código.
> **ADR-0114 ACEITE** — sonda A.0 antes da spec; gate duro.
> **Dependências:** P512 (grid/table lines fechados), P508 (diagnóstico que identificou curve elements como ausentes).

---

## 1. Sonda A.0 — Verificação de Substrato

### 1.1 Comandos de Verificação

```bash
cd /home/dikluwe/Documentos/Antigravity/typst-crystalline

# Sonda 1: curve.move
echo '#curve.move((0,0))' > /tmp/sonda_curve_move.typ
cargo run --release -p typst-wiring -- compile /tmp/sonda_curve_move.typ /tmp/out.pdf 2>&1

# Sonda 2: curve.line
echo '#curve.line((100,0))' > /tmp/sonda_curve_line.typ
cargo run --release -p typst-wiring -- compile /tmp/sonda_curve_line.typ /tmp/out.pdf 2>&1

# Sonda 3: curve.cubic
echo '#curve.cubic((0,0), (50,50), (100,0))' > /tmp/sonda_curve_cubic.typ
cargo run --release -p typst-wiring -- compile /tmp/sonda_curve_cubic.typ /tmp/out.pdf 2>&1

# Sonda 4: curve.quad
echo '#curve.quad((0,0), (50,50))' > /tmp/sonda_curve_quad.typ
cargo run --release -p typst-wiring -- compile /tmp/sonda_curve_quad.typ /tmp/out.pdf 2>&1

# Sonda 5: curve.close
echo '#curve.close()' > /tmp/sonda_curve_close.typ
cargo run --release -p typst-wiring -- compile /tmp/sonda_curve_close.typ /tmp/out.pdf 2>&1

# Sonda 6: Substrato (variants, construtores, layout)
rg -n "CurveMove\|CurveLine\|CurveCubic\|CurveQuad\|CurveClose" src/entities/content.rs --type rs
rg -n "native_curve" src/stdlib/ --type rs
rg -n "CurveMove\|CurveLine\|CurveCubic\|CurveQuad\|CurveClose" src/rules/layout/ --type rs
```

### 1.2 Resultado Esperado da Sonda

| Elemento | Parser | Variants | Construtores | Layout | Estado |
|----------|--------|----------|--------------|--------|--------|
| `curve.move` | FAIL | AUSENTE | AUSENTE | AUSENTE | **Materializar** |
| `curve.line` | FAIL | AUSENTE | AUSENTE | AUSENTE | **Materializar** |
| `curve.cubic` | FAIL | AUSENTE | AUSENTE | AUSENTE | **Materializar** |
| `curve.quad` | FAIL | AUSENTE | AUSENTE | AUSENTE | **Materializar** |
| `curve.close` | FAIL | AUSENTE | AUSENTE | AUSENTE | **Materializar** |

**Hipótese:** 5/5 elementos estão completamente ausentes (gate duro satisfeito).

---

## 2. Classificação Língua vs Mecânica (ADR-0107, ADR-0108)

| Elemento | Sintaxe (língua) | Semântica (língua) | Morfologia (língua) | Mecânica (diverge) |
|----------|------------------|--------------------|---------------------|-------------------|
| `curve.move` | `curve.move((0,0))` | Mover caneta para ponto | Arg: posição (array de 2) | Estrutura interna do `CurveElem` |
| `curve.line` | `curve.line((100,0))` | Linha reta para ponto | Arg: posição (array de 2) | Estrutura interna do `CurveElem` |
| `curve.cubic` | `curve.cubic((0,0), (50,50), (100,0))` | Curva cúbica de Bézier | Args: 3 pontos | Estrutura interna do `CurveElem` |
| `curve.quad` | `curve.quad((0,0), (50,50))` | Curva quadrática de Bézier | Args: 2 pontos | Estrutura interna do `CurveElem` |
| `curve.close` | `curve.close()` | Fechar path | Sem args | Estrutura interna do `CurveElem` |

**Conclusão:** Todos são **língua** (sintaxe, semântica, morfologia). A mecânica (representação interna do path) diverge de propósito.

---

## 3. Implementação (se gate duro satisfeito)

### 3.1 — Adicionar `CurveElem` ao `Content`

**Arquivo alvo:** `src/entities/content.rs`

```rust
pub enum Content {
    // ... variants existentes
    Curve(CurveElem),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CurveElem {
    pub segments: Vec<CurveSegment>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CurveSegment {
    Move(Point),
    Line(Point),
    Cubic(Point, Point, Point), // control1, control2, end
    Quad(Point, Point),         // control, end
    Close,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Point {
    pub x: Length,
    pub y: Length,
}
```

### 3.2 — Registrar Construtores Nativos

**Arquivo alvo:** `src/stdlib/structural.rs` (ou módulo `curve`)

```rust
// Namespace anexado de curve (se curve for função com namespace)
// Ou: funções standalone no scope global

curve_namespace.define("move", Value::Func(native_curve_move));
curve_namespace.define("line", Value::Func(native_curve_line));
curve_namespace.define("cubic", Value::Func(native_curve_cubic));
curve_namespace.define("quad", Value::Func(native_curve_quad));
curve_namespace.define("close", Value::Func(native_curve_close));

fn native_curve_move(args: Args) -> SourceResult<Value> {
    let point = args.expect::<Point>("point")?;
    Ok(Value::Content(Content::Curve(CurveElem {
        segments: vec![CurveSegment::Move(point)],
    })))
}

fn native_curve_line(args: Args) -> SourceResult<Value> {
    let point = args.expect::<Point>("point")?;
    Ok(Value::Content(Content::Curve(CurveElem {
        segments: vec![CurveSegment::Line(point)],
    })))
}

fn native_curve_cubic(args: Args) -> SourceResult<Value> {
    let c1 = args.expect::<Point>("control1")?;
    let c2 = args.expect::<Point>("control2")?;
    let end = args.expect::<Point>("end")?;
    Ok(Value::Content(Content::Curve(CurveElem {
        segments: vec![CurveSegment::Cubic(c1, c2, end)],
    })))
}

fn native_curve_quad(args: Args) -> SourceResult<Value> {
    let control = args.expect::<Point>("control")?;
    let end = args.expect::<Point>("end")?;
    Ok(Value::Content(Content::Curve(CurveElem {
        segments: vec![CurveSegment::Quad(control, end)],
    })))
}

fn native_curve_close(_args: Args) -> SourceResult<Value> {
    Ok(Value::Content(Content::Curve(CurveElem {
        segments: vec![CurveSegment::Close],
    })))
}
```

**Nota:** `Point` precisa de parser/coerção de array de 2 elementos (`(0, 0)` → `Point { x: 0, y: 0 }`). Verificar se o cristalino já tem `Value::Point` ou similar.

### 3.3 — Layout de Curve

**Arquivo alvo:** `src/rules/layout/curve.rs` (novo) ou `src/rules/layout/mod.rs`

```rust
fn layout_curve(elem: &CurveElem, ctx: &mut LayoutContext) -> Vec<Frame> {
    let mut frame = Frame::new();
    let mut current_pos = Point::zero();

    for segment in &elem.segments {
        match segment {
            CurveSegment::Move(p) => {
                current_pos = p.clone();
            }
            CurveSegment::Line(p) => {
                frame.add_line(current_pos.clone(), p.clone(), Stroke::default());
                current_pos = p.clone();
            }
            CurveSegment::Cubic(c1, c2, end) => {
                frame.add_cubic_bezier(current_pos.clone(), c1.clone(), c2.clone(), end.clone(), Stroke::default());
                current_pos = end.clone();
            }
            CurveSegment::Quad(control, end) => {
                frame.add_quadratic_bezier(current_pos.clone(), control.clone(), end.clone(), Stroke::default());
                current_pos = end.clone();
            }
            CurveSegment::Close => {
                // Fechar path (não implementado se não houver path tracking)
            }
        }
    }

    vec![frame]
}
```

**Nota:** Se o cristalino não suporta Bézier curves no `Frame`, usar aproximação poligonal ou scope-out de renderização (mas aceitar sintaxe).

---

## 4. Validação

### 4.1 Testes Básicos

```typst
// test-curve-elements.typ
#curve.move((0,0))
#curve.line((100,0))
#curve.cubic((0,0), (50,50), (100,0))
#curve.quad((0,0), (50,50))
#curve.close()

// Path completo
#curve.move((0,0))
#curve.line((100,0))
#curve.line((100,100))
#curve.close()
```

### 4.2 Corpus P490+P500

```bash
for f in lab/parity/corpus/p490/*.typ lab/parity/corpus/p500/*.typ; do
  target/release/typst "$f" /tmp/out.pdf >/dev/null 2>&1     && echo "OK" || echo "FAIL: $(basename $f)"
done
```

**Esperado:** 37/37 OK (não-regressão).

---

## 5. Critério de Fecho

- [ ] Sonda A.0 executada (5 comandos + 3 comandos de substrato).
- [ ] Tabela de resultados preenchida (seção 1.2).
- [ ] Gate duro satisfeito (≥3 ausentes → spec de materialização).
- [ ] 513a implementado: `curve.move((0,0))` funciona.
- [ ] 513b implementado: `curve.line((100,0))` funciona.
- [ ] 513c implementado: `curve.cubic((0,0), (50,50), (100,0))` funciona.
- [ ] 513d implementado: `curve.quad((0,0), (50,50))` funciona.
- [ ] 513e implementado: `curve.close()` funciona.
- [ ] 5 testes unitários novos passam.
- [ ] Corpus P490+P500: 37/37 OK (não-regressão).
- [ ] PANICs: 0 (preservado).
- [ ] Documentação atualizada (L0 + L1 + L3).
- [ ] Sentinela `p513_curve_elements` adicionada.
- [ ] `00_nucleo/diagnosticos/paridade-funcional-p513.md` produzido.

---

## 6. Próximo Passo (P514)

Com P513 fechado, **todas as brechas de linguagem identificadas no P508 estarão resolvidas**:

- ✅ Math styles (12 funções) — P510
- ✅ Math elements granulares (7 elementos) — P511
- ✅ Grid/Table HLine/VLine (4 elementos) — P512
- ✅ Curve elements (5 elementos) — P513
- ✅ Stdlib core (str, dict, calc, image, raw, etc.) — P509
- ✅ Runtime state (state, counter, context) — P506
- ✅ Selectors, args nomeados, field access — P494-P496

**Brecha restante:** Trilha 5 (fontdb/shaping) — paridade de produção, não de linguagem.

**Recomendação:** P514 = **Relatório Final de Paridade de Linguagem** — documentar que o cristalino atinge paridade de linguagem completa com Typst 0.15.0, e que a única brecha restante é de produção (shaping, PDF com fontes reais).

Alternativa: P514 = **Iniciar Trilha 5 (fontdb)** — o único caminho para paridade de produção.

---

## A. Apêndice — Referência Rápida

```typst
// 513a: curve.move
#curve.move((0,0))

// 513b: curve.line
#curve.line((100,0))

// 513c: curve.cubic
#curve.cubic((0,0), (50,50), (100,0))

// 513d: curve.quad
#curve.quad((0,0), (50,50))

// 513e: curve.close
#curve.close()

// Path completo
#curve.move((0,0))
#curve.line((100,0))
#curve.line((100,100))
#curve.close()
```
