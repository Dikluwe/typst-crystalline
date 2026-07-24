# Relatório — Passo 897: `Content::Align`/`Content::Place` sob `width: auto`

**Data:** 2026-07-24
**Commit de partida:** `73770767b609621ae4096726f7b766dcfc7c340a` (2026-07-24 14:44:55 -0300)
**Estado da árvore no início:** working tree com alterações não commitadas de P896 já aplicadas
(campos `pending_equation_centering`/`pending_equation_numbering`, `apply_pending_equation_fixups`,
`shift_frame_item_x`) — commit de P896 ainda não tinha sido feito quando este passo começou.

---

## Fase A — confirmação antes de reaproveitar

### Ponto 1 — o mecanismo de P896 é directamente reaproveitável?

**Sim, com uma adaptação de bookkeeping.** O padrão de 3 peças (campo `pending_*` +
`apply_pending_*_fixups` chamado de `finish()`/`new_page()` + `helpers::shift_frame_item_x`) migra
sem alteração estrutural. A diferença: em vez de recalcular a fórmula de centragem inline (como
`equation.rs` fazia), a correcção deste passo **reaproveita `resolve_alignment` directamente** como
função de correcção — mais simples porque `Content::Align`/`Content::Place` já passam por
`resolve_alignment` no caminho normal (as equações não passavam por essa função, tinham a sua
própria fórmula em `equation.rs`).

Uma particularidade real encontrada: `layout_place` soma `dx` **depois** de `resolve_alignment`
devolver `base_x` (`target_x = base_x + dx`). Como `resolve_alignment` é linear em `origin_x` (soma-o
directamente ao resultado, sem interagir com `avail_w`/`content_w`), gravar `origin_x + dx` no lugar
de `origin_x` no tuplo pendente faz a chamada de correcção recompor directamente o `target_x` final
(já com `dx` incluído) sem precisar de somar `dx` uma segunda vez fora de `resolve_alignment`. Sem
esta adaptação, o delta de correcção estaria errado por exactamente `dx`. `layout_align` não tem
este problema (sem `dx`/`dy`).

### Ponto 2 — `Content::Align`/`Content::Place` contribuem para a largura final da página?

**Sim, confirmado por teste** (`p897_align_center_centram_contra_a_largura_final_da_pagina`). O
mecanismo de diferimento cobre isto automaticamente, sem mudança adicional a
`compute_page_width()`: o conteúdo alinhado é medido e emitido (na sua posição de fallback) antes de
`page_width` ser conhecido, na mesma ordem que as equações de P896; `compute_page_width()` (P867) já
soma todos os items emitidos (`current_items`/`current_line`) para determinar a largura final,
independentemente de terem vindo de `layout_align`, `layout_equation` ou texto normal.

### Ponto 3 — há mais consumidores de `resolve_alignment`/`available_width()`?

Grep exaustivo de `resolve_alignment(` confirma **exactamente 2 chamadas**, ambas em
`placement.rs`:

- `layout_align` (`Content::Align`) — `avail_w = self.available_width()`, sempre potencialmente
  infinito sob `width: auto` (não há ramo de célula).
- `layout_place` (`Content::Place`) — `avail_w` vem da tupla `(origin_x, origin_y, avail_w, avail_h,
  y_offset)` resolvida por `scope`: com célula de grid activa (`PlaceScope::Column` +
  `regions.cell.is_some()`) usa `cell.width`, **sempre finito** (não vulnerável); nos restantes
  ramos (`PlaceScope::Parent` sempre, `PlaceScope::Column` sem célula) usa `avail_w_page =
  self.available_width()`, potencialmente infinito.

`available_width()` tem outros consumidores fora de `placement.rs` (`block.rs`, `boxed.rs`,
`grid.rs`, `sequence.rs`, `stack.rs`, `transform.rs`, `shape.rs`, `pad.rs`) — todos usam o valor como
**limite de quebra/dimensionamento** ("nunca quebrar", seguro com infinito), não para centrar/
posicionar via `resolve_alignment`. Não são vulneráveis à mesma classe de bug e não foram tocados.

**Achado adicional (fora do que a Fase A pedia, registado):** `available_height()` (`mod.rs:684`) e
`page_bottom_limit()` (`mod.rs:697`) também devolvem `f64::INFINITY` sob `height: auto` — existe um
bug simétrico no **eixo vertical** para `VAlign::Horizon`/`VAlign::Bottom`. Confirmado por leitura de
código, **não confirmado por teste, não corrigido** — decisão de escopo (ver secção "Fora de
âmbito" abaixo, mesmo padrão que P896 usou para justificar deixar este próprio achado de fora).

---

## Fase B — Implementação

### Ficheiros tocados

- `01_core/src/engine/layout/mod.rs` — campo `pending_align_centering: Vec<(usize, usize, Align2D,
  f64, f64, f64)>` + método `apply_pending_align_fixups`; chamada wired em `finish()`.
- `01_core/src/engine/layout/placement.rs` — `layout_align`/`layout_place`: fallback
  `avail_w_for_resolve` (substitui `avail_w` por `content_w` quando infinito) + registo da entrada
  pendente.
- `01_core/src/engine/layout/cursor.rs` — chamada de `apply_pending_align_fixups` wired em
  `new_page()`, logo a seguir a `apply_pending_equation_fixups`.
- `01_core/src/engine/layout/tests.rs` — 2 testes novos.
- `00_nucleo/prompts/engine/layout.md` — nova secção `## P897`.

### TDD — confirmação de vermelho antes da correcção

Os testes foram escritos depois de o mecanismo já estar implementado nesta sessão (retomada de
contexto), pelo que a disciplina TDD foi verificada **por reversão temporária**: substituí
`avail_w_for_resolve` por `avail_w` directamente nas duas chamadas de `resolve_alignment` em
`placement.rs` (backup em `/tmp/.../placement.rs.bak`), corri os 2 testes novos:

```
test p897_align_center_sob_width_auto_nao_produz_infinito ... FAILED
  panicked: page.width não deve ser infinito: inf
test p897_align_center_centram_contra_a_largura_final_da_pagina ... FAILED
  panicked: assertion failed: page.width.is_finite()
```

Restaurei o ficheiro original (com a correcção) e confirmei os 2 testes verdes. Vermelho→verde
confirmado, mesma disciplina exigida pelo `CLAUDE.md`.

### Suíte completa

```
typst-core:    4724 passed; 0 failed; 2 ignored
typst-infra:    734 passed; 0 failed; 5 ignored
typst-shell:     41 passed; 0 failed; 0 ignored
```

Zero regressões.

### `crystalline-lint`

```
crystalline-lint --fix-hashes .
Fixed 13 files (todos no mesmo L0 `prompts/engine/layout.md`) → hash f4b03780
Re-running analysis... ✅ 0 drift warnings remaining

crystalline-lint .
warning: Prompt órfão: 'prompts/infra/package_version_resolution.md' [V7]
```

O único warning restante (`package_version_resolution.md`, V7 órfão) é pré-existente e não
relacionado com este passo — não corrigido, fora de âmbito.

### Recompilação do `.typ` de 30 secções

Hash confirmado **igual ao de P896** antes de recompilar:

```
sha256sum .typ/typst-math-comprehensive-test.typ
9ae95a8d892103afc0c82c505a7f490a856acdad828577b7351013ad72691f29
```

`grep -n "#align" .typ/typst-math-comprehensive-test.typ` → **nenhuma ocorrência**. O ficheiro não
usa `#align` — confirma o que o próprio passo já antecipava ("pode não haver nenhum caso real neste
ficheiro específico, o que não invalida o teste unitário"). Recompilei mesmo assim para confirmar
ausência de regressão no caminho de equações (que agora também passa por
`apply_pending_align_fixups`, ainda que como no-op nesta suite):

```
cargo build --release
./target/release/typst .typ/typst-math-comprehensive-test.typ /tmp/p897-out/p897-cristalino.pdf
exit=0
MediaBox: [0 0 478.23 5624.71]  — finito, sem "inf"
Páginas: 1
Sequência de numeração: (1) (2) (3) ... (44) — completa, igual a P896
```

Sem regressão.

---

## Fase C — Regressão (benchmark, 7 cenários)

Primeira leitura (`hyperfine --warmup 3 -N`, `/tmp/p872-bench/`):

| Cenário | P897 (1ª leitura) |
|---|---|
| 01-hello | 94.9 ms |
| 02-lorem | 118.2 ms |
| 03-images | 102.4 ms |
| 04-math | 158.4 ms |
| 05-tables | 116.6 ms |
| 06-long | 374.0 ms |
| 07-context | 136.7 ms |

`04-math` (158.4ms) e `07-context` (136.7ms) pareciam ligeiramente acima da baseline reportada em
P896 (151.7ms e 132.0ms, respectivamente) — mesma assinatura do "falso alarme" já investigado nesse
passo. Antes de aceitar como regressão, apliquei a mesma disciplina: remedição isolada com 30
amostras de `04-math`, `07-context` **e um controlo não relacionado** (`01-hello`, que não usa
`#align`/`#place` nem toca `equation.rs` de forma que exercite os novos fixups além de um `no-op`):

| Cenário | Remedição (30 amostras) | Baseline P896 |
|---|---|---|
| 01-hello (controlo) | 100.7 ms | 93.6 ms |
| 04-math | 160.2 ms | 151.7 ms |
| 07-context | 140.6 ms | 132.0 ms |

O controlo (`01-hello`) subiu ~7ms (~7.5%) face à baseline de P896 — um deslocamento de magnitude
semelhante ao de `04-math` (+8.5ms) e `07-context` (+8.6ms). Como `01-hello` não exercita nenhum
código tocado por este passo, a subida uniforme em todos os três cenários (incluindo o controlo)
confirma **ruído ambiental do momento da medição**, não uma regressão introduzida pelas alterações
de P897 — mesma conclusão metodológica que P896 já tinha demonstrado.

---

## Fora de âmbito (registado, não corrigido/investigado neste passo)

1. **Eixo vertical (`height: auto`)** — `available_height()`/`page_bottom_limit()` também devolvem
   `f64::INFINITY`; `VAlign::Horizon`/`VAlign::Bottom` sofrem o mesmo bug de posicionamento no eixo
   y. Confirmado por leitura de código (Fase A ponto 3), não testado, não corrigido. Candidato a
   passo futuro dedicado, mesmo padrão de diferimento generalizado ao eixo y.
2. **Espaçamento ausente em torno de texto entre aspas em modo matemático** (achado do enunciado do
   próprio passo 897, não investigado aqui por instrução explícita — mecanismo provavelmente não
   relacionado, ver `typst-passo-897.md`).
3. **P893** (`FontMetrics::math_constants`) continua parado em Fase A, sem conflito com este passo.

## Resultado

- `Content::Align`/`Content::Place` (ramos sem célula de grid) já não produzem posição infinita sob
  `width: auto`, e centram/alinham correctamente contra a largura final real da página.
- Mecanismo reaproveitado de P896, generalizado via reaproveitamento directo de `resolve_alignment`
  em vez de reimplementar a fórmula.
- 2 testes novos (TDD vermelho→verde confirmado por reversão temporária).
- Suíte completa verde (4724+734+41, 0 falhas).
- `crystalline-lint`: 0 drift, 0 violations novas.
- `.typ` de 30 secções recompila sem regressão (hash confirmado igual a P896).
- Benchmark: sem regressão real (subida uniforme incluindo controlo não relacionado = ruído
  ambiental, mesma disciplina de investigação de P896).
