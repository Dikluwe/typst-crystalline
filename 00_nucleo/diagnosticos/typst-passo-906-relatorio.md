# Relatório — Passo 906: mecanismo de esticamento horizontal de glifo

**Data:** 2026-07-25
**Commit de partida:** `e91d65b9e` (P905)

---

## Resumo executivo

O pedido original — construir o análogo horizontal de `vertical_glyph_variants`/
`vertical_glyph_assembly` para `underbrace`/`overbrace`/`underbracket`/`overbracket` e acentos
largos — foi cumprido e está **confirmado visualmente em PDF real**, casos isolados batendo com o
vanilla. Mas confirmar isso exigiu descer por uma cadeia de **5 bugs pré-existentes, nunca antes
exercitados com dados reais**, em camadas cada vez mais profundas do pipeline (métricas → export →
selecção de fontes → composição de layout). Nenhum foi introduzido por este passo — todos ficaram
latentes porque o próprio mecanismo de esticamento de glifo (ambos os eixos) nunca tinha sido
correctamente exercitado em produção. Cada descoberta foi confirmada por medição directa (PDF real,
`pdftotext`/instrumentação), nunca por inferência, e o dono foi consultado em 3 pontos de decisão
antes de expandir o âmbito.

## Fase A — desenho (confirmado com o dono antes da Fase B)

**Vanilla**: mecanismo único, genérico por eixo (`GlyphFragment::stretch(engine, target, short_fall,
axis)`), lê `MathVariants.horizontal_constructions`/`.vertical_constructions` (campos simétricos da
tabela OpenType MATH). `underbrace`/`overbrace` não são casos especiais — resolvidos como um accent
esticado no eixo X, mesmo caminho dos acentos largos.

**Dados da fonte** (NewCMMath-Regular, via `fontTools`, não inferidos): `⏟`/`⏞`/`⎵`/`⎴`/`⏝`/`⏜`
(chaves/colchetes/parênteses de underbrace/overbrace/underbracket/overbracket/underparen/overparen)
têm 8 variantes + assembly de 5 partes cada. `hat`/`tilde` combinantes (U+0302/U+0303) têm 8
variantes, sem assembly.

**Decisões confirmadas com o dono**:
1. Design do trait: **aditivo** (`horizontal_glyph_variants`/`horizontal_glyph_assembly`, dois
   métodos novos, zero mudança aos verticais existentes) — não unificar por parâmetro `axis`.
   Desejo de unificar registado para o futuro, não implementado agora.
2. Alcance: chaves/colchetes **+** acentos largos (hat/tilde sobre base larga) — não só chaves.
3. `underbrace`/`overbrace`/`underbracket`/`overbracket` não existiam como funções em cristalino
   (só o genérico `underover(base, under:, over:)`, P297) — incluídas neste passo, sem elas a
   confirmação visual pedida pelo próprio passo não teria como acontecer.

## Fase B — protocolo de dois agentes

**Agente A** escreveu 15 testes (`p906_*`) contra scaffolds com corpos placeholder deliberados —
incluindo um test double novo (`StubHorizontalMetrics`, dados configuráveis, já que `FixedMetrics`
só devolve vazio) para exercitar a lógica de selecção de variante/composição de assembly de forma
determinística. 10 vermelhos confirmados, 5 verdes de regressão (comportamento inalterado). Ficheiros
tocados: só `math/layout/tests.rs` e `eval/tests.rs` — nenhum ficheiro de produção.

**Agente B** implementou contra os 15 testes: `extract_variants_horizontal`/
`extract_assembly_horizontal` (03_infra, espelham as verticais trocando `.horizontal_constructions`),
`layout_assembly_horizontal` (empilha em X em vez de Y), `layout_stretchy_glyph_horizontal` (espelha
`layout_stretchy_delimiter`), guard de 1-carácter partilhado (`layout_stretchy_or_node`) em
`layout_underover`/`layout_accent`, e o braço de dispatch `underbrace`/`overbrace`/`underbracket`/
`overbracket` em `eval/math.rs` (aninhamento de 2 `MathUnderover` quando há anotação). 15/15 verdes,
suite completa sem regressão. Revisão do orquestrador confirmou os diffs linha a linha antes de
prosseguir.

## Fase C — a cadeia de 5 bugs pré-existentes (confirmação visual)

Ao confirmar visualmente `underbracket(a+b+c)` em PDF real, o glifo aparecia invisível/`.notdef`,
apesar do layout (largura, posição) estar correcto e testado. Investigar isso, com autorização
explícita do dono em 2 pontos de checkpoint, revelou:

### 1. `FallbackFontMetrics` nunca implementava `vertical_glyph_variants`/`vertical_glyph_assembly`

Gap já confirmado e deliberadamente adiado em P891/P893 (`infra/font_metrics.md`), nunca corrigido.
Como `FallbackFontMetrics` é a **única** implementação usada no pipeline real
(`pipeline.rs:126`), **todo** o esticamento de glifo — vertical incluído, não só o mecanismo
horizontal novo — estava silenciosamente inactivo em PDFs reais, apesar dos testes unitários
sempre terem passado (usam `FontBookMetrics`/stubs directamente). Corrigido com o mesmo mecanismo
já usado por `math_kern` (P891): `resolve_primary_with_math_fallback` + `covering` + as funções de
extracção. `&dyn FontMetrics` (delegação P858) tinha o mesmo gap, corrigido em paralelo.

### 2. `emit_glyph_pdf` (export PDF) — `/F1` hardcoded + remap de subsetting em falta

Dois bugs em cadeia, só visíveis um depois do outro: (a) `FontScenario::Multifont` sempre escrevia
`/F1 Tf`, mesmo quando o glifo vinha de uma fonte diferente da primeira embutida; (b) nem Cidfont
nem Multifont aplicavam o remap de subsetting (P516) ao `glyph_id` — desenhava sempre o índice
original, não o renumerado da fonte subsetada embutida. Corrigido reaproveitando
`per_font_glyph_reverse` (mesmo `build_math_glyph_reverse_map` já usado no subsetting DEBT-9/P45)
para (a), e `remap_glyph_id` já usado no caminho de texto para (b).

### 3. `collect_fonts_in_items` (selecção de fontes) cego a `FrameItem::Glyph`

A escolha de QUAIS fontes embutir (Cidfont de 1 vs Multifont de várias) é feita cedo no pipeline, a
partir de caracteres vistos em `Text`/`TextShaped` — o braço `FrameItem::Glyph { .. } => {}`
ignorava por completo qualquer glifo de esticamento. Uma equação cujo único conteúdo a precisar da
fonte MATH fosse um glifo de esticamento (sem mais texto itálico matemático à volta — isolado pelo
achado #5 abaixo) escolhia só a fonte de corpo como candidata única. Corrigido com um novo campo em
`FrameItem::Glyph` (`base_char: char`, o carácter original pedido ao esticamento — não o `glyph_id`
resultante, que só é interpretável dentro da fonte de onde veio) e um novo método
`FallbackFontMetrics::resolve_font_combo(c, style)` que devolve a identidade da fonte via
`covering`/`resolve_primary_with_math_fallback`.

### 4. `layout_underover`/`layout_accent` — convenção "topo do box"

Mesmo padrão já corrigido em `frac.rs` (P905) e `root.rs` (P901), nunca antes auditado aqui. Só
visível ao aninhar `MathUnderover` (caso `underbrace`/`overbrace` **com** anotação — estrutura
aninhada de 2 níveis): funcionava por coincidência quando a caixa era o conteúdo de topo da equação
(P899 Parte A, `hat(a)` sozinho), quebrava ao ser usada como sub-caixa de outra. Corrigido com a
mesma técnica: offsets calculados a partir da baseline própria de cada sub-caixa, não do topo.

### 5. `apply_math_default` não recursa em `Content::MathUnderover` (achado incidental, não corrigido)

Confirmado mas **fora de âmbito** — `apply_math_default` (aplica itálico por omissão a
identificadores de 1 letra) não tem braço para `MathUnderover`, cai no catch-all `other =>
other.clone()`. `base`/`under`/`over` nunca recebem o tratamento itálico — por isso os testes
visuais deste passo mostram "a+b+c" em vez de "𝑎+𝑏+𝑐" dentro de `underbrace(...)`. Pré-existente,
não introduzido por P906, registado para passo dedicado.

## Achados novos, fora de âmbito (registados, não corrigidos)

1. **Resolução de face para caracteres em múltiplas fontes candidatas**: `covering()` pode preferir
   a fonte de corpo (sem tabela MATH) mesmo em contexto math para caracteres comuns (ex.
   parênteses), se `resolve_primary` já a encontrar antes do fallback matemático ser consultado.
   Achado incidental, não confirmado em profundidade.
2. **`hat(a+b)`/`tilde(a+b)` produzem largura zero**: o ramo "variante encontrada + `glyph_to_char`
   mapeado" de `layout_stretchy_glyph_horizontal` mapeia o glyph_id da variante GRANDE de volta ao
   MESMO `base_char` pequeno e renderiza via `layout_text_node`, que remede o avanço NATURAL do
   char base (zero, para combining marks) — descarta a largura/glifo da variante encontrada.
   Confirmado por medição directa (`pdftotext -bbox`, largura zero). **Consequência em cascata
   confirmada**: numa sequência de equações de bloco, uma equação com este bug (altura/largura
   subestimada) pode comprimir o espaçamento vertical das equações seguintes na mesma página —
   reproduzido isolando `underbrace(...)` sozinho (correcto) vs precedido por `hat(x)`/`tilde(x)`/
   `vec(x)` (aparência degradada só nesse segundo caso). Não é um bug novo em `underbrace` — é o
   mesmo bug #2 propagando visualmente. Candidato a passo dedicado.
3. **Gap de `layout_underover` maior que o vanilla**: sem constante de gap explícita (empilha
   directamente por `height()`), ao contrário do vanilla (`underbar_vertical_gap`/
   `overbar_vertical_gap` da tabela MATH). Estruturalmente correcto após a correcção #4, só o
   espaçamento é mais largo que o ideal.

## Suíte completa (final)

```
typst-core:    4761 passed; 0 failed; 3 ignored   (+15 vs P905, os novos testes p906_*)
typst-infra:    734 passed; 0 failed; 5 ignored
typst-shell:     41 passed; 0 failed
```

Zero regressões em todo o percurso (verificado repetidamente após cada uma das 5 correcções).

## `crystalline-lint`

7 ficheiros L1 sincronizados (`math/layout/mod.rs`, `tests.rs`, `entities/layout_types.rs`) + 4 L3
(`export/builder.rs`, `export/stream.rs`, `font_metrics.rs`, `pipeline.rs`). `--fix-hashes .`: 0
drift após. Lint completo: só o warning V7 pré-existente e não relacionado
(`infra/package_version_resolution.md`).

## Confirmação visual (PDF real, `New Computer Modern`, comparado com vanilla onde aplicável)

- `underbracket(a+b+c)` / `overbracket(a+b+c)`: **correcto**, bate com o vanilla (bracket esticado,
  cobrindo a largura da base, glifo visível e correcto).
- `underbrace(a+b+c)` (sem anotação): **correcto**.
- `underbrace(a+b+c, "soma")` (com anotação, estrutura aninhada): **estruturalmente correcto**
  (chave visível, "soma" legível, sem sobreposição) — gap maior que o vanilla (achado #3 acima).
- `hat(a+b)`/`tilde(a+b)`: **ainda incorrecto** (achado #2 acima, fora de âmbito).
- `.typ` de 30 secções: hash inalterado
  (`9ae95a8d892103afc0c82c505a7f490a856acdad828577b7351013ad72691f29`), recompila `exit=0`,
  sequência `(1)`...`(44)` completa.

## Benchmark (Fase C final)

7 cenários, `hyperfine --warmup 5 -N -m 20`. Mesma ordem de grandeza da baseline P905 (94.4ms,
122.7ms, 93.8ms, 137.8ms, 99.4ms, 184.2ms, 107.3ms vs 94.1/123.2/93.9/137.2/99.5/184.6/107.3ms) —
sem regressão.

## Resumo por item

| Item | Veredicto | Estado |
|---|---|---|
| Trait `FontMetrics` aditivo (2 métodos) | Confirmado com o dono, implementado, testado | ✅ |
| `underbrace`/`overbrace`/`underbracket`/`overbracket` (sugar functions) | Confirmado com o dono, implementado, testado, visualmente confirmado | ✅ |
| `layout_stretchy_glyph_horizontal`/`layout_assembly_horizontal` | Implementado, 15 testes verdes | ✅ |
| `FallbackFontMetrics` sem overrides verticais/horizontais | Bug pré-existente (P891/P893), corrigido | ✅ |
| `emit_glyph_pdf` `/F1` hardcoded + remap em falta | Bug pré-existente, corrigido | ✅ |
| `collect_fonts_in_items` cego a `Glyph` | Bug pré-existente, corrigido | ✅ |
| `layout_underover`/`layout_accent` convenção "topo do box" | Bug pré-existente (mesma classe de P901/P905), corrigido | ✅ |
| `apply_math_default` não recursa `MathUnderover` | Confirmado, **fora de âmbito** | ⚠️ Registado |
| `hat`/`tilde` largura zero (+ consequência em cascata) | Confirmado, **fora de âmbito** | ⚠️ Registado |
| Gap de `layout_underover` maior que vanilla | Confirmado, **fora de âmbito** | ⚠️ Registado |
