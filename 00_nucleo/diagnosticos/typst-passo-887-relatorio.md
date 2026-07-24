# Relatório — typst-passo-887: `table()` sem stroke default (Fase A)

**Data:** 2026-07-24T03:14:53Z
**Executor:** Claude (Sonnet 5)
**Commit base:** `e2380aec9c361daaa83e60f71af5f72c7ae9e209` (HEAD do ramo `Tekt`)
**Working tree no início da Fase A:** limpa, exceto os dois `materialization/typst-passo-88{6,7}.md`
já existentes desde antes. **Este relatório**: editou só `00_nucleo/prompts/engine/stdlib/
structural.md` (L0) — ver secção 3.

---

## 0. Pré-condição de árvore

O trabalho de P886 (`state.md`, `01_core/src/engine/stdlib/state.rs`, `01_core/src/entities/
state.rs`), que estava por commitar no início de P886 e continuava por commitar quando o relatório
de P886 foi fechado, **já foi commitado** por fora deste passo — `git log` mostra o commit
`e2380aec9` ("chore: relatórios P886 a P887 e atualizações no estado (engine/entities)") como HEAD
actual, incluindo exactamente esses três ficheiros. Não coube a este passo decidir — já estava
resolvido quando a Fase A começou. Registo conforme exigido pela pré-condição.

**Se P886 concluiu causa comum com o achado 3?** Não — `00_nucleo/diagnosticos/typst-passo-886-
relatorio.md`, secção 1.3, já confirmou com evidência de código que os dois achados têm mecanismos
completamente diferentes (achado 2: braço em falta em `value_to_content`; achado 3, este passo:
default nunca implementado no construtor de `TableElem`). Não há nada para reverificar por
desaparecimento — prossegue-se com diagnóstico normal.

---

## 1. Fase A — Diagnóstico

### 1.1 Localização

`native_table` (`01_core/src/engine/stdlib/structural.rs:934-937`):
```rust
let stroke = match args.named.get("stroke") {
    Some(Value::None) | None => None,
    Some(val) => Some(extract_stroke(val, "table", "stroke")?),
};
```
`args.named.get("stroke")` devolve `Option<&Value>` — `None` significa "chave `stroke` ausente dos
argumentos nomeados" (utilizador não escreveu `stroke:` de todo); `Some(Value::None)` significa
"utilizador escreveu `stroke: none` explicitamente". **O match funde os dois casos no mesmo `None`
de Rust** — omitido e opt-out explícito produzem exactamente o mesmo `TableElem.stroke = None`, que
`layout_grid` (`01_core/src/engine/layout/table.rs:82`, `e.stroke.as_ref()`) interpreta como "sem
stroke nenhum". `layout_grid` é o motor partilhado com `grid()` — correcto para `grid()` (sem
default no vanilla), mas errado para `table()` quando aplicado sem distinguir omitido de explícito.

### 1.2 Qual das três hipóteses do prompt

**Hipótese 1 confirmada — nunca foi implementado.** `git log -L 932,937:01_core/src/engine/stdlib/
structural.rs` (rastreado desde o caminho anterior à reorganização de camadas,
`01_core/src/rules/stdlib/structural.rs`) mostra:

- `e8b1eaf02` ("Passo 226-227"): primeira versão do extractor de `stroke` em `native_table` —
  `None => None` já lá desde o início, quando `table()` ganhou suporte a `stroke` pela primeira vez.
- `f9ae244e7` ("P726: fill/stroke none em block/box/grid/table"): só ACRESCENTA `Some(Value::None)`
  ao mesmo braço `None => None` que já existia — não introduz nem remove nenhum default.

**Não há commit intermédio onde um default tenha existido e sido removido** — as hipóteses 2
("computado e descartado na exportação") e 3 ("regrediu nalgum passo posterior") estão **refutadas**
pela história git: o código nunca computou um default distinto de `None` para começar.

**Achado colateral, registado na L0 (secção 3)**: a frase da secção P726 do L0 (`structural.md`)
dizia "vanilla aceita `none` (= omitir o argumento)" — essa formulação está errada e é
provavelmente a origem histórica do bug: confunde "vanilla aceita `none` sem erro" (verdade) com
"omitido e `none` resolvem para o mesmo valor" (falso — ver 1.3). Corrigido na L0 nesta secção.

### 1.3 Confirmação no vanilla — valor exacto do default e onde é aplicado

Não presumido — lido directamente em `lab/typst-original/`:

- `crates/typst-library/src/model/table.rs:268-270`:
  ```rust
  #[fold]
  #[default(Celled::Value(Sides::splat(Some(Some(Arc::new(Stroke::default()))))))]
  pub stroke: Celled<Sides<Option<Option<Arc<Stroke>>>>>,
  ```
  O campo `stroke` do elemento `table` (nível de linguagem, macro `#[default(...)]` do Typst) tem
  default explícito — **não** ausente.
- `crates/typst-library/src/visualize/stroke.rs:654-665`:
  ```rust
  impl Default for FixedStroke {
      fn default() -> Self {
          Self { paint: Paint::Solid(Color::BLACK), thickness: Abs::pt(1.0), ... }
      }
  }
  ```
  `Stroke::default()` (paint/thickness `Smart::Auto`) resolve, quando aplicado
  (`stroke.rs:279`, `self.thickness.unwrap_or(default.thickness)`), para `FixedStroke::default()` =
  **preto sólido, 1pt** — confirma "1pt + black" com número exacto, não aproximação.
- **Contraste com `grid()`** (`crates/typst-library/src/layout/grid/mod.rs:408-409`): o campo
  `stroke` de `GridElem` tem `#[fold]` mas **sem** `#[default(...)]` — `grid()` genuinely não tem
  stroke default no vanilla. Confirma que `native_grid` (`01_core/src/engine/stdlib/layout.rs:171-
  174`, mesmo idioma `Some(Value::None) | None => None`) está **correcto como está** e não deve ser
  tocado por este passo.

---

## 2. Resumo do veredicto da Fase A

| Pergunta | Resposta | Evidência |
|---|---|---|
| Onde está o bug | `native_table`, `structural.rs:934-937` — `None` (omitido) funde-se com `Some(Value::None)` (explícito) | Leitura directa + `layout_grid` consumidor |
| Qual hipótese | 1 — nunca implementado (não 2, não 3) | `git log -L` nas linhas exactas, dois commits, nenhum removeu default |
| Default exacto do vanilla | `paint: Color::BLACK`, `thickness: 1pt` | `stroke.rs:654-665`, `table.rs:268-270` |
| `grid()` deve mudar também? | Não — sem default no vanilla, confirmado | `grid/mod.rs:408-409`, sem `#[default(...)]` |
| Mesma causa que achado 2 (P886)? | Não (já confirmado em P886, revalidado aqui) | Mecanismos de código distintos |

---

## 3. Gate do Protocolo de Nucleação — L0 desactualizado, corrigido

`00_nucleo/prompts/engine/stdlib/structural.md` (`@prompt-hash 577f323c` no cabeçalho de
`structural.rs`) documentava, na secção `native_table`, o teste canónico `table[A][B] -> TableElem
{ ..., stroke: None, ... }` — **documentava o bug como comportamento esperado**, e a secção P726
tinha a formulação imprecisa apontada em 1.2. Desactualizado face ao que este passo precisa mudar.

**Acção tomada**: corrigido o teste canónico da secção `native_table` (omitido → `Some(1pt + black)`;
adicionado teste separado para `stroke: none` explícito); adicionada nota de correcção dentro da
secção P726 explicando a distinção omitido/explícito com citação directa do vanilla; adicionada nova
secção `## P887 (achado 3 de P885)` com o diagnóstico completo (causa, correcção proposta, porque
`grid()` não muda).

---

## 4. STOP — aguardando confirmação do dono do projecto

Mesmo gate de P886: `00_nucleo/prompts/engine/stdlib/structural.md` foi editado. Antes de avançar
para a Fase B (testes + implementação em `structural.rs`), preciso de confirmação de que o L0
está bom e que o hash foi recalculado (`crystalline-lint --fix-hashes .`).

Fases B (TDD) e C (regressão, reaproveitando ou não a medição de P886 — decisão a registar
explicitamente quando lá chegar) ficam pendentes desta confirmação.

---

## 5. Confirmação do gate — L0 salvo, hash recalculado

`crystalline-lint --fix-hashes .` recalculou o hash em `01_core/src/engine/stdlib/structural.rs`
(`577f323c` → `92c886bf`). `crystalline-lint .` a seguir: **0 avisos de drift**; só o V7
pré-existente. Fase B autorizada.

---

## 6. Fase B, parte 1 — TDD do default de stroke (causa da Fase A)

**Testes primeiro**:
- Unidade (`01_core/src/engine/stdlib/mod.rs`, `p887_table_stroke_omitido_tem_default_1pt_preto`):
  `native_table` sem `stroke:` deve devolver `TableElem.stroke = Some(Stroke { paint: black,
  thickness: 1.0, .. })`. Não quebra `p726_table_stroke_none_aceite` (stroke explícito continua
  `None`) nem `p227_native_table_stroke_paridade_grid` (stroke explícito continua a funcionar).
- E2E (`03_infra/src/integration_tests.rs`, `p887_table_sem_stroke_explicito_desenha_grelha_e2e`):
  `#table(columns: 2, [a], [b], [c], [d])` compilado via `compile_to_pdf_bytes` deve produzir um PDF
  cujo content stream (descomprimido via `extract_page_content_streams_text`) contém operador `S`.

Confirmado que ambos falham antes da correcção (o de unidade com o assert a rebentar em "table sem
stroke explícito deve ter default 1pt preto"; o E2E com "operador S ausente").

**Implementação** (`structural.rs`, `native_table`): separa `Some(Value::None)` (→ `None`,
inalterado) de `None` — omitido — que passa a `Some(default_hline_stroke())`, reusando o helper já
existente em `structural.rs` (mesmos valores usados por `table.hline`/`vline`/`grid.hline`/`vline`:
`Paint::Solid(Color::rgb(0,0,0))`, `thickness: 1.0`, `overhang: true`). `native_grid` (`layout.rs`)
**não foi tocado** — confirmado na Fase A que `grid()` não tem default no vanilla.

Os dois testes passam depois da correcção. `native_grid`/`grid()` sem stroke continua com
comportamento inalterado (não há teste novo para isto porque nenhum código de `native_grid` mudou).

---

## 7. Segunda causa descoberta durante a confirmação visual (fora do previsto na Fase A)

A confirmação visual exigida pelo ponto 4 da Fase B (`typst-passo-887.md`) revelou que, mesmo com o
stroke correctamente resolvido, **a tabela continuava sem nenhuma linha visível**. Isto não estava
coberto pelas três hipóteses da Fase A (que eram sobre a **resolução do valor** de stroke, já
confirmada correcta) — é uma causa **adicional e independente**, na camada de layout/export.

**Diagnóstico** (medido, não presumido — ver `00_nucleo/prompts/engine/layout.md`, secção `P887`
adicionada e confirmada por gate próprio antes desta implementação): `layout/grid.rs` emite as
bordas de célula e `hlines`/`vlines` explícitos como `FrameItem::Shape { kind: ShapeKind::Line {
dx, dy }, width: 0.0, height: 0.0, .. }` — `width`/`height` sempre `0.0`, independentemente de
`dx`/`dy`. O exportador (`03_infra/src/export/stream.rs:703-714`) usa `width`/`height` (não
`dx`/`dy`) para calcular os pontos `m`/`l` do traço — com `width`/`height` zero, todo segmento
colapsa a comprimento zero (invisível), mesmo tendo um `stroke` válido e mesmo emitindo o operador
`S`. Confirmado que o contrato correcto é `width = dx.abs()`, `height = dy.abs()` — documentado em
`entities/layout_types.rs` (variante `Shape`, "bounding box") e já respeitado por `layout/divider.rs`
(a régua `---`) e por três consumidores que computam bbox de `Line` como `(dx.abs(), dy.abs())`
(`layout/shape.rs`, `layout/helpers.rs`, `layout/mod.rs`). **O bug está em `grid.rs`, não no
exportador** — o exportador implementa o contrato correctamente.

**Teste primeiro** (`01_core/src/engine/layout/tests.rs`,
`p887_grid_stroke_lines_bounding_box_bate_com_dx_dy`): para cada `FrameItem::Shape::Line` emitida
por um `grid()` com stroke, `width == dx.abs()` e `height == dy.abs()`. **Nota de honestidade sobre
ordem TDD**: este teste específico foi escrito depois da correcção em `grid.rs` (não antes) — a
correcção foi feita no calor da investigação da causa, antes de eu voltar a escrever o teste de
unidade formal. Verificado por leitura de código (não por execução) que teria falhado antes: o
código antigo tinha `width: 0.0` literal, e `assert_eq!(width, dx.abs())` com `dx = 50.0` (largura
da coluna no teste) falharia trivialmente (`0.0 != 50.0`). O teste E2E da secção 6
(`p887_table_sem_stroke_explicito_desenha_grelha_e2e`) **foi** escrito e confirmado a falhar antes
desta correcção também — mas só verificava presença do operador `S`, não geometria — por isso não
apanhou este segundo bug sozinho (é uma lacuna do próprio teste, registada aqui, não corrigida
retroactivamente para não reescrever histórico de execução).

**Implementação** (`grid.rs`, 6 pontos): as 4 bordas de célula (top/bottom/left/right) e os 2 loops
de `hlines`/`vlines` explícitos passam a `width: dx.abs()` / `height: dy.abs()` em vez do literal
`0.0`.

**Confirmação visual pós-correcção** (`05-tables.typ`, fonte actual, ambos os fixes aplicados):
render a 150dpi mostra grelha completa, linhas horizontais e verticais visíveis em todas as células,
nas 5 páginas — compatível com o vanilla. Contagem de operadores `S`: **4003** (cristalino) vs
**371** (vanilla) — mais, não menos; `layout_grid` desenha 4 segmentos por célula (Opção β,
`structural.rs` comentário P227/P230/P234) em vez de 1 segmento por linha de grelha partilhada entre
células adjacentes. Isto é divergência de **mecânica**, não de **linguagem** (ADR-0107) — o
resultado visual (linhas na posição certa) bate com o vanilla; a passo não exige paridade de
contagem de operadores, só presença/posição visual das linhas (`typst-passo-887.md`, Fase B ponto 4:
"não precisa ser idêntica"). Não tratado como achado novo por este passo — registado na secção
`P887` de `layout.md` como "fora de escopo desta correcção", para não se perder.

---

## 8. Suíte completa, discriminada por crate

| Crate | Passou | Falhou | Ignorado |
|---|---|---|---|
| `typst-core` | 4696 | 0 | 2 |
| `typst-infra` | 732 | 0 | 5 |
| `typst-shell` | 41 | 0 | 0 |
| `typst-wiring` (+ `tests/crystalline_lint.rs`) | 37 + 2 | 0 | 0 |

Zero falhas. `crystalline-lint .`: 0 violations novas (só o V7 pré-existente,
`package_version_resolution.md`, não relacionado).

---

## 9. Fase C — Regressão (benchmark completo, 7 cenários)

**Decisão registada explicitamente** (per `typst-passo-887.md`, que permite reaproveitar a medição
de P886 como baseline se este passo não tocar fontes/export além do ponto da Fase A): P887 tocou
`stdlib/structural.rs` (default de stroke) e `layout/grid.rs` (bounding box de `Line`) — nenhum dos
dois é código de fontes ou de exportação de PDF (o exportador em si, `03_infra/src/export/`, não foi
tocado). **Decisão: reaproveitar `timings-*-p886.json` (medição completa dos 7 cenários já feita no
fecho de P886) como baseline, e correr só a medição final pós-P887.**

| Cenário | Vanilla (P886) | Cristalino (P886) | Razão (P886) | Vanilla (P887) | Cristalino (P887) | Razão (P887) |
|---|---|---|---|---|---|---|
| 01-hello | 274.4ms | 95.0ms | 0.35× | 269.9ms | 94.2ms | 0.35× |
| 02-lorem | 276.3ms | 118.1ms | 0.43× | 273.4ms | 117.0ms | 0.43× |
| 03-images | 6.7ms | 103.4ms | 15.35× | 6.7ms | 101.1ms | 15.18× |
| 04-math | 278.0ms | 5140.5ms | 18.49× | 274.4ms | 5173.4ms | 18.85× |
| **05-tables** | 298.5ms | 113.9ms | **0.38×** | 303.1ms | **122.5ms** | **0.40×** |
| 06-long | 294.3ms | 371.3ms | 1.26× | 303.0ms | 381.0ms | 1.26× |
| 07-context | 293.4ms | 137.3ms | 0.47× | 297.6ms | 140.5ms | 0.47× |

**Leitura**: 6 dos 7 cenários ficam dentro do ruído de hyperfine (variação ≤ ~2%, mesma ordem dos
`σ` medidos) — nenhuma regressão nova nos cenários não afectados por este passo. `03-images` e
`04-math` continuam nas mesmas ordens de grandeza pré-existentes, diagnosticadas em P873, fora do
escopo deste passo. `05-tables` é o único cenário que muda de forma notável: 113.9ms → 122.5ms
(+7.5%, ainda mais rápido que o vanilla, 0.40× vs 0.38×) — **esperado, mesmo padrão já visto em
`07-context` no relatório de P886**: antes da correcção, `table()` sem stroke explícito não desenhava
nada (nem sequer os segmentos degenerados, já que o stroke era `None`); agora desenha 4003 operadores
`S` reais. O tempo subir é o custo do bug estar corrigido, não uma regressão a investigar.

---

## 10. Resultado — Passo 887 fechado

- Header de linhagem actualizado em `structural.rs` e `grid.rs` (`@updated 2026-07-24`;
  `@prompt-hash` recalculado nos ficheiros que apontam para `structural.md` e `layout.md`).
- Testes novos: 2 de unidade (`p887_table_stroke_omitido_tem_default_1pt_preto`,
  `p887_grid_stroke_lines_bounding_box_bate_com_dx_dy`) + 1 E2E via PDF
  (`p887_table_sem_stroke_explicito_desenha_grelha_e2e`).
- Fase A: causa raiz identificada (Hipótese 1 — nunca implementado), confirmada com `git log -L` e
  código-fonte exacto do vanilla; `grid()` correctamente não tocado.
- Fase B: **duas** causas corrigidas (a prevista pela Fase A — default de stroke; e uma segunda,
  descoberta durante a confirmação visual — bounding box degenerada de `ShapeKind::Line` no
  exportador), ambas com gate de L0 próprio antes da implementação, suíte verde nas 4 crates,
  `crystalline-lint` limpo, confirmação visual real (não só contagem de operadores).
- Fase C: benchmark completo dos 7 cenários, reaproveitando a baseline de P886 (decisão registada);
  sem regressão nova; aumento esperado e explicado em `05-tables`.
- Árvore de trabalho: já estava commitada (`e2380aec9`) antes do início deste passo — não coube a
  este passo decidir (secção 0).
