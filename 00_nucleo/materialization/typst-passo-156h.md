# Passo 156H — box inline container (Layout Fase 2 sub-passo 2)

**Série**: 156H (passo **substantivo escopo M**;
materialização Fase 2 Layout, segunda sub-fase). **Sexto
passo consecutivo** da sequência granular Layout
(P156C+D+E+F+G+H).
**Padrão**: P156A (historiograma) → P156B (diagnóstico
Layout) → P156C (pad+hide) → P156D (h+v) → P156E
(pagebreak) → P156F (skew via Transform) → P156G (block) →
**P156H (box)**.

**Precondição**: Passo 156G encerrado; ADR-0061 PROPOSTO
(Layout roadmap; mantém PROPOSTO até P156I per decisão
humana); 1250 tests; 61 ADRs; 14 DEBTs abertos; cobertura
Layout 61% (11/18 implementado puro pós-P156G); cobertura
user-facing total 58%.

**Numeração**: P156H segue P156G na convenção de letras
consecutivas. **Não conflita** com P157 (Model Fase 2 table
foundations).

**Natureza**: passo **substantivo escopo M** (1 feature
container inline com 5 atributos; ~15-22 testes adicionados
estimados; sem crates novas; sem ADRs novas; sem DEBTs novos
esperados).

**Decisão arquitectural P156H** (per resposta humana
2026-04-25 + reaplicação do padrão estabelecido em P156G):
**variant rico `Content::Box { body, width, height, inset,
baseline }`**.

Razão: P156G estabeleceu padrão "containers ricos preferem
variants explícitos quando atributos não são propriedades de
texto" (relatório P156G §15.4). Box reusa este padrão sem
nova decisão arquitectural — reduz custo de P156H e mantém
coerência com Block.

**Particularidade material face a P156G (Block)**:

| Aspecto | Block (P156G) | Box (P156H) |
|---------|---------------|-------------|
| Posicionamento | structural (força flush_line) | inline (cabe linha actual) |
| Largura default | full (page width) | content-based |
| Atributo único | `breakable: bool` | `baseline: Length` |
| Layouter | flush + inset + body | append inline + inset + baseline offset |
| Comum | body, width, height, inset | body, width, height, inset |

Box é qualitativamente diferente de Block na **mecânica de
posicionamento** (inline vs structural). Restantes 4
atributos (body, width, height, inset) replicam Block.

**Atributos vanilla scope-out** (per ADR-0054 graded):
- `outset` (idem Block scope-out).
- `fill`, `stroke`, `radius`, `clip` (renderer support
  refino futuro).
- `stroke-overhang` (avançado).

**ADRs aplicáveis**:
- **ADR-0026 + ADR-0026-R1**: Content enum aceita variants
  novos.
- **ADR-0033**: paridade funcional para box.
- **ADR-0036**: atomização — consumer explícito.
- **ADR-0037**: coesão por domínio — Layout permanece em
  `rules/layout/` e `rules/stdlib/layout.rs`.
- **ADR-0054**: perfil observacional graded — box cumprido
  com aproximação aceite (atributos avançados scope-out;
  baseline semantic real pode adiar).
- **ADR-0061** (PROPOSTO): plano de Layout Fase X. Sexta
  aplicação; segunda Fase 2.

---

## Contexto

P156G fechou block com decisão arquitectural variant rico
+ 5 fields (body, width, height, inset, breakable). P156G §15.5
recomendou para passos seguintes:

> 1. Inventário pré-código pode ser **mais curto**.
> 2. **Reusar pattern Block**: 5-6 fields explícitos,
>    cobertura arms mecânica, layouter inspirado em
>    Pad/Block.
> 3. Regression tests para containers existentes ainda
>    valem.

P156H aplica esta recomendação directamente.

**`box(body, ...)` em vanilla**:
- Body posicional opcional.
- Atributos numerosos:
  - `width: Smart<Rel<Length>>` (largura).
  - `height: Sizing` (altura).
  - `baseline: Rel<Length>` (ajuste vertical).
  - `inset: Sides<Rel<Length>>` (margem interna).
  - `outset: Sides<Rel<Length>>` (margem externa) —
    scope-out.
  - `fill: Option<Color>` — scope-out.
  - `stroke: Sides<Option<Stroke>>` — scope-out.
  - `radius: Corners<Rel<Length>>` — scope-out.
  - `clip: bool` — scope-out.
  - `stroke-overhang: Length` — scope-out.

**Atributos implementados em P156H**:
- `body` (opcional; fallback Empty).
- `width: Option<Length>` (per padrão `Smart<T>` →
  `Option<T>`).
- `height: Option<Length>` (idem).
- `inset: Sides<Length>` (uniforme em stdlib; per P156G).
- `baseline: Length` (default Length::ZERO).

**Hipóteses a confirmar empiricamente em 156H.1** (não
compromisso):

- **Mecânica inline em layouter actual**: como Strong/Emph
  (P22) appendam à linha actual sem flush. Reusar
  pattern.
- **`baseline` semantic**: layouter actual tem mecanismo
  de ajuste vertical mid-linha? Provável: não. Decisão
  default: armazenar baseline; aplicar offset apenas se
  trivial; caso contrário scope-out.
- **Largura default content-based**: layouter mede body
  para determinar largura quando `width: None`. Reusar
  `measure_content_constrained` (estabelecido P156C+).

---

## Objectivo

Ao fim do passo:

1. **Variant `Content::Box { body, width, height, inset,
   baseline }`** adicionado em
   `01_core/src/entities/content.rs`.

2. **Cobertura exaustiva de arms** em todos os ficheiros
   que pattern-match sobre `Content` (modelo P156G):
   - `entities/content.rs::is_empty()`.
   - `entities/content.rs::plain_text()`.
   - `entities/content.rs::PartialEq::eq`.
   - `entities/content.rs::map_content`.
   - `entities/content.rs::map_text`.
   - `rules/introspect.rs::materialize_time`.
   - `rules/introspect.rs::walk`.
   - `rules/layout/mod.rs::layout_content`.
   - `rules/layout/mod.rs::measure_content_constrained`.

3. **`native_box`** em
   `01_core/src/rules/stdlib/layout.rs` expondo
   `#box(body, width: ?, height: ?, inset: ?, baseline: ?)`.

4. **Layouter box inline**: append à linha actual sem
   flush; aplica width/height; aplica inset; aplica
   baseline offset se trivial (caso contrário scope-out
   com limitação registada).

5. **Testes** unit + eval (~15-22 testes adicionados
   estimados):
   - Construtor `Content::Box`.
   - `is_empty()`, `plain_text()`, `partial_eq`,
     `map_content`/`map_text` cobertura.
   - `eval_box` defaults (sem args nomeados).
   - `eval_box` com width/height/inset/baseline (cada um
     individual + caso composto).
   - `eval_box` rejeita atributos avançados scope-out
     (fill, stroke, etc.) com erro hard.
   - `eval_box` rejeita negativos (width/height/inset).
   - Layouter: box mantém-se inline (não força flush).
   - Layouter: box com width respeita largura.
   - **Regression tests** para Block/Pad/Hide
     (verificar que arms novos não quebraram containers
     existentes).

6. **L0 prompts** + hashes propagados:
   - `00_nucleo/prompts/entities/content.md` ganha secção
     "Variant `Content::Box` — Passo 156H".
   - Hash `entities/content.rs` recomputado (era
     `5702d2e3` pós-P156G).
   - Headers `@updated`: data execução.

7. **Inventário 148 actualizado**:
   - Tabela A.5 Layout: linha `box` ausente →
     `implementado` (com anotação sobre atributos parciais
     per ADR-0054 graded).
   - Cobertura Layout: 11/18 → **12/18 = 67%**.
   - Tabela A linha "Layout": `11/0/3/4/0=18` →
     `12/0/3/3/0=18`.
   - Total user-facing: 58% → **~59%**.
   - Tabela B Content variants: 49 → **50**.
   - §7 entrada 7: actualizar progresso Layout (P156H
     cumprido; restantes 3 entradas Layout: stack, repeat,
     columns/colbreak).

8. **README dos ADRs actualizado**:
   - Tabela "Estado por ADR": linha ADR-0061 mantém-se
     PROPOSTO.
   - Distribuição inalterada.
   - Total inalterado (61 ADRs).
   - Entrada nova em "Passos-chave da história dos ADRs"
     para P156H.

9. **ADR-0061 NÃO actualizada** neste passo (per decisão
   humana). Mantém-se PROPOSTO.

10. **Sem DEBTs criados/fechados** (esperado).

11. **Relatório do passo** em
    `00_nucleo/materialization/typst-passo-156h-relatorio.md`
    com §análise de risco modelo P156F/G.

Este passo **não**:

- Toca outras features Layout além de box.
- Implementa atributos avançados scope-out (fill, stroke,
  radius, clip, outset).
- Implementa baseline semantic real se layouter actual não
  suporta (scope-out documentado).
- Adiciona show rules.
- Toca série paridade.
- Modifica ADR-0061.

---

## Decisões já tomadas

1. **Variant rico** per padrão estabelecido em P156G.
   Sem nova decisão arquitectural neste passo.

2. **5 fields**: body, width, height, inset, baseline.

3. **Granularidade**: 1 container num passo. Escopo M
   (entre P156C-E S/S+ e P156G M+).

4. **Localização canónica**: `01_core/src/rules/stdlib/layout.rs`.

5. **Assinatura natives**: 5-param canónica.

6. **Atributos avançados scope-out**: outset, fill, stroke,
   radius, clip, stroke-overhang. Per ADR-0054 graded.

7. **Inventário 156H.1 curto** (decisão arquitectural já
   estabelecida; só verificar especificidades de box).

8. **Tests adicionados**: alvo 15-22.

9. **ADR-0061 NÃO anotada**.

10. **Show rules adiadas**.

11. **Risco de regressão baixo** (aditivo); regression tests
    explícitos para Block/Pad/Hide.

## Decisões diferidas (resolvidas neste passo)

12. **`baseline` semantic real**: layouter actual pode não
    ter mecânica de ajuste vertical mid-linha. Decisão
    default: armazenar como atributo; aplicar offset
    apenas se trivial; caso contrário scope-out documentado
    análogo a `breakable: false` em P156G.

13. **`width` content-based default**: quando `None`,
    layouter mede body. Usar `measure_content_constrained`.

14. **`width` aplicado em layouter**: per Block scope-out
    em P156G ("Layouter actual sem mecânica de largura
    útil por arm"), mesmo scope-out aplica-se a Box.
    Refino futuro com refactor multi-region (DEBT-56).

15. **`inset` uniforme apenas**: per padrão P156C/G.

16. **`inset.right` scope-out** em layout (idem Block
    P156G).

17. **Box aninhado em Block**: caso normal (e.g. Block
    contendo texto com Box inline). Suportado
    estruturalmente; tests.

18. **Box em conteúdo de blockquote ou lista**: análogo;
    suportado estruturalmente.

19. **Atributos negativos**: rejeitar (consistente
    Pad/h/v/Block).

20. **Baseline negativo**: aceitável (move box para cima);
    por agora aceitar mas registar scope-out se layouter
    não aplica.

---

## Escopo

**Dentro**:

- Modificação de `01_core/src/entities/content.rs`
  (variant novo + arms cobertura).
- Modificação de `01_core/src/rules/introspect.rs`.
- Modificação de `01_core/src/rules/layout/mod.rs`.
- Modificação de `01_core/src/rules/stdlib/layout.rs`
  (`native_box`).
- Modificação de `01_core/src/rules/stdlib/mod.rs`
  (re-export).
- Modificação de `01_core/src/rules/eval/mod.rs`
  (registo).
- Tests novos.
- L0 prompts + hashes.
- Inventário 148 + README ADRs.
- Relatório do passo.

**Fora**:

- Modificação de outros ficheiros L1/L2/L3/L4 não-listados.
- Implementação de outras features Layout.
- Implementação de atributos avançados (fill/stroke/radius/
  clip/outset).
- Show rules.
- Crates externas.
- ADRs novas.
- DEBTs novos.
- Modificação de ADR-0061 (anotação cumulativa após P156I).
- Modificação de ADR-0060.
- Trabalho em `lab/parity/`.

---

## Sub-passos

### 156H.1 — Inventário curto pré-materialização

**Reduzido face a P156G** (decisão arquitectural já
estabelecida).

**A.1.1 — Verificar pré-condições**:

```bash
view 01_core/src/entities/content.rs   # confirmar 49 variants pós-P156G
grep -nE "^pub enum Content" 01_core/src/entities/content.rs
grep -nE "Box|inline" 01_core/src/entities/content.rs
grep -nE "fn native_box" 01_core/src/rules/stdlib/
```

Confirmar:
- Content tem 49 variants.
- Sem `Content::Box`.
- Sem `native_box` em stdlib.

**A.1.2 — Mecânica inline no layouter actual**:

```bash
grep -nE "Strong|Emph" 01_core/src/rules/layout/mod.rs
view 01_core/src/rules/layout/mod.rs   # arms para Strong/Emph
```

Documentar:
- Como Strong/Emph appendam à linha actual sem flush.
- Mecanismo de medição mid-linha.
- Que método/função usar para Box (provavelmente similar a
  Strong/Emph mas com inset/baseline).

**A.1.3 — `measure_content_constrained` para Box**:

```bash
grep -nE "Content::Strong|Content::Emph" \
  01_core/src/rules/layout/mod.rs   # arm measure
```

Confirmar:
- Como medir body de Box quando `width: None`.

**A.1.4 — Baseline mecanismo**:

```bash
grep -nE "baseline|cursor_y" 01_core/src/rules/layout/cursor.rs
view 01_core/src/rules/layout/cursor.rs
```

Determinar:
- Cursor tem mecânica de baseline-relative offset?
- Se sim, reusar.
- Se não, scope-out baseline semantic em P156H.

### 156H.2 — Adicionar variant Content::Box

Edição de `01_core/src/entities/content.rs`:

```rust
pub enum Content {
    // ... 49 variants existentes (Text, Strong, ..., Block)
    Box {
        body:     Box<Content>,
        width:    Option<Length>,
        height:   Option<Length>,
        inset:    Sides<Length>,
        baseline: Length,
    },
}
```

**Variant count**: 49 → **50** (+1).

### 156H.3 — Cobertura exaustiva de arms

Modelo P156G aplicado:

| Função | Tratamento Box |
|--------|----------------|
| `Content::is_empty()` | proxy `body.is_empty()` |
| `Content::plain_text()` | recurse body (transparente) |
| `PartialEq::eq` | comparação 5-fields |
| `Content::map_content` | recurse body; preserva atributos Copy |
| `Content::map_text` | idem |
| `materialize_time` | recurse body; preserva atributos |
| `walk` | walk body |
| `layout_content` | append inline + inset + baseline (scope-out se complexo) |
| `measure_content_constrained` | dimensões: width respeitado se Some, inset adiciona, height força mínimo |

### 156H.4 — `native_box`

Em `01_core/src/rules/stdlib/layout.rs`:

```rust
pub fn native_box(_ctx, args, _world, _file, _fig)
    -> SourceResult<Value>
{
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s))     => Content::text(s.as_str()),
        Some(other) => return Err(...),
        None => Content::Empty,  // body opcional
    };

    let mut width:    Option<Length> = None;
    let mut height:   Option<Length> = None;
    let mut inset_uniform: Option<Length> = None;
    let mut baseline: Length = Length::ZERO;

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "width"    => width    = Some(extract_length(value)?),
            "height"   => height   = Some(extract_length(value)?),
            "inset"    => inset_uniform = Some(extract_length(value)?),
            "baseline" => baseline = extract_length(value)?,
            // atributos avançados scope-out:
            "outset" | "fill" | "stroke" | "radius" | "clip" | "stroke-overhang" => {
                return Err(invalid_argument(format!(
                    "{:?} é atributo avançado scope-out per ADR-0054 graded; \
                     refino futuro", key)));
            }
            other => return Err(unexpected_named(other)),
        }
    }

    // Validação: width/height/inset negativos rejeitados.
    // baseline aceita negativo (move box para cima).

    let inset = match inset_uniform {
        Some(l) => Sides::uniform(l),
        None    => Sides::uniform(Length::ZERO),
    };

    Ok(Value::Content(Content::Box {
        body: Box::new(body),
        width, height, inset, baseline,
    }))
}
```

Registo em `make_stdlib`:

```rust
scope.define("box", Value::Func(Func::native("box", native_box)));
```

Re-export em `stdlib/mod.rs`:

```rust
pub use crate::rules::stdlib::layout::{
    native_align, native_block, native_box, native_grid, ...
};
```

Stdlib funcs: 39 → **40** (+1).

### 156H.5 — Layouter box

Em `01_core/src/rules/layout/mod.rs::layout_content`:

```rust
match content {
    Content::Box { body, width, height, inset, baseline } => {
        let font = self.font_size_pt.val();
        let inset_left   = inset.left.resolve_pt(font);
        let inset_top    = inset.top.resolve_pt(font);
        let inset_bottom = inset.bottom.resolve_pt(font);
        // inset.right scope-out (idem Block).

        // Box é INLINE: NÃO força flush_line.

        // Aplica inset_left ao cursor.x antes do body.
        self.cursor_x += Pt(inset_left);

        // Aplica baseline offset se trivial (consoante 156H.1).
        // Se layouter sem mecânica baseline → scope-out;
        // armazenar baseline mas não aplicar.
        let _ = baseline;  // armazenado; aplicação defere

        // body é processado in-place na linha actual.
        let saved_y = self.cursor_y;
        self.cursor_y += Pt(inset_top);

        let _ = width;  // armazenado; refino futuro
        let _ = height; // armazenado; refino futuro

        self.layout_content(body);

        self.cursor_y += Pt(inset_bottom);
        // Restaura cursor.y para mesmo da linha (box é inline).
        // Excepto se height for Some e maior que box altura.
        self.cursor_y = saved_y;
    }
    // ... fallback
}
```

Em `measure_content_constrained` (modelo Block):

```rust
match content {
    Content::Box { body, width, height, inset, baseline: _ } => {
        let font = self.font_size_pt.val();
        let inset_l = inset.left.resolve_pt(font);
        let inset_r = inset.right.resolve_pt(font);
        let inset_t = inset.top.resolve_pt(font);
        let inset_b = inset.bottom.resolve_pt(font);

        let body_max = match width {
            Some(w) => w.resolve_pt(font).min(max_width - inset_l - inset_r),
            None    => (max_width - inset_l - inset_r).max(0.0),
        };

        let (bw, bh) = self.measure_content_constrained(body, body_max);
        let total_w = bw + inset_l + inset_r;
        let body_h_with_inset = bh + inset_t + inset_b;
        let total_h = match height {
            Some(h) => h.resolve_pt(font).max(body_h_with_inset),
            None    => body_h_with_inset,
        };
        (total_w, total_h)
    }
    // ... fallback
}
```

### 156H.6 — Tests adicionados (alvo 15-22)

| Ficheiro | Testes |
|----------|--------|
| `entities/content.rs::tests` | (1) box_constructor_default; (2) box_constructor_explicit_atributos; (3) box_is_empty_proxy_para_body; (4) box_plain_text_recurse; (5) box_partial_eq; (6) box_map_text_recurse |
| `stdlib/mod.rs::tests` | (7) `native_box` defaults sem args; (8) `native_box` body opcional aceita Empty; (9) `native_box` com width; (10) `native_box` com height; (11) `native_box` com inset uniforme; (12) `native_box` com baseline; (13) `native_box` com baseline negativo aceito; (14) `native_box` combina atributos; (15) `native_box` rejeita fill (atributo avançado); (16) `native_box` rejeita width negativo; (17) `native_box` rejeita inset negativo; (18) `native_box` rejeita named arg desconhecido; (19) **regression** native_block + native_pad + native_hide continuam a funcionar |
| `layout/tests.rs` | (20) layout_box_mantem_inline_nao_forca_flush — texto antes + box + texto depois ficam na mesma linha; (21) layout_box_inset_top_aplica_avanco_y_temporario |

**Total**: ~21 tests novos. Tests cumulativos: **1250 →
~1271**.

### 156H.7 — L0 prompts + hashes

Editar `00_nucleo/prompts/entities/content.md`:

Adicionar secção "Variant `Content::Box` — Passo 156H
(ADR-0061 Fase 2, sub-passo 2)" após secção P156G Block:

```markdown
## Variant Box (Passo 156H)

`Content::Box { body, width, height, inset, baseline }`:
- container INLINE (não força flush_line);
- atributos vanilla: width, height, inset, baseline + 6
  scope-out (outset, fill, stroke, radius, clip,
  stroke-overhang);
- baseline semantic real adiada se layouter actual não
  suporta (consistente com `breakable: false` em Block).

Padrão arquitectural reaplicado de Block (P156G): variant
rico para containers cujos atributos não são propriedades
de texto. Distinção material face a Block: posicionamento
inline vs structural.
```

Recomputar hashes:

```bash
cd 01_core
cargo run --bin crystalline-lint -- --fix-hashes
```

Verificar:
- `entities/content.rs`: hash novo (era `5702d2e3` pós-
  P156G; será diferente pós-P156H).
- `entities/content.md`: hash propagado.

Headers `@updated`: data execução.

### 156H.8 — Inventário 148 actualizado

Em `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:

**Tabela A.5 Layout — linha reescrita**:
- `box(...)`: ausente → **implementado** ¹⁵ (Passo 156H;
  com anotação sobre atributos parciais).

**Tabela A linha "Layout"**: `11/0/3/4/0=18` →
**`12/0/3/3/0=18`**. Cobertura Layout: 61% → **67%**.

**Total user-facing**: `61/21/22/35/2=141` →
**`62/21/22/34/2=141`**. Cobertura: 58% → **~59%**.

**Tabela B Content variants**: 49 → **50**.

**§7 entrada 7**: actualizar progresso Layout Fase 2
(P156H cumprido; 3 entradas Layout restantes: stack,
repeat, columns/colbreak).

### 156H.9 — README ADRs actualizado

- Tabela "Estado por ADR": linha ADR-0061 mantém-se
  PROPOSTO.
- Total: 61 inalterado.
- Distribuição: PROPOSTO 11 inalterado.
- "Passos-chave" entrada nova:
  ```
  P156H: aplicação sexta de ADR-0061 — box inline container.
  Cobertura Layout 61% → 67%. Padrão Block reaplicado.
  ```

### 156H.10 — Relatório do passo

Ficheiro: `00_nucleo/materialization/typst-passo-156h-relatorio.md`.

Secções (modelo P156G):
1. Sumário executivo.
2. Inventário curto 156H.1.
3. Variant `Content::Box` — forma final + diff.
4. Cobertura exaustiva de arms.
5. `native_box` — assinatura + registo.
6. Layouter — diff (inline vs structural).
7. Tests adicionados (incluindo regression).
8. L0 prompts + hashes propagados.
9. Inventário 148 actualizado.
10. README ADRs actualizado.
11. Próximo passo (P156I = stack).
12. Limitações registadas.
13. Verificação final.
14. **Análise de risco de regressão** (modelo P156F/G).

---

## Verificação

1. ✅ `cargo build --workspace`: clean.
2. ✅ `cargo test --workspace --lib`: ~1271 passed
   (1250 → +21); zero falhas.
3. ✅ `crystalline-lint .`: zero violations.
4. ✅ Hashes propagados consistentes.
5. ✅ `Content::Box` variant em produção (49 → 50).
6. ✅ Stdlib `#box(...)` invocável (39 → 40 funcs).
7. ✅ Cobertura arms exaustiva (9 sítios actualizados).
8. ✅ Inventário 148 reflecte cobertura aumentada (61% →
   67%).
9. ✅ README ADRs entrada P156H.
10. ✅ Sem ADR criada / revogada / revisada.
11. ✅ Sem DEBT criado / fechado.
12. ✅ ADR-0061 inalterada.
13. ✅ ADR-0060 inalterada.
14. ✅ **Sem regressão** em containers existentes (Block,
    Pad, Hide) — regression test verifica.
15. ✅ Sem regressão geral.
16. ✅ Relatório do passo escrito (com §análise de risco).

---

## Critério de conclusão

| # | Critério | Estado |
|---|----------|--------|
| 1 | `Content::Box` compila + tests passam | ✅ |
| 2 | Stdlib `#box(...)` invocável com 4 atributos | ✅ |
| 3 | Layouter box mantém-se inline (não força flush) | ✅ |
| 4 | Atributos scope-out documentados em limitações | ✅ |
| 5 | Baseline armazenado (aplicação real consoante 156H.1) | ✅ |
| 6 | Inventário 148 reflecte cobertura 67% Layout | ✅ |
| 7 | Próximo passo (156I = stack) tem âncora | ✅ |
| 8 | Sem regressão | ✅ |
| 9 | §análise de risco no relatório | ✅ |
| 10 | Relatório do passo escrito | ✅ |

---

## O que pode sair errado

### Cenários gerais

- **Inventário 156H.1 revela que mecânica inline é
  diferente do esperado**: Strong/Emph podem usar pattern
  específico não-aplicável a Box. Investigar; ajustar
  layouter consoante. **Pausar e consultar** se discrepância
  radical.

- **`baseline` exige refactor não-trivial**: layouter actual
  pode não ter mecânica de offset vertical mid-linha.
  Decisão default: scope-out documentado como limitação
  (consistente com `breakable: false` em P156G).

- **Volume tests excede 22**: aceitável; ajustar relatório.

- **Volume tests inferior a 15**: investigar; box tem 5
  atributos + edge cases.

- **Box em conteúdo aninhado** (Block contendo Box, Box
  contendo Box, etc.): suportado estruturalmente; tests
  cobrem; case patológico raro mas válido.

### Cenários específicos a Box

- **Box width content-based default**: `measure_content_constrained`
  é chamado para body; se body tem dimensões dinâmicas
  (e.g. depende de page width), comportamento pode ser
  surpreendente. Aceitar; documentar.

- **Box dentro de Block com width**: cascata de larguras
  pode causar overflow. Aceitável; per ADR-0054 graded.

- **Baseline negativo**: aceitável (move box para cima).
  Se layouter actual não aplica, registar limitação.

- **Box vazio com `width: 100pt`**: caso patológico aceitável.
  Box ocupa 100pt de largura sem conteúdo. Test edge case.

---

## Notas operacionais

- **Padrão "passos granulares" — sexta aplicação
  consecutiva**. P156C+D+E+F+G+H. **N=6 aplicações** sem
  reformulação.

- **Padrão "inventariar primeiro"** (P156F/G) reaplicado
  com inventário curto (decisão arquitectural já
  estabelecida).

- **Reuso de pattern Block**: P156H replica decisão
  arquitectural de P156G (variant rico), tabela arms
  (matriz), validação rigorosa. Reduz custo de pensamento.

- **Padrão emergente "Smart<T> → Option<T>"**: P156H aplica
  novamente (`Smart<Rel<Length>>` para width →
  `Option<Length>`). **Quarta aplicação consecutiva**
  (P156E parity, P156F skew angles, P156G block width,
  P156H box width). **N=4 — patamar empírico forte;
  candidato a registo formal em ADR futuro**.

- **ADR-0061 mantém PROPOSTO**: per decisão humana.
  Anotação cumulativa após P156I.

- **Variants count**: 49 → **50** (+1, Box). Após P156I
  (stack): 51.

- **Stdlib funcs**: 39 → **40** (+1). Após P156I: ~41.

- **Pós-156H**:
  - 9 features Layout implementadas total (pad, hide, h, v,
    pagebreak, skew, block, box + align/move/rotate/scale
    via Transform unificado).
  - Cobertura Layout: 61% → **67%**.
  - Cobertura user-facing total: 58% → ~59%.
  - **Próximo**: P156I (stack — Fase 2 sub-passo 3) ou
    alternativa humana.

- **Slope cumulativo**:
  - P156C +11% (4→6/18).
  - P156D +11% (6→8/18).
  - P156E +6% (8→9/18).
  - P156F +6% (9→10/18).
  - P156G +5% (10→11/18).
  - **P156H +6% (11→12/18)**.
  - **Total**: 38%→67% = +29% em 6 passos.
  - Restantes 5 pontos para 72% target em 1 passo (P156I)
    = ~5%. Realista.

- **Hipótese metodológica em consolidação**: padrão
  "inventariar primeiro + reusar template + decisão sub-
  condicional" funciona em 2 aplicações consecutivas
  (P156F + P156G + P156H — 3 com este). Se P156I confirma,
  N=4 aplicações — patamar empírico forte.

- **Quarentena vanilla**: continua opção 3.

- **Série paridade**: continua suspensa em P153.

- **Hash do código `entities/content.rs`** (após P156G):
  `5702d2e3`. Após P156H: novo (a recomputar).

- **P156I será último passo Fase 2** atinge target 72%.
  Pausa para decisão humana sobre Fase 3 (columns L+ via
  DEBT-56) ou outra prioridade.
