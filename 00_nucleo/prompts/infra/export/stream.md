# Prompt L0 — `infra/export/stream` — PageContext + emit unificado
Hash do Código: d2084747

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/export/stream.rs`
**Criado em**: 2026-05-19 (P307c)
**Atualizado em**: 2026-07-03 (P548 — correção do sinal do delta TJ)
**ADRs**: ADR-0098 (SSoT helpers unificados pós-P281)

---

## Contexto

Cluster central de emit PDF — implementação do "single source of
truth" pós-P281: helpers unificados invocados tanto pelo caminho
top-level (`build_page_stream`) como pelo caminho local em Group
(`draw_item_local`).

- `FontScenario` enum dispatch para `emit_text_pdf` / `emit_glyph_pdf`.
- `PageContext` agregador de `ptr_to_idx`, `img_refs`, `pat_ptr_to_idx`, `pat_refs`, `font_scenario`.
- `build_page_stream` orquestra emit page-level.
- `draw_item_local` emit recursivo em Groups (clip masks, transforms).
- Shape primitives: `emit_shape_path_local` + `emit_rounded_rect_ops` + `emit_stroke_paint` + `line_rg_prefix`.

## Restrições estruturais

- L3. Usa formatadores `format!`/`String` — não toca FS.
- `FontScenario` e `PageContext` são `pub(crate)` — instanciados em `super::builder` mas atravessam tipo nas chamadas.
- Helpers de emit são `pub(super)` — chamados por `super::builder` e self.
- `FrameItem::Image` inclui `orientation` (P776); o emit compõe a matriz `cm`
  com a transformação EXIF correspondente, preservando os bytes originais do
  JPEG/PNG (paridade `typst-pdf/src/image.rs::exif_transform`). A matriz é emitida
  com precisão de 5 casas decimais (P777), replicando o vanilla e evitando desvios
  de sub-pixel nas bordas em orientações com flip/rotate.
- Excede limite 800 LOC ADR-0037 Regra 2 (~685 LOC). Sub-divisão futura em `stream/{page,text,shape,draw}.rs` em P-stream-decomp dedicado se justificado.
- P1133: `emit_rounded_rect_ops` recebe `Corners<Pt>` de `ShapeKind<Pt>` já resolvido pelo
  layout. O emissor pode fazer clamp geométrico por largura/altura, mas não
  pode resolver `Length`, usar contexto zero nem projetar `.abs`.

## Interface

```rust
pub(crate) enum FontScenario<'a> {
    Type1,
    Cidfont { char_to_gid, glyph_mapping, glyph_to_nominal },
    Multifont { fonts, per_font_char_to_gid, per_font_glyph_mapping, per_font_glyph_to_nominal },
}
pub(crate) struct PageContext<'a> { /* ptr_to_idx, img_refs, pat_*, font_scenario */ }
impl<'a> PageContext<'a> {
    pub(crate) fn type1(...) -> Self;
    pub(crate) fn cidfont(..., char_to_gid, glyph_mapping, glyph_to_nominal) -> Self;
    pub(crate) fn multifont(..., fonts, per_font_char_to_gid, per_font_glyph_mapping, per_font_glyph_to_nominal) -> Self;
}

pub(super) fn emit_text_pdf(ops, pos_x, base_y, text, style, scenario);
pub(super) fn emit_glyph_pdf(ops, pos_x, base_y, glyph_id, size, scenario);
pub(super) fn emit_stroke_paint(...);
pub(super) fn line_rg_prefix(color: &Option<Color>) -> String;
pub(super) fn build_page_stream(page: &Page, ctx: &PageContext) -> Vec<u8>;
pub(super) fn emit_shape_path_local(ops, kind, w, h);
pub(super) fn emit_rounded_rect_ops(...);
pub(super) fn draw_item_local(ops, item, ctx, ...);
```

## Invariantes (ADR-0098)

- `emit_text_pdf` / `emit_glyph_pdf` / `line_rg_prefix` chamados em ambos os caminhos top-level e local — alteração simétrica garantida por construção.
- Para `color: None` em Line, `line_rg_prefix` retorna `""` (bit-exact pré-P285).
- Hash de `stream.rs` é métrica de aderência ao padrão (P282-P306, 23 passos cumulativos).

## Critérios de verificação

- Tests P281+ em `tests.rs` validam paridade entre caminhos.
- Test `p282_line_stroke_color_simetric` valida que `RG` injecção é simétrica.
- Snapshot binário em `p307b_snapshot_tests.rs` valida invariante observable.

## §P486 — `x_offset` no TJ array (Sub-item B)

**P486** adiciona suporte a `ShapedGlyph.x_offset` no operador PDF `TJ` em
`emit_shaped_pdf`. Aplicado simetricamente em `Cidfont` e `Multifont`.

### Fórmula por glifo com x_offset != 0

```
[ {-x_offset_tu} <GID> {advance_tu} ... ] TJ
```

Onde:
- `x_offset_tu = -(x_offset / upm * 1000)` — pré-glifo: desloca cursor
- `advance_tu` — ver §P520 (delta model)

O `x_offset` é uma translação visual do glifo actual; não altera o avanço
para o glifo seguinte.

### `y_offset` — scope-out confirmado

`y_offset` requer sequências `Td` (saída do array TJ) e não há corpus LTR com
`y_offset != 0`. Scope-out declarado — não implementado em P486.

### Testes adicionados P486

- `p486_emit_x_offset_zero_equivale_p485`: x_offset=0 → sem número antes do GID
- `p486_emit_x_offset_nonzero_aplica_ajuste`: x_offset=-50, upm=1000 → "50 " antes do GID
- `p486_emit_x_offset_positivo`: x_offset=30, upm=1000 → "-30 " antes do GID

## §P520 — Kerning via delta model no TJ

**P520** corrige o avanço do operador `TJ` para reflicta kerning aplicado
pelo shaper (rustybuzz). O CIDFont declara `/W` com larguras nominais
(`hmtx`), pelo que o `TJ` deve conter apenas o *delta* entre a largura
declarada e o avanço real.

### Fórmula por glifo

```
nominal    = glyph_to_nominal.get(glyph_id).unwrap_or(x_advance)
advance_tu = (nominal - x_advance) as f64 / upm * 1000.0
[ {-x_offset_tu} <GID> {advance_tu} ... ] TJ
```

No operador PDF `TJ`, cada número é **subtraído** da coordenada horizontal
antes de desenhar o próximo glifo. Portanto:

- `advance_tu` **positivo** quando `x_advance < nominal` (kerning negativo —
  aproxima o próximo glifo).
- `advance_tu` **negativo** quando `x_advance > nominal` (kerning positivo —
  afasta o próximo glifo).
- `glyph_to_nominal` é construído em `builder.rs` a partir do `hmtx` da fonte
  original, para todos os `glyph_id` usados no documento (`collect_glyph_ids`
  + codepoints mapeados).

### Testes adicionados P520

- `p520_emit_shaped_kerning_delta`: x_advance=599, nominal=639, upm=1000 → delta = +40

---

## Histórico de Revisões

| Data | Motivo | Ficheiros afetados |
|------|--------|--------------------|
| 2026-05-19 | Criação — P307c: PageContext + emit unificado | `stream.rs` |
| 2026-07-03 | P548 — correção do sinal do delta TJ: `advance_tu = (nominal - x_advance)` em vez de `(x_advance - nominal)` | `stream.rs`, `stream.md`, `builder.md`, `tests.rs` |
| 2026-08-23 | P1133 — RoundedRect transporta `Corners<Pt>` resolvido; L3 deixa de apagar contexto relativo | `geometry.rs`, layout Block/Boxed/cursor, `stream.rs`, `render.rs`, `svg.rs` |

---

## §P788 — `draw_item_top`: flip Y para filhos de `FrameItem::Link` ao nível da página

**Decisão:** a emissão top-level por item foi extraída de `build_page_stream`
para `draw_item_top(ops, item, page_height, ctx) -> ops` (recebe/devolve
`ops` por valor — braço `Link` chama recursivamente sem conflito de borrow).
O braço `FrameItem::Link` passa a desenhar os filhos **pelo caminho
top-level com flip Y** (`pdf_y = page_height - pos.y`).

**Causa raiz (medida, P786 A9 + `#link` genérico):** os filhos de Link ao
nível da página eram desenhados por `draw_item_local` — que NÃO aplica flip
(assume a matriz `cm` invertida de um `Group` envolvente). Sem Group, os
filhos apareciam com `pos.y` crua → fundo da página (medido: y≈754 em vez
de y≈68; vanilla: coordenadas idênticas após a correção). `draw_item_local`
mantém o seu papel dentro de `Group` (matriz já invertida) — inalterado.

**Critério de aceitação:** teste `p788_link_top_level_filho_tem_flip_y` —
filho `Text` a (70,100) em página 800 → stream contém `70.0 700.0 Td`
(nunca `70.0 100.0 Td`).

## P836 — selecção de fonte por variações

`FontScenario::Multifont` e `font_index_for_style` passam a chavear por
`(FontList, FontVariant, FontVariations)`: o índice `/F{n}` de cada
run de texto é resolvido comparando também `style.variations`
(`unwrap_or_default`), coerente com a chave da pipeline/builder.

## P906 — `emit_glyph_pdf`: dois bugs em cadeia, `/F1` hardcoded + remap de subsetting em falta

**Contexto**: ao confirmar visualmente o mecanismo de esticamento horizontal
(`compiler/layout.md` §P906), `underbracket(a+b+c)` produzia PDF com glifos
invisíveis/`.notdef`, apesar do layout (largura, posição) estar correcto e
testado. Medido directamente no PDF exportado (`pdftotext -bbox`,
instrumentação temporária), não inferido.

**Achado 1**: `emit_glyph_pdf` escrevia sempre `/F1 Tf` incondicionalmente,
mesmo no ramo `Multifont` (várias fontes embutidas, `/F1`.._`/Fn`). Corrigido
usando `per_font_glyph_reverse` (mapa reverso glyph_id→char por fonte, o
MESMO `build_math_glyph_reverse_map` já usado no subsetting DEBT-9/P45,
partilhado — ver `builder.rs` abaixo) para encontrar em qual fonte o
`glyph_id` está efectivamente presente. `FontScenario::Cidfont` não muda
(só uma fonte candidata, `/F1` sempre correcto aí).

**Achado 2, só visível depois de corrigir o 1º**: nem `Cidfont` nem
`Multifont` aplicavam `remap_glyph_id`/`glyph_mapping` (P516, já aplicado em
`emit_text_pdf`/`emit_shaped_pdf`) ao `glyph_id` recebido — desenhava sempre
o índice ORIGINAL (pré-subsetting) da fonte completa, mas a fonte
efectivamente embutida está subsetada (renumerada) sempre que `glyph_mapping`
não é vazio. `<XXXX> Tj` referenciava um slot de glifo errado/`.notdef` na
fonte embutida mesmo com `/Fn` já correcto. Corrigido aplicando o mesmo
`remap_glyph_id` já usado no caminho de texto.

**Pré-existente, partilhado com o eixo vertical**: nenhum dos dois bugs é
específico ao esticamento horizontal — `layout_stretchy_delimiter`
(vertical, delimitadores altos) usa exactamente o mesmo `emit_glyph_pdf`.
Nunca antes exercitado com dados reais porque `FallbackFontMetrics` não
implementava `vertical_glyph_variants`/`assembly` (gap adiado desde P891/
P893, corrigido no mesmo passo — `infra/font_metrics.md` §P906) — sem isso,
`emit_glyph_pdf` nunca recebia um `glyph_id` de uma fonte diferente de `/F1`
nem precisava de remap, então os dois bugs ficaram latentes.


## P953 — `cm` do `FrameItem::Group`: VERIFICADO correcto (`[a, -b, -c, d]`)

**Investigação** (`typst-passo-953` Fase A): a direcção de `#rotate` foi
suspeita de estar invertida, mas a medição final (zoom a alta resolução do
render + transformação efectiva do texto nos dois PDFs) confirmou que a forma
original `[a, -b, -c, d]` (negação de `b` e `c`) está **correcta** — o
efectivo no espaço de ecrã é idêntico ao do vanilla (`[cos, sin, -sin, cos]`
para `rotate(45deg)`, rotação horária descendente nos dois). Uma alteração
experimental (negar `b`/`d` + inverter o sinal em L1) foi medida como
incorreta e **revertida sem deixar rasto**. Registo: a convenção actual está
validada contra o vanilla — não mudar sem uma medição equivalente.

## P956 — `StreamMode`: modo verboso (vanilla-espelhado, novo padrão) vs compacto (formato Passo 20)

**ADR-0126 (emendada P956)**: o exportador passa a ter dois modos de emissão
de texto. **Verboso = padrão de produção** (espelha a semântica do vanilla
operador a operador); **Compacto = o formato actual** (Passo 20), preservado
atrás da flag `--compact` sem alteração de bytes. `BDC`/`EMC` (PDF tagueado)
**não** entra neste passo — eixo separado (ADR-0126 §1.3).

### O padrão vanilla (medido — typst 0.15.1, `temp/p956/min-vanilla.pdf`)

Por run de texto, o vanilla emite (stream descomprimido, `mutool clean -d`):

```pdf
q 1 0 0 -1 70.86614 763.78564 cm
/c0 cs 0 scn
BT 0 Tr /f0 11 Tf 1 0 0 -1 0 0 Tm [(…)] TJ
ET
Q
```

- `q` + `cm 1 0 0 -1 x y`: isola o bloco e carrega posição + flip Y (o frame
  vanilla é y-down; o flip é por bloco, não pré-calculado).
- `/cN cs … scn`: colour space nomeado (em `/Resources/ColorSpace`) + cor de
  preenchimento, declarado **por bloco** (preto = `/c0 cs 0 scn`, com c0
  ICCBased gray; cores = ICCBased sRGB, 3 componentes).
- `BT 0 Tr /f0 11 Tf 1 0 0 -1 0 0 Tm […] TJ ET`: `Tr` explícito, `Tm` com
  duplo flip (glifos direitos; a posição está toda no `cm`), kerning no array
  `TJ`. Depois `Q`.

### Emissão verbose cristalina (alvo deste passo)

```pdf
q 1 0 0 -1 {x} {y} cm
/c0 cs {r} {g} {b} scn
BT 0 Tr [/F{n} {size} Tf] [{tc}] 1 0 0 -1 0 0 Tm [{…}] TJ ET
Q
```

Regras:

- **Posição**: top-level usa `ty = page_height − pos.y` (o mesmo valor que o
  compacto passa a `Td` — confirmado na medição: o `ty` do vanilla já é a
  coordenada y-up da página, ex.: 763.78 = 841.89 − 78.1); local (dentro de
  Group) usa `ty = pos.y` directo, como o compacto. Ou seja, `y_eff` é
  exactamente o valor que hoje alimenta o `Td` em cada caminho — o que muda
  é o envelope, não a coordenada.
- **`Tm` sempre `1 0 0 -1 0 0`** — posição inteiramente no `cm`.
- **Porque a geometria é idêntica à do compacto por construção** (derivação):
  seja `F = flip(1,0,0,-1,0,0)` e `G` a `cm` de um Group envolvente (identidade
  no top-level). O compacto desenha texto com matriz efectiva `T(x,y)·G`
  (`Td` com CTM=`G`, `Tm` identidade). O verbose emite `cm = T(x,y)·F` e
  `Tm = F`: efectiva = `F·(T(x,y)·F)·G = T(x,y)·(F·F)·G = T(x,y)·G` — **a
  mesma matriz**, porque `F·F = I`. Os dois modos produzem glifos nas mesmas
  posições por construção algébrica; o decalque Fase D é a verificação
  empírica disto. O vanilla segue a mesma regra (medido em `#rotate(45deg)`:
  o bloco de texto traz `cm = R×F` composta e `Tm = F` constante).
- **`0 Tr` explícito** em todos os blocos; faux-bold (P139) usa `2 Tr` +
  `{stroke} w` no mesmo envelope.
- **`cs`/`scn` por bloco**: fill = `style.fill` ou preto `0 0 0` por omissão.
  `/c0` é o colour space sRGB ICCBased declarado nos recursos da página
  (ver `builder.md` §P956).
- **Conteúdo do array `TJ` inalterado** (P485/P486/P520/P548 — delta model,
  `x_offset`, remap de subsetting): o helper que constrói o array é
  partilhado pelos dois modos.
- **`emit_text_pdf`** (Type1 fallback / `Text` não-shaped): mesmo envelope,
  `(…) Tj`. **`emit_glyph_pdf`** (stretchy): mesmo envelope, `<gid> Tj`.
- **`/F1..N` mantidos** (nomes de recurso são arbitrários em PDF; o decalque
  compara geometria e operadores, não nomes).
- Line/Image/Shape/Group: **inalterados** neste passo (já usam `q…Q`/`cm`);
  o escopo verbose é a emissão de texto (`Text`/`TextShaped`/`Glyph`).

### Dispatch

`PageContext` ganha `mode: StreamMode` (construtores `type1`/`cidfont`/
`multifont` recebem-no — ver `builder.md` §P956). `draw_item_top` e
`draw_item_local` despacham: `Compact` → os helpers actuais, **byte-inalterados**;
`Verbose` → variantes verbose dos mesmos helpers (partilham o construtor do
array TJ e o `fill`/font selection).

### Regra para testes (Fase B.3 de P956)

Todo o teste que assecre bytes/operadores do stream declara **explicitamente**
o modo que espera (`StreamMode::Verbose` ou `StreamMode::Compact`) — nenhum
teste fica ambíguo sobre qual formato está a verificar. Os testes actuais do
formato Passo 20 passam a declarar `Compact` (o formato que verificam não
muda); testes novos do verbose verificam o envelope `q/cm` + `Tm` + `0 Tr` +
`cs`/`scn` e a equivalência de posição final (`Td` compacto vs `cm`+`Tm`
verbose → mesma baseline).

## P979 — agrupamento de runs de texto num único `BT…ET`

**Data:** 2026-08-05 · **Gate:** confirmado pelo dono em 2026-08-05
("Continue" após `typst-passo-979-faseA.md`).

**Regra do vanilla** (lida e confirmada): o vanilla emite **um `BT…ET`
por `TextItem`** — `typst-pdf/src/text.rs:50-57` chama
`surface.draw_glyphs` uma vez por item, e o krilla
(`crates/krilla/src/content.rs:626-700`, `fill_stroke_glyph_run`) abre um
`begin_text()` por chamada. Os TextItems do vanilla vêm do shaping de
linha do typst-layout: uma linha de prosa com estilo uniforme é **um**
TextItem (muitos glifos) — daí o agrupamento. Mudanças de
fonte/estilo/cor/transformação partem o run (TextItems separados ⇒
blocos separados). Dentro do bloco, os espaços inter-palavra vão como
ajustes `TJ` (delta model).

**Cristalino actual**: um bloco `BT…ET` por item de texto
(`stream.rs`, envelope verbose P956) — em prosa, um por palavra+espaço;
em math, um por glifo/run.

**Medição do ganho** (Fase A.4; `temp/p979/`):

| documento | BT cristalino | BT vanilla | stream cristalino | stream vanilla |
|---|---|---|---|---|
| 02-lorem (prosa) | **503** | **36** | 82 486 B | 30 816 B |
| 30 secções (math) | 2074 | 1919 | 259 267 B | 246 018 B |

**Desenho proposto para a Fase B** (protocolo de dois agentes):

- No emissor verbose, agrupar itens `Text`/`TextShaped` **consecutivos de
  prosa**
  com envelope idêntico (mesma fonte, tamanho, fill, Tr/stroke, tracking,
  direcção, `units_per_em`) e **mesma baseline** (`pos.y` igual): um só
  prefixo de envelope (`q/cm/cs/scn/BT/Tr/Tf/Tc/Tm`) e um só array `TJ`,
  com o gap entre itens como ajuste `TJ` computado das **posições**
  (não dos advances) — posições bit-idênticas por construção.
- Nunca fundir através de: mudança de qualquer campo do envelope,
  itens não-texto, quebra de linha (y diferente), Groups/Links (o
  conteúdo de um Group é outro âmbito de coordenadas).
- Itens cujo `TextStyle.math == true` preservam a granularidade de L1: um
  bloco por fragmento, sem conversão de `pos.x` absoluto em fronteira `TJ`.
  Esta restrição foi promovida à produção em P1133 depois de a seção 02
  refutar a hipótese original de identidade visual: a quantização inteira
  de `TJ` produzia 59 pixels divergentes a 288dpi, enquanto o split era
  pixel-idêntico ao vanilla.
- Critério de aceitação: prosa continua agrupada (~36 blocos no lorem);
  matemática conserva posições absolutas por fragmento e a seção 02 é
  pixel-idêntica ao vanilla no modo verbose normal.

## P1120 — texto dentro de `FrameItem::Group`: envelope de reflexão (glifos espelhados)

**Data:** 2026-08-20 · **Classe (ADR-0127):** correcção de paridade, fluxo
contínuo (L0 primeiro + resselo; gate = teste RED→GREEN + revalidação).

### Medição (antes da decisão, ADR-0108)

`.typ/sec_36.typ` (4 caixas com `rotate`/`scale`/`skew` sobre equações),
compilado com o cristalino no estado de P1119 e com o vanilla ratificado
(`/usr/local/bin/typst`), render a 144 ppi: **as posições dos glifos batem**
(Δ < 0,3 pt) mas **todos os glifos dentro dos grupos saem espelhados na
vertical** — `x²` lê-se com o `2` invertido, `dif x` sai `qx`, o integral
desenha-se ao contrário. Visível no PNG, invisível numa auditoria que só
compare a translação do `cm` (foi o que a auditoria de P1119 comparou).

Stream medido (cristalino, P1119):

```pdf
q 0.965926 -0.258819 -0.258819 -0.965926 28.22370 86.50105 cm   % Group
  q 1 0 0 -1 0.00000 0.00000 cm ... BT ... 1 0 0 -1 0 0 Tm ... Q  % filho
Q
```

Vanilla (mesmo documento) não aninha: emite **um `cm` por bloco de glifo**
com a matriz já composta (`0.9659 −0.2588 −0.2588 −0.9659 …`) e `Tm
1 0 0 −1`. As duas formas são mecânica diferente (ADR-0107: mecânica pode
divergir) — o que não pode divergir é o glifo desenhado.

### Álgebra (a causa)

O `cm` do `Group` (§P1119) é a matriz Typst→PDF **completa**: `[a, −b, c, −d]`
— o flip do eixo Y já lá está composto. Os helpers de texto são partilhados
com o caminho top-level e assumem que o flip **ainda não** foi aplicado:

- verbose: `q 1 0 0 −1 x y cm` + `Tm 1 0 0 −1` (duplo flip = glifo direito);
- compacto: `x y Td`, sem flip nenhum.

Debaixo do `cm` do grupo, a matriz efectiva do glifo fica
`Tm · M_bloco · M_grupo`. Com `M_bloco` a trazer o seu flip e `M_grupo`
outro, sobra um flip → espelho vertical. Com o compacto (sem flip próprio),
sobra o flip do grupo → o mesmo espelho.

### Regra

`draw_item_local` envolve **cada** emissão de texto (`Text`, `TextShaped`,
`Glyph`, nos dois modos) num **envelope de reflexão**:

```pdf
q
1 0 0 -1 0 0 cm      % repõe a pré-condição dos helpers
<bloco de texto emitido em (pos.x, -pos.y)>
Q
```

`F∘F = I` no eixo do glifo (fica direito) e a reflexão da coordenada
(`−pos.y`) recoloca a posição y-down local no sítio certo. Verificação
algébrica no próprio teste: o produto `Tm · M_bloco · M_reflexão · M_grupo`
tem de dar `d = +1` (sem espelho) e a translação `origem_do_grupo + local`.

Line/Shape/Image em espaço local **não** mudam: já emitem coordenadas locais
y-down sob o flip do grupo.

### Aceitação

- `p1120_verbose_group_filho_local_reflectido` (substitui
  `p956_verbose_group_filho_local_sem_flip`, que fixava a convenção que
  produzia o espelho): envelope emitido + verificação algébrica do produto.
- `.typ/sec_36.typ` renderizado a 144 ppi contra o vanilla: 484 pixels
  diferentes em 1536×326 (0,1 %), todos na caixa 2 (resíduo de largura da
  equação, §Gaps).

### Gaps deixados abertos (medidos, não corrigidos aqui)

- `FrameItem::Image` em espaço local não recebe o envelope (imagens dentro
  de grupos transformados ficam com o flip do grupo). Não exercitado por
  `.typ/sec_36.typ`.
- Glifos bitmap (CBDT) dentro de grupos: mesma reserva.

## P1140.5-A — render visual de `FrameItem::Semantic`

### Medição antes da decisão

Não há hoje BDC/EMC, MCID ou StructTreeRoot no stream cristalino. Emitir apenas
`/Formula BDC` seria estrutura incompleta e não reproduziria o vanilla.

### Decisão

Nesta fase, o stream recursa nos filhos de Semantic exatamente como container
transparente nos modos Verbose e Compact, preservando o metadado no valor de
entrada. Não emite tagging parcial. P1140.6 liga MCIDs e árvore estrutural.

## P1140.6 — conteúdo marcado de fórmulas

### Medição antes da decisão

P1140.5 entrega `Semantic(Formula)` ao emissor, mas o stream ainda recursa
transparentemente e não possui MCID. O vanilla envolve a pintura da fórmula
em conteúdo marcado associado à árvore estrutural.

### Decisão

Quando `PdfTags::Enabled`, `PageContext` recebe a tabela determinística de
MCIDs da página. Ao entrar em `FrameItem::Semantic { kind: Formula, items,
.. }`, o emissor escreve `/Formula << /MCID n >> BDC`, desenha os filhos
exatamente uma vez pelo mesmo caminho visual e escreve `EMC`. A numeração é
zero-based e segue a ordem de pintura dos envelopes na página. Group e Link
podem estar dentro da fórmula sem criar novo MCID. Envelopes Semantic
aninhados recebem MCIDs próprios e balanceados.

Quando `PdfTags::Disabled`, Semantic continua container visual transparente e
nenhum operador de tagging é emitido. A regra vale igualmente para Verbose e
Compact; o conteúdo visual fora de BDC/EMC permanece idêntico. `alt` nunca é
escrito no content stream e nunca vira texto visual.

## P1286 — marcado de `pdf.artifact`

### Medição anterior à decisão

O receipt P1286 observou `/Artifact BMC ... EMC` para `other`,
`/Artifact<</Attached[/Top]/Subtype/Header/Type/Pagination>>BDC` para header e
`/Artifact<</Type/Background>>BDC` para background em PDF 2.0. O body fora do
wrapper permaneceu na estrutura; dentro do artifact não ganhou MCID. AT real
permaneceu `Unknown`.

### Decisão condicionada ao gate

Com `PdfTags::Enabled`, `SemanticKind::Artifact(kind)` abre marcado Artifact,
desenha filhos exatamente uma vez e fecha `EMC`; não usa MCID nem
`StructElem`. `Other` usa `BMC`. Kinds tipados usam property list conforme o
mapeamento PDF aplicável; no fragmento medido, Header e Background reproduzem
as propriedades acima. Com `PdfTags::Disabled`, o envelope é container visual
transparente sem BMC/BDC/EMC. Verbose/Compact não altera essa semântica.

Fallbacks dos doze kinds por versão PDF e efeito real em AT continuam
`Unknown`; teste estrutural positivo não os converte em sucesso.

## P1288 — conteúdo marcado de tabelas (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

`03_infra/src/export/stream.rs:138-174` atribui MCID somente a Formula fora de
Artifact; `semantic_opening` em `:1196-1222` não reconhece tabela. A fonte
vanilla pinada mede Table/TR/TH/TD e associações header/data em
`typst-pdf/src/tags/context/table.rs:197-239,289-353`.

### Decisão proposta

Com `PdfTags::Enabled`, a travessia da árvore de tabela atribui MCIDs
determinísticos às unidades de conteúdo que participam da structure tree,
preserva ordem de pintura e envelopes balanceados. Tabela simples usa
Table→TR→TD; header automático uniforme usa Table→THead/TBody→TR→TH/TD; linha
explicitamente mista conserva TR direta e Data explícita em header permanece
TD. Scope/level não viram texto nem operadores visuais; level também não vira
atributo numérico PDF.

MCIDs reiniciam em zero por página. Cada página usa seu `StructParents`, e a
entrada de `ParentTree /Nums` conserva a ordem local dos MCIDs. Repetição
visual multipágina reutiliza o mesmo TH lógico e não duplica THead, IDs ou
relações; uma única árvore lógica atravessa as páginas.

Com `PdfTags::Disabled`, todos os carriers de tabela são containers visuais
transparentes e nenhum BMC/BDC/EMC/MCID é emitido. Verbose/Compact não muda a
semântica. O stream não decide ParentTree, header IDs nem summary; entrega os
MCIDs ao builder pela mesma pré-passagem determinística. A estrutura medida
não promete comportamento de AT, reflow nem conformidade PDF/UA.
