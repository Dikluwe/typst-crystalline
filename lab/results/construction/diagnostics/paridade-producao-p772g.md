---
# Diagnóstico — P772g: correcção de `layout_place` duplicando origem de célula
quando aninhado em `Content::Align`/footnotes

> **Passo:** 772g
> **Data:** 2026-07-16
> **Commit-base:** `2c7025a9a949ecafda8a2550c8c3692db3c9ad0b` (working tree com
> alterações não commitadas — ver `git diff --stat` no fecho do passo).
> **Tipo:** Implementação directa (causa já confirmada por instrumentação e
> coordenadas em P772f).

---

## 1. Sonda — mecanismo exacto e wrappers afectados

### 1.1 O problema medido em P772f

`layout_place` (`placement.rs`), sob `PlaceScope::Column` com
`regions.cell`/`cell_origin_x` `Some`, emite coordenadas **absolutas** (`target_x = cx
+ alinhamento + dx`). Isto está correcto quando o consumidor imediato trata os itens
devolvidos como já finais — mas está errado quando esse consumidor **também** soma o
seu próprio `target_x` absoluto a todos os itens que recebe (duplicando a origem).

### 1.2 Duas famílias de chamadores de `layout_sub_frame`, confirmadas por leitura
directa de código (não assumidas)

- **Consumidor absoluto** — passa a `origin_x` real da região e não soma nada extra
  aos itens devolvidos:
  - `grid.rs` (chamada per-célula, `origin_x: body_x`; tradução final usa `x: Pt(lx)`
    sem re-somar `body_x` — `grid.rs:571-579` antes desta correcção).
  - `box.rs`/`layout_sub_frame_inline` — não reinicia `line_start_x`, opera no cursor
    real do pai (`sub_frame.rs:165-220`).
  - `pad.rs`/`stack.rs` — não usam `layout_sub_frame` de todo; deslocam
    `cursor_x`/`line_start_x` directamente no frame real (`pad.rs:39-44`,
    confirmado sem chamadas a `layout_sub_frame` em `stack.rs`).
- **Consumidor relativo (recompositor)** — passa `origin_x: 0.0`, calcula depois um
  `target_x` absoluto a partir do seu próprio estado, e soma-o inteiro a cada item:
  - `layout_align` (`placement.rs:31-40` antes da correcção).
  - Emissão de footnotes (`cursor.rs:705-714`, `target_x = left_x` somado a cada item
    antes da correcção).
  - `columns.rs::layout_segmented` — usa `margin` como origem local do buffer da
    coluna, depois traduz por `dx = column_x_offsets[idx] - margin`
    (`columns.rs:222,230-231,259-263`).

`Content::Place` é sempre um consumidor absoluto a jusante — só compõe correctamente
sob outro consumidor absoluto. Aninhado num recompositor, a origem é somada duas
vezes.

### 1.3 Decisão de desenho (confirmada com o humano antes de tocar código)

Duas opções foram avaliadas (`AskUserQuestion`, ver histórico da sessão):

- **Opção A (escolhida)** — corrigir os wrappers recompositores para se comportarem
  como consumidores absolutos (passar a origem real, somar só o delta incremental de
  alinhamento). `layout_place` fica inalterado.
- Opção B (não escolhida) — nova flag no `Layouter` consultada por `layout_place`.
  Mais fiel à formulação original do achado, mas adiciona estado partilhado e não
  resolve a inconsistência de convenção entre wrappers.

O humano confirmou a Opção A. O L0 (`00_nucleo/prompts/engine/layout.md`, secção
"Contrato de composição de coordenadas entre `layout_sub_frame` e `Content::Place`")
foi actualizado **antes** do código, conforme o Protocolo de Nucleação — hash fixado
via `crystalline-lint --fix-hashes .` (`b1739fd9`, aplicado a todos os ficheiros do
módulo `layout`).

### 1.4 `columns.rs` — auditado, **não precisou de alteração**

Hipótese inicial: `columns.rs::layout_segmented` também duplicaria a origem (usa o
mesmo padrão "origem local + tradução final"). **Medido antes de decidir** (ADR-0108):
construí um repro (`#columns(2, grid(... align(top, place(...))))`) e comparei via
`mutool trace` com o binário já contendo a correcção de `layout_align` mas **sem**
qualquer alteração a `columns.rs`. Resultado: círculo cristalino em `(41.996, 41.998)`
absoluto vs vanilla `(41.996626, 41.996628)` — bate dentro de tolerância sub-pt.

Explicação: tanto `grid.rs` (`col_starts` começa em `self.page_config.margin`) como
`columns.rs` (`line_start_x = margin`) usam a mesma constante `page_config.margin`
como referência local para o seu frame — coincidência de nomenclatura que, na
prática, torna o `dx = column_x_offsets[idx] - margin` de `columns.rs` consistente com
qualquer conteúdo (incluindo `Content::Place` com origem absoluta calculada
relativamente a esse mesmo `margin` local). A correcção de `layout_align` já é
suficiente porque usa `self.regions.current.line_start_x.0` — que reflecte
correctamente o valor local-a-coluna quando `align` está aninhado dentro de
`columns()`. **Não alterei `columns.rs`** — confirmado sem regressão, sem alteração
necessária.

---

## 2. Implementação

### 2.1 `placement.rs::layout_align`

Antes: `layout_sub_frame(body, SubLayoutRegion { origin_x: 0.0, .. })`, depois
`new_x = Pt(target_x + ix)` para cada item.

Depois: `origin_x_abs = self.regions.current.line_start_x.0` capturado antes da
chamada; `layout_sub_frame(body, SubLayoutRegion { origin_x: origin_x_abs, .. })`;
`delta_x = target_x - origin_x_abs`; `new_x = Pt(ix + delta_x)`.

Prova algébrica (registada no L0): para conteúdo "normal" cujo `item.x` é aditivo a
partir da origem do sub-frame, `item.x_absoluto = origin_x_abs + item.x_relativo`;
logo `item.x_absoluto + delta_x = origin_x_abs + item.x_relativo + (target_x -
origin_x_abs) = target_x + item.x_relativo` — **idêntico ao valor produzido antes**.
Para `Content::Place` aninhado (que já emite `item.x` absoluto, sem termo relativo),
somar só `delta_x` (não `target_x` inteiro) elimina a duplicação. Eixo Y **inalterado**
— já usava o padrão delta correcto (`sub_origin_y` subtraído antes de somar `target_y`)
porque `layout_sub_frame` nunca teve um parâmetro `origin_y` (o sub-frame começa
sempre em `ascender`, um valor local pequeno, não uma origem absoluta a duplicar).

### 2.2 `cursor.rs::flush_pending_footnote_bodies`

Mesmo padrão. Antes: `origin_x: 0.0` + `target_x = left_x` somado inteiro. Depois:
`origin_x: left_x` + `target_x = 0.0` (delta, já correcto porque footnotes não têm
alinhamento horizontal próprio — estão sempre ancoradas a `left_x`, logo o delta é
sempre 0).

### 2.3 `layout_place` (`placement.rs`)

**Inalterado**, conforme a Opção A escolhida.

---

## 3. Validação

### 3.1 Os 3 repros de P772f, `mutool trace` antes/depois

| Repro | Antes (x medido) | Depois (x medido) | Vanilla (referência) |
|---|---|---|---|
| `place()` directo (sem `align`) dentro de `block()` | 35.247 / 120.285 | 35.247 / 120.285 (sem alteração — já correcto) | 35.247 / 125.287 (diff = gutter, gap conhecido não corrigido nesta linha, P772f §3.2) |
| `align(top, place(...))` dentro de `block()` (achado B) | 55.494 / **225.57** (quase fora da página de 226.77pt) | **35.247 / 120.285** (idêntico ao directo) | 35.247 / 125.287 |
| Caso completo do grid original (2 colunas) | idem acima | idem acima, ambos os círculos dentro da página, gap ≈ 85pt (não ≈ 180pt) | idem |

### 3.2 Wrappers adicionais confirmados

| Wrapper | Antes | Depois | Método |
|---|---|---|---|
| `align` (`placement.rs`) | duplicava | corrigido | `mutool trace`, repros acima |
| footnote (`cursor.rs`) | duplicava (mesmo padrão, confirmado por leitura + repro dedicado) | corrigido — círculo em `place()`+`align()` dentro do body de uma footnote fica dentro da página | `mutool trace`, repro `#footnote[#grid(...align(top,place(...)))]` — círculo cristalino `(35.247, 108.141)` vs vanilla `(35.247469, 108.13836)`, sub-pt |
| `columns.rs` | hipótese de duplicação | **auditado, sem alteração necessária** (§1.4) | `mutool trace`, repro `#columns(2, grid(...align(top,place(...))))` — `(41.996, 41.998)` vs vanilla `(41.996626, 41.996628)`, sub-pt |
| `box.rs`, `pad.rs`, `stack.rs` | não usam o padrão recompositor (usam `layout_sub_frame_inline` ou manipulação directa de cursor) | inalterados, confirmados por leitura de código como já consumidores absolutos | leitura de código (`boxed.rs`, `pad.rs`, `stack.rs`) |
| `measure_content_real` (`mod.rs:1751`) | `Layouter` isolado próprio, sem `regions.cell` | não aplicável — não é um wrapper de conteúdo em curso, é medição standalone | leitura de código |
| `place.rs` (`float: true`) | mecanismo `floats_pending`/`DeferredFloat`, distinto do `float: false` corrigido aqui | fora do âmbito — dispatcher separado, não usa o padrão `layout_place` scope Column absoluto da mesma forma | leitura de código, não testado com repro dedicado (nenhum indício de duplicação: `float:true` exige `scope: Parent`, que já usa o mesmo `in_sub_frame` desde P763f) |

### 3.3 Testes de regressão novos (`01_core/src/engine/layout/tests.rs`, prefixo `p772g_`)

- `p772g_place_dentro_de_align_dentro_de_grid_cell_bate_com_place_directo` — compara
  `place()` directo vs `align(top, place())` na mesma posição de célula; falha se
  divergirem (regressão directa do achado B).
- `p772g_place_dentro_de_align_dentro_de_grid_de_duas_colunas_nao_sai_da_pagina` —
  reprodução completa de P772f §2.4; falha se algum círculo sair da página ou se a
  distância entre colunas sugerir duplicação (`gap ≥ 100pt`).
- `p772g_footnote_com_place_dentro_de_align_bate_com_vanilla_estrutura` — regressão da
  correcção em `cursor.rs`.

Todos os 3 passam. Testes de footnote pré-existentes (`p304_*`, `p305_*`, `p537_*`,
`p552_*` — 23 testes no total via `cargo test ... footnote`) continuam verdes, incluindo
os que verificam posição Y exacta e empilhamento por coluna — confirma zero regressão
na emissão normal de footnotes (sem `Place` aninhado).

### 3.4 Suite completa

- `cargo test --workspace --release`: **4163 passed (typst-core lib, +3 novos) + 644
  passed (integration) + 33 + 29 + 2 + 2, 0 failed**. Medido no working tree deste
  passo (commit-base `2c7025a9a`).
- `crystalline-lint .`: **0 violations** (mesmo warning V7 pré-existente e não
  relacionado de P772f, prompt órfão `package_version_resolution.md`).

---

## 4. Critério de fecho — checklist

- [x] Todos os wrappers que usam `layout_sub_frame` com `origin_x: 0.0` levantados e
      testados individualmente (§1.2, §3.2).
- [x] Correcção aplicada — mas na Opção A confirmada com o humano: os wrappers
      recompositores (`layout_align`, footnotes) tornados consumidores absolutos, em
      vez de `layout_place` ganhar lógica condicional. `layout_place` permanece
      inalterado, conforme decisão registada em §1.3.
- [x] Os 3 repros de P772f validados com coordenadas exactas (ΔX/ΔY ≈ 0 dentro de
      tolerância sub-pt) — §3.1.
- [x] Cada wrapper adicional confirmado como afectado (`align`, footnotes) ou não
      (`columns`, `box`, `pad`, `stack`) e testado individualmente — §3.2.
- [x] `cargo test --workspace` verde — §3.4.
- [x] `crystalline-lint .` zero violações — §3.4.
- [x] L0 de `placement`/`layout` actualizado antes do código — §1.3,
      `00_nucleo/prompts/engine/layout.md` secção "Contrato de composição de
      coordenadas entre `layout_sub_frame` e `Content::Place`", hash `b1739fd9`.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772g.md` (este ficheiro).

---

## 5. Próximo passo

Com P772g e P772h fechados, restam pendentes as duas decisões arqueológicas de P772f
(`header:`/`footer:` inventados — remover ou implementar `split_header_footer`;
código órfão de `align`+`place` per-cell em `grid.rs` §3.3 de P772f — reverter ou
formalizar com L0 próprio). Depois dessas duas decisões, retomar a varredura da
stdlib: `visualize::image::svg` (7 itens), `foundations::scope` (7 itens),
`text::font::*` (~22 itens).
