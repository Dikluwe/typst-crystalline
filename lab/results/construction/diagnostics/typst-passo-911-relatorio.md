# Relatório — Passo 911: auditoria de `attach.rs`/`matrix.rs`/`cases.rs`/`delimited.rs`/`stretchy.rs`/`assembly.rs` contra a fórmula real do vanilla

**Data:** 2026-07-25
**Commit de partida:** `fe23ecb20` (P909 — "conclui atomizacao dos modulos accent, cancel, op e underover"). Árvore de
trabalho no início deste passo: limpa em `01_core`/`03_infra` (só `00_nucleo/materialization/typst-passo-911.md` +
3 PDFs de scratch de sessões anteriores, sem relação com este passo). Sem conflito com P906/P907/P908 (nenhum
tocou os seis ficheiros aqui auditados) nem com P909 (mexeu em `accent.rs`/`cancel.rs`/`op.rs`/`underover.rs`,
mesmo directório, ficheiros diferentes).

**Natureza do passo:** auditoria preventiva (P911 previa poder não encontrar nada). **Achado em todos os seis
módulos** — não é o resultado "nulo" previsto como bom-e-válido, mas três achados reais e confirmados (um deles
partilhado por três módulos). Nenhuma correcção aplicada nesta passagem — todos os achados são de dimensão
"grande" pelo critério da própria auditoria (mudam comportamento observável em produção, exigem TDD dedicado) —
destacados para passo(s) próprio(s), conforme o método do próprio P911 manda.

---

## Achado transversal A — `covering()` resolve caracteres comuns (`(`, `)`, `{`, `[`, `]`) para a fonte de
corpo, nunca a fonte MATH — variantes/assembly ficam sempre vazias

**Módulos afectados:** `delimited.rs`, `matrix.rs`, `cases.rs` (chamadores de `layout_stretchy_delimiter`);
mecanismo em `stretchy.rs` + `03_infra/src/font_metrics.rs::covering`.

### Medição (não inferência)

Instrumentação temporária (`eprintln!` em `matrix.rs:95` e `glyph_variants.rs::select_with_advance`, revertida
antes de terminar o passo — `git diff` confirmado vazio, suite recompilada e recorrida depois, 4778 passed / 0
failed, igual à contagem pré-passo) mediu, para `$ mat(1,2;3,4) $` vs `$ mat(1,2;3,4;5,6;7,8;9,10;11,12) $`:

```
min_height_du=2726   n_variants=0 advances=[]
min_height_du=9070   n_variants=0 advances=[]
```

`min_height_du` está correcto e cresce com o conteúdo (2726 → 9070) — a fórmula de conversão pt→DU não é a
causa. `n_variants=0` é a causa: `vertical_glyph_variants('(', ...)`/`vertical_glyph_assembly` devolvem sempre
listas vazias no pipeline real, apesar de `NewCMMath-Book.otf` ter 8 variantes verticais reais para `(`
(confirmado via `fontTools`: `parenleft` avança 997→2991 DU + assembly de 3 partes, `upem=1000`).

Causa exacta (`03_infra/src/font_metrics.rs:867-905`, `fn covering`): itera `primary` (lista de fontes
candidatas) **na ordem em que `resolve_primary_with_math_fallback` a construiu** — corpo primeiro
(`resolve_primary(style)`), fonte MATH só **acrescentada a seguir** (`primary.push(...)`, linha 852) — e devolve
a **primeira** face cujo `glyph_index(c)` exista (linha 875). Como `(`/`)`/`{`/`[`/`]` existem em praticamente
qualquer fonte de corpo, `covering()` acerta sempre a fonte de corpo — que não tem tabela MATH — **antes** de
sequer tentar a fonte MATH. `extract_variants`/`extract_assembly` (`font_metrics.rs:31-58,61+`) devolvem
`GlyphVariants::default()`/`GlyphAssembly::default()` assim que `face.tables().math` é `None` (linha 38/67).

Confirmado empiricamente por `mutool trace` (glyph ID real emitido no PDF, não `pdftotext` — per ADR-0119 ponto
3), comparando cristalino vs vanilla no mesmo `.typ`, mesma fonte confirmada (`mutool info` — ambos
`NewCMMath-Book`, `PAEJLA+`/`AAAAAA+` são só prefixos de subsetting):

| Caso | Cristalino (glyph, sempre) | Vanilla (glyph, cresce) |
|---|---|---|
| `(1/2)` | `glyph="1"` | `glyph="1"` |
| `(1/2/3/4)` | `glyph="1"` (**inalterado**) | `glyph="5"` |
| `(1/2/3/4/5/6/7/8)` | `glyph="1"` (**inalterado**) | `glyph="12"` (via `)`) |
| `mat(1,2;3,4)` | `glyph="1"`/`"2"` | `glyph="1"`/`"6"` |
| `mat(...)` 6 linhas | `glyph="1"`/`"2"` (**inalterado**) | `glyph="7"`/`"16"` |
| `cases(1,2)` | `glyph="9"` | `glyph="1"` |
| `cases(...)` 8 linhas | `glyph="9"` (**inalterado**) | `glyph="4"` |

Em **nenhum** dos seis casos o glifo do delimitador em cristalino muda com o conteúdo — confirma que o problema
é sistemático (não um limiar específico não atingido), consistente com `n_variants=0` medido directamente.

### Achado secundário no mesmo mecanismo — margem/fórmula do alvo (`min_height_du`) diverge da vanilla

Mesmo corrigindo o achado A (variantes deixarem de vir vazias), a fórmula do **alvo** já teria divergido da
vanilla nos três módulos:

- **`delimited.rs:26-31`**: usa `body_box.ascent + body_box.descent` (soma simples). Vanilla
  (`typst-layout/src/math/fenced.rs:84-103`, `relative_to_from_fragments`) usa, quando `balanced=true` — e
  `balanced` é **sempre `true`** para `MathDelimited`/`lr()` real (`typst-library/src/math/ir/resolve.rs:976`,
  único chamador de `FencedItem::create` com esse `balanced` para delimitadores de utilizador) —
  `2.0 * (ascent - axis).max(descent + axis)`: o alvo é o dobro da maior distância ao eixo matemático, não a
  soma bruta — maior sempre que o conteúdo não é simétrico em torno do eixo (o caso comum). `delimited.md` não
  documenta este termo — nunca foi auditado.
- **`matrix.rs:94-99`/`cases.rs:28-33`**: idêntica soma simples. Vanilla (`resolve.rs:1162-1188`,
  `resolve_delimiters`, usada por matrizes/vectores/cases — `balanced=false` aqui, correctamente **não**
  balanceado, então a soma simples está certa nesse aspecto) aplica **`target = 1.1 × relative_to`**
  (`Ratio::new(1.1)`, `resolve.rs:1171`) — 10% de margem extra que `matrix.rs`/`cases.rs` não aplicam.
- **`stretchy.rs`/`glyph_variants.rs::select_with_advance`**: vanilla subtrai **`DELIM_SHORT_FALL = 0.1em`**
  (`typst-library/src/math/lr.rs:17`) do alvo antes de comparar (`glyph.rs:268`, `short_target = target -
  short_fall`) — tolerância que evita escalar para a variante seguinte por uma margem irrisória. Cristalino
  compara `advance >= min_advance` sem short-fall nenhum.

Estes três desvios de fórmula são reais mas **secundários** ao achado A — hoje são inobserváveis em produção
porque a lista de variantes está sempre vazia (o alvo nunca chega a ser comparado a nada). Tornam-se relevantes
assim que o achado A for corrigido.

**Veredicto: achado confirmado, causa isolada, correcção NÃO aplicada neste passo** (é a correcção "grande" que
o próprio P905 já antecipava e destacava para "passo dedicado sobre `layout_stretchy_delimiter`" — este passo
generaliza esse achado de `√`/`sqrt` para **todos** os delimitadores comuns, incluindo `(`/`)`/`{`/`[`/`]`, com
causa raiz agora isolada em `covering()`, não em `layout_stretchy_delimiter` em si). Candidato a passo dedicado:
(1) preferir a fonte MATH sobre a de corpo quando `style.math` e o glyph tem tabela MATH, antes de aceitar a
primeira cobertura em `covering()`; (2) somar os três desvios de fórmula acima (`balanced` em `delimited.rs`,
`1.1×` em `matrix.rs`/`cases.rs`, `short_fall` em `select_with_advance`); (3) teste novo com métricas reais (não
`FixedMetrics`) exercitando variantes não-vazias ponta-a-ponta — **gap de cobertura confirmado**: todos os
testes existentes de `vertical_glyph_variants`/`layout_stretchy_delimiter` usam `FixedMetrics`, que devolve
sempre lista vazia (`fixed_metrics_sem_variantes_vertextuais`) — o caminho com variantes reais nunca foi
exercitado por teste algum.

---

## Achado transversal B — `assembly.rs::layout_assembly` nunca repete peças extensoras; `_target_advance` é
parâmetro morto

**Módulo:** `assembly.rs`.

`GlyphPart.is_extender` existe no modelo de dados (`glyph_variants.rs:69`, doc comment: "se true, esta peça pode
ser repetida para preencher altura") e `GlyphAssembly::min_advance()` já assume "sem repetição de extensores"
no próprio nome/comentário (`glyph_variants.rs:86-91`) — mas **nenhuma função no cristalino chega a repetir uma
peça**. `layout_assembly` (`assembly.rs:17-77`) itera `assembly.parts` **exactamente uma vez cada**, empilhando
com sobreposição de conectores — e o parâmetro que receberia a altura-alvo, `_target_advance`, está
**explicitamente marcado como não usado** (prefixo `_`) na própria assinatura da função (linha 21). O resultado:
a altura montada é sempre a mesma (soma fixa de peças menos sobreposições), **qualquer que seja o alvo pedido**.

Vanilla (`typst-layout/src/math/fragment/glyph.rs:567-660`, `fn assemble`) faz precisamente o oposto: um `loop`
com contador `repeat` (linhas 580-627) que **testa repetidamente** quantas cópias das peças extensoras (via
`parts(assembly, repeat)`) são precisas até `full >= target` (ou `repeat >= MAX_REPEATS`), com um `ratio` de
espalhamento entre sobreposição máxima e mínima para preencher o alvo com precisão sem repetição extra.
Repetir extensoras é o mecanismo central de "delimitador de altura arbitrária" do MATH table do OpenType — sem
ele, `layout_assembly` só pode produzir UMA altura fixa, nunca escala para conteúdo maior que o alcançado por
essa única passagem pelas peças (tipicamente maior que a maior variante pronta, mas ainda finito e fixo).

**Veredicto: achado confirmado por leitura de código (não precisa de medição empírica adicional — o parâmetro
morto já é a prova; a assinatura da função admite a lacuna)**. Consequência prática: mesmo corrigindo o Achado A
(`covering()`), qualquer conteúdo alto o suficiente para exigir montagem por partes (matrizes muito grandes,
radicais muito altos) ainda receberia um delimitador de altura errada — só desta vez fixa-mas-maior-que-base, em
vez de sempre-base. Candidato a passo dedicado (pode ser o mesmo do Achado A, já que ambos vivem na mesma cadeia
`layout_stretchy_delimiter → variantes vazias → assembly`): implementar o `loop`/`repeat` sobre peças
`is_extender`, com teste TDD usando `StubMetrics`/dados configuráveis (mesmo padrão de P906,
`StubHorizontalMetrics`) para exercitar deterministicamente sem depender do achado A estar corrigido primeiro.

---

## Achado — `attach.rs`: offsets verticais de sub/sobrescrito são constantes fixas; vanilla computa valor
adaptativo (cramped, extremos da base, gap mínimo simultâneo)

**Módulo:** `attach.rs` (braço não-`is_limits`, scripts laterais — a maioria dos casos reais).

`sup_offset`/`sub_offset` (`attach.rs:40-47`) são **sempre** `constants.superscript_shift_up`/
`subscript_shift_down` — a mesma constante do tipo de letra, independente de qualquer coisa sobre a base ou os
scripts em si. Vanilla (`typst-layout/src/math/scripts.rs:318-382`, `compute_script_shifts`) computa
`shift_up`/`shift_down` como o **máximo** de quatro/três termos cada:

- `shift_up = max(sup_shift_up_ou_cramped, ascent-sup_drop_max [se não text-like], sup_bottom_min+tl.descent,
  sup_bottom_min+tr.descent)` — `superscript_shift_up_cramped` é escolhida em vez de `superscript_shift_up`
  quando `EquationElem::cramped` está activo (linhas 325-330) — cristalino não tem conceito de "cramped" em
  `attach.rs` nenhum.
- `shift_down` — análogo, com `subscript_shift_down`/`descent+sub_drop_min`/`bl.ascent-sub_top_max`/
  `br.ascent-sub_top_max`.
- **Interacção sup+sub simultâneos** (linhas 363-379): se AMBOS existem, calcula o gap real entre eles
  (`sup_bottom - sub_top`) e, se menor que `sub_superscript_gap_min`, **aumenta** `shift_up`/`shift_down` para
  garantir esse mínimo — mecanismo ausente por completo em `attach.rs` (cada offset é calculado isoladamente,
  nunca em função do outro).

`attach.md` (P799) documenta explicitamente que manteve "a geometria vertical existente (offsets fixos...)"
como estava — mas não documenta/decide a divergência acima; é lacuna de auditoria, não decisão consciente.

Kerning (quadrantes, já parcialmente coberto por P891): `attach.rs:220-249` usa só `base_kern.<corner>.kern_at`
com **uma** altura de correcção (`sub_box.ascent`/`sup_box.ascent`, sem ajustar pelo shift). Vanilla
(`scripts.rs:389-425`, `fn math_kern`) calcula **duas** alturas de correcção (`corr_height_top`/
`corr_height_bot`, com o shift subtraído) e **soma** o kern da BASE com o kern do PRÓPRIO SCRIPT
(`script.kern_at_height(pos.inv(), height)`, linha 415) para cada uma, tomando o maior dos dois somados
(linha 424) — cristalino só lê a tabela da base, nunca a do script, e só uma altura.

**Veredicto: achado confirmado por leitura de código, termo a termo (`file:line` dos dois lados acima)**.
Geometria tipográfica (ADR-0123), não mecânica — afecta posição visível de qualquer `x^2_i` com base
não-trivial ou scripts simultâneos próximos. Não corrigido aqui (achado grande — implementar
`compute_script_shifts` completo, incluindo `cramped`, é trabalho de TDD dedicado, mesmo padrão dos passos
anteriores desta frente). Candidato a passo dedicado.

---

## Resumo por módulo (ordem do método)

| # | Módulo | `file:line` cristalino | `file:line` vanilla | Resultado |
|---|---|---|---|---|
| 1 | `delimited.rs` | `delimited.rs:16-38` | `fenced.rs:11-103` | Achado A (parte: `balanced` ausente) |
| 2 | `matrix.rs` | `matrix.rs:93-101` | `resolve.rs:1162-1188` | Achado A (partilhado) + margem `1.1×` ausente |
| 3 | `cases.rs` | `cases.rs:28-35` | `resolve.rs:1162-1188` | Achado A (partilhado) + margem `1.1×` ausente |
| 4 | `attach.rs` | `attach.rs:40-47,220-249` | `scripts.rs:318-425` | Achado próprio — offsets/kern não-adaptativos |
| 5 | `stretchy.rs` | `stretchy.rs:16-60`, `glyph_variants.rs:34-39` | `glyph.rs:265-310`, `lr.rs:17` | Achado A (mecanismo/causa raiz) + `short_fall` ausente |
| 6 | `assembly.rs` | `assembly.rs:17-77` | `glyph.rs:567-660` | Achado B — sem repetição de `is_extender` |

Nenhum dos seis módulos ficou "auditado, sem achado" — resultado diferente do previsto como possível pelo
próprio texto do passo, mas dentro do espaço de resultados que ele antecipa ("pode encontrar", achados de causa
confirmada, não inventados).

## Fase B — não aplicada

Nenhum achado foi decidido para correcção dentro deste passo (todos são "achado grande" pelo critério do
próprio P911 — mudam fórmula/comportamento de produção, precisam de TDD dedicado e, no caso do Achado A,
decisão de produto sobre a ordem de preferência fonte-de-corpo vs fonte-MATH em `covering()`).

## Fase C — Regressão

Não aplicável — nenhuma correcção de produção nesta passagem (regra do próprio P911: só corre se algo foi
corrigido na Fase B). Confirmado sem alteração de código de produção: `git diff --stat` contra `fe23ecb20`
mostra zero ficheiros de `01_core`/`03_infra` alterados; suite completa recorrida no fim (`typst-core`: 4778
passed, 0 failed, 3 ignored — igual à contagem antes da instrumentação de debug).

## `crystalline-lint`

`crystalline-lint .`: só o warning V7 pré-existente e não relacionado
(`00_nucleo/prompts/infra/package_version_resolution.md`). Sem drift novo (nenhum `.rs` de produção alterado).

---

## Resumo executivo (tabela final)

| Item | Veredicto | Estado |
|---|---|---|
| Achado A — `covering()` prefere fonte de corpo para `(`/`)`/`{`/`[`/`]`, variantes sempre vazias | Confirmado por `eprintln` + `mutool trace`, causa isolada (`font_metrics.rs:867-905`) | ⚠️ Destacado para passo dedicado |
| Achado A (secundário) — `balanced` ausente (`delimited.rs`), `1.1×` ausente (`matrix.rs`/`cases.rs`), `short_fall` ausente (`select_with_advance`) | Confirmado por leitura `file:line`, inobservável até Achado A ser corrigido | ⚠️ Destacado (mesmo passo dedicado) |
| Achado B — `assembly.rs` não repete `is_extender`, `_target_advance` morto | Confirmado por leitura de código (parâmetro não usado é a prova) | ⚠️ Destacado para passo dedicado |
| Achado — `attach.rs` offsets fixos vs fórmula adaptativa da vanilla (cramped, extremos, gap simultâneo) | Confirmado por leitura `file:line` termo a termo | ⚠️ Destacado para passo dedicado |
| Gap de cobertura de teste — nenhum teste exercita `vertical_glyph_variants`/`_assembly` com dados não-vazios (todos usam `FixedMetrics`) | Confirmado (grep dos testes existentes) | ⚠️ Registado para o mesmo passo dedicado |
| Regressão | N/A — nenhuma correcção aplicada | ✅ Suite inalterada (4778/0/3) |
