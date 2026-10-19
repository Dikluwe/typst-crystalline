# Relatório — typst-passo-825: `typst_library::math` — classes, field access bare, fence espaçado, LeftRightAlternator (achado #12 de P810)

**Data:** 2026-07-22
**Executor:** Kimi Code (subagente, a pedido do agente principal — prompt lido de `00_nucleo/materialization/typst-passo-825.md`; único ficheiro acedido em `materialization/`). Medições originais: §12 de `00_nucleo/diagnosticos/typst-passo-810-relatorio.md`.
**Proveniência das medições:** commit HEAD `2acc14eac28468795c9d14c8a450fa5e320bf888`; working tree **não commitado** em todas as medições (`git diff HEAD --stat` no início: 64 ficheiros, +4451/-430 — passos P823…P826 + P813; ao fechar, 2026-07-22T03:58 (-03:00), 69 ficheiros, +5071/-430). Horas: sonda ANTES 03:03–03:20, DEPOIS 03:49–03:58. Binário cristalino rebuildado (`cargo build --release`, 18.23s) após a última alteração e antes da medição final.
**Binários:** `./target/release/typst` (cristalino — `typst <input> <out.pdf>`), `lab/typst-original/target/release/typst` (vanilla 0.15.0 — `typst compile <input> <out.pdf>`). Fixtures em `temp/p825/`. Geometria com `mutool trace`.

**Nota de enquadramento (ADR-0108):** o texto de implementação do sub-A no prompt ("adicionar as classes em falta") tem a premissa **invertida** face à medição — o achado original de P810 §12(a) regista que o **cristalino aceita 15 nomes** e o **vanilla rejeita 5 deles no cast**. A sonda reconfirmou P810 (vanilla rejeita com erro de cast; cristalino compilava). A correcção feita é a **restrição** ao domínio do cast do vanilla (10 nomes), não a adição — a medição manda sobre o enquadramento do prompt.

---

## Sub-achado A — domínio de classes: 15 aceites no cristalino vs 10 no vanilla

### Sonda (ANTES, literal)

`$ #math.class("<nome>", "x") $` (fixtures `temp/p825/cls-*.typ`):

```text
cls-alphabetic  V: error: expected "normal", "punctuation", "opening", "closing", "fence",
                     "large", "relation", "unary", "binary", or "vary"    C: compila (silêncio)
cls-diacritic   V: idem                                                    C: compila
cls-glyph-part  V: idem                                                    C: compila
cls-space       V: idem                                                    C: compila
cls-special     V: idem                                                    C: compila
cls-relation    V: compila                                                 C: compila (controlo)
cls-vary        V: compila                                                 C: compila (controlo)
cls-banana      V: error: expected "normal", ..., or "vary"
                C: error: class(): 'banana' não é uma MathClass reconhecida   (PT, <detached>)
cls-int         V: error: expected "normal", ..., or "vary", found integer
                C: error: class() espera string, recebeu content              (tipo errado: content)
```

**Código identificado:** vanilla — domínio do cast em `lab/typst-original/crates/typst-library/src/foundations/cast.rs:502-520` (10 strings; as variantes `Alphabetic`/`Diacritic`/`GlyphPart`/`Space`/`Special` existem no enum, `cast.rs:482-500`, mas **não** no cast — são classes internas atribuídas automaticamente). Cristalino — `01_core/src/engine/stdlib/structural.rs:2199-2229` (`native_math_class` validava contra as 15 de `entities/math_class.rs::parse_math_class:115-134`).

### Implementação

`01_core/src/engine/stdlib/structural.rs` — `CAST_DOMAIN` (10 nomes) + `CAST_MSG` (verbatim); string fora do domínio → `expected "normal", ..., or "vary"`; não-string → mesma mensagem + `, found {tipo}` com nomes longos vanilla (`vanilla_type_name_class`, mesma convenção de `loading.rs:561`/`pdf.rs:88`). `parse_math_class` (15 nomes) mantido — usado pelo `repr()` (`eval/repr.rs:391`). `01_core/src/engine/eval/math.rs::eval_math_arg_value` — literais escalares (`Int`/`Float`/`Bool`/`Numeric`) avaliam em modo código (paridade: `#class(3, x)` reporta `found integer`, não `found content`). L0: `00_nucleo/prompts/engine/stdlib/structural.md` (domínio + erros de cast + testes canónicos actualizados).

### Validação (DEPOIS, literal)

```text
cls-alphabetic/diacritic/glyph-part/space/special  C: error: expected "normal", ..., or "vary"   ✓ verbatim vanilla
cls-banana   C: error: expected "normal", ..., or "vary"                                          ✓ verbatim vanilla
cls-int      C: error: expected "normal", ..., or "vary", found integer                           ✓ verbatim vanilla
cls-relation / cls-vary  C: compila                                                               ✓ controlo
```

Testes: `p825a_aceita_as_10_classes_do_cast_vanilla`, `p825a_rejeita_variantes_internas_fora_do_cast`, `p825a_nome_desconhecido_tem_mesmo_erro_de_cast`, `p825a_arg_nao_string_reporta_tipo_vanilla` (structural.rs), `p825a_e2e_class_int_reporta_found_integer` (eval/tests.rs) — confirmados a falhar ANTES (`p825a_rejeita...`, `p825a_nome...`, `p825a_arg...` FAILED; os e2e nem compilavam a asserção) e a passar DEPOIS.

## Sub-achado B — field access bare em modo math compila (não devia)

### Sonda (ANTES, literal)

```text
$ math.class("relation", "x") $   V: error: unknown variable: math (+3 hints)   C: compila
$ sym.suit.heart $                V: error: unknown variable: sym  (+3 hints)   C: compila
$ calc.gcd(4, 6) $                V: error: unknown variable: calc (+3 hints)   C: avalia (erro de args)
$ emoji.face $                    V: error: unknown variable: emoji             C: compila
$ std.math.class("relation","x") $  V: compila                                  C: compila (controlo)
$ class("relation", x) $            V: compila                                  C: compila (controlo)
$ x.campo $                         V: error: unknown variable: campo           C: erro equivalente (controlo)
```

Hints verbatim do vanilla (medidos completos):
```text
hint: `math` is not available directly in math, but is in the standard library
hint: to access `math` in code mode you can add a hash: `#math`
hint: or access `math` in math mode by using the `std` module: `std.math`
```

**Código identificado:** cristalino — `01_core/src/engine/eval/math.rs:144-146` (`eval_math_callee`, arm `Expr::MathIdent`: `scopes.get(name)` devolvia o módulo global sem restrição — lacuna aberta por P782 para field access genérico). O erro `unknown_variable_math` (math.rs:67-93) já era verbatim (P820) — o cristalino já o emitia correctamente para `$ math $` isolado. Relação com P782: é a **mesma área** — a correcção reforça a validação de P782 (não duplica). Distinção de caminhos confirmada no parser: `#expr` vai por `embedded_code_expr` (`engine/parse/math.rs:62`, target `Expr::Ident` de código); bare vai por `MathIdent`/`FieldAccess` do lexer math (`engine/parse/math.rs:64`) — a regra aplica-se só ao braço `MathIdent`, o caminho `#` não é tocado.

**Regra medida no vanilla:** módulos globais (`math`, `sym`, `calc`, `emoji`, …) rejeitados bare; funções expostas no scope math (`class`, `mat`, `lr`, `text`), bindings de utilizador (incl. closures) e o módulo `std` continuam acessíveis (todos medidos — ver fixtures `b*.typ`, `s*.typ`).

### Implementação

`01_core/src/engine/eval/math.rs` — no braço `Expr::MathIdent` de `eval_math_callee`: valor resolvido `Value::Module` e nome ≠ `std` → `unknown_variable_math(span, name, true)` (verbatim, com as 3 hints). **Reversão medida de P782:** o critério `$sym.suit.heart$` → ♥ de P782 foi refutado pelo vanilla (erro) — o teste `p782_field_access_bare_resolve_simbolo` foi reescrito para asserir o erro, com a razão registada em comentário. L0: `00_nucleo/prompts/engine/eval.md` (§P825 + nota de revogação nos critérios de P782).

### Validação (DEPOIS, literal)

```text
$ math.class("relation", "x") $   C: error: unknown variable: math  (1:2) + as 3 hints verbatim   ✓
$ sym.suit.heart $                C: error: unknown variable: sym   (1:2) + hints                 ✓
$ calc.gcd(4, 6) $                C: error: unknown variable: calc  (1:2) + hints                 ✓
$ std.math.class("relation","x") $  C: compila                                                     ✓ excepção vanilla
$ x #sym.suit.heart y $             C: compila (♥)                                                 ✓ caminho # intacto
$ class("relation", x) $            C: compila                                                     ✓ controlo
```

Testes: `p825b_math_class_bare_erro_unknown_variable`, `p825b_calc_bare_erro_unknown_variable`, `p825b_hints_verbatim_vanilla`, `p825b_std_module_continua_acessivel_bare`, `p825b_hash_field_access_continua_a_funcionar`, `p825b_class_bare_continua_a_funcionar` + `p782_field_access_bare_resolve_simbolo` reescrito — 3 confirmados a falhar ANTES, todos a passar DEPOIS.

## Sub-achado C — `fence` espaçado / `class("normal")` — formalização de scope-out (sem código)

### Sonda (literal)

```text
$ a | b $          V: a @287.11 | @296.58 b @303.29  (gaps ≈ 3.65pt dos dois lados de |)
                   C: a @289.91 | @296.51 b @298.77  (gaps ≈ 0.78/0pt)
$ a #math.class("normal", "+") b $
                   V: gaps ≈ 3.65pt dos dois lados do átomo wrapped
                   C: gaps ≈ 0.78/0pt (tabela de classes com Normal)
```

O mecanismo vanilla é o de átomos **"spaced"** (`is_spaced` — fences e átomos wrapped por `class()` recebem o espaço textual ≈ 3.65pt a 11pt em vez da tabela de classes), ligado à regra "spaced frames" já registada como fora-de-escopo no L0 desde P772y.

### Decisão formalizada (sem código, padrão P807/P812-C)

O L0 `00_nucleo/prompts/engine/math/layout/spacing.md` já registava "spaced frames" como fora de escopo mas **não** nomeava os dois casos medidos. Foi acrescentada a cláusula **P825 (sub-C)** que formaliza, com os números medidos: (1) fence "spaced" (`$ a | b $`, gap 3.65 vs ≈0); (2) `class("normal", ...)` "spaced" (gaps 3.65 vs 0). Divergência de língua conhecida e registada — não implementar sem decisão explícita. Nenhum código alterado neste sub-achado.

## Sub-achado D — `LeftRightAlternator` em `mat` ausente

### Sonda (ANTES, literal)

`$ mat(a &= b; x x x x &= y y) $` (`temp/p825/d1.typ`):

```text
VANILLA:  a @291.37  = @300.245  b @311.86   (linha 1 — a à direita do ponto, = alinhado)
          xxxx @272.02..290.90  = @300.245  y @311.86..   (linha 2 — mesmo x do =)
CRISTALINO ANTES:  a @286.49  = @293.09  b @302.19
          xxxx @273.29..293.09  = @299.69  y @308.79..   (= NÃO alinhados: 293.09 vs 299.69)
```

**Código identificado:** vanilla — `math/ir/resolve.rs:1068` (células de alinhamento) + `typst-layout/math/run.rs:320-331` (`LeftRightAlternator::Right` — pares à direita, ímpares à esquerda) e `run.rs:76-94` + `run.rs:363-373` (`alignment_lspace` — o espaço de classe do limite é movido para a célula par). Cristalino — `01_core/src/engine/math/layout/matrix.rs:27` usava sempre `GridAlign::Center`; o `&` dentro da `MathSequence` da célula era filtrado sem efeito (`math/layout/mod.rs:688-693`). Já existia `GridAlign::Alternating` para equações com `&` (não reutilizado por `mat`).

### Implementação

- `01_core/src/engine/math/layout/matrix.rs` — `split_cell_on_align_point` (parte a `MathSequence` da célula nos `MathAlignPoint`); se alguma célula tem `&`, a grelha usa `GridAlign::Alternating`; o espaçamento de classe do limite (`align_boundary_spacing` — `spacing::spacing_between` entre o último nó da célula esquerda e o primeiro da direita, paridade `alignment_lspace`) é somado à largura da `MathBox` da célula par (espaço dentro da célula direita-alinhada, como o vanilla). Sem `&`, caminho inalterado (`GridAlign::Center`).
- `01_core/src/engine/math/layout/mod.rs` — passagem 2 de `layout_grid_rows` extraída para `layout_grid_boxes(grid_boxes, align, column_gap, align_boundaries, style)`: aceita células medidas e marcas de limite `&` (que não levam `column_gap`); `layout_grid_rows` mede e delega com marcas vazias (largura calculada pela fórmula clássica nesse caso — comportamento anterior preservado).
- L0: `00_nucleo/prompts/engine/math/layout/matrix.md` (regra P825) + `_comum.md` (`layout_grid_boxes`).

### Validação (DEPOIS, literal — `mutool trace d1.c2.pdf`)

```text
CRISTALINO DEPOIS:  a @291.56  = @301.215  b @310.32
                    xxxx @271.76..291.56  = @301.215  y @310.32..   (= ALINHADOS ✓)
```

- `=` das duas linhas no mesmo x (301.215) ✓ — o achado central está fechado;
- coluna par alinhada à direita (tinta de `a` e de `xxxx` termina no mesmo ponto da grelha) ✓;
- gap do limite antes de `=` = THICK incorporado na célula par ✓ (teste unitário com valor exacto 5/18em);
- controlo: equação de duas linhas com `&` (`d5.typ`) inalterada (`=` alinhados em 299.687, como ANTES) ✓ sem regressão;
- resíduo ~0.5–1pt em posições absolutas face ao vanilla: avanços de glyphs math do cristalino (x adv 6.6 vs 6.29 etc. — divergência pré-existente de P809–P812, fora de âmbito).

Testes: `p825d_mat_align_relacoes_alinhadas_entre_linhas`, `p825d_mat_align_coluna_par_alinhada_a_direita`, `p825d_mat_align_spacing_de_classe_no_limite`, `p825d_mat_sem_align_mantem_colunas_centradas` — os 3 primeiros confirmados a falhar ANTES, todos a passar DEPOIS.

## Suítes (comando + contagem ANTES/DEPOIS)

| Suíte | ANTES | DEPOIS |
|---|---|---|
| `cargo test -p typst-core` | 4472 passed; 0 failed; 2 ignored | **4487 passed; 0 failed; 2 ignored** |
| `cargo test -p typst-infra` | 668 passed; 0 failed; 5 ignored | **668 passed; 0 failed; 5 ignored** (inalterado) |

4472 + 15 testes novos (5 p825a — 4 unitários + 1 e2e; 6 p825b; 4 p825d) = 4487 ✓ — a contagem bate com os testes declarados. Um teste pré-existente reescrito (`p782_field_access_bare_resolve_simbolo` — premissa refutada pelo vanilla, registada no comentário e no L0). Testes novos confirmados a falhar ANTES: 10 FAILED + compilação de asserções dependentes de código novo.

## Lint

`~/.cargo/bin/crystalline-lint --fix-hashes .` (5 L0 tocados: `structural.md`, `eval.md`, `matrix.md`, `_comum.md`, `spacing.md`) → hashes sincronizados, 0 drift warnings. `~/.cargo/bin/crystalline-lint .` → **6 warnings, todos V7 "prompt órfão" pré-existentes** (`eval/field-access.md`, `layout/enum_item.md`, `model/document.md`, `stdlib/layout.md`, `stdlib/structural.md`, `infra/package_version_resolution.md`) — o mesmo conjunto reportado em P822/P826/P813, nenhum V3/V4/V5/V13/V14. **Zero violations novas.** (Nota: `stdlib/structural.md` consta como órfão apesar de referenciado no header multi-`@prompt` de `structural.rs` — anomalia pré-existente da detecção, não introduzida nem agravada aqui.)

## Scope-outs e débitos (confirmados, não tentados)

- **Átomos "spaced" do vanilla** (fence + `class()` wrapped): sub-C formalizado no L0 `spacing.md` — não implementar sem decisão explícita (ligado à regra "spaced frames", P772y).
- **Equações simples com `&`** (fora de `mat`): o limite `&` também não incorpora o espaço de classe no caminho `layout_grid` de equações (Δ medido ≈ 0.56pt em `$ a &= b \ xxxx &= yy $`) — não sinalizado por P810; mesma mecânica do sub-D, candidato natural de follow-up.
- **Avanços de glyphs math** (x adv 6.6 vs 6.29 vanilla, etc.): divergência pré-existente (domínio P809/P811/P812), causa dos resíduos de posição absoluta no sub-D.
- **Spans `<detached>`** nas mensagens de cast de `class()`: convenção existente do módulo (o vanilla anexa span do argumento) — apresentação, mesma nota de P826.
- **Mensagens dos outros argumentos de `class()`** (body em falta, named inesperado) continuam em PT — convenção do módulo, fora do sub-A.
