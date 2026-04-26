# Relatório — Passo 156H: box inline container (Layout Fase 2 sub-passo 2)

**Data**: 2026-04-25.
**Natureza**: passo **substantivo escopo M** (5 atributos
implementados; ~21 tests; reaplicação directa do template
Block); **sexta aplicação consecutiva** de ADR-0061 (Layout
Fase X roadmap, status `PROPOSTO`); **decisão arquitectural
reusada** de P156G sem nova decisão.
**Spec**: `00_nucleo/materialization/typst-passo-156h.md`.

**Outputs materiais**:
- 1 variant novo em `Content`: `Boxed { body, width, height,
  inset, baseline }` (naming evita conflito com `std::boxed::Box`).
- Stdlib `#box(body, width: ?, height: ?, inset: ?,
  baseline: ?)`.
- Cobertura exaustiva de arms em 4 ficheiros L1.
- 21 tests novos (alvo era 15-22; meta atingida).
- L0 prompt `entities/content.md` ganha secção Boxed.
- Inventário 148 actualizado (Layout 11/0/3/4/0=18 →
  12/0/3/3/0=18; cobertura 61% → **67%**; tabela B 49 → 50).
- README ADRs entrada P156H.
- Este relatório (com §15 análise de risco).

---

## §1 — Sumário executivo

P156H é **segunda aplicação Fase 2** (containers ricos),
seguindo P156G (block). Materializou `box(body, ...)` com
**decisão arquitectural reusada** do template Block sem
nova investigação — primeira validação empírica do padrão
emergente "Fase 2 reaplica decisões".

**Trabalho material**:
- **Inventário 156H.1 curto** confirmou: Strong/Emph removidos
  em P101 → emitem `Content::Styled` (inline natural sem
  flush); cursor.rs sem mecânica baseline mid-linha →
  baseline scope-out semantic real per ADR-0054 graded.
- **`Content::Boxed`** (naming evita conflito com `std::Box`)
  adicionado ao enum.
- **Cobertura exaustiva** em 9 sítios pattern-match Content
  (modelo Block reaplicado).
- **stdlib `#box(...)`** com 4 atributos nomeados (width,
  height, inset, baseline); **6 atributos scope-out**
  (outset, fill, stroke, radius, clip, stroke-overhang)
  rejeitados com erro hard.
- **Layouter inline**: NÃO força flush_line; aplica
  inset.left + body + inset.right como avanço cursor.x;
  width/height/baseline/inset.top/bottom armazenados mas
  semantic real adiada.
- **Validação**: width/height/inset negativos rejeitados;
  baseline negativo aceito (semantic legítima).

**Tests**: **1250 → 1271** (+21). Layout cobertura: **61%
→ 67%** (11/18 → 12/18). User-facing total: 58% → **59%**.

**ADR-0061 mantém-se `PROPOSTO`** per decisão humana
2026-04-25.

**Padrão emergente Fase 2 confirmado**: reaplicação directa
do template Block reduz custo cognitivo. P156I (stack) tem
agora dois precedentes (Block + Boxed) — decisão
arquitectural totalmente estabelecida.

---

## §2 — Inventário curto (sub-passo 156H.1)

### §2.1 Mecânica inline em layouter actual

```bash
grep -E "Content::Strong|Content::Emph|Self::Strong" \
  01_core/src/rules/layout/mod.rs
```

**Descoberta**: comentário em `mod.rs:267` confirma que
"`Content::Strong` e `Content::Emph` removidos do enum"
desde P101. Em vez disso emitem `Content::Styled([Bold/Italic])`
que é tratado in-place sem flush_line — **mantém-se inline
naturalmente**. Sem método específico de "append inline" no
Layouter — basta NÃO chamar `flush_line` no arm.

**Conclusão**: arm de Box deve simplesmente layouted body
in-place (cursor.x avança via `layout_content` recursivo),
sem flush_line.

### §2.2 Mecanismo baseline em cursor.rs

```bash
grep "baseline" 01_core/src/rules/layout/cursor.rs
```

**Descoberta**: única ocorrência (`cursor.rs:98`) refere-se
a `cursor.y` da linha actual ("o seu `.style.leading` é o
valor efectivo do baseline"). **Não há mecânica
`baseline_offset` mid-linha**. Aplicar baseline a Box
exigiria refactor.

**Decisão**: `baseline` armazenado mas semantic real adiada
(consistente com `breakable: false` em P156G). Limitação
documentada per ADR-0054 graded.

### §2.3 Estado pré-P156H verificado

- Content tem 49 variants (Block em P156G foi 49º).
- Sem `Content::Box` ou `Content::Boxed`.
- Sem `native_box` em stdlib.

---

## §3 — `Content::Boxed` — forma final + diff

```rust
// 01_core/src/entities/content.rs (após Pagebreak, antes de Block)
Boxed {
    body:     Box<Content>,
    width:    Option<Length>,
    height:   Option<Length>,
    inset:    Sides<Length>,
    baseline: Length,
},
```

**Naming**: variant Rust é `Boxed` (não `Box`) para evitar
ambiguidade com `std::boxed::Box`. Stdlib expõe `#box(...)`
(paridade vanilla); construtor Rust: `Content::boxed(...)`.

**Variant count**: 49 → **50** (+1).

---

## §4 — Cobertura exaustiva de arms

| Função | Ficheiro | Tratamento Boxed |
|--------|----------|------------------|
| `Content::is_empty()` | `entities/content.rs` | proxy `body.is_empty()` |
| `Content::plain_text()` | `entities/content.rs` | recurse body |
| `PartialEq::eq` | `entities/content.rs` | comparação 5-fields |
| `Content::map_content` | `entities/content.rs` | recurse body; preserva atributos Copy |
| `Content::map_text` | `entities/content.rs` | idem |
| `materialize_time` | `rules/introspect.rs` | recurse body; preserva atributos |
| `walk` | `rules/introspect.rs` | walk body |
| `layout_content` | `rules/layout/mod.rs` | inline: inset.left + body + inset.right |
| `measure_content_constrained` | `rules/layout/mod.rs` | dimensões análogas Block |

**Verificação**: `cargo build -p typst-core` clean.

---

## §5 — `native_box` — assinatura + registo

### §5.1 `native_box` em `stdlib/layout.rs`

Análogo a `native_block` (P156G); diferenças:
- `breakable` → `baseline` (Length em vez de bool).
- `baseline` aceita negativo (move box para cima).
- 6 atributos scope-out (vs 9 de Block) — set ligeiramente
  menor.

### §5.2 Registo

`stdlib/mod.rs`:
```rust
pub use crate::rules::stdlib::layout::{
    native_align, native_block, native_box, native_grid, ...
};
```

`eval/mod.rs::make_stdlib`:
```rust
scope.define("box", Value::Func(Func::native("box", native_box)));
```

Stdlib funcs: 39 → **40** (+1).

---

## §6 — Layouter — diff

### §6.1 `Content::Boxed` em `layout_content`

```rust
Content::Boxed { body, width, height, inset, baseline } => {
    let font = self.font_size_pt.val();
    let inset_left  = inset.left.resolve_pt(font);
    let inset_right = inset.right.resolve_pt(font);

    // Box é INLINE: NÃO chama flush_line.
    self.cursor_x += Pt(inset_left);

    let _ = width;    // armazenado; refino futuro
    let _ = height;   // armazenado; refino futuro
    let _ = baseline; // armazenado; refino futuro

    self.layout_content(body);

    self.cursor_x += Pt(inset_right);
}
```

**Diferenças face a Block (P156G)**:
- Sem `flush_line` antes (Block força).
- Sem ajuste de `cursor_y` (Block aplica inset_top + altura
  + inset_bottom).
- Sem `line_start_x` shift (Block ajusta para indent).
- Apenas `cursor_x` avança (avanço inline).

### §6.2 `Content::Boxed` em `measure_content_constrained`

Análogo a Block (mesma lógica width/height/inset; baseline
ignorado em medição porque é offset apenas, não dimensão).

---

## §7 — Tests adicionados (lista + contagens)

### §7.1 Em `entities/content.rs::tests` (6)

1. `boxed_constructor_default` — defaults sensatos.
2. `boxed_constructor_explicit_atributos` — incl baseline
   negativo aceite.
3. `boxed_is_empty_proxy_para_body` — proxy semantic.
4. `boxed_plain_text_recurse_no_body` — recurse texto.
5. `boxed_partial_eq` — comparação 5-fields incl baseline.
6. `boxed_map_text_recurse_no_body` — leaf preserva
   atributos.

### §7.2 Em `stdlib/mod.rs::tests` (12 + regression)

7. `native_box_defaults_sem_args_named` — defaults.
8. `native_box_sem_body_aceita_empty` — body opcional.
9. `native_box_com_width_length` — width Length.
10. `native_box_com_height_int_pt` — Int interpretado pt.
11. `native_box_com_inset_uniforme` — inset 4 lados iguais.
12. `native_box_com_baseline` — baseline Length.
13. `native_box_baseline_negativo_aceito` — semantic legítima.
14. `native_box_combina_atributos` — caso composto.
15. `native_box_rejeita_atributo_avancado` — fill rejeitado.
16. `native_box_rejeita_width_negativo` — Err hard.
17. `native_box_rejeita_inset_negativo` — Err hard.
18. `native_box_rejeita_named_arg_desconhecido` — alignment
    rejeitado.
19. `native_block_pad_hide_continuam_a_funcionar_apos_p156h`
    — **regression** Block + Pad + Hide.

### §7.3 Em `layout/tests.rs` (2)

20. `layout_box_mantem_inline_nao_forca_flush` — A + box(M)
    + B na mesma linha (verificado por posições Y idênticas).
21. `layout_box_inset_left_aplica_avanco_horizontal` —
    inset=20pt empurra body em pelo menos 20pt à direita.

**Total**: **21 tests novos** (alvo spec era 15-22; meta
atingida).

**Tests cumulativos**: 1250 → **1271** (+21 = 6 unit content
+ 13 stdlib + 2 layout E2E).

---

## §8 — L0 prompts + hashes propagados

### §8.1 `entities/content.md` actualizado

Secção nova "Variant `Content::Boxed` (box inline) — Passo
156H (ADR-0061 Fase 2, sub-passo 2)" adicionada após secção
P156G Block. Inclui:
- Decisão arquitectural reusada (Opção A modificada de P156G).
- Forma estrutural do variant + naming `Boxed`.
- Atributos implementados + scope-outs documentados.
- **Distinção material face a Block** (tabela comparativa).
- Comportamento dos métodos canónicos.
- Renderização layouter inline detalhada.
- Validação em `native_box` (incluindo baseline negativo
  aceito).
- Construtores Rust + Stdlib.
- Limitações conscientes.
- Padrão emergente Fase 2 confirmado.

### §8.2 Hashes via `crystalline-lint --fix-hashes .`

```
Fixed 1 file:
  ./01_core/src/entities/content.rs             → 5bb6e3d2
Re-running analysis... ✅ 0 drift warnings remaining
```

`entities/content.rs`: `5702d2e3` (P156G) → **`5bb6e3d2`** (P156H).

---

## §9 — Inventário 148 actualizado

Ficheiro: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`.

**Tabela A.5 Layout — linha reescrita**:
- `box(...)`: `ausente` → **`implementado`** ¹⁵ (Passo 156H).

**Tabela A — Vista user-facing (resumo)**:
- Linha "Layout" (nota ⁵ ⁶ ⁸ ¹⁰ ¹² ¹³ ¹⁵): `11/0/3/4/0=18`
  → **`12/0/3/3/0=18`**.
- Total user-facing: `61/21/22/35/2=141` →
  **`62/21/22/34/2=141`**.
- Cobertura user-facing total: 58% → **59%**.

**Tabela B — Arquitectural**:
- `Content` variants (cristalino) ³ ⁴ ⁷ ⁹ ¹¹ ¹⁴ ¹⁶:
  `37/9/3/0/0=49` → **`38/9/3/0/0=50`** (+`Boxed`).
- `Content` variants (vanilla extra ausentes): `~5` → `~4`.

**Notas novas ¹⁵ + ¹⁶** descrevem transição P156H.

**§7 entrada 7**: refinamento P156H documentado com lista
das 3 entradas Layout restantes (`stack`, `repeat`,
`columns`/`colbreak`) e mapeamento para P156I (stack —
último Fase 2) e Fase 3 (repeat, columns).

---

## §10 — README ADRs actualizado

Ficheiro: `00_nucleo/adr/README.md`.

- **Tabela "Estado por ADR"**: ADR-0061 mantém-se `PROPOSTO`.
- **Total**: 61 inalterado.
- **Distribuição**: PROPOSTO 11 inalterado.
- **"Passos-chave"**: entrada nova para P156H com detalhe
  da reaplicação do padrão Block, distinção inline vs
  structural, baseline negativo aceito, 6 scope-outs, tests
  1250→1271 (+21), Layout 61%→**67%**.

---

## §11 — Próximo passo

P156H encerrou-se com **segunda aplicação Fase 2**
materializada (67% Layout). Próximo passo é **decisão
humana** entre prioridades em ADR-0061:

- **Opção A — P156I (stack — último Fase 2)**: S-M
  (composição com `dir: Dir`). Cobertura → 72% (13/18).
  **Atinge target Fase 1+2 declarado em ADR-0061**.

- **Opção B — Fase 3 Layout**: repeat (M, baixo valor),
  columns (L+ com DEBT-56). Cobertura → 78-100%.

- **Opção C — P157 (Model Fase 2 table foundations)**:
  M+ alternativo (per ADR-0060 renumerada).

- **Opção D — Footnote area**: sub-fase prioritária
  explícita per ADR-0061 Decisão 5.

- **Opção E — Outra prioridade humana**.

**Recomendação descritiva**: P156I (stack) é o último
sub-passo Fase 2; fechá-lo atinge target 72% e fecha série
Fase 2 antes de decisão sobre Fase 3 (que tem DEBT-56
column flow L+ a considerar).

---

## §12 — Limitações registadas

1. **6 atributos vanilla scope-out** (outset, fill, stroke,
   radius, clip, stroke-overhang). Refino futuro per
   ADR-0054 graded.

2. **`inset` uniforme apenas** em stdlib. Refino futuro
   para dict.

3. **`width`/`height` armazenados mas não impõem limite real**
   (Layouter actual sem mecânica de largura útil por arm).
   Refino multi-region (DEBT-56).

4. **`baseline` armazenado mas semantic real adiada**.
   Cursor.rs actual sem mecânica de offset vertical
   mid-linha. Refino futuro.

5. **`inset.top`/`inset.bottom` armazenados mas não aplicados
   em layout inline** (alterariam line_height; refino futuro).

6. **`inset.right` aplicado** (avança cursor.x após body)
   — diferença vs Block onde `inset.right` é scope-out.
   Box é mais simples porque é puramente inline.

7. **Sem show rules `#show box: ...`** neste passo.

8. **Box aninhado em Block ou box** suportado
   estruturalmente; não testado explicitamente em E2E
   (case patológico raro).

9. **ADR-0061 mantém `PROPOSTO`** (per decisão humana).

---

## §13 — Verificação final

Critérios da spec P156H (§Verificação):

1. ✅ `cargo build --workspace`: clean.
2. ✅ `cargo test --workspace --lib`: **1271 passed**;
   0 failed; 6 ignored. (1032 typst-core + 215 integration
   + 24 outros; 1250 → 1271 = +21).
3. ✅ `crystalline-lint .`: **zero violations**.
4. ✅ Hashes propagados consistentes: `entities/content.rs`
   ↔ `entities/content.md` (`5bb6e3d2`).
5. ✅ `Content::Boxed` variant em produção (49 → 50 variants).
6. ✅ Stdlib `#box(...)` invocável (39 → 40 funcs).
7. ✅ Cobertura arms exaustiva (9 sítios actualizados).
8. ✅ Inventário 148 reflecte cobertura aumentada (61% →
   67%).
9. ✅ README ADRs entrada P156H.
10. ✅ Sem ADR criada / revogada / revisada.
11. ✅ Sem DEBT criado / fechado.
12. ✅ ADR-0061 inalterada (mantém-se PROPOSTO).
13. ✅ ADR-0060 inalterada.
14. ✅ **Sem regressão** em containers existentes (Block,
    Pad, Hide) — regression test verifica explicitamente.
15. ✅ Sem regressão geral (todos os 1250 tests pré-P156H
    continuam a passar; +21 novos passam).
16. ✅ Relatório do passo escrito (este ficheiro com §15
    análise de risco).

---

## §14 — Notas operacionais

- **Padrão "passos granulares" — sexta aplicação
  consecutiva**. P156C+D+E+F+G+H todos com cadência
  estável. **N=6 aplicações** sem reformulação.

- **Padrão "inventariar primeiro" reusado com inventário
  curto**: P156H §2.1-2.3 bem mais conciso que P156G.1
  (decisão arquitectural já estabelecida — só verificar
  especificidades de inline + baseline).

- **Reuso directo do template Block**: P156H replica
  estrutura de variant + arms cobertura + native + tests
  sem nova decisão arquitectural. Custo cognitivo reduzido
  significativamente.

- **Naming `Boxed` em vez de `Box`**: pequena divergência
  necessária para evitar conflito com `std::boxed::Box` em
  contextos onde o type Box e o variant Box poderiam
  ser ambíguos. Stdlib mantém `#box(...)` (paridade vanilla);
  divergência fica oculta a utilizadores Typst.

- **ADR-0061 mantém PROPOSTO**: per decisão humana.

- **Variants count**: 49 → **50** (+1, Boxed). Após P156I
  (stack): 51.

- **Stdlib funcs**: 39 → **40** (+1).

- **Pós-156H**:
  - 9 features Layout implementadas total.
  - Cobertura Layout: 61% → **67%**.
  - Cobertura user-facing total: 58% → **59%**.
  - **Próximo**: P156I (stack — último Fase 2; atinge
    target 72%).

- **Slope cumulativo**:
  - P156C +11% (4→6/18).
  - P156D +11% (6→8/18).
  - P156E +6% (8→9/18).
  - P156F +6% (9→10/18).
  - P156G +5% (10→11/18).
  - **P156H +6% (11→12/18)**.
  - **Total Fase 1+início Fase 2**: 38%→67% = +29% em 6
    passos.
  - Restante 5 pontos para 72% target em 1 passo (P156I).

- **Quarentena vanilla**: continua opção 3.

- **Série paridade**: continua suspensa em P153.

- **Hash do código `entities/content.rs`**: `5702d2e3` (P156G)
  → **`5bb6e3d2`** (P156H).

---

## §15 — Análise de risco de regressão

### §15.1 Risco a priori

P156H era **segundo container Fase 2**, com escopo M
qualitativamente similar a P156G (block — variant rico com
5 fields). Risco de regressão:
- Cobertura arms em 9 sítios (mecânico).
- Possibilidade de quebrar containers existentes (Block,
  Pad, Hide) por mistakes em pattern-match.
- Conflito naming `Box` vs `std::boxed::Box`.

### §15.2 Risco realizado

**Baixo, igual ou inferior a P156G**. Mitigado por:
- Decisão arquitectural reusada (zero novo design).
- Naming `Boxed` antecipadamente diferente para evitar
  conflito std.
- Cobertura arms feita sistematicamente (matriz §4) —
  build clean ao primeiro try.
- Regression test #19 (`native_block_pad_hide_continuam_a_funcionar`)
  verificou explicitamente que Block + Pad + Hide
  continuam a funcionar.

### §15.3 Mitigações activadas

1. **Inventário curto** (decisão arquitectural já
   estabelecida) → tempo focado em verificar
   especificidades inline + baseline.
2. **Naming preventivo** `Boxed` evita conflito Rust.
3. **Regression tests** estendidos para cobrir todos os
   containers Layout existentes.

### §15.4 Lições

- **Padrão "inventariar primeiro" + reuso de template**
  funcionou pela segunda vez consecutiva (P156G + P156H).
  N=2 aplicações reforçam.
- **Reuso de template é mais barato que decisão de novo**:
  P156H foi mais rápido que P156G apesar do escopo
  similar.
- **Pequenas divergências de naming** (Boxed vs Box) podem
  ser previstas e mitigadas em vez de descobertas mid-passo.

### §15.5 Recomendação meta-metodológica para P156I

1. Inventário **muito curto** (decisão arquitectural
   estabelecida em P156G + reaplicada em P156H).
2. **Reusar template directamente**: stack é variant rico
   com fields explícitos para `dir: Dir`, spacing, children.
3. Regression tests estendidos para incluir Boxed também.
4. Antecipar especificidade do stack (não tem body único
   mas children Vec<Content> — diferente de Block/Box).

---

## §16 — Cross-references

- Spec: `00_nucleo/materialization/typst-passo-156h.md`.
- Diagnóstico (origem): `00_nucleo/diagnosticos/diagnostico-layout-passo-156b.md`.
- Relatórios precedentes: `typst-passo-156c-relatorio.md`,
  `typst-passo-156d-relatorio.md`,
  `typst-passo-156e-relatorio.md`,
  `typst-passo-156f-relatorio.md`,
  `typst-passo-156g-relatorio.md`.
- ADR-0061 (aplicada): `00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md`.
- Inventário 148 actualizado:
  `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`.
- README ADRs: `00_nucleo/adr/README.md`.
- L0 prompts: `entities/content.md` (secção Boxed
  adicionada).
- Vanilla source consultado:
  `lab/typst-original/crates/typst-library/src/layout/container.rs`
  (BoxElem com 10 atributos).
- Cristalino código tocado:
  - `01_core/src/entities/content.rs` (variant Boxed +
    construtor + cobertura 5 arms + 6 tests).
  - `01_core/src/rules/introspect.rs` (arms Boxed em
    materialize_time + walk).
  - `01_core/src/rules/layout/mod.rs` (arms Boxed em
    layout_content + measure_content_constrained).
  - `01_core/src/rules/layout/tests.rs` (2 tests E2E).
  - `01_core/src/rules/stdlib/layout.rs` (`native_box`).
  - `01_core/src/rules/stdlib/mod.rs` (re-export + 13 tests
    incluindo regression).
  - `01_core/src/rules/eval/mod.rs` (registo em `make_stdlib`).
