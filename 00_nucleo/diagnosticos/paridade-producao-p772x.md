# P772x — Correção: decoração não propaga através de `layout_sub_frame`

> **Passo:** 772x
> **Data:** 2026-07-17
> **Commit-base:** working tree após P772w (não commitado no início deste passo).
> **Dependência:** P772w (achado, causa raiz identificada).

---

## 1. Sonda — confirmação dos 7 call-sites e do mecanismo exato

```bash
grep -rn "\.layout_sub_frame(" 01_core/src/engine/layout/*.rs | grep -v tests.rs
```

Confirmados exatamente **7** call-sites (nenhum a mais, nenhum a menos — bate com a contagem de P772w):

| # | Call-site | Papel |
|---|---|---|
| 1 | `placement.rs::layout_align` (`Content::Align`) | Emissão real |
| 2 | `placement.rs::layout_place` (`Content::Place`, `float: false`) | Emissão real — **caso motivador de P772w** |
| 3 | `grid.rs` Fase 2 (emissão de célula) | Emissão real |
| 4 | `cursor.rs` (footnote body, Pass 1+2) | Emissão real (dois passes) |
| 5 | `place.rs` (`Content::Place`, `float: true`) | Emissão **diferida** (`DeferredFloat`, flush na página) |
| 6 | `grid.rs` Fase 1 (medição de altura de linha) | Medição pura — `_sub_items` já descartado antes deste passo |
| 7 | `mod.rs::measure_content_real` (`measure()`) | Medição pura, `Layouter` isolado e efémero |

### Mecanismo de `flush_line`/`decoration_lines_collector`

```bash
grep -n "decoration_lines_collector\|fn flush_line" 01_core/src/engine/layout/cursor.rs
```

`flush_line()` (`cursor.rs:218`) regista um `DecoSegment { start_x: line_start_x, end_x: cursor_x,
baseline_y: cursor_y }` em `self.decoration_lines_collector` (se `Some`) **antes** do drain da linha —
confirmado exatamente onde (`cursor.rs:230-238`). `layout_sub_frame` (`sub_frame.rs`) nunca chamava
`flush_line()` — fazia o seu próprio flush manual da última linha, sem o hook.

### Tradução de coordenadas — confirmado por leitura de código, não assumido

`00_nucleo/prompts/engine/layout.md` §"Contrato de composição de coordenadas" já documentava que cada
call-site de `layout_sub_frame` aplica a sua própria translação aos `FrameItem`s devolvidos. Lida
directamente em cada um dos 4 call-sites de emissão imediata:

| Call-site | Translação de `FrameItem` (já existente) |
|---|---|
| `layout_align` | `new_x = ix + delta_x`; `new_y = target_y + iy - sub_origin_y` |
| `layout_place` | `new_x = target_x + ix`; `new_y = target_y + iy - y_offset` |
| `grid.rs` (célula) | `new_x = lx` (já absoluto); `new_y = body_y + (ly - local_start_y)` |
| `cursor.rs` (footnote) | `new_x = ix + target_x` (target_x=0.0); `new_y = iy + target_y` |

Confirmado (não assumido): cada `DecoSegment { start_x, end_x, baseline_y }` é tratável como dois pontos
x (`start_x`, `end_x`) e um y (`baseline_y`) — a MESMA fórmula aplicada a `(ix, iy)` de cada `FrameItem`
aplica-se directamente aos campos do segmento.

`layout_sub_frame` **não tinha** acesso privilegiado ao collector do pai além do campo partilhado
`self.decoration_lines_collector` (mesma instância de `Layouter`, `&mut self`) — confirmado por leitura:
não existe um caminho novo necessário, só a disciplina de swap-in/out local (ver §2).

---

## 2. Implementação

### L0 atualizado antes do código

`00_nucleo/prompts/engine/layout.md` §"Sub-layout isolado (`layout_sub_frame`, Passo 629)" — assinatura
actualizada para `(f64, Vec<FrameItem>, Vec<DecoSegment>)`, mecanismo de swap do collector documentado,
nova secção "Decoração através de `layout_sub_frame` (P772x)" com a tabela de classificação dos 7
call-sites e a nota de paridade ADR-0107 (ver §4).

### Mecanismo

1. **`layout_sub_frame`** (`sub_frame.rs`): no início, faz swap de `self.decoration_lines_collector` por
   um collector **local** (`Some(Vec::new())` só se havia um ambiente activo — sem overhead quando não há
   decoração em curso, mesma disciplina de `flush_line`/`decorations.rs`). O flush manual da última linha
   ganha o mesmo hook de `flush_line()` — regista o segmento antes de drenar. No fim, recupera os segmentos
   locais e restaura o collector ambiente (LIFO).
2. **4 call-sites de emissão imediata** (`layout_align`, `layout_place`, `grid.rs` célula, `cursor.rs`
   footnote): cada um aplica aos `DecoSegment`s devolvidos a MESMA translação já aplicada aos
   `FrameItem`s, e reinsere no collector ambiente (`self.decoration_lines_collector`, se `Some`).
3. **1 call-site de emissão diferida** (`place.rs`, `float: true`): `DeferredFloat` ganhou um campo
   `deco_segments: Vec<DecoSegment>` (coordenadas locais). Em `emit_deferred_float` (`cursor.rs`, chamado
   no flush da página), os segmentos são traduzidos com a mesma fórmula usada para `body_items` e
   reinseridos no collector **se ainda estiver `Some` nesse momento** — best-effort, documentado como tal
   (ver §5, achado residual).
4. **2 call-sites de medição pura** (`grid.rs` Fase 1, `mod.rs::measure_content_real`): descartam o 3º
   elemento da tupla (`_deco`) — não emitem items reais, não devem contribuir decoração.

---

## 3. Validação — repro original + checklist de sub-layouts

Todos os testes usam `mutool trace | grep -c "<stroke_path"` (só tags de abertura — `grep -c stroke_path`
sem qualificar conta tags de abertura E fecho, inflacionando o número para o dobro; confirmado por
inspecção directa do output antes de fixar a metodologia).

| Caso | Vanilla | Cristalino antes | Cristalino depois | Confirmado visualmente |
|---|---:|---:|---:|---|
| `#underline[Some text #place(top+left)[explanation].]` (repro original P772w) | 4 | 1 | **2** | ✅ ambos os trechos sublinhados |
| `#underline[#grid(columns: 1, [Cell text])]` | 1 | — | **1** | ✅ igual ao vanilla |
| `#underline[#box[Boxed text]]` | 1 | — | **1** | ✅ igual ao vanilla (box não usa `layout_sub_frame` — nunca esteve afectado) |
| `#strike[Some text #place(..)[explanation].]` | 3 | — | **2** | ✅ ambos os trechos riscados |
| `#overline[Some text #place(..)[explanation].]` | 3 | — | **2** | ✅ ambos os trechos com overline |
| `#underline[Normal text, no sub-frame.]` (não-regressão) | 2 | 1 | **1** | ✅ sem alteração — não usa sub-frame |

### Nota sobre a contagem exacta (vanilla ≠ cristalino, mesmo depois da correcção)

A contagem de `stroke_path` do vanilla é sistematicamente maior que a do cristalino **mesmo no caso de
não-regressão sem nenhum sub-frame envolvido** (`#underline[Normal text, no sub-frame.]`: vanilla 2,
cristalino 1) — confirma que a diferença de contagem é uma característica pré-existente do exportador
vanilla (segmenta o stroke por `MCID`/span de texto — confirmado por leitura directa do content stream
descomprimido do PDF vanilla, `qwe...cm...m...l...S` repetido por span), **não** algo introduzido ou
incompleto nesta correcção. Confirmado por render (`mutool draw`) que a decoração aparece visualmente
contínua e no local correcto em todos os casos — ver §4 (ADR-0107).

### Suite completa

```
cargo build --release --workspace --tests   → 0 erros
cargo test --workspace --release
  typst-core:   4209 passed, 0 failed  (+5 novos, P772x)
  typst-infra:   647 passed, 0 failed, 5 ignored
  typst-shell:    33 passed, 0 failed
  typst-wiring:    2 passed, 0 failed
  cli (integration): 29 passed, 0 failed
  crystalline_lint (integration): 2 passed, 0 failed
crystalline-lint .
  0 violações (mesmo warning V7 pré-existente sobre
  package_version_resolution.md, não relacionado)
```

### Testes automatizados novos

`01_core/src/engine/layout/tests.rs`, secção "P772x — decoração propaga através de `layout_sub_frame`":
`p772x_underline_propaga_atraves_de_place`, `p772x_underline_propaga_atraves_de_grid`,
`p772x_strike_propaga_atraves_de_place`, `p772x_overline_propaga_atraves_de_place`,
`p772x_underline_sem_sub_frame_sem_regressao`.

---

## 4. Paridade — mecanismo vs língua (ADR-0107)

A contagem exacta de operadores `S` (stroke) que o vanilla emite para uma decoração não é replicada
byte-a-byte — é mecanismo de exportação PDF (segmentação por `MCID`/span de texto), não língua. O
observável de língua correcto é: **a decoração aparece, visualmente contínua, sobre o texto correcto** —
confirmado por render em todos os 5 casos testados (§3). Replicar a segmentação exacta por `MCID` exigiria
re-desenhar o mecanismo P284/P286 para emitir um segmento por span de texto/estilo, não por linha visual —
fora do âmbito desta correcção (que visava especificamente a AUSÊNCIA de decoração, não a sua
granularidade de exportação).

---

## 5. Achado residual — `place(float: true)` é best-effort, não garantido

Durante a implementação, confirmou-se que o caso `place(float: true, ...)` (floats deferidos, emitidos no
flush da página via `DeferredFloat`) tem uma limitação estrutural que os outros 4 call-sites de emissão
imediata não têm: o `decoration_lines_collector` ambiente só existe enquanto o consumer
`Underline`/`Strike`/`Overline` (`decorations.rs`) está dentro do seu próprio `layout_content(body)` — mas
um float só é efectivamente desenhado (`emit_deferred_float`) no flush da página (`new_page()`/`finish()`),
que pode ocorrer **muito depois** desse `layout_content(body)` ter retornado (ex.: floats que só flusham
no fim do documento). Implementado como **best-effort**: os segmentos são capturados e traduzidos, e
reinseridos no collector **se ainda estiver `Some` nesse momento** — funciona quando o flush ocorre
enquanto o consumer decorador ainda está activo (ex.: quebra de página a meio do body decorado), mas não é
garantido no caso geral. Documentado explicitamente no código (`DeferredFloat::deco_segments`, `mod.rs`) e
no L0 — não é uma lacuna silenciosa. O exemplo do próprio vanilla que motivou o achado original de P772w
(`#place(float: true, bottom + right)[explanation]`) usa `float: true`, mas o repro CONCRETO usado para
validar este passo (`#place(top+left)[explanation]`, sem `float:`) usa o default `float: false` — coberto
integralmente pela correcção principal.

---

## Critério de fecho do passo

- [x] Lista completa dos 7 call-sites de `layout_sub_frame` confirmada (nenhum a mais).
- [x] Mecanismo de tradução de `DecoSegment` do referencial do sub-frame para o do pai implementado,
      reaproveitando a translação já usada para `FrameItem`s, um a um nos 4 call-sites de emissão imediata
      + 1 de emissão diferida (best-effort, achado residual §5) + 2 de medição pura (sem-op).
- [x] Repro original de P772w (`place()`, `float: false` default) corrigido — confirmado visualmente
      (ambos os trechos decorados); contagem exacta de `stroke_path` diverge do vanilla por razões de
      mecanismo de exportação (§4), não por decoração ausente.
- [x] Testado em mais 2 consumidores de `layout_sub_frame` (`grid` — corrigido e confirmado igual ao
      vanilla; `box` — nunca esteve afectado, não usa `layout_sub_frame`).
- [x] Testado com `strike`/`overline`, não só `underline` — generaliza correctamente.
- [x] Sem regressão em decoração fora de sub-frame.
- [x] `cargo test --workspace` verde (4209 em typst-core, +5 novos).
- [x] `crystalline-lint .` zero violações.
- [x] L0 de `layout`/`sub_frame` atualizado antes do código.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772x.md`, com contagem de `stroke_path`
      antes/depois para cada caso testado (§3).

---

## Próximo passo

Conforme P772w: `image::pdf`, `math.class(...)`, ou fallback de fontes matemáticas — os três débitos
restantes — ou encerrar a série P765a-P772x com um resumo final. Registado também nesta sessão: o
achado residual `place(float: true)` best-effort (§5), de escopo muito mais estreito que o achado
original — candidato a nota de rodapé num resumo final, não a passo dedicado isolado, salvo evidência
futura de que o caso `float: true` é comum o suficiente para justificar.
