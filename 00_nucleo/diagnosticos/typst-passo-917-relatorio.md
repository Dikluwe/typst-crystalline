# Relatório P917 — causa real do "delimitador não cresce", reexecução de P912/913/914, e Fase B de P893 (nunca aprovada)

**Commit de referência (working tree no início e no fim desta sessão):** `a39da04dc` (working
tree modificada, não commitada — ver `git diff --stat` na secção final; nenhum commit foi feito
durante este passo).
**Baseline de testes (antes de qualquer alteração):** typst-core: 4783 · typst-infra: 740 ·
typst-shell: 41 (herdado de P916).
**Baseline de testes (fim deste passo):** typst-core: 4786 · typst-infra: 743 · typst-shell: 41 ·
typst-wiring (bin): 2 · cli: 37 · crystalline_lint: 2 — todos `0 failed`.
**Data:** 2026-07-26.

---

## Fase A — diagnóstico do achado aberto de P916 ("delimitador fixo")

**Precedente**: `typst-passo-916-relatorio.md`, achado aberto — `(1/2)`, `(1/2/3/4)` etc. mediam
sempre o mesmo glyph ID no `mutool trace`, interpretado como "o glifo não cresce apesar de P912".
As três hipóteses do `typst-passo-917.md` original eram: lista de variantes vazia, fórmula do alvo
insuficiente, ou ordem de decisão variante-vs-assembly errada.

**Método**: instrumentação directa (`eprintln!` temporário em `layout_stretchy_delimiter`,
revertido antes da Fase B) com a fonte real do pipeline (`NewCMMath-Regular.otf`, embutida via
`typst_assets`, mesmo `rev` pinado em `Cargo.toml`) e o binário de produção
(`cargo build -p typst-wiring --bin typst`), compilando os 4 casos fixos de P911/P916.

**Nenhuma das três hipóteses se confirmou**:

| Caso | `target_du` | `select_with_advance` |
|---|---:|---|
| `(1/2)` | 1662.00 | `Some((6612, 1793.0))` |
| `(1/2/3/4)` | 2402.18 | `Some((6678, 2991.0))` |
| `(1/2/3/4/5/6/7/8)` | 2942.59 | `Some((6678, 2991.0))` — tecto das 8 variantes |
| `mat(1,2;3,4)` | 2898.60 | `Some((6678, 2991.0))` |

`vertical_glyph_variants` devolvia 8 variantes (não vazia); a variante seleccionada crescia
correctamente com o alvo. A comparação de "glyph ID igual em todos os casos" de P916 era um
artefacto de comparar índices de glifo **locais ao subset** entre PDFs independentes — cada
compilação gera o seu próprio subset renumerado; glyph IDs originais confirmados diferentes
(`6612` vs `6678`) via `ttf_parser` directo e cross-check com `fontTools` na mesma fonte
(`NewCMMath-Regular.otf`, `~/.cargo/git/checkouts/typst-assets-.../files/fonts/`).

**Causa real** (medida, não hipótese): no ramo "sem mapeamento — emitir como Glyph" de
`layout_stretchy_delimiter`, `x_advance` (e por extensão `MathBox.width`) vinha de
`variants.select_with_advance(target_du).1` — o campo `advance` de `GlyphVariant`, que é a medida
ao longo do **eixo de esticamento** (altura, para construções verticais; `AdvanceMeasurement` da
tabela MATH). Confirmado quantitativamente: para a variante seleccionada em `(1/2)`
(`parenleft.v4`, glyph 6612 na fonte real), `advance` = 1793 du (→ 19.72pt a 11pt — usado
indevidamente como largura) vs. avanço horizontal nativo real (`hmtx`, via `fontTools`) = 597 du
(→ 6.57pt — o valor correcto). Confirmado uma terceira vez pelo `/Widths` da fonte embutida no PDF
exportado (`mutool trace` → `adv=".597"`). Mesma classe de bug em `assembly.rs` (`full_advance`,
medida do eixo de empilhamento, reaproveitada como `x_advance`/largura das peças). O vanilla
(`glyph.rs:293`, `typst-layout/src/math/fragment/glyph.rs`) sempre usa `font.x_advance(glyph_id)`
— o avanço nativo do glifo — nunca a medida do eixo de esticamento.

**Efeito visível**: gap enorme entre o delimitador esticado e o conteúdo seguinte (confirmado
visualmente, `mutool draw -r 300`), não falha de crescimento vertical (que já funcionava desde
P912).

## Fase B — implementação

**Desenho**: `GlyphVariant`/`GlyphPart` (`01_core/src/entities/glyph_variants.rs`) ganham o campo
`hor_advance: f64` (avanço horizontal nativo, `hmtx`), distinto de `advance`/`full_advance` (eixo
de esticamento/empilhamento, só para `select`/`select_with_advance`/cálculo de posição). Populado
em `extract_variants`/`extract_assembly`/`extract_variants_horizontal`/`extract_assembly_horizontal`
(`03_infra/src/font_metrics.rs`) via `face.glyph_hor_advance(glyph_id)`. Novo método
`GlyphVariants::select_variant(min_advance) -> Option<&GlyphVariant>` (devolve a variante completa,
não só `(glyph_id, advance)`). `stretchy.rs`/`assembly.rs` passam a usar `hor_advance` para
`x_advance`/`MathBox.width`/largura das peças — `advance`/`full_advance` continuam a determinar
altura/posição/decisão de tamanho, inalterados.

**L0 actualizados antes do código** (Trava Arquitetural, protocolo respeitado): `entities/
glyph_variants.md`, `infra/font_metrics.md`, `engine/math/layout/stretchy.md`, `engine/math/
layout/assembly.md` — secções `§P917` novas em cada um. Hashes sincronizados
(`crystalline-lint --fix-hashes .`, 4 ficheiros `.rs` actualizados) antes de qualquer edição de
`.rs`.

**TDD**: teste novo `p917_stretchy_delimiter_largura_usa_hor_advance_nao_advance`
(`01_core/src/engine/math/layout/tests.rs`), dados sintéticos extremos (`advance=9000`,
`hor_advance=40`) para que uma regressão futura falhe alto e claro. Confirmado vermelho antes da
correcção (`obteve 108.0000`, esperado `0.48`), verde depois.

**Confirmação visual** (`(1/2)`, `mutool draw -r 300`, comparado ao vanilla `typst 0.15.1`): antes
da correcção, gap grande entre `(` e o conteúdo; depois, parêntese hug o conteúdo, visualmente
equivalente ao vanilla. Casos `c5`/`c7` (assembly, 6+ linhas) continuam com um achado **pré-
existente e não relacionado**, confirmado idêntico antes/depois desta correcção (ver secção
"Achados registados, não corrigidos" abaixo).

## Fase C — regressão

Ficheiros canónicos de 7 cenários de passos anteriores (`01-hello` … `07-context`) não estavam
presentes no repositório (eram recriados por sessão, nunca commitados) — reconstruídos
equivalentes nas mesmas 7 categorias, mais `04-math` cobrindo fracção aninhada + matriz + attach +
sqrt + cases. Binários `before`/`after` (release) da mesma sessão, `hyperfine --warmup 5 -N -m 20`:

| Cenário | Razão (after vs before) |
|---|---|
| 01-hello | 1.00–1.02× |
| 02-lorem | 1.01× |
| 03-images | 1.00× |
| 04-math | 1.01× |
| 05-tables | 1.00× |
| 06-long | 1.00× |
| 07-context | 1.01× |

Todos dentro do ruído de `hyperfine` — **nenhuma regressão**.

---

## Reexecução de P912/913/914 — auditoria em vez de reescrita

O pedido original era "reexecutar do zero". Auditoria feita primeiro (ADR-0108, medir antes de
decidir): `covering()` (P912) e as 3 correcções de fórmula secundárias (`delimited.rs`,
`matrix.rs`/`cases.rs`, `DELIM_SHORT_FALL`) confirmados correctos por instrumentação ao vivo (Fase
A acima). O algoritmo de repetição de extensores (P913, `assembly.rs`) confirmado correcto — loop
`repeat`/`ratio` já implementado, testes síncronos já passam, `MAX_REPEATS` respeitado. `attach.rs`
(P914) já tinha sido revisto cepticamente linha-a-linha por P916 Parte D, sem achados novos.

**Decisão tomada com o dono** (apresentada a evidência, escolhida a opção "fechar o gap real" em
vez de "redo literal do zero"): o que realmente faltava era a cobertura de fonte real que o
próprio L0 de P912 exigia ("testes com métricas reais, não `FixedMetrics`") e nunca foi entregue —
confirmado por grep: só existia 1 teste de fonte real em todo o `03_infra` (`math_kern`), nenhum
para `vertical_glyph_variants`/`vertical_glyph_assembly`/`math_constants`. Essa lacuna é
precisamente o que permitiu ao bug de P917 sobreviver indetectado pela suíte automática.

**3 testes novos** (`03_infra/src/font_metrics.rs`), todos com ground truth calculado via
`ttf_parser` directamente no próprio teste (não hardcoded, mesma disciplina de P891):

- `p912_fallback_font_metrics_vertical_glyph_variants_le_tabela_math_real` — variantes verticais de
  `(` batem byte-a-byte (glyph_id, advance, **hor_advance**) com a tabela MATH real.
- `p913_fallback_font_metrics_vertical_glyph_assembly_le_tabela_math_real` — peças do assembly
  vertical de `(` batem byte-a-byte com a tabela MATH real.
- `p914_fallback_font_metrics_math_constants_le_tabela_math_real` — ver achado de P893 abaixo (este
  teste começou vermelho e revelou o achado maior desta sessão).

---

## Achado maior, fora do pedido original — Fase B de P893 (diagnosticada, nunca aprovada)

Ao escrever o teste de fonte real para P914 (dependência directa: `attach.rs` consome
`MathConstants` via `self.constants`, cacheado uma vez em `MathLayouter::new`), descobri que
`typst-passo-893-relatorio.md` já tinha **medido, desenhado a correcção completa, escrito e
sincronizado o hash dos 4 L0s necessários, e parado explicitamente à espera de confirmação do
dono** ("STOP — aguardando confirmação do dono do projecto") — confirmação essa nunca dada. Sem
essa Fase B, `FallbackFontMetrics::math_constants` (a única implementação usada em produção)
**nunca lia a tabela MATH real**, devolvendo sempre `MathConstants::fallback()` (valores baseados
em STIX Two Math) independentemente da fonte carregada.

**Confirmado nesta sessão, duas formas independentes**:

1. Teste `p914_...` (acima): antes da correcção, `362.0 != 363.0` (fallback vs real,
   `superscript_shift_up`).
2. Binário de produção instrumentado (`eprintln!` temporário em `MathLayouter::new`, revertido):
   compilando `(1/2)`, `axis_height=500` (fallback) — não `250` (valor real de
   `NewCMMath-Regular.otf`, confirmado via `fontTools`).

| Constante | Fallback (STIX) | Real (NewCMMath-Regular) | Divergência |
|---|---:|---:|---:|
| `axis_height` | 500 | 250 | **-50% (2×)** |
| `subscript_shift_down` | 130 | 247 | **+90%** |
| `upper_limit_gap_min` | 100 | 200 | **+100% (2×)** |
| `fraction_rule_thickness` | 66 | 40 | -39% |

Tabela completa das 13 constantes já em `typst-passo-893-relatorio.md`; valores acima
reconfirmados nesta sessão, não copiados sem verificar.

**Apresentado ao dono com a evidência acima; aprovada a implementação da Fase B.**

**Implementação** (design já fixado pelos 4 L0s de P893, hash já sincronizado — nenhum L0 novo
necessário, só Fase B):

- `FontMetrics::math_constants(&self) -> MathConstants` ganha `style: &TextStyle`.
- `FallbackFontMetrics::math_constants(&self, style)`: resolve `primary` (mesma cadeia de
  `math_kern`) e usa a **primeira face com tabela MATH presente** (não `primary.first()` — no caso
  comum a primeira candidata é a fonte de corpo genérica sem MATH). Sem candidato: fallback,
  comportamento inalterado.
- `FontBookMetrics::math_constants` aceita e ignora `style` (face única).
- Lógica de leitura extraída para função livre partilhada `math_constants_from_face` (mesmo padrão
  de `math_kern_from_face`, P891).
- `MathLayouter::new(metrics, block, style)` ganha o terceiro parâmetro; único call site de
  produção (`equation.rs:62`) passa `&math_style`; ~66 call sites de teste em `tests.rs`
  actualizados mecanicamente (`&default_style()` — `FixedMetrics`/`StubHorizontalMetrics` não
  sobrepõem `math_constants`, o valor de `style` não afecta o resultado desses testes).
- `impl FontMetrics for &dyn FontMetrics` **não** reencaminha `math_constants` — decisão já
  registada em P893 (mesma situação de `math_kern`, caminho de produção usa o tipo concreto).

**Suíte completa verde após a mudança** (contagem no topo deste relatório). Benchmark: mesma tabela
da Fase C acima (medido em conjunto, mesmo binário `after`) — sem regressão.

**Confirmação visual**: `x^2_i + (1/2)` a 20pt, comparado ao vanilla `typst 0.15.1` — melhora visível
face ao binário anterior, mas revelou um achado adicional pré-existente (ver abaixo).

---

## Achado registado, não corrigido (fora de âmbito desta sessão)

**Desalinhamento vertical entre elementos adjacentes numa sequência matemática** — `x^2_i +
(1/2)`: o grupo `(1/2)` aparece desalinhado verticalmente em relação a `x^2_i +` (vanilla mantém
tudo numa banda horizontal consistente). **Confirmado pré-existente**: renderizado idêntico com o
binário anterior a qualquer correcção desta sessão (`git stash` + rebuild), portanto não é
regressão de P917 nem de P893 — só ficou mais fácil de notar sem o gap de P917 a dominar
visualmente. Hipótese não confirmada: `apply_axis_offset` recentra cada `MathBox` (attach,
delimited) independentemente antes de `hconcat`, sem eixo comum entre irmãos numa sequência.
Candidato a passo dedicado futuro — não instrumentado nem diagnosticado nesta sessão.

**Assembly não atinge a altura total pedida em matrizes/casos de 6+ linhas** (`c5_mat6l`,
`c7_cases8`) — também confirmado pré-existente (idêntico antes/depois de P917), já registado como
"caminho estruturalmente diferente do vanilla" em P916/`typst-passo-917.md` original. Não
investigado nesta sessão (fora do escopo do achado de P917, que era sobre largura, não altura de
assembly).

---

## Resultado final

| Item | Estado |
|---|---|
| P917 Fase A — causa real (não as 3 hipóteses do L0 original) | ✅ Confirmada por instrumentação |
| P917 Fase B — `hor_advance` em `GlyphVariant`/`GlyphPart`, consumido em `stretchy.rs`/`assembly.rs` | ✅ Implementado, TDD, L0 actualizado antes do código |
| P917 Fase C — benchmark 7 cenários | ✅ Sem regressão |
| P912 — covering()/fórmulas | ✅ Já correcto (confirmado, não reescrito) |
| P913 — repetição de extensores | ✅ Já correcto (confirmado, não reescrito) |
| P914 — attach.rs | ✅ Já correcto (P916 Parte D) |
| P912/913/914 — gap de cobertura de fonte real | ✅ Fechado — 3 testes novos |
| P893 Fase B — `math_constants` real (nunca aprovada até agora) | ✅ Implementado, testado, com aprovação explícita do dono |
| Desalinhamento entre elementos math adjacentes | ⚠️ Registado, pré-existente, não corrigido |
| Assembly não atinge altura em matrizes 6+ linhas | ⚠️ Registado, pré-existente, não corrigido |
| Suíte completa verde | ✅ core 4786 · infra 743 · shell 41 · wiring 2 · cli 37 · lint 2, `0 failed` |
| `crystalline-lint .` | ✅ 0 novo (só V7 pré-existente, não relacionado) |
| Commit | ❌ Não commitado — decisão do dono |

### Proveniência

- Commit base: `a39da04dc` (`git log -1`), working tree modificada (13 ficheiros, `git diff
  --stat`: `882 insertions(+), 169 deletions(-)`), nenhum commit feito nesta sessão.
- Fontes usadas para ground truth: `NewCMMath-Regular.otf` embutida via `typst_assets`
  (`~/.cargo/git/checkouts/typst-assets-525e6d15ef7950cb/c0ae970/files/fonts/`, mesmo `rev` pinado
  em `Cargo.toml`) — não `03_infra/fixtures/fonts/NewCMMath-Book.otf` (ficheiro diferente, glyph
  IDs diferentes; confundir os dois foi um erro intermédio desta sessão, corrigido antes de
  qualquer conclusão).
- Todos os números de teste/benchmark reproduzíveis via `cargo test --workspace` e
  `tools/perf`-style `hyperfine --warmup 5 -N -m 20` sobre os binários release da working tree
  actual vs. `git stash` (estado pré-sessão).
