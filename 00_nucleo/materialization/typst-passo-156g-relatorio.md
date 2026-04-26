# Relatório — Passo 156G: block container (Layout Fase 2 sub-passo 1)

**Data**: 2026-04-25.
**Natureza**: passo **substantivo escopo M+** (5 atributos
implementados; ~20 tests; **primeira aplicação Fase 2**
qualitativamente diferente das Fase 1 sub-passos);
**quinta aplicação consecutiva** de ADR-0061 (Layout Fase X
roadmap, status `PROPOSTO`); **decisão arquitectural não-trivial**
em 156G.2 baseada em inventário rigoroso 156G.1.
**Spec**: `00_nucleo/materialization/typst-passo-156g.md`.

**Outputs materiais**:
- 1 variant novo em `Content`: `Block { body, width, height,
  inset, breakable }`.
- Stdlib `#block(body, width: ?, height: ?, inset: ?,
  breakable: true)`.
- Cobertura exaustiva de arms em 4 ficheiros L1.
- 20 tests novos (alvo era 15-25; meta atingida).
- L0 prompt `entities/content.md` ganha secção documentando
  decisão arquitectural P156G.2.
- Inventário 148 actualizado (Layout 10/0/3/5/0=18 →
  11/0/3/4/0=18; cobertura 56% → **61%**; tabela B 48 → 49).
- README ADRs entrada P156G.
- Este relatório (com §15 análise de risco).

---

## §1 — Sumário executivo

P156G é o **primeiro container rico Fase 2** após 4 sub-passos
Fase 1 (P156C/D/E/F). Materializou `block(body, ...)` com
subset Fase 1 dos atributos vanilla per perfil ADR-0054
graded. Crítico: a **decisão arquitectural** (variant rico vs
Style cascade vs struct dedicada vs híbrido) era não-trivial
e tinha precedente para **inventariação primeiro** estabelecido
em P156F.

Trabalho material:
- **Inventário 156G.1** confirmou que `Style` enum cobre só
  propriedades de texto (Bold/Italic/Size/Fill/HeadingLevel) —
  vocabulário não-encaixa para width/height/inset/breakable
  de container. **Decisão**: variant rico (Opção A modificada).
- **`Content::Block { body, width, height, inset, breakable }`**
  adicionado ao enum.
- **Cobertura exaustiva** em 9 sítios pattern-match Content
  (is_empty proxy body; plain_text recurse; PartialEq 5-fields;
  map_content/map_text recurse; introspect recurse;
  layout_content full impl; measure_content_constrained com
  dimensões corretas).
- **stdlib `#block(...)`** com 4 atributos nomeados (width,
  height, inset, breakable); **9 atributos vanilla scope-out**
  (outset, fill, stroke, radius, clip, spacing, above/below,
  sticky) rejeitados com erro hard.
- **Layouter**: força flush_line + aplica inset top/left + body
  + flush + inset bottom + height mínimo forçado. Reusa
  arquitectura `Content::Pad` (P156C) com `inset.right`
  scope-out (mesma razão).
- **Validação rigorosa**: width/height/inset negativos
  rejeitados; breakable deve ser Bool.

**Tests**: **1230 → 1250** (+20). Layout cobertura: **56%
→ 61%** (10/18 → 11/18). User-facing total: 57% → **58%**.

**ADR-0061 mantém-se `PROPOSTO`** per decisão humana
2026-04-25.

**Padrão emergente Fase 2 confirmado**: containers ricos
preferem variants explícitos quando atributos não são
propriedades de texto. P156H (box) e P156I (stack) seguem
mesmo modelo — decisão arquitectural reaplicável.

---

## §2 — Inventário pré-materialização (sub-passo 156G.1)

### §2.1 Estado actual cristalino

```bash
grep -E "Block|block" 01_core/src/entities/content.rs   # nenhum Block existente
grep "fn native_block" 01_core/src/rules/stdlib/         # zero hits
grep "^pub enum Style\b" 01_core/src/entities/style.rs   # 5 variantes
```

Confirmações:
- **Sem `Content::Block`** existente. (Apenas field `block:
  bool` em variants Quote/Equation/Raw — semantic distinto.)
- **`Style` enum** tem 5 variantes: `Bold(bool)`,
  `Italic(bool)`, `Size(Pt)`, `Fill(Color)`,
  `HeadingLevel(u8)`. **Todos relacionados a propriedades de
  TEXTO**.
- **`Sides<T>`** existe (P156C) — `Sides<Length>` é
  construível via `::uniform(L)` ou `::new(l, t, r, b)`.
- **`Rel<T>`** NÃO existe; vanilla usa `Rel<Length>` para
  width.
- **`Sizing` enum (Auto/Length/Fr)** NÃO existe como tipo
  separado; `TrackSizing` é o equivalente granular para Grid.
- **Renderer fill/stroke**: layouter actual emite
  `FrameItem::Shape` para rect/ellipse (P78), mas usar para
  "fundo de container" exigiria emit Shape rectangle por
  baixo do body — não trivial sem refactor.

### §2.2 Análise das 4 hipóteses arquitecturais

| Critério | Opção A (variant rico) | Opção B (Styled) | Opção C (variant + struct) | Opção D (híbrido) |
|----------|------------------------|------------------|----------------------------|-------------------|
| Coerência com vanilla | alta | **baixa** | alta | média |
| Reuso de infra Style | irrelevante | alto | irrelevante | médio |
| **Adequação semântica** | **alta** (container ≠ texto) | **baixa** (Style cobre texto) | alta | média |
| Tests proporcionais | médios | poucos | médios | médios |
| Risco de regressão | baixo (aditivo) | médio (toca Style) | baixo | médio |
| Coerência com P156C Pad | **alta** | baixa | média | baixa |

**Crítico**: Style enum **não foi desenhado** para conter
width/height/inset/breakable. Forçar essas propriedades em
Style misturaria dois domínios distintos (texto vs container)
e tornaria o enum ambíguo. Opção B foi descartada
empiricamente.

### §2.3 Decisão sub-condicional

**Opção A modificada** escolhida:
- Variant `Content::Block` com 5 fields explícitos.
- Atributos avançados scope-out (outset, fill, stroke,
  radius, clip, spacing, above, below, sticky) per ADR-0054
  graded.
- Coerente com `Content::Pad` (P156C) que também tem
  fields explícitos.
- Padrão para P156H (box) e P156I (stack): mesma decisão
  reaplicável.

---

## §3 — `Content::Block` — forma final + diff

```rust
// 01_core/src/entities/content.rs (após Pagebreak)
Block {
    body:      Box<Content>,
    width:     Option<Length>,
    height:    Option<Length>,
    inset:     Sides<Length>,
    breakable: bool,
},
```

**Construtor Rust**: `Content::block(body, width, height,
inset, breakable)`.

**Stdlib**: `#block(body, width: ?, height: ?, inset: ?,
breakable: true)` em `stdlib/layout.rs::native_block`.

**Variant count**: 48 → **49** (+1).

---

## §4 — Cobertura exaustiva de arms

| Função | Ficheiro | Tratamento Block |
|--------|----------|------------------|
| `Content::is_empty()` | `entities/content.rs` | proxy `body.is_empty()` |
| `Content::plain_text()` | `entities/content.rs` | recurse body (transparente) |
| `PartialEq::eq` | `entities/content.rs` | comparação 5-fields |
| `Content::map_content` | `entities/content.rs` | recurse body; preserva atributos Copy |
| `Content::map_text` | `entities/content.rs` | idem |
| `materialize_time` | `rules/introspect.rs` | recurse body; preserva atributos |
| `walk` | `rules/introspect.rs` | walk body (counters/labels resolvem) |
| `layout_content` | `rules/layout/mod.rs` | full impl: flush+inset_top+offset_left+body+flush+inset_bottom+height_min |
| `measure_content_constrained` | `rules/layout/mod.rs` | dimensões: width respeitado se Some, inset adiciona aos lados, height força mínimo se Some |

**Verificação**: `cargo build -p typst-core` clean.

---

## §5 — `native_block` — assinatura + registo

### §5.1 `native_block` em `stdlib/layout.rs`

```rust
pub fn native_block(_ctx, args, _world, _file, _fig)
    -> SourceResult<Value>
{
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s))     => Content::text(s.as_str()),
        Some(other) => return Err(...),
        None => Content::Empty,  // body opcional (vanilla aceita)
    };

    let mut width:         Option<Length> = None;
    let mut height:        Option<Length> = None;
    let mut inset_uniform: Option<Length> = None;
    let mut breakable:     bool = true;

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "width"     => width  = Some(extract_length(value)?),
            "height"    => height = Some(extract_length(value)?),
            "inset"     => inset_uniform = Some(extract_length(value)?),
            "breakable" => breakable = ...Bool check...,
            other => return Err(...scope-out msg...),
        }
    }

    // Validação: negativos rejeitados.
    // ...

    let inset = match inset_uniform {
        Some(l) => Sides::uniform(l),
        None    => Sides::uniform(Length::ZERO),
    };

    Ok(Value::Content(Content::Block {
        body: Box::new(body),
        width, height, inset, breakable,
    }))
}
```

**Decisões locais**:
- Body opcional (vanilla aceita; cristalino fallback Empty).
- `width`/`height`/`inset` aceitam `Length` (via
  `extract_length` helper de P156C — Length, Float-pt,
  Int-pt).
- `inset` aceita Length uniforme apenas (refino futuro
  para dict).
- `breakable` deve ser Bool; default true.
- Atributos scope-out (fill/stroke/etc) rejeitados com
  mensagem explicativa incluindo "atributos avançados
  scope-out per ADR-0054 graded — refino futuro".
- Negativos rejeitados (consistente pad/h/v/pagebreak).

### §5.2 Registo

`stdlib/mod.rs`:
```rust
pub use crate::rules::stdlib::layout::{
    native_align, native_block, native_grid, native_h, ...
};
```

`eval/mod.rs::make_stdlib`:
```rust
scope.define("block", Value::Func(Func::native("block", native_block)));
```

Stdlib funcs: 38 → **39** (+1).

---

## §6 — Layouter — diff

### §6.1 `Content::Block` em `layout_content`

```rust
Content::Block { body, width, height, inset, breakable: _ } => {
    let font = self.font_size_pt.val();
    let inset_left   = inset.left.resolve_pt(font);
    let inset_top    = inset.top.resolve_pt(font);
    let inset_bottom = inset.bottom.resolve_pt(font);
    // inset.right scope-out (Layouter sem largura útil por arm).

    if self.cursor_x.0 > self.line_start_x.0 { self.flush_line(); }

    let start_y = self.cursor_y.0;
    self.cursor_y += Pt(inset_top);

    let saved_line_start = self.line_start_x;
    self.line_start_x = saved_line_start + Pt(inset_left);
    self.cursor_x     = self.line_start_x;

    let _ = width;  // armazenado mas não consumido (refino futuro)

    self.layout_content(body);
    self.flush_line();

    self.cursor_y += Pt(inset_bottom);

    if let Some(h) = height {
        let h_pt = h.resolve_pt(font);
        let consumed = self.cursor_y.0 - start_y;
        if consumed < h_pt {
            self.cursor_y += Pt(h_pt - consumed);
        }
    }

    self.line_start_x = saved_line_start;
    self.cursor_x     = saved_line_start;
}
```

### §6.2 `Content::Block` em `measure_content_constrained`

```rust
Content::Block { body, width, height, inset, breakable: _ } => {
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
```

Em grid measurement, todos os atributos contribuem para
dimensões correctas (incluindo `inset.right` e `width` que
são scope-out no `layout_content`).

---

## §7 — Tests adicionados (lista + contagens)

### §7.1 Em `entities/content.rs::tests` (6)

1. `block_constructor_default_field_values` — defaults
   sensatos.
2. `block_with_explicit_width_height_inset` — atributos
   explícitos.
3. `block_is_empty_proxy_para_body` — proxy semantic.
4. `block_plain_text_recurse_no_body` — recurse texto.
5. `block_partial_eq` — comparação 5-fields.
6. `block_map_text_recurse_no_body` — leaf preserva
   atributos.

### §7.2 Em `stdlib/mod.rs::tests` (12 + regression)

7. `native_block_defaults_sem_args_named` — defaults.
8. `native_block_sem_body_aceita_empty` — body opcional.
9. `native_block_com_width_length` — width Length.
10. `native_block_com_height_int_pt` — Int interpretado pt.
11. `native_block_com_inset_uniforme` — inset 4 lados iguais.
12. `native_block_com_breakable_false` — Bool.
13. `native_block_combina_atributos` — caso composto.
14. `native_block_rejeita_named_arg_avancado` — fill
    rejeitado.
15. `native_block_rejeita_width_negativo` — Err hard.
16. `native_block_rejeita_inset_negativo` — Err hard.
17. `native_block_rejeita_breakable_nao_bool` — tipo errado.
18. `native_pad_e_hide_continuam_a_funcionar_apos_p156g` —
    **regression** verifica que cobertura arms exaustiva
    foi adicionada correctamente em todos os 9 sítios sem
    quebrar containers existentes.

### §7.3 Em `layout/tests.rs` (2)

19. `layout_block_inset_avanca_cursor_y` — A → block(body,
    inset=10pt) → C; verifica que body está abaixo de A e
    C abaixo de body.
20. `layout_block_height_forca_minimo_vertical` — compara
    doc com height=None vs height=100pt; verifica que B em
    doc2 está pelo menos 50pt mais abaixo.

**Total**: **20 tests novos** (alvo spec era 15-25; meta
atingida).

**Tests cumulativos**: 1230 → **1250** (+20 = 6 unit
content + 12 stdlib + 2 layout E2E + regression Pad/Hide
inclusive em #18).

---

## §8 — L0 prompts + hashes propagados

### §8.1 `entities/content.md` actualizado

Secção nova "Variant `Content::Block` — Passo 156G (ADR-0061
Fase 2, sub-passo 1)" adicionada após secção P156F skew.
Inclui:
- **Decisão arquitectural escolhida (Opção A modificada)**
  com rationale completo (Style enum cobre só texto).
- Forma estrutural do variant.
- Atributos implementados + scope-outs documentados.
- Comportamento dos métodos canónicos.
- Renderização layouter detalhada.
- Validação em `native_block`.
- Construtores Rust + Stdlib.
- Limitações conscientes (9 atributos vanilla scope-out;
  inset uniforme apenas; width não impõe limite; breakable
  semantic adiada).
- **Padrão emergente Fase 2** documentado para informar
  P156H/I.

### §8.2 Headers `@updated`

- `entities/content.rs`: `@updated 2026-04-25` (já estava).

### §8.3 Hashes via `crystalline-lint --fix-hashes .`

```
Fixed 1 file:
  ./01_core/src/entities/content.rs             → 5702d2e3
Re-running analysis... ✅ 0 drift warnings remaining
```

`entities/content.rs`: `4321258d` (P156F) → **`5702d2e3`** (P156G).

---

## §9 — Inventário 148 actualizado

Ficheiro: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`.

**Tabela A.5 Layout — linha reescrita**:
- `block(...)`: `ausente` → **`implementado`** ¹³ (Passo 156G).

**Tabela A — Vista user-facing (resumo)**:
- Linha "Layout" (nota ⁵ ⁶ ⁸ ¹⁰ ¹² ¹³): `10/0/3/5/0=18` →
  **`11/0/3/4/0=18`**.
- Total user-facing: `60/21/22/36/2=141` →
  **`61/21/22/35/2=141`**.
- Cobertura user-facing total: 57% → **58%**.

**Tabela B — Arquitectural**:
- `Content` variants (cristalino) ³ ⁴ ⁷ ⁹ ¹¹ ¹⁴: `36/9/3/0/0=48`
  → **`37/9/3/0/0=49`** (+`Block`).
- `Content` variants (vanilla extra ausentes): `~6` → `~5`.

**Notas novas ¹³ + ¹⁴** descrevem transição P156G com:
- Decisão arquitectural variant rico sobre Style cascade.
- 9 atributos vanilla scope-out documentados.
- **Primeira aplicação Fase 2** marcada explicitamente.

**§7 entrada 7**: refinamento P156G documentado com lista
das 4 entradas Layout restantes (`box`, `stack`, `repeat`,
`columns`/`colbreak`) e mapeamento para Fase 2 (P156H box;
P156I stack) e Fase 3 (repeat, columns).

---

## §10 — README ADRs actualizado

Ficheiro: `00_nucleo/adr/README.md`.

- **Tabela "Estado por ADR"**: ADR-0061 mantém-se `PROPOSTO`.
- **Total**: 61 inalterado.
- **Distribuição**: PROPOSTO 11 inalterado.
- **"Passos-chave"**: entrada nova para P156G com detalhe
  da decisão arquitectural (variant rico vs Style cascade),
  9 atributos scope-out, padrão emergente Fase 2, tests
  1230→1250 (+20), Layout 56%→**61%**.

---

## §11 — Próximo passo

P156G encerrou-se com **primeira aplicação Fase 2**
materializada (61% Layout). Próximo passo é **decisão
humana** entre prioridades em ADR-0061:

- **Opção A — P156H (box)**: Fase 2 sub-passo 2 (M, inline
  container). Pode reusar padrão arquitectural de Block.
  Cobertura → 67% (12/18).

- **Opção B — P156I (stack)**: Fase 2 sub-passo 3 (S-M,
  composição com `dir: Dir`). Cobertura → 67% se intercalar
  com box.

- **Opção C — P157 (Model Fase 2 table foundations)**:
  M+ alternativo (per ADR-0060 renumerada).

- **Opção D — Footnote area**: sub-fase prioritária
  explícita per ADR-0061 Decisão 5.

- **Opção E — Outra prioridade humana**.

**Recomendação descritiva**: P156H (box) é o próximo natural
da sequência; reusa decisão arquitectural de P156G (variant
rico) e estabelece padrão para P156I.

---

## §12 — Limitações registadas

1. **9 atributos vanilla scope-out** (outset, fill, stroke,
   radius, clip, spacing, above/below, sticky). Refino
   futuro per ADR-0054 graded.

2. **`inset` uniforme apenas** em stdlib. Vanilla aceita
   dict ou número. Refino futuro para dict
   `{left, right, top, bottom}`.

3. **`width` armazenado mas não impõe limite real**.
   Layouter actual sem mecânica de largura útil por arm.
   Refino futuro com refactor multi-region (DEBT-56).

4. **`inset.right` scope-out** em layout (mesma razão que
   `Pad.right` em P156C — refino multi-region).

5. **`breakable: false` semantic real adiada**. Armazenado
   mas layouter não impede quebra mid-block. Refino futuro
   exigiria refactor multi-region.

6. **Sem show rules `#show block: ...`** neste passo
   (consistente com adiamento P154B/P155/P156C/D/E/F).

7. **Block aninhado**: suportado estruturalmente; insets
   cumulativos via cursor advance. Não testado
   explicitamente (case patológico raro).

8. **ADR-0061 mantém `PROPOSTO`** (per decisão humana).

---

## §13 — Verificação final

Critérios da spec P156G (§Verificação):

1. ✅ `cargo build --workspace`: clean.
2. ✅ `cargo test --workspace --lib`: **1250 passed**;
   0 failed; 6 ignored. (1011 typst-core + 215 integration
   + 24 outros; 1230 → 1250 = +20).
3. ✅ `crystalline-lint .`: **zero violations**.
4. ✅ Hashes propagados consistentes: `entities/content.rs`
   ↔ `entities/content.md` (`5702d2e3`).
5. ✅ `Content::Block` variant em produção (48 → 49 variants).
6. ✅ Stdlib `#block(...)` invocável (38 → 39 funcs).
7. ✅ Cobertura arms exaustiva (9 sítios actualizados).
8. ✅ Inventário 148 reflecte cobertura aumentada (56% →
   61%).
9. ✅ README ADRs entrada P156G.
10. ✅ Sem ADR criada / revogada / revisada.
11. ✅ Sem DEBT criado / fechado.
12. ✅ ADR-0061 inalterada (mantém-se PROPOSTO).
13. ✅ ADR-0060 inalterada.
14. ✅ **Sem regressão** em tests existentes (regression
    test `native_pad_e_hide_continuam_a_funcionar_apos_p156g`
    verifica explicitamente).
15. ✅ Sem regressão geral (todos os 1230 tests pré-P156G
    continuam a passar; +20 novos passam).
16. ✅ Relatório do passo escrito (este ficheiro com §15
    análise de risco).

---

## §14 — Notas operacionais

- **Padrão "passos granulares" — quinta aplicação
  consecutiva**. P156C+D+E+F+G todos com cadência estável,
  zero reformulações mid-passo. **N=5 aplicações**
  reforçam a hipótese.

- **Padrão "inventariar primeiro" formalizado em P156F
  reaplicado em P156G**: 156G.1 expandido a análise das 4
  hipóteses arquitecturais; 156G.2 escolheu com base em
  evidência. Sem pausa-e-consulta porque inventário deu
  pista clara (Style enum cobre só texto).

- **Decisão arquitectural impacta P156H/I**: Block estabelece
  modelo "variant rico para containers". Box e Stack
  provavelmente seguem mesmo modelo, simplificando
  decisões.

- **ADR-0061 mantém PROPOSTO**: per decisão humana.

- **Variants count**: 48 → **49** (+1, Block). Após P156H
  (box): 50. Após P156I (stack): 51.

- **Stdlib funcs**: 38 → **39** (+1).

- **Pós-156G**:
  - 8 features Layout implementadas total.
  - Cobertura Layout: 56% → **61%**.
  - Cobertura user-facing total: 57% → **58%**.
  - **Próximo**: P156H (box) ou alternativa humana.

- **Slope cumulativo Fase 1+início Fase 2**:
  - P156C +11% (4→6/18).
  - P156D +11% (6→8/18).
  - P156E +6% (8→9/18).
  - P156F +6% (9→10/18).
  - P156G +5% (10→11/18) — **Fase 2 inicia**.
  - **Total**: 38%→61% = +23% em 5 passos.
  - Restantes 11 pontos para 72% target em 2-3 passos
    (P156H/I + opcional Fase 3) = ~5-6% por passo. Realista.

- **Quarentena vanilla**: continua opção 3.

- **Série paridade**: continua suspensa em P153.

- **Hash do código `entities/content.rs`**: `4321258d`
  (P156F) → **`5702d2e3`** (P156G).

---

## §15 — Análise de risco de regressão

(Modelo P156F §15.)

### §15.1 Risco a priori

P156G era **primeiro container rico Fase 2**, com escopo M+
qualitativamente diferente de P156C/D/E/F (Fase 1 sub-passos
S/S+). Risco de regressão derivado de:
- Escopo maior (5 fields vs 1-3 dos anteriores).
- Possibilidade de tocar Style cascade infra (opção B).
- Cobertura arms em 9 sítios.

### §15.2 Risco realizado

**Baixo**. Mitigado por:
- Decisão arquitectural variant rico (Opção A) **eliminou
  risco da opção B** que tocaria Style infra.
- Cobertura arms feita sistematicamente (matriz §4) —
  build clean ao primeiro try.
- Regression test #18 (`native_pad_e_hide_continuam_a_funcionar`)
  verificou explicitamente que containers Pad/Hide existentes
  não foram quebrados pela adição de Block.

### §15.3 Mitigações activadas

1. **Inventário 156G.1 expandido** (análise das 4 hipóteses)
   → escolheu opção que minimiza risco (variant aditivo).
2. **Decisão arquitectural justificada empiricamente**
   (Style cobre só texto) → não-arbitrária; reproducível.
3. **Regression test explícito** para Pad/Hide (P156C
   containers).

### §15.4 Lições

- **Padrão "inventariar primeiro" + decisão sub-condicional
  por critérios** funcionou pela segunda vez consecutiva
  (P156F + P156G). Hipótese reforçada.
- **Escopo M+ de container rico não é
  qualitativamente diferente de S/S+** quando a decisão
  arquitectural é clara: tests proporcionais a fields,
  cobertura arms mecânica, layouter reusa Pad pattern.
- **Padrão emergente Fase 2 documentado** (variant rico
  para containers) **simplifica P156H/I**: decisão
  arquitectural já estabelecida; reaplicação directa.

### §15.5 Recomendação meta-metodológica para P156H/I

1. Inventário pré-código pode ser **mais curto** (decisão
   arquitectural já estabelecida; só verificar especificidades
   de box/stack).
2. **Reusar pattern Block**: 5-6 fields explícitos, cobertura
   arms mecânica, layouter inspirado em Pad/Block.
3. Regression tests para containers existentes ainda valem.

---

## §16 — Cross-references

- Spec: `00_nucleo/materialization/typst-passo-156g.md`.
- Diagnóstico (origem): `00_nucleo/diagnosticos/diagnostico-layout-passo-156b.md`.
- Relatórios precedentes: `typst-passo-156c-relatorio.md`,
  `typst-passo-156d-relatorio.md`,
  `typst-passo-156e-relatorio.md`,
  `typst-passo-156f-relatorio.md`.
- ADR-0061 (aplicada): `00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md`.
- Inventário 148 actualizado:
  `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`.
- README ADRs: `00_nucleo/adr/README.md`.
- L0 prompts: `entities/content.md` (secção Block adicionada
  documentando decisão arquitectural).
- Vanilla source consultado:
  `lab/typst-original/crates/typst-library/src/layout/container.rs`
  (BlockElem com 14 atributos).
- Cristalino código tocado:
  - `01_core/src/entities/content.rs` (variant Block +
    construtor + cobertura 5 arms + 6 tests).
  - `01_core/src/rules/introspect.rs` (arms Block em
    materialize_time + walk).
  - `01_core/src/rules/layout/mod.rs` (arms Block em
    layout_content + measure_content_constrained).
  - `01_core/src/rules/layout/tests.rs` (2 tests E2E).
  - `01_core/src/rules/stdlib/layout.rs` (`native_block`).
  - `01_core/src/rules/stdlib/mod.rs` (re-export + 12 tests
    incluindo regression).
  - `01_core/src/rules/eval/mod.rs` (registo em `make_stdlib`).
