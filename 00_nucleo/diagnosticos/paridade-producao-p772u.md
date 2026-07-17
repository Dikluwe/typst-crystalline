# P772u — Colapso de espaço em Cantarell-VF (CFF2/HVAR) no shaper

> **Passo:** 772u
> **Data:** 2026-07-17
> **Commit-base:** `2b981e7e0f2421c4952bb9b86b5f9a8ba1825014` (HEAD no início do passo).
> **Dependência:** P772o (achado, causa em `advance()` já eliminada para Cantarell, hipótese de `shaper.rs` registada).

---

## 1. Passo 0 — Reconfirmação da reprodução

```bash
cat > /tmp/p772u-test.typ <<'EOF'
#set text(font: "Cantarell", weight: 800)
Weight test here
EOF
./target/release/typst compile /tmp/p772u-test.typ /tmp/p772u-800.pdf
```

Reproduzido no binário pós-P772o: palavras coladas ("Weighttesthere"),
confirmado por render (`mutool draw -r 300`) e por análise de pixels. A
correcção de P772o (métricas de layout, `FallbackFontMetrics::advance()`)
não resolveu Cantarell — a causa é outra.

---

## 2. Instrumentação — hipótese CFF2/HVAR (shaper vs métricas) REFUTADA

Instrumentação directa comparando, para o texto "Weight test here" a
wght=800:

1. `ttf_parser::Face::set_variation()` + `glyph_hor_advance()` (mesmo
   caminho de `advance()`, font_metrics.rs).
2. `rustybuzz::Face::set_variations()` + `rustybuzz::shape()` (mesmo
   caminho de `try_shape()`, o desenho real).

**Resultado: os dois mecanismos concordam exactamente**, glifo a glifo
(mesmo `x_advance` em unidades de fonte para todas as 16 posições,
incluindo o espaço: 200fu a wght=800 em ambos, vs 220fu na instância por
omissão). A hipótese original do passo — "o shaper não aplica os deltas
de HVAR/CFF2 da mesma forma que ttf_parser" — está **refutada** por
medição directa. `rustybuzz` e `ttf_parser` aplicam a variação de peso
de forma idêntica para este texto/fonte.

### Confirmação adicional: não é característica de desenho da fonte

Vanilla (`lab/typst-original`, binário `target/release/typst` já
compilado) renderiza o mesmo documento com espaçamento limpo e igual
entre as três palavras — confirma que o colapso é um defeito do
cristalino, não um traço de desenho do Cantarell a peso alto (bearings
negativos a pesos extremos, etc.).

---

## 3. Segunda hipótese — posições de `FrameItem` (layout) — também correcta

Instrumentação de `shape_item`/`fix_line_positions_page` (eprintln!
temporário, removido no fecho do passo) mostra que os `FrameItem::Text`
(um por palavra) chegam ao shaper já correctamente posicionados:

```
"Weight" pos.x=70.8667 → TextShaped width_real=38.0600
"test"   pos.x=111.1267 (gap = 2.2pt, correcto)
"here"   pos.x=133.6657 (gap = 2.2pt, correcto)
```

`fix_line_positions_page` (P582) não introduz desvio (`w_est == w_real`
para os três itens, `shift` permanece 0). O ficheiro PDF gerado (dump via
`mutool show`) confirma os mesmos três `Td` — **70.867, 111.127, 133.666**
— exactamente correctos, com gaps de 2.2pt entre palavras.

**Conclusão parcial:** nem o shaper (§2) nem o posicionamento de
`FrameItem` (§3) têm o bug — a causa está mais a jusante, no exportador
PDF.

---

## 4. Causa real — `glyph_to_nominal` em `build_multifont` usa a face ERRADA

### 4.1 Achado, por dissecação do conteúdo do PDF

O array `/W` do CIDFont mostra larguras **já correctas** para wght=800
(`223 [1020]` para 'W', etc. — bate com a instrumentação §2). Mas os
deltas do operador `TJ` no content stream (`[ <00DF> -40 <011F> -18 ... ]
TJ`) só fazem sentido se o "nominal" usado para os calcular (P520,
`glyph_to_nominal`) for a instância **por omissão** (sem variação), não a
instância a wght=800:

```
GID 223 ('W'): nominal_omissão=980, x_advance(wght800)=1020
  → delta_esperado = 980 - 1020 = -40   ✓ bate com o TJ observado (-40)
GID 430 ('t'): nominal_omissão=361, x_advance(wght800)=405
  → delta_esperado = 361 - 405 = -44    ✓ bate com o TJ observado (-44)
```

Confirmado para os 6 glifos de "Weight" — bate exactamente em todos.

### 4.2 Mecanismo do bug

`build_multifont` (`03_infra/src/export/builder.rs`) construía
`glyph_to_nominal` (linha ~745, antes desta correcção) a partir da face
`face` **tal como recebida** (sem variação de eixo — `Face::parse`
directo, nenhum `set_variation`), num ponto do código **anterior** à
instanciação da fonte embutida (`instantiate_variable_font`, P530).
`/W`, por outro lado, é construído a partir de `face_for_widths`
(`widths_array`), que já usa `subset_face` — a face **pós-instanciação**
(peso correcto).

O modelo de delta do operador `TJ` (P520) assume que `nominal` (baseline
usado para calcular o delta) e `w0` (a largura efectivamente declarada em
`/W`, lida pelo leitor de PDF em tempo de render) vêm da **mesma**
instância — a fórmula:

```
deslocamento_real = w0 - TJ = w0 - (nominal - x_advance)
```

só se reduz a `x_advance` (o avanço correcto, medido pelo shaper) quando
`w0 == nominal`. Quando `nominal` vem da instância por omissão e `w0` já
vem da instância a wght=800, o deslocamento real passa a ser:

```
deslocamento_real = w0 - nominal + x_advance = 2×x_advance - nominal_omissão
```

— um excesso de avanço, por glifo, **igual à própria variação de peso**
(1020-980=40 unidades extra para 'W', etc.). Somado ao longo de "Weight"
(6 letras), o excesso acumulado é **2,29pt** — quase exactamente o gap de
2,2pt que devia separar "Weight" de "test", explicando o colapso quase
total observado visualmente.

`build_cidfont` (fonte única, sem instanciação nesse caminho) não sofre
disto: `glyph_to_nominal` e `/W` usam consistentemente a mesma face sem
variação — mas por isso também nunca embute peso variável real (a fonte
embutida fica sempre na instância por omissão nesse caminho).
`build_multifont` é o único caminho que instancia (P530) — e é
precisamente aí que a inconsistência existia.

---

## 5. Implementação

### 5.1 L0 actualizado antes do código

`00_nucleo/prompts/infra/export/builder.md`:
- §P520 — nova subsecção "§P772u — `glyph_to_nominal` tem de usar a MESMA
  instância que `/W`", com a fórmula, o mecanismo do bug e a correcção
  (aplicar a mesma variação de eixo a um clone de `face`, condicional ao
  sucesso da instanciação).
- §P560 — corrigida a regra de detecção de tipo de fonte (ver §6 abaixo,
  achado colateral) e renomeada para "§P560/§P772u".
- Entrada em "Histórico de Revisões".

### 5.2 Código — `03_infra/src/export/builder.rs`, `build_multifont`

`glyph_to_nominal` (espaço de GID **original**, pré-subset — o mesmo
espaço que `ShapedGlyph::glyph_id` produzido pelo shaper) passa a ser
calculado **depois** da tentativa de instanciação, com uma flag
`instancing_applied: bool` que regista se `instantiate_variable_font`
teve sucesso:

```rust
let mut nominal_face = face.clone();
if instancing_applied {
    for v in &axis_vars {
        nominal_face.set_variation(v.tag, v.value);
    }
}
// usar nominal_face.glyph_hor_advance(...) para construir glyph_to_nominal
```

Se a instanciação falhar (Python/fontTools ausente — mesmo aviso já
existente), `/W` fica na instância por omissão e `glyph_to_nominal`
acompanha esse fallback (não aplica a variação) — os dois lados do delta
continuam consistentes entre si em qualquer ambiente, mesmo que a fonte
embutida não tenha o peso correcto nesse caso degradado (limitação
pré-existente, já avisada, fora do âmbito desta correcção).

---

## 6. Achado colateral — CFF2 mal detectado em `font_embedding_data`

Durante a investigação, uma primeira tentativa de correcção (declarar
`/CIDFontType0`/`/FontFile3` em vez de `/CIDFontType2`/`/FontFile2` para
Cantarell) **não resolveu** o colapso visual sozinha — mas revelou um
segundo defeito real, independente do de §4: `cff_table_data()`
(`builder.rs`) só verificava `face.tables().cff` (CFF1) — `ttf_parser`
expõe `cff` e `cff2` como campos **distintos**. Cantarell-VF.otf (CFF2,
confirmado via `fontTools`: tabelas incluem `CFF2`, não `CFF `) caía
silenciosamente no ramo TrueType (`/CIDFontType2` + `/FontFile2`), apesar
de não ter tabela `glyf`. Confirmado que a fonte instanciada por
`fontTools.varLib.instancer.instantiateVariableFont` **mantém** CFF2
(não converte para CFF1) — o defeito é real e afecta qualquer fonte CFF2
embutida, não só as com peso variável.

Corrigido em paralelo (`font_embedding_data`/`is_cff2_font`): fontes CFF2
passam a usar `/CIDFontType0` + `/FontFile3` + stream `/Subtype
/OpenType` com `font_data` **completo** (não há subtype PDF para "CFF2
puro" — ISO 32000-2 §9.9.4 só define `Type1C`/`CIDFontType0C` para CFF1
puro e `OpenType` para o contêiner completo).

Este achado, sozinho, **não** era a causa do colapso (confirmado: mesmo
com a detecção CFF2 corrigida, o colapso persistia até §4 ser corrigido
também) — mas é um bug de conformidade PDF real e independente, mantido
como parte deste passo por ter sido descoberto na mesma investigação.

---

## 7. Validação

### 7.1 Pesos 100–900, Cantarell-VF

```
render mutool + poppler (pdftoppm) para wght ∈ {100, 400, 700, 800, 900}
```

Todos os cinco pesos renderizam "Weight test here" com espaçamento limpo
e igual entre as três palavras, em **ambos** os leitores (mutool E
poppler — confirma que o bug não era um quirk de um leitor específico:
antes da correcção, poppler reproduzia o mesmo colapso assimétrico que
mutool, incluindo o aviso "Embedded font file may be invalid").

### 7.2 Sem regressão — Ubuntu Sans (glyf/gvar, fix de P772o)

```
#set text(font: "Ubuntu Sans", weight: 800)
Weight test here
```

Renderiza correctamente, sem colapso — confirma que a correcção de
`glyph_to_nominal` não quebra o caminho já corrigido em P772o para fontes
`glyf`/`gvar`.

### 7.3 Testes automatizados novos

`03_infra/src/export/tests.rs`:

- `p772u_fonte_cff2_usa_cidfont_type0_opentype` — CFF2 (fixture
  `Cantarell-VF.otf`, adicionada a `03_infra/fixtures/fonts/`) gera
  `/CIDFontType0` + `/FontFile3` + `/Subtype /OpenType`; não gera
  `/CIDFontType2` nem `/CIDFontType0C`.
- `p772u_multifont_delta_tj_consistente_com_variacao_de_peso` — testa
  directamente o invariante quebrado em §4: para um `FrameItem::TextShaped`
  sintético (GID 223 = 'W', `x_advance=1020` — medido a wght=800),
  `w0(/W) - delta(TJ) == x_advance`. Passa em **ambos** os ambientes
  (com e sem `TYPST_CRYSTALLINE_PYTHON`/fontTools disponível — verificado
  manualmente nas duas condições), confirmando que a correcção mantém
  auto-consistência independentemente de a instanciação ter sucesso.

### 7.4 Suite completa

```
cargo build --release --workspace --tests   → 0 erros
cargo test --workspace --release
  typst-core:   4192 passed, 0 failed
  typst-infra:   647 passed, 0 failed, 5 ignored  (+2 novos, P772u)
  typst-shell:    33 passed, 0 failed
  typst-wiring:    2 passed, 0 failed
  cli (integration): 29 passed, 0 failed
  crystalline_lint (integration): 2 passed, 0 failed
crystalline-lint .
  0 violações (mesmo warning V7 pré-existente sobre
  package_version_resolution.md, não relacionado)
```

---

## Critério de fecho do passo

- [x] Reprodução reconfirmada com o binário pós-P772o.
- [x] `shaper.rs` instrumentado, avanços comparados com `ttf_parser`
      (mesmo mecanismo de `advance()`) — concordância exacta, hipótese
      CFF2/HVAR do shaper **refutada**.
- [x] Hipótese CFF2/HVAR do shaper refutada com evidência (glifo a glifo,
      incluindo o espaço isolado).
- [x] Nova causa investigada (não abandonado sem causa): posições de
      `FrameItem` (layout, §3) confirmadas correctas; causa real isolada
      no exportador PDF (`build_multifont::glyph_to_nominal`, §4) — uma
      inconsistência entre a face usada para `/W` (pós-instanciação) e a
      face usada para o baseline do delta `TJ` (pré-instanciação).
- [x] Corrigido — validado em todos os pesos testados (100-900), sem
      regressão em Ubuntu Sans (glyf/gvar, P772o) nem na suite geral.
- [x] `cargo test --workspace` verde (647 em typst-infra, +2 novos).
- [x] `crystalline-lint .` zero violações.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772u.md`.

---

## Próximo passo

Conforme decidido em P772t: `table()` header/footer (extensão de P772i),
seguido do resíduo de risco plausível do inventário (`foundations::target_`,
`plugin_`, `image::pdf`, `layout::frame`, `math` — 23 itens).
